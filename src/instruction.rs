#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Amount(u128),
    Code(String),
}

/// An instruction is an operation on the stack.
/// This is the valid instruction set for the deterministic executor.
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Push(Value),
    Halt,
    Load,
    Transfer,
    Add,
    Sub,
    Mul,
    MulPercentage,
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
    /// with_instructions creates a new Program from a sequence of instructions.
    /// It validates the instructions before creating the Program. So Valid Program is ensured when this function return Ok.
    pub fn with_instructions(instructions: Vec<Instruction>) -> Result<Self, ProgramError> {
        let res = Self::new(instructions);
        Program::validate(&res)?;
        Ok(res)
    }

    /// returns an iterator from program instructions as readonly
    pub fn iter(&self) -> impl Iterator<Item = &Instruction> {
        self.instructions.iter()
    }

    /// returns the readonly instruction based on explicit index
    pub fn get(&self, idx: usize) -> Option<&Instruction> {
        self.instructions.get(idx)
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    fn validate(inst: &Self) -> Result<(), ProgramError> {
        if inst.instructions.is_empty() {
            return Err(ProgramError::Empty);
        }

        let mut found_halt = false;
        for inst in inst.instructions.iter() {
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

        if inst.instructions.last() != Some(&Instruction::Halt) {
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
        let program = Program::new(vec![Instruction::Push(Value::Amount(7)), Instruction::Halt]);

        assert!(matches!(Program::validate(&program), Ok(())));
        assert_eq!(program.instructions.len(), 2);
    }

    #[test]
    fn validate_rejects_empty_program() {
        let program = Program::new(vec![]);

        assert!(matches!(
            Program::validate(&program),
            Err(ProgramError::Empty)
        ));
    }

    #[test]
    fn validate_rejects_program_without_halt() {
        let program = Program::new(vec![Instruction::Add]);

        assert!(matches!(
            Program::validate(&program),
            Err(ProgramError::MissingHalt)
        ));
    }

    #[test]
    fn validate_rejects_program_with_multiple_halts() {
        let program = Program::new(vec![Instruction::Halt, Instruction::Halt]);

        assert!(matches!(
            Program::validate(&program),
            Err(ProgramError::MultipleHalt)
        ));
    }

    #[test]
    fn validate_rejects_program_when_halt_is_not_last() {
        let program = Program::new(vec![Instruction::Halt, Instruction::Sub]);

        assert!(matches!(
            Program::validate(&program),
            Err(ProgramError::HaltNotLast)
        ));
    }
}
