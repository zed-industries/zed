[![version-badge][]][version] [![license-badge][]][license] [![rust-version-badge][]][rust-version]

Another Rust [Base58][] codec implementation.

Compared to [`base58`][] this is significantly faster at decoding (about
2.4x as fast when decoding 32 bytes), almost the same speed for encoding
(about 3% slower when encoding 32 bytes), doesn't have the 128 byte
limitation and supports a configurable alphabet.

Compared to [`rust-base58`][] this is massively faster (over ten times as
fast when decoding 32 bytes, almost 40 times as fast when encoding 32
bytes), has no external dependencies and supports a configurable alphabet.

# Rust Version Policy

This crate only supports the current stable version of Rust, patch releases may
use new features at any time.

# License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[version-badge]: https://img.shields.io/crates/v/bs58.svg?style=flat-square
[version]: https://crates.io/crates/bs58
[license-badge]: https://img.shields.io/crates/l/bs58.svg?style=flat-square
[license]: #license
[rust-version-badge]: https://img.shields.io/badge/rust-latest%20stable-blueviolet.svg?style=flat-square
[rust-version]: #rust-version-policy

[Base58]: https://en.wikipedia.org/wiki/Base58
[`base58`]: https://github.com/debris/base58
[`rust-base58`]: https://github.com/nham/rust-base58
[clippy]: https://github.com/rust-lang-nursery/rust-clippy
