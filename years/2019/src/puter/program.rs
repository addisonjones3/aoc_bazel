use anyhow::{Error, anyhow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Sum,
    Mult,
    // EndProgram,
}

impl TryFrom<u8> for Operation {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Operation::Sum),
            2 => Ok(Operation::Mult),
            // 99 => Ok(Operation::EndProgram),
            _ => Err(anyhow!("Operation not implemented for {value}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub operation: Operation,
    pub pos_1: usize,
    pub pos_2: usize,
    pub target_pos: usize,
}

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub slots: Vec<usize>,
}
