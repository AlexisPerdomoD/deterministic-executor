# Deterministic Executor

A small deterministic execution engine provided as a Rust library.

It executes a structurally validated, finite program against in-memory state. A successful run returns the resulting state; a failed run returns an error and no resulting state. The caller owns any original copy it needs to retain.

## Purpose

This is a systems-development learning project. Its focus is on:

- bytecode interpretation and stack-based execution;
- ownership of mutable state and controlled mutation;
- program validation, failure semantics, and non-partial results;
- deterministic execution results and explicit state-transition semantics;
- Rust ownership, error modeling, testing, and later concurrency trade-offs.

The system is deliberately small so that its behavior, invariants, and costs can be understood precisely.

## VM responsibility boundary

The VM executes a validated program against an input state and returns the resulting state on success. Its responsibility is to apply instruction-defined transitions deterministically and enforce their local invariants: checked arithmetic, required account references, and consistent balance/nonce updates.

The VM does not decide whether an operation is authorized by an external policy. If `owner_code` is absent, execution may credit the receiver as a system-issued value; the VM neither requires nor invents a sender account. The component requesting execution is responsible for deciding whether that request is authorized.

Authorization, state persistence, and transport are outside the engine. They must not be implicitly attributed to the interpreter or confused with the semantic validity of a state transition.

These boundaries are reflected in the tests: with `owner_code`, a transfer debits the owner, credits the receiver, and increments only the owner's nonce; without it, a transfer credits the receiver without a sender debit or nonce change. A failed transfer leaves account balances and nonces unchanged. Integration tests exercise the public API and returned state; internal tests may inspect private handlers. The stack is private, and its contents after an error are not part of the VM's result contract.

## Initial scope

Version 1 provides:

- a compact instruction set and validated programs that end with `Halt`;
- an in-memory account state with balances and nonces;
- a stack VM with checked arithmetic, account loading, transfers, and explicit errors;
- a `Result<State, VMError>` execution contract;
- unit and integration tests for instruction behavior, state transitions, errors, and repeatability.

## Non-goals

The initial version is not a server, blockchain, wallet, database, full programming language, consensus system, or parallel executor. It has no TCP/HTTP transport, parser, persistence, authorization policy, transaction-level nonce/replay validation, separate execution receipt, state digest, or configurable instruction/gas limit.

Authorization is an external responsibility. When `owner_code` is absent, the VM permits a system-issued credit; it does not decide whether the caller is authorized to request one. Other excluded capabilities can be separate experiments after the library contract is complete and understood.

## Public API

| Module        | Main public types                                 | Purpose                                                            |
| ------------- | ------------------------------------------------- | ------------------------------------------------------------------ |
| `instruction` | `Value`, `Instruction`, `Program`, `ProgramError` | Define instructions and construct structurally validated programs. |
| `state`       | `Account`, `State`, `StateBuilder`                | Create an input state and inspect account balances/nonces.         |
| `vm`          | `VM`, `VMBuilder`, `VMError`                      | Build and run a VM, returning `Result<State, VMError>`.            |

## Example: execute from another crate

The crate is consumed as `deterministic_executor` in Rust code. This example builds a state and program, executes a transfer, and reads the returned state:

```rust
use std::{collections::HashMap, error::Error};

use deterministic_executor::{
    instruction::{Instruction, Program, Value},
    state::{Account, State},
    vm::VM,
};

fn transfer() -> Result<u128, Box<dyn Error>> {
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
    Ok(result.account("bob").expect("bob was created above").bal)
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(transfer()?, 50);
    Ok(())
}
```

## Build and verify

```sh
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

## Repository structure

```text
src/
├── lib.rs
├── instruction.rs
├── state.rs
└── vm.rs
tests/
├── instruction.rs
├── vm.rs
├── execution.rs
└── determinism.rs
```
