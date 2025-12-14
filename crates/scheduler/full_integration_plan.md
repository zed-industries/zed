# Full GPUI Scheduler Integration Plan

## ✅ COMPLETED WORK (as of this session)

### 1. Unified RunnableMeta (Phase 2a partial) - DONE
- Re-exported `RunnableMeta` from scheduler crate in `gpui/src/platform.rs`
- Removed GPUI's duplicate `RunnableMeta` struct definition
- Removed `RunnableVariant::Scheduler` variant (now uses `RunnableVariant::Meta` for both)
- Updated all platform dispatchers (Mac, Linux, Windows, Test) to remove `Scheduler` variant handling
- Updated `PlatformScheduler` to use `RunnableVariant::Meta` instead of `RunnableVariant::Scheduler`

### 2. Waiting Hints (Phase 3b) - SKIPPED
- NOT NEEDED: TestScheduler already has superior `pending_traces` system with `TracingWaker`
- Automatically captures backtraces for ALL pending futures via `PENDING_TRACES=1` env var
- More comprehensive than manual waiting hints

## 🔜 NEXT STEPS FOR FUTURE WORK

### Near-term (recommended next tasks):
1. **Unify Task types (Phase 1)** - Make GPUI's `Task<T>` re-export or wrap scheduler's `Task<T>`
   - Add `is_ready()` method to scheduler's `Task<T>` if needed
   - Keep `detach_and_log_err` as extension trait in GPUI (needs `App` context)

### Medium-term:
2. **Eliminate RunnableVariant entirely (Phase 2a full)** - Remove `Compat` variant by converting timer callbacks to use `RunnableMeta`
3. **Delegate task queues to TestScheduler (Phase 2b)** - Most complex change

---

## Executive Summary

This document outlines a plan for deeper integration between the `scheduler` crate and GPUI. The current state is a **hybrid integration** where `TestDispatcher` uses `TestScheduler` for timing/clock/rng but maintains its own task queues. This plan describes how to move toward GPUI using the scheduler crate's executors more directly.

## Current State (Hybrid Integration)

### What's Already Done

1. **TestScheduler for timing primitives**: `TestDispatcher` delegates `now()`, `advance_clock()`, `rng()`, `allow_parking()`, and `forbid_parking()` to `TestScheduler`.

2. **PlatformScheduler adapter**: A `Scheduler` trait implementation wraps `PlatformDispatcher` for production use.

3. **RunnableVariant enum**: GPUI dispatchers handle three runnable types:
   - `Meta(Runnable<RunnableMeta>)` - GPUI's native tasks with source location
   - `Compat(Runnable)` - Legacy compatibility tasks
   - `Scheduler(Runnable<RunnableMeta>)` - Tasks from scheduler crate

4. **Priority support**: Both GPUI and scheduler have `Priority` enums with weighted scheduling.

### Current Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                          GPUI                                    │
├─────────────────────────────────────────────────────────────────┤
│  BackgroundExecutor              ForegroundExecutor             │
│  - spawns via PlatformDispatcher - spawns via PlatformDispatcher│
│  - handles Realtime priority     - handles Priority             │
│  - labeled tasks                 - spawn_local checks           │
│  - scoped execution                                             │
├─────────────────────────────────────────────────────────────────┤
│  TestDispatcher (HYBRID)         Platform Dispatchers           │
│  - owns task queues              - Mac/Linux/Windows            │
│  - delegates timing to           - dispatch to thread pools     │
│    TestScheduler                 - real time                    │
│  - GPUI-specific features                                       │
├─────────────────────────────────────────────────────────────────┤
│                      Scheduler Crate                             │
│  TestScheduler: timing/clock/rng                                │
│  ForegroundExecutor/BackgroundExecutor (unused by GPUI)         │
│  Task<T>, Timer, Clock                                          │
└─────────────────────────────────────────────────────────────────┘
```

### Why Hybrid?

GPUI's executors have features the scheduler crate doesn't:
- **Realtime priority**: Spawns dedicated OS threads for audio/realtime work
- **Task labels**: Allow tests to deprioritize specific tasks
- **Scoped execution**: `scoped()` and `await_on_background()` for borrowing
- **Labeled spawn**: `spawn_labeled()` for test control
- **`detach_and_log_err`**: Convenient error logging on detach
- **Waiting hints**: Debug info when tests park unexpectedly

## Goals for Full Integration

### Primary Goals

1. **Single source of truth for test scheduling**: All test task scheduling goes through scheduler crate's `TestScheduler`
2. **Unified Task type**: Use scheduler's `Task<T>` throughout, or make them interchangeable
3. **Simplified dispatcher hierarchy**: Reduce code duplication between GPUI and scheduler
4. **Session isolation**: Leverage scheduler's session semantics for multi-context tests

### Non-Goals (for now)

- Replacing production dispatchers (Mac/Linux/Windows remain platform-specific)
- Changing the external GPUI API significantly
- Breaking existing tests

## Integration Phases

### Phase 1: Unify Task Types ✦ Medium effort

**Problem**: GPUI has its own `Task<T>` wrapper, scheduler has another. They're nearly identical but not interchangeable.

**Solution**: Make GPUI's `Task<T>` a re-export or thin wrapper of scheduler's `Task<T>`.

**Steps**:
1. Add `is_ready()` method to scheduler's `Task<T>`:
   ```rust
   // In scheduler/src/executor.rs
   impl<T> Task<T> {
       pub fn is_ready(&self) -> bool {
           match &self.0 {
               TaskState::Ready(_) => true,
               TaskState::Spawned(task) => task.is_finished(),
           }
       }
   }
   ```

2. `detach_and_log_err` cannot live in scheduler (needs `App` context). Options:
   - Keep as extension trait in GPUI
   - Add `detach_with_callback` to scheduler that GPUI wraps
   
3. Update GPUI to re-export:
   ```rust
   // In gpui/src/executor.rs
   pub use scheduler::Task;
   
   // Extension trait for GPUI-specific functionality
   pub trait TaskExt<T, E> {
       fn detach_and_log_err(self, cx: &App);
   }
   
   impl<T: 'static, E: Debug + 'static> TaskExt<T, E> for Task<Result<T, E>> {
       fn detach_and_log_err(self, cx: &App) {
           // existing implementation
       }
   }
   ```

4. Ensure `#[must_use]` and `Debug` traits match

**Files affected**:
- `crates/scheduler/src/executor.rs`
- `crates/gpui/src/executor.rs`

**Risk**: Low - mostly additive changes

---

### Phase 2: Migrate TestDispatcher Task Queues ✦ High effort

**Problem**: `TestDispatcher` maintains its own `foreground`, `background`, `deprioritized_background`, and `delayed` queues, duplicating logic in `TestScheduler`.

**Solution**: Have `TestDispatcher` delegate task scheduling to `TestScheduler` while keeping GPUI-specific features as a layer on top.

**Sub-phases**:

#### 2a: Eliminate RunnableVariant

Currently, GPUI dispatchers handle three runnable variants. This complexity exists because:
- `Meta` and `Scheduler` both have `RunnableMeta` but from different crate paths
- `Compat` exists for timer callbacks that use bare `async_task::spawn`

**Steps**:
1. Unify `RunnableMeta` - make GPUI re-export scheduler's version
2. Convert all task spawning to use the unified metadata type
3. Remove `RunnableVariant` in favor of single `Runnable<RunnableMeta>` type
4. Update all platform dispatchers

**Files affected**:
- `crates/gpui/src/platform.rs` (RunnableVariant definition)
- `crates/gpui/src/platform/mac/dispatcher.rs`
- `crates/gpui/src/platform/linux/dispatcher.rs`
- `crates/gpui/src/platform/windows/dispatcher.rs`
- `crates/gpui/src/platform/test/dispatcher.rs`
- `crates/gpui/src/executor.rs`

#### 2b: Delegate Task Queues to TestScheduler

**Steps**:
1. Remove `foreground`, `background` fields from `TestDispatcherState`
2. Implement GPUI's `dispatch` by calling `TestScheduler::schedule_background`
3. Implement GPUI's `dispatch_on_main_thread` by calling `TestScheduler::schedule_foreground`
4. Keep `deprioritized_background` as GPUI-specific layer
5. Keep `delayed` or delegate to scheduler's timer infrastructure

**Challenges**:
- `TestScheduler` expects `SessionId` for foreground tasks; need to track session per dispatcher
- Task labels for deprioritization need custom handling
- Execution hash tracking for determinism verification needs preservation

---

### Phase 3: Add GPUI Features to Scheduler ✦ Medium effort

Some GPUI features should move to the scheduler crate to reduce duplication.

#### 3a: Task Labels / Deprioritization

**Problem**: GPUI tests can deprioritize tasks by label. Scheduler has no concept of this.

**Options**:
1. Add `TaskLabel` to scheduler's `RunnableMeta`
2. Keep deprioritization as GPUI-only layer
3. Add generic "task tags" to scheduler

**Recommendation**: Option 2 for now - keep as GPUI layer. Deprioritization is test-specific and not commonly needed.

#### 3b: Waiting Hints / Backtraces

**Problem**: When tests hang, GPUI captures waiting hints and backtraces for debugging.

**Solution**: NOT NEEDED - TestScheduler already has superior `pending_traces` system with `TracingWaker` that automatically captures backtraces for ALL pending futures via `PENDING_TRACES=1` env var.

---

### Phase 4: Unify Executors ✦ High effort (OPTIONAL)

**Problem**: Both GPUI and scheduler have `BackgroundExecutor` and `ForegroundExecutor` types.

**Current State**: GPUI executors wrap `PlatformDispatcher`. Scheduler executors wrap `Arc<dyn Scheduler>`.

**Options**:

#### Option A: GPUI wraps scheduler executors
- GPUI's `BackgroundExecutor` wraps scheduler's `BackgroundExecutor`
- Adds GPUI-specific methods (labeled spawn, scoped, etc.)
- **Pro**: Single scheduling implementation
- **Con**: Complex wrapping, scheduler needs to expose more

#### Option B: Keep separate, share primitives
- Both use same `Task<T>`, `RunnableMeta`, `Priority`
- Different executor implementations
- **Pro**: Simpler, maintains flexibility
- **Con**: Some code duplication

#### Option C: Scheduler provides core, GPUI extends
- Scheduler provides trait-based executor interface
- GPUI implements the traits with additional features
- **Pro**: Clean separation
- **Con**: Significant refactoring

**Recommendation**: Option B for now. Full unification has diminishing returns given GPUI's unique requirements (Realtime priority, platform integration, labeled tasks).

---

### Phase 5: Production Scheduler Adapter Improvements ✦ Low effort

**Problem**: `PlatformScheduler` is minimal - it just adapts `PlatformDispatcher` to `Scheduler` trait.

**Steps**:
1. Consider if `Scheduler` trait should live in GPUI instead of scheduler crate
2. Improve timer implementation (currently spawns a task just to send on channel)
3. Add session tracking if needed for future features

---

## Recommended Execution Order

### Near-term (High Value, Manageable Risk)

#### 1. Unify RunnableMeta (Phase 2a partial)

Make GPUI re-export scheduler's `RunnableMeta`. Currently both define identical structs:

```rust
// scheduler/src/scheduler.rs
#[derive(Clone, Debug)]
pub struct RunnableMeta {
    pub location: &'static Location<'static>,
}

// gpui/src/platform.rs  
#[derive(Debug)]
pub struct RunnableMeta {
    pub location: &'static core::panic::Location<'static>,
}
```

**Changes needed**:

```rust
// gpui/src/platform.rs - REMOVE the struct definition, add:
pub use scheduler::RunnableMeta;

// gpui/src/platform.rs - Simplify RunnableVariant:
pub enum RunnableVariant {
    Meta(Runnable<RunnableMeta>),  // Now uses scheduler's RunnableMeta
    Compat(Runnable),              // Keep for now (timer callbacks)
    // Remove Scheduler variant - it's now same as Meta
}
```

**Migration path for Scheduler variant removal**:
- Search for `RunnableVariant::Scheduler` uses
- Convert to `RunnableVariant::Meta`
- Update all dispatcher match arms

#### 2. Eliminate RunnableVariant::Compat (Phase 2a full)

Convert timer callbacks to use `RunnableMeta` instead of plain `Runnable`, eliminating the need for the `Compat` variant entirely.

### Medium-term (Higher Effort)

3. **Unify Task types** (Phase 1)
   - After RunnableMeta unified, this becomes cleaner

### Long-term (Optional)

4. **Delegate task queues** (Phase 2b)
   - Most complex change
   - Only if clear benefit emerges

5. **Executor unification** (Phase 4)
   - Defer unless concrete need arises

## Technical Considerations

### Test Determinism

The hybrid approach currently works because:
- RNG is shared (from TestScheduler)
- Clock is shared (from TestScheduler) 
- Task selection uses shared RNG

Moving task queues to scheduler must preserve:
- Same RNG consumption pattern
- Same selection algorithm (weighted by priority)
- Same handling of foreground vs background ratio

**Critical difference to address**: GPUI's `TestDispatcher::tick()` uses a different selection algorithm than `TestScheduler::step()`:

```rust
// GPUI: Weighted by queue size (foreground vs background)
let main_thread = rng.random_ratio(
    foreground_len as u32,
    (foreground_len + background_len) as u32,
);

// Scheduler: Weighted by Priority enum weights
// (and preserves intra-session ordering for foreground)
```

To migrate, either:
1. Add GPUI-compatible selection mode to TestScheduler
2. Accept behavior change (may affect test seeds)
3. Keep GPUI's tick() implementation, only delegate storage

### Session Isolation

`TestScheduler` has `SessionId` for isolating foreground tasks. Currently unused by GPUI.

For multi-context tests (e.g., `cx_a`, `cx_b`), consider:
- Each `TestAppContext` gets its own `SessionId`
- Foreground tasks are isolated per session
- Background tasks are global (current behavior)

### Backwards Compatibility

Tests use these APIs that must keep working:
- `cx.executor().run_until_parked()`
- `cx.executor().advance_clock(duration)`
- `cx.executor().tick()`
- `#[gpui::test(iterations = N)]`
- `SEED=N` environment variable

## Metrics for Success

1. **Reduced code duplication**: Fewer lines handling task scheduling in GPUI
2. **Simpler mental model**: One place to understand test scheduling
3. **No test regressions**: All existing tests pass with same seeds
4. **Better debugging**: Unified tracing/logging across scheduler and GPUI

## First Concrete Task: Unify RunnableMeta

Here's the minimal change to start:

### Step 1: Export RunnableMeta from scheduler

Already done - scheduler exports `RunnableMeta` from `scheduler.rs`.

### Step 2: Update GPUI imports

```rust
// crates/gpui/src/platform.rs

// BEFORE:
use scheduler::RunnableMeta as SchedulerRunnableMeta;
// ...
#[doc(hidden)]
#[derive(Debug)]
pub struct RunnableMeta {
    pub location: &'static core::panic::Location<'static>,
}

pub enum RunnableVariant {
    Meta(Runnable<RunnableMeta>),
    Compat(Runnable),
    Scheduler(Runnable<SchedulerRunnableMeta>),
}

// AFTER:
pub use scheduler::RunnableMeta;

pub enum RunnableVariant {
    Meta(Runnable<RunnableMeta>),
    Compat(Runnable),
    // Scheduler variant removed - now same as Meta
}
```

### Step 3: Update all dispatcher match arms

In each platform dispatcher, change:
```rust
// BEFORE:
match runnable {
    RunnableVariant::Meta(r) => r.run(),
    RunnableVariant::Compat(r) => r.run(),
    RunnableVariant::Scheduler(r) => r.run(),
}

// AFTER:
match runnable {
    RunnableVariant::Meta(r) => r.run(),
    RunnableVariant::Compat(r) => r.run(),
}
```

### Step 4: Update PlatformScheduler

```rust
// crates/gpui/src/platform/platform_scheduler.rs

// BEFORE:
fn schedule_foreground(&self, _session_id: SessionId, runnable: Runnable<RunnableMeta>) {
    self.dispatcher.dispatch_on_main_thread(
        RunnableVariant::Scheduler(runnable),  // <-- was Scheduler
        GpuiPriority::default(),
    );
}

// AFTER:
fn schedule_foreground(&self, _session_id: SessionId, runnable: Runnable<RunnableMeta>) {
    self.dispatcher.dispatch_on_main_thread(
        RunnableVariant::Meta(runnable),  // <-- now Meta (same type)
        GpuiPriority::default(),
    );
}
```

**Estimated diff size**: ~50 lines across 6-7 files

---

## Appendix: File Inventory

### Scheduler Crate
- `src/scheduler.rs` - Main entry, re-exports, `Priority`, `RunnableMeta`, `Scheduler` trait
- `src/executor.rs` - `ForegroundExecutor`, `BackgroundExecutor`, `Task<T>`
- `src/test_scheduler.rs` - `TestScheduler`, `TestSchedulerConfig`
- `src/clock.rs` - `Clock` trait, `TestClock`, `SystemClock`

### GPUI Crate
- `src/executor.rs` - GPUI's `BackgroundExecutor`, `ForegroundExecutor`, `Task<T>`, `Priority`
- `src/platform.rs` - `RunnableMeta`, `RunnableVariant`, `PlatformDispatcher` trait
- `src/platform/platform_scheduler.rs` - `PlatformScheduler` adapter
- `src/platform/test/dispatcher.rs` - `TestDispatcher` (hybrid)
- `src/platform/mac/dispatcher.rs` - macOS dispatcher
- `src/platform/linux/dispatcher.rs` - Linux dispatcher  
- `src/platform/windows/dispatcher.rs` - Windows dispatcher
- `src/test.rs` - `run_test` function
- `src/app/test_context.rs` - `TestAppContext`
