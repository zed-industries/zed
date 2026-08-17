## Unreleased

# 0.10.3 – 2025-08-26

- [add #[inline] to to_regular_range](https://github.com/phil-opp/rust-bit-field/pull/30)
- [allow empty range on get/set bits](https://github.com/phil-opp/rust-bit-field/pull/34)
- [Add Security Policy](https://github.com/phil-opp/rust-bit-field/pull/29)

# 0.10.2 – 2023-02-25

- Add `#[track_caller]` to methods ([#27](https://github.com/phil-opp/rust-bit-field/pull/27))

# 0.10.1 – 2020-08-23

- Added bit manipulation functions for 128-bit integers ([#24](https://github.com/phil-opp/rust-bit-field/pull/24))

## [0.10.0] - 2019-05-03
### Added
 - Support all range types (`Range`, `RangeInclusive`, `RangeFrom`, …) for `get_bits` and `set_bits` methods ([#22](https://github.com/phil-opp/rust-bit-field/pull/22))

### Changed
 - **Breaking**: `BitField` trait now has a `BIT_LENGTH` associated const instead of a `bit_length` associated function.
 - `BitField` and `BitArray` methods are now inlined which causes much higher performance.

## [0.9.0] - 2017-10-15
### Changed
 - Bit indexes in `BitField` is now `usize` instead of `u8`.

## [0.8.0] - 2017-07-16
### Added
 - `BitArray` trait to make bit indexing possible with slices.
### Changed
 - `bit_length` in `BitField` is now an associated function instead of a method (`bit_length()` instead of `bit_length(&self)`)

## [0.7.0] - 2017-01-16
### Added
 - `BitField` was also implemented for: `i8`, `i16`, `i32`, `i64` and `isize`
### Changed
 - `length()` method in `BitField` is now called `bit_length()`
 - `get_range()` method in `BitField` is now called `get_bits()`
 - `set_range()` method in `BitField` is now called `set_bits()`
### Removed
 - `zero()` and `one()` constructor was removed from `BitField` trait.
