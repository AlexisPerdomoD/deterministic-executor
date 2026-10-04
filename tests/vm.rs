use std::collections::HashMap;

use deterministic_executor::{
    instruction::{Instruction, Program, ProgramError, Value},
    state::{Account, State},
    vm::{VM, VMError},
};

fn program(instructions: Vec<Instruction>) -> Program {
    Program::with_instructions(instructions).expect("test program should be valid")
}

fn state(owner: Option<&str>, accounts: &[(&str, u128, u64)]) -> State {
    let accounts = accounts
        .iter()
        .map(|(code, bal, nonce)| {
            (
                (*code).to_owned(),
                Account {
                    bal: *bal,
                    nonce: *nonce,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let builder = State::builder().accounts(accounts);
    match owner {
        Some(code) => builder.owner_code(code.to_owned()).build(),
        None => builder.build(),
    }
}

fn run(instructions: Vec<Instruction>, state: State) -> Result<State, VMError> {
    match VM::builder()
        .program(program(instructions))
        .state(state)
        .build()
    {
        Ok(vm) => vm.run(),
        Err(_) => panic!("VM should build with a program"),
    }
}

fn successful_run(instructions: Vec<Instruction>, state: State) -> State {
    match run(instructions, state) {
        Ok(state) => state,
        Err(_) => panic!("test program should execute successfully"),
    }
}

#[test]
fn public_builder_requires_a_program() {
    assert!(matches!(
        VM::builder().build(),
        Err(VMError::ProgramNotProvided)
    ));
}

#[test]
fn arithmetic_programs_can_feed_transfers() {
    let cases = [
        (Instruction::Add, 10, 4, 14),
        (Instruction::Sub, 10, 4, 6),
        (Instruction::Mul, 10, 4, 40),
        (Instruction::MulPercentage, 250, 20, 50),
    ];

    for (operation, lhs, rhs, expected) in cases {
        let result = successful_run(
            vec![
                Instruction::Push(Value::Amount(lhs)),
                Instruction::Push(Value::Amount(rhs)),
                operation,
                Instruction::Push(Value::Code("receiver".to_owned())),
                Instruction::Transfer,
                Instruction::Halt,
            ],
            state(None, &[("receiver", 0, 0)]),
        );

        let receiver = result.account("receiver").unwrap();
        assert_eq!((receiver.bal, receiver.nonce), (expected, 0));
    }
}

#[test]
fn load_arithmetic_and_transfer_compose_into_a_deterministic_state_transition() {
    let result = successful_run(
        vec![
            Instruction::Push(Value::Code("source".to_owned())),
            Instruction::Load,
            Instruction::Push(Value::Amount(5)),
            Instruction::Add,
            Instruction::Push(Value::Code("receiver".to_owned())),
            Instruction::Transfer,
            Instruction::Halt,
        ],
        state(None, &[("source", 12, 3), ("receiver", 0, 8)]),
    );

    let source = result.account("source").unwrap();
    let receiver = result.account("receiver").unwrap();
    assert_eq!((source.bal, source.nonce), (12, 3));
    assert_eq!((receiver.bal, receiver.nonce), (17, 8));
}

#[test]
fn transfer_uses_owner_as_sender_and_updates_only_sender_nonce() {
    let result = successful_run(
        vec![
            Instruction::Push(Value::Amount(7)),
            Instruction::Push(Value::Code("receiver".to_owned())),
            Instruction::Transfer,
            Instruction::Halt,
        ],
        state(Some("owner"), &[("owner", 20, 5), ("receiver", 2, 11)]),
    );

    let owner = result.account("owner").unwrap();
    let receiver = result.account("receiver").unwrap();
    assert_eq!((owner.bal, owner.nonce), (13, 6));
    assert_eq!((receiver.bal, receiver.nonce), (9, 11));
}

#[test]
fn transfer_without_owner_code_credits_system_issued_value() {
    let result = successful_run(
        vec![
            Instruction::Push(Value::Amount(9)),
            Instruction::Push(Value::Code("receiver".to_owned())),
            Instruction::Transfer,
            Instruction::Halt,
        ],
        state(None, &[("receiver", 3, 4)]),
    );

    let receiver = result.account("receiver").unwrap();
    assert_eq!((receiver.bal, receiver.nonce), (12, 4));
}

#[test]
fn execution_returns_arithmetic_errors_through_the_public_api() {
    let result = run(
        vec![
            Instruction::Push(Value::Amount(u128::MAX)),
            Instruction::Push(Value::Amount(1)),
            Instruction::Add,
            Instruction::Halt,
        ],
        State::default(),
    );

    assert!(matches!(result, Err(VMError::ArithmeticValueOverflow)));
}

#[test]
fn public_errors_support_standard_rust_error_handling() {
    fn assert_error<T: std::error::Error>() {}

    assert_error::<ProgramError>();
    assert_error::<VMError>();
    assert!(!ProgramError::Empty.to_string().is_empty());
    assert!(!VMError::ProgramNotProvided.to_string().is_empty());
    assert!(!format!("{:?}", VMError::ProgramNotProvided).is_empty());
}

#[test]
fn external_client_example_can_propagate_errors_with_question_mark()
-> Result<(), Box<dyn std::error::Error>> {
    let accounts = HashMap::from([
        ("alice".to_owned(), Account { bal: 100, nonce: 0 }),
        ("bob".to_owned(), Account { bal: 20, nonce: 0 }),
    ]);
    let state = State::builder()
        .owner_code("alice".to_owned())
        .accounts(accounts)
        .build();
    let program = Program::with_instructions(vec![
        Instruction::Push(Value::Amount(30)),
        Instruction::Push(Value::Code("bob".to_owned())),
        Instruction::Transfer,
        Instruction::Halt,
    ])?;

    let result = VM::builder().program(program).state(state).build()?.run()?;

    assert_eq!(result.account("bob").unwrap().bal, 50);
    Ok(())
}
