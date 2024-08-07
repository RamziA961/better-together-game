use std::f32::consts::PI;

use anyhow::Result;
use nalgebra::{vector, ComplexField, Vector3};
use rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{
    select,
    sync::{broadcast, mpsc},
    time,
};
use tracing::{error, info, instrument};

// half extents
const GROUND_DIM_HE: [f32; 3] = [40., 40., 0.1];

const PAWN_DIM_HE: [f32; 3] = [5.; 3];
const PAWN_START: [f32; 3] = [0., 20., 0.];
const PAWN_MASS: f32 = 20.;

const MAX_ANGULAR_VEL: f32 = 2. * PI;
const MAX_LINEAR_VEL: f32 = 10.;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicState {
    pawn_pos: [f32; 3],
    pawn_rot: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimulationUpdate {
    PublicState(PublicState),
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum Instruction {
    Jump,
    Left,
    Right,
    Up,
    Down,
    Cw,
    Ccw,
}

impl Instruction {
    fn apply(&self, rb: &mut RigidBody) {
        let lin_vel = rb.linvel().norm();
        let ang_vel = rb.angvel().norm().real();

        match self {
            Instruction::Up => {
                let diff = MAX_LINEAR_VEL - lin_vel;

                if diff >= 0. {
                    rb.apply_impulse(vector![0., 0., diff.min(1.)], true);
                }
            }
            Instruction::Down => {
                let diff = -MAX_LINEAR_VEL - lin_vel;

                if diff >= 0. {
                    rb.apply_impulse(vector![0., 0., diff.min(-1.)], true);
                }
            }
            Instruction::Jump => todo!(),
            Instruction::Left => todo!(),
            Instruction::Right => todo!(),
            Instruction::Cw => todo!(),
            Instruction::Ccw => todo!(),
        }
    }
}

pub struct Simulation {
    channel: broadcast::Sender<SimulationUpdate>,
    update_interval: time::Interval,
    instructions_channel: mpsc::Receiver<Instruction>,
    instruction_interval: time::Interval,
}

impl Simulation {
    pub fn new(
        channel: broadcast::Sender<SimulationUpdate>,
        update_interval_ms: time::Duration,
        instructions_channel: mpsc::Receiver<Instruction>,
        instruction_interval_ms: time::Duration,
        
    ) -> Self {
        let instruction_interval = time::interval(instruction_interval_ms);
        let update_interval = time::interval(update_interval_ms);
        Self {
            channel,
            update_interval,
            instructions_channel,
            instruction_interval,
        }
    }

    #[instrument(skip_all)]
    pub async fn run(&mut self, ctx: &mut SimulationContext) -> Result<()> {
        let (mut r_set, mut c_set, pawn_handle) = Self::initialize_world();
        let mut phys_pipeline = PhysicsPipeline::new();

        for _i in 0..10000 {
            //info!(step=_i);
            let res = select! {
                instruction = self.pop_instruction() => instruction,
                done = async {
                    Self::step(&mut phys_pipeline, &mut r_set, &mut c_set, ctx);
                    None
                } => done,
            };

            let body = &mut r_set[pawn_handle];

            if let Some(instruction) = res {
                instruction.apply(body);
            }

            let trans = body.translation();
            let rot = body.rotation();

            if trans.y < -2. {
                break;
            }
            
            self.channel
                .send(SimulationUpdate::PublicState(PublicState {
                    pawn_pos: [trans.x, trans.y, trans.z],
                    pawn_rot: [rot.i, rot.j, rot.k, rot.w],
                }))
                .map_err(|e| {
                    error!(err=%e, "Failed to send simulation update.");
                    e
                })?;

            //if _i % 20 == 0 {
            //    info!("{:?}", body.position().translation.y);
            //}
        }

        self.channel.send(SimulationUpdate::Done)?;
        Ok(())
    }

    #[instrument(skip_all)]
    async fn pop_instruction(&mut self) -> Option<Instruction> {
        //info!("Popping instruction");
        self.instruction_interval.tick().await;
        let out = self.instructions_channel.recv().await;
        info!("Popped instruction: {out:?}");
        out
    }
    
    #[instrument(skip_all)]
    fn step(
        phys_pipeline: &mut PhysicsPipeline,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        ctx: &mut SimulationContext,
    ) {
        phys_pipeline.step(
            &ctx.gravity,
            &ctx.integration_parameters,
            &mut ctx.island_manager,
            &mut ctx.broad_phase,
            &mut ctx.narrow_phase,
            rigid_body_set,
            collider_set,
            &mut ctx.impulse_joint_set,
            &mut ctx.multibody_joint_set,
            &mut ctx.ccd_solver,
            Some(&mut ctx.query_pipeline),
            &(),
            &(),
        );

 //       info!("step complete");
    }

    fn initialize_world() -> (RigidBodySet, ColliderSet, RigidBodyHandle) {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let ground =
            ColliderBuilder::cuboid(GROUND_DIM_HE[0], GROUND_DIM_HE[1], GROUND_DIM_HE[2]).build();
        collider_set.insert(ground);

        let pawn_rigid_boy = RigidBodyBuilder::dynamic()
            .translation(vector![PAWN_START[0], PAWN_START[1], PAWN_START[2]])
            .build();
        let pawn_handle = rigid_body_set.insert(pawn_rigid_boy);
        let pawn_collider = ColliderBuilder::cuboid(PAWN_DIM_HE[0], PAWN_DIM_HE[1], PAWN_DIM_HE[2])
            .mass(PAWN_MASS)
            .build();
        collider_set.insert_with_parent(pawn_collider, pawn_handle, &mut rigid_body_set);

        (rigid_body_set, collider_set, pawn_handle)
    }
}

pub struct SimulationContext {
    gravity: Vector3<f32>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
}

impl SimulationContext {
    pub fn new(gravity: Vector3<f32>) -> Self {
        Self {
            gravity,
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }
}

impl Default for SimulationContext {
    fn default() -> Self {
        Self {
            gravity: vector![0., 9.81, 0.],
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }
}
