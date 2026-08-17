# Changelog

Notable changes to this project should be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
The backtrace crate attempts to adhere to the modified [Cargo interpretation of SemVer](https://doc.rust-lang.org/cargo/reference/resolver.html#semver-compatibility).
As a unique component of `std` it may make exceptional changes in order to support `std`.

## [Unreleased]

## [0.3.76](https://github.com/rust-lang/backtrace-rs/compare/backtrace-v0.3.75...backtrace-v0.3.76) - 2025-09-26

### Behavior
- Fix inverted polarity of "full printing" logic in rust-lang/backtrace-rs#726:
  Previously we used to do the opposite of what you would expect.

### Platform Support

- Windows: Removed hypothetical soundness risk from padding bytes in rust-lang/backtrace-rs#737
- Fuchsia: Added appropriate alignment checks during `Elf_Nhdr` parsing in rust-lang/backtrace-rs#725
- Cygwin: Added support in rust-lang/backtrace-rs#704
- Windows (32-bit Arm): Restore support in rust-lang/backtrace-rs#685
- NuttX (32-bit Arm): Use builtin `_Unwind_GetIP` in rust-lang/backtrace-rs#692
- RTEMS: Enable libunwind in rust-lang/backtrace-rs#682

### Dependencies

- Update cpp_demangle to 0.5 in rust-lang/backtrace-rs#732
- Update memchr to 2.7.6 in rust-lang/backtrace-rs#734
- Switch from windows-targets to windows-link in rust-lang/backtrace-rs#727
- Update ruzstd to 0.8.1 in rust-lang/backtrace-rs#718
- Update object to 0.37 in rust-lang/backtrace-rs#718
- Update addr2line to 0.25 in rust-lang/backtrace-rs#718
