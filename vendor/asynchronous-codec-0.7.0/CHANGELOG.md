# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0]

### Changed

- Use GATs for `Encoder` trait to allow encoding of borrowed data.
  See [PR 9](https://github.com/mxinden/asynchronous-codec/pull/9).

## [0.6.2]

### Fixed

- Error handling in `CborCodec` and `JsonCodec` `decode`, more specifically not advancing the data buffer on partial decoding. See [#7](https://github.com/mxinden/asynchronous-codec/pull/7) for details.

## [0.6.1] - 2022-11-08

### Added

- `Framed::send_high_water_mark` and `Framed::set_send_high_water_mark` [#3].

[#3]: https://github.com/mxinden/asynchronous-codec/pull/3

## [0.6.0] - 2021-02-01

### Changed

- Permit conversion into and creation from "parts"
  [#2](https://github.com/mxinden/asynchronous-codec/pull/2).

## [0.5.0] - 2021-01-06

### Changed

- Update to `bytes` `v1` and `pin-project-lite` `v0.2`.
