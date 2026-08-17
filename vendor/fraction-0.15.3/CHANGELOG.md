# Change Log

## [0.15.3] - 2024-05-12
### Changed
 - `num` crate min required version is now `0.4.2`

## [0.15.2] - 2024-04-28
### Added
 - GenericFraction ConstOne and ConstZero trait implementations (special thanks to Raimundo Saona, aka @saona-raimundo)

## [0.15.1] - 2024-02-11
### Added
 - "with-unicode" feature implementation to format (and parse) floats with Unicode characters (special thanks to @feefladder)

## [0.15.0] - 2024-01-01
### Added
 - GenericFraction try_from/try_into implementations for primitive types (u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64)
 - GenericFraction try_from/try_into implementations for BigInt/BigUint ("with-bigint" feature)
 - GenericDecimal try_from/try_into implementations for primitive types (u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64)
 - GenericDecimal try_from/try_into implementations for BigInt/BigUint ("with-bigint" feature)
 - Tests for all GenericDecimal ops (overloadable operators - std::ops)

### Changed
 - GenericDecimal ops (overloadable operators) refactoring. Each operator is now implemented separately in its own module, allowing decoupled code with its own optimisations and tests.

### Fixed
 - PartialOrd will now reuse Ord implementation where possible (refactor to make clippy happier)
 - Fixed logic in `sub_assign` and `checked_sub` that could sometimes produce incorrect results for a couple of edge cases with a negative zero (`-0`) as an operand.

## [0.14.0] - 2023-09-27
### Added
 - `approx` module with "Approximate mathematical operations", `fn sqrt` (special thanks to @squ1dd13 for the contribution!)
 - `with-approx` feature, enabling the `approx` module


## [0.13.1] - 2023-02-25
### Added
 - Clippy hint to allow manual filtering in GenericFraction::to_i64 implementation
 - A couple of tests for floor/ceil functions

### Fixed
 - ceil & floor incorrect behaviour for negative numbers (special thanks to Morris Hansing aka morri2)

## [0.13.0] - 2023-01-01

### Changed
 - `PartialCmp` now compares `NaN` with the other values and returns it as the smallest possible value. Thus, `NaN < -Inf`.

### Fixed
 - `partial_cmp` now behaves exactly the same as `cmp` (PartialOrd is now consistent with Ord).
    This fixes an issue introduced in `0.12.2` with the implementation of `Ord`,
    where `Ord` would behave differently from `PartialOrd` with `NaN` values.
    Special thanks to Hsingai Tigris Altaica aka DrAlta for fixing this.

## [0.12.2] - 2022-12-04

### Added
 - `Ord` trait implementation for GenericFraction and GenericDecimal (special thanks to Hsingai Tigris Altaica aka DrAlta)

## [0.12.1] - 2022-10-18

### Added
 - Support to `add` castable values (e.g. `f + 1u8`, 1u8 will be transparently casted to `Fraction` with `.into()`)
 - Support to `sub` castable values (e.g. `f - 1u8`, 1u8 will be transparently casted to `Fraction` with `.into()`)
 - Support to `div` castable values (e.g. `f / 1u8`, 1u8 will be transparently casted to `Fraction` with `.into()`)
 - Support to `mul` castable values (e.g. `f * 1u8`, 1u8 will be transparently casted to `Fraction` with `.into()`)
 - Support to `add_assign` castable values (e.g. `f += 1u8`, 1u8 will be transparently casted to `Fraction` with `.into()`)
 - Support to `sub_assign` castable values (e.g. `f -= 1u8`, 1u8 will be transparently casted to `Fraction` with `.into()`)
 - Support to `div_assign` castable values (e.g. `f /= 1u8`, 1u8 will be transparently casted to `Fraction` with `.into()`)
 - Support to `mul_assign` castable values (e.g. `f *= 1u8`, 1u8 will be transparently casted to `Fraction` with `.into()`)

### Changed
 - Refactoring of the fraction module. std::ops implementations moved into separate submodules.
 - generic::read_generic_integer performance improved for when target type matches source (~83% improvement, which is 5 times faster).
   As the result this can affect GenericFraction::from performance for non-float types.
 - From<(A, B)> implementation is migrated to GenericFraction::new_generic (~85% performance improvement and with no heap allocations, which is ~7 times faster).

## [0.12.0] - 2022-10-13

### Changed
 - `num` version `0.4` is now required (`0.2`, `0.3` are no longer supported)
 - Multiple functions made const in GenericFraction, GenericDecimal and fraction::display::Format
   Special thanks to Stijn Frishert (aka stijnfrishert).

### Deprecated
 - fn `decimal::GenericDecimal::apply_ref` is deprecated.

### Removed
 - Removed deprecated fn `decimal::GenericDecimal::from_decimal_str`. Use `FromStr::from_str` instead.
 - Removed deprecated fn `fraction::GenericFraction::from_decimal_str`. Use `FromStr::from_str` instead.
 - Removed deprecated fn `fraction::GenericFraction::format_as_decimal`. Use `format!(\"{:.1$}\", fraction, precision)` instead.
 - Removed deprecated fn `fraction::GenericFraction::new_raw_neg`. Use `new_raw_signed` instead.

## [0.11.2] - 2022-09-18
- `DynaInt` now implements serde `Serialize & Unserialize` (Thanks to Richard Davies aka @optevo for the contribution!)

## [0.11.1] - 2022-08-07

### Fixed
 - Fraction::from\_str trims trailing zeroes before calculating denom (Thanks to @khigia for the contribution!)

## [0.11.0] - 2022-06-19

### Changed
 - `num` dependency versions extended from `0.2` to `>=0.2,<5` (Thanks to Joel Natividad aka jqnatividad for the contribution!)
 - `lazy_static` dependency versions extended from `1.1` to `1`

## [0.10.0] - 2022-02-07

### Added
 - `std::str::FromStr` trait implementation for GenericFraction and GenericDecimal
  Special thanks to Scott Wilson for the contribution
 - Deprecated `GenericFraction::from_decimal_str` and `GenericDecimal::from_decimal_str` in favour of of `std::str::FromStr`


## [0.9.0] - 2021-08-22

### Added
 - Default trait implementation for GenericFraction and GenericDecimal
 - `postgres-types: ^0.2` and `bytes: 1` are new optional dependencies (feature: with-postgres-support)
 - `GenericFraction::new_raw_signed` constructor

### Changed
 - Juniper supported version upgraded from 0.11 to 0.15
 - Postgres supported version upgraded from 0.15 to 0.19 (might be down to 0.16, but untested).

### Removed
 - `postgres` crate is not a dependency any longer
 - Deprecated fn `GenericFraction::format_as_decimal` is removed

## [0.8.0] - 2020-12-17
### Changed
 - More efficient f32/f64 conversion to Fractions and Decimals (up to 10 times faster and not using memory allocation anymore)
   Special thanks to Christopher Rabotin for the contribution!

## [0.7.0] - 2020-12-05
### Added
 - fraction::display::Format implements Clone trait (becomes cloneable)
 - fraction::Sign implements PartialOrd and Ord traits (becomes orderable)
 - GenericDecimal::calc_precision max_precision optional argument to limit the calculation

### Changed
 - Decimal::from_fraction now limits precision calculation to 255
 - rustfmt for the whole codebase
 - small readability refactoring for some methods

## [0.6.3] - 2020-05-07
### Added
 - std::iter::{Sum, Product} implemented for GenericFraction and GenericDecimal

## [0.6.2] - 2019-05-20
### Addad
 - std::error::Error implemented for error::ParseError

## [0.6.1] - 2019-03-29
### Added
 - dynaint, `Into<BigUint>` implementation
 - fraction, `GenericFraction::into_fraction<I>` method implementation

## [0.6.0] - 2019-02-05
### Added
- division::divide_to_callback implementation. Some other functions refactored to be using it internally
- fraction::display module implementation, Fraction ::std::fmt::Display implementation supporting all the features of the std::fmt::Formatter
- Decimal ::std::fmt::Display implementation supporting all the features of the std::fmt::Formatter

### Changed
- Juniper updated to 0.11
- GenericFraction::format_as_decimal is now deprecated. Use format! macro instead, or division module if you need more control
- division module functions signatures now have flags for trailing zeroes

### Removed
- Deprecated functions

## [0.5.0] - 2018-11-26
### Changed
- Division module API; functions to return division state for later reuse (remainder and divisor)


## [0.4.1] - 2018-10-19
### Added
 - `DynaInt`, initial `std::fmt::Display` implementation


## [0.4.0] - 2018-10-10
### Bugs
 - `Hash` implementation for `GenericFraction` now returns equal hashes for negative and positive zeroes

### Added
- Lossless division, fraction decimal representation with infinite precision
- Decimal type, built on top of Fraction
- PostgreSQL integration
- Juniper integration
- Types with dynamic growth into heap on overflow
- Generic integer conversions (usize -> i8, i8 -> u8, etc)
- Examples and documentation for new features
- Re-exporting the bunch of `num` traits so that the library can be used without explicit dependency on `num`

### Refactoring
- The lib has been split into modules with separate features

### Modules
- `convert` module with traits for optimistic convesion
- `decimal` module with `GenericDecimal` implementation, `Juniper` and `PostgreSQL` integration for it
- `division` module with lossless infinite division implementation without memory allocation
- `dynaint` module with `DynaInt` type implementation (dynamically growing integer)
- `error` module with shared library error types
- `fraction` module with `GenericFraction` implementation, `Juniper` and `PostgreSQL` integration for it
- `generic` module with `GenericInteger` trait implementation, generic integer types conversion
- `prelude` module with some predefined type aliases such as `Fraction` and `Decimal`
- `tests` module with some tests

### Features
- `with-bigint` (default), integration with `num::{BigInt, BigUint}` types
- `with-decimal` (default), `GenericDecimal` type implementation
- `with-dynaint` (default), `DynaInt` type implementation
- `with-juniper-support`, `Juniper 0.10` integration
- `with-postgres-support`, `Postgres 0.15` integration

### Changes
- `GenericFraction` redundant methods deprecated: `new_nan`, `new_inf`, `new_inf_neg`, `into_big`, `format_as_float`


## [0.3.7] - 2017-11-10
### Bugs
 - Fix comparisons with negative numbers


## [0.3.6] - 2017-07-27
### Bugs
- `T in GenericFraction<T>` is `Clone + Integer` from now onwards (thanks to Taryn Hill aka Phrohdoh)


## [0.3.5] - 2017-04-17
### Changed
 - `num` package dependency version updated from "0.1.36" to "0.1.37"


## [0.3.4] - 2016-12-11
### Bugs
- `fn _new` now returns NaN for 0/0 (was Infinity before)
- `fn sign` now returns values for GenericFraction::Infinite values too
- `fn neg_zero` now returns zero with negative sign (was positive before)
- `fn recip` now handles zero values gracefully (does not panic, returns Infinity)

### Added
- Lots of documentation


## [0.3.3] - 2016-11-18
### Refactoring
- More efficient implementation of `From<[unsigned ints]>`
- More generic implementation of `From<BigInt>`


## [0.3.2] - 2016-11-13
### Refactoring
- `Zero::is_zero` to be used everywhere in math, rather than making new zero vals + comparing with them

### Added
- `fn new_nan` constructor
- `fn new_inf` constructor
- `fn new_inf_neg` constructor


## [0.3.1] - 2016-11-12
### Added
- `fn format_as_float` implemented for `GenericFraction<T>` (it was only available for BigFraction before)

### Changed
- `Into<T>` to be used in bounds rather than `From<N>`, since it's more flexible (thanks to Alexander Altman for the patch)
- number of bug fixes within `fn format_as_float` + test coverage


## [0.3.0] - 2016-11-08
### Added
- `From<(N, D)>` generic implementation (through std::fmt::Display)

### Changed
- `GenericFraction<T>` copy semantic to be applied only when `T: Clone`
- `GenericFraction` impl, constructors refactoring (new, new_raw)
- `num` upgraded up to `0.1.36` (from `0.1.34`)

### Removed
- `fn new` does not perform type casting through fmt::Display anymore (that functionality moved out as `From<(N, D)>`)


## [0.2.2] - 2016-09-17
### Added
- `impl From<num::BigInt> for BigFraction`
- `impl From<num::BigUint> for BigFraction`


## [0.2.1] - 2016-08-28
### Changed
- Package description has been changed


## [0.2.0] - 2016-08-28
### Added
- [num crate](https://crates.io/crates/num) is a dependency now
- `GenericFraction<T>` implemented upon `num::Ratio<T>`
- `BigFraction` implementation based on `num::BigRational` (using heap)
- `num::traits::Bounded` trait implemented
- `fn min_positive_value` implemented
- `num::traits::ToPrimitive` trait implemented
- `num::traits::Signed` trait implemented
- `From<f64>` trait implementation now relies on `format!` macro instead of `f64::fract`
- `BigFraction` struct using `num::BigUint`
- `fn format_as_float` for BigFraction has been implemented

### Changed
- The codebase has been rewritten and the license has been changed from `LGPL-3` to `MIT/Apache2` dual
- no more convertions into INFINITY on arithmetic overflows
- `fn to_f64` now returns `Option<f64>` instead of `f64` (`num::trait::ToPrimitive` implementation)
- `From` trait implementation uses `fmt::Display` from now on

### Removed
- `fn unpack` removed
- `std::cmp::Ord` implementation removed in regard to `NaN` values


## [0.1.0] - 2016-01-24
### Added
- Basic implementation
