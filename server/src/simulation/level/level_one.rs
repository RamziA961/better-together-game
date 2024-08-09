use std::f32::consts::PI;

use nalgebra::vector;
use rapier3d::prelude::{nalgebra, ColliderBuilder, ColliderSet, RigidBodyBuilder, RigidBodySet};

use super::Level;

// half extents
const GROUND_DIM_HE: [f32; 3] = [40., 40., 0.1];

const PAWN_DIM_HE: [f32; 3] = [5.; 3];
const PAWN_START: [f32; 3] = [0., 20., 0.];
const PAWN_MASS: f32 = 20.;

const MAX_ANGULAR_VEL: f32 = 2. * PI;
const MAX_LINEAR_VEL: f32 = 10.;

pub struct LevelOne {
    level: Level,
}

impl LevelOne {
    pub fn new() -> Self {
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

        Self {
            level: Level::new(rigid_body_set, collider_set, vec![pawn_handle]),
        }
    }

    pub fn get_level(&self) -> &Level {
        &self.level
    }

    pub fn get_level_mut(&mut self) -> &mut Level {
        &mut self.level
    }
}
