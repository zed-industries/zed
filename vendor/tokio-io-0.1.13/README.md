# tokio-io

Core I/O abstractions for the Tokio stack.

[![Build Status](https://travis-ci.org/tokio-rs/tokio-io.svg?branch=master)](https://travis-ci.org/tokio-rs/tokio-io)

> **Note:** This crate has been **deprecated in tokio 0.2.x** and has been moved
> into [`tokio::io`].

[`tokio::io`]: https://docs.rs/tokio/latest/tokio/io/index.html

[Documentation](https://docs.rs/tokio-io/0.1.12/tokio_io)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
tokio-io = "0.1"
```

Next, add this to your crate:

```rust
extern crate tokio_io;
```

You can find extensive documentation and examples about how to use this crate
online at [https://tokio.rs](https://tokio.rs). The [API
documentation](https://docs.rs/tokio-io) is also a great place to get started
for the nitty-gritty.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tokio by you, shall be licensed as MIT, without any additional
terms or conditions.
