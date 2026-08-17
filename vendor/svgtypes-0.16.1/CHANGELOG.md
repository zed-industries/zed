# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

The latest published SVG Types release is [0.16.1](#0160-2026-01-09) which was released on 2026-01-09.
You can find its changes [documented below](#0161-2026-01-09).

## [Unreleased]

This release has an [MSRV][] of 1.85.

## [0.16.1][] (2026-01-09)

This release has an [MSRV][] of 1.85.

### Changed

- Update Kurbo to v0.13. ([#60][] by [@nicoburns][])

## [0.16.0][] (2025-10-26)

This release has an [MSRV][] of 1.82.

### Added

- Add helper functions to PathSegment ([#37][] by [@LaurenzV][])

### Changed

- Update Kurbo to v0.12. ([#56][] by [@HaHa421][])

## [0.15.3][] (2025-01-20)

This release has an [MSRV][] of 1.65.

This is the first release under the stewardship of [Linebender][], who is now responsible for maintenance of this crate.
Many thanks to Yevhenii Reizner for the years of hard work that he has poured into this and other crates.

### Added

- Support floating point hue in `hsl()` and `hsla()`. ([#31][] by [@erxclau][])

### Fixed

- Round hues in HSL to RGB conversion. ([#34][] by [@erxclau][])
- Correct rounding of alpha. ([#35][] by [@waywardmonkeys][])

### Changed

- Set the MSRV in `Cargo.toml` and updated the edition to 2021. ([#40][] by [@tomcur][])
- Simplify color component rounding and bounds checking. ([#43][] , [#44][] by [@tomcur][])

## [0.15.2][] (2024-08-20)

### Fixed

- Path parsing with `S` or `T` segments after `A`. Was broken since v0.12

## [0.15.1][] (2024-05-07)

### Fixed

- Allow double quotes in FuncIRI.

## [0.15.0][]  (2024-04-03)

### Changed

- Bump `kurbo` and `siphasher`

## [0.14.0][] (2024-02-05)

### Added

- `font-family` parsing. ([#20][] by [@LaurenzV][])
- `font` shorthand parsing. ([#21][] by [@LaurenzV][])

## [0.13.0][] (2023-12-03)

### Added

- `Paint::ContextFill` and `Paint::ContextStroke`. ([#17][] by [@LaurenzV][])
- `transform-origin` parsing. ([#18][] by [@LaurenzV][])

## [0.12.0][] (2023-10-01)

### Added

- Allow parsing of float RGB values from CSS Color 4 draft like `rgb(3.14, 110, 201)`.
  The values itself would still be stored as `u8`. ([#14][] by [@yisibl][])
- Allow quotes in FuncIRI. ([#15][] by [@romanzes][])

## [0.11.0][] (2023-03-25)

### Added

- `SimplifyingPathParser` handles implicit MoveTo commands after ClosePath now.
  Previously, `M 10 20 L 30 40 Z L 50 60` would have been parsed as is,
  but now it will be parsed as `M 10 20 L 30 40 Z M 10 20 L 50 60`.

## [0.10.0][] (2023-02-04)

### Changed

- Bump `kurbo`
- Bump MSRV to 1.65 (because of `kurbo`)

## [0.9.0][] (2022-12-25)

### Added

- `SimplifyingPathParser` that allows parsing an already simplified Path Data.

## [0.8.2][] (2022-10-23)

### Added

- `paint-order`

## [0.8.1][] (2022-06-11)

### Added

- Support #RRGGBBAA and #RGBA color notation as per CSS Color 4. ([#13][] by [@demurgos]]

## [0.8.0][] (2021-09-12)

### Added

- `EnableBackground`
- `Number`. Previously accessible via `Steam::parse_number`.
- `IRI`. Previously accessible via `Steam::parse_iri`.
- `FuncIRI`. Previously accessible via `Steam::parse_func_iri`.

### Changed

- `Stream` is now private.

## [0.7.0][] (2021-09-04)

**Breaking**: Almost a complete rewrite. This crate is strictly a parser from now.

### Added

- [`<filter-value-list>`](https://www.w3.org/TR/filter-effects-1/#typedef-filter-value-list)
  parsing using `FilterValueListParser`.
- `ViewBoxError`

### Removed

- Writing support.
- Container types. Only stack allocated types and pull-based parsers are available.
- `FuzzyEq` and `FuzzyZero`.

## [0.6.0][] (2021-08-22)

### Added

- CSS3 colors support (`rgba`, `hsl`, `hsla`, `transparent`).
- `turn` angle unit.
- `Angle::to_degrees`.

### Changed

- Move to Rust 2018 edition.
- Rename `Stream::skip_string` into `Stream::consume_string`.
- Rename `Color::new` into `Color::new_rgb`.
- `Color` struct gained an `alpha` field.
- Rename `Angle::num` into `Angle::number`.
- Rename `Length::num` into `Length::number`.

## [0.5.0][] (2019-08-12)

### Added

- Implement `Default` for `Length`, `LengthList`, `NumberList`, `Points` and `Path`.

### Changed

- The minimum Rust version is 1.31

### Removed

- `PathBuilder`. Use `Path::push_*` instead.
- `Style` parser. Use an external CSS parser instead, like `simplecss`.
- `ElementId` and `AttributeId`.
- `phf` dependency. Only `siphasher` is used now.

## [0.4.4][] (2019-06-11)

- Update `float-cmp`.

## [0.4.3][] (2019-06-10)

### Added

- `Transform::prepend`.
- Implement `FuzzyEq` and `FuzzyZero` for `f32`.
- Parsing of `Color`, `Paint`, `ElementId` and `AttributeId` can be disabled now.

## [0.4.2][] (2019-03-15)

### Changed

- The `XmlByteExt` trait is private now.

## [0.4.1][] (2019-01-06)

### Fixed

- Style with comments parsing.

## [0.4.0][] (2019-01-02)

### Added

- An [`angle`](https://www.w3.org/TR/SVG11/types.html#DataTypeAngle) value type.

### Changed

- `Length::from_str` will return an error if an input string has trailing data.
  So length like `1mmx` was previously parsed without errors.

## [0.3.0][] (2018-12-13)

### Changed

- `PathParser` will return `Result<PathSegment>` instead of `PathSegment` from now.
- `Error` was rewritten.

### Removed

- `FromSpan` trait. Use `FromStr`.
- `StrSpan`. All strings are `&str` now.
- `TextPos`. All errors have position in characters now.
- `xmlparser` dependency.
- `log` dependency.

## [0.2.0][] (2018-09-12)

### Added

- `black`, `white`, `gray`, `red`, `green` and `blue` constructors to the `Color` struct.

### Changed

- `StyleParser` will return `(StrSpan, StrSpan)` and not `StyleToken` from now.
- `StyleParser` requires entity references to be resolved before parsing from now.

### Removed

- `failure` dependency.
- `StyleToken`.
- `Error::InvalidEntityRef`.

## [0.1.1][](2018-05-23)

### Added

- `encoding` and `standalone` to AttributeId.
- `new_translate`, `new_scale`, `new_rotate`, `new_rotate_at`, `new_skew_x`, `new_skew_y`
  and `rotate_at` methods to the `Transform`.

### Changed

- `StreamExt::parse_iri` and `StreamExt::parse_func_iri` will parse
  not only well-formed data now.

### Fixed

- `Paint::from_span` poor performance.

[MSRV]: README.md#minimum-supported-rust-version-msrv
[Linebender]: https://github.com/linebender

[#13]: https://github.com/linebender/svgtypes/pull/13
[#14]: https://github.com/linebender/svgtypes/pull/14
[#15]: https://github.com/linebender/svgtypes/pull/15
[#17]: https://github.com/linebender/svgtypes/pull/17
[#18]: https://github.com/linebender/svgtypes/pull/18
[#20]: https://github.com/linebender/svgtypes/pull/20
[#21]: https://github.com/linebender/svgtypes/pull/21
[#31]: https://github.com/linebender/svgtypes/pull/31
[#34]: https://github.com/linebender/svgtypes/pull/34
[#35]: https://github.com/linebender/svgtypes/pull/35
[#40]: https://github.com/linebender/svgtypes/pull/40
[#43]: https://github.com/linebender/svgtypes/pull/43
[#44]: https://github.com/linebender/svgtypes/pull/44
[#56]: https://github.com/linebender/svgtypes/pull/56

[@demurgos]: https://github.com/demurgos
[@erxclau]: https://github.com/erxclau
[@HaHa421]: https://github.com/HaHa421
[@Laurenzv]: https://github.com/LaurenzV
[@romanzes]: https://github.com/romanzes
[@tomcur]: https://github.com/tomcur
[@waywardmonkeys]: https://github.com/waywardmonkeys
[@yisibl]: https://github.com/yisibl

[Unreleased]: https://github.com/RazrFalcon/svgtypes/compare/v0.16.1...HEAD
[0.16.1]: https://github.com/RazrFalcon/svgtypes/compare/v0.16.0...v0.16.1
[0.16.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.15.3...v0.16.0
[0.15.3]: https://github.com/RazrFalcon/svgtypes/compare/v0.15.2...v0.15.3
[0.15.2]: https://github.com/RazrFalcon/svgtypes/compare/v0.15.1...v0.15.2
[0.15.1]: https://github.com/RazrFalcon/svgtypes/compare/v0.15.0...v0.15.1
[0.15.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.13.0...v0.14.0
[0.13.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.8.2...v0.9.0
[0.8.2]: https://github.com/RazrFalcon/svgtypes/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/RazrFalcon/svgtypes/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.4...v0.5.0
[0.4.4]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/RazrFalcon/svgtypes/compare/v0.1.0...v0.1.1
