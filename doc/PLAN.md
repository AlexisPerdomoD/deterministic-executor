# Systems Development Roadmap

## Purpose

This repository is a technical learning space, not an interview-preparation checklist and not an attempt to become a blockchain or Rust specialist quickly.

The goal is to extend an existing backend foundation toward systems-oriented reasoning:

- where state lives and who may mutate it;
- representation, ownership, copying and allocation;
- execution and failure semantics;
- deterministic behavior;
- concurrency, shared state and contention;
- observable costs: CPU work, memory, I/O and synchronization.

Rust is the implementation language for the main project because it makes memory and concurrency decisions explicit. The concepts should remain transferable to Go, which is already the stronger language for implementation and future pair-coding work.

## Project: Deterministic State-Transition Library

Build a small bytecode interpreter that executes validated programs against an in-memory account state.

It is deliberately **not** a blockchain, database, programming language, or zero-knowledge system. It is a compact system through which to study how an execution engine works.

```text
program + input state
           |
           v
     program validation + execution
           |
           v
       output state or error
```

## Context: what are we building?

In friendly terms, this project is a small Rust library that executes a validated finite program over in-memory state in a predictable way.

It receives a program and an owned state, checks the program structure, runs it, and either returns a new state or an error without exposing a resulting state. The caller retains any original state copy it needs.

```text
Initial state:
Alice: 100
Bob:    20

Program and state:
owner_code = Alice
PUSH Amount(30)
PUSH Code("Bob")
TRANSFER
HALT

Final state:
Alice:  70
Bob:    50
```

The sender is the optional `owner_code` in the input state. Without one, `Transfer` can credit the receiver as a system-issued value; authorization to request that execution is external to this library.

This is not meant to be a complete programming language. There is no user-facing syntax, parser, compiler, class system, garbage collector, peer-to-peer network, consensus protocol, or cryptographic proof system.

### Why systems use this pattern

The same basic model appears whenever software needs to apply defined rules to changing data and make the result reproducible:

- **Financial systems:** apply operations atomically, without persisting half of a failed operation.
- **Blockchains and smart-contract runtimes:** independent machines must obtain the same state from the same transactions.
- **Rule engines and workflows:** configurable steps validate inputs and advance a process through valid states.
- **Simulations and games:** actions change a shared world according to deterministic rules.
- **Replicated/distributed systems:** a sequence of commands can be replayed to reconstruct the same state.

The project implements the minimal shared pattern, not any complete product in those categories:

```text
input + rules + current state
             |
             v
     controlled execution
             |
             v
new state or explicit error
```

### Core parts

| Part                | Responsibility                                                        | Question it exposes                                |
| ------------------- | --------------------------------------------------------------------- | -------------------------------------------------- |
| **State**           | Holds accounts, balances, nonces, and optional `owner_code`.          | Where does mutable data live?                      |
| **Program**         | A finite instruction sequence validated to end with one `Halt`.       | Which instruction sequences may execute?           |
| **Instruction set** | Defines the small, fixed vocabulary understood by the VM.             | What operations can the machine perform?           |
| **VM/interpreter**  | Runs the fetch/execute loop and returns `Result<State, VMError>`.     | How do instructions change execution-local state?  |
| **Tests**           | Check transitions, errors, and repeatability through the library API. | Does equal input produce equal observable results? |

The VM is the engine and library API in this version. There is no separate server, transaction executor, receipt pipeline, persistence layer, or digest subsystem.

### Project purpose

The purpose is not to build a token-transfer demo or claim to have built a blockchain. It is to have a small enough system to reason precisely about:

- who owns state and which runtime operations may mutate it;
- how an operation fails without corrupting state;
- what must be deterministic and which implementation details can break determinism;
- what is copied, borrowed, allocated, or later shared;
- where costs arise and what changes when execution becomes concurrent.

### Why this project

It is a good successor to Redix:

| Redix already demonstrates            | This project adds                               |
| ------------------------------------- | ----------------------------------------------- |
| networking and a custom wire protocol | instruction execution and VM state              |
| concurrent connections                | controlled shared-state/concurrency experiments |
| in-memory data structures             | deterministic state transitions                 |
| AOF/RDB persistence                   | in-memory state transitions                     |
| Go systems implementation             | Rust ownership and error modeling               |

The project is small enough to finish, but produces tangible artifacts for a portfolio: a tested library, a clear contract, and notes explaining its technical trade-offs.

## Explicit non-goals

Do not add these to the initial project:

- networking, HTTP APIs, UI, authentication or databases;
- consensus, peer-to-peer communication or a token;
- a parser, compiler, garbage collector or full programming language;
- real cryptographic proof generation or zkVM implementation;
- premature parallel execution;
- generic abstractions that have no immediate use.

Each can be a separate future experiment only after the single-threaded engine is complete and understood.

## Definition of done: version 1

Version 1 is complete when it can:

1. Execute a compact, fixed instruction set using a program counter and private stack.
2. Read and update a finite in-memory account state.
3. Validate program structure before execution: non-empty, exactly one final `Halt`.
4. Return `Result<State, VMError>`; on failure return no resulting state, while any caller-retained input copy remains unchanged.
5. Apply checked arithmetic and the defined transfer contracts: owner transfers debit the owner and increment its nonce; ownerless transfers may credit as system execution.
6. Produce the same observable balances and nonces for equal program and input-state contents across fresh VM instances.
7. Cover these contracts with unit and integration tests and document ownership, invariants, costs, and non-goals.

Authorization policy, transaction-level nonce/replay validation, a separate receipt type, state digest/canonical serialization, persistence, and transport are outside version 1.

## Minimal model

### State

Use an in-memory map of account codes to balances and nonces. The VM reads accounts by key and does not expose map iteration or a serialized state format.

```text
State = { AccountCode -> (Balance, Nonce) }, optional owner_code
```

### Execution input

The library executes a validated program against an owned state. It does not model a transaction envelope or validate a transaction nonce in this version.

```text
Execution = {
  state,
  program
}
```

### Instruction set

The current set is `Push(Amount|Code)`, `Add`, `Sub`, `Mul`, `MulPercentage`, `Load`, `Transfer`, and `Halt`.

Every instruction should have specified stack effects and failure cases (underflow, overflow, insufficient balance, invalid program counter). Keep arithmetic policy explicit; checked arithmetic is a sensible default.

### Important invariants

- A successful run returns the resulting state; a failed run returns only an error, not a partial resulting state.
- Balances never become negative.
- A successful owner transfer debits the owner, credits the receiver, and increments the owner's account nonce.
- A transfer without `owner_code` may credit the receiver; the VM does not enforce authorization.
- Every valid program is finite and ends in `Halt`; there is no configurable instruction/gas or stack limit.
- Equal program and input-state contents produce equal observable balances and nonces; no canonical encoding or digest is required.

## Delivery sequence

### Milestone 0 — Rust orientation (small experiments)

Write short, disposable experiments before beginning the engine:

- owned `String` versus borrowed `&str`;
- `Vec<T>` growth and slices;
- `Result` and custom error enums;
- `HashMap` versus `BTreeMap` and why iteration order matters;
- moving data into a thread; `Send`, `Sync`, `Arc` and `Mutex`.

The outcome is not a collection of exercises. It is a concise note for each experiment: what was owned, borrowed, copied, allocated, and rejected by the compiler.

### Milestone 1 — Program and stack VM

- Define `Instruction`, validated `Program`, `VM`, and `VMError`.
- Implement fetch/execute around a program counter and private operand stack.
- Test instruction effects, stack errors, checked arithmetic, loads, and transfers.
- Keep programs finite with a final `Halt`; a configurable execution budget is out of scope.

**Question to answer:** What data belongs to the program, the VM, and an individual execution?

### Milestone 2 — State-transition contract

- Define state and account types, including the optional owner context.
- Enforce balance, account-nonce increment, and arithmetic constraints during execution.
- Return the new state on success and no state value on error.
- Keep authorization, transaction replay checks, persistence, transport, and receipt types outside this version.

**Question to answer:** Where is the atomicity boundary, and what is copied or mutated to preserve it?

### Milestone 3 — Deterministic behavior

- Test repeatability across fresh VM instances with equal program and state contents.
- Compare observable balances and nonces; canonical serialization and a digest are not required.
- Add benchmarks only if a concrete performance question becomes part of the project scope.

**Question to answer:** Which implementation details can make apparently identical execution produce different results?

### After version 1

Do not add extensions as part of closing this project. Batch execution, state commitments, and persistence would each require a separate scope and contract.

## Suggested repository shape

```text
deterministic-executor/
├── README.md
├── doc/PLAN.md
├── src/
│   ├── lib.rs
│   ├── instruction.rs
│   ├── state.rs
│   └── vm.rs
├── tests/
│   ├── instruction.rs
│   ├── vm.rs
│   ├── execution.rs
│   └── determinism.rs
```

Start as one Rust crate. Do not split it into workspace crates until a real boundary requires it.

## How to work on it

Use a short feedback loop:

```text
learn one concept -> build the smallest version -> write its invariant/test
-> observe a limitation -> research the reason -> document the decision
```

For every milestone, answer these questions in the README or a short note:

- What is the unit of execution?
- Where does mutable state live?
- Who owns it during execution?
- What is copied versus shared or borrowed?
- What errors can occur and does the state remain valid?
- What is deterministic and what could accidentally make it non-deterministic?
- What cost would become important at 10x or 100x the workload?

## Resources

### Core

- [The Rust Programming Language](https://doc.rust-lang.org/book/): prioritize chapters 4 (ownership), 6 (enums), 8 (collections), 9 (error handling), 10 (generics/traits), 13 (iterators/closures), 15–16 (smart pointers/concurrency), and 18 (patterns).
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/): short executable complements to the book.
- [Crafting Interpreters](https://craftinginterpreters.com/): use the bytecode VM chapters to understand the fetch/decode/execute loop and bytecode representation. Do not treat finishing the book or implementing Lox as a prerequisite.

### Video resource

- [“Crafting Interpreters: Day 1” — Uncle Scientist](https://www.youtube.com/watch?v=WdoAJ_ouWRM): valid companion resource because it maps _Crafting Interpreters_ ideas into Rust. Follow it selectively; pause to implement and explain each part yourself rather than reproducing a tutorial project line by line.

### Focused follow-up

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/): consult only when measuring a concrete performance question.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/): reference material for later; do not begin with `unsafe` or advanced memory internals.
- [Go memory model](https://go.dev/ref/mem): revisit after the Rust concurrency experiments to compare the guarantees and trade-offs of both languages.

## Portfolio framing

Describe the project honestly as:

> A small Rust library for deterministic bytecode execution over in-memory state, built to study ownership, checked state transitions, and execution failure semantics.

Avoid presenting it as a blockchain, production VM, or cryptographic verifier. The portfolio value is the quality of the scope, tests, documentation and trade-off reasoning—not the size of the codebase.

## Relationship to `plan.md`

`plan.md` remains useful as a broad map of topics. This document is the operational plan: it narrows the work to a finishable project, moves advanced verification and blockchain-specific material out of version 1, and removes interview-timing pressure from the technical learning path.
