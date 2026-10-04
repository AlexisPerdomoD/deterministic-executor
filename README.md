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

## Límite de responsabilidad de la VM

La VM ejecuta un programa validado sobre un estado recibido y, si termina correctamente, devuelve el estado resultante. Su responsabilidad es aplicar las transiciones de las instrucciones de forma determinista y mantener sus invariantes locales: límites aritméticos, referencias requeridas y actualización consistente de balances y nonce.

La VM no decide si una operación está autorizada por una política externa. Si no hay `owner_code`, la ejecución puede acreditar valor al receptor como emisión de sistema; la VM no exige ni inventa una cuenta emisora. La legitimidad de esa emisión corresponde al componente que solicita la ejecución.

La autorización, la persistencia del estado y la capa de transporte quedan fuera del motor. No deben atribuirse implícitamente al intérprete ni confundirse con la validez semántica de una transición.

Estas responsabilidades se reflejan en las pruebas: con `owner_code`, una transferencia debita al owner, acredita al receptor e incrementa solo el nonce del owner; sin `owner_code`, acredita al receptor sin débito ni cambio de nonce. Una transferencia fallida conserva balances y nonces. Los tests de integración validan el estado devuelto y la API pública; los tests internos pueden comprobar handlers privados. El stack es privado y sus contenidos tras un error no forman parte del resultado contractual de la VM.

## Initial scope

Version 1 provides:

- a compact instruction set and validated programs that end with `Halt`;
- an in-memory account state with balances and nonces;
- a stack VM with checked arithmetic, account loading, transfers, and explicit errors;
- a `Result<State, VMError>` execution contract;
- unit and integration tests for instruction behavior, state transitions, errors, and repeatability.

## Non-goals

The initial version is not a server, blockchain, wallet, database, full programming language, consensus system, or parallel executor. It has no TCP/HTTP transport, parser, persistence, authorization policy, transaction-level nonce/replay validation, execution receipt type, state digest, or configurable instruction/gas limit.

Authorization is an external responsibility. When `owner_code` is absent, the VM permits a system-issued credit; it does not decide whether the caller is authorized to request one. Other excluded capabilities can be separate experiments after the library contract is complete and understood.

## Milestones

1. **Rust orientation** — ownership, borrowing, errors, and collections.
2. **Program and VM** — validated instructions, stack execution, state transitions, and explicit errors.
3. **Execution contract** — successful state results, error behavior, and deterministic repeatability tests.
4. **Optional extension** — a separate experiment only after the library contract is complete.

## Structure

The project starts as one Rust crate. Its core is organized around execution responsibilities rather than application-backend layers:

```text
src/
├── lib.rs          # library entry point
├── main.rs         # minimal executable entry point
├── instruction.rs  # instruction-set definitions
├── vm.rs           # stack VM and execution loop
└── state.rs        # account/state model
```

See [`doc/PLAN.md`](doc/PLAN.md) for the operational roadmap, invariants, and learning resources.
