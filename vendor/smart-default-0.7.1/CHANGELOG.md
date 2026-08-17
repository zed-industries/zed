# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.7.1 - 2023-04-24
### Fixed
- Fixed bug where the macro fails on valid default expression that is also a
  valid attribute meta because it was expecting the `_code` hack.

## 0.7.0 - 2023-04-23
### Changed
- Update `syn` to version 2

## 0.6.0 - 2019-12-13
### Changed
- Update `syn`, `quote` and `proc-macro2` versions to `1.*.*`.

## 0.5.2 - 2019-04-13
### Fixed
- Omit linting of generated code by adding `#[automatically_derived]` attribute.

## 0.5.1 - 2019-03-01
### Fixed
- Don't use a multi-pattern `if let`, as this unnecessarily requires version
  1.33.0 of Rust.

## 0.5.0 - 2019-03-01
### Changed
- When the default is a string literal, strap an `.into()` after it to
  automatically convert it to `String` when needed.

## 0.4.0 - 2019-02-19
### Added
- `#[default(_code = "...")]` syntax for defaults that cannot be parsed as
  attributes no matter what.

## 0.3.0 - 2018-11-02
### Changed
- Require Rust 1.30+.
- Use direct attribute value instead of having to wrap them in strings.
- Moved the docs from the module level to the custom derive.

### Added
- `#[default(...)]` syntax in addition to `#[default = ...]`. This is required
  to deal with some parsing limitations.

## 0.2.0 - 2017-08-21
### Added
- Support generic types.
- Generate doc for the trait impl that describes the default values.

## 0.1.0 - 2017-08-18
### Added
- Custom derive `SmartDefault` for implementing the `Default` trait.
- `#[default = ...]` attribute on fields for smart-default.
