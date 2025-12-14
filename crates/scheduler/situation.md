# Investigation: How Did a Broken Test Pass CI?

## Executive Summary

This document investigates how commit `fd1494c31a` ("Fix remote server completions not being queried from all LSP servers" - Nov 17, 2025) introduced a test bug that causes `test_collaborating_with_completion` to hang, yet passed CI and was merged.

**Root Cause Identified**: The test had a race condition where LSP request handlers were registered AFTER typing the trigger character. Whether this worked depended on task scheduling order, which varies by seed. The test passed on CI because, at that time, seed 0's task ordering happened to register handlers before the completion request was processed. Later code changes altered the task set, changing seed 0's behavior from "pass" to "hang".

## The Bug

The commit set up LSP request handlers AFTER typing the trigger character:

```rust
// Type trigger first - this spawns async tasks to send completion request
editor_b.update_in(cx_b, |editor, window, cx| {
    editor.handle_input(".", window, cx);
});
cx_b.focus(&editor_b);

// THEN set up handlers (race condition!)
cx_a.executor().start_waiting();
fake_language_server
    .set_request_handler::<lsp::request::Completion, _, _>(...)
    .next()  // <-- Waits for handler to receive a request
    .await
    .unwrap();
```

## Why This Is Non-Deterministic

### How TestDispatcher Works

The `TestDispatcher` provides deterministic async execution in tests:
- Tasks are queued in `foreground` and `background` vectors
- `tick()` **randomly selects** which task to run based on a seeded RNG
- Same seed = identical task ordering
- Different seed = different task ordering

Key code from `dispatcher.rs`:
```rust
let ix = state.random.random_range(0..background_len);
runnable = state.background.swap_remove(ix);
```

### The Race Condition

When "." is typed:
1. `handle_input` is called synchronously
2. This spawns async tasks to send the completion request to the LSP
3. The test then calls `set_request_handler(...).next().await`

**The race**: Do the completion request tasks run BEFORE or AFTER the handler is registered?

- If handler registered first → request finds handler → **PASS**
- If request sent first → no handler → request goes to `on_unhandled_notification` → no response sent → timeout → **HANG**

### What Happens When No Handler Exists

In `FakeLanguageServer::new()`:
```rust
move |msg| {
    // Log the unhandled message
    notifications_tx.try_send((msg.method.to_string(), ...)).ok();
    true  // Claims to have handled it, but sends NO RESPONSE
}
```

The request caller waits forever for a response that never comes.

## Why Seed 0 Behavior Changed

The test is "deterministic per seed per code version" - meaning seed 0 on commit A might pass while seed 0 on commit B might hang. This is NOT true non-determinism.

The task ordering is fully determined by the seed, but **which tasks exist** depends on the code. When code changes add/remove/reorder async operations, the task set changes, and the RNG produces a different effective ordering.

### Timeline of Relevant Changes

| Date | Commit | Change | Impact |
|------|--------|--------|--------|
| Nov 17 | `fd1494c31a` | Broken test introduced | CI passes (seed 0 worked at this point) |
| Dec 6 | `16666f5357` | Changed `rust_lang()` to richer version | Added outline/indent/bracket queries |
| Dec 11 | `95dbc0efc2` | Priority scheduler added | Added `Priority` parameter to dispatch |
| Dec 11 | `ecb8d3d4dd` | Reverted priority scheduler | |
| Dec 11 | `5a6198cc39` | Added `await_on_background` | Real `Condvar` for blocking |
| Dec 12 | `636d11ebec` | Re-added priority scheduler | Added `spawn_realtime` with real threads |
| Dec 14 | `5b970f5d85` | **FIX**: handler ordering + panic in `spawn_realtime` | Test now passes |

### Key Change: `rust_lang()` Refactor

Commit `16666f5357` changed tests to use a richer `rust_lang()` with:
- Outline queries
- Indent queries
- Bracket queries
- Text object queries
- Highlight queries

This likely spawned additional async tasks during buffer operations, changing the task set and thus the effective ordering for seed 0.

## Confirmed Behavior with Logging

With the fix applied, running `SEED=0 cargo test -p collab test_collaborating_with_completion -- --nocapture`:

```
[FAKE_LSP] Registering handler for: textDocument/completion
[FAKE_LSP] Registering handler for: textDocument/completion
[TEST] About to type '.' trigger character
[TEST] Typed '.' - now running until parked
[FAKE_LSP] Unhandled request/notification: method=textDocument/didChange id=None
[LSP_STORE] Sending LSP request: lsp_types::request::Completion to server 0
[FAKE_LSP] Handler invoked for: textDocument/completion
[TEST] First completion handler CALLED
[LSP_STORE] Sending LSP request: lsp_types::request::Completion to server 1
[FAKE_LSP] Handler invoked for: textDocument/completion
[TEST] Second completion handler CALLED
```

The handlers are registered BEFORE typing, so when the completion request arrives, a handler exists to receive it.

## The Fix (Commit `5b970f5d85`)

### 1. Test Ordering Fix

Moved handler setup BEFORE typing the trigger character:

```rust
// Set up handlers FIRST
let mut first_completion_request = fake_language_server
    .set_request_handler::<lsp::request::Completion, _, _>(...);
let mut second_completion_request = second_fake_language_server
    .set_request_handler::<lsp::request::Completion, _, _>(...);

// THEN type the trigger character
editor_b.update_in(cx_b, |editor, window, cx| {
    editor.handle_input(".", window, cx);
});

// Now wait for requests (they will find the handlers)
first_completion_request.next().await.unwrap();
second_completion_request.next().await.unwrap();
```

### 2. `spawn_realtime` Panic

Made `TestDispatcher::spawn_realtime` panic to prevent future non-determinism:

```rust
fn spawn_realtime(&self, _priority: crate::RealtimePriority, _f: Box<dyn FnOnce() + Send>) {
    panic!(
        "spawn_realtime is not supported in TestDispatcher. \
        Real OS threads break test determinism - tests would become \
        flaky and unreproducible even with the same SEED. \
        Use a different Priority (High, Medium, Low) instead."
    );
}
```

## Lessons Learned

1. **Test ordering matters**: When testing async code with fake servers, set up handlers BEFORE triggering the requests.

2. **Seeded randomness ≠ reproducibility across versions**: A seeded test is reproducible for a fixed codebase, but code changes alter the task set and thus the effective ordering.

3. **Real threads break test determinism**: Any use of real OS threads (`std::thread::spawn`, real `Condvar`, etc.) in test code paths can cause flaky tests.

4. **`#[gpui::test]` without iterations only tests seed 0**: Consider using `#[gpui::test(iterations = 10)]` for race-prone tests to catch ordering issues.

## Debugging Tools

```bash
# See task scheduling with detailed logging
DEBUG_SCHEDULER=1 cargo test -p collab test_collaborating_with_completion -- --nocapture

# Run with specific seed
SEED=0 cargo test -p collab test_collaborating_with_completion

# Run multiple iterations to find failing seeds
ITERATIONS=100 cargo test -p collab test_collaborating_with_completion

# Show pending task traces on parking panic
PENDING_TRACES=1 cargo test -p collab test_collaborating_with_completion
```

## Files Referenced

- `crates/gpui/src/platform/test/dispatcher.rs` - TestDispatcher implementation
- `crates/gpui/src/executor.rs` - Task spawning and `await_on_background`
- `crates/lsp/src/lsp.rs` - `FakeLanguageServer` and request handling
- `crates/collab/src/tests/editor_tests.rs` - The test itself
- `crates/project/src/lsp_store.rs` - Completion request flow