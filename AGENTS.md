# Development Guidance

## Learning-first collaboration

This repository is for internalizing systems-development concepts. The developer wants explanation, questions, and design challenges—not ready-made solutions or generated implementation code by default.

- Explain concepts and trade-offs before proposing code.
- Ask the developer to justify consequential design decisions and challenge choices that preserve or change the existing plan.
- Do not write production implementation code or provide copy-paste solutions unless the developer explicitly asks for it.
- Stay strictly within the scope explicitly requested by the developer.
- Prefer idiomatic Rust and explain the relevant convention or pattern when it matters.

## Architecture

The architecture is **runtime-oriented**: it models deterministic execution, state transitions, ownership, validation, and failure semantics.

- Keep the focus on the execution engine, not application-backend layers.
- Call out when patterns from application backend engineering—such as controllers, repositories, service layers, DTOs, or premature abstractions—would obscure the runtime-oriented design.
- Do not introduce networking, HTTP, databases, authentication, consensus, P2P, tokens, parsers, or premature parallel execution unless explicitly requested.

## Tests and validation

- An initial test structure may be created when requested.
- If a test fails, do **not** change source code to fix it. Stop, report the failure and relevant context to the developer, and wait for explicit direction.
- Encourage each module or behavior to have its relevant tests passing before it is pushed.
- Run or recommend formatting and linting as part of validation: `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` when applicable.

## Git

- Keep commits atomic and self-contained.
- Use conventional commit types such as `feat:`, `fix:`, `test:`, `docs:`, and `chore:`.
- Write meaningful commit descriptions when the change warrants them.
- Never push to a remote branch unless the developer explicitly requests it.
