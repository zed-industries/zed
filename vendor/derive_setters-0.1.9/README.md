# derive_setters

[![Latest Version](https://img.shields.io/crates/v/derive_setters.svg)](https://crates.io/crates/derive_setters)
![Requires rustc 1.68+](https://img.shields.io/badge/rustc-1.68+-red.svg)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/derive_setters)

Rust macro to automatically generates setter methods for a struct's fields. This can be used to add setters to a plain
data struct, or to help in implementing builders.

For a related library that creates separate builder types, see 
[`rust-derive-builder`](https://github.com/colin-kiegel/rust-derive-builder).

# Basic example

```rust
use derive_setters::*;

#[derive(Default, Setters, Debug, PartialEq, Eq)]
struct BasicStruct {
    #[setters(rename = "test")]
    a: u32,
    b: u32,
    c: u32,
}

assert_eq!(
    BasicStruct::default().test(30).b(10).c(20),
    BasicStruct { a: 30, b: 10, c: 20 },
);
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
for inclusion in enumset by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
