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

## CRITICAL ISSUE: extension_host test hang (UNRESOLVED)

**Test**: `extension_host::extension_store_test::test_extension_store_with_test_extension`

**Symptom**: Test hangs indefinitely awaiting `fake_servers.next()` for the first fake LSP server spawn. This causes CI timeouts (60min) on all platforms.

### What we've learned

#### 1) The scheduler semantic change broke `run_until_parked`
With the new `TestScheduler`, `tick()` only processes runnable tasks and expired timers but **does not advance time**. This meant:
- `run_until_parked` (which loops `while tick() {}`) would stop when no runnables remain
- but `has_pending_tasks()` would still be true if timers were pending
- tests expecting "drain all progress" semantics would stall

**Fix applied**: `gpui::BackgroundExecutor::run_until_parked` now:
- ticks all runnable work
- calls `advance_clock_to_next_timer()` when no runnables remain
- repeats until no runnables and no timers
- this restores historical "drain everything that can make progress" semantics

#### 2) The LSP startup task IS running and calling `create_fake_language_server`
Instrumentation shows:
- `[project::lsp_store][start_language_server] attempting name="gleam"`
- `[project::lsp_store][start_language_server] resolved_binary ...`
- `[project::lsp_store][start_language_server] attempting_fake ...`
- `[project::lsp_store][start_language_server] using_fake ...`
- `[language::language_registry] create_fake_language_server: called name=gleam id=0 generation=0 ...`

So the scheduler IS progressing the task. The fake server IS being created.

#### 3) The fake server registration is NOT being overwritten
Added a `generation: u64` counter to `FakeLanguageServerEntry` to detect overwrites:
- test registers: `generation=0`
- `create_fake_language_server` uses: `generation=0`
- no "overwriting existing fake server registration" warnings appeared

So it's the same entry.

#### 4) But the receiver still gets nothing
The test's `await fake_servers.next()` times out after 10s, even though:
- `create_fake_language_server` is called
- it does `tx.unbounded_send(fake_server.clone())`

This points to: **the sender and receiver are not paired** (different channels).

### Leading hypothesis: Two LanguageRegistry instances

The most likely explanation:

1. Test calls `Project::test(...)` which creates a `LanguageRegistry` internally (`LanguageRegistry::test(cx.executor())`)
2. Test extracts `language_registry = project.languages().clone()` — this is an `Arc<LanguageRegistry>`, so it's a reference to the same registry
3. Test calls `language_registry.register_fake_lsp_server(...)` on that registry, getting a receiver
4. **Somewhere during extension initialization or LSP startup**, a NEW `LanguageRegistry` is created (or the project's registry gets replaced)
5. When `LocalLspStore::start_language_server` calls `this.languages.create_fake_language_server(...)`, it's using a **different** registry instance
6. That different registry has no fake server entries, OR has a different channel pair

Result: sender and receiver are on different channels → send succeeds (or returns None) but receiver never gets the value.

### Next steps for investigation

#### STEP 1: Prove/disprove the "two registries" hypothesis

Add a unique identity marker to each `LanguageRegistry` instance:

```rust
// In LanguageRegistry or LanguageRegistryState
#[cfg(any(test, feature = "test-support"))]
registry_id: u64,  // assigned from AtomicU64::fetch_add
```

Log it in:
- `LanguageRegistry::new()` / `LanguageRegistry::test()` (when created)
- `register_fake_lsp_server` (when test registers)
- `create_fake_language_server` (when LSP store creates)
- In the test, right after `let language_registry = project.languages().clone()`

If registry IDs differ between registration and creation: **confirmed different instances**.

#### STEP 2: If confirmed, trace where the new registry comes from

Likely suspects:
- `language_extension::init(...)` — check if it creates or replaces the language registry
- Extension loading / reloading paths in `ExtensionStore`
- Project initialization race: does something async replace `project.languages` after `Project::test` returns?

Key code paths to audit:
- `crates/language_extension/src/language_extension.rs` — does `init` create a new registry?
- `crates/extension_host/src/extension_store.rs` — does extension loading mutate/replace the language registry?
- `crates/project/src/lsp_store.rs` — is `this.languages` actually a stable reference to the project's registry?

#### STEP 3: Fix options (once root cause is known)

**Option A**: If extension init replaces the registry:
- Ensure test uses the registry **after** extension init, not before
- Or ensure extension init mutates the existing registry instead of replacing it

**Option B**: If LSP store has a stale/cloned registry:
- Ensure `LspStore` holds an `Arc<LanguageRegistry>` that points to the same instance the test registered with
- Avoid cloning registries; always pass `Arc<LanguageRegistry>` by reference

**Option C**: If the issue is test ordering:
- Move `register_fake_lsp_server` to happen AFTER all initialization (extension load, language_extension::init, etc.)
- Currently test does: register → install_dev_extension → open_buffer
- Try: install_dev_extension → register → open_buffer

### Files with instrumentation added (can be cleaned up after fix)

- `crates/gpui/src/executor.rs` — `GPUI_RUN_UNTIL_PARKED_LOG=1` logging (keep minimal version)
- `crates/language/src/language_registry.rs` — generation counter, registration/creation logs
- `crates/project/src/lsp_store.rs` — LSP startup detailed logs
- `crates/extension_host/src/extension_store_test.rs` — per-await timeouts, progress markers

### Reference logs

Detailed logs captured in `/tmp/ci-hang/`:
- `generation_debug.log` — shows generation=0 on both registration and creation
- `generation_debug2.log` — includes scheduler debug output
- `run_until_parked_debug.log` — shows run_until_parked correctly draining after fix

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