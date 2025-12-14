# GPUI Scheduler Integration Plan

## Goal

Eliminate GPUI's `PlatformDispatcher` abstraction and move entirely to the `Scheduler` trait from the scheduler crate. This will:
- Remove the redundant `TestDispatcher` wrapper
- Unify task scheduling across GPUI and scheduler
- Simplify the codebase significantly

## Current State

### What's Been Completed

1. **Task Queue Delegation**: TestDispatcher delegates foreground/background task scheduling to TestScheduler
   - `dispatch()` → `scheduler.schedule_background_with_priority()`
   - `dispatch_on_main_thread()` → `scheduler.schedule_foreground(session_id)`

2. **Unified Priority Types**: GPUI reexports `Priority` and `RealtimePriority` from scheduler crate

3. **Unified RunnableMeta**: Both GPUI and scheduler use `scheduler::RunnableMeta`

4. **Removed Deprioritization**: Removed the `deprioritize()` feature and `TaskLabel` handling

5. **Timer Unification**: `BackgroundExecutor::timer()` uses `Scheduler::timer()` directly in tests

6. **`is_main_thread` Delegation**: TestDispatcher delegates to scheduler

7. **Clock Delegation**: `advance_clock` and `advance_clock_to_next_timer` delegate to scheduler

8. **Tick Delegation**: `tick()` delegates to `scheduler.tick()` / `scheduler.tick_background_only()`

9. **Random Delay Delegation**: `simulate_random_delay()` delegates to `scheduler.yield_random()`

10. **Simplified TestDispatcher**: Now ~130 lines, mostly just PlatformDispatcher trait impl

### What TestDispatcher Still Does

Located at `crates/gpui/src/platform/test/dispatcher.rs` (~130 lines):

1. **PlatformDispatcher trait impl** - Required interface for GPUI executors
2. **`unparkers`** - Thread synchronization for GPUI's `block_on` when parking is allowed
3. **`session_id`** - For foreground task tracking per dispatcher clone

### Key Files

**Scheduler Crate** (`crates/scheduler/`):
- `src/scheduler.rs` - `Scheduler` trait, `Priority`, `RealtimePriority`, `RunnableMeta`, `SessionId`
- `src/test_scheduler.rs` - `TestScheduler` implementation (timers, clock, rng, is_main_thread)

**GPUI Crate** (`crates/gpui/`):
- `src/executor.rs` - GPUI's `BackgroundExecutor`, `ForegroundExecutor`, `Task`
- `src/platform.rs` - `PlatformDispatcher` trait
- `src/platform/test/dispatcher.rs` - `TestDispatcher` (thin wrapper around TestScheduler)

---

## Completed Phases

### ✅ Phase 1: Create SharedRng Wrapper

Created `SharedRng` wrapper type that handles locking internally.

### ✅ Phase 3: Move simulate_random_delay to TestScheduler

`TestDispatcher::simulate_random_delay()` now delegates directly to `TestScheduler::yield_random()`.

### ✅ Phase 4: Remove TaskLabel

Removed the unused `TaskLabel` infrastructure:
- Removed `TaskLabel` struct and its impl from `gpui/src/executor.rs`
- Removed `spawn_labeled` method from `BackgroundExecutor`
- Updated callers to use regular `spawn`
- Removed `label` parameter from `PlatformDispatcher::dispatch()` trait method
- Updated all dispatcher implementations (Mac, Linux, Windows, Test, PlatformScheduler)

---

## Blocked Phase

### 🔶 Phase 2: Delegate block_internal to scheduler.block()

**Problem:** The scheduler's `blocked_sessions` mechanism prevents foreground tasks from the blocked session from running during `step_filtered()` calls. This breaks tests that call `run_until_parked()` inside an async test.

**Current workaround:** GPUI keeps its own `block_internal` implementation that uses `tick()` directly.

**Possible solutions to investigate:**
- Have the scheduler distinguish between "internal stepping during block()" vs "user-initiated tick()"
- Temporarily remove session from `blocked_sessions` while polling the user's future
- Rethink the `blocked_sessions` mechanism entirely

---

## Next Steps

### 🎯 Phase 5: Create Thin PlatformDispatcher Wrapper for TestScheduler

Move the `unparkers` mechanism into the scheduler crate, then create a minimal wrapper:

```rust
pub struct TestDispatcherAdapter {
    session_id: SessionId,
    scheduler: Arc<TestScheduler>,
}

impl PlatformDispatcher for TestDispatcherAdapter {
    // ~50 lines of trait method forwarding
}
```

### Phase 6: Delete TestDispatcher

Once the adapter is working, delete `crates/gpui/src/platform/test/dispatcher.rs` entirely.

### Phase 7: Unify Executor Types (Medium Term)

Both GPUI and scheduler have `BackgroundExecutor`, `ForegroundExecutor`, and `Task<T>`.

Options:
1. **GPUI reexports scheduler's types** - Cleanest, but requires API compatibility
2. **Extension traits** - GPUI-specific methods like `detach_and_log_err` as extension traits

### Phase 8: Consider Eliminating PlatformDispatcher (Long Term)

For production (Mac/Linux/Windows), either:
1. Have production dispatchers implement `Scheduler` directly
2. Or keep `PlatformDispatcher` as internal implementation detail

---

## Success Criteria

1. `crates/gpui/src/platform/test/dispatcher.rs` is deleted or reduced to ~50 lines
2. TestScheduler (or thin wrapper) implements PlatformDispatcher
3. No duplicate task queue management between GPUI and scheduler
4. All existing tests pass
5. Minimal code in GPUI for test scheduling support

---

## Notes for Implementation

- SessionId is allocated per TestDispatcher clone. Ensure the same session allocation happens in any replacement.
- The `block_on` implementation in `executor.rs` has complex logic for parking, unparking, and timeout handling.
- `spawn_realtime` panics in TestDispatcher - this is correct behavior (real threads break determinism).