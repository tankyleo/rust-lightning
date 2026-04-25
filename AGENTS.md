# Repository Guidelines

## Project Structure & Module Organization

This is the `rust-lightning` Cargo workspace. The root `Cargo.toml` lists the primary crates: `lightning` for core protocol logic, plus support crates such as `lightning-invoice`, `lightning-net-tokio`, `lightning-persister`, `lightning-background-processor`, and `lightning-liquidity`. Core source lives under each crate's `src/` directory, with major `lightning` modules in paths like `lightning/src/ln`, `lightning/src/chain`, `lightning/src/routing`, and `lightning/src/offers`.

Integration and shared test support live in `lightning-tests/`. Fuzz targets and fuzzing documentation live in `fuzz/`. CI entry points are in `ci/`. Release-note fragments for compatibility or user-visible changes belong in `pending_changelog/`.

## Build, Test, and Development Commands

Use the workspace root for normal development commands.

- `cargo test --workspace`: run workspace tests.
- `cargo test -p lightning`: run tests for the core crate only.
- `cargo +1.75.0 fmt`: format code with the repository MSRV toolchain.
- `cargo clippy --workspace --all-targets`: run Rust lints locally.
- `./ci/check-lint.sh` and `./ci/ci-tests.sh`: run CI-style linting or broader test checks when validating larger changes.

## Coding Style & Naming Conventions

Rust code is formatted with `rustfmt.toml`: hard tabs for indentation, spaces only for alignment, and `max_width = 100`. Use `snake_case` for functions, modules, variables, and test names; use `CamelCase` for types and traits; use `SCREAMING_SNAKE_CASE` for constants.

Keep changes reviewable. Do not mix formatting-only churn, code movement, and behavior changes in the same commit. Avoid new dependencies unless clearly justified; this project treats dependency growth as a security and maintenance risk.

## Testing Guidelines

New features and protocol behavior changes should include functional tests. Prefer focused tests near the affected module or in `lightning-tests/` when cross-crate behavior is involved. Fuzzing is encouraged for parsers, serialization, state-machine edges, and network-message handling; add targets under `fuzz/src/` when appropriate.

Useful deterministic test variables include `LDK_TEST_DETERMINISTIC_HASHES=1` and `LDK_TEST_CONNECT_STYLE=<mode>` for reproducing test behavior.

## Commit & Pull Request Guidelines

Recent history favors short imperative subjects such as `Add ...`, `Reject ...`, `Pull out ...`, and `Drop ...`, often with backticks around Rust identifiers or feature names. Final PR commits should be atomic, compile independently, and explain both the issue and rationale in the body when the change is nontrivial.

Pull requests should include a clear description, testing performed, linked issues when relevant, and `pending_changelog/` entries for nontrivial backwards-compatibility or serialization-impacting changes. Security-sensitive reports should follow `SECURITY.md`, not public issue discussion.
