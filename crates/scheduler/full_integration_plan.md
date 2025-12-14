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

10. **Simplified TestDispatcher**: Now ~70 lines, removed unparkers mechanism

### Key Files

**Scheduler Crate** (`crates/scheduler/`):
- `src/scheduler.rs` - `Scheduler` trait, `Priority`, `RealtimePriority`, `RunnableMeta`, `SessionId`
- `src/test_scheduler.rs` - `TestScheduler` implementation (timers, clock, rng, is_main_thread, block)

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

## In Progress / Broken Phase

### 🔴 Phase 5: Simplify block_internal and Remove Unparkers

**What was attempted:**
1. Simplified `block_internal` in `gpui/src/executor.rs`:
   - Removed debug logging infrastructure (`debug_log`, `DEBUG_SCHEDULER`, etc.)
   - Removed `Parker`/`Unparker` usage - now uses simple `waker_fn` with `AtomicBool`
   - Simplified parking logic: when parking is allowed, just calls `std::thread::yield_now()`
   - Removed the complex unparker push/park_timeout dance

2. Simplified `TestDispatcher` in `gpui/src/platform/test/dispatcher.rs`:
   - Removed `TestDispatcherState` struct entirely
   - Removed `unparkers: Vec<Unparker>` field
   - Removed `unpark_all()`, `push_unparker()`, `unparker_count()` methods
   - Removed `self.unpark_all()` calls from `dispatch()` and `dispatch_on_main_thread()`
   - TestDispatcher is now just `{ session_id, scheduler }` (~70 lines total)

**Failing tests (3 in editor crate):**
```
test element::tests::test_soft_wrap_editor_width_auto_height_editor ... FAILED
test inlays::inlay_hints::tests::test_no_hint_updates_for_unrelated_language_files ... FAILED
test inlays::inlay_hints::tests::test_inside_char_boundary_range_hints ... FAILED
```

**Root cause hypothesis:**
The soft wrapping and inlay hint tests involve async operations that need tasks to run. The issue is likely that:
1. When `tick()` returns false (no tasks ready), the old code would park with a timeout and wait for new tasks
2. The old `unparkers` mechanism would wake blocked threads when new tasks were dispatched
3. The new code just calls `yield_now()` which doesn't actually wait for anything

The tests that fail are ones where async work (like soft wrap calculation) happens in background tasks, and the test needs those tasks to complete before assertions run.

**Possible fixes to investigate:**
1. **Restore the parker/unparker mechanism** but keep it simpler - the parking is actually needed for some tests
2. **Use scheduler.block() for parking cases only** - when parking is allowed, delegate to scheduler.block()
3. **Add a "wait for tasks" primitive** to TestScheduler that the block_internal can use

**Key insight:** The unparkers weren't just dead code - they serve to wake up blocked `block_internal` calls when new tasks are scheduled. Without them, `block_internal` busy-loops with `yield_now()` which may not give tasks time to run properly.

---

## Next Phase (after fixing Phase 5)

### Phase 6: Further Simplify TestDispatcher

After Phase 5 works correctly, evaluate if `TestDispatcher` can be reduced further or if `TestScheduler` can directly implement `PlatformDispatcher`.

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

1. `crates/gpui/src/platform/test/dispatcher.rs` reduced to ~50-70 lines (no unparkers) ✅
2. GPUI's `block_internal` simplified ✅
3. All existing tests pass ❌ (3 failing)
4. No duplicate blocking/parking logic between GPUI and scheduler

---

## Notes for Implementation

- `SessionId` is allocated per `TestDispatcher` clone. This is used for foreground task routing.
- The scheduler's `block()` uses `thread::park()` which works with the `Thread` handle stored in `TestScheduler`.
- `spawn_realtime` panics in `TestDispatcher` - this is correct (real threads break determinism).
- The `#[cfg(not(any(test, feature = "test-support")))]` version of `block_internal` doesn't tick tasks at all - it just parks and waits for the waker. This is correct for production.
- **Important:** The failing tests involve async operations (soft wrap, inlay hints) that schedule background work. The test framework needs to properly wait for this work to complete.