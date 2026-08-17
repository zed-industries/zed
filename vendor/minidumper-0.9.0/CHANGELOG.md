<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.9.0] - 2026-01-13
### Changed
- [PR#103](https://github.com/EmbarkStudios/crash-handling/pull/103) updated `minidump-writer` to 0.11.
- [PR#111](https://github.com/EmbarkStudios/crash-handling/pull/111) made `SocketName` public and removed the `From<&str>` implementation, resolving [#106](https://github.com/EmbarkStudios/crash-handling/issues/106).
- [PR#111](https://github.com/EmbarkStudios/crash-handling/pull/111) updated MSRV to 1.85.0.

## [0.8.3] - 2024-06-08
## [0.8.2] - 2024-02-15
### Changed
- [PR#83](https://github.com/EmbarkStudios/crash-handling/pull/83) updated `scroll` to 0.12.

## [0.8.1] - 2024-01-29
### Changed
- [PR#81](https://github.com/EmbarkStudios/crash-handling/pull/81) resolved [#80](https://github.com/EmbarkStudios/crash-handling/issues/80) by updating `polling` to 0.3.

## [0.8.0] - 2023-04-03
### Changed
- [PR#70](https://github.com/EmbarkStudios/crash-handling/pull/70) removed `winapi` in favor of embedded bindings.

## [0.7.0] - 2022-11-17
### Fixed
- [PR#63](https://github.com/EmbarkStudios/crash-handling/pull/63) replaced `windows-sys` with `winapi`, resolving [#61](https://github.com/EmbarkStudios/crash-handling/issues/61). Eventually `winapi` will be removed as well.

## [0.6.0] - 2022-10-21
### Changed
- [PR#60](https://github.com/EmbarkStudios/crash-handling/pull/60) updated `minidump-writer -> 0.5` and `windows-sys -> 0.42`.

## [0.5.1] - 2022-09-02
### Fixed
- [PR#58](https://github.com/EmbarkStudios/crash-handling/pull/58) fixed an issue on Linux where the socket poll would erroneously return an error if it was interrupted. This is now handled gracefully. Thanks [@MarcusGrass](https://github.com/MarcusGrass)!

## [0.5.0] - 2022-07-21
### Changed
- [PR#50](https://github.com/EmbarkStudios/crash-handling/pull/50) updated `minidump-writer` to take advantage of improvements in writing macos minidumps.

## [0.4.0] - 2022-07-19
### Changed
- [PR#41](https://github.com/EmbarkStudios/crash-handling/pull/41) added support for detecting stale client connections for cases where the OS might not efficiently close the client end of the connection so that the server notices and removes the client from the event loop.
- [PR#44](https://github.com/EmbarkStudios/crash-handling/pull/44) updated `minidump-writer` to 0.3, which includes improved support for MacOS

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
[Unreleased]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.9.0...HEAD
[0.9.0]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.8.3...minidumper-0.9.0
[0.8.3]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.8.2...minidumper-0.8.3
[0.8.2]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.8.1...minidumper-0.8.2
[0.8.1]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.8.0...minidumper-0.8.1
[0.8.0]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.7.0...minidumper-0.8.0
[0.7.0]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.6.0...minidumper-0.7.0
[0.6.0]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.5.1...minidumper-0.6.0
[0.5.1]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.5.0...minidumper-0.5.1
[0.5.0]: https://github.com/EmbarkStudios/crash-handling/compare/minidumper-0.4.0...minidumper-0.5.0
[0.4.0]: https://github.com/EmbarkStudios/crash-handling/compare/0.3.1...minidumper-0.4.0
[0.3.1]: https://github.com/EmbarkStudios/crash-handling/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/EmbarkStudios/crash-handling/compare/crash-handler-v0.1.0...0.3.0
[crash-handler-v0.1.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/crash-handler-v0.1.0
[sadness-generator-v0.1.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/sadness-generator-v0.1.0
[crash-context-v0.2.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/crash-context-v0.2.0
[crash-context-v0.1.0]: https://github.com/EmbarkStudios/crash-handling/releases/tag/crash-context-v0.1.0
