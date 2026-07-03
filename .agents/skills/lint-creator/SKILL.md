---
name: lint-creator
description: Add, modify, or test a custom dylint lint in `tooling/lints`. Covers the module/registration layout, UI test conventions (including negative tests and the gpui test fixture), how to run tests with the crate's pinned nightly and update `.stderr` files, and how to validate a lint against the real codebase with `single-lint`. Use whenever the user asks to write a new lint or debug an existing one.
---

# Adding a dylint to `tooling/lints`

## Before writing anything

1. Check whether clippy already covers the pattern (search https://rust-lang.github.io/rust-clippy/master/). If it does, stop and report that to the user instead of writing a redundant lint.

## Layout

- One module per lint: `src/<lint_name>.rs`, declared in `src/lib.rs`.
- Register the lint in `register_lints` in `src/lib.rs`: add it to the `lint_store.register_lints(&[...])` slice **and** add a `register_late_pass` call.
- Use `src/notify_in_render.rs` as the template: `rustc_session::declare_lint!` with `### What it does` / `### Why is this bad?` doc sections, `impl_lint_pass!`, and a `LateLintPass` impl that bails early with `let ... else` / early returns.
- Do NOT copy the diagnostics style of `shared_string_from_str_literal` in `lib.rs` — it predates the rules below (it emits a machine-applicable suggestion). The two lints living directly in `lib.rs` also predate the one-module-per-lint rule.
- Reuse the helpers in `src/render_helpers.rs` (`is_directly_in_render_method`, `is_gpui_context`) for render/gpui checks.

## Diagnostics rules

- Flag only; never suggest how to fix, and never use `Applicability::MachineApplicable`.
- Keep detection and reporting as simple as possible: prefer `span_lint` with a one-sentence message over `span_lint_and_then` with notes.
- Skip macro-expanded code (`expr.span.from_expansion()`).

## UI tests (required)

- Every lint needs `ui/<lint_name>.rs` and `ui/<lint_name>.stderr`.
- The `.rs` file must include **negative cases**: code that resembles the bad pattern but must produce no diagnostic. Their absence from the `.stderr` file is the assertion.
- UI tests that need gpui types use the fake `gpui` crate in `test_fixture/` (wired up by `gpui_fixture_rustc_flags` in `lib.rs`). Extend the fixture if a type or method is missing — never add real gpui as a dependency.
- The crate is deliberately **not** part of the zed workspace and pins its own nightly (`rust-toolchain.toml`). Run tests from inside the crate:

  ```
  cd tooling/lints && cargo test
  ```

- To update a `.stderr` file after an intentional change: run the test, find the `Actual stderr saved to PATH` line in the failure report, and copy that file over the checked-in `.stderr`. There is no bless env var. A correct `.stderr` typically ends with one blank line.

## After the lint works

- Add the lint to the "Current lints" list in `tooling/lints/README.md`.
- Smoke-test it against the real codebase:

  ```
  tooling/lints/single-lint <lint_name> -p <crate>
  ```

  (defaults to `--workspace` if no package is given; the script handles the `--force-warn` and cache-cleaning gotchas documented in the README).
