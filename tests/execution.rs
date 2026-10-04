use std::collections::HashMap;

use deterministic_executor::{instruction, state, vm};

fn state(owner: &str, owner_balance: u128, receiver_balance: u128) -> state::State {
    let accounts = HashMap::from([
        (
            owner.to_owned(),
            state::Account {
                bal: owner_balance,
                nonce: 4,
            },
        ),
        (
            "receiver".to_owned(),
            state::Account {
                bal: receiver_balance,
                nonce: 9,
            },
        ),
    ]);

    state::State::builder()
        .owner_code(owner.to_owned())
        .accounts(accounts)
        .build()
}

fn run(
    instructions: Vec<instruction::Instruction>,
    state: state::State,
) -> Result<state::State, vm::VMError> {
    let program =
        instruction::Program::with_instructions(instructions).expect("program should be valid");
    match vm::VM::builder().program(program).state(state).build() {
        Ok(vm) => vm.run(),
        Err(_) => panic!("VM should build with a program"),
    }
}

#[test]
fn successful_execution_returns_the_complete_updated_state() {
    let result = match run(
        vec![
            instruction::Instruction::Push(instruction::Value::Amount(30)),
            instruction::Instruction::Push(instruction::Value::Code("receiver".to_owned())),
            instruction::Instruction::Transfer,
            instruction::Instruction::Halt,
        ],
        state("owner", 100, 10),
    ) {
        Ok(state) => state,
        Err(_) => panic!("valid transfer should succeed"),
    };

    let owner = result.account("owner").unwrap();
    let receiver = result.account("receiver").unwrap();
    assert_eq!((owner.bal, owner.nonce), (70, 5));
    assert_eq!((receiver.bal, receiver.nonce), (40, 9));
}

#[test]
fn failed_execution_returns_only_an_error_and_preserves_the_callers_copy() {
    let original = state("owner", 100, 10);
    let execution_state = original.clone();

    let result = run(
        vec![
            instruction::Instruction::Push(instruction::Value::Amount(30)),
            instruction::Instruction::Push(instruction::Value::Code("receiver".to_owned())),
            instruction::Instruction::Transfer,
            instruction::Instruction::Push(instruction::Value::Amount(u128::MAX)),
            instruction::Instruction::Push(instruction::Value::Amount(1)),
            instruction::Instruction::Add,
            instruction::Instruction::Halt,
        ],
        execution_state,
    );

    assert!(matches!(result, Err(vm::VMError::ArithmeticValueOverflow)));
    let owner = original.account("owner").unwrap();
    let receiver = original.account("receiver").unwrap();
    assert_eq!((owner.bal, owner.nonce), (100, 4));
    assert_eq!((receiver.bal, receiver.nonce), (10, 9));
}
