# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.1] - 2025-09-29

### Fixed

Replaced obsolete `doc_auto_cfg` with `doc_cfg`, to fix Rust nightly builds with the `doc_cfg` flag enabled.

## [1.2.0] - 2025-09-14

### Changed

- MSRV updated to Rust 1.61 to support the switch to `serde_core`.
- camino now depends on `serde_core` rather than `serde`. This allows camino's compilation to be parallelized with `serde_derive`.
- `serde` and `proptest` are no longer available as features. This is technically a breaking change, but these features were already no-ops. Instead, use `serde1` and `proptest1` respectively.

## [1.1.12] - 2025-08-26

### Added

- `Utf8PathBuf::from_os_string` and `Utf8Path::from_os_str` conversions.
- `TryFrom<OsString> for Utf8PathBuf` and `TryFrom<&OsStr> for &Utf8Path` conversions.

Thanks to [BenjaminBrienen](https://github.com/BenjaminBrienen) for your first contribution!

## [1.1.11] - 2025-08-17

### Added

- `Utf8PathBuf::leak` on Rust 1.89 and above.

## [1.1.10] - 2025-06-02

### Changed

- Hand-write serde implementations, dropping the dependency on `serde_derive`. Thanks to [Enselic](https://github.com/Enselic) for initiating the discussion and for your first contribution!

## [1.1.9] - 2024-08-17

### Added

- Top-level function `absolute_utf8` wraps `std::path::absolute`, converting paths to UTF-8.
  Requires Rust 1.79 and above.

## [1.1.8] - 2024-08-15

### Changed

- Use `OsStr::as_encoded_bytes` on Rust 1.74 and above, making conversions from `OsStr` to `str` virtually free ([#93](https://github.com/camino-rs/camino/pull/93)). Thanks [@h-a-n-a](https://github.com/h-a-n-a) for your first contribution!

## [1.1.7] - 2024-05-14

### Fixed

- Resolve `unexpected_cfg` warnings.

## [1.1.6] - 2023-07-11

### Added

- Implement `Deserialize` for `Box<Utf8Path>`.

## [1.1.5] - 2023-07-11

(This release was not published due to an internal issue.)

## [1.1.4] - 2023-03-09

### Added

- Implement `DerefMut` for `Utf8PathBuf` on Rust 1.68 and above.

## [1.1.3] - 2023-02-21

### Added

- New method `Utf8DirEntry::into_path` to return an owned `Utf8PathBuf`.

## [1.1.2] - 2022-08-12

### Added

- New convenience methods [`FromPathBufError::into_io_error`] and
  [`FromPathError::into_io_error`].

## [1.1.1] - 2022-08-12

### Fixed

- Fixed a build regression on older nightlies in the 1.63 series
  ([#22](https://github.com/camino-rs/camino/issues/22)).
- Documentation fixes.

## [1.1.0] - 2022-08-11

### Added

- New methods, mirroring those in recent versions of Rust:
  - `Utf8Path::try_exists` checks whether a path exists. Note that while `std::path::Path` only provides this method for Rust 1.58 and above, `camino` backfills the method for all Rust versions it supports.
  - `Utf8PathBuf::shrink_to` shrinks a `Utf8PathBuf` to a given size. This was added in, and is gated on, Rust 1.56+.
  - `Utf8PathBuf::try_reserve` and `Utf8PathBuf::try_reserve_exact` implement fallible allocations. These were added in, and are gated on, Rust 1.63+.
- A number of `#[must_use]` annotations to APIs, mirroring those added to `Path` and `PathBuf` in recent versions of Rust. The minor version bump is due to this change.

## [1.0.9] - 2022-05-19

### Fixed

- Documentation fixes.

## [1.0.8] - 2022-05-09

### Added

- New methods `canonicalize_utf8`, `read_link_utf8` and `read_dir_utf8` return `Utf8PathBuf`s, erroring out if a resulting path is not valid UTF-8.
- New feature `proptest1` introduces proptest `Arbitrary` impls for `Utf8PathBuf` and
  `Box<Utf8Path>` ([#18], thanks [mcronce](https://github.com/mcronce) for your first contribution!)

[#18]: https://github.com/camino-rs/camino/pull/18

## [1.0.7] - 2022-01-16

### Added

- `Utf8Path::is_symlink` checks whether a path is a symlink. Note that while `std::path::Path` only
  provides this method for version 1.58 and above, `camino` backfills the method for all Rust versions
  it supports.

### Changed

- Update repository links to new location [camino-rs/camino](https://github.com/camino-rs/camino).
- Update `structopt` example to clap 3's builtin derive feature.
  (camino continues to work with structopt as before.)

## [1.0.6] - 2022-01-16

(This release was yanked due to a publishing issue.)

## [1.0.5] - 2021-07-27

### Added

- `Utf8PathBuf::into_std_path_buf` converts a `Utf8PathBuf` to a `PathBuf`; equivalent to the
  `From<Utf8PathBuf> for PathBuf` impl, but may aid in type inference.
- `Utf8Path::as_std_path` converts a `Utf8Path` to a `Path`; equivalent to the
  `AsRef<&Path> for &Utf8Path` impl, but may aid in type inference.

## [1.0.4] - 2021-03-19

### Fixed

- `Hash` impls for `Utf8PathBuf` and `Utf8Path` now match as required by the `Borrow` contract ([#9]).

[#9]: https://github.com/camino-rs/camino/issues/9

## [1.0.3] - 2021-03-11

### Added

- `TryFrom<PathBuf> for Utf8PathBuf` and `TryFrom<&Path> for &Utf8Path`, both of which return new error types ([#6]).
- `AsRef<Utf8Path>`, `AsRef<Path>`, `AsRef<str>` and `AsRef<OsStr>` impls for `Utf8Components`, `Utf8Component` and
  `Iter`.

[#6]: https://github.com/camino-rs/camino/issues/6

## [1.0.2] - 2021-03-02

### Added

- `From` impls for converting a `&Utf8Path` or a `Utf8PathBuf` into `Box<Path>`, `Rc<Path>`, `Arc<Path>` and `Cow<'a, Path>`.
- `PartialEq` and `PartialOrd` implementations comparing `Utf8Path` and `Utf8PathBuf` with `Path`, `PathBuf` and its
  variants, and comparing `OsStr`, `OsString` and its variants.

## [1.0.1] - 2021-02-25

### Added

- More `PartialEq` and `PartialOrd` implementations.
- MSRV lowered to 1.34.

## [1.0.0] - 2021-02-23

Initial release.

[1.2.1]: https://github.com/camino-rs/camino/releases/tag/camino-1.2.1
[1.2.0]: https://github.com/camino-rs/camino/releases/tag/camino-1.2.0
[1.1.12]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.12
[1.1.11]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.11
[1.1.10]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.10
[1.1.9]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.9
[1.1.8]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.8
[1.1.7]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.7
[1.1.6]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.6
[1.1.5]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.5
[1.1.4]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.4
[1.1.3]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.3
[1.1.2]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.2
[1.1.1]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.1
[1.1.0]: https://github.com/camino-rs/camino/releases/tag/camino-1.1.0
[1.0.9]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.9
[1.0.8]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.8
[1.0.7]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.7
[1.0.6]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.6
[1.0.5]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.5
[1.0.4]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.4
[1.0.3]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.3
[1.0.2]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.2
[1.0.1]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.1
[1.0.0]: https://github.com/camino-rs/camino/releases/tag/camino-1.0.0
