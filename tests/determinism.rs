use std::collections::HashMap;

use deterministic_executor::{instruction, state, vm};

fn state(accounts: &[(&str, u128, u64)]) -> state::State {
    let accounts = accounts
        .iter()
        .map(|(code, bal, nonce)| {
            (
                (*code).to_owned(),
                state::Account {
                    bal: *bal,
                    nonce: *nonce,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    state::State::builder().accounts(accounts).build()
}

fn execute(state: state::State) -> state::State {
    let program = instruction::Program::with_instructions(vec![
        instruction::Instruction::Push(instruction::Value::Code("alice".to_owned())),
        instruction::Instruction::Load,
        instruction::Instruction::Push(instruction::Value::Amount(5)),
        instruction::Instruction::Add,
        instruction::Instruction::Push(instruction::Value::Code("bob".to_owned())),
        instruction::Instruction::Transfer,
        instruction::Instruction::Halt,
    ])
    .expect("program should be valid");

    match vm::VM::builder().program(program).state(state).build() {
        Ok(vm) => match vm.run() {
            Ok(state) => state,
            Err(_) => panic!("deterministic test program should succeed"),
        },
        Err(_) => panic!("VM should build with a program"),
    }
}

fn observable_balances(state: &state::State) -> [(u128, u64); 2] {
    let alice = state.account("alice").unwrap();
    let bob = state.account("bob").unwrap();
    [(alice.bal, alice.nonce), (bob.bal, bob.nonce)]
}

#[test]
fn equivalent_inputs_produce_the_same_observable_state_across_fresh_vms() {
    let first = execute(state(&[("alice", 12, 2), ("bob", 3, 7)]));
    let second = execute(state(&[("bob", 3, 7), ("alice", 12, 2)]));

    assert_eq!(observable_balances(&first), observable_balances(&second));
    assert_eq!(observable_balances(&first), [(12, 2), (20, 7)]);
}
