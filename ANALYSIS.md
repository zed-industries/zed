# Crash Analysis: Task polled after completion in RealFs::metadata

## Crash Summary
- **Sentry Issue:** ZED-4GH (https://sentry.io/organizations/zed-dev/issues/7199309438/)
- **Error:** `expect()` on `None` with message "Task polled after completion"
- **Crash Site:** `async_task::task::Task<T>::poll` triggered from `fs::RealFs::metadata`
- **First Seen:** 2026-01-18T16:20:55Z
- **Last Seen:** 2026-02-16T19:51:52Z
- **Event Count:** 223
- **Platform:** Linux (Ubuntu)
- **Channel:** stable

## Root Cause

The crash occurs when `RealFs::metadata()` is awaiting spawned tasks on an executor that has been closed (during application shutdown). The call chain is:

1. `BackgroundScanner::scan_dir()` calls `self.fs.metadata(&child_abs_path).await`
2. `RealFs::metadata()` spawns multiple background tasks using `self.executor.spawn()`
3. When the executor is closed during shutdown, these spawned tasks are cancelled
4. Awaiting a cancelled task using the standard `Task<T>` triggers a panic with "Task polled after completion"

The async-task crate's `Task<T>` implementation uses `expect("Task polled after completion")` which panics when a cancelled/completed task is polled. This happens because:

```rust
// In async_task::task::Task<T>::poll
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    match poll_task::<T>(self.ptr.as_ptr(), cx) {
        Poll::Ready(t) => Poll::Ready(t.expect("Task polled after completion")),  // <-- PANIC HERE
        Poll::Pending => Poll::Pending,
    }
}
```

The three places in `RealFs::metadata()` that spawn and await tasks without handling cancellation:
1. `std::fs::symlink_metadata` task (line ~884)
2. `std::fs::metadata` task for symlink targets (line ~903)  
3. `path_buf.is_executable()` task (line ~935)

## Reproduction

The crash can be triggered by:
1. Opening a project with many files in Zed
2. While the background scanner is scanning directories, close the application
3. The executor is closed while `scan_dir` tasks are still awaiting metadata operations
4. The spawned metadata tasks are cancelled but the awaiting code doesn't handle cancellation

Test command:
```
cargo test -p fs test_realfs_metadata_after_executor_close
```

## Suggested Fix

Use the `fallible()` method on spawned tasks to convert `Task<T>` to `FallibleTask<T>`. A `FallibleTask<T>` returns `Option<T>` instead of panicking when the task is cancelled:

```rust
// Before (panics if executor is closed):
let result = self.executor.spawn(async move { ... }).await;

// After (returns None if executor is closed):
let result = self.executor.spawn(async move { ... }).fallible().await;
```

When `fallible().await` returns `None`, we should return an error indicating the task was cancelled. This error will propagate up to `scan_dir` which already handles errors gracefully by logging and continuing:

```rust
Err(err) => {
    log::error!("error processing {:?}: {err:#}", child_abs_path.display());
    continue;
}
```

The fix should be applied to all three task spawns in `RealFs::metadata()`.
