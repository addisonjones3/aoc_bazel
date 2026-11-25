use std::ops::{Add, Mul};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Sum(OpVals),
    Mult(OpVals),
    EndProgram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpVals {
    pub val_1: usize,
    pub val_2: usize,
    pub target_pos: usize,
}

impl TryFrom<&[usize]> for OpVals {
    type Error = PuterError;

    fn try_from(values: &[usize]) -> Result<Self, Self::Error> {
        match values {
            [val_1, val_2, target_pos] => Ok(Self {
                val_1: *val_1,
                val_2: *val_2,
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
    pub fn read_instruction_at(
        &self,
        instruction_index: usize,
        instruction_len: usize,
    ) -> Result<Instruction, PuterError> {
        let op_num = self
            .memory
            .get(instruction_index)
            .ok_or(PuterError::NoValueAtMemoryIndex(instruction_index))?;
        let op = Operation::try_from(*op_num)?;
        Ok(match op {
            add_or_mult @ (Operation::Sum | Operation::Mult) => {
                let indices = self
                    .read_values_at(instruction_index + 1, instruction_index + instruction_len)?;

                let vals = self.read_values_at(indices[0], indices[1] + 1)?;

                let op_vals = OpVals::try_from(vec![vals[0], vals[1], indices[2]].as_slice())?;
                match add_or_mult {
                    Operation::Sum => Instruction::Sum(op_vals),
                    Operation::Mult => Instruction::Mult(op_vals),
                    _ => unreachable!(),
                }
            }
            Operation::EndProgram => Instruction::EndProgram,
        })
    }

    pub fn apply_instruction(&mut self, instr: Instruction) -> Result<(), PuterError> {
        let (new_val, target_pos) = match instr {
            Instruction::Sum(op_vals) => (op_vals.val_1.add(op_vals.val_2), op_vals.target_pos),
            Instruction::Mult(op_vals) => (op_vals.val_1.mul(op_vals.val_2), op_vals.target_pos),
            Instruction::EndProgram => return Ok(()),
        };

        let target_val = self
            .memory
            .get_mut(target_pos)
            .ok_or(PuterError::NoValueAtMemoryIndex(target_pos))?;

        *target_val = new_val;

        Ok(())
    }

    fn read_value_at(&self, index: usize) -> Result<usize, PuterError> {
        Ok(self
            .memory
            .get(index)
            .copied()
            .ok_or(PuterError::NoValuesAtMemoryRange))?
    }

    fn read_values_at(&self, begin: usize, end: usize) -> Result<Vec<usize>, PuterError> {
        Ok((begin..end)
            .into_iter()
            .map(|i| self.memory.get(i).copied())
            .collect::<Option<_>>()
            .ok_or(PuterError::NoValuesAtMemoryRange))?
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INSTR_1: Instruction = Instruction::Sum(OpVals {
        val_1: 30,
        val_2: 40,
        target_pos: 3,
    });

    fn simple_exec() -> Executor {
        Executor { memory: vec![1] }
    }

    fn full_exec() -> Executor {
        Executor {
            memory: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        }
    }

    #[test]
    fn test_execute_add() {
        let mut exec = simple_exec();
        let op_vals = OpVals {
            val_1: 2,
            val_2: 3,
            target_pos: 0,
        };

        exec.apply_instruction(Instruction::Sum(op_vals))
            .expect("should apply");

        let add_val = exec.read_value_at(0).expect("should get value");

        assert_eq!(add_val, 5);

        exec.apply_instruction(Instruction::Mult(op_vals))
            .expect("should apply");

        let mult_val = exec.read_value_at(0).expect("should get value");

        assert_eq!(mult_val, 6);
    }

    #[test]
    fn test_read_and_apply_instruction() {
        let mut exec = full_exec();

        let instr_1 = exec
            .read_instruction_at(0, 4)
            .expect("expected instruction");
        assert_eq!(instr_1, INSTR_1);

        exec.apply_instruction(instr_1)
            .expect("expected apply success");

        let new_val = exec.read_value_at(3).expect("couldn't get new val");
        assert_eq!(new_val, 70);
    }
}
