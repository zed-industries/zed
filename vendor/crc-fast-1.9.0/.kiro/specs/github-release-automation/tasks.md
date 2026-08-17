# Implementation Plan

- [x] 1. Create GitHub Actions release workflow file
  - Create `.github/workflows/release.yml` with workflow structure
  - Configure workflow to trigger on tags matching `[0-9]+.[0-9]+.[0-9]+` pattern
  - Set up workflow permissions for creating releases and uploading assets
  - _Requirements: 1.1, 6.5_

- [x] 2. Implement validation job
  - [x] 2.1 Create validate job that waits for test workflow completion
    - Use `lewagon/wait-on-check-action` or similar to wait for test workflow to complete
    - Poll test workflow status until it completes (success or failure)
    - Fail the validate job if test workflow fails
    - Set reasonable timeout (e.g., 60 minutes) to prevent infinite waiting
    - _Requirements: 6.1, 6.3_

  - [x] 2.2 Add quality checks to validate job
    - Run `cargo fmt --check` to verify code formatting
    - Run `cargo clippy -- -D warnings` to catch linting issues
    - _Requirements: 6.4_

- [x] 3. Implement build matrix job
  - [x] 3.1 Define build matrix with all target platforms
    - Configure matrix with 5 targets: x86_64-linux, aarch64-linux, aarch64-macos, x86_64-windows, aarch64-windows
    - Map each target to appropriate runner: ubuntu-22.04, ubuntu-22.04-arm, macos-14, windows-2022, windows-11-arm
    - Include platform, arch, and extension variables in matrix
    - _Requirements: 1.2_

  - [x] 3.2 Set up Rust toolchain in build job
    - Use `actions-rust-lang/setup-rust-toolchain` action
    - Configure stable toolchain
    - _Requirements: 6.2_

  - [x] 3.3 Build release binaries
    - Run `cargo build --release` to build library and CLI binaries
    - Verify all expected files exist in target/release directory
    - _Requirements: 1.4, 2.1, 3.1_

- [x] 4. Create platform-specific package staging
  - [x] 4.1 Implement Linux package staging
    - Create directory structure: lib/, include/, bin/
    - Copy libcrc_fast.so and libcrc_fast.a to lib/
    - Copy libcrc_fast.h to include/
    - Copy checksum, arch-check, get-custom-params to bin/
    - Set executable permissions on binaries
    - _Requirements: 2.2, 3.2, 3.4, 4.1, 4.4_

  - [x] 4.2 Implement macOS package staging
    - Create directory structure: lib/, include/, bin/
    - Copy libcrc_fast.dylib and libcrc_fast.a to lib/
    - Copy libcrc_fast.h to include/
    - Copy checksum, arch-check, get-custom-params to bin/
    - Set executable permissions on binaries
    - _Requirements: 2.3, 3.2, 3.4, 4.2, 4.4_

  - [x] 4.3 Implement Windows package staging
    - Create directory structure: bin/, lib/, include/
    - Copy crc_fast.dll to bin/
    - Copy crc_fast.dll.lib and crc_fast.lib to lib/
    - Copy libcrc_fast.h to include/
    - Copy checksum.exe, arch-check.exe, get-custom-params.exe to bin/
    - _Requirements: 2.4, 3.3, 4.3, 4.5_

- [ ] 5. Add package metadata files
  - [x] 5.1 Create VERSION file
    - Extract version from git tag
    - Write version number to VERSION file in package root
    - _Requirements: 7.1_

  - [x] 5.2 Create README.txt file
    - Generate platform-specific installation instructions
    - Include basic usage information and link to documentation
    - Add to package root
    - _Requirements: 7.2_

  - [x] 5.3 Copy license files
    - Copy LICENSE-MIT and LICENSE-Apache to package root
    - _Requirements: 7.3_

- [x] 6. Create compressed packages
  - [x] 6.1 Implement tar.gz creation for Linux and macOS
    - Use tar command to create compressed archive
    - Name package: crc-fast-{version}-{platform}-{arch}.tar.gz
    - _Requirements: 5.2, 5.3, 5.5_

  - [x] 6.2 Implement zip creation for Windows
    - Use PowerShell Compress-Archive or 7zip
    - Name package: crc-fast-{version}-windows-{arch}.zip
    - _Requirements: 5.4, 5.5_

- [X] 7. Generate and upload checksums
  - [x] 7.1 Generate SHA256 checksums
    - Calculate SHA256 hash for each package
    - Create .sha256 file with hash and filename
    - _Requirements: 7.4_

  - [x] 7.2 Upload packages and checksums as artifacts
    - Use `actions/upload-artifact` to upload package files
    - Upload checksum files as separate artifacts
    - _Requirements: 7.5_

- [X] 8. Implement publish job
  - [x] 8.1 Download all build artifacts
    - Use `actions/download-artifact` to retrieve all packages and checksums
    - Organize files for upload
    - _Requirements: 1.5_

  - [x] 8.2 Check for existing release
    - Use GitHub API to check if release exists for the tag
    - Get release ID if it exists
    - _Requirements: Decision 9_

  - [x] 8.3 Create or update GitHub release
    - If release doesn't exist, create new release with basic information
    - If release exists, prepare to update it with assets
    - Use `softprops/action-gh-release` or similar action
    - _Requirements: 1.5_

  - [x] 8.4 Upload packages and checksums to release
    - Upload all .tar.gz and .zip packages as release assets
    - Upload all .sha256 checksum files as release assets
    - _Requirements: 1.5, 7.5_

- [ ]* 9. Add workflow documentation
  - [ ]* 9.1 Create workflow README
    - Document how to trigger releases
    - Explain manual release creation workflow
    - Document package naming conventions
    - _Requirements: All_

  - [ ]* 9.2 Add inline workflow comments
    - Comment complex workflow steps
    - Explain matrix configuration
    - Document platform-specific logic
    - _Requirements: All_

- [ ]* 10. Test release workflow
  - [ ]* 10.1 Test on feature branch
    - Create test tag on feature branch
    - Verify workflow triggers correctly
    - Check that all jobs complete successfully
    - _Requirements: All_

  - [ ]* 10.2 Validate package contents
    - Download each platform package
    - Extract and verify directory structure
    - Check that all files are present
    - Verify file permissions on Unix platforms
    - _Requirements: 2.1-2.5, 3.1-3.4, 4.1-4.5, 7.1-7.3_

  - [ ]* 10.3 Test package functionality
    - Test linking against dynamic library
    - Test linking against static library
    - Run each CLI binary to verify functionality
    - Verify checksums match package contents
    - _Requirements: 2.1-2.5, 3.1-3.4_
