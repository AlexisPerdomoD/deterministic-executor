# Systems Development Roadmap

> [ !NOTE ]
> El producto es un motor determinista de ejecución y transición de estado; el intérprete es su núcleo educativo y operativo.

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

## Project: Deterministic State Machine

Build a small bytecode interpreter that executes transactions against an in-memory key-value state.

It is deliberately **not** a blockchain, database, programming language, or zero-knowledge system. It is a compact system through which to study how an execution engine works.

```text
transaction + input state
           |
           v
     validation + execution
           |
           v
 result + output state + state digest
```

## Context: what are we building?

In friendly terms, this project is a small engine that executes instructions over a state in a predictable and safe way.

It receives a transaction containing a small program, checks that the operation is valid, runs it, and either produces a new state or returns an error without leaving partial changes behind.

```text
Initial state:
Alice: 100
Bob:    20

Transaction:
move 30 units from Alice to Bob

Final state:
Alice:  70
Bob:    50
```

The operation can be represented as executable instructions rather than as one hard-coded `transfer()` call:

```text
LOAD Alice
PUSH 30
TRANSFER Bob
HALT
```

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

| Part                 | Responsibility                                                                                      | Question it exposes                                     |
| -------------------- | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| **State**            | Holds the data that may change: accounts, balances, and nonces.                                     | Where does mutable data live?                           |
| **Transaction**      | Carries an intended operation and its instruction program.                                          | What is the unit of work?                               |
| **Validation**       | Rejects malformed, stale, unauthorized, or impossible work before execution.                        | Which inputs are valid?                                 |
| **Instruction set**  | Defines the small, fixed vocabulary of operations the engine understands.                           | What can the machine do?                                |
| **VM/interpreter**   | Runs the fetch/decode/execute loop with a program counter and operand stack.                        | How do instructions change execution-local state?       |
| **Executor**         | Establishes the atomic boundary: commits successful work or keeps the original state after failure. | When do changes become visible?                         |
| **Receipt**          | Describes the observable result: success/failure, error, and resource use.                          | What happened during execution?                         |
| **Canonical digest** | Hashes a stable representation of output state.                                                     | Can another execution compare or reproduce this result? |

The **VM/interpreter is an internal execution mechanism**, not the whole product. The actual project is a deterministic state-transition engine: it validates work, executes it under explicit rules, and commits a valid new state.

### Project purpose

The purpose is not to build a token-transfer demo or claim to have built a blockchain. It is to have a small enough system to reason precisely about:

- who owns state and who may mutate it;
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
| AOF/RDB persistence                   | canonical state representation and hashing      |
| Go systems implementation             | Rust ownership and error modeling               |

The project is small enough to finish, but produces tangible artifacts for a portfolio: a testable engine, benchmarks, an architecture document, and short notes explaining technical trade-offs.

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

1. Execute a compact, fixed instruction set using a program counter and stack.
2. Read and update a finite in-memory account/key-value state.
3. Validate a transaction before execution.
4. Return an explicit result or execution error without partially applying invalid transactions.
5. Produce the same output state and digest from the same input state and transaction.
6. Prove the above with unit and property-style determinism tests.
7. Document state ownership, invariants, known costs, and non-goals.

That is already a complete, meaningful portfolio project. Merkle trees and parallel scheduling are version-2 work, not requirements for completion.

## Minimal model

### State

Start with an ordered map of account IDs to integer balances. Ordering is intentional: it forces a deterministic traversal and serialization story.

```text
State = { AccountId -> Balance }
```

### Transaction

Start with a transaction that contains a sender, a sequence/nonce, and bytecode. The nonce gives a concrete validation invariant without needing signatures.

```text
Transaction = {
  sender,
  nonce,
  program
}
```

### Instruction set

Keep it intentionally narrow. A useful first set:

```text
Push(i64)       push a constant onto the stack
Add              pop two values and push their sum
Sub              pop two values and push their difference
Load(AccountId)  push an account balance
Store(AccountId) pop a value and update an account balance
Transfer(To)     move a stack-supplied amount from sender to recipient
Halt             successfully stop execution
```

Every instruction should have specified stack effects and failure cases (underflow, overflow, insufficient balance, invalid program counter). Keep arithmetic policy explicit; checked arithmetic is a sensible default.

### Important invariants

- Execution either commits the whole transaction or returns an error with the original state intact.
- Balances never become negative.
- A transaction nonce is accepted only once for a sender.
- Program execution is bounded by an instruction/gas limit.
- State serialization has a canonical order.
- Equal input state plus equal transaction produces equal result, state and digest.

## Delivery sequence

### Milestone 0 — Rust orientation (small experiments)

Write short, disposable experiments before beginning the engine:

- owned `String` versus borrowed `&str`;
- `Vec<T>` growth and slices;
- `Result` and custom error enums;
- `HashMap` versus `BTreeMap` and why iteration order matters;
- moving data into a thread; `Send`, `Sync`, `Arc` and `Mutex`.

The outcome is not a collection of exercises. It is a concise note for each experiment: what was owned, borrowed, copied, allocated, and rejected by the compiler.

### Milestone 1 — Stack VM

- Define `Instruction`, `Program`, `Vm` and `VmError`.
- Implement fetch, decode and execute around a program counter and `Vec<i64>` stack.
- Add table-driven tests for each instruction and malformed program.
- Add a configurable instruction limit.

**Question to answer:** What data belongs to the program, the VM, and an individual execution?

### Milestone 2 — Transactional state transition

- Define state, account and transaction types.
- Validate nonce and balance constraints.
- Execute a transaction against a working state and commit only on success.
- Return an `ExecutionReceipt` containing status, instruction count and state digest.

**Question to answer:** Where is the atomicity boundary, and what is copied or mutated to preserve it?

### Milestone 3 — Determinism and observability

- Define canonical state serialization.
- Hash the canonical bytes with a standard, documented hash crate.
- Test repeatability across fresh engine instances and different insertion orders.
- Add basic benchmarks: instruction loop, state read/write, and transaction execution.
- Record allocations only if a profiler/allocator measurement reveals something worth discussing.

**Question to answer:** Which implementation details can make apparently identical execution produce different results?

### Milestone 4 — Optional, bounded extensions

Choose at most one after version 1 is done:

1. **Conflict-aware batch executor:** declare read/write account sets, execute non-conflicting transactions in parallel, then commit in a deterministic order.
2. **Merkle-style commitment:** replace the flat digest with a simple sorted binary Merkle tree.
3. **Persistence format:** append valid transactions and deterministically rebuild state.

The recommended extension is the conflict-aware batch executor because it directly exposes the concurrency questions relevant to both Rust and Go.

## Suggested repository shape

```text
deterministic-executor/
├── README.md                 # problem, scope, architecture and quick start
├── docs/
│   ├── architecture.md       # ownership/state diagram and execution flow
│   ├── invariants.md         # rules enforced by code and tests
│   ├── determinism.md        # canonicalization and known hazards
│   └── decisions/            # short ADRs for consequential choices
├── src/
│   ├── instruction.rs
│   ├── vm.rs
│   ├── state.rs
│   ├── transaction.rs
│   └── executor.rs
├── tests/
│   ├── vm.rs
│   ├── execution.rs
│   └── determinism.rs
└── benches/
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

> A small deterministic execution engine in Rust built to study bytecode interpretation, transactional state transitions, canonical state representation, and concurrency trade-offs.

Avoid presenting it as a blockchain, production VM, or cryptographic verifier. The portfolio value is the quality of the scope, tests, documentation and trade-off reasoning—not the size of the codebase.

## Relationship to `plan.md`

`plan.md` remains useful as a broad map of topics. This document is the operational plan: it narrows the work to a finishable project, moves advanced verification and blockchain-specific material out of version 1, and removes interview-timing pressure from the technical learning path.
