unicode-general-category
========================

<div align="center">
  <a href="https://github.com/yeslogic/unicode-general-category/actions/workflows/ci.yml">
    <img src="https://github.com/yeslogic/unicode-general-category/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
  <a href="https://docs.rs/unicode-general-category">
    <img src="https://docs.rs/unicode-general-category/badge.svg" alt="Documentation"></a>
  <a href="https://crates.io/crates/unicode-general-category">
    <img src="https://img.shields.io/crates/v/unicode-general-category.svg" alt="Version"></a>
  <img src="https://img.shields.io/badge/unicode-16.0-informational" alt="Unicode Version">
  <a href="https://github.com/yeslogic/unicode-general-category/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/unicode-general-category.svg" alt="License"></a>
</div>

<br>

Fast lookup of the Unicode General Category property for `char` in Rust using
Unicode 16.0 data. This crate is no-std compatible.

Usage
-----

```rust
use unicode_general_category::{get_general_category, GeneralCategory};

fn main() {
    assert_eq!(get_general_category('A'), GeneralCategory::UppercaseLetter);
}
```

Performance & Implementation Notes
----------------------------------

[ucd-generate] is used to generate `tables.rs`. A build script (`build.rs`)
compiles this into a two level look up table. The look up time is constant as
it is just indexing into two arrays.

The two level approach maps a code point to a block, then to a position within
a block. The allows the second level of block to be deduplicated, saving space.
The code is parameterised over the block size, which must be a power of 2. The
value in the build script is optimal for the data set.

This approach trades off some space for faster lookups. The tables take up
about 45KiB. Benchmarks showed this approach to be ~5–10× faster than the
typical binary search approach.

It's possible there are further optimisations that could be made to eliminate
some runs of repeated values in the first level array.

[ucd-generate]: https://github.com/yeslogic/ucd-generate
