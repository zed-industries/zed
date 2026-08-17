# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [Unreleased]
### Added

### Changed

### Removed

## [7.5.4] - 2023-11-18
### Changed
- `base64` dependency was bumped to 0.21
- `clap` dependency was bumped to major version 4
- MSRV was bumped to 1.64.0 (due to clap)

## [7.5.3] - 2023-11-11
### Changed

- Fix record iteration when recording only zeros (#125)

## [7.5.2] - 2022-09-18
### Changed
- Optimize `Histogram::clone` ([#113])
- Optimize `Histogram::add` when the histogram parameters are equivalent and the target histogram is empty ([#113])

## [7.5.1] - 2022-08-13
### Changed
- Significantly optimized `quantile_below` ([#112])

[#112]: https://github.com/HdrHistogram/HdrHistogram_rust/pull/112

## [7.5.0] - 2022-02-12

### Added
- `Histogram::is_auto_resize` ([#106])

[#106]: https://github.com/HdrHistogram/HdrHistogram_rust/pull/106

## [7.4.0] - 2021-10-24

### Changed
- Minimum supported Rust version bumped to 1.48 (due to `nom 7`)
- `nom` dependency was bumped to 7.0.0 ([#102])

[#102]: https://github.com/HdrHistogram/HdrHistogram_rust/pull/102

## [7.3.0] - 2021-05-04
### Added
 - Implementations of `Add` and `Sub` for `Histogram<T>`.

### Changed
 - Fixed benchmarks, which didn't compile.

## [7.2.0] - 2020-11-29
### Changed
- Minimum supported Rust version bumped to 1.44 (due to `nom 6`)
- `crossbeam-channel` dependency was bumped to 0.5
- `nom` dependency was bumped to 6.0.1
- `base64` dependency was bumped to 0.13

## [7.1.0] - 2020-05-21
### Changed
- Minimal rust version was increased to 1.37.0
- `base64` dependency was bumped to 0.12

## [7.0.0] - 2019-12-20
### Added
- `std::error:Error` implementation for internal Error types: `CreationError`, `AdditionError`, `SubtractionError`, `RecordError`, `UsizeTypeTooSmall`, `DeserializeError`, `IntervalLogWriterError`, `V2DeflateSerializeError`, `V2SerializeError`
- Changelog

### Changed
- `DeserializeError` and `V2DeflateSerializeError` lost derived traits: `PartialEq`, `Eq`, `Clone`, `Copy`
- Inner error type from `std::io::ErrorKind` to `std::io::Error` in types: `DeserializeError`, `V2DeflateSerializeError`, `V2SerializeError`, `IntervalLogWriterError` to support `Display`.

[Unreleased]: https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.5.2...HEAD
[7.5.2]: https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.5.1...v7.5.2
[7.5.1]: https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.5.0...v7.5.1
[7.5.0]: https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.4.0...v7.5.0
[7.4.0]: https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.3.0...v7.4.0
[7.3.0]: https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.2.0...v7.3.0
[7.2.0]: https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.1.0...v7.2.0
[7.1.0]: https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.0.0...v7.1.0
[7.0.0]: https://github.com/HdrHistogram/HdrHistogram_rust/compare/v6.3.4...v7.0.0
