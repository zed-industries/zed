<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.11.0] - 2025-09-15
### Changed
- [PR#146](https://github.com/rust-minidump/minidump-writer/pull/146) changed `windows::MinidumpWriter::dump_crash_context` to take crash_context by reference.
- [PR#155](https://github.com/rust-minidump/minidump-writer/pull/155) refactored a large portion of the code.

### Fixed
- [PR#150](https://github.com/rust-minidump/minidump-writer/pull/150) fixed undefined behavior in the Linux crash dump writing.

## [0.10.2] - 2025-02-03
### Added
- [PR#143](https://github.com/rust-minidump/minidump-writer/pull/143)
  - turn many errors that are currently treated as critical (and thus prevent minidump generation) into non-critical "soft" errors
  - collect non-critical errors and serialize them into a new JSON stream in the minidump

### Changed
- [PR#145](https://github.com/rust-minidump/minidump-writer/pull/145) updated dependencies.

## [0.10.1] - 2024-09-20
### Fixed
- [PR#129](https://github.com/rust-minidump/minidump-writer/pull/129) added checking of additions to ensure invalid memory offsets are gracefully handled.
- [PR#135](https://github.com/rust-minidump/minidump-writer/pull/135) resolved [#134](https://github.com/rust-minidump/minidump-writer/issues/134) by attempting to read the executables build id from the mapped file if it could not be retrieved from process memory.
- [PR#136](https://github.com/rust-minidump/minidump-writer/pull/136) changed to an older type to avoid requiring a semi-recent rust version.

## [0.10.0] - 2024-08-20
### Changed
- [PR#118](https://github.com/rust-minidump/minidump-writer/pull/118) resolved [#72](https://github.com/rust-minidump/minidump-writer/issues/72) by adding support for reading process memory via `process_vm_readv` and `/proc/{pid}/mem`, in addition to the original `PTRACE_PEEKDATA`. This gives significant performance benefits as memory can now be read in blocks of arbitrary size instead of word-by-word with ptrace.
- [PR#128](https://github.com/rust-minidump/minidump-writer/pull/128) and [PR#133](https://github.com/rust-minidump/minidump-writer/pull/133) updated the lockfile.

### Fixed
- [PR#127](https://github.com/rust-minidump/minidump-writer/pull/127) resolved [#27](https://github.com/rust-minidump/minidump-writer/issues/27) by allowing programs to pass the needed auxv information, still falling back to reading `/proc/{pid}/auxv` to fill missing information, and being more permissive by still writing a dump if some or all of the auxv information could not be retrieved successfully rather than completely failing to write the minidump.
- [PR#131](https://github.com/rust-minidump/minidump-writer/pull/131) resolved [#124](https://github.com/rust-minidump/minidump-writer/issues/124) by reinjecting non-`SIGSTOP` signals after `ptrace::attach` so that the thread would be in the correct state after `ptrace::detach`.

## [0.9.0] - 2024-07-20
### Fixed
- [PR#117](https://github.com/rust-minidump/minidump-writer/pull/117) resolved [#79](https://github.com/rust-minidump/minidump-writer/issues/79) by enabling reading of a module's build id and soname directly from the mapped process rather than relying on file reading, though that is still used as a fallback.

### Changed
- [PR#126](https://github.com/rust-minidump/minidump-writer/pull/126) updated `minidump-common` -> 0.22.

## [0.8.9] - 2024-04-01
### Fixed
- [PR#110](https://github.com/rust-minidump/minidump-writer/pull/110) changed it so that `SIGCONT` is sent regardless if the process was not able to be `SIGSTOP`ed quickly enough.
- [PR#113](https://github.com/rust-minidump/minidump-writer/pull/113) fixed a segfault(!) on linux if it was compiled with rustc 1.77.0 in release mode.

## [0.8.8] - 2024-03-21
### Fixed
- [PR#108](https://github.com/rust-minidump/minidump-writer/pull/108) resolved [#28](https://github.com/rust-minidump/minidump-writer/issues/28) by sending a `SIGSTOP` to the process that is about to be dumped to (hopefully) increase the robustness of the dumping process by reducing the chance of errors, particularly with regard to threads. This is done as a best effort, and will perform the old behavior if the process has not stopped within a timeout (by default 100ms), which can be overriden by the user.

## [0.8.7] - 2024-03-04
### Changed
- [PR#106](https://github.com/rust-minidump/minidump-writer/pull/106) bumped `minidump-common`, `minidump`, `minidump-processor`, and `minidump-unwind` -> 0.21.

## [0.8.6] - 2024-02-26
### Changed
- [PR#104](https://github.com/rust-minidump/minidump-writer/pull/104) slightly tweaked .so version parsing in the case of more "exotic" versions such as `libdbus-1.so.3.34.2rc5`. Previously this was parsed as `3.34.25` but would cause ambiguity if there was ever an _actual_ .25 patch/age in the future. Now, the last version is parsed as 1-2 numbers, ignoring non-digit characters if the last component has them. If 2 numbers are parsed, the last number is now placed in [VS_FIXEDFILEINFO::product_version_lo](https://docs.rs/minidump-common/latest/minidump_common/format/struct.VS_FIXEDFILEINFO.html#structfield.product_version_lo) so that it is distinct from the patch/age component placed in [VS_FIXEDFILEINFO::product_version_hi](https://docs.rs/minidump-common/latest/minidump_common/format/struct.VS_FIXEDFILEINFO.html#structfield.product_version_hi).

## [0.8.5] - 2024-02-23
### Added
- [PR#103](https://github.com/rust-minidump/minidump-writer/pull/103) added `.so` file versions as additional metadata to minidumps, resolving [this Mozilla bug](https://bugzilla.mozilla.org/show_bug.cgi?id=1847098). There is no true standard for .so file versions, so this is a best effort to pull what version information we can from the .so filename. The version components are `major.minor.release` similarly to semver, where `major` -> [VS_FIXEDFILEINFO::file_version_hi](https://docs.rs/minidump-common/latest/minidump_common/format/struct.VS_FIXEDFILEINFO.html#structfield.file_version_hi), `major` -> [VS_FIXEDFILEINFO::file_version_lo](https://docs.rs/minidump-common/latest/minidump_common/format/struct.VS_FIXEDFILEINFO.html#structfield.file_version_lo),  and `release` -> [VS_FIXEDFILEINFO::product_version_hi](https://docs.rs/minidump-common/latest/minidump_common/format/struct.VS_FIXEDFILEINFO.html#structfield.product_version_hi)
  - `libmozsandbox.so` -> `0.0.0`
  - `libstdc++.so.6.0.32` -> `6.0.32`
  - `libcairo-gobject.so.2.11800.0` -> `2.11800.0`
  - `libm.so.6` -> `6.0.0`
  - `libabsl_time_zone.so.20220623.0.0` -> `20220623.0.0`
  - `libdbus-1.so.3.34.2rc5` -> `3.34.25`

## [0.8.4] - 2024-02-15
### Changed
- [PR#97](https://github.com/rust-minidump/minidump-writer/pull/97) bumped `goblin` -> 0.8.
- [PR#99](https://github.com/rust-minidump/minidump-writer/pull/99) bumped `minidump-common` -> 0.20, `scroll` -> 0.12, `memmap2` -> 0.9.

## [0.8.3] - 2023-11-07
### Added
- [PR#94](https://github.com/rust-minidump/minidump-writer/pull/94) added support for writing [file information](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_handle_descriptor) for every file open in the process the dump is being performed for into the [`MINIDUMP_HANDLE_DATA_STREAM`](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_handle_data_stream) stream.
- [PR#90](https://github.com/rust-minidump/minidump-writer/pull/90) added support for including the `/proc/<pid>/limits` file in the [`MozLinuxLimits`](https://docs.rs/minidump-common/latest/minidump_common/format/enum.MINIDUMP_STREAM_TYPE.html#variant.MozLinuxLimits) stream. This information can be used together with the file information described above to diagnose situations where the process was killed by the kernel due to file handle limits being hit. Thanks [@lissyx](https://github.com/lissyx)!

### Changed
- [PR#94](https://github.com/rust-minidump/minidump-writer/pull/94) updated several dependencies to align with `minidump-common`, which was also bumped.

## [0.8.2] - 2023-09-21
### Added
- [PR#86](https://github.com/rust-minidump/minidump-writer/pull/86) added support for `i686-android-linux`.

### Fixed
- [PR#85](https://github.com/rust-minidump/minidump-writer/pull/85) removed the dependency on `chrono`.
- [PR#89](https://github.com/rust-minidump/minidump-writer/pull/89) resolved [#88](https://github.com/rust-minidump/minidump-writer/issues/88) by merging ranges that were mapped, but had 1 or more unmapped ranges in between them.

### Changed
- [PR#87](https://github.com/rust-minidump/minidump-writer/pull/87) updated some dependencies.

## [0.8.1] - 2023-06-21
### Added
- [PR#70](https://github.com/rust-minidump/minidump-writer/pull/70) resolved [#8](https://github.com/rust-minidump/minidump-writer/issues/8) by adding support for writing `MemoryInfoListStream` on Linux/Android targets, this allows minidump consumers to process minidumps more easily without needing to parse and understand Linux-specific information. Thanks [@afranchuk](https://github.com/afranchuk)!
- [PR#81](https://github.com/rust-minidump/minidump-writer/pull/81) stabilized `arm` and `aarch64` support for `unknown-linux` and `linux-android`, as well as adding support for `x86_64-linux-android`.

### Changed
- [PR#70](https://github.com/rust-minidump/minidump-writer/pull/70) replaced the custom reading of procfs information used when generating a minidump on Linux to use the `procfs` crate instead, removing a bunch of code.
- [PR#80](https://github.com/rust-minidump/minidump-writer/pull/80) along with [PR#84](https://github.com/rust-minidump/minidump-writer/pull/84) replaced `procfs` with `procfs-core`, removing unneeded dependencies such as `windows-sys`.

### Fixed
- [PR#78](https://github.com/rust-minidump/minidump-writer/pull/78) resolved [#24](https://github.com/rust-minidump/minidump-writer/issues/24) by ignoring guard pages when dumping the stack to the minidump in the event of a stack overflow.
- [PR#83](https://github.com/rust-minidump/minidump-writer/pull/83) resolved [#82](https://github.com/rust-minidump/minidump-writer/issues/82) by correctly aligning a structure.

## [0.8.0] - 2023-04-03
### Removed
- [PR#77](https://github.com/rust-minidump/minidump-writer/pull/77) removed the dependency on `winapi`, all bindings are either part of `minidump-writer` or `crash-context` now.

### Changed
- [PR#77](https://github.com/rust-minidump/minidump-writer/pull/77) closed [#67](https://github.com/rust-minidump/minidump-writer/issues/67) by allowing the user to specify the `MinidumpType` flags when creating a minidump.

### Fixed
- [PR#68](https://github.com/rust-minidump/minidump-writer/pull/68) resolved [#29](https://github.com/rust-minidump/minidump-writer/issues/29) by ignoring the bening `ESRCH` error when detaching pthreads. Thanks [@afranchuk](https://github.com/afranchuk)!
- [PR#74](https://github.com/rust-minidump/minidump-writer/pull/74) resolved [#73](https://github.com/rust-minidump/minidump-writer/issues/73) by ensuring the `NT_GNU_BUILD_ID` section had the proper correct `GNU` name before using it as the build identifier.

## [0.7.0] - 2022-11-17
### Changed
- [PR#65](https://github.com/rust-minidump/minidump-writer/pull/65) updated `crash-context` to 0.5, which has support for a custom `capture_context` to replace `RtlCaptureContext` on Windows, due to improper bindings and deficiencies, resolving [#63](https://github.com/rust-minidump/minidump-writer/issues/63).
- [PR#65](https://github.com/rust-minidump/minidump-writer/pull/65) replaced _most_ of the custom bindings from [PR#60](https://github.com/rust-minidump/minidump-writer/pull/60) with bindings from either `crash-context` or `winapi`.

## [0.6.0] - 2022-11-15
### Changed
- [PR#60](https://github.com/rust-minidump/minidump-writer/pull/60) removed the dependency on `windows-sys` due the massive version churn, resolving [#58](https://github.com/rust-minidump/minidump-writer/issues/58). This should allow projects to more easily integrate this crate into their project without introducing multiple versions of transitive dependencies.
- [PR#62](https://github.com/rust-minidump/minidump-writer/pull/62) replaced `MDExceptionCodeLinux` with `minidump_common::ExceptionCodeLinux`.
- [PR#64](https://github.com/rust-minidump/minidump-writer/pull/64) updated dependencies.

## [0.5.0] - 2022-10-21
### Changed
- [PR#53](https://github.com/rust-minidump/minidump-writer/pull/53) made the `mem_writer` and `dir_section` modules public. Thanks [@sage-msft](https://github.com/sage-msft)!
- [PR#55](https://github.com/rust-minidump/minidump-writer/pull/55) bumped `nix`, `minidump-common`, `minidump`, `minidump-processor` and `dump_syms`. Thanks
[@sfackler](https://github.com/sfackler)!
- [PR#57](https://github.com/rust-minidump/minidump-writer/pull/57) bumped `windows-sys` to 0.42.

### Removed
- [PR#56](https://github.com/rust-minidump/minidump-writer/pull/56) removed the writing of the [HandleOperationListStream](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_handle_operation_list) stream, as it was essentially untested and was of dubious usefulness.

## [0.4.0] - 2022-07-21
### Changed
- [PR#50](https://github.com/rust-minidump/minidump-writer/pull/50) updated `minidump-common` and `crash-context`.

### Fixed
- [PR#50](https://github.com/rust-minidump/minidump-writer/pull/50) resolved [#33](https://github.com/rust-minidump/minidump-writer/issues/33) by encoding the full exception info in the `exception_information` field of the exception stream.
- [PR#50](https://github.com/rust-minidump/minidump-writer/pull/50) resolved [#34](https://github.com/rust-minidump/minidump-writer/issues/34) by unwrapping `EXC_CRASH` exceptions to retrieve the wrapped exception.

## [0.3.1] - 2022-07-18
### Fixed
- [PR#47](https://github.com/rust-minidump/minidump-writer/pull/47) resolved [#46](https://github.com/rust-minidump/minidump-writer/issues/46) by handling the special case of `dyld`.

## [0.3.0] - 2022-07-15
### Fixed
- [PR#42](https://github.com/rust-minidump/minidump-writer/pull/42) resolved [#41](https://github.com/rust-minidump/minidump-writer/issues/41) by capping the VM read of task memory to avoid a syscall failure, as well as made it so that if an error does occur when reading the module's file path, the module is still written to the minidump, as the file path is less important than the UUID in terms of module identification.
- [PR#44](https://github.com/rust-minidump/minidump-writer/pull/44) resolved [#43](https://github.com/rust-minidump/minidump-writer/issues/43) by correctly calculating the base address of each loaded module. The bug was inherited from Breakpad.
- [PR#44](https://github.com/rust-minidump/minidump-writer/pull/44) and [PR#45](https://github.com/rust-minidump/minidump-writer/pull/45) resolved [#37](https://github.com/rust-minidump/minidump-writer/issues/37) by making the `crash_context::CrashContext` optional on MacOS and Windows, to make creating a minidump without necessarily having an actual crash more convenient for users.

## [0.2.1] - 2022-05-25
### Added
- [PR#32](https://github.com/rust-minidump/minidump-writer/pull/32) resolved [#23](https://github.com/rust-minidump/minidump-writer/issues/23) by adding support for the thread names stream on MacOS.

## [0.2.0] - 2022-05-23
### Added
- [PR#21](https://github.com/rust-minidump/minidump-writer/pull/21) added an initial implementation for `x86_64-apple-darwin` and `aarch64-apple-darwin`

## [0.1.0] - 2022-04-26
### Added
- Initial release, including basic support for `x86_64-unknown-linux-gnu/musl` and `x86_64-pc-windows-msvc`

<!-- next-url -->
[Unreleased]: https://github.com/rust-minidump/minidump-writer/compare/0.11.0...HEAD
[0.11.0]: https://github.com/rust-minidump/minidump-writer/compare/0.10.2...0.11.0
[0.10.2]: https://github.com/rust-minidump/minidump-writer/compare/0.10.1...0.10.2
[0.10.1]: https://github.com/rust-minidump/minidump-writer/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/rust-minidump/minidump-writer/compare/0.9.0...0.10.0
[0.9.0]: https://github.com/rust-minidump/minidump-writer/compare/0.8.9...0.9.0
[0.8.9]: https://github.com/rust-minidump/minidump-writer/compare/0.8.8...0.8.9
[0.8.8]: https://github.com/rust-minidump/minidump-writer/compare/0.8.7...0.8.8
[0.8.7]: https://github.com/rust-minidump/minidump-writer/compare/0.8.6...0.8.7
[0.8.6]: https://github.com/rust-minidump/minidump-writer/compare/0.8.5...0.8.6
[0.8.5]: https://github.com/rust-minidump/minidump-writer/compare/0.8.4...0.8.5
[0.8.4]: https://github.com/rust-minidump/minidump-writer/compare/0.8.3...0.8.4
[0.8.3]: https://github.com/rust-minidump/minidump-writer/compare/0.8.2...0.8.3
[0.8.2]: https://github.com/rust-minidump/minidump-writer/compare/0.8.1...0.8.2
[0.8.1]: https://github.com/rust-minidump/minidump-writer/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/rust-minidump/minidump-writer/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/rust-minidump/minidump-writer/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/rust-minidump/minidump-writer/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/rust-minidump/minidump-writer/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/rust-minidump/minidump-writer/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/rust-minidump/minidump-writer/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/rust-minidump/minidump-writer/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/rust-minidump/minidump-writer/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/rust-minidump/minidump-writer/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/rust-minidump/minidump-writer/releases/tag/0.1.0
