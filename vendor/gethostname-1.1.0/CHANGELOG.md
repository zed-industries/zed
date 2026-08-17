# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0] – 2025-10-14

### Changed
- Use windows-link instead of windows-targets (see [CB-22]).
- Bump MSRV to 1.71 (see [CB-22]).

[CB-22]: https://codeberg.org/swsnr/gethostname.rs/pulls/22

## [1.0.2] – 2025-05-01

### Changed
- Migrate from Github to <https://codeberg.org/swsnr/gethostname.rs>.
- Bump MSRV to 1.68

## [1.0.1] – 2025-03-25

### Changed
- `gethostname` now has a `must_use` attribute.
- Update rustix dependency to version 1.

## [1.0.0] – 2025-02-01

We can safely declare this stable now.

## [0.5.0] – 2024-07-06

### Changed

- On Unix, use `rustix` to provide a simple implementation of `gethostname`
  without `unsafe` (see [GH-10] and [GH-17]).

[GH-10]: https://codeberg.org/swsnr/gethostname.rs/pulls/10
[GH-17]: https://codeberg.org/swsnr/gethostname.rs/pulls/17

## [0.4.3] – 2023-05-13

### Changed

- Removed the `windows` dependency in favor of using embedded bindings, see
  [GH-11].

[GH-11]: https://codeberg.org/swsnr/gethostname.rs/pulls/11

## [0.4.2] – 2023-04-13

### Changed

- Update dependencies.

## [0.4.1] – 2022-12-01

### Changed

- Update repository URL to <https://codeberg.org/swsnr/gethostname.rs>.

## [0.4.0] – 2022-10-28

### Changed

- Replace `winapi` with windows-rs, see [GH-7].
- Bump MSRV to 1.64 as required by windows-rs, see [GH-7].

[GH-7]: https://codeberg.org/swsnr/gethostname.rs/pulls/7

## [0.3.0] – 2022-10-09

### Changed

- Bump MSRV to 1.56.

## [0.2.3] – 2022-03-12

### Changed

- Limit `gethostname()` to `cfg(unix)` and `cfg(windows)` to provide more useful
  build failures on other platforms.

## [0.2.2] – 2022-01-14

## [0.2.1] – 2019-12-18

### Changed

- Consolidate documetation.
- Update crates.io metadata.

## [0.2.0] – 2019-01-22

### Added

- Add Windows implementation (see [GH-1]).

[GH-1]: https://codeberg.org/swsnr/gethostname.rs/pulls/1

### Changed

- Pin supported Rust version to 1.31

## 0.1.0 – 2019-01-20

Initial release.

### Added

- `gethostname()` for non-Windows platforms.

[Unreleased]: https://codeberg.org/swsnr/gethostname.rs/compare/v1.1.0...HEAD
[1.1.0]: https://codeberg.org/swsnr/gethostname.rs/compare/v1.0.2...v1.1.0
[1.0.2]: https://codeberg.org/swsnr/gethostname.rs/compare/v1.0.1...v1.0.2
[1.0.1]: https://codeberg.org/swsnr/gethostname.rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://codeberg.org/swsnr/gethostname.rs/compare/v0.5.0...v1.0.0
[0.5.0]: https://codeberg.org/swsnr/gethostname.rs/compare/v0.4.3...v0.5.0
[0.4.3]: https://codeberg.org/swsnr/gethostname.rs/compare/v0.4.2...v0.4.3
[0.4.2]: https://codeberg.org/swsnr/gethostname.rs/compare/v0.4.1...v0.4.2
[0.4.1]: https://codeberg.org/swsnr/gethostname.rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://codeberg.org/swsnr/gethostname.rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://codeberg.org/swsnr/gethostname.rs/compare/v0.2.3...v0.3.0
[0.2.3]: https://codeberg.org/swsnr/gethostname.rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://codeberg.org/swsnr/gethostname.rs/compare/gethostname-0.2.1...v0.2.2
[0.2.1]: https://codeberg.org/swsnr/gethostname.rs/compare/gethostname-0.2.0...gethostname-0.2.1
[0.2.0]: https://codeberg.org/swsnr/gethostname.rs/compare/gethostname-0.1.0...gethostname-0.2.0
