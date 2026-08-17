---
inclusion: always
---

# Code Quality Requirements

## Pre-Completion Checks

Before marking any task as completed, you MUST run the following commands and ensure they pass without errors or warnings:

### 1. Code Formatting
```bash
cargo fmt --check
```
If this fails, run `cargo fmt` to fix formatting issues, then verify with `--check` again.

### 2. Linting
```bash
cargo clippy -- -D warnings
```
All clippy warnings must be resolved. This ensures code follows Rust best practices and catches potential issues.

### 3. Testing
```bash
cargo test
```
All tests must pass to ensure no regressions were introduced.

## Why These Requirements Matter

- **cargo fmt**: Ensures consistent code formatting across the entire codebase, making it easier to read and maintain
- **cargo clippy**: Catches common mistakes, suggests idiomatic Rust patterns, and helps prevent bugs before they reach production
- **cargo test**: Validates that all functionality works as expected and no existing features were broken

## Failure Handling

If any of these commands fail:
1. Fix the issues identified
2. Re-run the commands to verify fixes
3. Only then mark the task as completed

These checks are non-negotiable for maintaining code quality and consistency.