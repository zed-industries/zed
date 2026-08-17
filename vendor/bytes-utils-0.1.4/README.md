# Additional utils for the [bytes] crate

[![Actions Status](https://github.com/vorner/bytes-utils/workflows/test/badge.svg)](https://github.com/vorner/bytes-utils/actions)
[![codecov](https://codecov.io/gh/vorner/bytes-utils/branch/main/graph/badge.svg?token=GKITN8ZOE1)](https://codecov.io/gh/vorner/bytes-utils)
[![docs](https://docs.rs/bytes-utils/badge.svg)](https://docs.rs/bytes-utils)

Few utilities to make working with the types and traits in the [bytes] crate
even more convenient and powerful.

Currently contains:

* `SegmentedBuf` that can concatenate multiple `Buf`s into a bigger one without
  copying.
* `Str` and `StrMut`, string wrappers around `Bytes` and `BytesMut`.

## Features

`no_std` builds are supported by disabling the `std` feature, which is enabled by default.

# License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

[bytes]: https://docs.rs/bytes
