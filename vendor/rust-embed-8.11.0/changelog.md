# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

Thanks to [Mark Drobnak](https://github.com/AzureMarker) for the changelog.

## [8.11.0] - 2026-01-14

 - Fix RustEmbed::iter() signature as it was breaking the static lifetime.

## [8.10.0] - 2026-01-13

 - RustEmbed::iter(): replace concrete return type with generic iterator, to make it easier to implement an empty fake embed. Thanks to Johannes Altmanninger <aclopte@gmail.com>.

## [8.9.0] - 2025-10-31

 - Ignore paths that couldn't be canonicalized. Thanks to zvolin <mac.zwolinski@gmail.com>.

## [8.9.0] - 2025-10-18

 - fix export of RustEmbed macro. Thanks to LorrensP-2158466 <lorrens.pantelis@student.uhasselt.be>.

## [8.7.2] - 2025-05-14

 - Update repo links to sub-packages

## [8.7.1] - 2025-05-05

 - Update documentation and repo links

## [8.7.0] - 2025-04-10

 - add deterministic timestamps flag for deterministic builds [#259](https://github.com/pyrossh/rust-embed/pull/259). Thanks to [daywalker90](https://github.com/daywalker90)


## [8.6.0] - 2025-02-25

- Update include-flate to 0.3 [#246](https://github.com/pyrossh/rust-embed/pull/246). Thanks to [krant](https://github.com/krant)
- refactor: remove redundant reference and closure [#250](https://github.com/pyrossh/rust-embed/pull/250). Thanks to [hamirmahal](https://github.com/hamirmahal)
- refactor: replace map().unwrap_or_else(). [#250](https://github.com/pyrossh/rust-embed/pull/255). Thanks to [hamirmahal](https://github.com/hamirmahal)
- Compatible with Axum 0.7.9 [#253](https://github.com/pyrossh/rust-embed/pull/253). Thanks to [wkmyws](https://github.com/wkmyws)
- Add allow_missing option to derive macro [#256](https://github.com/pyrossh/rust-embed/pull/256). Thanks to [lirannl](https://github.com/lirannl)


## [8.5.0] - 2024-07-09

- Allow users to specify a custom path to the rust_embed crate in generated code[#232](https://github.com/pyrossh/rust-embed/pull/232). Thanks to [Wulf](https://github.com/Wulf)
- Increase minimum rust-version to v1.7.0.0

## [8.4.0] - 2024-05-11

- Re-export RustEmbed as Embed [#245](https://github.com/pyrossh/rust-embed/pull/245/files). Thanks to [pyrossh](https://github.com/pyrossh)
- Do not build glob matchers repeatedly when include-exclude feature is enabled [#244](https://github.com/pyrossh/rust-embed/pull/244/files). Thanks to [osiewicz](https://github.com/osiewicz)
- Add `metadata_only` attribute [#241](https://github.com/pyrossh/rust-embed/pull/241/files). Thanks to [ddfisher](https://github.com/ddfisher)
- Replace `expect` with a safer alternative that returns `None` instead [#240](https://github.com/pyrossh/rust-embed/pull/240/files). Thanks to [costinsin](https://github.com/costinsin)
- Eliminate unnecessary `to_path` call [#239](https://github.com/pyrossh/rust-embed/pull/239/files). Thanks to [smoelius](https://github.com/smoelius)

## [8.3.0] - 2024-02-26

- Fix symbolic links in debug builds [#235](https://github.com/pyrossh/rust-embed/pull/235/files). Thanks to [Buckram123](https://github.com/Buckram123)

## [8.2.0] - 2023-12-29

- Fix naming collisions in macros [#230](https://github.com/pyrossh/rust-embed/pull/230/files). Thanks to [hwittenborn](https://github.com/hwittenborn)

## [8.1.0] - 2023-12-08

- Add created to file metadata. [#225](https://github.com/pyrossh/rust-embed/pull/225/files). Thanks to [ngalaiko](https://github.com/ngalaiko)

## [8.0.0] - 2023-08-23

- Store file contents statically and use binary search for lookup. [#217](https://github.com/pyrossh/rust-embed/pull/217/files). Thanks to [osiewicz](https://github.com/osiewicz)

## [6.8.1] - 2023-06-30

- Fix failing compilation under compression feature [#182](https://github.com/pyrossh/rust-embed/issues/182). Thanks to [osiewicz](https://github.com/osiewicz)

## [6.8.0] - 2023-06-30

- Update `include-flate` to v0.2 [#182](https://github.com/pyrossh/rust-embed/issues/182)

## [6.7.0] - 2023-06-09

- Update `syn` to v2.0 [#211](https://github.com/pyrossh/rust-embed/issues/211)

## [6.6.1] - 2023-03-25

- Fix mime-guess feature not working properly [#209](https://github.com/pyrossh/rust-embed/issues/209)

## [6.6.0] - 2023-03-05

- sort_by_file_name() requires walkdir v2.3.2 [#206](https://github.com/pyrossh/rust-embed/issues/206)
- Add `mime-guess` feature to statically store mimetype [#192](https://github.com/pyrossh/rust-embed/issues/192)

## [6.4.2] - 2022-10-20

- Fail the proc macro if include/exclude are used without the feature [#187](https://github.com/pyrossh/rust-embed/issues/187)

## [6.4.1] - 2022-09-13

- Update sha2 dependency version in utils crate [#186](https://github.com/pyrossh/rust-embed/issues/186)

## [6.4.0] - 2022-04-15

- Order files by filename [#171](https://github.com/pyros2097/rust-embed/issues/171). Thanks to [apognu](https://github.com/apognu)

## [6.3.0] - 2021-11-28

- Fixed a security issue in debug mode [#159](https://github.com/pyros2097/rust-embed/issues/159). Thanks to [5225225](https://github.com/5225225)

## [6.2.0] - 2021-09-01

- Fixed `include-exclude` feature when using cargo v2 resolver

## [6.1.0] - 2021-08-31

- Added `include-exclude` feature by [mbme](https://github.com/mbme)

## [6.0.1] - 2021-08-21

- Added doc comments to macro generated functions

## [6.0.0] - 2021-08-01

Idea came about from [Cody Casterline](https://github.com/NfNitLoop)

- Breaking change the `Asset::get()` api has changed and now returns an `EmbeddedFile` which contains a `data` field which is the bytes of the file and
  a `metadata` field which has theses 2 properties associated to the file `hash` and `last_modified`;

## [5.9.0] - 2021-01-18

### Added

- Added path prefix attribute

## [5.8.0] - 2021-01-06

### Fixed

- Fixed compiling with latest version of syn

## [5.7.0] - 2020-12-08

### Fixed

- Fix feature flag typo

## [5.6.0] - 2020-07-19

### Fixed

- Fixed windows path error in release mode

### Changed

- Using github actions for CI now

## [5.5.1] - 2020-03-19

### Fixed

- Fixed warnings in latest nightly

## [5.5.0] - 2020-02-26

### Fixed

- Fixed the `folder` directory being relative to the current directory.
  It is now relative to `Cargo.toml`.

## [5.4.0] - 2020-02-24

### Changed

- using rust-2018 edition now
- code cleanup
- updated examples and crates

## [5.3.0] - 2020-02-15

### Added

- `compression` feature for compressing embedded files

## [5.2.0] - 2019-12-05

## Changed

- updated syn and quote crate to 1.x

## [5.1.0] - 2019-07-09

## Fixed

- error when debug code tries to import the utils crate

## [5.0.1] - 2019-07-07

## Changed

- derive is allowed only on unit structs now

## [5.0.0] - 2019-07-05

## Added

- proper error message stating only unit structs are supported

## Fixed

- windows latest build

## [4.5.0] - 2019-06-29

## Added

- allow rust embed derive to take env variables in the folder path

## [4.4.0] - 2019-05-11

### Fixed

- a panic when struct has doc comments

### Added

- a warp example

## [4.3.0] - 2019-01-10

### Fixed

- debug_embed feature was not working at all

### Added

- a test run for debug_embed feature

## [4.2.0] - 2018-12-02

### Changed

- return `Cow<'static, [u8]>` to preserve static lifetime

## [4.1.0] - 2018-10-24

### Added

- `iter()` method to list files

## [4.0.0] - 2018-10-11

### Changed

- avoid vector allocation by returning `impl AsRef<[u8]>`

## [3.0.2] - 2018-09-05

### Added

- appveyor for testing on windows

### Fixed

- handle paths in windows correctly

## [3.0.1] - 2018-07-24

### Added

- panic if the folder cannot be found

## [3.0.0] - 2018-06-01

### Changed

- The derive attribute style so we don't need `attr_literals` and it can be used in stable rust now. Thanks to [Mcat12](https://github.com/Mcat12).

```rust
#[folder("assets/")]
```

to

```rust
#[folder = "assets/"]
```

### Removed

- log dependecy as we are not using it anymore

## [2.0.0] - 2018-05-26

### Changed

- Reimplemented the macro for release to use include_bytes for perf sake. Thanks to [lukad](https://github.com/lukad).

## [1.1.1] - 2018-03-19

### Changed

- Fixed usage error message

## [1.1.0] - 2018-03-19

### Added

- Release mode for custom derive

### Changed

- Fixed tests in travis

## [1.0.0] - 2018-03-18

### Changed

- Converted the rust-embed macro `embed!` into a Rust Custom Derive Macro `#[derive(RustEmbed)]` which implements get on the struct

```rust
let asset = embed!("examples/public/")
```

to

```rust
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;
```

## [0.5.2] - 2018-03-16

### Added

- rouille example

## [0.5.1] - 2018-03-16

### Removed

- the plugin attribute from crate

## [0.5.0] - 2018-03-16

### Added

- rocket example

### Changed

- Converted the rust-embed executable into a macro `embed!` which now loads files at compile time during release and from the fs during dev.

## [0.4.0] - 2017-03-2

### Changed

- `generate_assets` to public again

## [0.3.5] - 2017-03-2

### Added

- rust-embed prefix to all logs

## [0.3.4] - 2017-03-2

### Changed

- the lib to be plugin again

## [0.3.3] - 2017-03-2

### Changed

- the lib to be proc-macro from plugin

## [0.3.2] - 2017-03-2

### Changed

- lib name from `rust-embed` to `rust_embed`

## [0.3.1] - 2017-03-2

### Removed

- hyper example

## [0.3.0] - 2017-02-26

### Added

- rust-embed executable which generates rust code to embed resource files into your rust executable
  it creates a file like assets.rs that contains the code for your assets.

## [0.2.0] - 2017-03-16

### Added

- rust-embed executable which generates rust code to embed resource files into your rust executable
  it creates a file like assets.rs that contains the code for your assets.
