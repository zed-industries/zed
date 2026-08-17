## 0.4.0

### Breaking Changes
* **Improved error handling** - All functions now return `io::Result` instead of `Result<T, i32>`
  - `fork()` now returns `io::Result<Fork>` (was `Result<Fork, i32>`)
  - `daemon()` now returns `io::Result<Fork>` (was `Result<Fork, i32>`)
  - `setsid()` now returns `io::Result<libc::pid_t>` (was `Result<libc::pid_t, i32>`)
  - `getpgrp()` now returns `io::Result<libc::pid_t>` (was `Result<libc::pid_t, i32>`)
  - `waitpid()` now returns `io::Result<()>` (was `Result<(), i32>`)
  - `chdir()` now returns `io::Result<()>` (was `Result<libc::c_int, i32>`)
  - `close_fd()` now returns `io::Result<()>` (was `Result<(), i32>`)

### Major Improvements
* **Fixed file descriptor reuse bug** (Issue #2)
  - Added `redirect_stdio()` function that redirects stdio to `/dev/null` instead of closing
  - Prevents silent file corruption when daemon opens files after stdio is closed
  - `daemon()` now uses `redirect_stdio()` instead of `close_fd()`
  - Matches industry standard implementations (libuv, systemd, BSD daemon(3))
  
### Benefits
* **Better error diagnostics** - Errors now capture and preserve `errno` values
* **Rich error messages** - Error display shows descriptive text (e.g., "Permission denied") instead of `-1`
* **Rust idioms** - Integrates seamlessly with `?` operator, `anyhow`, `thiserror`, and other error handling crates
* **Type safety** - Can match on `ErrorKind` variants for specific error handling
* **Debugging** - `.raw_os_error()` provides access to underlying errno when needed
* **Correctness** - No more file descriptor reuse bugs that could corrupt data files

### Added
* `Fork` enum now derives `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` for better usability
* `redirect_stdio()` function - Safer alternative to `close_fd()`
* Comprehensive tests for stdio redirection (`tests/stdio_redirect_tests.rs`)
  - Test demonstrating the fd reuse bug with `close_fd()`
  - Tests verifying `redirect_stdio()` prevents fd reuse
  - Tests confirming `daemon()` uses correct behavior

### Improved
* Simplified `close_fd()` implementation using iterator pattern
* Enhanced documentation with detailed error descriptions for all functions
* Updated all examples to use proper error handling patterns
* Added warnings to `close_fd()` documentation about fd reuse risks

### Security
* **CRITICAL FIX**: `daemon()` no longer vulnerable to file descriptor reuse bugs
  - Previously, files opened after `daemon(false, false)` could get fd 0, 1, or 2
  - Any `println!`, `eprintln!`, or panic would write to those files, corrupting them
  - Now stdio is redirected to `/dev/null`, keeping fd 0,1,2 occupied
  - New files always get fd >= 3

## 0.3.1
* Added comprehensive test coverage for `getpgrp()` function
  - Unit tests in `src/lib.rs` (`test_getpgrp`, `test_getpgrp_in_parent`)
  - Integration test `test_getpgrp_returns_process_group` in `tests/integration_tests.rs`
* Added `coverage` recipe to `.justfile` for generating coverage reports with grcov

## 0.3.0

### Changed
* Updated Rust edition from 2021 to 2024
* Applied edition 2024 formatting standards (alphabetical import ordering)

### Added
* **Integration tests directory** - Added `tests/` directory with comprehensive integration tests
  - `daemon_tests.rs` - 5 tests for daemon functionality (detached process, nochdir, process groups, command execution, no controlling terminal)
  - `fork_tests.rs` - 7 tests for fork functionality (basic fork, parent-child communication, multiple children, environment inheritance, command execution, different PIDs, waitpid)
  - `integration_tests.rs` - 5 tests for advanced patterns (double-fork daemon, setsid, chdir, process isolation, getpgrp)

### Improved
* Significantly expanded test coverage from 1 to 13 comprehensive unit tests
* Added tests for all public API functions:
  - `fork()` - Multiple test scenarios including child execution
  - `daemon()` - Daemon pattern tested (double-fork with setsid)
  - `waitpid()` - Proper parent-child synchronization
  - `setsid()` - Session management and verification
  - `getpgrp()` - Process group queries
  - `chdir()` - Directory changes with verification
  - `close_fd()` - File descriptor management
* Added real-world usage pattern tests:
  - Classic double-fork daemon pattern
  - Multiple sequential forks
  - Command execution in child processes
* Improved test quality with proper cleanup and zombie process prevention
* Enhanced CI/CD integration with LLVM coverage instrumentation
* **Total test count: 35 tests** (13 unit + 17 integration + 5 doc tests)

### Fixed
* Daemon tests now properly test the daemon pattern without calling `daemon()` directly
  (which would call `exit(0)` and terminate the test runner)

### Updated
* GitHub Actions: codecov/codecov-action from v4 to v5

## 0.2.0
* Added waitpid(pid: i32)
