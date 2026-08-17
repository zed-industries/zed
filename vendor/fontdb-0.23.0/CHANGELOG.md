# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.23.0] - 2024-10-09
### Changed
- `ttf-parser` updated.

## [0.22.0] - 2024-09-09
### Changed
- Fallback to known font dirs if none were loaded via fontconfig. Linux-only.
  [@MoSal](https://github.com/@MoSal)

## [0.21.0] - 2024-08-06
### Added
- Symlinked files and directories will now be included when loading system fonts.
  [@jcdickinson](https://github.com/@jcdickinson)

## [0.20.0] - 2024-07-02
### Changed
- `ttf-parser` updated.

## [0.19.0] - 2024-07-02
### Changed
- `ttf-parser` updated.

## [0.18.0] - 2024-06-01
### Changed
- `Database::push_face_info` returns an `ID` now.
  [@laurmaedje](https://github.com/@laurmaedje)

## [0.17.0] - 2024-05-10
### Added
- Up to 10% faster `Database::load_system_fonts`.
  [@qarmin](https://github.com/@qarmin) and [@y5](https://github.com/@y5)

### Changed
- Latest `ttf-parser`.

## [0.16.2] - 2024-02-19
### Fixed
- System fonts loading on Windows when the system drive is not `C:\\`.
  [@tronical](https://github.com/@tronical)

## [0.16.1] - 2024-02-09
### Fixed
- Treat fonts with non-zero italic angle as italic.

## [0.16.0] - 2023-10-31
### Changed
- `ttf-parser` and `memmap2` dependencies update.

## [0.15.0] - 2023-10-01
### Changed
- Enable the `fontconfig` feature by default. Linux-only.
- MSRV bumped to 1.60 due to `log`.

### Fixed
- Fix fontconfig alias matching order.
  [@declantsien](https://github.com/@declantsien)

## [0.14.1] - 2023-05-10
### Fixed
- Return valid IDs from `Database::load_font_source()`.
  [@notgull](https://github.com/notgull)

## [0.14.0] - 2023-05-09
### Changed
- `Database::load_font_source()` returns a list of loaded face IDs now.
  [@notgull](https://github.com/notgull)
- `ttf-parser` and `memmap2` dependencies update.

## [0.13.1] - 2023-04-23
### Added
- Load system fonts on RedoxOS. [@FloVanGH](https://github.com/FloVanGH)

### Fixed
- Improve missing `XDG_CONFIG_HOME` environment variable handling. Linux only.
  [@7sDream](https://github.com/7sDream)
- Improve downloadable fonts detection on macOS. [@messense](https://github.com/messense)

## [0.13.0] - 2023-02-21
### Added
- `Database::default()`. [@7sDream](https://github.com/7sDream)

### Changed
- Database uses `slotmap::SlotMap` instead of `Vec` as an internal storage now.
  This allows us to have O(1) indexing by `ID` by sacrificing faces iteration speed a bit.
  [@7sDream](https://github.com/7sDream)
- `Database::remove_face` no longer returns `bool`.
- `Database::faces` returns an Iterator and not a slice now.
- MSRV bumped to 1.49

## [0.12.0] - 2023-02-05
### Fixed
- Face weight matching.

## [0.11.2] - 2023-01-10
### Added
- Implement `Display` trait for `ID`. [@7sDream](https://github.com/7sDream)

## [0.11.1] - 2022-12-26
### Fixed
- Always prefer _Typographic Family_ to _Family Name_ when available.
  [@CryZe](https://github.com/CryZe)
- Prevent duplicated family names.

## [0.11.0] - 2022-12-25
### Added
- Support localized family names.
- Improve fontconfig support. [@declantsien](https://github.com/declantsien)

### Changed
- `FaceInfo::family` was replaced with `FaceInfo::families` and contains a list of family
  names now.

### Fixed
- Improve family name detection in variable fonts.

## [0.10.0] - 2022-11-08
### Added
- `no_std` support. [@jackpot51](https://github.com/jackpot51)

## [0.9.3] - 2022-10-26
### Added
- `Database::family_name` is public now.

## [0.9.2] - 2022-10-22
### Added
- `Database::push_face_info`
- `ID::dummy`

### Fixed
- Expand home path `~` prefix during fontconfig paths resolving.
  [@snoyer](https://github.com/snoyer)

## [0.9.1] - 2022-02-21
### Changed
- Reduce binary size by 10% using less generic code.
- Simplify Database::query implementation.

## [0.9.0] - 2022-02-20
### Added
- Way faster fonts scanning by using a more low-level `ttf-parser` API
  which allows us to parse only required TrueType tables.
  On my hardware, `load_system_fonts()` loaded 898 fonts in 9ms instead of 11ms in the release mode
  and in 35ms instead of 52ms in debug.
  Currently, we're parsing only `name`, `OS/2` and `post` tables.

## [0.8.0] - 2022-02-12
### Added
- Load user fonts on Windows.
- `fontconfig` feature to allow retrieving font dirs from the fontconfig config file
  instead of using hardcoded paths. Linux-only. [@Riey](https://github.com/Riey)

## [0.7.0] - 2021-10-04
### Changed
- The `Source` enum has a new variant `SharedFile`, used for unsafe persistent
  memory mappings.
- `FaceInfo` stores `Source` directly now, not anymore in an `Arc`. Instead `Source::Binary`
  now stores an `Arc` of the data.

## [0.6.2] - 2021-09-04
### Fixed
- Fix compilation without the `fs` feature.

## [0.6.1] - 2021-09-04
### Changed
- Split the `fs` build feature into `fs` and `memmap`. [@neinseg](https://github.com/neinseg)

## [0.6.0] - 2021-08-21
### Added
- Search in `$HOME/.fonts` on Linux. [@Linus789](https://github.com/Linus789)

### Changed
- Generic font families are preset by default instead of being set to `None`.

## [0.5.4] - 2021-05-25
### Added
- Implement `Eq`, `Hash` for `Query`, `Family`, `Weight` and `Style`.
  [@dhardy](https://github.com/dhardy)

### Changed
- Update `ttf-parser`

## [0.5.3] - 2021-05-19
### Changed
- Update `ttf-parser`

## [0.5.2] - 2021-05-19
### Changed
- Update `memmap2`
- Add additional search dir for macOS.

## [0.5.1] - 2020-12-20
### Fixed
- Compilation on Windows.

## [0.5.0] - 2020-12-20
### Added
- `FaceInfo::post_script_name`
- `FaceInfo::monospaced`
- `Database::load_system_fonts`

## [0.4.0] - 2020-12-06
### Changed
- Use a simple `u32` for ID instead of UUID.

## [0.3.0] - 2020-12-05
### Changed
- `ttf-parser` updated.

## [0.2.0] - 2020-07-21
### Changed
- `ttf-parser` updated.

### Fixed
- Stretch processing. `ttf-parser` was incorrectly parsing this property.

[Unreleased]: https://github.com/RazrFalcon/fontdb/compare/v0.23.0...HEAD
[0.23.0]: https://github.com/RazrFalcon/fontdb/compare/v0.22.0...v0.23.0
[0.22.0]: https://github.com/RazrFalcon/fontdb/compare/v0.21.0...v0.22.0
[0.21.0]: https://github.com/RazrFalcon/fontdb/compare/v0.20.0...v0.21.0
[0.20.0]: https://github.com/RazrFalcon/fontdb/compare/v0.19.0...v0.20.0
[0.19.0]: https://github.com/RazrFalcon/fontdb/compare/v0.18.0...v0.19.0
[0.18.0]: https://github.com/RazrFalcon/fontdb/compare/v0.17.0...v0.18.0
[0.17.0]: https://github.com/RazrFalcon/fontdb/compare/v0.16.2...v0.17.0
[0.16.2]: https://github.com/RazrFalcon/fontdb/compare/v0.16.1...v0.16.2
[0.16.1]: https://github.com/RazrFalcon/fontdb/compare/v0.16.0...v0.16.1
[0.16.0]: https://github.com/RazrFalcon/fontdb/compare/v0.15.0...v0.16.0
[0.15.0]: https://github.com/RazrFalcon/fontdb/compare/v0.14.1...v0.15.0
[0.14.1]: https://github.com/RazrFalcon/fontdb/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/RazrFalcon/fontdb/compare/v0.13.1...v0.14.0
[0.13.1]: https://github.com/RazrFalcon/fontdb/compare/v0.13.0...v0.13.1
[0.13.0]: https://github.com/RazrFalcon/fontdb/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/RazrFalcon/fontdb/compare/v0.11.2...v0.12.0
[0.11.2]: https://github.com/RazrFalcon/fontdb/compare/v0.11.1...v0.11.2
[0.11.1]: https://github.com/RazrFalcon/fontdb/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/RazrFalcon/fontdb/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/RazrFalcon/fontdb/compare/v0.9.3...v0.10.0
[0.9.3]: https://github.com/RazrFalcon/fontdb/compare/v0.9.2...v0.9.3
[0.9.2]: https://github.com/RazrFalcon/fontdb/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/RazrFalcon/fontdb/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/RazrFalcon/fontdb/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/RazrFalcon/fontdb/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/RazrFalcon/fontdb/compare/v0.6.2...v0.7.0
[0.6.2]: https://github.com/RazrFalcon/fontdb/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/RazrFalcon/fontdb/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/RazrFalcon/fontdb/compare/v0.5.4...v0.6.0
[0.5.4]: https://github.com/RazrFalcon/fontdb/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/RazrFalcon/fontdb/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/RazrFalcon/fontdb/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/RazrFalcon/fontdb/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/RazrFalcon/fontdb/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/RazrFalcon/fontdb/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/RazrFalcon/fontdb/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/RazrFalcon/fontdb/compare/v0.1.0...v0.2.0
