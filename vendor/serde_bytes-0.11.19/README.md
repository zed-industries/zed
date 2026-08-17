# serde\_bytes [![Build Status](https://img.shields.io/github/actions/workflow/status/serde-rs/bytes/ci.yml?branch=master)](https://github.com/serde-rs/bytes/actions?query=branch%3Amaster) [![Latest Version](https://img.shields.io/crates/v/serde_bytes.svg)](https://crates.io/crates/serde_bytes)

Wrapper types to enable optimized handling of `&[u8]` and `Vec<u8>`.

```toml
[dependencies]
serde_bytes = "0.11"
```

## Explanation

Without specialization, Rust forces Serde to treat `&[u8]` just like any
other slice and `Vec<u8>` just like any other vector. In reality this
particular slice and vector can often be serialized and deserialized in a
more efficient, compact representation in many formats.

When working with such a format, you can opt into specialized handling of
`&[u8]` by wrapping it in `serde_bytes::Bytes` and `Vec<u8>` by wrapping it
in `serde_bytes::ByteBuf`.

Additionally this crate supports the Serde `with` attribute to enable efficient
handling of `&[u8]` and `Vec<u8>` in structs without needing a wrapper type.

## Example

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Efficient<'a> {
    #[serde(with = "serde_bytes")]
    bytes: &'a [u8],

    #[serde(with = "serde_bytes")]
    byte_buf: Vec<u8>,
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
