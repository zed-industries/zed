# Integration Tests

This directory contains integration tests for the `fork` library. These tests run in separate processes and can properly test functions like `daemon()` that call `exit()`.

## Overview

The integration tests are organized into four files:
- **`daemon_tests.rs`** - 5 tests for daemon functionality
- **`fork_tests.rs`** - 7 tests for fork/waitpid functionality  
- **`integration_tests.rs`** - 5 tests for advanced patterns
- **`common/mod.rs`** - Shared test utilities

**Total: 17 integration tests** providing comprehensive coverage of process management, daemon creation, and fork patterns.

## Test Files

### `daemon_tests.rs` - Daemon Functionality Tests (5 tests)

Tests the `daemon()` function with real process daemonization. Each test documents:
- What is being tested
- Expected behavior (numbered steps)
- Why it matters for daemon creation

Tests include:
- **test_daemon_creates_detached_process** - Verifies daemon process creation and PID management
- **test_daemon_with_nochdir** - Tests `nochdir` option preserves current directory
- **test_daemon_process_group** - Verifies daemon process group structure (double-fork pattern)
- **test_daemon_with_command_execution** - Tests command execution in daemon context
- **test_daemon_no_controlling_terminal** - Verifies daemon has no controlling terminal

### `fork_tests.rs` - Fork Functionality Tests (7 tests)

Tests the core `fork()` and `waitpid()` functions. Each test explains the expected parent-child behavior.

Tests include:
- **test_fork_basic** - Basic fork/waitpid functionality and cleanup
- **test_fork_parent_child_communication** - File-based parent-child IPC pattern
- **test_fork_multiple_children** - Creating and managing multiple child processes
- **test_fork_child_inherits_environment** - Environment variable inheritance across fork
- **test_fork_child_can_execute_commands** - Command execution in child processes
- **test_fork_child_has_different_pid** - PID uniqueness between parent and child
- **test_waitpid_waits_for_child** - Proper parent-child synchronization

### `integration_tests.rs` - Advanced Pattern Tests (5 tests)

Tests complex usage patterns combining multiple operations. Documents real-world daemon scenarios.

Tests include:
- **test_double_fork_daemon_pattern** - Classic double-fork daemon creation (standard pattern)
- **test_setsid_creates_new_session** - Session management and session leader verification
- **test_chdir_changes_directory** - Directory changes in child processes
- **test_process_isolation** - File system isolation between parent/child (separate memory)
- **test_getpgrp_returns_process_group** - Process group queries and verification

### `common/mod.rs` - Shared Test Utilities

Provides reusable helper functions to reduce code duplication:
- `get_unique_test_dir()` - Creates unique test directories with atomic counter
- `get_test_dir()` - Creates simple test directories
- `setup_test_dir()` - Sets up and cleans test directory
- `wait_for_file()` - Waits for file creation with timeout
- `cleanup_test_dir()` - Removes test directory

## Running Tests

```bash
# Run all tests (unit + integration + doc)
cargo test

# Run only integration tests
cargo test --tests

# Run specific test file
cargo test --test daemon_tests
cargo test --test fork_tests
cargo test --test integration_tests

# Run specific test
cargo test --test daemon_tests test_daemon_creates_detached_process

# Run with output
cargo test --test daemon_tests -- --nocapture

# Run with verbose output
cargo test --test fork_tests -- --nocapture --test-threads=1
```

## How Integration Tests Work

Unlike unit tests in `src/lib.rs`, integration tests:

1. **Run in separate processes** - Each test file is compiled as its own binary
2. **Can call `daemon()`** - The parent process in tests doesn't terminate the test runner
3. **Use file-based communication** - Temporary files in `/tmp` for parent-child verification
4. **Have proper isolation** - Each test uses unique temporary directories to avoid conflicts
5. **Clean up after themselves** - Temporary files are removed after test completion
6. **Document expected behavior** - Each test has detailed comments explaining what happens

## Test Documentation

Every test includes comprehensive documentation:

```rust
#[test]
fn test_name() {
    // Clear description of what is being tested
    // Expected behavior:
    // 1. First step
    // 2. Second step
    // 3. Third step
    // 4. Fourth step
    // 5. Final verification
    
    [test implementation]
}
```

This makes it easy to:
- Understand test purpose at a glance
- Debug failures quickly
- Use tests as usage examples
- Onboard new contributors

## Test Isolation

Each test uses a unique temporary directory to prevent conflicts when running in parallel:

```rust
// Daemon tests use atomic counter for uniqueness
let test_dir = setup_test_dir(get_unique_test_dir("daemon_creates_detached"));

// Fork tests use descriptive prefixes
let test_dir = setup_test_dir(get_test_dir("fork_communication"));

// Integration tests use specific names
let test_dir = setup_test_dir(get_test_dir("int_double_fork"));
```

This allows tests to run in parallel without interfering with each other.

## Coverage

Integration tests provide coverage for:

- **Daemon creation** - Real process daemonization (not mocked)
- **Process groups** - Session management and process group creation
- **File descriptors** - Proper handling of stdin/stdout/stderr
- **IPC patterns** - Parent-child communication via files
- **Command execution** - Running commands in forked/daemon processes
- **Environment inheritance** - Variable passing across fork
- **Process isolation** - Memory separation and filesystem sharing
- **Double-fork pattern** - Standard daemon creation technique
- **PID management** - Process ID tracking and verification

These tests complement the 13 unit tests in `src/lib.rs` and 5 doc tests to provide **comprehensive coverage** (35 total tests) of the library's functionality.

## Module Structure

```
tests/
├── common/
│   └── mod.rs          # Shared utilities (51 lines)
├── daemon_tests.rs     # Daemon tests (260 lines, 5 tests)
├── fork_tests.rs       # Fork tests (290 lines, 7 tests)
├── integration_tests.rs # Advanced tests (226 lines, 5 tests)
└── README.md           # This file
```

Total: **827 lines** of well-documented integration test code with **~160 lines** of explanatory comments.

