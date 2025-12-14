# GPUI Scheduler Integration Plan

## Goal

Unify GPUI's async execution with the scheduler crate, eliminating duplicate blocking/scheduling logic and enabling deterministic testing.

## ✅ Integration Complete

All phases have been completed. GPUI now uses the scheduler crate for all async execution.

## Completed Work

### ✅ Phase 1: Scheduler Trait and TestScheduler

The scheduler crate provides:
- `Scheduler` trait with `block()`, `schedule_foreground()`, `schedule_background_with_priority()`, `timer()`, `clock()`
- `TestScheduler` implementation for deterministic testing
- `ForegroundExecutor` and `BackgroundExecutor` that wrap `Arc<dyn Scheduler>`
- `Task<T>` type with `ready()`, `is_ready()`, `detach()`, `from_async_task()`

### ✅ Phase 2: PlatformScheduler

Created `PlatformScheduler` in GPUI (`crates/gpui/src/platform_scheduler.rs`) that:
- Implements `Scheduler` trait for production use
- Wraps `PlatformDispatcher` (Mac, Linux, Windows)
- Uses `parking::Parker` for blocking operations
- Uses `dispatch_after` for timers
- Provides a `PlatformClock` that delegates to the dispatcher

### ✅ Phase 3: Unified GPUI Executors

GPUI's executors (`crates/gpui/src/executor.rs`) now:
- Always use `scheduler::BackgroundExecutor` and `scheduler::ForegroundExecutor` internally
- Select `TestScheduler` or `PlatformScheduler` based on dispatcher type (no optional fields)
- Wrap `scheduler::Task<T>` in a thin `gpui::Task<T>` that adds `detach_and_log_err()`
- Use `Scheduler::block()` for all blocking operations

### ✅ Phase 4: Removed Duplicate Logic

Eliminated from GPUI:
- Custom blocking loop implementations (now delegated to scheduler)
- Separate test/production code paths for spawn/block operations
- `TaskLabel` and deprioritization infrastructure
- `unparker` mechanism (no longer needed - scheduler handles task coordination)

### ✅ Phase 5: Simplify block_internal and Remove Unparkers

Final cleanup:
- Removed debug logging infrastructure from executor.rs
- Simplified block_internal to use waker_fn without Parker
- Removed unparkers mechanism from TestDispatcher
- TestDispatcher now just holds session_id and scheduler (~70 lines)

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                          GPUI                                │
│  ┌─────────────────────┐   ┌─────────────────────────────┐  │
│  │ BackgroundExecutor  │   │    ForegroundExecutor       │  │
│  │  - scheduler: Arc   │   │  - inner: scheduler::Fg     │  │
│  │  - dispatcher: Arc  │   │  - dispatcher: Arc          │  │
│  └─────────┬───────────┘   └─────────────┬───────────────┘  │
│            │                             │                   │
│  ┌─────────▼─────────────────────────────▼───────────────┐  │
│  │              scheduler::*Executor                      │  │
│  │         (spawn, block_on, block_with_timeout)          │  │
│  └─────────────────────────┬─────────────────────────────┘  │
│                            │                                 │
│  ┌─────────────────────────▼─────────────────────────────┐  │
│  │                  Arc<dyn Scheduler>                    │  │
│  └───────────┬───────────────────────────┬───────────────┘  │
│              │                           │                   │
│  ┌───────────▼───────────┐   ┌───────────▼───────────────┐  │
│  │   PlatformScheduler   │   │      TestScheduler        │  │
│  │  (production)         │   │   (deterministic tests)   │  │
│  └───────────────────────┘   └───────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Design Decisions

### Foreground Priority Not Supported

`ForegroundExecutor::spawn_with_priority` accepts a priority parameter but ignores it. This is acceptable because:
- macOS (primary platform) ignores foreground priority anyway
- TestScheduler runs foreground tasks in order
- There are no external callers of this method in the codebase
- Linux/Windows could theoretically use it, but the benefit is minimal

The method is kept for API compatibility but documents that priority is ignored.

### Profiler Integration Unchanged

The profiler task timing infrastructure (`add_task_timing`) continues to work because:
- `PlatformScheduler::schedule_background_with_priority` calls `dispatcher.dispatch()`
- `PlatformScheduler::schedule_foreground` calls `dispatcher.dispatch_on_main_thread()`
- The platform dispatchers (Linux, Windows) wrap task execution with profiler timing
- macOS doesn't use task-level profiling (uses Instruments instead)

### Session IDs for Foreground Isolation

Each `ForegroundExecutor` gets a `SessionId` to prevent reentrancy when blocking. This ensures that when blocking on a future, we don't run foreground tasks from the same session (which could cause issues with re-entrancy).

### Runtime Scheduler Selection

In test builds, we check `dispatcher.as_test()` to choose between `TestScheduler` and `PlatformScheduler`. This allows the same executor types to work in both test and production environments.

## Key Design Principles

1. **No optional fields**: Both test and production paths use the same executor types with different `Scheduler` implementations underneath.

2. **Scheduler owns blocking logic**: The `Scheduler::block()` method handles all blocking, including timeout and task stepping (for tests).

3. **GPUI Task wrapper**: Thin wrapper around `scheduler::Task` that adds `detach_and_log_err()` which requires `&App`.

## Test Helpers

Test-only methods on `BackgroundExecutor`:
- `block_test()` - for running async tests synchronously
- `advance_clock()` - move simulated time forward
- `tick()` - run one task
- `run_until_parked()` - run all ready tasks
- `allow_parking()` / `forbid_parking()` - control parking behavior
- `simulate_random_delay()` - yield randomly for fuzzing
- `rng()` - access seeded RNG

## Files Changed

- `crates/scheduler/src/scheduler.rs` - Updated `Scheduler::block()` signature to take `Pin<&mut dyn Future>` and return `bool`
- `crates/scheduler/src/executor.rs` - Added `from_async_task()`, `use<Fut>` on `block_with_timeout`
- `crates/scheduler/src/test_scheduler.rs` - Updated `block()` implementation
- `crates/scheduler/src/tests.rs` - Fixed `block_with_timeout` test assertions
- `crates/gpui/src/executor.rs` - Rewritten to use scheduler executors
- `crates/gpui/src/platform_scheduler.rs` - New file implementing `Scheduler` for production
- `crates/gpui/src/gpui.rs` - Added `platform_scheduler` module
- `crates/gpui/src/profiler.rs` - Added `#[allow(dead_code)]` to `add_task_timing`
- `crates/gpui/Cargo.toml` - Added `chrono` dependency
- `crates/gpui/src/platform/test/dispatcher.rs` - Simplified to ~70 lines, delegates to TestScheduler
- `crates/miniprofiler_ui/src/miniprofiler_ui.rs` - Changed `.dispatcher` to `.dispatcher()`
- `crates/repl/src/repl.rs` - Changed `.dispatcher` to `.dispatcher()`
- `crates/zed/src/reliability.rs` - Changed `.dispatcher` to `.dispatcher()`

## Tests Status

✅ All GPUI tests pass (including Mac platform tests)
✅ All scheduler tests pass
✅ All three originally failing editor tests pass:
  - `test_soft_wrap_editor_width_auto_height_editor`
  - `test_no_hint_updates_for_unrelated_language_files`
  - `test_inside_char_boundary_range_hints`
✅ Clippy passes with no warnings
✅ No unused dependencies (cargo-machete passes)

## Future Considerations

### Potential Improvements

1. **Foreground priority support**: If needed in the future, add `schedule_foreground_with_priority` to the `Scheduler` trait and plumb it through to platforms that support it (Linux, Windows).

2. **Profiler integration in scheduler**: Could move task timing into the scheduler crate itself for more consistent profiling across all code paths.

3. **Additional test utilities**: The `TestScheduler` could be extended with more debugging/introspection capabilities.