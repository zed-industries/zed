# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.1](https://github.com/yoshuawuyts/miow/compare/v0.6.0...v0.6.1) - 2025-09-10

### Other

- Upgrade `windows-sys` from 0.48 to 0.60-0.61.
- Upgrade `socket2` from 0.5 to 0.6 (dev dependency only).

## [v0.6.0] - 2023-08-04
### Added
- Add note about this crate being unmaintained.

### Changed
- Breaking: Upgrade `windows-sys` from 0.42 to 0.48.
- Upgrade `socket2` from 0.4 to 0.5 (dev dependency only).

## [v0.5.0] - 2022-11-10
### Changed
- Upgrade `windows-sys` from 0.28 to 0.42.

## [v0.4.0] - 2021-11-29
### Changed
- Replaced `winapi` with `windows-sys`.
- `CompletionStatus` now guarantees `#[repr(transparent)]`.

### Internal
- Added CI integration.

## [v0.3.7] - 2021-03-22
### Changed
- Upgrade `rand` dev-dependency from 0.4 -> 0.8
- Upgrade `socket2` dependency from 0.3 to 0.4 and make it a dev-dependency
