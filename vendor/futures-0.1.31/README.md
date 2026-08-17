# futures-rs

This library is an implementation of **zero-cost futures** in Rust.

[![Build Status](https://img.shields.io/github/workflow/status/rust-lang/futures-rs/CI/master)](https://github.com/rust-lang/futures-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/futures.svg?maxAge=2592000)](https://crates.io/crates/futures)

[Documentation](https://docs.rs/futures)

[Tutorial](https://tokio.rs/docs/getting-started/futures/)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
futures = "0.1.26"
```

Next, add this to your crate:

```rust
extern crate futures;

use futures::Future;
```

For more information about how you can use futures with async I/O you can take a
look at [https://tokio.rs](https://tokio.rs) which is an introduction to both
the Tokio stack and also futures.

### Feature `use_std`

`futures-rs` works without the standard library, such as in bare metal environments.
However, it has a significantly reduced API surface. To use `futures-rs` in
a `#[no_std]` environment, use:

```toml
[dependencies]
futures = { version = "0.1.26", default-features = false }
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Futures by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
