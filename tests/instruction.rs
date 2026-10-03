use deterministic_executor::instruction::{Instruction, Program, ProgramError};

#[test]
fn public_api_reports_empty_program() {
    let result = Program::with_instructions(vec![]);

    assert!(matches!(result, Err(ProgramError::Empty)));
}

#[test]
fn public_api_reports_missing_halt() {
    let result = Program::with_instructions(vec![Instruction::Add]);

    assert!(matches!(result, Err(ProgramError::MissingHalt)));
}

#[test]
fn public_api_reports_multiple_halts() {
    let result = Program::with_instructions(vec![Instruction::Halt, Instruction::Halt]);

    assert!(matches!(result, Err(ProgramError::MultipleHalt)));
}

#[test]
fn public_api_reports_halt_that_is_not_last() {
    let result = Program::with_instructions(vec![Instruction::Halt, Instruction::Add]);

    assert!(matches!(result, Err(ProgramError::HaltNotLast)));
}
