# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [0.9.8] - 2025-08-22
### Added
- `MmapOptions::no_reserve_swap`.
  [@nhtyy](https://github.com/nhtyy)

## [0.9.7] - 2025-07-13
### Fixed
- Fix incomplete validation of mapping length, which could lead to violation of safety requirements of `slice::from_raw_parts` on 32-bit platforms.
  [@nabijaczleweli](https://github.com/nabijaczleweli)

## [0.9.6] - 2025-05-14
### Fixed
- Fix huge page mappings with non-default page-bits.
  [@Alex1s](https://github.com/Alex1s)

## [0.9.5] - 2024-09-13
### Added
- `Advise::is_supported` and `UncheckedAdvice::is_supported`. Linux only.
  [@xzfc](https://github.com/xzfc)
- Documentation improvements.
  [@RalfJung](https://github.com/RalfJung)
  [@betelgeuse](https://github.com/betelgeuse)
  [@ADSteele916](https://github.com/ADSteele916)

## [0.9.4] - 2024-01-25
### Changed
- The `libc` crate >= 0.2.151 is required now.

### Fixed
- Build on Android with an older `libc` crate.

## [0.9.3] - 2023-12-19
### Fixed
- Build on Android.

## [0.9.2] - 2023-12-17
### Fixed
- Build on FreeBSD.

## [0.9.1] - 2023-12-16
### Changed
- Added `MmapOptions::huge` method to support mapping hugetlb. Linux only.
  [@ollie-etl](https://github.com/ollie-etl)
  [@oliverbunting](https://github.com/oliverbunting)

## [0.9.0] - 2023-10-03
### Changed
- The `Advice` struct was split into two enums: `Advice` and `UncheckedAdvice`.<br>
  `Advice` can be passed to safe `advise` and `advise_range` methods.
  And `UncheckedAdvice` can be passed to unsafe `unchecked_advise`
  and `unchecked_advise_range` methods.<br>
  [@adamreichold](https://github.com/adamreichold)

## [0.8.0] - 2023-09-25
### Changed
- The `Advice` type is a struct and not an enum now.
  [@adamreichold](https://github.com/adamreichold)

### Fixed
- Some of the `Advise` variants were unsound and now require `unsafe` to be constructed.
  [@adamreichold](https://github.com/adamreichold)

## [0.7.1] - 2023-06-24
### Fixed
- Mapping beyond 4GB offset on 32 bit glibc. Linux-only.
  [@lvella](https://github.com/lvella)

## [0.7.0] - 2023-06-08
### Added
- `Mmap::remap`, `MmapMut::remap` and `MmapRaw::remap`. Linux-only.
  [@Phantomical](https://github.com/Phantomical)
- `Advice::PopulateRead` and `Advice::PopulateWrite`. Linux-only.
  [@Jesse-Bakker](https://github.com/Jesse-Bakker)

### Changed
- libc crate >= 0.2.143 is required now.

## [0.6.2] - 2023-05-24
### Fixed
- Alignment for empty files on Windows.
  [@timvisee](https://github.com/timvisee)

## [0.6.1] - 2023-05-10
### Added
- Add `MmapOptions::map_raw_read_only` to avoid intermediate invalid `Mmap` instances.
  [@adamreichold](https://github.com/adamreichold)

## [0.6.0] - 2023-05-09
### Changed
- `lock()` and `unlock` methods require `&self` and not `&mut self` now.
  [@timvisee](https://github.com/timvisee)

## [0.5.10] - 2023-02-22
### Added
- `MmapOptions::map_anon` accounts for `populate` on Linux now.
  [@jsgf](https://github.com/jsgf)

## [0.5.9] - 2023-02-17
### Added
- `From<Mmap> for MmapRaw` and `From<MmapMut> for MmapRaw`.
  [@swlynch99](https://github.com/swlynch99)
- `Mmap::advise_range`, `MmapMut::advise_range`, `MmapRaw::advise_range`.
  [@ho-229](https://github.com/ho-229)

## [0.5.8] - 2022-11-09
### Added
- `MmapRaw::advise`, `MmapRaw::lock` and `MmapRaw::unlock`.
  [@diwic](https://github.com/diwic)
- Improve `MmapMut::make_exec` documentation.

## [0.5.7] - 2022-08-15
### Changed
- Simplify file size retrieving code.
  [@saethlin](https://github.com/saethlin)

## [0.5.6] - 2022-08-11
### Added
- Memory locking and unlocking. See `Mmap::lock`, `Mmap::unlock`,
  `MmapMut::lock` and `MmapMut::unlock`.
  [@vmx](https://github.com/vmx)

## [0.5.5] - 2022-07-09
### Fixed
- Limit mapping length to `isize::MAX` to prevent undefined behavior
  on calling `std::slice::from_raw_parts`. Technically affects only 32-bit systems.
  [@adamreichold](https://github.com/adamreichold)

## [0.5.4] - 2022-06-04
### Added
- Add madvice operations specific to Darwin. [@turbocool3r](https://github.com/turbocool3r)
- Implement common traits for the `Advice` enum. [@nyurik](https://github.com/nyurik)

### Changed
- Make stub implementation Infallible. [@coolreader18](https://github.com/coolreader18)
- Use `tempfile` crate instead of `tempdir` in tests.
  [@alexanderkjall](https://github.com/alexanderkjall)

## [0.5.3] - 2022-02-10
### Added
- `Mmap::advise` and `MmapMut::advise`. [@nyurik](https://github.com/nyurik)

## [0.5.2] - 2022-01-10
### Added
- `flush`, `flush_async`, `flush_range` and `flush_async_range` to `MmapRaw` matching
  the corresponding methods on `MmapMut`.
  [@cberner](https://github.com/cberner)

## [0.5.1] - 2022-01-09
### Fixed
- Explicitly call `fstat64` on Linux, emscripten and l4re targets.
  [@adamreichold](https://github.com/adamreichold)

## [0.5.0] - 2021-09-19
### Added
- `MmapOptions` accepts any type that supports `RawHandle`/`RawFd` returning now.
  This allows using `memmap2` not only with Rust std types, but also with
  [async-std](https://github.com/async-rs/async-std) one.
  [@adamreichold](https://github.com/adamreichold)
- (unix) Memoize page size to avoid repeatedly calling into sysconf machinery.
  [@adamreichold](https://github.com/adamreichold)

### Changed
- (win) Use `std::os::windows::io::AsRawHandle` directly, without relying on `std::fs::File`.
  [@adamreichold](https://github.com/adamreichold)
- Do not panic when failing to release resources in Drop impls.
  [@adamreichold](https://github.com/adamreichold)

## [0.4.0] - 2021-09-16
### Added
- Optional [`StableDeref`](https://github.com/storyyeller/stable_deref_trait) support.
  [@SimonSapin](https://github.com/SimonSapin)

### Changed
- Mapping of zero-sized files is no longer an error.
  [@SimonSapin](https://github.com/SimonSapin)
- MSRV changed from 1.31 to 1.36

## [0.3.1] - 2021-08-15
### Fixed
- Integer overflow during file length calculation on 32bit targets.
- Stub implementation. [@Mrmaxmeier](https://github.com/Mrmaxmeier)

## [0.3.0] - 2021-06-10
### Changed
- `MmapOptions` allows mapping using Unix descriptors and not only `std::fs::File` now.
  [@mripard](https://github.com/mripard)

## [0.2.3] - 2021-05-24
### Added
- Allow compilation on unsupported platforms.
  The code will panic on access just like in `std`.
  [@jcaesar](https://github.com/jcaesar)

## [0.2.2] - 2021-04-03
### Added
- `MmapOptions::populate`. [@adamreichold](https://github.com/adamreichold)

### Fixed
- Fix alignment computation for `flush_async` to match `flush`.
  [@adamreichold](https://github.com/adamreichold)

## [0.2.1] - 2021-02-08
### Added
- `MmapOptions::map_raw` and `MmapRaw`. [@diwic](https://github.com/diwic)

## [0.2.0] - 2020-12-19
### Changed
- MSRV is 1.31 now (edition 2018).
- Make anonymous memory maps private by default on unix. [@CensoredUsername](https://github.com/CensoredUsername)
- Add `map_copy_read_only`. [@zseri](https://github.com/zseri)

## 0.1.0 - 2020-01-18
### Added
- Fork [memmap-rs](https://github.com/danburkert/memmap-rs).

### Changed
- Use `LICENSE-APACHE` instead of `README.md` for some tests since it's immutable.

### Removed
- `winapi` dependency. [memmap-rs/pull/89](https://github.com/danburkert/memmap-rs/pull/89)

[Unreleased]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.9.5...HEAD
[0.9.5]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.9.4...v0.9.5
[0.9.4]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.9.3...v0.9.4
[0.9.3]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.9.2...v0.9.3
[0.9.2]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.7.1...v0.8.0
[0.7.1]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.6.2...v0.7.0
[0.6.2]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.10...v0.6.0
[0.5.10]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.9...v0.5.10
[0.5.9]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.8...v0.5.9
[0.5.8]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.2.3...v0.3.0
[0.2.3]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/RazrFalcon/memmap2-rs/compare/v0.1.0...v0.2.0
