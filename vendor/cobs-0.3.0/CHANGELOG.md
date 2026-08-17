Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

# [unreleased]

# [v0.3.0] 2025-01-31

## Added

- Added `max_encoding_overhead` const function.
  [#20](https://github.com/jamesmunns/cobs.rs/pull/20)
- Unittests for `decode_in_place` and `decode_in_place_report`.
  [#36](https://github.com/jamesmunns/cobs.rs/pull/36)
- New `alloc` feature which enables support for APIs returning `Vec`s.
  [#24](https://github.com/jamesmunns/cobs.rs/pull/24)
- New `std` feature, deprecate the `use_std` feature but keep it for backwards compatiblity.
  [#38](https://github.com/jamesmunns/cobs.rs/pull/38)
- Documentation of the new `std` and `alloc` features in the README.
  [#38](https://github.com/jamesmunns/cobs.rs/pull/38)
- Optional `defmt` support feature-gated by the optional `defmt` feature.
  [#40](https://github.com/jamesmunns/cobs.rs/pull/40)
- Add optional `serde` support that adds `serde` derives for some errors and data structures.
  [#41](https://github.com/jamesmunns/cobs.rs/pull/41)

## Changed

- Bumped rust edition to 2021
- Improved error handling by replacing unit errors with a new `DecodeError` enumeration for
  the decoding module and a `DestBufTooSmallError` for the encoding module.
  [#30](https://github.com/jamesmunns/cobs.rs/pull/30)
- `max_encoding_length` is now a const function. [#20](https://github.com/jamesmunns/cobs.rs/pull/20)
- `decode_in_place` and `decode_in_place_report` now check the destination index and return
  an appropriate `DecodeError` instead of panicking if an out-of-bounds access happens.
  [#36](https://github.com/jamesmunns/cobs.rs/pull/36)
- Consistent behaviour of decode functions for empty input: All funtions now return
  `DecodeError::FrameEmpty`.
  [#39](https://github.com/jamesmunns/cobs.rs/pull/39)

## Fixed

- Fixed wrong encoded length when source length was divisible by 254.
  [#19](https://github.com/jamesmunns/cobs.rs/issues/19)
