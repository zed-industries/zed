# GPUI Scheduler Integration

This document describes the integration of GPUI's async execution with the scheduler crate, including architecture, design decisions, and lessons learned.

## Goal

Unify GPUI's async execution with the scheduler crate, eliminating duplicate blocking/scheduling logic and enabling deterministic testing.

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

### Scheduler Trait

The scheduler crate provides:
- `Scheduler` trait with `block()`, `schedule_foreground()`, `schedule_background_with_priority()`, `timer()`, `clock()`
- `TestScheduler` implementation for deterministic testing
- `ForegroundExecutor` and `BackgroundExecutor` that wrap `Arc<dyn Scheduler>`
- `Task<T>` type with `ready()`, `is_ready()`, `detach()`, `from_async_task()`

### PlatformScheduler

`PlatformScheduler` in GPUI (`crates/gpui/src/platform_scheduler.rs`):
- Implements `Scheduler` trait for production use
- Wraps `PlatformDispatcher` (Mac, Linux, Windows)
- Uses `parking::Parker` for blocking operations
- Uses `dispatch_after` for timers
- Provides a `PlatformClock` that delegates to the dispatcher

### GPUI Executors

GPUI's executors (`crates/gpui/src/executor.rs`):
- `gpui::ForegroundExecutor` wraps `scheduler::ForegroundExecutor` internally
- `gpui::BackgroundExecutor` holds `Arc<dyn Scheduler>` directly
- Select `TestScheduler` or `PlatformScheduler` based on dispatcher type
- Wrap `scheduler::Task<T>` in a thin `gpui::Task<T>` that adds `detach_and_log_err()`
- Use `Scheduler::block()` for all blocking operations

---

## Design Decisions

### Key Design Principles

1. **No optional fields**: Both test and production paths use the same executor types with different `Scheduler` implementations underneath.

2. **Scheduler owns blocking logic**: The `Scheduler::block()` method handles all blocking, including timeout and task stepping (for tests).

3. **GPUI Task wrapper**: Thin wrapper around `scheduler::Task` that adds `detach_and_log_err()` which requires `&App`.

### Foreground Priority Not Supported

`ForegroundExecutor::spawn_with_priority` accepts a priority parameter but ignores it. This is acceptable because:
- macOS (primary platform) ignores foreground priority anyway
- TestScheduler runs foreground tasks in order
- There are no external callers of this method in the codebase

### Session IDs for Foreground Isolation

Each `ForegroundExecutor` gets a `SessionId` to prevent reentrancy when blocking. This ensures that when blocking on a future, we don't run foreground tasks from the same session.

### Runtime Scheduler Selection

In test builds, we check `dispatcher.as_test()` to choose between `TestScheduler` and `PlatformScheduler`. This allows the same executor types to work in both test and production environments.

### Profiler Integration

The profiler task timing infrastructure continues to work because:
- `PlatformScheduler::schedule_background_with_priority` calls `dispatcher.dispatch()`
- `PlatformScheduler::schedule_foreground` calls `dispatcher.dispatch_on_main_thread()`
- All platform dispatchers wrap task execution with profiler timing

---

## Intentional Removals

### `spawn_labeled` and `deprioritize`

**What was removed**:
- `BackgroundExecutor::spawn_labeled(label: TaskLabel, future)`
- `BackgroundExecutor::deprioritize(label: TaskLabel)`
- `TaskLabel` type

**Why**: These were only used in a few places for test ordering control. The new priority-weighted scheduling in `TestScheduler` provides similar functionality through `Priority::High/Medium/Low`.

**Migration**: Use `spawn()` instead of `spawn_labeled()`. For test ordering, use explicit synchronization (channels, etc.) or priority levels.

### `start_waiting` / `finish_waiting` Debug Methods

**What was removed**:
- `BackgroundExecutor::start_waiting()`
- `BackgroundExecutor::finish_waiting()`
- Associated `waiting_backtrace` tracking in TestDispatcher

**Why**: The new `TracingWaker` in `TestScheduler` provides better debugging capability. Run tests with `PENDING_TRACES=1` to see backtraces of all pending futures when parking is forbidden.

### Realtime Priority

**What was removed**: `Priority::Realtime` variant and associated OS thread spawning.

**Why**: There were no in-tree call sites using realtime priority. The correctness/backpressure semantics are non-trivial:
- Blocking enqueue risks stalling latency-sensitive threads
- Non-blocking enqueue implies dropping runnables under load, which breaks correctness for general futures

Rather than ship ambiguous or risky semantics, we removed the API until there is a concrete in-tree use case.

---

## Lessons Learned

These lessons were discovered during integration testing and represent important design constraints.

### 1. Never Cache `Entity<T>` in Process-Wide Statics

**Problem**: `gpui::Entity<T>` is a handle tied to a particular `App`'s entity-map. Storing an `Entity<T>` in a process-wide static (`OnceLock`, `LazyLock`, etc.) and reusing it across different `App` instances causes:
- "used a entity with the wrong context" panics
- `Option::unwrap()` failures in leak-detection clone paths
- Nondeterministic behavior depending on test ordering

**Solution**: Cache plain data (env var name, URL, etc.) in statics, and create `Entity<T>` per-`App`.

**Guideline**: Never store `gpui::Entity<T>` or other `App`-context-bound handles in process-wide statics unless explicitly keyed by `App` identity.

### 2. `block_with_timeout` Behavior Depends on Tick Budget

**Problem**: In `TestScheduler`, "timeout" behavior depends on an internal tick budget (`timeout_ticks`), not just elapsed wall-clock time. During the allotted ticks, the scheduler can poll futures and step other tasks.

**Implications**:
- A future can complete "within a timeout" in tests due to scheduler progress, even without explicit `advance_clock()`
- Yielding does not advance time
- If a test needs time to advance, it must do so explicitly via `advance_clock()`

**For deterministic timeout tests**: Set `scheduler.set_timeout_ticks(0..=0)` to prevent any scheduler stepping during timeout, then explicitly advance time.

### 3. Realtime Priority Must Panic in Tests

**Problem**: `Priority::Realtime` spawns dedicated OS threads outside the test scheduler, which breaks determinism and causes hangs/flakes.

**Solution**: The test dispatcher's `spawn_realtime` implementation panics with a clear message. This is an enforced invariant, not an implementation detail.

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

## Code Quality Notes

### `dispatch_after` Panics in TestDispatcher

This is intentional:
```rust
fn dispatch_after(&self, _duration: Duration, _runnable: RunnableVariant) {
    panic!(
        "dispatch_after should not be called in tests. \
        Use BackgroundExecutor::timer() which uses the scheduler's native timer."
    );
}
```

In tests, `TestScheduler::timer()` creates native timers without using `dispatch_after`. Any code hitting this panic has a bug.

---

## Files Changed

Key files modified during integration:

- `crates/scheduler/src/scheduler.rs` - `Scheduler::block()` signature takes `Pin<&mut dyn Future>` and returns `bool`
- `crates/scheduler/src/executor.rs` - Added `from_async_task()`
- `crates/scheduler/src/test_scheduler.rs` - Deterministic scheduling implementation
- `crates/gpui/src/executor.rs` - Rewritten to use scheduler executors
- `crates/gpui/src/platform_scheduler.rs` - New file implementing `Scheduler` for production
- `crates/gpui/src/platform/test/dispatcher.rs` - Simplified to delegate to TestScheduler
- `crates/gpui/src/platform.rs` - Simplified `RunnableVariant`, removed `TaskLabel`
- Platform dispatchers (mac/linux/windows) - Removed label parameter from dispatch

---

## Future Considerations

1. **Foreground priority support**: If needed, add `schedule_foreground_with_priority` to the `Scheduler` trait.

2. **Profiler integration in scheduler**: Could move task timing into the scheduler crate for more consistent profiling.

3. **Additional test utilities**: The `TestScheduler` could be extended with more debugging/introspection capabilities.