# PLAN: Reproducing and fixing the extension host LSP test hang

## Goal

Find and fix the root cause of the CI timeouts on macOS/Linux/Windows caused by a test that hangs indefinitely. The primary suspect has been identified as:

- `extension_host::extension_store_test::test_extension_store_with_test_extension`

The immediate hang is that the test awaits a fake LSP server spawn (`fake_servers.next().await`) that never arrives.

This plan is written to be executed iteratively until the hang is eliminated and CI completes within its 60-minute job timeout.

---

## High-signal symptom

Running the workspace tests (as CI does) shows one test running forever:

- `extension_host extension_store_test::test_extension_store_with_test_extension`

When isolated and run with `--nocapture`, it stalls after:
- dev extension install completes
- buffer is opened with LSP
- the extension successfully calls `latest_github_release` and `download_file` and extraction completes
- then it blocks forever awaiting the first fake LSP server: `fake_servers.next().await`

---

## Known relevant code paths

### The test hang point

In `crates/extension_host/src/extension_store_test.rs`, the hang is at:

- `let fake_server = fake_servers.next().await.unwrap();`

This receiver gets a value only when the fake server is pushed to the channel.

### Fake server emission gate

In `crates/language/src/language_registry.rs`, `create_fake_language_server` creates the fake server but only sends it to the channel **after** it receives `lsp::notification::Initialized`:

- `fake_server.try_receive_notification::<lsp::notification::Initialized>().await.is_some()`
- then `tx.unbounded_send(fake_server.clone())`

If `Initialized` never arrives, the channel never yields a server, and the test hangs forever.

### Fake server creation

`crates/project/src/lsp_store.rs` starts language servers and, in tests, attempts to create a fake server in `start_language_server`:

- it awaits `get_language_server_binary`
- then calls `languages.create_fake_language_server(...)`
- if it returns `Some(server)`, the fake server should be used instead of spawning a real process

If `create_fake_language_server` returns `None`, or if it returns `Some` but never emits the fake server to the channel (due to waiting on `Initialized` forever), the test will stall.

---

## Reproduction commands (iterate fast)

### 0) Ensure enough disk
The workspace build can consume >200GiB. If disk is full, tests won’t run reliably.

- Check disk usage (system / repo)
- If needed, run `cargo clean` to free space
- Prefer keeping logs in `/tmp` to avoid growing the repo

### 1) Run only the suspect test with a hard timeout and logs

Use a short outer timeout wrapper (macOS doesn’t ship `timeout` by default). Prefer 10–15 seconds once the build is warm.

Run:

- `cargo test -p extension_host extension_store_test::test_extension_store_with_test_extension -- --nocapture --test-threads=1`

Environment:

- `RUST_LOG=info` (or `debug` if needed)

Capture output to a log file under `/tmp/ci-hang/…`.

### 2) Confirm the hang is still at `fake_servers.next().await`

The log should show progress markers like:
- install_dev_extension completed
- opened buffer with LSP
- awaiting first fake LSP server spawn
- then it stops

If it no longer hangs there, update this plan to the new hang point and continue.

---

## Instrumentation (keep it minimal and high-signal)

Add logs that pin down whether `Initialized` ever arrives.

### A) LanguageRegistry: log fake-server lifecycle

In `crates/language/src/language_registry.rs` inside `create_fake_language_server`, add logs:

- created fake server, waiting for Initialized
- background task started
- received Initialized (then sending to channel)
- did NOT receive Initialized (never sending)

This tells whether the hang is because `Initialized` never occurs.

### B) Extension host (already useful to keep)

In `crates/extension_host/src/wasm_host/wit/since_v0_8_0.rs`, log:

- `latest_github_release` begin and success (repo/version/assets)
- `download_file` begin/request/status/done
- capability grant decision for download_file

These confirm the hang is not from download/network.

### C) The test: add progress markers around critical awaits

In `crates/extension_host/src/extension_store_test.rs`, log:

- before/after `install_dev_extension`
- before/after registering fake LSP server
- before/after `open_local_buffer_with_lsp`
- immediately before awaiting `fake_servers.next().await`
- immediately after receiving the fake server

Keep these logs even after fixing; they prevent regressions.

---

## Primary hypotheses to validate (in order)

### Hypothesis 1 (most likely): `Initialized` notification never arrives for the fake server

Evidence expected:
- LanguageRegistry logs show “created fake server … waiting for Initialized”
- but never log “received Initialized … sending”
- so fake server is never pushed to the receiver

Next steps if confirmed:
1. Identify why `Initialized` never arrives:
   - Is `LanguageServer::initialize(...)` not being called on the fake server?
   - Is it being called but not emitting `Initialized` to the fake server’s notification stream?
   - Is it blocked behind tasks that never make progress (scheduler/foreground/background starvation)?

2. Locate where the host or LSP layer emits `Initialized` for fake servers:
   - Determine whether `Initialized` is expected to be a client->server notification, a server->host notification, or synthetic for fakes.
   - Confirm the direction matches `try_receive_notification::<Initialized>()` usage.

3. Fix either:
   - Ensure fake servers receive (or synthesize) `Initialized` reliably in tests, OR
   - Stop gating “yield fake server to tests” on `Initialized`, if that guarantee is unnecessary for correctness.

### Hypothesis 2: The fake server is created, but the background task that waits for `Initialized` never runs

Evidence expected:
- Log shows “created fake server …”
- but not even “background task started …”
- or it starts but never progresses beyond awaiting notification

This suggests executor scheduling starvation or a deadlock.

Next steps:
- Confirm the background executor is ticking during the test.
- Add a small “heartbeat” runnable scheduled on background executor to verify the scheduler keeps making progress.
- Inspect any recent changes in scheduler integration that alter how test executors drive background/foreground work.

### Hypothesis 3: `create_fake_language_server` returns `None` due to missing fake server entry

Evidence expected:
- No LanguageRegistry logs from `create_fake_language_server`
- or add a log just before the `?` that fetches `fake_server_entries.get_mut(name)` to see if it’s missing

Next steps:
- Ensure fake server registration happens before starting the language server.
- Confirm server name matches exactly (`LanguageServerName("gleam")`).
- Confirm `LanguageRegistry` used by the project matches the one you registered with (no duplicate registries).

---

## Fix strategy (do not stop at “it times out less”)

### Step 1: Make the hang impossible

Short-term safety fix (acceptable if it does not weaken correctness materially):

- Modify `create_fake_language_server` so it sends the fake server into the channel **without waiting** for `Initialized`, OR it sends with a bounded wait (e.g. send immediately, and also log if Initialized never arrives).

Rationale:
- The CI hang is catastrophic (60-minute timeout across OSes).
- Most tests need the fake server handle to set request handlers and inspect binary paths; they can tolerate receiving the fake server before it’s “initialized”.

If you do this, add a comment explaining why the `Initialized` gate was removed or bounded in test mode.

### Step 2: Determine the real underlying bug

Even if the safety fix unblocks CI, keep digging:

- Why isn’t `Initialized` being observed?
- Is it a protocol direction mistake?
- Is initialization not occurring due to scheduler integration changes?
- Is there a deadlock between foreground and background execution when tests block?

### Step 3: Restore correctness (if needed)

If removing the gate causes flaky behavior:
- Replace unconditional send with:
  - send immediately to allow tests to proceed
  - separately log/assert that `Initialized` arrives eventually, with a bounded timeout under test support

This maintains correctness checks without hanging indefinitely.

---

## Validation matrix (must pass)

### Local
1. Run the single test:
   - `cargo test -p extension_host extension_store_test::test_extension_store_with_test_extension -- --nocapture --test-threads=1`
   - It must complete quickly (<30s cold, <10s warm).

2. Run extension_host crate tests:
   - `cargo test -p extension_host -- --test-threads=1`
   - Ensure no new hangs.

3. Run nextest for key suites:
   - `cargo nextest run -p extension_host -j 1 --no-fail-fast`
   - `cargo nextest run --workspace -j 1 --no-fail-fast` (expect longer, but must proceed past the prior hang)

### CI
- Verify that `run_tests_{mac,linux,windows}` no longer hit the 60-minute job timeout.
- The previously hanging test must complete, and the workflow should finish.

---

## Operational notes for fast iteration

- Prefer `-j 1` for reproducibility while isolating the hang.
- Keep hard timeouts short (10–15s once build is warm) and capture logs to `/tmp/ci-hang/`.
- After each change:
  - re-run the single test first
  - only then widen scope (crate → workspace)
- Avoid speculative refactors; only change what reduces the hang risk or improves observability.

---

## Deliverables (when solved)

- A code fix that prevents the hang and restores deterministic progress
- Updated logs removed or reduced (keep only those that are useful long-term)
- CI green on macOS/Linux/Windows without timeout
- A brief note in the PR description explaining:
  - what caused the hang
  - why the fix is correct
  - how it was validated