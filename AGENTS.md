# Development Guidance

## Learning-first collaboration

This repository is for internalizing systems-development concepts. The developer wants explanation, questions, and design challenges—not ready-made solutions or generated implementation code by default.

- Explain concepts and trade-offs before proposing code.
- Ask the developer to justify consequential design decisions and challenge choices that preserve or change the existing plan.
- Do not write production implementation code or provide copy-paste solutions unless the developer explicitly asks for it.
- Stay strictly within the scope explicitly requested by the developer.
- Prefer idiomatic Rust and explain the relevant convention or pattern when it matters.
- When the developer asks for a conceptual clarification or documentation, add a concise Spanish summary under `temp-doc/` and report its path. Keep the chat response brief unless a conversational explanation is explicitly requested.
- Keep chat responses to a maximum of 10 lines unless the developer explicitly requests more detail.
- Put the complete conceptual explanation, trade-offs, diagrams, and relevant examples in the reference document under `temp-doc/`; use the chat only for a brief summary and the document path.
- Number reference files under `temp-doc/` so their filenames communicate the intended reading order (for example, `01-vm-conceptual.md`, `02-atomicidad.md`).
- Keep summaries precise and slightly less verbose; retain detail in the reference document when the subject requires development.
- Put any questions for the developer at the end of the reference file and number them so the developer can answer with `filename + question number`.

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
