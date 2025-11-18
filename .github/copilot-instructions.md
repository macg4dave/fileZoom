# Repository custom instructions for GitHub Copilot

Purpose: Provide concise, repository-wide context and preferences for Copilot.

## Project overview
- CLI file manager written in Rust, focused on usability and simplicity, with mouse support.
- 100% RUST with no external dependencies beyond standard Rust crates.
- Supports MacOS, Debian and Fedora.

## Folder structure (important paths)
- `/app` — binary and library code
- `/README.md` — project README
- `/.github` — Copilot instructions and prompt files

## Tools & environment
- Language: Rust.
- Build/test: `cargo build`, `cargo test`, `cargo run`.
- IDE: `rust-analyzer`, `cargo fmt`, `clippy` encouraged.

## Coding standards & conventions
- Follow `rustfmt` formatting and prefer idiomatic Rust (snake_case for identifiers, `Result` error handling, explicit `unwrap` only in small examples/tests).
- Prefer clarity over cleverness; add doc comments on public APIs.
- Preserve public CLI flags and their semantics.
- No unsafe or only with justification and added tests.
- Performance changes must not alter user-visible behavior.
- Patch that minimally changes code and explicitly calls out impacted modules.
- Make as modular code as possible; avoid large monolithic functions.
- Write modular functions with single responsibility.
- Write clear variable and function names. 
- Write comments for complex logic.
- Write doc comments for public APIs. 
- Write examples in doc comments where helpful.
- Write idiomatic Rust, leveraging iterators, pattern matching, and error handling best practices.
- Run the test suite locally and include full test output (`cargo test` or targeted test command).
- test output (`cargo test` or targeted test command). 
- tests must pass before submission.
- Build must pass before submission.
- fix any build errors and rerun build upto 5 times.
- fix any test errors and rerun test upto 5 times.



## Testing and PRs
- Use `cargo test` for unit and integration tests. Include tests for any public behavior.
- Do not remove functions/modules without test updates passing.
- List tests it expects to pass and any additional tests it proposes.
- Write tests for any new behavior. 
- PRs should be small, focused, and include test results.
- Write/Update documentation for any public API changes.
- Write/Update tests for any behavioral.

## Response preferences
- Keep answers concise and focused
- When giving code, prefer small patches that compile with `cargo test` or `cargo run`.

## Security & external references
- Do not fetch or assume private/internal resources. If a response requires external data, ask for the data or a public reference.

## Prompt files & path-specific instructions
- Use `.github/instructions/` for path-specific rules and `*.prompt.md` for reusable prompts in VS Code (public preview).

## Limitations and notes
- Copilot Code Review reads only the first 4,000 characters of any custom instruction file — keep critical rules near the top.
- Avoid overly prescriptive or fragile constraints (e.g., exact character limits or strict output formats) that may reduce usefulness.

## Hard constraints to include in every AI prompt
- Run the test suite locally and include full test output (`cargo test` or targeted test command).
- Make the smallest possible change needed to solve the request.
- Add or update tests for any behavioral change.
- Preserve public APIs and CLI machine-facing outputs unless explicitly allowed.
- Avoid removing features or tests. If you must remove or change behavior, add a migration note and tests demonstrating the new behavior.