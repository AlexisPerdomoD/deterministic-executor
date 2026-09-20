/// An instruction is an operation on the stack.
/// This is the valid instruction set for the deterministic executor.
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Push { value: i64 },
    Halt,
    Add,
    Sub,
}

/// An error that can occur when validating a program.
#[derive(Debug, PartialEq, Eq)]
pub enum ProgramError {
    Empty,
    MissingHalt,
    MultipleHalt,
    HaltNotLast,
}

/// A program is a sequence of instructions.
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    /// from_instructions creates a new Program from a sequence of instructions.
    /// It validates the instructions before creating the Program. So Valid Program is ensured when this function return Ok.
    pub fn from_instructions(instructions: Vec<Instruction>) -> Result<Self, ProgramError> {
        let res = Self::new(instructions);
        match res.validate() {
            Ok(()) => Ok(res),
            Err(val) => Err(val),
        }
    }

    /// size returns the number of instructions in the program.
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    fn validate(&self) -> Result<(), ProgramError> {
        if self.instructions.is_empty() {
            return Err(ProgramError::Empty);
        }

        let mut found_halt = false;
        for inst in self.instructions.iter() {
            if inst != &Instruction::Halt {
                continue;
            }

            if found_halt {
                return Err(ProgramError::MultipleHalt);
            }

            found_halt = true;
        }

        if !found_halt {
            return Err(ProgramError::MissingHalt);
        }

        if self.instructions.last() != Some(&Instruction::Halt) {
            return Err(ProgramError::HaltNotLast);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_accepts_program_that_ends_with_halt() {
        let program = Program::new(vec![Instruction::Push { value: 7 }, Instruction::Halt]);

        assert!(matches!(program.validate(), Ok(())));
        assert_eq!(program.len(), 2);
    }

    #[test]
    fn validate_rejects_empty_program() {
        let program = Program::new(vec![]);

        assert!(matches!(
            program.validate(),
            Err(ProgramError::Empty)
        ));
    }

    #[test]
    fn validate_rejects_program_without_halt() {
        let program = Program::new(vec![Instruction::Add]);

        assert!(matches!(
            program.validate(),
            Err(ProgramError::MissingHalt)
        ));
    }

    #[test]
    fn validate_rejects_program_with_multiple_halts() {
        let program = Program::new(vec![Instruction::Halt, Instruction::Halt]);

        assert!(matches!(
            program.validate(),
            Err(ProgramError::MultipleHalt)
        ));
    }

    #[test]
    fn validate_rejects_program_when_halt_is_not_last() {
        let program = Program::new(vec![Instruction::Halt, Instruction::Sub]);

        assert!(matches!(
            program.validate(),
            Err(ProgramError::HaltNotLast)
        ));
    }
}
