# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Unreleased

### Added

### Removed

### Changed

### Fixed

# [0.4.0] - 2024-10-26

### Added

- MSRV policy

### Removed

- All `unsafe` code

### Changed

- Updated MSRV to 1.69
- `block_on` now accepts `IntoFuture` instead of `Future` (this is a backward-compatible change)

# [0.3.0] - 2023-02-07

### Added

- `pollster::main` and `pollster::test` procedural macros, akin to `tokio::main` and `tokio::test`

### Changed

- Improved performance by removing unnecessary allocation
