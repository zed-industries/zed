# Bytes

A utility library for working with bytes.

[![Crates.io](https://img.shields.io/crates/v/bytes.svg?maxAge=2592000)](https://crates.io/crates/bytes)
[![Build Status](https://travis-ci.org/carllerche/bytes.svg?branch=master)](https://travis-ci.org/carllerche/bytes)

[Documentation](https://docs.rs/bytes/0.4.12/bytes/)

## Usage

To use `bytes`, first add this to your `Cargo.toml`:

```toml
[dependencies]
bytes = "0.4.12"
```

Next, add this to your crate:

```rust
extern crate bytes;

use bytes::{Bytes, BytesMut, Buf, BufMut};
```

## Serde support

Serde support is optional and disabled by default. To enable use the feature `serde`.

```toml
[dependencies]
bytes = { version = "0.4.12", features = ["serde"] }
```

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `bytes` by you, shall be licensed as MIT, without any additional
terms or conditions.

