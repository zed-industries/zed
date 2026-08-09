# Design: Rust/C boundary lint prototype (prototypes/0003-boundary-lint)

Status: approved, ready for planning.

## Background

RFC 0001 §4 ("Static analysis that rejects logic in user C/C++") is the one open item that hasn't had any prototype work yet — items 1, 2, 3, and 5 are resolved or have data points from `prototypes/0001-hello-blink` and `prototypes/0002-arduino-core`. This design covers a standalone prototype that proves the detection/rejection mechanism works, using the tree-sitter C++ grammar the main Zed/Citadel `crates/` tree already depends on (`tree-sitter = "0.26.9"`, `tree-sitter-cpp` pinned to a specific git rev — see `Cargo.toml:836,839`).

Two things prompted this: (1) verifying that logic constructs written in C/C++ sketch files can actually be detected and rejected, and (2) making it obvious, when a violation is rejected, that the logic belongs in Rust instead — not just that it's disallowed.

## Goal / scope

Build `prototypes/0003-boundary-lint`: a standalone native Rust CLI (not integrated into the IDE) that parses `.cpp` sketch files with tree-sitter and rejects violations of Citadel's Rust/C boundary rule (`CLAUDE.md`, top-level `README.md`).

**In scope — six rules:**

| # | Construct | Detection |
|---|---|---|
| 1 | `if` | `if_statement` node |
| 2 | `for` | `for_statement` node |
| 3 | `while` / `do-while` | `while_statement` / `do_statement` node |
| 4 | Ternary | `conditional_expression` node |
| 5 | Computed intermediate variable | A variable declaration's initializer subtree contains a `binary_expression` node (arithmetic/comparison/logical operators) |
| 6 | User-defined function-like macro | `preproc_function_def` node (`#define NAME(...) ...`) |

**Explicitly out of scope (left as open RFC 0001 §4 questions, documented in the README, not decided by this prototype):**
- Whether a `for` loop inside `setup()` should be exempt from rule 2.
- `.ino` file support (only `.cpp` is parsed).
- Computed expressions appearing anywhere other than a declaration's initializer (e.g. inline arithmetic in a function-call argument).
- Any actual IDE integration (editor squiggles, `crates/languages` changes, LSP diagnostics). Per this session's earlier investigation, Citadel's diagnostics pipeline (`crates/language/src/diagnostic.rs`, `crates/project/src/lsp_store.rs`) is entirely LSP-shaped with no non-LSP diagnostic source pattern today, and `crates/` is otherwise 100% unmodified upstream Zed — wiring a custom lint into it is a separate, much larger effort than this prototype and is not attempted here.

## Repository layout

```
prototypes/0003-boundary-lint/
├── README.md
├── Cargo.toml            # native binary — no_std/avr-none does NOT apply here, this runs on the developer's machine
├── src/
│   └── main.rs            # CLI: parse each argument file with tree-sitter-cpp, walk the tree, collect violations, report
└── examples/
    └── bad_sketch.cpp     # deliberately violates all 6 rules, one violation of each kind, used for manual verification
```

- Dependencies: `tree-sitter = "0.26.9"` and `tree-sitter-cpp` pinned to the same git rev already used in the main `Cargo.toml` (`5cb9b693cfd7bfacab1d9ff4acac1a4150700609`), for consistency with the rest of the repo's toolchain even though this prototype's `Cargo.toml` is standalone (its own `[workspace]`, like 0001/0002's Rust crates).
- Only `.cpp` files are parsed with `tree-sitter-cpp`. The tool does not follow `#include` directives — it only looks at the file(s) passed on the command line. This naturally satisfies RFC 0001 §4's stated scope ("only the user's `.ino`/top-level sketch files, never `libraries/` or vendored headers") since vendored core/library code is never passed to the tool directly.

## CLI and detection algorithm

```
$ boundary-lint <file.cpp> [file2.cpp ...]
```

For each file: parse with `tree-sitter-cpp`, walk the full syntax tree, and for every node encountered:
- If the node kind is `if_statement`, `for_statement`, `while_statement`, `do_statement`, `conditional_expression`, or `preproc_function_def` → record a violation with that rule's fixed message.
- If the node kind is a variable declaration (`declaration` with an `init_declarator` child) → walk its initializer subtree; if any descendant node is `binary_expression`, record a "computed intermediate variable" violation.

A file's walk does **not** stop at the first violation — all violations in a file are collected and reported. Exit code is 0 if no file has any violation, 1 otherwise.

**Violation output format** (one block per violation):
```
<file>:<line>:<col>: error: <rule-id>
  <Japanese explanation of why this is rejected, and where the logic belongs instead>
```

Each of the 6 rules has its own fixed message explaining both the rejection reason and how to move the logic to Rust (e.g. pointing at `extern "C"` functions like `citadel_tick()` from `prototypes/0001-hello-blink`/`0002-arduino-core` as the pattern to follow). This per-violation message is the mechanism satisfying "make it clear logic belongs in Rust" — there is no separate success-path Rust-boundary summary.

**Clean-file output:**
```
<file>: OK
```

**Summary line** after all files are processed: `<N> files checked, <M> file(s) have violations (<K> violations)` or `<N> files checked, 0 violations`.

## Verification plan

- `cargo run -- ../0001-hello-blink/cpp/io.cpp` → expect `OK`, 0 violations.
- `cargo run -- ../0002-arduino-core/cpp/sketch.cpp` → expect `OK`, 0 violations.
- `cargo run -- examples/bad_sketch.cpp` → expect exactly 6 violations, one of each rule kind, with correct line numbers.
- Record the actual command output in `prototypes/0003-boundary-lint/README.md`.

## Definition of done

- `cargo run -- <file>...` works against the three verification files above with the expected pass/fail results.
- `examples/bad_sketch.cpp` triggers all 6 rules, no more, no fewer.
- `prototypes/0003-boundary-lint/README.md` documents: the six rules, the explicitly out-of-scope items, and the recorded verification output.
