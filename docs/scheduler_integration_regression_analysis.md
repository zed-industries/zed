# Scheduler Integration (scheduler-integration branch) — Regression Risk Analysis & Proposed Tests

This document captures a review of the `scheduler-integration` branch compared to `main`, with a focus on:

- Problems in the **plan** that could lead to regressions
- Plan vs **implementation** mismatches
- Implementation concerns / what might go wrong / what might be missing
- Concrete **tests** worth adding to catch regressions early

The branch integrates GPUI’s async execution with the `scheduler` crate and updates test infrastructure to be deterministic via `TestScheduler`.

---

## Updates from testing / experimentation (learned weak points)

### 1) Caching `Entity<T>` in process-wide statics breaks across `App` contexts (causes panics and test flakiness)

**What happened**

While running `zed` tests, multiple failures showed up with panics like:

- `used a entity with the wrong context`
- `called Option::unwrap() on a None value` during `Entity` clone paths

The backtraces pointed at a `OnceLock<Entity<ApiKeyState>>` used by the Mistral provider for the Codestral API key entity (a process-wide singleton).

**Root cause**

`gpui::Entity<T>` is **not** a plain owned value; it is a handle tied to a particular `App`’s entity-map/context. If you store an `Entity<T>` in a process-wide static and then create a *different* `App` instance later (very common in tests, but also plausible in multi-app contexts), reusing that cached `Entity<T>` will reference the wrong entity-map.

This manifests as:
- context assertion failures (`wrong context`)
- weak `entity_map` upgrades failing (leading to `unwrap()` panics in leak-detection-only clone paths)
- nondeterministic behavior depending on test ordering (because the first `App` that initializes the static “wins”)

**Fix applied**

Avoid caching `Entity<T>` in process-wide statics. Instead:
- cache plain data (env var name, URL, etc.), and
- create the `Entity<T>` per-`App`.

In this case, the `OnceLock<Entity<ApiKeyState>>` was removed and `codestral_api_key(cx)` now constructs the entity per call / per `App`.

**Takeaway / design guideline**

> Never store `gpui::Entity<T>` (or other `App`-context-bound handles) in process-wide statics (`static`, `OnceLock`, `LazyLock`), unless the value is explicitly per-`App` or keyed by an `App` identity.

**Test recommendation**

Add (or keep) at least one regression test that creates two distinct `App` instances sequentially in the same process and exercises any global/singleton initialization paths, asserting no “wrong context” panics occur.

---

## Scope reviewed (high-level)

Major surfaces touched by this branch include:

- `crates/gpui/src/executor.rs` (large rewrite; realtime priority handling moved here)
- `crates/gpui/src/platform_scheduler.rs` (new)
- `crates/scheduler/src/{scheduler.rs,test_scheduler.rs,executor.rs}` (API and determinism plumbing)
- platform dispatchers (`crates/gpui/src/platform/*/dispatcher.rs`)
- large set of tests updated for ordering/timing (`collab`, editor inlay hints, etc.)

---

## Executive summary (what to worry about)

### Highest regression risk areas

1. **Realtime tasks (`Priority::Realtime`) implementation uses a bounded, blocking channel**
   - This can block scheduling paths and create deadlocks or latent stalls under load.
   - It can also mask failures (ignored send errors) leading to “stuck” tasks.

2. **Documentation/plan says realtime panics in tests, but runtime guard may be missing**
   - If realtime is accidentally used in tests, it could introduce nondeterminism or hangs.
   - Different platforms/test dispatchers could behave differently (panic vs silently spawning threads).

3. **Behavioral changes in test ordering**
   - Removal of label-based deprioritization and changes in scheduling semantics can subtly reorder operations.
   - Many tests were already adjusted; more order-sensitive tests may still exist, especially in collaboration and UI pipelines.

### Medium risk areas

- Timer semantics: switching to scheduler-native timers can change edge ordering and “when” callbacks occur.
- `block()` signature and semantics change (pinned future, returns bool): easy to misuse and hard to reason about without tests.
- Profiler timing recording for realtime tasks: potential double-counting or inconsistent task timing signals.

---

## Plan vs implementation mismatches

### 1) “Realtime panics in tests” is a documented promise that needs enforcement

The plan documents:

- “`Priority::Realtime` will panic in tests because real OS threads break test determinism.”

Risk if not enforced:

- A developer uses realtime priority in a test (directly or indirectly).
- Depending on platform/test dispatcher behavior, the test may:
  - become nondeterministic and flaky
  - hang (if the realtime thread waits for work / blocks)
  - pass locally and fail in CI

**What to ensure**:

- There should be an explicit guard: if the current scheduler/dispatcher indicates tests, `Priority::Realtime` should panic with a clear message.

**Why it matters**:

- This is the sort of “rare option” that will bite later: everything works until one test or feature accidentally uses it.

### 2) The plan downplays lock-ordering issues in `TestScheduler`

The plan calls out potential inconsistent lock ordering between internal mutexes (e.g. rng/state) and describes it as low priority because “single-threaded”.

Risk:

- Even if the scheduler is conceptually single-threaded, tests and supporting code can invoke methods from multiple threads.
- A deadlock from inconsistent lock ordering is catastrophic and hard to diagnose (especially in CI).

Mitigation:

- Enforce lock ordering consistently.
- Add a stress test to ensure no deadlocks if concurrent calls occur (even if “not intended”).

---

## Implementation concerns / weak points (what might go wrong)

### A) Realtime path uses a bounded, blocking channel in the scheduling closure

Observed pattern:

- realtime spawns a dedicated OS thread
- schedules `async_task::Runnable`s by sending them through `flume::bounded(1)`
- scheduling closure calls `tx.send(runnable)` and ignores the result

Risks:

1. **Blocking send in scheduling closure**
   - `send` can block if buffer is full.
   - The scheduling closure is often invoked from the async runtime / scheduler worker.
   - Blocking there can stall progress and create deadlocks (especially if the receiver needs other tasks to run to make progress).

2. **Backpressure semantics may differ from previous implementation**
   - Even if “equivalent” conceptually, the bounded buffer changes throughput behavior and can surface as latency spikes or stalls.

3. **Ignored `send` result**
   - If the receiver drops early (thread exits), send will fail.
   - Ignoring this can silently drop wakeups; tasks can hang in a “never re-polled” state.

4. **Thread lifetime / leakage**
   - If sender lives longer than expected, the thread loops forever on `recv`.
   - If the task is dropped/canceled, ensure sender drops promptly.

**Suggested mitigation**:

- Ensure the schedule path cannot block:
  - prefer `try_send` + fallback queueing strategy, or an unbounded channel
  - or push onto a lock-free queue and signal via lightweight notification
- Consider logging or metrics when send fails (debug builds at least)
- Add explicit test guards (panic in tests) as promised

### B) Profiler timing integration for realtime tasks may be inconsistent

The realtime worker thread records task timings around each runnable.

Risks:

- Double-recording (start event + end event) might differ from other execution paths
- Profiler UI or consumers might assume a single record per runnable, or might aggregate incorrectly

This may already match existing platform behavior, but is worth validating explicitly to avoid subtle regressions in profiling, performance diagnostics, or telemetry.

### C) Timer and `block()` semantics need parity tests

Scheduler changes:

- Timers now route through scheduler-native timer mechanisms
- `Scheduler::block` signature changed to accept a pinned future reference and return a completion boolean

Risks:

- Timeout boundary differences vs prior implementation (especially around “just at the deadline”)
- Futures continued after timeout could be incorrectly polled or starved
- The pinned-future API makes it easy to accidentally:
  - repoll after timeout in an invalid state
  - assume ownership moved into block (it doesn’t)
- `block()` reentrancy/session isolation behavior might differ from old GPUI behavior

---

## What could be missing (defensive checks / invariants)

1. **Explicit runtime check** for realtime in test environment (panic early)
2. **Non-blocking scheduling invariant** in realtime schedule callback
3. **TestScheduler concurrency safety**: even if not intended, tests should not deadlock if two threads touch it
4. **Timer determinism contract**: ordering and firing semantics should be documented and tested
5. **Cancellation semantics**: dropping tasks should not leak threads or leave runnables queued forever

---

## Recommended tests to add or improve

### 1) Test: realtime priority panics in tests

Goal:

- Prevent accidental nondeterministic realtime usage in tests.

Test idea:

- Build a test `App` / executor on the test dispatcher path
- Call `spawn_with_priority(Priority::Realtime(..), async { .. })`
- Assert panic message contains guidance: use `Priority::High` in tests.

Why:

- This enforces the plan’s documented contract.

### 2) Test: realtime scheduling callback does not block

Goal:

- Catch deadlock/stall potential from bounded channel and blocking `send`.

Test idea (requires controllable dispatcher/fake dispatcher or careful harnessing):

- Create a dispatcher/scheduler scenario where the realtime receiver does not consume (or consumes slowly).
- Trigger multiple wakeups/schedules.
- Assert that the scheduling path returns promptly and system continues making progress.

If hard to test deterministically today, that’s a signal to adjust the design to avoid blocking sends.

### 3) TestScheduler: priority weighting sanity + determinism

Goals:

- Ensure scheduling remains deterministic for a fixed RNG seed.
- Ensure `High` tends to run more often than `Low` in repeated selection without starvation.

Test idea:

- Spawn multiple tasks at different priorities that increment counters.
- Run a fixed number of ticks.
- Assert:
  - total run counts match expectation
  - `High` counter > `Medium` > `Low` by a margin (not strict ordering)
  - with same seed, results are stable across runs

### 4) TestScheduler: no deadlock under concurrent access (lock ordering regression test)

Goal:

- Prevent reintroducing lock ordering issues.

Test idea:

- Spawn two threads that repeatedly call methods that acquire both locks (rng and state) in different sequences.
- Run for a short bound and ensure completion (no deadlock).
- This test should be small and gated for determinism (or use timeouts).

Even if “not supported,” it prevents catastrophic CI hangs.

### 5) Timer semantics tests (determinism + ordering)

Goals:

- Ensure timers don’t fire early.
- Ensure `advance_clock()` triggers timers deterministically.
- If ordering matters, define it and test it.

Test ideas:

- Schedule timers at different durations; advance clock in steps; assert firings.
- Multiple timers at same timestamp: ensure stable ordering (or document it as unspecified and test only that all fire).

### 6) `block()` / timeout behavior tests

Goals:

- Ensure `block_with_timeout` returns `false` only on timeout.
- Ensure future can continue after timeout and still complete.
- Ensure `block()` does not accidentally run re-entrant foreground tasks for same session if session isolation is required.

Test ideas:

- A future that completes after N ticks; timeout at N-1: assert false then later true.
- A future that yields and schedules work; ensure `block` processes until ready/timeout properly.

### 7) Profiler timing consistency test (optional but valuable)

Goal:

- Ensure profiler events are consistent across dispatchers and realtime path.

Test idea:

- Spawn a task that runs a known number of runnables.
- Validate profiler receives expected event count and timestamps in non-decreasing order.

This may require internal visibility/test hooks.

---

## Suggested implementation guardrails (small changes with big payoff)

1. **Panic in tests for realtime priority**
   - Detect test scheduler/dispatcher and panic with a clear message.

2. **Avoid blocking in scheduling callback**
   - Prefer non-blocking `try_send` or an unbounded queue.
   - At minimum, ensure failures are visible in debug logs.

3. **Document timer ordering**
   - If stable ordering is desired, implement and test it.
   - Otherwise, document it as unspecified so tests don’t accidentally rely on it.

---

## Next steps (how to use this doc)

- Use this as a checklist while running CI and reviewing flake-prone areas.
- Prioritize adding:
  1) realtime-in-tests panic test
  2) timer semantics tests
  3) block/timeout continuation tests
- Consider redesigning realtime scheduling to ensure scheduling never blocks.

---