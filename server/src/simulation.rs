use std::f32::consts::PI;

use crate::updates::{Coordinates, InstructionUpdate, Orientation, SimulationUpdate, SpatialData};
use anyhow::Result;
use nalgebra::{vector, Vector3};
use rapier3d::prelude::*;
use tokio::{
    select,
    sync::{broadcast, mpsc},
    time,
};
use tracing::{error, info, instrument};

pub mod instruction;
pub mod level;

// half extents
const GROUND_DIM_HE: [f32; 3] = [50., 0.05, 50.];

const PAWN_DIM_HE: [f32; 3] = [5.; 3];
const PAWN_START: [f32; 3] = [0., 20., 0.];
const PAWN_MASS: f32 = 20.;

const MAX_ANGULAR_VEL: f32 = 2. * PI;
const MAX_LINEAR_VEL: f32 = 10.;

#[derive(Debug, Clone)]
enum Action {
    ApplyInstruction,
    SendUpdate,
}

pub struct Simulation {
    channel: broadcast::Sender<SimulationUpdate>,
    update_interval: time::Duration,
    instructions_channel: mpsc::Receiver<InstructionUpdate>,
    instruction_interval: time::Duration,
}

impl Simulation {
    pub fn new(
        channel: broadcast::Sender<SimulationUpdate>,
        update_interval_ms: time::Duration,
        instructions_channel: mpsc::Receiver<InstructionUpdate>,
        instruction_interval_ms: time::Duration,
    ) -> Self {
        Self {
            channel,
            update_interval: update_interval_ms,
            instructions_channel,
            instruction_interval: instruction_interval_ms,
        }
    }

    #[instrument(skip_all)]
    pub async fn run(&mut self, ctx: &mut SimulationContext) -> Result<()> {
        let (mut r_set, mut c_set, pawn_handle) = Self::initialize_world();
        let mut phys_pipeline = PhysicsPipeline::new();

        let mut update_interval = time::interval(self.update_interval);
        let mut ins_interval = time::interval(self.instruction_interval);

        for _i in 0.. {
            let res = select! {
                biased;
                _ = update_interval.tick() => {
                    Self::step(&mut phys_pipeline, &mut r_set, &mut c_set, ctx);
                    Some(Action::SendUpdate)
                },
                _ = ins_interval.tick() => Some(Action::ApplyInstruction),
            };

            let should_log = false;
            if should_log {
                info!(step = _i, "Simulation loop ongoing");
                info!(channel_size = self.channel.len());
                info!("Action: {res:?}");
            }

            let body = &mut r_set[pawn_handle];
            let trans = body.translation();

            if trans.y < -10. {
                //break;
                body.set_position(
                    Isometry::new(
                        Vector3::new(PAWN_START[0], PAWN_START[1], PAWN_START[2]),
                        *body.position().rotation.axis_angle().unwrap().0,
                    ),
                    true,
                );

                body.set_linvel(vector![0., 0., 0.], true);
            }

            match res {
                Some(Action::ApplyInstruction) => {
                    self.send_update(body, should_log)?;
                }
                Some(Action::SendUpdate) => {
                    self.send_update(body, should_log)?;
                }
                _ => {}
            }
        }

        info!("Simulation loop complete");
        self.channel.send(SimulationUpdate {
            spatial_updates: vec![],
            done: Some(true),
        })?;

        Ok(())
    }

    #[instrument(skip_all)]
    fn send_update(&mut self, body: &mut RigidBody, should_log: bool) -> Result<()> {
        // takes a single body but want to support any number in future
        let trans = body.translation();
        let rot = body.rotation();

        let coor = Coordinates {
            x: trans.x,
            y: trans.y,
            z: trans.z,
        };

        let orient = Orientation {
            i: rot.i,
            j: rot.j,
            k: rot.k,
            w: rot.w,
        };

        let spatial_data = SpatialData {
            id: 1,
            coordinates: Some(coor),
            orientation: Some(orient),
        };

        let sim_up = SimulationUpdate {
            spatial_updates: vec![spatial_data],
            done: None,
        };

        if should_log {
            info!(
                channel_len = self.channel.len(),
                "Sending update to simulation channel {sim_up:?}"
            );
        }

        self.channel.send(sim_up).map(|_| ()).map_err(|e| {
            error!(err=%e, "Failed to send simulation update.");
            anyhow::anyhow!(e)
        })
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
    }

    fn initialize_world() -> (RigidBodySet, ColliderSet, RigidBodyHandle) {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let ground_rb = RigidBodyBuilder::fixed().build();
        let ground_handle = rigid_body_set.insert(ground_rb);
        let ground = ColliderBuilder::cuboid(GROUND_DIM_HE[0], GROUND_DIM_HE[1], GROUND_DIM_HE[2])
            .restitution(1.)
            .build();

        collider_set.insert_with_parent(ground, ground_handle, &mut rigid_body_set);

        let pawn_rigid_boy = RigidBodyBuilder::dynamic()
            .translation(vector![PAWN_START[0], PAWN_START[1], PAWN_START[2]])
            .linear_damping(0.)
            .build();
        let pawn_handle = rigid_body_set.insert(pawn_rigid_boy);
        let pawn_collider = ColliderBuilder::cuboid(PAWN_DIM_HE[0], PAWN_DIM_HE[1], PAWN_DIM_HE[2])
            .mass(PAWN_MASS)
            .restitution(1.)
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
            gravity: vector![0., -9.81, 0.],
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
