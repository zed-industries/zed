# `serde_fmt`

[![rust](https://github.com/KodrAus/serde_fmt/actions/workflows/rust.yml/badge.svg)](https://github.com/KodrAus/serde_fmt/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/serde_fmt.svg)](https://crates.io/crates/serde_fmt)
[![Documentation Latest](https://docs.rs/serde_fmt/badge.svg)](https://docs.rs/serde_fmt)

Convert any `serde::Serialize` into a `std::fmt::Debug`:

```rust
fn takes_serialize(v: impl Serialize) {
    dbg!(serde_fmt::to_debug(&v));

    // Do something with `v`
}
```

## Supported `rustc`

This library requires a Rust compiler that's at least `1.42.0`.

## Getting started

Add `serde_fmt` to your `Cargo.toml`:

```toml
[dependencies.serde_fmt]
version = "1.0.3"
```

By default, this library will depend on the standard library. To use it in no-std environments, you can disable the default crate features:

```toml
[dependencies.serde_fmt]
version = "1.0.3"
default-features = false
```
