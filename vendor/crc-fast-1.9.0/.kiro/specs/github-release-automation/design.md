# Design Document

## Overview

This design implements a GitHub Actions workflow that automatically builds and publishes binary packages when version tags are pushed to the repository. The workflow leverages the existing test infrastructure, builds optimized binaries for five target platforms, packages them according to platform conventions, and uploads them as GitHub release assets.

The solution uses a single workflow file that orchestrates testing, building, packaging, and publishing steps. It reuses the existing test workflow to ensure quality gates are met before creating releases.

## Architecture

### Workflow Structure

The release workflow consists of three main stages:

1. **Validation Stage**: Triggers existing test workflow and waits for completion
2. **Build Stage**: Compiles release binaries for all target platforms in parallel
3. **Publish Stage**: Creates packages, generates checksums, and uploads to GitHub release

### Workflow Trigger

```yaml
on:
  push:
    tags:
      - '[0-9]+.[0-9]+.[0-9]+'
```

This pattern matches semantic version tags like `1.5.0`, `2.0.1`, etc.

### Build Matrix

The workflow uses a matrix strategy to build for multiple platforms:

| Target Triple | OS Runner | Package Extension | Runner Rationale |
|--------------|-----------|-------------------|------------------|
| x86_64-unknown-linux-gnu | ubuntu-22.04 | .tar.gz | Native x86_64 Linux runner, explicit version for stability |
| aarch64-unknown-linux-gnu | ubuntu-22.04-arm | .tar.gz | Native ARM64 Linux runner, explicit version for stability |
| aarch64-apple-darwin | macos-14 | .tar.gz | Native Apple Silicon runner (M1/M2), explicit version for stability |
| x86_64-pc-windows-msvc | windows-2022 | .zip | Native x86_64 Windows runner, explicit version for stability |
| aarch64-pc-windows-msvc | windows-11-arm | .zip | Native ARM64 Windows runner (only ARM64 option available) |

**Runner Selection Rationale**:

#### General Principles
1. **Native Compilation**: Each runner matches the target architecture for optimal performance and to leverage architecture-specific CPU features (AVX-512, NEON, etc.)
2. **Binary Compatibility**: Binaries built on older OS versions are forward-compatible with newer versions, but not backward-compatible
3. **Single Binary Per Architecture**: One binary package per architecture is sufficient due to OS forward compatibility
4. **Explicit Versions**: Using explicit runner versions (ubuntu-22.04, windows-2022, macos-14) instead of -latest provides stability and predictability, preventing unexpected changes when GitHub updates their -latest pointers

#### Platform-Specific Decisions

**Linux (ubuntu-22.04 for x86_64, ubuntu-22.04-arm for aarch64)**:
- Explicit version (22.04) ensures consistent build environment over time
- Ubuntu 22.04 uses glibc 2.35, providing good compatibility with modern distributions
- Ubuntu binaries have excellent forward compatibility across distributions (Ubuntu, Debian, RHEL, etc.)
- When ubuntu-latest moves to 24.04 or newer, our release builds remain stable on 22.04
- No need for multiple Linux versions; one binary per architecture covers the ecosystem
- Using explicit versions allows intentional upgrades rather than automatic changes

**macOS (macos-14 for aarch64)**:
- `macos-14` is the first stable Apple Silicon (M1/M2) runner
- macOS binaries built on older versions run on newer versions (forward compatible)
- `macos-15` and `macos-latest` would work but offer no compatibility advantage
- Using `macos-14` provides maximum compatibility (works on macOS 14, 15, and future versions)
- No x86_64 macOS build needed: Apple Silicon Macs can run x86_64 binaries via Rosetta 2, but native ARM64 is preferred
- Single ARM64 binary covers all Apple Silicon Macs (M1, M2, M3, M4, etc.)

**Windows (windows-2022 for x86_64, windows-11-arm for aarch64)**:
- Explicit version (2022) ensures consistent build environment over time
- Windows Server 2022 binaries have excellent forward compatibility (run on Windows 11, future versions)
- MSVC runtime is statically linked in Rust release builds, eliminating runtime dependencies
- When windows-latest moves to Server 2025 or newer, our release builds remain stable on 2022
- `windows-11-arm` is the only ARM64 Windows runner available (no version choice)
- Using explicit versions allows intentional upgrades rather than automatic changes
- No need for multiple Windows versions; one binary per architecture covers the ecosystem

#### Why Not Multiple OS Versions?

**Rejected Approach**: Building separate packages for each OS version (e.g., macos-14, macos-15, ubuntu-22.04, ubuntu-24.04)

**Reasons**:
1. **Forward Compatibility**: Binaries built on older OS versions work on newer versions
2. **Maintenance Burden**: Multiple packages per architecture increases complexity without benefit
3. **User Confusion**: Users would need to choose between multiple packages for the same architecture
4. **Storage Costs**: More packages means more storage and bandwidth
5. **Testing Overhead**: The existing test workflow already validates across multiple OS versions (macos-14, macos-15, ubuntu-22.04, ubuntu-24.04, etc.), proving binary compatibility

**Conclusion**: One binary package per architecture provides maximum compatibility with minimum complexity. The test workflow validates compatibility across OS versions, while the release workflow builds on the oldest supported version for maximum forward compatibility.

## Components and Interfaces

### 1. Release Workflow File

**Location**: `.github/workflows/release.yml`

**Responsibilities**:
- Trigger on version tags
- Wait for test workflow completion
- Build release binaries for all targets
- Create platform-specific packages
- Generate SHA256 checksums
- Upload packages and checksums to GitHub release

**Key Jobs**:

#### Job: `validate`
- Waits for the existing test workflow to complete for the tagged commit
- Uses an action like `lewagon/wait-on-check-action` to poll test workflow status
- Fails the release if tests don't pass
- Includes timeout to prevent infinite waiting (e.g., 60 minutes)
- Runs quality checks (fmt, clippy) after tests pass

#### Job: `build` (matrix)
- Depends on `validate` job
- Checks out code
- Sets up Rust toolchain (stable)
- Runs quality checks (fmt, clippy)
- Builds release binaries with `cargo build --release` (LTO and stripping handled by Cargo.toml profile)
- Builds CLI binaries (checksum, arch-check, get-custom-params)
- Stages files in platform-specific directory structure
- Creates compressed package
- Generates SHA256 checksum
- Uploads package and checksum as artifacts

#### Job: `publish`
- Depends on all `build` jobs
- Downloads all artifacts
- Checks if GitHub release already exists for the tag
- If release exists, uploads packages and checksums to existing release
- If release doesn't exist, creates new release and uploads assets
- Allows manual release creation with custom notes before workflow runs

### 2. Package Structure

#### Linux Packages (x86_64 and aarch64)

```
crc-fast-{version}-linux-{arch}/
├── lib/
│   ├── libcrc_fast.so
│   └── libcrc_fast.a
├── include/
│   └── libcrc_fast.h
├── bin/
│   ├── checksum
│   ├── arch-check
│   └── get-custom-params
├── LICENSE-MIT
├── LICENSE-Apache
├── VERSION
└── README.txt
```

#### macOS Package (aarch64)

```
crc-fast-{version}-macos-aarch64/
├── lib/
│   ├── libcrc_fast.dylib
│   └── libcrc_fast.a
├── include/
│   └── libcrc_fast.h
├── bin/
│   ├── checksum
│   ├── arch-check
│   └── get-custom-params
├── LICENSE-MIT
├── LICENSE-Apache
├── VERSION
└── README.txt
```

#### Windows Packages (x86_64 and aarch64)

```
crc-fast-{version}-windows-{arch}/
├── bin/
│   ├── crc_fast.dll
│   ├── checksum.exe
│   ├── arch-check.exe
│   └── get-custom-params.exe
├── lib/
│   ├── crc_fast.dll.lib    (import library for linking against DLL)
│   └── crc_fast.lib         (static library)
├── include/
│   └── libcrc_fast.h
├── LICENSE-MIT
├── LICENSE-Apache
├── VERSION
└── README.txt
```

**Windows Library Files Explained**:
- `crc_fast.dll`: The dynamic library containing the actual code
- `crc_fast.dll.lib`: Import library needed to link against the DLL at compile time
- `crc_fast.lib`: Static library for static linking (standalone, doesn't need DLL)

### 3. Package Metadata Files

#### VERSION File
Contains the version number extracted from the git tag:
```
1.5.0
```

#### README.txt File
Provides basic installation and usage instructions:
```
crc-fast Binary Distribution
Version: {version}
Platform: {platform}-{arch}

Contents:
- Library files (dynamic and static)
- Header file for C/C++ integration
- Command-line utilities

Installation:
[Platform-specific instructions]

Usage:
See https://github.com/awesomized/crc-fast-rust for documentation

License: MIT OR Apache-2.0
```

### 4. Checksum Files

Each package has an accompanying `.sha256` file:
```
{sha256_hash}  crc-fast-{version}-{platform}-{arch}.{ext}
```

## Data Models

### Build Artifact Structure

```yaml
artifacts:
  - name: "crc-fast-{version}-{platform}-{arch}.{ext}"
    type: "package"
    checksum: "{sha256_hash}"
  - name: "crc-fast-{version}-{platform}-{arch}.{ext}.sha256"
    type: "checksum"
```

### Matrix Configuration

```yaml
matrix:
  include:
    - target: x86_64-unknown-linux-gnu
      os: ubuntu-22.04
      platform: linux
      arch: x86_64
      ext: tar.gz
      
    - target: aarch64-unknown-linux-gnu
      os: ubuntu-22.04-arm
      platform: linux
      arch: aarch64
      ext: tar.gz
      
    - target: aarch64-apple-darwin
      os: macos-14
      platform: macos
      arch: aarch64
      ext: tar.gz
      
    - target: x86_64-pc-windows-msvc
      os: windows-2022
      platform: windows
      arch: x86_64
      ext: zip
      
    - target: aarch64-pc-windows-msvc
      os: windows-11-arm
      platform: windows
      arch: aarch64
      ext: zip
```

## Error Handling

### Test Failure
- If the test workflow fails, the `validate` job fails
- The `build` jobs don't execute (dependency chain)
- No release is created
- GitHub Actions shows clear failure status

### Build Failure
- If any platform build fails, that specific matrix job fails
- Other platform builds continue
- The `publish` job waits for all builds
- If any build failed, `publish` job is skipped
- Partial releases are prevented

### Quality Check Failure
- If `cargo fmt --check` fails, build stops
- If `cargo clippy` produces warnings, build stops
- Ensures all released binaries meet quality standards

### Missing Files
- Build script validates all expected files exist before packaging
- Fails with clear error message if files are missing
- Prevents incomplete packages

### Upload Failure
- If GitHub release creation fails, workflow fails
- If asset upload fails, workflow retries (GitHub Actions default)
- All artifacts remain available for manual inspection

## Testing Strategy

### Pre-Release Testing
1. The existing test workflow must pass completely
2. All platforms in the test matrix must succeed
3. Format and clippy checks must pass

### Build Verification
1. Verify all expected files are present in target/release
2. Check file sizes are reasonable (not empty)
3. Validate binary executables are actually executable
4. Confirm library files have correct extensions

### Package Verification
1. Verify package structure matches specification
2. Check all metadata files are included
3. Validate checksums are generated correctly
4. Ensure package names follow naming convention

### Integration Testing
1. Test workflow on a feature branch with test tags
2. Verify packages can be downloaded
3. Test that libraries can be linked
4. Verify CLI binaries execute correctly
5. Validate checksums match package contents

### Manual Testing Checklist
Before first production release:
- [ ] Create test tag on feature branch
- [ ] Verify workflow triggers correctly
- [ ] Download each platform package
- [ ] Extract and verify contents
- [ ] Test linking dynamic library
- [ ] Test linking static library
- [ ] Run each CLI binary
- [ ] Verify checksums
- [ ] Test on actual target platforms

## Design Decisions

### Decision 1: Single Package Per Platform
**Rationale**: Combining libraries and CLI tools in one package simplifies distribution and ensures users get everything they need. Separate packages would create confusion about which package to download.

**Alternative Considered**: Separate library and CLI packages. Rejected because it adds complexity for minimal benefit.

### Decision 2: Include Both Dynamic and Static Libraries
**Rationale**: Different projects have different linking requirements. Providing both gives maximum flexibility without requiring users to build from source.

**Alternative Considered**: Separate packages for dynamic and static. Rejected because the size overhead is minimal and user experience is worse.

### Decision 3: Platform-Specific Directory Layouts
**Rationale**: Following platform conventions makes integration easier for users familiar with their platform's standards. Windows expects DLLs in bin/, while Unix expects shared libraries in lib/.

**Alternative Considered**: Unified layout across all platforms. Rejected because it would be non-standard on all platforms.

### Decision 4: Reuse Existing Test Workflow
**Rationale**: The existing test workflow is comprehensive and already validates all target platforms. Duplicating tests would be wasteful and create maintenance burden.

**Alternative Considered**: Inline tests in release workflow. Rejected because it duplicates logic and increases workflow complexity.

### Decision 5: Matrix Build Strategy
**Rationale**: Building all platforms in parallel maximizes speed and allows independent failure handling. GitHub Actions provides excellent matrix support.

**Alternative Considered**: Sequential builds. Rejected because it would significantly increase release time.

### Decision 6: SHA256 Checksums
**Rationale**: SHA256 is the industry standard for verifying download integrity. It's widely supported and provides strong security guarantees.

**Alternative Considered**: MD5 or SHA1. Rejected because they're cryptographically weak. Multiple checksums rejected as unnecessary.

### Decision 7: Separate Checksum Files
**Rationale**: Separate files allow users to download and verify checksums independently. This is the standard practice for software distribution.

**Alternative Considered**: Checksums in release notes. Rejected because it's less convenient for automated verification.

### Decision 8: Tag Pattern Without 'v' Prefix
**Rationale**: The project already uses tags like `1.5.0` without the 'v' prefix. Maintaining consistency with existing practice is important.

**Alternative Considered**: Requiring 'v' prefix. Rejected because it would break existing tagging convention.

### Decision 9: Update Existing Release Rather Than Create
**Rationale**: Allowing manual release creation before the workflow runs gives maintainers control over release notes, descriptions, and other metadata. The workflow focuses on building and uploading binaries, not managing release content.

**Workflow**: 
1. Maintainer creates release manually with desired notes
2. Maintainer creates and pushes tag
3. Workflow builds binaries and uploads to existing release
4. If no release exists, workflow creates a minimal one

**Alternative Considered**: Workflow always creates release. Rejected because it forces auto-generated release notes and removes maintainer control over release presentation.
