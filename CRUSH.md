# CRUSH.md — Zed Agent & AI Coding Handbook

## Build, Lint, and Test Commands

- **Build all crates:**
  ```bash
  cargo build --workspace
  ```
- **Build a specific crate:**
  ```bash
  cargo build -p <crate_name>
  ```
- **Lint (preferred):**
  ```bash
  ./script/clippy
  ```
- **Run tests for all crates:**
  ```bash
  cargo test --workspace
  ```
- **Run a single test in a specific crate:**
  ```bash
  cargo test -p <crate_name> <test_name>
  ```
- **Other utility scripts:**
  See `script/` (e.g. `script/check-licenses`, `script/check-keymaps`, `script/check-todos`)

## Code Style Guidelines (from `.cursorrules`, `CLAUDE.md`, `.clinerules`)

- Prioritize code correctness and clarity; optimize only where necessary.
- Only write comments to explain *why* when the reason is non-obvious.
- Minimize file/module sprawl; prefer adding logic to existing files.
- Do **not** use panicking code (`unwrap()`, panic, indexing w/o checks). Always propagate (`?`) or explicitly log errors; never discard with `let _ =`.
- Indexing/panics: write safe code, prefer `get` or bounds checks.
- Async error handling: errors must eventually reach the UI/user layer; never silently ignore async failures.
- Never create files named `mod.rs`; always use `src/thing.rs`.
- Use modern GPUI idioms: `Entity<T>`, `Context<T>`, `App`, explicit `Window`.
- Use latest-and-recommended APIs from `.cursorrules`/`CLAUDE.md` (e.g., no `Model<T>`, always closures for `spawn`).
- Clear up all borrows when working with entities and GPUI contexts.

## Naming/Type/Formatting Conventions

- Import order: std, external, workspace, local.
- snake_case for variables and functions; CamelCase for types, traits, structs; SHOUTY_CASE for constants.
- Prefer explicit types where interface is exported; type inference ok locally.
- Write idiomatic, concise Rust; do not pad with needless boilerplate.

## General & Miscellaneous
- Use `./script/clippy` instead of `cargo clippy`.
- Add new agent/project-specific rules here.
- Never commit secrets. If editing `.crush` or agent support files, update `.gitignore` (already handled here).

---

This file supersedes and summarises `.cursorrules`, `.clinerules`, and `CLAUDE.md`. Review them for detailed context and always follow the most restrictive/best-practice version of any rule.