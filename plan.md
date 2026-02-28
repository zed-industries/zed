# Plan: Fixes for Code Review Issues #1–#11 (excluding #12)

This plan tracks current progress and remaining work after the interrupted implementation session.

---

## Current Progress Snapshot

### Completed in code
- **#1 Test fix for thread target serialization round-trip**
  - Updated the failing test setup to enable the worktree feature flag.
  - Verified the test now passes (`test_thread_target_serialization_round_trip`).

- **#2 Preserve full first-send content for New Worktree**
  - Updated first-send interception to emit full message content blocks instead of plain text.
  - Updated New Worktree initialization path to pass through that full content.

- **#4 Deterministic repository classification**
  - Replaced first-match behavior with specificity-based matching (most specific path depth, deterministic tie-break).

- **#5 Open-file remap prefix collision handling**
  - Remap logic now chooses the most specific matching source root instead of first prefix match.

- **#6 Active-view update propagation**
  - Restored broader active-thread observation so panel/toolbar state updates on general server view changes, not only explicit thread-change events.

- **#7 Startup race handling for persisted `NewWorktree`**
  - Load-time validation no longer requires repositories to already be discovered for `NewWorktree`.
  - Hard constraints (feature flag and collab) still enforced at load time.

### Not yet completed
- **#3 Ensure worktree creation status cannot get stuck in `Creating`**
- **#8 Remove visual-test global observer leak**
- **#9 Explicit temp path deletion in visual test teardown**
- **#10 Fail fast on git setup command failures in visual tests**
- **#11 Restore error logging for detached `open_new` task path**

---

## Remaining Work by Item

## 3) Ensure worktree creation status cannot get stuck in `Creating`

### Goal
Guarantee every failure path exits `Creating` into a terminal state (`Error` or cleared state) so UI controls/spinners recover reliably.

### Remaining steps
- Add a catch-all failure handler around the full async creation flow (`setup_new_workspace` and call site) so unexpected errors always call `set_worktree_creation_error(...)`.
- Verify every early-return / error branch sets a terminal status (including panel/task scheduling failures).
- Add or update test coverage for:
  - failure in worktree creation receiver,
  - workspace unavailable path,
  - unexpected setup failure branch.
- Confirm UI can retry after failure without restart.

---

## 8) Remove visual-test global observer leak

### Goal
Ensure the visual test does not leave a global observer alive after completion.

### Remaining steps
- Store the observer subscription handle from the workspace observer registration in the thread-target visual test flow.
- Explicitly drop/clear that subscription during teardown.
- Re-run the relevant visual test flow to confirm isolation across repeated runs.

---

## 9) Explicitly delete preserved temp path in visual test teardown

### Goal
Prevent filesystem artifact accumulation from preserved temp dirs in visual tests.

### Remaining steps
- Keep explicit lifecycle ordering:
  1. stop/close windows and relevant background work,
  2. then recursively delete preserved temp path.
- Add best-effort cleanup with clear logging if deletion fails.
- Validate repeated visual-test runs do not accumulate leftovers.

---

## 10) Fail fast on git setup command failures in visual tests

### Goal
Make git setup failures immediate and actionable.

### Remaining steps
- Wrap each git setup command in helper(s) that:
  - check exit status,
  - include stderr/stdout context in failure message.
- Apply to init/config/add/commit commands in modified visual test flow(s).
- Keep command sequence unchanged; improve only validation and diagnostics.

---

## 11) Restore error logging for detached `open_new` task path

### Goal
Reinstate lost error visibility for detached `open_new` action path.

### Remaining steps
- Identify detached `open_new` call sites that currently use plain detach without logging in affected flow.
- Switch to detach-with-error-logging pattern where appropriate.
- Keep behavior unchanged apart from observability.
- Validate no warnings/errors are silently swallowed in that path.

---

## Validation Checklist (remaining)

- [x] Targeted `agent_ui` test for thread target serialization round-trip.
- [ ] Additional targeted `agent_ui` tests for first-send interception/content behavior.
- [ ] Broader `agent_ui` test pass for touched areas.
- [ ] Visual-test checks for thread target selector flow after #8/#9/#10.
- [ ] Confirm temp directories are cleaned up after visual test completion.
- [ ] Confirm detached `open_new` failures are visible in logs.

---

## Recommended Execution Order (updated)

1. **Finish state/lifecycle robustness**: #3  
2. **Visual test hygiene**: #8, #9, #10  
3. **Observability cleanup**: #11  
4. **Final validation pass** (targeted + broader tests)