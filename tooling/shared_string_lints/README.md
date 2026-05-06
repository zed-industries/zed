# `shared_string_lints`

A dylint library that flags `gpui::SharedString` values built from string
literals via a path that copies or allocates. `SharedString::new_static` stores
a `'static` pointer with no allocation and no copy; every other literal-based
constructor is strictly more expensive.

## What is detected

The lint fires on:

- `SharedString::from("…")`
- `SharedString::new("…")`
- `<SharedString as From<_>>::from("…")` (via `SharedString::from(x)` syntax)
- `"…".into()` whose inferred target type is `SharedString`

It does not fire on `SharedString::new_static("…")` or on any of these
constructors when the argument is not a string literal.

Two severity tiers are reported via the note:

- Literals longer than 23 bytes trigger an `Arc<str>` heap allocation on every
  call (`SmolStr`'s inline capacity is 23). The note says "heap-allocates".
- Literals 23 bytes or shorter stay inline in the `SmolStr`, so the cost is
  only a memcpy. The note says "copies the literal".

In both cases `SharedString::new_static` is strictly cheaper.

## Usage

Requires `cargo-dylint` and `dylint-link`:

```
cargo install cargo-dylint dylint-link
```

The library pins `nightly-2026-01-22` via `rust-toolchain.toml`; rustup will
install it automatically on first use.

### Run on the current zed worktree

From the root of the zed worktree this crate lives in:

```
cargo dylint --path tooling/shared_string_lints -- -p csv_preview
```

Replace `csv_preview` with the target crate, or use `--workspace` to lint
everything (slow: the whole workspace has to be checked under the dylint
toolchain).

### Apply suggestions automatically

```
cargo dylint --fix --path tooling/shared_string_lints -- -p csv_preview
```

All suggestions are emitted as `MachineApplicable`, so `--fix` rewrites them
in place. Review the resulting diff: the rewrite assumes `SharedString` is in
scope at the call site. In zed's crates that is almost always true via
`use gpui::SharedString` or a prelude, but not universally.

### Smoke test

A minimal fixture that exercises positive and negative cases lives under
`test_fixture/`. Run it with:

```
cd test_fixture && cargo dylint --all
```

Expected: three inline-copy warnings, two heap-allocation warnings, no
warnings on `SharedString::new_static(...)` or on variable arguments.

## Limitations

- Macro-expanded call sites are skipped to avoid suggesting edits to spans the
  user cannot touch. This may hide some hits inside `format!`-adjacent macros.
- The suggested replacement text uses the unqualified name `SharedString`. If
  the call site does not import `SharedString`, the fix will not compile and
  has to be edited to use a path (e.g. `gpui::SharedString::new_static(…)`).
- The lint does not currently detect `format!("static literal").into()` or
  `x.to_string().into()` chains. Those are separate patterns (cf. the plan
  document) and would go into additional lints alongside this one.
