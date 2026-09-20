# Deterministic Executor

A small deterministic execution engine written in Rust.

It executes a fixed instruction set against in-memory state and applies each transaction atomically: valid changes are committed together, while a failed execution leaves the original state unchanged.

## Purpose

This is a systems-development learning project. Its focus is on:

- bytecode interpretation and stack-based execution;
- ownership of mutable state and controlled mutation;
- validation, failures, and atomic state transitions;
- deterministic results and canonical state representation;
- Rust ownership, error modeling, testing, and later concurrency trade-offs.

The system is deliberately small so that its behavior, invariants, and costs can be understood precisely.

## Initial scope

Version 1 will provide:

- a compact, fixed instruction set;
- an in-memory account/key-value state with balances and nonces;
- transaction validation before execution;
- a bounded stack VM with explicit execution errors;
- atomic commit or rollback of state transitions;
- execution receipts and a deterministic state digest;
- tests for VM behavior, atomicity, and determinism.

## Non-goals

The initial version is not a blockchain, wallet, database, network service, full programming language, consensus system, or parallel executor. It has no HTTP API, persistence, signatures, peer-to-peer networking, or token.

Those are separate experiments to consider only after the single-threaded engine is complete and understood.

## Milestones

1. **Rust orientation** — ownership, borrowing, errors, collections, and small concurrency experiments.
2. **Stack VM** — instructions, program counter, operand stack, execution errors, and instruction limits.
3. **Transactional execution** — state, transactions, validation, atomic commit/rollback, and receipts.
4. **Determinism** — canonical serialization, state digest, repeatability tests, and basic benchmarks.
5. **Optional extension** — one bounded experiment: conflict-aware batch execution, a Merkle-style commitment, or persistence.

## Structure

The project starts as one Rust crate. Its core is organized around execution responsibilities rather than application-backend layers:

```text
src/
├── lib.rs          # library entry point
├── main.rs         # minimal executable entry point
├── instruction.rs  # instruction-set definitions
├── vm.rs           # stack VM and execution loop
├── state.rs        # account/state model
└── executor.rs     # validation and atomic state transitions
```

See [`doc/PLAN.md`](doc/PLAN.md) for the operational roadmap, invariants, and learning resources.
