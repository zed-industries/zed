# GPUI Scheduler Integration Plan

## Overview

This document outlines the integration of the `scheduler` crate into GPUI, providing unified scheduling infrastructure for deterministic testing.

## Goals

1. ✅ GPUI uses the `scheduler` crate's `TestScheduler` for deterministic testing
2. ✅ GPUI's public API remains unchanged (no breaking changes for consumers)
3. ✅ Minimize diff in existing test code by using wrapper types
4. ✅ Gain benefits of scheduler's session isolation semantics
5. ✅ Priority support for background tasks

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                          GPUI                                │
├─────────────────────────────────────────────────────────────┤
│  BackgroundExecutor          ForegroundExecutor             │
│  + priorities                + priorities                   │
│  + scoped execution          + GPUI-specific methods        │
│  + test helpers              + realtime support             │
├─────────────────────────────────────────────────────────────┤
│  TestDispatcher (hybrid)     PlatformScheduler (adapter)    │
│  - uses TestScheduler for    - wraps PlatformDispatcher     │
│    timing/clock/rng          - implements Scheduler trait   │
│  - keeps own task queues                                    │
├─────────────────────────────────────────────────────────────┤
│                      Scheduler Crate                         │
│  Scheduler trait, TestScheduler, Priority, executors,       │
│  Task, Timer, Clock (TestClock, SystemClock), RunnableMeta  │
└─────────────────────────────────────────────────────────────┘
```

## Completed Phases

### Phase 1: Scheduler Crate Enhancements ✅

- Added `RunnableMeta` for source location tracking
- Updated `Task<T>` to use `async_task::Task<T, RunnableMeta>`
- Added `SystemClock` for production use
- Updated `Scheduler` trait to use `Runnable<RunnableMeta>`

### Phase 2: GPUI Adapters ✅

- Created `PlatformScheduler` adapter implementing `Scheduler` trait
- Added `RunnableVariant::Scheduler` for scheduler crate's runnables
- Updated all platform dispatchers (Mac, Linux, Windows, Test)

### Phase 3: Hybrid TestDispatcher Integration ✅

`TestDispatcher` uses `TestScheduler` internally for timing/clock/rng while keeping its own task queues for `RunnableVariant` handling.

**Why hybrid approach:**
- GPUI uses `RunnableVariant` with 3 variants (Meta, Compat, Scheduler)
- `TestScheduler` expects `Runnable<RunnableMeta>` only
- Hybrid allows unified timing without breaking existing behavior

**What's delegated to TestScheduler:**
- `now()` → `scheduler.clock().now()`
- `advance_clock()` → `scheduler.advance_clock()`
- `rng()` → `scheduler.rng()`
- `allow_parking()` / `forbid_parking()` → scheduler

**What stays in TestDispatcher:**
- Task queues (foreground, background, deprioritized, delayed)
- GPUI-specific features (task labels, deprioritization, waiting hints)

### Phase 4: API Simplification ✅

- Changed `TestDispatcher::new(StdRng)` → `TestDispatcher::new(seed: u64)`
- Updated all call sites across the codebase
- Removed unused rand imports

### Phase 5: Priority Support ✅

Added priority support for background tasks:

```rust
pub enum Priority {
    High,   // weight: 60
    Medium, // weight: 30 (default)
    Low,    // weight: 10
}
```

- `BackgroundExecutor::spawn_with_priority(priority, future)`
- Priority-weighted random selection in `TestScheduler`
- Preserves intra-session ordering for foreground tasks

## Deferred: Full Executor Composition

Full executor composition (GPUI executors wrapping scheduler executors) is **deferred indefinitely**.

**Reasons:**
- GPUI executors have many features scheduler doesn't have:
  - Realtime priority (dedicated thread)
  - Task labels for test control
  - Scoped execution (`scoped`, `await_on_background`)
  - `detach_and_log_err`
- Current integration already achieves main goals
- Wrapping would require re-implementing most functionality

**Consider only if:**
- Concrete need to share more code between GPUI and scheduler
- Scheduler gains more GPUI-like features

## Test Results

- ✅ All 73 GPUI lib tests pass
- ✅ All 13 scheduler tests pass
- ✅ Clippy passes

## Files Modified

### Scheduler Crate
- `src/scheduler.rs` - `RunnableMeta`, `Priority`, `Scheduler` trait updates, `Hash` on `SessionId`
- `src/executor.rs` - `Runnable<RunnableMeta>`, `spawn_with_priority`
- `src/test_scheduler.rs` - `Runnable<RunnableMeta>`, `parking_allowed()`, priority-weighted selection
- `src/clock.rs` - `SystemClock`
- `src/tests.rs` - `test_background_priority_scheduling`

### GPUI Crate
- `src/platform/platform_scheduler.rs` - New `PlatformScheduler` adapter
- `src/platform.rs` - `RunnableVariant::Scheduler`, platform_scheduler module
- `src/platform/test/dispatcher.rs` - `TestScheduler` integration, `new(seed: u64)`
- `src/platform/mac/dispatcher.rs` - `trampoline_scheduler`, handle new variant
- `src/platform/linux/dispatcher.rs` - Handle new variant
- `src/platform/linux/headless/client.rs` - Handle new variant
- `src/platform/linux/wayland/client.rs` - Handle new variant
- `src/platform/linux/x11/client.rs` - Handle new variant
- `src/platform/windows/dispatcher.rs` - Handle new variant
- `src/test.rs` - Updated `run_test` to pass seed directly
- `src/app/test_context.rs` - Updated `TestAppContext::single()`
- `src/text_system/line_wrapper.rs` - Updated test helper
- `Cargo.toml` - Added scheduler dependency

### Other Crates
- `agent/src/edit_agent/evals.rs` - Use `rand::random()` for seed
- `agent_ui/src/evals.rs` - Use `rand::random()` for seed
- `editor/benches/display_map.rs` - Pass seed directly
- `editor/benches/editor_render.rs` - Pass seed directly
- `extension_host/benches/extension_compilation_benchmark.rs` - Pass seed directly

### Workspace
- `Cargo.toml` - Added scheduler to workspace dependencies

## Behavioral Notes

### Session Isolation
`TestScheduler` blocks same-session foreground tasks during `block_on()`. This is more correct but tests may have accidentally relied on the old behavior.

### Timer/Clock
Uses `TestClock` with explicit time control instead of `Instant::now()` offsets.

## Dependencies

The cloud repo (`zed-industries/cloud`) uses this scheduler crate. After changes:
1. Update cloud's git pin to new zed revision
2. Run cloud's tests to verify compatibility