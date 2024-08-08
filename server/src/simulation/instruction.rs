use crate::updates::Instruction;

pub trait InstructionHandler {
    fn apply_instruction(&mut self, instruction: Instruction);
}

/*
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
*/
