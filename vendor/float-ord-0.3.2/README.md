Total ordering for floats
=========================

[![Build Status](https://travis-ci.org/notriddle/rust-float-ord.svg?branch=master)](https://travis-ci.org/notriddle/rust-float-ord)
[![Crates.IO](https://img.shields.io/crates/v/float-ord.svg)](https://crates.io/crates/float-ord)
[![Documentation](https://docs.rs/float-ord/badge.svg)](https://docs.rs/float-ord)

A wrapper for floats that uses this ordering:

    NaN | -Infinity | x < 0 | -0 | +0 | x > 0 | +Infinity | NaN

## How does it work?

There is an [old family magic spell](http://stereopsis.com/radix.html) that
allows one to compare floating-point values without any floating-point work.
Simply interpret the fp value as an unsigned integer, flip its sign bit (if
positive) or all bits (if negative), and do the comparison normally.

The trick was developed on `f32` and `f64`, but it should work on anything
structured like the IEEE floats. Even the IEEE decimal formats.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
