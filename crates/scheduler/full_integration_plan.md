# GPUI Scheduler Integration Plan

## Goal

Unify GPUI's async execution with the scheduler crate, eliminating duplicate blocking/scheduling logic and enabling deterministic testing.

## Current Status: Review Complete, Remaining Work Identified

The integration is mostly complete. Code review identified one regression that must be fixed before merging.

---

## 🚨 Remaining Work

### Phase 6: Restore Realtime Priority Support

**Problem**: The old implementation had special handling for `Priority::Realtime(_)` which spawned tasks on dedicated OS threads with elevated priority (critical for audio processing). This was removed during the integration.

Old code in `spawn_internal`:
```rust
if let Priority::Realtime(realtime) = priority {
    dispatcher.spawn_realtime(realtime, Box::new(move || {
        while let Ok(runnable) = rx.recv() {
            runnable.run();
        }
    }));
    // ... create task that sends to channel
}
```

Current state: If `Priority::Realtime` is passed to `schedule_background_with_priority`, it flows through to `dispatcher.dispatch()` which hits `unreachable!()`:

```rust
// In mac/dispatcher.rs, linux/dispatcher.rs, windows/dispatcher.rs:
let queue_priority = match priority {
    Priority::Realtime(_) => unreachable!(),  // PANIC!
    Priority::High => ...,
    ...
};
```

**Solution options**:

1. **Handle in PlatformScheduler** (recommended): Add Realtime handling to `PlatformScheduler::schedule_background_with_priority`:
   ```rust
   fn schedule_background_with_priority(&self, runnable: Runnable<RunnableMeta>, priority: Priority) {
       if let Priority::Realtime(realtime) = priority {
           // Spawn dedicated thread, set up channel, etc.
           self.dispatcher.spawn_realtime(realtime, ...);
       } else {
           self.dispatcher.dispatch(runnable, priority);
       }
   }
   ```

2. **Handle in GPUI executor**: Keep Realtime handling in `gpui::BackgroundExecutor::spawn_with_priority` before delegating to scheduler.

3. **Panic with helpful message**: If we decide Realtime should be accessed differently, at minimum replace `unreachable!()` with a helpful panic message.

**Note**: Currently no code in the codebase spawns with `Priority::Realtime`, so this is not causing test failures. However, the capability must be preserved for audio workloads.

### Phase 7: Delete Dead Code

The file `crates/gpui/src/platform/platform_scheduler.rs` is dead code - it's an older version of `PlatformScheduler` with a different `block()` signature that is no longer compatible with the `Scheduler` trait. The actual implementation used is at `crates/gpui/src/platform_scheduler.rs` (referenced via `mod platform_scheduler` in `gpui.rs`).

**Action**: Delete `crates/gpui/src/platform/platform_scheduler.rs`

---

## ✅ Completed Work

### Phase 1: Scheduler Trait and TestScheduler

The scheduler crate provides:
- `Scheduler` trait with `block()`, `schedule_foreground()`, `schedule_background_with_priority()`, `timer()`, `clock()`
- `TestScheduler` implementation for deterministic testing
- `ForegroundExecutor` and `BackgroundExecutor` that wrap `Arc<dyn Scheduler>`
- `Task<T>` type with `ready()`, `is_ready()`, `detach()`, `from_async_task()`

### Phase 2: PlatformScheduler

Created `PlatformScheduler` in GPUI (`crates/gpui/src/platform_scheduler.rs`) that:
- Implements `Scheduler` trait for production use
- Wraps `PlatformDispatcher` (Mac, Linux, Windows)
- Uses `parking::Parker` for blocking operations
- Uses `dispatch_after` for timers
- Provides a `PlatformClock` that delegates to the dispatcher

### Phase 3: Unified GPUI Executors

GPUI's executors (`crates/gpui/src/executor.rs`) now:
- `gpui::ForegroundExecutor` wraps `scheduler::ForegroundExecutor` internally
- `gpui::BackgroundExecutor` holds `Arc<dyn Scheduler>` directly (creates temporary `scheduler::BackgroundExecutor` when spawning)
- Select `TestScheduler` or `PlatformScheduler` based on dispatcher type (no optional fields)
- Wrap `scheduler::Task<T>` in a thin `gpui::Task<T>` that adds `detach_and_log_err()`
- Use `Scheduler::block()` for all blocking operations

### Phase 4: Removed Duplicate Logic

Eliminated from GPUI:
- Custom blocking loop implementations (now delegated to scheduler)
- Separate test/production code paths for spawn/block operations
- `TaskLabel` and deprioritization infrastructure (see Intentional Removals below)
- `unparker` mechanism (no longer needed - scheduler handles task coordination)

### Phase 5: Simplify block_internal and Remove Unparkers

Final cleanup:
- Removed debug logging infrastructure from executor.rs
- Simplified block_internal to use waker_fn without Parker
- Removed unparkers mechanism from TestDispatcher
- TestDispatcher now just holds session_id and scheduler (~70 lines)

---

## Intentional Removals

### `spawn_labeled` and `deprioritize` Removed

**What was removed**:
- `BackgroundExecutor::spawn_labeled(label: TaskLabel, future)` - spawn with a label for test control
- `BackgroundExecutor::deprioritize(label: TaskLabel)` - deprioritize labeled tasks in tests
- `TaskLabel` type

**Why**: These were only used in a few places for test ordering control. The new priority-weighted scheduling in `TestScheduler` provides similar functionality through `Priority::High/Medium/Low`.

**Migration**: Use `spawn()` instead of `spawn_labeled()`. For test ordering, use explicit synchronization (channels, etc.) or priority levels.

**Approval**: @as-cii reviewed and approved this removal.

### `start_waiting` / `finish_waiting` Debug Methods Removed

**What was removed**:
- `BackgroundExecutor::start_waiting()` - mark that code is waiting (for debugging)
- `BackgroundExecutor::finish_waiting()` - clear the waiting marker
- Associated `waiting_backtrace` tracking in TestDispatcher

**Why**: The new `TracingWaker` in `TestScheduler` provides better debugging capability. Run tests with `PENDING_TRACES=1` to see backtraces of all pending futures when parking is forbidden.

---

## Code Quality Notes

### Lock Ordering Inconsistency (Low Priority)

In `TestScheduler`, there's inconsistent lock ordering between `rng` and `state` mutexes:

- `block()` line 375-377: locks `rng` first, then `state`
- `schedule_foreground()` line 428-430: locks `state` first, then `rng`

This could theoretically cause deadlocks with concurrent access, but `TestScheduler` is single-threaded so it's not a practical concern. Worth fixing for code hygiene but not blocking.

### `dispatch_after` Panics in TestDispatcher

`TestDispatcher::dispatch_after` intentionally panics:
```rust
fn dispatch_after(&self, _duration: Duration, _runnable: RunnableVariant) {
    panic!(
        "dispatch_after should not be called in tests. \
        Use BackgroundExecutor::timer() which uses the scheduler's native timer."
    );
}
```

This is correct because:
- In tests, `TestScheduler` is used (not `PlatformScheduler`)
- `TestScheduler::timer()` creates native timers without using `dispatch_after`
- Any code hitting this panic has a bug (should use `executor.timer()`)

---

## Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                            GPUI                                   │
│                                                                   │
│  ┌────────────────────────┐    ┌──────────────────────────────┐  │
│  │  gpui::Background-     │    │  gpui::ForegroundExecutor    │  │
│  │  Executor              │    │   - inner: scheduler::       │  │
│  │   - scheduler: Arc<    │    │           ForegroundExecutor │  │
│  │       dyn Scheduler>   │    │   - dispatcher: Arc          │  │
│  │   - dispatcher: Arc    │    └──────────────┬───────────────┘  │
│  └───────────┬────────────┘                   │                   │
│              │                                │                   │
│              │  (creates temporary            │ (wraps)           │
│              │   scheduler::Background-       │                   │
│              │   Executor when spawning)      │                   │
│              │                                │                   │
│              │    ┌───────────────────────────┘                   │
│              │    │                                               │
│              ▼    ▼                                               │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │                    Arc<dyn Scheduler>                        │ │
│  └──────────────────────────┬──────────────────────────────────┘ │
│                             │                                     │
│            ┌────────────────┴────────────────┐                   │
│            │                                 │                    │
│            ▼                                 ▼                    │
│  ┌───────────────────────┐     ┌───────────────────────────┐    │
│  │   PlatformScheduler   │     │      TestScheduler        │    │
│  │   (production)        │     │   (deterministic tests)   │    │
│  └───────────────────────┘     └───────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

---

## Design Decisions

### Foreground Priority Not Supported

`ForegroundExecutor::spawn_with_priority` accepts a priority parameter but ignores it. This is acceptable because:
- macOS (primary platform) ignores foreground priority anyway
- TestScheduler runs foreground tasks in order
- There are no external callers of this method in the codebase
- Linux/Windows could theoretically use it, but the benefit is minimal

The method is kept for API compatibility but documents that priority is ignored.

### Profiler Integration Unchanged

The profiler task timing infrastructure continues to work because:
- `PlatformScheduler::schedule_background_with_priority` calls `dispatcher.dispatch()`
- `PlatformScheduler::schedule_foreground` calls `dispatcher.dispatch_on_main_thread()`
- All platform dispatchers (Mac, Linux, Windows) wrap task execution with profiler timing
- Mac writes to `THREAD_TIMINGS` directly in its trampoline; Linux/Windows call `profiler::add_task_timing()`

### Session IDs for Foreground Isolation

Each `ForegroundExecutor` gets a `SessionId` to prevent reentrancy when blocking. This ensures that when blocking on a future, we don't run foreground tasks from the same session (which could cause issues with re-entrancy).

### Runtime Scheduler Selection

In test builds, we check `dispatcher.as_test()` to choose between `TestScheduler` and `PlatformScheduler`. This allows the same executor types to work in both test and production environments.

---

## Key Design Principles

1. **No optional fields**: Both test and production paths use the same executor types with different `Scheduler` implementations underneath.

2. **Scheduler owns blocking logic**: The `Scheduler::block()` method handles all blocking, including timeout and task stepping (for tests).

3. **GPUI Task wrapper**: Thin wrapper around `scheduler::Task` that adds `detach_and_log_err()` which requires `&App`.

---

## Test Helpers

Test-only methods on `BackgroundExecutor`:
- `block_test()` - for running async tests synchronously
- `advance_clock()` - move simulated time forward
- `tick()` - run one task
- `run_until_parked()` - run all ready tasks
- `allow_parking()` / `forbid_parking()` - control parking behavior
- `simulate_random_delay()` - yield randomly for fuzzing
- `rng()` - access seeded RNG
- `set_block_on_ticks()` - configure timeout tick range for block operations

---

## Files Changed

- `crates/scheduler/src/scheduler.rs` - Updated `Scheduler::block()` signature to take `Pin<&mut dyn Future>` and return `bool`
- `crates/scheduler/src/executor.rs` - Added `from_async_task()`, `use<Fut>` on `block_with_timeout`
- `crates/scheduler/src/test_scheduler.rs` - Updated `block()` implementation
- `crates/scheduler/src/tests.rs` - Fixed `block_with_timeout` test assertions
- `crates/gpui/src/executor.rs` - Rewritten to use scheduler executors
- `crates/gpui/src/platform_scheduler.rs` - New file implementing `Scheduler` for production
- `crates/gpui/src/platform/platform_scheduler.rs` - **DEAD CODE, should be deleted** (older incompatible version)
- `crates/gpui/src/gpui.rs` - Added `platform_scheduler` module
- `crates/gpui/src/profiler.rs` - Added `#[allow(dead_code)]` to `add_task_timing`
- `crates/gpui/Cargo.toml` - Added `chrono` dependency
- `crates/gpui/src/platform/test/dispatcher.rs` - Simplified to ~70 lines, delegates to TestScheduler
- `crates/gpui/src/platform.rs` - Simplified `RunnableVariant` to type alias, removed `TaskLabel` from dispatch
- `crates/gpui/src/platform/mac/dispatcher.rs` - Removed `RunnableVariant::Compat` handling
- `crates/gpui/src/platform/linux/dispatcher.rs` - Removed label parameter from dispatch
- `crates/gpui/src/platform/windows/dispatcher.rs` - Removed label parameter from dispatch
- `crates/miniprofiler_ui/src/miniprofiler_ui.rs` - Changed `.dispatcher` to `.dispatcher()`
- `crates/repl/src/repl.rs` - Changed `.dispatcher` to `.dispatcher()`, wrap runnables with metadata
- `crates/zed/src/reliability.rs` - Changed `.dispatcher` to `.dispatcher()`
- `crates/buffer_diff/src/buffer_diff.rs` - Use `spawn()` instead of `spawn_labeled()`
- `crates/fs/src/fake_git_repo.rs` - Use `spawn()` instead of `spawn_labeled()`
- `crates/language/src/buffer.rs` - Use `spawn()` instead of `spawn_labeled()`

---

## Tests Status

✅ All GPUI tests pass (including Mac platform tests)
✅ All scheduler tests pass
✅ All three originally failing editor tests pass:
  - `test_soft_wrap_editor_width_auto_height_editor`
  - `test_no_hint_updates_for_unrelated_language_files`
  - `test_inside_char_boundary_range_hints`
✅ Clippy passes with no warnings
✅ No unused dependencies (cargo-machete passes)

---

## Future Considerations

### Potential Improvements

1. **Foreground priority support**: If needed in the future, add `schedule_foreground_with_priority` to the `Scheduler` trait and plumb it through to platforms that support it (Linux, Windows).

2. **Profiler integration in scheduler**: Could move task timing into the scheduler crate itself for more consistent profiling across all code paths.

3. **Additional test utilities**: The `TestScheduler` could be extended with more debugging/introspection capabilities.

4. **Fix lock ordering**: Clean up the `rng`/`state` lock ordering inconsistency in `TestScheduler` for better code hygiene.