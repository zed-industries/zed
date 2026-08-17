# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.1] - 2021-03-25

### Added

- Moved to `const generics` MVP

## [v0.1.5] - 2021-03-02

### Fixed

- Security issue.

## [v0.1.4] - 2020-09-23

### Added

- `generic_array` 0.14 support

### Fixed

- Handling of targets with small pointer types, e.g. AVR

## [v0.1.3] - 2020-02-17

### Added

- `As{,Mut}Slice` implementations for arrays whose lengths are powers of 2 up to
  `1 << 16`.

## [v0.1.2] - 2019-11-22

### Added

- Support for v0.13.x of `generic-array`, in addition to the existing support
  for v0.12.x of `generic-array`.

## [v0.2.0] - 2019-08-29 - YANKED

- The 0.1.1 release was a breaking change, now in 0.2.0

## [v0.1.1] - 2019-08-29 - YANKED

- Bumped `generic_array` dependency

## [v0.1.0] - 2018-05-23

- Initial release

[Unreleased]: https://github.com/japaric/as-slice/compare/v0.2.1...HEAD
[v0.2.0]: https://github.com/japaric/as-slice/compare/v0.1.5...v0.2.1
[v0.1.5]: https://github.com/japaric/as-slice/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/japaric/as-slice/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/as-slice/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/as-slice/compare/v0.1.1...v0.1.2
[v0.2.0]: https://github.com/japaric/as-slice/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/japaric/as-slice/compare/v0.1.0...v0.1.1
