use rapier3d::prelude::{ColliderSet, RigidBodyHandle, RigidBodySet};

pub mod level_one;

pub struct Level {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    pawn_handles: Vec<RigidBodyHandle>,
}

impl Level {
    pub fn new(
        rigid_body_set: RigidBodySet,
        collider_set: ColliderSet,
        pawn_handles: Vec<RigidBodyHandle>,
    ) -> Self {
        Self {
            rigid_body_set,
            collider_set,
            pawn_handles,
        }
    }

    pub fn get_rigid_body_set(&self) -> &RigidBodySet {
        &self.rigid_body_set
    }

    pub fn get_rigid_body_set_mut(&mut self) -> &mut RigidBodySet {
        &mut self.rigid_body_set
    }

    pub fn get_collider_set(&self) -> &ColliderSet {
        &self.collider_set
    }

    pub fn get_collider_set_mut(&mut self) -> &mut ColliderSet {
        &mut self.collider_set
    }

    pub fn get_pawn_handles(&self) -> &Vec<RigidBodyHandle> {
        &self.pawn_handles
    }

    pub fn get_pawn_handles_mut(&mut self) -> &mut Vec<RigidBodyHandle> {
        &mut self.pawn_handles
    }
}
