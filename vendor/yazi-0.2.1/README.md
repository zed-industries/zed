# Yazi - Yet another zlib implementation

Yazi is a pure Rust implementation of the RFC 1950 DEFLATE specification with support for
the zlib wrapper. It provides streaming compression and decompression.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Apache 2.0 or MIT license.][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/yazi.svg
[crates-url]: https://crates.io/crates/yazi
[docs-badge]: https://docs.rs/yazi/badge.svg
[docs-url]: https://docs.rs/yazi
[license-badge]: https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg
[license-url]: #license

## Usage

The following demonstrates simple usage for compressing and decompressing in-memory buffers:

```rust
use yazi::*;
// Your source data.
let data = &(0..=255).cycle().take(8192).collect::<Vec<u8>>()[..];
// Compress it into a Vec<u8> with a zlib wrapper using the default compression level.
let compressed = compress(data, Format::Zlib, CompressionLevel::Default).unwrap();
// Decompress it into a Vec<u8>.
let (decompressed, checksum) = decompress(&compressed, Format::Zlib).unwrap();
// Verify the checksum.
assert_eq!(Adler32::from_buf(&decompressed).finish(), checksum.unwrap());
// Verify that the decompressed data matches the original.
assert_eq!(data, &decompressed[..]);
```

For detail on more advanced usage, see the full API [documentation](https://docs.rs/yazi).

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
