<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.6.3] - 2024-07-25
### Fixed
- [PR#89](https://github.com/EmbarkStudios/crash-handling/pull/89) fixed compilation for `arm-unknown-linux-gnueabihf`...again.

## [0.6.2] - 2024-06-08
### Changed
- Update MSRV to 1.62.0

## [0.6.1] - 2023-06-19
### Added
- [PR#76](https://github.com/EmbarkStudios/crash-handling/pull/76) added support for `i686-linux-android` and `x86_64-linux-android`. Thanks [@gabrielesvelto](https://github.com/gabrielesvelto)!

## [0.6.0] - 2023-04-03
### Changed
- [PR#70](https://github.com/EmbarkStudios/crash-handling/pull/70) removed the `winapi` dependency in favor of embedded bindings to avoid dependencies.
- [PR#70](https://github.com/EmbarkStudios/crash-handling/pull/70) removed the asm implementations for Windows CPU context retrieval in favor of using `RtlCaptureContext`. This means that floating state is not captured, but is otherwise and improvement.

### Added
- [PR#68](https://github.com/EmbarkStudios/crash-handling/pull/68) added capture context support for x86 Windows, but this change was supplanted in [PR#70](https://github.com/EmbarkStudios/crash-handling/pull/70) to use `RtlCaptureContext` instead.

### Fixed
- [PR#71](https://github.com/EmbarkStudios/crash-handling/pull/71) fixed the definition of `mcontext_t` for `i686-unknow-linux`. Thanks [@afranchuk](https://github.com/afranchuk)!

## [0.5.1] - 2022-11-17
### Fixed
- [PR#66](https://github.com/EmbarkStudios/crash-handling/pull/66) (apparently) resolved [#65](https://github.com/EmbarkStudios/crash-handling/issues/65) by...changing from AT&T to Intel syntax. This shouldn't have changed anything, but it did, and I'm too tired and have other things to work on, so here we are.

## [0.5.0] - 2022-11-17
### Added
- [PR#62](https://github.com/EmbarkStudios/crash-handling/pull/62) added a replacement implementation of `RtlCaptureContext`, and defines its own `CONTEXT` structure. This was needed due to both `winapi` and `windows-sys` incorrectly defining the `CONTEXT` and related structures on both x86_64 and aarch64 to not be correctly aligned, as well as `RtlCaptureContext` not actually capturing the floating point and vector state.

## [0.4.0] - 2022-07-21
### Added
- [PR#46](https://github.com/EmbarkStudios/crash-handling/pull/46) added support for unpacking `EXC_RESOURCE` exceptions on MacOS.
- [PR#47](https://github.com/EmbarkStudios/crash-handling/pull/47) added support for unpacking `EXC_GUARD` exceptions on MacOS.

### Changed
- [PR#47](https://github.com/EmbarkStudios/crash-handling/pull/47) changed `ExceptionInfo` to use unsigned types for all of its fields. While these are declared as signed, in practice all usage of them is as unsigned integers.

### Fixed
- [PR#47](https://github.com/EmbarkStudios/crash-handling/pull/47) fixed a potential issue with the IPC exception passing due to the structure's not being `#[repr(C, packed(4))]`, and the receiving side not (properly) accounting for the trailer that is added by the kernel to every mach msg.

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
[Unreleased]: https://github.com/EmbarkStudios/crash-handling/compare/{{tag_name}}...HEAD
[0.6.3]: https://github.com/EmbarkStudios/crash-handling/compare/crash-context-0.6.2...{{tag_name}}
[0.6.2]: https://github.com/EmbarkStudios/crash-handling/compare/crash-context-0.6.1...crash-context-0.6.2
[0.6.1]: https://github.com/EmbarkStudios/crash-handling/compare/crash-context-0.6.0...crash-context-0.6.1
[0.6.0]: https://github.com/EmbarkStudios/crash-handling/compare/crash-context-0.5.1...crash-context-0.6.0
[0.5.1]: https://github.com/EmbarkStudios/crash-handling/compare/crash-context-0.5.0...crash-context-0.5.1
[0.5.0]: https://github.com/EmbarkStudios/crash-handling/compare/crash-context-0.4.0...crash-context-0.5.0
[0.4.0]: https://github.com/EmbarkStudios/crash-handling/compare/0.3.1...crash-context-0.4.0
[0.3.1]: https://github.com/EmbarkStudios/crash-handling/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-v0.1.0...0.3.0
[crash-handler-v0.1.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/crash-handler-v0.1.0
[sadness-generator-v0.1.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/sadness-generator-v0.1.0
[crash-context-v0.2.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/crash-context-v0.2.0
[crash-context-v0.1.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/crash-context-v0.1.0
