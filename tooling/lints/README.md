# lints

A [dylint](https://github.com/trailofbits/dylint) library that flags various bad patterns in our codebase.

Install `dylint`, a pinned nightly toolchain and the necessary tools with

```
cargo install cargo-dylint dylint-link
cd tooling/lints
rustup toolchain install
```

The channel and its components (`rustc-dev`, `rust-src`, `llvm-tools-preview`)
are declared in `tooling/lints/rust-toolchain.toml`, so `rustup toolchain install`
picks them up automatically when run from that directory.

# Demo

```
./single-lint blocking_io_on_foreground
```


## Current lints
- `shared_string_from_str_literal` — `SharedString::new/from` etc where `SharedString::from_static` should be used instead.
- `async_block_without_await` — `async { … }` blocks whose body contains no `.await` expression.
- `entity_update_in_render` — `Entity::update`/`WeakEntity::update` mutating an entity inside `Render::render`.
- `notify_in_render` — `Context::notify()` called inside `Render::render`.
- `owned_string_into_shared` — `String::from(<lit>).into()` / `<lit>.to_string().into()` / `<lit>.to_owned().into()` whose target is `SharedString`, `Arc<str>`, `Rc<str>`, or `Cow<'_, str>`.
- `blocking_io_on_foreground` - Catch blocking IO calls that are called on the main thread (but not on closures or background threads)

## How to run

Ideally you run this as part of the `clippy` script in the `zed/scripts` directory since this will also run our other linters.

### Prerequisites

Install both tools (version 6 or later):

```
cargo install cargo-dylint dylint-link
```

- `cargo-dylint` is the `cargo` subcommand that builds and runs the lints; `dylint-link` is the linker used to build the lint library.

The workspace registers this library under `[workspace.metadata.dylint]` in the
root `Cargo.toml`, so Dylint discovers it automatically — you do not pass a
`--path`. The first run builds the library against its pinned nightly (see
`rust-toolchain.toml`) and is slow; later runs are cached.

### Run all lints against the whole repo

```
cargo dylint --all -- --workspace
```

### Run all lints against a single crate

```
cargo dylint --all -- -p project_panel
```

### Run a single lint

The library loads every lint at once. To run just one, use the `single-lint`
helper, which silences the rest and force-enables the one you name:

```
tooling/lints/single-lint blocking_io_on_foreground -p project_panel
```

The first argument is the lint name (one of the snake_case identifiers under
[Current lints](#current-lints)); everything after it is passed to `cargo check`
and defaults to `--workspace`. Under the hood the script runs:

```
DYLINT_RUSTFLAGS="-A warnings --force-warn <lint>" cargo dylint --all -- <args>
```

It also handles two non-obvious gotchas:

- `--force-warn` is required: after `-A warnings` silences the group, a plain
  `-W <lint>` does not reliably re-enable a driver-registered lint.
- `DYLINT_RUSTFLAGS` is not part of Cargo's fingerprint, so the script cleans the
  targeted package(s) first; otherwise Cargo replays a stale cache and the filter
  appears to do nothing.
