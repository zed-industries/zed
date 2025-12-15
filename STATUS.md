# STATUS — Scheduler integration (current branch state)

Branch: `scheduler-integration`

PR: https://github.com/zed-industries/zed/pull/44810

This file is intended to capture what changed on this branch, what we learned while validating it, and what decisions were made to reduce risk. It should help reviewers and future work pick up the thread quickly.

---

## Summary of major changes (since main)

### 1) Scheduler integration (core)
GPUI’s async execution has been unified around the `scheduler` crate:
- GPUI background/foreground executors delegate task scheduling, blocking, and timers through the scheduler abstraction.
- `TestScheduler` is used in tests to provide deterministic scheduling and deterministic time control.
- `PlatformScheduler` is used in production builds to route scheduling through platform dispatchers.

This branch touched high-risk areas:
- task scheduling and wakeup behavior
- timer semantics
- test determinism and ordering

---

## Testing-driven fixes and what we learned

### A) Do not cache `gpui::Entity<T>` in process-wide statics (OnceLock/static)
We reproduced and fixed a real failure mode where a process-wide cached `Entity<T>` (from one `App` context) was used from a later test `App` context. This can cause:
- “used a entity with the wrong context” panics
- `Option::unwrap()` failures in leak-detection / entity clone paths
- ordering-dependent flakes (whichever test initializes the static first “wins”)

Fix applied: avoid caching `Entity<T>` globally; construct per-`App` instead (store plain configuration data globally if needed).

### B) TestScheduler timeouts are not purely duration-based: `timeout_ticks` matters
We learned that in `TestScheduler`, “timeout” behavior depends on an internal tick budget (`timeout_ticks`) when a timeout is present. During the allotted ticks, the scheduler can poll futures and step other tasks. This means:
- a future can sometimes complete “within a timeout” in tests due to scheduler progress, even if you didn’t explicitly advance simulated time
- if a test needs deterministic timeout behavior, it must constrain the tick budget

We added/updated a scheduler regression test demonstrating:
- a timed-out future must remain pollable to completion later
- deterministic time advancement should be explicit (e.g. `advance_clock(...)`)
- deterministic timeout behavior can be enforced by setting `timeout_ticks` to `0..=0` for the test

---

## Realtime priority decision (important)

### Realtime priority has been removed (for now)
Even though a realtime-priority implementation historically existed (dedicated OS thread + bounded channel feeding runnables), we removed realtime priority entirely from:
- `scheduler::Priority`
- GPUI’s public API surface
- `PlatformDispatcher` trait and its platform implementations (mac/linux/windows/test)

Rationale:
- There were no in-tree call sites using realtime priority.
- The correctness/backpressure semantics are non-trivial for arbitrary futures:
  - blocking enqueue risks stalling latency-sensitive threads
  - non-blocking enqueue implies dropping runnables under load, which can break correctness for general futures (IO/state machines)
- Rather than ship ambiguous or risky semantics, we removed the API until there is a concrete in-tree use case and an explicitly defined contract.

This should be straightforward to reintroduce later once the semantics are agreed and tested.

CC: the prior realtime implementation on main was introduced in the “Multiple priority scheduler” work by @localcc (from blame).

---

## Current test status (local)
At the time of writing:
- `cargo test -p gpui` passed
- `cargo test -p scheduler` passed

(Other crates were not exhaustively re-run after the final realtime removal, but GPUI + scheduler are clean.)

---

## Review guidance / things to focus on
- Verify scheduler integration preserves expected semantics for:
  - blocking (`block_on`, `block_with_timeout`)
  - timer behavior (determinism in tests; routing in production)
  - task ordering changes that could affect tests or user-visible behavior
- Confirm removal of realtime priority is acceptable and doesn’t break any downstream expectations.
- Pay attention to any ordering-sensitive tests (collab/editor/etc.) that may rely on old behavior; prefer explicit synchronization in tests where possible.

---

## Future work (if/when reintroducing realtime)
Before restoring realtime priority:
- define a contract: coalescible vs must-run tasks
- define backpressure strategy and failure modes (block vs drop vs bounded secondary queue)
- guarantee scheduling callback cannot block UI / critical threads
- add deterministic tests that encode the chosen contract