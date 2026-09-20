use deterministic_executor::instruction::{Instruction, Program, ProgramError};

#[test]
fn public_api_creates_a_valid_program_and_exposes_its_length() {
    let instructions = vec![
        Instruction::Push { value: 10 },
        Instruction::Push { value: 2 },
        Instruction::Sub,
        Instruction::Halt,
    ];

    let program = Program::from_instructions(instructions).expect("program should be valid");

    assert_eq!(program.len(), 4);
}

#[test]
fn public_api_reports_empty_program() {
    let result = Program::from_instructions(vec![]);

    assert!(matches!(result, Err(ProgramError::Empty)));
}

#[test]
fn public_api_reports_missing_halt() {
    let result = Program::from_instructions(vec![Instruction::Add]);

    assert!(matches!(result, Err(ProgramError::MissingHalt)));
}

#[test]
fn public_api_reports_multiple_halts() {
    let result = Program::from_instructions(vec![Instruction::Halt, Instruction::Halt]);

    assert!(matches!(result, Err(ProgramError::MultipleHalt)));
}

#[test]
fn public_api_reports_halt_that_is_not_last() {
    let result = Program::from_instructions(vec![Instruction::Halt, Instruction::Add]);

    assert!(matches!(result, Err(ProgramError::HaltNotLast)));
}
