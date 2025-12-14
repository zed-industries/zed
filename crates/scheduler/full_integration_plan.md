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

5. **Removed Debug Infrastructure**: Removed waiting_hint, waiting_backtrace, debug logging from TestDispatcher

6. **Simplified TestDispatcher**: Now 242 lines, mostly just PlatformDispatcher trait impl

### Current Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         GPUI                                     │
│  ┌─────────────────┐    ┌──────────────────────────────────┐    │
│  │ BackgroundExecutor │    │ ForegroundExecutor              │    │
│  │ - spawn()         │    │ - spawn()                       │    │
│  │ - block_on()      │    │                                 │    │
│  └────────┬──────────┘    └────────────────┬────────────────┘    │
│           │                                │                     │
│           └────────────────┬───────────────┘                     │
│                            ▼                                     │
│              ┌─────────────────────────┐                         │
│              │ Arc<dyn PlatformDispatcher> │◄─── THE PROBLEM      │
│              └─────────────┬───────────┘                         │
│                            │                                     │
│     ┌──────────────────────┼──────────────────────┐              │
│     ▼                      ▼                      ▼              │
│ ┌──────────┐        ┌──────────────┐       ┌─────────────┐       │
│ │MacDispatcher│      │LinuxDispatcher│       │TestDispatcher│      │
│ └──────────┘        └──────────────┘       └──────┬──────┘       │
│                                                   │              │
└───────────────────────────────────────────────────┼──────────────┘
                                                    │
                                                    ▼
                                    ┌───────────────────────────┐
                                    │      TestScheduler        │
                                    │ (scheduler crate)         │
                                    │ - task queues             │
                                    │ - clock/timing            │
                                    │ - rng                     │
                                    └───────────────────────────┘
```

### What TestDispatcher Still Does

Located at `crates/gpui/src/platform/test/dispatcher.rs` (242 lines):

1. **PlatformDispatcher trait impl** - Required interface for GPUI executors
2. **`dispatch_after` delayed queue** - Scheduler has `timer()` (returns Future), not `dispatch_after` (stores runnable)
3. **`is_main_thread` tracking** - Sets flag based on whether running foreground/background task
4. **`unparkers`** - Thread synchronization for `block_on` when parking is allowed
5. **`advance_clock`** - Handles both scheduler clock and local delayed queue
6. **`simulate_random_delay`** - Custom yield future (0..10 range)

### Key Files

**Scheduler Crate** (`crates/scheduler/`):
- `src/scheduler.rs` - `Scheduler` trait, `Priority`, `RealtimePriority`, `RunnableMeta`, `SessionId`
- `src/test_scheduler.rs` - `TestScheduler` implementation
- `src/executor.rs` - `ForegroundExecutor`, `BackgroundExecutor`, `Task`
- `src/clock.rs` - `Clock` trait, `TestClock`

**GPUI Crate** (`crates/gpui/`):
- `src/executor.rs` - GPUI's `BackgroundExecutor`, `ForegroundExecutor`, `Task` (duplicates scheduler's!)
- `src/platform.rs` - `PlatformDispatcher` trait, `RunnableVariant` type alias
- `src/platform/test/dispatcher.rs` - `TestDispatcher` (the wrapper we want to eliminate)
- `src/platform/mac/dispatcher.rs` - `MacDispatcher`
- `src/platform/linux/dispatcher.rs` - `LinuxDispatcher`
- `src/platform/windows/dispatcher.rs` - `WindowsDispatcher`

---

## Next Steps: Eliminate PlatformDispatcher

### The Core Problem

GPUI has two parallel abstractions:
1. `PlatformDispatcher` trait - used by GPUI's executors
2. `Scheduler` trait - used by scheduler crate

They have different APIs:
- `PlatformDispatcher::dispatch_after(duration, runnable)` - stores runnable, runs after delay
- `Scheduler::timer(duration) -> Timer` - returns a future that completes after delay

### Strategy: Make Scheduler the Primary Abstraction

#### Phase 1: Add `dispatch_after` Support to Scheduler

Add a method to `Scheduler` trait that matches `PlatformDispatcher::dispatch_after`:

```rust
// In scheduler/src/scheduler.rs
pub trait Scheduler: Send + Sync {
    // ... existing methods ...
    
    /// Schedule a runnable to execute after a delay.
    /// This is the imperative equivalent of timer().
    fn schedule_after(&self, duration: Duration, runnable: Runnable<RunnableMeta>);
}
```

For `TestScheduler`, implement by adding to the timers list with a callback.

#### Phase 2: Add `is_main_thread` to Scheduler

The scheduler needs to track whether we're currently executing a foreground task:

```rust
// In TestScheduler
pub fn is_main_thread(&self) -> bool {
    self.state.lock().is_main_thread
}
```

Update `step()` to set this flag around task execution.

#### Phase 3: Add Parking/Unparker Support to Scheduler

Move the unparker mechanism into TestScheduler:

```rust
impl TestScheduler {
    pub fn push_unparker(&self, unparker: Unparker) { ... }
    pub fn unpark_all(&self) { ... }
}
```

Call `unpark_all()` when tasks are scheduled.

#### Phase 4: Implement PlatformDispatcher for TestScheduler

Either:
- Have `TestScheduler` implement `PlatformDispatcher` directly (couples scheduler to GPUI)
- Create a trivial newtype wrapper that implements `PlatformDispatcher` for `Arc<TestScheduler>`

The wrapper would be ~50 lines, just trait method forwarding.

#### Phase 5: Unify Executor Types

Both GPUI and scheduler have `BackgroundExecutor`, `ForegroundExecutor`, and `Task<T>`.

Options:
1. **GPUI reexports scheduler's types** - Cleanest, but requires ensuring API compatibility
2. **Keep both, share primitives** - More work, less clean
3. **Scheduler types become the implementation, GPUI wraps** - Current partial state

Recommendation: Have GPUI reexport scheduler's executor types, adding any GPUI-specific methods as extension traits.

#### Phase 6: Production Dispatchers

For production (Mac/Linux/Windows), either:
1. Create a `PlatformScheduler` that wraps `PlatformDispatcher` and implements `Scheduler`
2. Or keep `PlatformDispatcher` for production, only use `Scheduler` for tests

Current state: `PlatformScheduler` exists in `gpui/src/platform/platform_scheduler.rs` and wraps `PlatformDispatcher` to implement `Scheduler`. This could be inverted.

---

## Recommended Execution Order

### Immediate (Do Now)

1. **Add `schedule_after` to Scheduler trait**
   - Implement in TestScheduler using timer infrastructure
   - This eliminates the need for TestDispatcher's delayed queue

2. **Move `is_main_thread` tracking into TestScheduler**
   - Track in `step()` which type of task is running
   - Expose via `is_main_thread()` method

3. **Move unparker mechanism into TestScheduler**
   - Add `push_unparker()`, `unpark_all()`, `unparker_count()`
   - Call `unpark_all()` in `schedule_foreground()` and `schedule_background()`

4. **Create thin PlatformDispatcher impl for TestScheduler**
   - Either impl directly on TestScheduler or create `TestDispatcherAdapter(Arc<TestScheduler>)`
   - Should be ~50-80 lines total

5. **Delete TestDispatcher**
   - Update all references to use the new adapter
   - Remove `crates/gpui/src/platform/test/dispatcher.rs`

### Medium Term

6. **Unify Task types**
   - Have GPUI reexport `scheduler::Task<T>`
   - Or create extension trait for GPUI-specific methods like `detach_and_log_err`

7. **Unify Executor types**
   - Evaluate if GPUI's executors can be replaced with scheduler's
   - Main blocker: GPUI executors take `Arc<dyn PlatformDispatcher>`

### Long Term

8. **Consider eliminating PlatformDispatcher entirely**
   - Have production dispatchers implement `Scheduler` directly
   - Or keep as internal implementation detail

---

## API Comparison

### PlatformDispatcher (current GPUI trait)
```rust
pub trait PlatformDispatcher: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn dispatch(&self, runnable: RunnableVariant, label: Option<TaskLabel>, priority: Priority);
    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority);
    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant);
    fn now(&self) -> Instant;
    fn spawn_realtime(&self, priority: RealtimePriority, f: Box<dyn FnOnce() + Send>);
    fn as_test(&self) -> Option<&TestDispatcher>;
    // ... timing methods ...
}
```

### Scheduler (scheduler crate trait)
```rust
pub trait Scheduler: Send + Sync {
    fn schedule_foreground(&self, session_id: SessionId, runnable: Runnable<RunnableMeta>);
    fn schedule_background_with_priority(&self, runnable: Runnable<RunnableMeta>, priority: Priority);
    fn schedule_background(&self, runnable: Runnable<RunnableMeta>);
    fn timer(&self, duration: Duration) -> Timer;
    fn clock(&self) -> Arc<dyn Clock>;
    fn block(&self, session_id: Option<SessionId>, future: LocalBoxFuture<()>, timeout: Option<Duration>);
    fn as_test(&self) -> &TestScheduler;
}
```

### Proposed Additions to Scheduler
```rust
pub trait Scheduler: Send + Sync {
    // ... existing ...
    
    /// Schedule a runnable to run after a duration (imperative timer API)
    fn schedule_after(&self, duration: Duration, runnable: Runnable<RunnableMeta>);
    
    /// Check if currently executing a foreground task
    fn is_main_thread(&self) -> bool;
}

impl TestScheduler {
    /// For block_on parking support
    pub fn push_unparker(&self, unparker: Unparker);
    pub fn unpark_all(&self);
    pub fn unparker_count(&self) -> usize;
}
```

---

## Success Criteria

1. `crates/gpui/src/platform/test/dispatcher.rs` is deleted
2. TestScheduler (or thin wrapper) implements PlatformDispatcher
3. No duplicate task queue management between GPUI and scheduler
4. All existing tests pass
5. Minimal code in GPUI for test scheduling support

---

## Notes for Implementation

- The `simulate_random_delay()` method uses range 0..10, while scheduler's `yield_random()` uses 0..2 with 10% chance of 10..20. Tests may depend on the exact distribution. Consider making this configurable or accepting the change.

- `spawn_realtime` panics in TestDispatcher - this is correct behavior (real threads break determinism).

- SessionId is allocated per TestDispatcher clone. When eliminating TestDispatcher, ensure the same session allocation happens (probably in the PlatformDispatcher wrapper's Clone impl).

- The `block_on` implementation in `executor.rs` has complex logic for parking, unparking, and timeout handling. This may need adjustment when changing the dispatcher abstraction.

- `TaskLabel` is still defined but unused after removing deprioritization. Can be removed entirely or kept for future use.