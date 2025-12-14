# GPUI Scheduler Integration Plan

## Goal

Eliminate GPUI's `PlatformDispatcher` abstraction and move entirely to the `Scheduler` trait from the scheduler crate. This will:
- Remove the redundant `TestDispatcher` wrapper
- Unify task scheduling across GPUI and scheduler
- Simplify the codebase significantly

## Current State (After Recent Work)

### What's Been Completed

1. **Task Queue Delegation**: TestDispatcher delegates foreground/background task scheduling to TestScheduler
   - `dispatch()` → `scheduler.schedule_background_with_priority()`
   - `dispatch_on_main_thread()` → `scheduler.schedule_foreground(session_id)`

2. **Unified Priority Types**: GPUI reexports `Priority` and `RealtimePriority` from scheduler crate
   - Removed duplicate definitions from `gpui/src/executor.rs`
   - Added `Realtime(RealtimePriority)` variant to scheduler's Priority

3. **Unified RunnableMeta**: Both GPUI and scheduler use `scheduler::RunnableMeta`
   - `RunnableVariant` is now just a type alias: `pub type RunnableVariant = Runnable<RunnableMeta>`

4. **Removed Deprioritization**: Removed the `deprioritize()` feature and `TaskLabel` handling

5. **Removed Debug Infrastructure**: Removed waiting_hint, waiting_backtrace, debug logging, `start_waiting`/`finish_waiting` from TestDispatcher

6. **Timer Unification**: `BackgroundExecutor::timer()` now uses `Scheduler::timer()` directly in tests
   - Eliminated the `delayed` queue from TestDispatcher
   - `dispatch_after` now panics in tests (should never be called)

7. **`is_main_thread` Delegation**: TestScheduler now tracks whether we're running a foreground task
   - TestDispatcher delegates `is_main_thread()` to scheduler

8. **Clock Delegation**: `advance_clock` and `advance_clock_to_next_timer` are simple delegations to scheduler

9. **Tick Delegation**: `tick()` is now a simple delegation to `scheduler.tick()` / `scheduler.tick_background_only()`

10. **Simplified TestDispatcher**: Now ~170 lines, mostly just PlatformDispatcher trait impl

### Current Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         GPUI                                     │
│  ┌─────────────────┐    ┌──────────────────────────────────┐    │
│  │ BackgroundExecutor │    │ ForegroundExecutor              │    │
│  │ - spawn()         │    │ - spawn()                       │    │
│  │ - timer()         │    │                                 │    │
│  │ - block_on()      │    │                                 │    │
│  └────────┬──────────┘    └────────────────┬────────────────┘    │
│           │                                │                     │
│           └────────────────┬───────────────┘                     │
│                            ▼                                     │
│              ┌─────────────────────────┐                         │
│              │ Arc<dyn PlatformDispatcher> │                      │
│              └─────────────┬───────────┘                         │
│                            │                                     │
│     ┌──────────────────────┼──────────────────────┐              │
│     ▼                      ▼                      ▼              │
│ ┌──────────┐        ┌──────────────┐       ┌─────────────┐       │
│ │MacDispatcher│      │LinuxDispatcher│       │TestDispatcher│      │
│ └──────────┘        └──────────────┘       └──────┬──────┘       │
│                                                   │ (thin wrapper)
└───────────────────────────────────────────────────┼──────────────┘
                                                    │
                                                    ▼
                                    ┌───────────────────────────┐
                                    │      TestScheduler        │
                                    │ (scheduler crate)         │
                                    │ - task queues             │
                                    │ - timers                  │
                                    │ - clock                   │
                                    │ - rng                     │
                                    │ - is_main_thread          │
                                    └───────────────────────────┘
```

### What TestDispatcher Still Does

Located at `crates/gpui/src/platform/test/dispatcher.rs` (~170 lines):

1. **PlatformDispatcher trait impl** - Required interface for GPUI executors
2. **`unparkers`** - Thread synchronization for GPUI's `block_on` when parking is allowed
3. **`simulate_random_delay`** - Custom yield future (0..10 range)
4. **`session_id`** - For foreground task tracking per dispatcher clone

### Key Files

**Scheduler Crate** (`crates/scheduler/`):
- `src/scheduler.rs` - `Scheduler` trait, `Priority`, `RealtimePriority`, `RunnableMeta`, `SessionId`
- `src/test_scheduler.rs` - `TestScheduler` implementation (timers, clock, rng, is_main_thread)
- `src/executor.rs` - `ForegroundExecutor`, `BackgroundExecutor`, `Task`
- `src/clock.rs` - `Clock` trait, `TestClock`

**GPUI Crate** (`crates/gpui/`):
- `src/executor.rs` - GPUI's `BackgroundExecutor`, `ForegroundExecutor`, `Task`
- `src/platform.rs` - `PlatformDispatcher` trait, `RunnableVariant` type alias
- `src/platform/test/dispatcher.rs` - `TestDispatcher` (thin wrapper around TestScheduler)
- `src/platform/mac/dispatcher.rs` - `MacDispatcher`
- `src/platform/linux/dispatcher.rs` - `LinuxDispatcher`
- `src/platform/windows/dispatcher.rs` - `WindowsDispatcher`

---

## Next Steps

### Phase 1: Create SharedRng Wrapper

The `rng()` method currently returns `Arc<Mutex<StdRng>>`, requiring callers to call `.lock()` before using `Rng` methods. This is error-prone. Create a wrapper type that handles locking internally:

```rust
pub struct SharedRng(Arc<Mutex<StdRng>>);

impl SharedRng {
    pub fn lock(&self) -> MutexGuard<StdRng> { self.0.lock() }
    pub fn random_range<T, R>(&self, range: R) -> T { self.0.lock().random_range(range) }
    pub fn random_bool(&self, p: f64) -> bool { self.0.lock().random_bool(p) }
    pub fn random<T>(&self) -> T where StandardUniform: Distribution<T> { self.0.lock().random() }
}
```

Update `TestScheduler::rng()` and `BackgroundExecutor::rng()` to return `SharedRng` instead of `Arc<Mutex<StdRng>>`.

### Phase 2: Move Unparkers to TestScheduler

Move the unparker mechanism into TestScheduler to unify blocking between GPUI's `block_internal` and scheduler's `block`:

```rust
impl TestScheduler {
    pub fn push_unparker(&self, unparker: Unparker);
    pub fn unpark_all(&self);
    pub fn unparker_count(&self) -> usize;
}
```

Call `unpark_all()` when tasks are scheduled in `schedule_foreground()` and `schedule_background()`.

### Phase 3: Move simulate_random_delay to TestScheduler

The scheduler already has `yield_random()`. Either:
- Use scheduler's `yield_random()` directly
- Or make the range configurable if the 0..10 vs 0..2 difference matters

### Phase 4: Create Thin PlatformDispatcher Wrapper for TestScheduler

Create a minimal wrapper that implements `PlatformDispatcher` for `Arc<TestScheduler>`:

```rust
pub struct TestDispatcherAdapter {
    session_id: SessionId,
    scheduler: Arc<TestScheduler>,
}

impl PlatformDispatcher for TestDispatcherAdapter {
    // ~50 lines of trait method forwarding
}
```

### Phase 5: Delete TestDispatcher

Once the adapter is working, delete `crates/gpui/src/platform/test/dispatcher.rs` entirely.

### Phase 6: Unify Executor Types (Medium Term)

Both GPUI and scheduler have `BackgroundExecutor`, `ForegroundExecutor`, and `Task<T>`.

Options:
1. **GPUI reexports scheduler's types** - Cleanest, but requires API compatibility
2. **Keep both, share primitives** - Current partial state
3. **Extension traits** - GPUI-specific methods like `detach_and_log_err` as extension traits on scheduler's types

### Phase 7: Consider Eliminating PlatformDispatcher (Long Term)

For production (Mac/Linux/Windows), either:
1. Have production dispatchers implement `Scheduler` directly
2. Or keep `PlatformDispatcher` as internal implementation detail, with `PlatformScheduler` wrapper

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

- The `block_on` implementation in `executor.rs` has complex logic for parking, unparking, and timeout handling. This may need adjustment when changing the dispatcher abstraction.

- `TaskLabel` is still defined but unused after removing deprioritization. Can be removed entirely.

- `spawn_realtime` panics in TestDispatcher - this is correct behavior (real threads break determinism).