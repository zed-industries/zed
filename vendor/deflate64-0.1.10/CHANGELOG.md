# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/

## [Unreleased]
### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.1.10] - 2025-10-01
## [0.1.9] - 2024-07-16
### Fixed
- Infinite loop with empty output buffer in `Deflate64Decoder` [`#30`](https://github.com/anatawa12/deflate64-rs/pull/30)

## [0.1.8] - 2024-03-11
### Fixed
- Panic with Invalid Data [`#26`](https://github.com/anatawa12/deflate64-rs/pull/26) [`#24`](https://github.com/anatawa12/deflate64-rs/pull/24)

## [0.1.7] - 2024-01-27
## [0.1.6] - 2023-10-15
### Fixed
- Overflow substract with some archive [`#14`](https://github.com/anatawa12/deflate64-rs/pull/14)

## [0.1.5] - 2023-08-19
### Added
- test: 7zip compatibility test
- Changelog file

### Changed
- Remove `unsafe` code

## [0.1.4]
### Added
- `Deflate64Decoder`, Streaming `Read` decoder implementation

### Fixed
- Overflow error in debug build

## [0.1.3]
### Added
- Many documentation comment
- `InflaterManaged.errored()`

### Changed
- Remove Box usage in `InflaterManaged`

## [0.1.2] - 2023-01-16
### Fixed
- Release build will cause compilation error

### Fixed
- Several bugs

## [0.1.1] - 2023-01-15
### Added
- Implement Debug in many struct

## [0.1.0] - 2023-07-29
### Added
- Initial Deflate64 implementation

[Unreleased]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.10...HEAD
[0.1.10]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/anatawa12/deflate64-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/anatawa12/deflate64-rs/releases/tag/v0.1.0
