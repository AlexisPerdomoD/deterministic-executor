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

## Límite de responsabilidad de la VM

La VM ejecuta un programa validado sobre un estado recibido y, si termina correctamente, devuelve el estado resultante. Su responsabilidad es aplicar las transiciones de las instrucciones de forma determinista y mantener sus invariantes locales: límites aritméticos, referencias requeridas y actualización consistente de balances y nonce.

La VM no decide si una operación está autorizada por una política externa. Si no hay `owner_code`, la ejecución puede acreditar valor al receptor como emisión de sistema; la VM no exige ni inventa una cuenta emisora. La legitimidad de esa emisión corresponde al componente que solicita la ejecución.

La autorización, la persistencia del estado y la capa de transporte quedan fuera del motor. No deben atribuirse implícitamente al intérprete ni confundirse con la validez semántica de una transición.

Estas responsabilidades se reflejan en las pruebas: con `owner_code`, una transferencia debita al owner, acredita al receptor e incrementa solo el nonce del owner; sin `owner_code`, acredita al receptor sin débito ni cambio de nonce. Una transferencia fallida conserva balances y nonces. Los tests de integración validan el estado devuelto y la API pública; los tests internos pueden comprobar handlers privados. El stack es privado y sus contenidos tras un error no forman parte del resultado contractual de la VM.

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
