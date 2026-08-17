# Requirements Document

## Introduction

This feature automates the creation and distribution of binary packages for the crc-fast library when a tagged version release is created on GitHub. The system will build platform-specific packages containing both library files (dynamic and static) and command-line utilities, following industry-standard packaging conventions for each target platform and architecture combination.

## Glossary

- **Release Workflow**: A GitHub Actions workflow that triggers on git tag creation and produces binary artifacts
- **Binary Package**: A compressed archive containing compiled library files, headers, and executables for a specific platform-architecture combination
- **Dynamic Library**: A shared library file (.so, .dylib, or .dll) that can be linked at runtime
- **Static Library**: A library archive (.a or .lib) that is linked at compile time
- **CLI Binary**: Command-line executable programs (checksum, arch-check, get-custom-params)
- **Target Triple**: A Rust platform identifier (e.g., x86_64-unknown-linux-gnu)
- **Package Layout**: The directory structure and file organization within a binary package
- **Release Asset**: A file attached to a GitHub release that users can download

## Requirements

### Requirement 1

**User Story:** As a library consumer, I want to download pre-built binaries for my platform, so that I can use crc-fast without compiling from source

#### Acceptance Criteria

1. WHEN a git tag matching the pattern {MAJOR}.{MINOR}.{PATCH} is pushed, THE Release Workflow SHALL trigger automatically
2. THE Release Workflow SHALL build binaries for x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu, aarch64-apple-darwin, x86_64-pc-windows-msvc, and aarch64-pc-windows-msvc targets
3. THE Release Workflow SHALL execute all tests successfully before building release binaries
4. THE Release Workflow SHALL produce release-optimized builds using the existing Cargo.toml release profile (LTO, strip, opt-level 3)
5. THE Release Workflow SHALL upload all binary packages as GitHub release assets

### Requirement 2

**User Story:** As a C/C++ developer, I want library packages with both dynamic and static libraries, so that I can choose the linking strategy that fits my project

#### Acceptance Criteria

1. THE Release Workflow SHALL include both dynamic library files and static library files in library packages
2. WHERE the target is Linux, THE Release Workflow SHALL include libcrc_fast.so and libcrc_fast.a files
3. WHERE the target is macOS, THE Release Workflow SHALL include libcrc_fast.dylib and libcrc_fast.a files
4. WHERE the target is Windows, THE Release Workflow SHALL include crc_fast.dll, crc_fast.dll.lib (import library), and crc_fast.lib (static library) files
5. THE Release Workflow SHALL include the libcrc_fast.h header file in all library packages

### Requirement 3

**User Story:** As a command-line user, I want executable binaries for the checksum utilities, so that I can use crc-fast tools directly from my terminal

#### Acceptance Criteria

1. THE Release Workflow SHALL build checksum, arch-check, and get-custom-params executable binaries
2. THE Release Workflow SHALL include CLI binaries in library packages
3. WHERE the target is Windows, THE Release Workflow SHALL include .exe extensions on executable files
4. WHERE the target is Linux or macOS, THE Release Workflow SHALL set executable permissions on binary files

### Requirement 4

**User Story:** As a package consumer, I want packages organized according to platform conventions, so that I can easily integrate them into my build system

#### Acceptance Criteria

1. WHERE the target is Linux, THE Release Workflow SHALL organize files in lib/ and include/ directories
2. WHERE the target is macOS, THE Release Workflow SHALL organize files in lib/ and include/ directories
3. WHERE the target is Windows, THE Release Workflow SHALL organize files in bin/ and include/ directories
4. THE Release Workflow SHALL place CLI executables in bin/ subdirectory for Linux and macOS targets
5. THE Release Workflow SHALL place CLI executables in bin/ subdirectory for Windows targets alongside the DLL

### Requirement 5

**User Story:** As a release maintainer, I want descriptive package names, so that users can easily identify the correct package for their platform

#### Acceptance Criteria

1. THE Release Workflow SHALL name packages using the format crc-fast-{version}-{os}-{arch}.{extension}
2. WHERE the target is Linux, THE Release Workflow SHALL use .tar.gz extension
3. WHERE the target is macOS, THE Release Workflow SHALL use .tar.gz extension
4. WHERE the target is Windows, THE Release Workflow SHALL use .zip extension
5. THE Release Workflow SHALL include the git tag version in the package filename

### Requirement 6

**User Story:** As a quality-conscious maintainer, I want the release workflow to reuse existing test infrastructure, so that releases are only created when all tests pass

#### Acceptance Criteria

1. THE Release Workflow SHALL depend on successful completion of the existing test workflow
2. THE Release Workflow SHALL use the same Rust toolchain version as the test workflow
3. THE Release Workflow SHALL fail and prevent release creation if any test fails
4. THE Release Workflow SHALL run cargo fmt and cargo clippy checks before building
5. THE Release Workflow SHALL only trigger on tags matching [0-9]+.[0-9]+.[0-9]+ pattern

### Requirement 7

**User Story:** As a downstream consumer, I want packages to include version information, so that I can verify I'm using the correct release

#### Acceptance Criteria

1. THE Release Workflow SHALL include a VERSION or version.txt file in each package
2. THE Release Workflow SHALL include a README or INSTALL file with basic usage instructions
3. THE Release Workflow SHALL include LICENSE files in each package
4. THE Release Workflow SHALL generate a checksum file (SHA256) for each package
5. THE Release Workflow SHALL upload checksum files as separate release assets
