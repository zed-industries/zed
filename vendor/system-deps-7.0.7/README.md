# system-deps [![](https://img.shields.io/crates/v/system-deps.svg)](https://crates.io/crates/system-deps) [![](https://docs.rs/system-deps/badge.svg)](https://docs.rs/system-deps) [![codecov](https://codecov.io/gh/gdesmott/system-deps/branch/main/graph/badge.svg?token=13DAFV8M8G)](https://codecov.io/gh/gdesmott/system-deps) [![CI](https://github.com/gdesmott/system-deps/workflows/CI/badge.svg)](https://github.com/gdesmott/system-deps/actions)

`system-deps` lets you write system dependencies in `Cargo.toml` metadata,
rather than programmatically in `build.rs`. This makes those dependencies
declarative, so other tools can read them as well.

For now only `pkg-config` dependencies are supported, but we are planning to
[expand it](https://github.com/gdesmott/system-deps/issues/3) at some point.

Users can override dependency flags using environment variables if needed.
`system-deps` also allows `-sys` crates to optionally internally build and
static link the required system library.

`system-deps` has been started as a fork of the
[metadeps](https://github.com/joshtriplett/metadeps) project.

## Documentation

See the [crate documentation](https://docs.rs/system-deps/).

## Usage

In your `Cargo.toml`:

```toml
[build-dependencies]
system-deps = "2.0"
```

Then, to declare a dependency on `testlib >= 1.2` add the following section:

```toml
[package.metadata.system-deps]
testlib = "1.2"
```

Finally, in your `build.rs`, add:

```rust
fn main() {
    system_deps::Config::new().probe().unwrap();
}
```

See the [crate documentation](https://docs.rs/system-deps/) for more advanced features.
