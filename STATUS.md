# Scheduler Integration - Debugging Status

## Problem

PR #44810 causes Zed to hang on startup on Linux/Windows, but works fine on Mac.

From PR comment by @yara-blue and @localcc:
> "With it applied zed hangs without ever responding when you open it"

## What Was Cleaned Up

Removed unrelated changes that were accidentally committed:
- `ConfiguredApiCard`, `InstructionListItem`, `api_key.rs` (UI components)
- Debug instrumentation in `terminal_tool.rs`, `capability_granter.rs`, `wasm_host/wit/since_v0_8_0.rs`, `lsp_store.rs`
- Planning docs (`.rules`, `PLAN.md`, old `STATUS.md`)

Kept `language_registry.rs` changes as they may be relevant to debugging.

## Analysis So Far

### Code paths verified as correct:

1. **Priority queue algorithm** - The weighted random selection in `crates/gpui/src/queue.rs` is mathematically sound. When the last non-empty queue is checked, the probability is always 100%.

2. **`async_task::Task` implements `Unpin`** - So `Pin::new(task).poll(cx)` is valid.

3. **`parking::Parker` semantics** - If `unpark()` is called before `park()`, the `park()` returns immediately. This is correct.

4. **Waker creation** - `waker_fn` with `Unparker` (which is `Clone + Send + Sync`) should work correctly.

5. **`PlatformScheduler::block` implementation** - Identical logic to the old `block_internal` for production builds.

### The blocking flow:

1. Task spawned → runnable scheduled → sent to priority queue
2. Background thread waiting on condvar in `PriorityQueueReceiver::recv()`
3. `send()` pushes to queue and calls `condvar.notify_one()`
4. Background thread wakes, pops item, runs runnable
5. When task completes, `async_task` wakes the registered waker
6. Waker calls `unparker.unpark()`
7. `parker.park()` returns
8. Future is polled again, returns `Ready`

### Files involved:

- `crates/gpui/src/platform_scheduler.rs` - `PlatformScheduler::block()` implementation
- `crates/gpui/src/executor.rs` - `BackgroundExecutor::block()` wraps futures
- `crates/gpui/src/queue.rs` - Priority queue with `parking_lot::Condvar`
- `crates/gpui/src/platform/linux/dispatcher.rs` - Background thread pool
- `crates/scheduler/src/executor.rs` - `scheduler::BackgroundExecutor::spawn_with_priority`

## What to investigate next

### 1. Verify background threads are actually running

Add logging at the start of background worker threads in `LinuxDispatcher::new()`:
```rust
.spawn(move || {
    log::info!("[LinuxDispatcher] background worker {} started", i);
    for runnable in receiver.iter() {
        // ...
    }
})
```

### 2. Verify tasks are being dispatched

Add logging in `PlatformScheduler::schedule_background_with_priority`:
```rust
fn schedule_background_with_priority(&self, runnable: Runnable<RunnableMeta>, priority: Priority) {
    log::info!("[PlatformScheduler] dispatching task priority={:?}", priority);
    self.dispatcher.dispatch(runnable, priority);
}
```

### 3. Verify the priority queue send/receive

In `crates/gpui/src/queue.rs`, add logging to `send()` and `recv()`:
```rust
fn send(&self, priority: Priority, item: T) -> Result<(), SendError<T>> {
    // ...
    self.condvar.notify_one();
    log::debug!("[PriorityQueue] sent item, notified condvar");
    Ok(())
}

fn recv(&self) -> Result<...> {
    log::debug!("[PriorityQueue] recv() waiting...");
    while queues.is_empty() {
        self.condvar.wait(&mut queues);
    }
    log::debug!("[PriorityQueue] recv() got item");
    // ...
}
```

### 4. Check timing of dispatcher creation vs task spawning

Trace when `LinuxDispatcher::new()` is called vs when the first `spawn()` happens. If tasks are spawned before background threads are ready, they might be lost.

### 5. Check for platform-specific differences in `parking` or `parking_lot`

The `parking` crate (used for `Parker`/`Unparker`) and `parking_lot` (used for `Condvar` in the priority queue) may have platform-specific behavior. Check their GitHub issues for Linux-specific bugs.

### 6. Verify the startup sequence

The hang happens during startup. Key calls in `crates/zed/src/main.rs`:
```rust
// Line ~292: Tasks spawned BEFORE app.run()
let system_id = app.background_executor().spawn(system_id());
let installation_id = app.background_executor().spawn(installation_id());
let session = app.background_executor().spawn(Session::new(session_id.clone()));

// Line ~513-515: Inside app.run() callback, these BLOCK waiting for the tasks
let system_id = cx.background_executor().block(system_id).ok();
let installation_id = cx.background_executor().block(installation_id).ok();
let session = cx.background_executor().block(session);
```

If background threads aren't running yet when `block()` is called, or if the tasks never got dispatched, it will hang forever.

## Hypotheses to test

1. **Background threads not started yet** - Race condition where tasks are dispatched before threads are listening on the queue.

2. **Condvar notification lost** - `notify_one()` called but no thread was waiting yet, and subsequent waits miss it.

3. **Platform-specific parking behavior** - `parking::Parker` or `parking_lot::Condvar` behaves differently on Linux.

4. **Priority queue never releases items** - Something in the weighted random selection is wrong on Linux (different RNG behavior?).

## Running tests

To get logs, set `RUST_LOG=info` or `RUST_LOG=debug` when running Zed.

For the extension_host test hang (separate issue):
```bash
cargo test -p extension_host extension_store_test::test_extension_store_with_test_extension -- --nocapture
```

## Key commits

- `5b07e2b242` - "WIP: scheduler integration debugging" - This accidentally added unrelated UI components
- `d8ebd8101f` - "WIP: scheduler integration debugging + agent terminal diagnostics" - Added debug instrumentation (now removed)