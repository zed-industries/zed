# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.5.0] - 2026-01-07

This release matches HarfBuzz [v12.3.0][harfbuzz-12.3.0], and has an MSRV (minimum supported Rust version) of 1.85.

- Update to read-fonts 0.37.0 (and bump MSRV to 1.85).
- Various performance improvements.

## [0.4.1] - 2025-12-08

This release matches HarfBuzz [v12.2.0][harfbuzz-12.2.0], and has an MSRV (minimum supported Rust version) of 1.82.

- Make Script::from_iso15924_tag const.
- Avoid panic when saving syllable indices.

## [0.4.0] - 2025-11-10

This release matches HarfBuzz [v12.2.0][harfbuzz-12.2.0], and has an MSRV (minimum supported Rust version) of 1.82.

- Enable more HarfBuzz tests.
- Fix bug from [HarfBust puzzle](https://github.com/harfbuzz/harfbuzz/issues/5535).
- Update to read-fonts 0.36.0.

## [0.3.2] - 2025-10-15

This release matches HarfBuzz [v12.1.0][harfbuzz-12.1.0], and has an MSRV (minimum supported Rust version) of 1.82.

- Fix "would apply" logic for chained sequence context format 3. This bug was preventing accurate classification of
  characters in Indic syllables for some fonts.
- Various optimizations.

## [0.3.1] - 2025-09-12

This release matches HarfBuzz [v11.5.0][harfbuzz-11.5.0], and has an MSRV (minimum supported Rust version) of 1.82.

- Actually bump MSRV from 1.80 to 1.82.

## [0.3.0] - 2025-09-12

This release matches HarfBuzz [v11.5.0][harfbuzz-11.5.0], and has an MSRV (minimum supported Rust version) of 1.82.

- Update to read-fonts 0.35.0.
- Bump MSRV from 1.80 to 1.82.

## [0.2.1] - 2025-09-12

This release matches HarfBuzz [v11.5.0][harfbuzz-11.5.0], and has an MSRV (minimum supported Rust version) of 1.80.

- Update to Unicode 17.0.
- Fix panic when processing chained sequence context format 3.
- Add accessors for script, language and direction to `ShapePlan`.
- Various optimizations.

## [0.2.0] - 2025-08-29

This release matches HarfBuzz [v11.4.4][harfbuzz-11.4.4], and has an MSRV (minimum supported Rust version) of 1.80.

- Major optimizations to speed up AAT shaping.

## [0.1.2] - 2025-08-20

This release matches HarfBuzz [v11.3.3][harfbuzz-11.3.3], and has an MSRV (minimum supported Rust version) of 1.80.

- Major optimizations to speed up shaping.
- Initial support for shape plan caching in the form of `ShapePlanKey`.

## [0.1.1] - 2025-08-11

This release matches HarfBuzz [v11.3.3][harfbuzz-11.3.3], and has an MSRV (minimum supported Rust version) of 1.75.

- Major optimizations to speed up shaping.

## [0.1.0] - 2025-06-10

This release matches HarfBuzz [v11.2.1][harfbuzz-11.2.1], and has an MSRV (minimum supported Rust version) of 1.75.

- Initial Release of HarfRuzz.

HarfRust is a fork of RustyBuzz.
See [their changelog](https://github.com/harfbuzz/rustybuzz/blob/main/CHANGELOG.md) for details of prior releases.

[Unreleased]: https://github.com/harfbuzz/harfrust/compare/0.5.0...HEAD
[0.5.0]: https://github.com/harfbuzz/harfrust/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/harfbuzz/harfrust/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/harfbuzz/harfrust/compare/0.3.2...0.4.0
[0.3.2]: https://github.com/harfbuzz/harfrust/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/harfbuzz/harfrust/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/harfbuzz/harfrust/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/harfbuzz/harfrust/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/harfbuzz/harfrust/compare/0.1.2...0.2.0
[0.1.2]: https://github.com/harfbuzz/harfrust/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/harfbuzz/harfrust/compare/0.1.0...0.1.1
<!-- The last release of RustyBuzz before 0.1.0. -->
[0.1.0]: https://github.com/harfbuzz/harfrust/compare/8c52723ff75e91a33ae36e527baed871097e64bf...0.1.0

[harfbuzz-11.2.1]: https://github.com/harfbuzz/harfbuzz/releases/tag/11.2.1
[harfbuzz-11.3.3]: https://github.com/harfbuzz/harfbuzz/releases/tag/11.3.3
[harfbuzz-11.4.4]: https://github.com/harfbuzz/harfbuzz/releases/tag/11.4.4
[harfbuzz-11.5.0]: https://github.com/harfbuzz/harfbuzz/releases/tag/11.5.0
[harfbuzz-12.1.0]: https://github.com/harfbuzz/harfbuzz/releases/tag/12.1.0
[harfbuzz-12.2.0]: https://github.com/harfbuzz/harfbuzz/releases/tag/12.2.0
[harfbuzz-12.3.0]: https://github.com/harfbuzz/harfbuzz/releases/tag/12.3.0

[@khaledhosny]: https://github.com/khaledhosny

[#65]: https://github.com/harfbuzz/harfrust/pull/65
