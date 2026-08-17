<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.7.0] - 2026-01-13
### Fixed
- [PR#105](https://github.com/EmbarkStudios/crash-handling/pull/105) fixed pthread interposition when statically linking `musl`.

### Changed
- [PR#111](https://github.com/EmbarkStudios/crash-handling/pull/111) updated MSRV to 1.85.0.

## [0.6.3] - 2025-05-13
### Fixed
- [PR#94](https://github.com/EmbarkStudios/crash-handling/pull/94) resolved a case of UB, resolving [#93](https://github.com/EmbarkStudios/crash-handling/issues/93). Thanks [@Chain-Fox](https://github.com/Chain-Fox)!
- [PR#96](https://github.com/EmbarkStudios/crash-handling/pull/96) resolved [#92](https://github.com/EmbarkStudios/crash-handling/issues/92) by avoid creating a copy of the crash context on the stack, which on `aarch64-linux` would result in a stack overflow in the signal handler.

### Added
- [PR#96](https://github.com/EmbarkStudios/crash-handling/pull/96) added testing of `aarch64-linux` to CI.

### Changed
- [PR#95](https://github.com/EmbarkStudios/crash-handling/pull/95) updated crates, and changed the edition to 2024.

## [0.6.2] - 2024-06-08
### Added
- [PR#86](https://github.com/EmbarkStudios/crash-handling/pull/86) (carrying on from [PR#85](https://github.com/EmbarkStudios/crash-handling/pull/85)) added support for [vectored exception handlers](https://learn.microsoft.com/en-us/windows/win32/debug/vectored-exception-handling) on Windows, which can catch heap corruption exceptions that the vanilla exception handler cannot catch. Thanks [Tom!](https://github.com/h3r2tic)!

## [0.6.1] - 2024-01-29
### Added
- [PR#81](https://github.com/EmbarkStudios/crash-handling/pull/81) resolved [#79](https://github.com/EmbarkStudios/crash-handling/issues/79) by adding `make_single_crash_event`.

## [0.6.0] - 2023-04-03
### Changed
- [PR#70](https://github.com/EmbarkStudios/crash-handling/pull/70) removed `winapi` in favor of embedded bindings.

## [0.5.1] - 2022-11-17
### Fixed
- [PR#64](https://github.com/EmbarkStudios/crash-handling/pull/64) fixed compilation for `aarch64-linux-android`. Additional targets were added to CI so they get caught before release.

## [0.5.0] - 2022-11-17
### Added
- [PR#62](https://github.com/EmbarkStudios/crash-handling/pull/62) and [PR#63](https://github.com/EmbarkStudios/crash-handling/pull/63) added support for handling `SIGABRT` on Windows.

### Fixed
- [PR#62](https://github.com/EmbarkStudios/crash-handling/pull/62) changed from using `RtlCaptureContext` on Windows to using `crash_context::capture_context`. This implementation fixes a crash issue due to `winapi` and `windows-sys` having [improper bindings](https://github.com/microsoft/win32metadata/issues/1044).

### Changed
- [PR#62](https://github.com/EmbarkStudios/crash-handling/pull/62) changed from using `RtlCaptureContext` on Windows to using `crash_context::capture_context`. This implementation additionally captures floating point and vector state on `x86_64` unlike `RtlCaptureContext`.

## [0.4.0] - 2022-10-21
### Added
- [PR#60](https://github.com/EmbarkStudios/crash-handling/pull/60) resolved [#59](https://github.com/EmbarkStudios/crash-handling/issues/59) by adding support for `PR_SET_PTRACER` before invoking the user callback, ensuring that an external process has permissions to perform `ptrace` operations on the crashing process, even if `/proc/sys/kernel/yama/ptrace_scope` is set to restricted (1), as this is the default for most newer distributions.

### Changed
- [PR#60](https://github.com/EmbarkStudios/crash-handling/pull/60) bumped `windows-sys` to 0.42.

## [0.3.3] - 2022-07-21
### Added
- [PR#46](https://github.com/EmbarkStudios/crash-handling/pull/46) resolved [#33](https://github.com/EmbarkStudios/crash-handling/issues/33) by adding support for `EXC_RESOURCE` exceptions. Since not all resource exceptions are fatal, they are checked and only reported to the user callback if they are indeed fatal.
- [PR#47](https://github.com/EmbarkStudios/crash-handling/pull/47) resolved [#34](https://github.com/EmbarkStudios/crash-handling/issues/34) by adding support for `EXC_GUARD` exceptions.

## [0.3.2] - 2022-07-19
### Fixed
- [PR#38](https://github.com/EmbarkStudios/crash-handling/pull/38) resolved [#31](https://github.com/EmbarkStudios/crash-handling/issues/31) and [#35](https://github.com/EmbarkStudios/crash-handling/issues/35) by adding support for 64-bit codes in the mach exception information, as well as now handling `EXC_CRASH` exceptions.
- [PR#43](https://github.com/EmbarkStudios/crash-handling/pull/42) resolved [#42](https://github.com/EmbarkStudios/crash-handling/issues/42) by fixing a bug on `aarch64-linux`. Thanks [@sfackler](https://github.com/sfackler)!

## [0.3.1] - 2022-05-25
### Changed
- Updated to `minidump-writer` 0.2.1 which includes support for MacOS thread names, and aligns on crash-context 0.3.0.

## [0.3.0] - 2022-05-23
### Added
- First usable release of `crash-context`, `crash-handler`, `sadness-generator`, and `minidumper` crates.

## [crash-handler-v0.1.0] - 2022-04-29
### Added
- Initial publish of crash-handler with Linux, Windows, and MacOS support

## [sadness-generator-v0.1.0] - 2022-04-29
### Added
- Initial published of sadness-generator, can generated crashes on Linux, Windows, and MacOS

## [crash-context-v0.2.0] - 2022-04-29
### Added
- Add Windows and MacOS support

## [crash-context-v0.1.0] - 2022-04-21
### Added
- Initial pass of crash-context, Linux only

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.7.0...HEAD
[0.7.0]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.6.3...crash-handler-0.7.0
[0.6.3]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.6.2...crash-handler-0.6.3
[0.6.2]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.6.1...crash-handler-0.6.2
[0.6.1]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.6.0...crash-handler-0.6.1
[0.6.0]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.5.1...crash-handler-0.6.0
[0.5.1]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.5.0...crash-handler-0.5.1
[0.5.0]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.4.0...crash-handler-0.5.0
[0.4.0]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.3.3...crash-handler-0.4.0
[0.3.3]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-0.3.2...crash-handler-0.3.3
[0.3.2]: https://github.com/EmbarkStudios/crash-handling/compare/0.3.1...crash-handler-0.3.2
[0.3.1]: https://github.com/EmbarkStudios/crash-handling/compare/0.3.1...0.3.1
[0.3.0]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-v0.1.0...0.3.0
[crash-handler-v0.1.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/crash-handler-v0.1.0
[sadness-generator-v0.1.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/sadness-generator-v0.1.0
[crash-context-v0.2.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/crash-context-v0.2.0
[crash-context-v0.1.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/crash-context-v0.1.0
