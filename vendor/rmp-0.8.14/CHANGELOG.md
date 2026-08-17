# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased (0.8.11)
### Added
- Implemeneted support for `#![no_std]` in `rmpv`
   - Adds new `feature="std"` (on by default)
- Introduces new `RmpRead` and `RmpWrite` traits.
   - Needed because `std::io::Read` (and Write) are missing on `#![no_std]`
- Introduces new `Bytes` and `ByteBuf` wrappers, that implement RmpRead/RmpWrite for no\_std targets.

## 0.8.6 - 2017-04-23
### Added
- New `rmp::decode::read_str_from_slice` function for zero-copy reading strings from slices.

### Changed
- Deprecate `rmp::decode::read_str_ref`, because it's useless and inconvenient.

## 0.8.5 - 2017-03-13
### Fixed
- Fix compilation on rustc 1.13.0.

## 0.8.4 - 2017-03-09
### Added
- Derive `Debug` for `MarkerReadError`.

## 0.8.3 - 2017-03-04
### Fixed
- Fixed `write_sint`, so it should mark positive values as unsigned integers.
  Before this change it marked signed integers larger than `4294967296` with I64 marker, which is not consistent with other MessagePack implementations.
  Now it should mark such integers with U64 marker.

## 0.8.2 - 2017-02-01
### Added
- Conversion from `ValueWriteError` into I/O error.

## 0.8.1 - 2017-01-05
### Changed
- Fixed docs link.

## 0.8.0 - 2017-01-05
### Added
- Marker now implements `From` and `Into` traits for `u8`.
- Add `read_int` function, which allows to read integer values and automatically cast to the expected result type even if they aren't the same. An additional `OutOfRange` error will be returned in the case of failed numeric cast.
- Add `NumValueReadError` enum with additional `OutOfRange` variant to be able to detect whether integer decoding failed because of out of range.

### Changed
- Update `byteorder` dependency to 1.0.
- Unexpected EOF variant has been merged with the default one in the I/O Error enum.
- Function `write_sint` now encodes 64-bit signed integers using the most compact representation.
- Function `write_uint` now encodes 64-bit unsigned integers using the most compact representation.
- Rename `read_array_size` function to `read_array_len` for consistency.
- Rename `read_map_size` function to `read_map_len` for consistency.
- Make `FixedValueWriteError` struct private. All functions, that previously returned such kind of error now return the Standard I/O error.

### Removed
- Move `Value` and `ValueRef` enums and associated functions into the separate `rmpv` crate.
- Remove conversions from `byteorder` crate errors, because since 0.5 there are no such errors.
- Remove `write_sint_eff` function - its functionality can now be done using `write_sint` instead.
- Remove `write_uint_eff` function - its functionality can now be done using `write_uint` instead.
- Integral functions like `read_*_loosely` and `read_*_fit` were removed in favor of generic `read_int` function, which allows to read integral values and cast them to the specified result type even if they aren't the same.
- Remove `read_bin_borrow` function.

## 0.7.5 - 2016-07-24
### Added
- Add `is_*` methods for Value for checking its underlying value without explicit matching.
- Add `as_*` methods for Value for borrowing its underlying value.
- Value is now indexable by integer.

## 0.7.4 - 2016-07-18
### Added
- Value now can be initialized from primitive values using From trait.

## 0.7.3 - 2015-09-23
### Changed
- Restricted upper version limit for dependencies.

## 0.7.2 - 2015-09-23
### Added
- Implemented `Display` trait for `Value`.

## 0.7.1 - 2015-09-11
### Changed
- Use `to_owned` instead of `to_string` while converting `ValueRef` into `Value`.
  This change improves `ValueRef::to_owned()` method performance by approximately 10-20%.s Also after this change it's cheaper to decode directly into `ValueRef` with further converting to owned value rather than decoding directly into `Value`.

## 0.7.0 - 2015-08-24
### Changed
- The big single crate has been refactored, which results in three crates: `rmp`, `rmp-serialize` and `rmp-serde`.

## 0.6.0 - 2015-08-17
### Added
- Initial support for [Serde](https://github.com/serde-rs/serde) serializer and deserializer.
- Efficient bytes serialization with Serde.
- Efficient binaries deserialization with Serde using `ByteBuf`.
- Rust serialize Decoder now can provide the underlying reader both by reference or by value, destroying itself in the last case.

### Changed
- Update well-formness for `BigEndianRead` trait to be implemented only for sized types.
- Renamed `PositiveFixnum` marker to `FixPos`.
- Renamed `NegativeFixnum` marker to `FixNeg`.
- Renamed `FixedString` marker to `FixStr`.
- Renamed `FixedArray` marker to `FixArray`.
- Renamed `FixedMap` to `FixMap`.
- Minor documentation updates and markdown fixes.

## 0.5.1 - 2015-08-10
### Changed
- Now the `rustc_serialize::Encoder` should encode signed integers using the most effective underlying representation.
- Now the `rustc_serialize::Decoder` should properly map integers to the result type if the decoded value fits in
  result type's range.

## 0.5.0 - 2015-08-01
### Added
- New `ValueRef` value struct represents MessagePack'ed value, but unlike an owning `Value` it owns nothing except its
  structure. It means that all strings and binaries it contains are borrowed from the byte array from which the value
  was created.
- New `BorrowRead` trait, which looks like a standard `BufRead` but unlike the standard this has an explicit internal
  buffer lifetime, which allows to borrow from underlying buffer while mutating the type.
- Encoding function for `ValueRef` with its own error category.
- Decoding function for `ValueRef` with its own error category.
- Conversion method from `ValueRef` to `Value`.
- More benchmarks and tests.

### Changed
- Derive `Copy` trait for `Integer` and `Float` enums.

## 0.4.0 - 2015-07-17
### Added
- Low level `write_str` function allows to serialize the UTF-8 encoded strings the most efficient way.
- Low level `write_bin` function allows to serialize the binary array the most efficient way.
- Implemented `std::error::Error` trait for error types.

## 0.3.2 - 2015-07-05
### Changed
- Encoder now should return proper error types.

## 0.3.1 - 2015-06-28
### Changed
- Stabilizing enum serialization/deserialization. Now every enum is serialized as [int, [args...]].
- Suppressed some warnings appeared on updated compiler.

## 0.3.0 - 2015-06-25
### Added
- Enum serialization/deserialization.

## 0.2.2 - 2015-06-15
### Changed
- Minor integer decoding performance tweaking.

## 0.2.1 - 2015-05-30
### Added
 - Benchmarking module.

### Changed
- Increased string decoding performance by ~30 times.
- Exported `read_value` function to the `rmp::decode` module.
- Exported `Value` struct to the root crate namespace.

## 0.2.0 - 2015-05-27
### Added
- Introducing a `Value` algebraic data type, which represents an owning MessagePack object. It can
  be found in `rmp::value` module.
- The Value ADT encoding and decoding functions.
- Low-level ext type decoders.

## 0.1.1 - 2015-05-18
### Changed
- Added documentation and repository site in Cargo.toml.
- Added keywords to ease searching using crates.io.

## 0.1.0 - 2015-05-15
### Added
- Initial commit.
- This CHANGELOG file to hopefully serve as an evolving example of a standardized open source
  project CHANGELOG.
