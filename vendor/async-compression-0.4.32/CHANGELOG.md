# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.30](https://github.com/Nullus157/async-compression/compare/async-compression-v0.4.29...async-compression-v0.4.30) - 2025-08-31

### Other

- rm unused dep from async-compression and compression-codecs ([#381](https://github.com/Nullus157/async-compression/pull/381))

## [0.4.29](https://github.com/Nullus157/async-compression/compare/async-compression-v0.4.28...async-compression-v0.4.29) - 2025-08-28

### Other

- Update Deps.rs badge ([#380](https://github.com/Nullus157/async-compression/pull/380))
- move async-compression to crates/ ([#379](https://github.com/Nullus157/async-compression/pull/379))

## [0.4.28](https://github.com/Nullus157/async-compression/compare/async-compression-v0.4.27...async-compression-v0.4.28) - 2025-08-23

### Fixed

- fix wasi ci testing and update doc in README ([#367](https://github.com/Nullus157/async-compression/pull/367))

### Other

- Fix Cargo.toml: add back version for async-compression ([#372](https://github.com/Nullus157/async-compression/pull/372))
- Have separate package.version field for compression-* ([#369](https://github.com/Nullus157/async-compression/pull/369))
- Re-export compression_codecs as codecs ([#368](https://github.com/Nullus157/async-compression/pull/368))
- Fix breaking API change ([#366](https://github.com/Nullus157/async-compression/pull/366))
- Fix docs.rs build for compression-codecs ([#365](https://github.com/Nullus157/async-compression/pull/365))
- Separate codecs as a separate crate, allow direct configuration ([#363](https://github.com/Nullus157/async-compression/pull/363))
- *(deps)* bump actions/checkout from 4 to 5 ([#360](https://github.com/Nullus157/async-compression/pull/360))
- Fix doc link for futures-io ([#361](https://github.com/Nullus157/async-compression/pull/361))
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.4.32](https://github.com/Nullus157/async-compression/compare/async-compression-v0.4.31...async-compression-v0.4.32) - 2025-09-25

### Other

- updated the following local packages: compression-codecs

## [0.4.31](https://github.com/Nullus157/async-compression/compare/async-compression-v0.4.30...async-compression-v0.4.31) - 2025-09-25

### Other

- Flush compressed data out of encoders more often ([#383](https://github.com/Nullus157/async-compression/pull/383))

## [0.4.27](https://github.com/Nullus157/async-compression/compare/v0.4.26...v0.4.27) - 2025-07-13

### Fixed

- Zstd fastest level now does compression. ([#357](https://github.com/Nullus157/async-compression/pull/357))

## [0.4.26](https://github.com/Nullus157/async-compression/compare/v0.4.25...v0.4.26) - 2025-07-13

### Features

- Add multi-thread support for XZ. ([#353](https://github.com/Nullus157/async-compression/pull/353))

## [0.4.25](https://github.com/Nullus157/async-compression/compare/v0.4.24...v0.4.25) - 2025-06-17

### Changed

- Update `bzip2` dependency to `0.6`. ([#350](https://github.com/Nullus157/async-compression/pull/350))

## [0.4.24](https://github.com/Nullus157/async-compression/compare/v0.4.23...v0.4.24) - 2025-06-09

### Other

- Fix clippy error
- narrow tokio requirement to account for RUSTSEC-2025-0023
- move constructors back to top of rustdoc pages

## [0.4.23](https://github.com/Nullus157/async-compression/compare/v0.4.22...v0.4.23) - 2025-04-21

### Changed

- Update `brotli` dependency to `8.0`.
- Update `liblzma` dependency to `0.4`.

## [0.4.22](https://github.com/Nullus157/async-compression/compare/v0.4.21...v0.4.22) - 2025-03-25

### Added

- Add LZ4 encoders and decoders.
- Expose `DeflateEncoder::{total_in, total_out}()` methods.


## [0.4.21](https://github.com/Nullus157/async-compression/compare/v0.4.20...v0.4.21) - 2025-03-15

### Fixed

- When flate encoding, do not mark internal state as flushed if it ran out of buffer space.
- Add debug assertion in `produce` method to check buffer capacity in implementations for `BufWriter`.

## [0.4.20](https://github.com/Nullus157/async-compression/compare/v0.4.19...v0.4.20) - 2025-02-28

### Added

- Add support for `wasm32-wasip1-*` targets.

## [0.4.19](https://github.com/Nullus157/async-compression/compare/v0.4.18...v0.4.19) - 2025-02-27

### Changed

- Update `bzip2` dependency to `0.5`.

### Fixed

- Ensure that flush finishes before continuing.

## [0.4.18](https://github.com/Nullus157/async-compression/compare/v0.4.17...v0.4.18) - 2024-11-23

### Fixed

- Adjust `Level::Precise` clamp range for flate2.

## [0.4.17](https://github.com/Nullus157/async-compression/compare/v0.4.16...v0.4.17) - 2024-10-20

### Fixed

- Fix occasional panics when consuming from pending buffers.

## [0.4.16](https://github.com/Nullus157/async-compression/compare/v0.4.15...v0.4.16) - 2024-10-16

### Other

- Implement pass-through `AsyncBufRead` on write-based encoders & decoders.

## [0.4.15](https://github.com/Nullus157/async-compression/compare/v0.4.14...v0.4.15) - 2024-10-13

### Feature
- Implement pass-through `AsyncRead` or `AsyncWrite` where appropriate.
- Relax `AsyncRead`/`AsyncWrite` bounds on `*::{get_ref, get_mut, get_pin_mut, into_inner}()` methods.

## [0.4.14](https://github.com/Nullus157/async-compression/compare/v0.4.13...v0.4.14) - 2024-10-10

### Fixed
- In Tokio-based decoders, attempt to decode from internal state even if nothing was read.

## [0.4.13](https://github.com/Nullus157/async-compression/compare/v0.4.12...v0.4.13) - 2024-10-02

### Feature
- Update `brotli` dependency to to `7`.

## [0.4.12](https://github.com/Nullus157/async-compression/compare/v0.4.11...v0.4.12) - 2024-07-21

### Feature
- Enable customizing Zstd decoding parameters.

## [0.4.11](https://github.com/Nullus157/async-compression/compare/v0.4.10...v0.4.11) - 2024-05-30

### Other
- Expose total_in/total_out from underlying flate2 encoder types.

## [0.4.10](https://github.com/Nullus157/async-compression/compare/v0.4.9...v0.4.10) - 2024-05-09

### Other
- *(deps)* update brotli requirement from 5.0 to 6.0 ([#274](https://github.com/Nullus157/async-compression/pull/274))
- Fix pipeline doc: Warn on unexpected cfgs instead of error ([#276](https://github.com/Nullus157/async-compression/pull/276))
- Update name of release-pr.yml
- Create release.yml
- Create release-pr.yml

## 0.4.9

 - bump dep brotli from 4.0 to 5.0

## 0.4.8

 - bump dep brotli from 3.3 to 4.0

## 0.4.7

- Flush available data in decoder even when there's no incoming input.

## 0.4.6

- Return errors instead of panicking in all encode and decode operations.

## 0.4.5

- Add `{Lzma, Xz}Decoder::with_mem_limit()` methods.

## 0.4.4

- Update `zstd` dependency to `0.13`.

## 0.4.3

- Implement `Default` for `brotli::EncoderParams`.

## 0.4.2

- Add top-level `brotli` module containing stable `brotli` crate wrapper types.
- Add `BrotliEncoder::with_quality_and_params()` constructors.
- Add `Deflate64Decoder` behind new crate feature `deflate64`.

## 0.4.1 - 2023-07-10

- Add `Zstd{Encoder,Decoder}::with_dict()` constructors.
- Add `zstdmt` crate feature that enables `zstd-safe/zstdmt`, allowing multi-threaded functionality to work as expected.

## 0.4.0 - 2023-05-10

- `Level::Precise` variant now takes a `i32` instead of `u32`.
- Add top-level `zstd` module containing stable `zstd` crate wrapper types.
- Add `ZstdEncoder::with_quality_and_params()` constructors.
- Update `zstd` dependency to `0.12`.
- Remove deprecated `stream`, `futures-bufread` and `futures-write` crate features.
- Remove Tokio 0.2.x and 0.3.x support (`tokio-02` and `tokio-03` crate features).

## 0.3.15 - 2022-10-08

- `Level::Default::into_zstd()` now returns zstd's default value `3`.
- Fix endianness when reading the `extra` field of a gzip header.
