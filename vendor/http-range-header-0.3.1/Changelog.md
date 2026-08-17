<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
### Added
### Changed
### Fixed

## [0.3.1] - 2023-07-21

### Fixed
- Now accepts ranges that are out of bounds, but truncates them down to an in-range 
value, according to the spec, thanks @jfaust!
- Clean up with clippy pedantic, update docs, format, etc. Resulted in a bench improvement of almost
5%.

## [0.3.0] - 2021-11-25

### Changed

- Only expose a single error-type to make usage more ergonomic

## [0.2.1] - 2021-11-25

### Added

- Make some optimizations

## [0.2.0] - 2021-11-25

### Changed

- Rename to http-range-header

## [0.1.0] - 2021-11-25

### Added

- Released first version under parse_range_headers
