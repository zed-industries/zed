# Dylint Roadmap: SharedString Performance Hygiene in Zed

This document records the plan for using
[dylint](https://github.com/trailofbits/dylint) to find and fix
performance-relevant misuses of `gpui::SharedString` in the Zed repository.
Dylint is a Rust linting tool that loads lints from dynamic libraries, so a
project can ship its own lints without modifying Clippy. The lints are
written against the same `LateLintPass` interface that Clippy uses.

## 1. Why this matters

`SharedString` is the immutable string type used throughout Zed's UI. It
wraps [`SmolStr`](https://docs.rs/smol_str/0.3.6/smol_str/), a small-string
optimized type. `SmolStr`'s cost model has three tiers:

- **Free**: `SharedString::new_static(&'static str)` stores the static pointer
  directly. No copy, no allocation, no atomic ref-count.
- **Cheap**: `From<&str>` for a literal of at most 23 bytes copies the bytes
  into inline storage. One memcpy, no allocation. (`smol_str` 0.3 sets
  `INLINE_CAP = 23` on 64-bit targets.)
- **Expensive**: `From<&str>` for a literal longer than 23 bytes allocates a
  fresh `Arc<str>` and copies into it. Same cost applies to `From<String>`,
  `From<&String>`, `From<Cow>`: `SmolStr::from` never reuses the input
  allocation.

Implication: any conversion path other than `new_static` is strictly more
expensive than `new_static` for a string literal. The lint catches that.

## 2. Lints

Four lints were identified during investigation. Only Lint A is implemented.

### Lint A — `shared_string_from_str_literal` (implemented)

Detects:

- `SharedString::from("…")`
- `SharedString::new("…")`
- `<SharedString as From<_>>::from("…")`
- `"…".into()` whose inferred target is `SharedString`

Excludes `SharedString::new_static("…")`. Reports two severity tiers in the
diagnostic note, distinguishing literals that exceed the 23-byte inline cap
(heap allocation per call) from literals that stay inline (memcpy per call).

Suggestion is `SharedString::new_static("…")`, marked `MachineApplicable`. The
suggestion preserves the original source token for the literal (raw strings,
escapes, byte strings).

### Lint B — `shared_string_double_alloc` (planned, not implemented)

Detects:

- `x.to_string().into()`
- `x.to_owned().into()`
- `String::from(x).into()`

…where the `.into()` result is `SharedString` and `x` derefs to `str` without
already being `String`. Each pattern allocates a `String` only to immediately
copy it into an `Arc<str>`. The fix is `x.into()` directly (or
`SharedString::new_static(x)` if `x` is itself a literal, composing with Lint
A).

This lint targets ~23 sites across the workspace as of the initial grep.

### Lint C — `shared_string_format_no_args` (planned, not implemented)

Detects `format!("static literal").into()` with no format arguments. The lint
matches on the expanded `format_args!` having a single literal piece and no
captures. Suggestion is `SharedString::new_static("…")`.

Low volume (no clean hits in the current grep) but trivial to write.

### Lint D — `shared_string_clone_to_string` (deferred)

Detects `x.clone().to_string()` where `x: SharedString`. False-positive prone:
some callers genuinely want an owned `String`. Recommended `Allow` by default,
gated on the consuming expression's expected type being `SharedString`.
Skip for the first cut.

### Lint E (out of scope) — newtype `.to_owned().into()`

The pattern in `agent/src/native_agent_server.rs:110`:

```rust
provider: provider.to_owned().into(),
```

bounces through a string copy when the originating type already has a
zero-copy `From` impl that moves out a `SharedString` field. Detection
requires whole-program type information; not worth attempting in the lint.

## 3. Implementation status

The Lint A library lives at `tooling/shared_string_lints/` on branch
`shared-string-lint`, currently checked out at
`/Users/mrg/agents/zed-shared-string-lint`. A single commit on that branch
adds the crate.

Layout:

```
tooling/shared_string_lints/
├── .cargo/config.toml         # routes linking through dylint-link
├── Cargo.toml                 # standalone workspace, crate-type = ["cdylib"]
├── README.md                  # usage and limitations
├── rust-toolchain.toml        # nightly-2026-01-22 (matches clippy_utils pin)
├── src/lib.rs                 # the LateLintPass
└── test_fixture/
    ├── Cargo.toml             # workspace.metadata.dylint = [{ path = ".." }]
    ├── gpui_shared_string/    # minimal stand-in (lint keys off crate name)
    └── consumer/              # five positive cases, three negatives
```

Verified end-to-end against the real `csv_preview` crate. All three known
hits in that crate were detected; no false positives.

## 4. Running the lint

Prerequisites:

```sh
cargo install cargo-dylint dylint-link
```

The toolchain pinned in `rust-toolchain.toml` is installed automatically by
rustup on first use.

Single crate:

```sh
cargo dylint --path tooling/shared_string_lints -- -p csv_preview
```

Whole workspace, capturing output:

```sh
NO_COLOR=1 cargo dylint --pipe-stderr dylintfixes.txt \
  --path tooling/shared_string_lints -- --workspace
```

Heap-tier hits only:

```sh
NO_COLOR=1 cargo dylint --path tooling/shared_string_lints \
  -- --workspace 2>&1 \
  | rg -B1 -A6 'heap-allocates on every call' \
  > dylintfixes-heap.txt
```

Apply suggestions in place:

```sh
cargo dylint --fix --path tooling/shared_string_lints -- -p <crate>
```

Review the resulting diff. The fix uses the unqualified name
`SharedString::new_static`, so any crate that does not import `SharedString`
will need a manual path edit (typically `gpui::SharedString::new_static`).

## 5. Benchmark plan

Three tiers, ordered by signal-to-noise.

### Tier 1 — per-call microbenchmark (most reliable)

Lives at `crates/gpui_shared_string/benches/constructors.rs`. Uses
[`criterion`](https://docs.rs/criterion/) with `black_box` so the optimizer
cannot fold literals at compile time.

Compares four constructors at two length tiers:

| Constructor                                        | 9-byte literal | 28-byte literal |
|----------------------------------------------------|----------------|-----------------|
| `SharedString::new_static`                         | < 1 ns         | < 1 ns          |
| `SharedString::from(&str)`                         | 3-8 ns         | 40-150 ns       |
| `<&str>::into::<SharedString>()`                   | 3-8 ns         | 40-150 ns       |
| `s.to_string().into::<SharedString>()`             | 30-80 ns       | 80-200 ns       |

Long-tier numbers are allocator-bound and vary with the system allocator
(`mimalloc`, `jemalloc`, `system`). The point of the microbench is to
establish the per-call delta. Aggregate benchmarks then multiply by call
frequency.

### Tier 2 — render-loop microbenchmarks

Three target crates, in priority order:

1. **`editor`** — gutter rendering. The strongest candidate because the
   currently-affected literals exceed the 23-byte cap. `render_breakpoint`
   constructs `"No executable code is associated with this line."` (50
   bytes) and `"Right-click for more options"` (28 bytes) per visible
   breakpoint per frame. With 30 breakpoints in view at 60 fps, the existing
   code does 1,800 heap allocations per second from these two sites alone.
   `display_map::grapheme_at` does `s.to_owned().into()` per visible
   grapheme, which is even hotter. Bench shape: drive
   `EditorElement::paint_gutter` (or the closest testable equivalent) on a
   synthetic buffer with N breakpoints, M bookmarks, and a controlled
   invisibles density.

2. **`command_palette`** — picker render-match. Per-keystroke re-render of
   ~50 visible matches. Bench shape: build a `CommandPaletteDelegate` with
   1,000 synthetic actions, call `render_match` for each in a tight loop.
   Generalizable to the other ~20 picker delegates in the workspace
   (`file_finder`, `branch_picker`, `language_model_selector`, etc.).

3. **`project_panel`** — `render_entry` per file-tree entry. Visible most of
   the time for most users; re-renders on focus change, scroll, and
   filesystem events. Bench shape: synthetic worktree with N entries, drive
   `render_entry` for each.

Crates considered but not selected:

- `git_graph` (`render_table_rows`): clean shape, but used by a smaller
  user population. Run if time permits.
- `csv_preview`: cleanest synthetic stress test but only inline-tier hits
  and an unrepresentative usage pattern. Useful as a unit-style validation
  of the lint, not a credible end-to-end argument.
- `collab_panel`, `tab_bar`, debugger lists: too few items per render to
  exceed measurement noise.

### Tier 3 — aggregate heap-pressure trace

Use [`dhat`](https://docs.rs/dhat/latest/dhat/) with a feature-gated global
allocator:

```rust
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    // ... rest of zed startup
}
```

Run a scripted workflow on `main` and on the post-fix branch. Suggested
script: open zed against the zed repo itself, open
`crates/editor/src/editor.rs`, scroll to the bottom, jump back to the top,
switch to the project panel, open and close three files. Compare
`total_blocks`, `total_bytes`, `max_blocks`, `max_bytes`. The
`Arc<str>::new` frames originating in `<SharedString as From<&str>>::from`
form a distinct stack-trace bucket, so attribution is direct.

`heaptrack` (Linux) is a higher-resolution alternative. Instruments
→ Allocations on macOS produces equivalent data.

End-to-end frame-time benchmarks are explicitly **not** recommended. Even
the heap-tier savings are below the noise floor of macOS frame timing
without aggressive controls for thermal throttling and background
processes.

## 6. Recommended execution order

1. Run Lint A across the full workspace and produce `dylintfixes.txt`. Sort
   hits by tier (heap vs. inline) and by crate.
2. Apply `--fix` to the high-priority crates first (`editor`,
   `project_panel`, `command_palette`, `agent_ui`, `git_ui`). Review the
   diff for import issues.
3. Land the Tier 1 microbench. The numbers from this bench are the
   reference everyone else will multiply through.
4. Land the Tier 2 editor gutter benchmark. This is where the strongest
   single-PR result is expected.
5. Run the Tier 3 dhat trace before/after the editor fixes. Cite the
   allocation-count delta in the PR description.
6. Implement Lint B (`shared_string_double_alloc`), since `agent_ui` and
   `editor` have a meaningful number of `format!(…).into()` and
   `to_owned().into()` hits that Lint A misses. Reuse the same
   `dylint_linting::declare_late_lint!` skeleton.
7. Optionally implement Lint C if Lint B's output suggests a long tail of
   no-args `format!(…).into()` cases.

## 7. Open questions and follow-ups

- **Suggestion path qualification.** Lint A emits the unqualified
  `SharedString::new_static`. About 95% of zed crates `use gpui::SharedString`
  already, so the rewrite compiles. The remaining ~5% need manual edits.
  Consider a more conservative emission that uses `gpui::SharedString::new_static`
  unconditionally, at the cost of uglier diffs.
- **Macro-hygiene exemption.** The lint bails on `expr.span.from_expansion()`.
  This hides any matches inside macros such as `element_id!(...)` if any are
  introduced. Worth revisiting once a concrete loss is identified.
- **`element_id` crate-level helper.** Many of the inline-tier hits are
  `ElementId::Name("literal".into())`. A constructor
  `ElementId::Name::new_static("literal")` (or just
  `ElementId::name_static("literal")`) would let the lint suggestions become
  shorter and would steer future code toward the cheap path. This is a Zed
  API change, not a lint change.
- **`Lint` ↔ ECS-style fix runner.** If Zed adopts the lint long-term,
  wiring it into `./script/clippy` so CI flags regressions is the natural
  next step. For a one-shot fix campaign this is unnecessary.
- **Upstreaming.** Whether to keep the lint in-tree (`tooling/`) or as an
  out-of-tree repo depends on whether this is a one-shot or an ongoing
  hygiene tool. For a one-shot, keep it in `tooling/` with a note in the
  README explaining that the toolchain pin is intentional and not part of
  the workspace.

## 8. References

- Dylint repository and user guide:
  <https://github.com/trailofbits/dylint>
- Clippy's lint-writing guide (the same APIs apply):
  <https://github.com/rust-lang/rust-clippy/blob/master/book/src/development/adding_lints.md>
- `smol_str` source for the inline-capacity constant:
  <https://docs.rs/smol_str/0.3.6/smol_str/>
- `dhat` heap profiler:
  <https://docs.rs/dhat/latest/dhat/>
- Criterion benchmark harness:
  <https://docs.rs/criterion/>
