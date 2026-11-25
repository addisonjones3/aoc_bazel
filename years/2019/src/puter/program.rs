use anyhow::{Error, anyhow};

#[derive(Debug)]
pub enum PuterError {
    BadInstructionRead(Error),
    InvalidInstructionValue(usize),
    NoValueAtMemoryIndex(usize),
    NoValuesAtMemoryRange,
    InvalidOpVals(Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Sum,
    Mult,
    EndProgram,
}

impl TryFrom<usize> for Operation {
    type Error = PuterError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Operation::Sum),
            2 => Ok(Operation::Mult),
            99 => Ok(Operation::EndProgram),
            _ => Err(PuterError::InvalidInstructionValue(value)),
        }
    }
}

pub enum Instruction {
    Sum(OpVals),
    Mult(OpVals),
    EndProgram,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpVals {
    pub pos_1: usize,
    pub pos_2: usize,
    pub target_pos: usize,
}

impl TryFrom<&[usize]> for OpVals {
    type Error = PuterError;

    fn try_from(values: &[usize]) -> Result<Self, Self::Error> {
        match values {
            [pos_1, pos_2, target_pos] => Ok(Self {
                pos_1: *pos_1,
                pos_2: *pos_2,
                target_pos: *target_pos,
            }),
            _ => Err(PuterError::InvalidOpVals(anyhow!(
                "could not create OpVals from {values:?}"
            ))),
        }
    }
}

#[derive(Debug)]
pub struct Executor {
    pub memory: Vec<usize>,
}

impl Executor {
    pub fn read_instruction_at(&self, instruction_index: usize) -> Result<Instruction, PuterError> {
        let op_num = self
            .memory
            .get(instruction_index)
            .ok_or(PuterError::NoValueAtMemoryIndex(instruction_index))?;
        let op = Operation::try_from(*op_num)?;
        Ok(match op {
            add_or_mult @ (Operation::Sum | Operation::Mult) => {
                let vals = self.read_values_at(instruction_index + 1, instruction_index + 4)?;
                let op_vals = OpVals::try_from(vals.as_slice())?;
                match add_or_mult {
                    Operation::Sum => Instruction::Sum(op_vals),
                    Operation::Mult => Instruction::Mult(op_vals),
                    _ => unreachable!(),
                }
            }
            Operation::EndProgram => Instruction::EndProgram,
        })
    }

    fn read_values_at(&self, begin: usize, end: usize) -> Result<Vec<usize>, PuterError> {
        Ok((begin + 1..end)
            .into_iter()
            .map(|i| self.memory.get(i).copied())
            .collect::<Option<_>>()
            .ok_or(PuterError::NoValuesAtMemoryRange))?
    }
}
