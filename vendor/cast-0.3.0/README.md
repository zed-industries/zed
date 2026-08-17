[![crates.io](https://img.shields.io/crates/d/cast.svg)](https://crates.io/crates/cast)
[![crates.io](https://img.shields.io/crates/v/cast.svg)](https://crates.io/crates/cast)

# `cast`

> Ergonomic, checked cast functions for primitive types

``` rust
extern crate cast;

// `u8` and `u16` are checked cast functions, use them to cast from any numeric
// primitive to `u8`/`u16` respectively
use cast::{u8, u16, Error};

// Infallible operations, like integer promotion, are equivalent to a normal
// cast with `as`
assert_eq!(u16(0u8), 0u16);

// Everything else will return a `Result` depending on the success of the
// operation
assert_eq!(u8(0u16), Ok(0u8));
assert_eq!(u8(256u16), Err(Error::Overflow));
assert_eq!(u8(-1i8), Err(Error::Underflow));
assert_eq!(u8(1. / 0.), Err(Error::Infinite));
assert_eq!(u8(0. / 0.), Err(Error::NaN));
```

## [API docs](https://docs.rs/cast)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
