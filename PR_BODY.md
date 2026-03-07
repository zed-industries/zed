# Fix crash: Task polled after completion in RealFs::metadata

## Crash Summary

**Sentry Issue:** [ZED-4GH](https://sentry.io/organizations/zed-dev/issues/7199309438/)

The application crashes with "Task polled after completion" panic when the executor is closed while `RealFs::metadata` operations are still in progress. This typically occurs during application shutdown while the background scanner is still scanning directories. The crash has been observed 223 times on Linux systems running the stable channel.

## Root Cause

`RealFs::metadata()` spawns background tasks using `self.executor.spawn()` and awaits them directly without handling the case where the executor might be closed. When the executor is closed (e.g., during application shutdown), awaiting a cancelled task causes the async-task crate to panic with "Task polled after completion".

The call chain leading to the crash:
1. `BackgroundScanner::scan_dir()` calls `self.fs.metadata(&child_abs_path).await`
2. `RealFs::metadata()` spawns background tasks for `symlink_metadata`, `metadata`, and `is_executable`
3. When the executor closes, these tasks are cancelled
4. Awaiting the cancelled tasks triggers the panic

## Fix

Use the `fallible()` method on spawned tasks to convert `Task<T>` to `FallibleTask<T>`. A `FallibleTask<T>` returns `Option<T>` instead of panicking when the task is cancelled:

```rust
// Before (panics if executor is closed):
let result = self.executor.spawn(async move { ... }).await;

// After (returns None if executor is closed):
let result = self.executor.spawn(async move { ... }).fallible().await;
```

When `fallible().await` returns `None`, we return an error indicating the task was cancelled. This error propagates up to `scan_dir` which already handles errors gracefully by logging and continuing.

## Validation

- Code compiles successfully: `cargo build -p fs`
- Clippy passes: `./script/clippy -p fs`
- Added regression test: `test_realfs_metadata_after_executor_close`
  - Note: Test cannot run in this environment due to missing system libraries (xcb, xkbcommon), but it is verified to compile correctly

## Potentially Related Issues

### High Confidence
- None found - this appears to be a novel crash pattern

### Medium Confidence
- [#39191](https://github.com/zed-industries/zed/pull/39191) - Similar crash in BackgroundScanner during worktree operations (different code path)

### Low Confidence
- [#8528](https://github.com/zed-industries/zed/issues/8528) - SIGABRT crash, but different cause (large file handling)

## Reviewer Checklist

- [ ] Verify the `fallible()` pattern is consistent with other uses in the codebase
- [ ] Confirm the error messages are appropriate for debugging
- [ ] Review if similar patterns exist in other `RealFs` methods that should also be fixed
- [ ] Validate test execution in CI environment

---

Release Notes:

- Fixed a crash that could occur when closing Zed while directory scanning was in progress on Linux
