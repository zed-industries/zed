# Forever-Retry with Progressive Timeout - Implementation Tasks

---

## T1: Replace RetryStrategy enum

**Files**: `crates/agent/src/thread.rs`

**Description**: Replace `RetryStrategy::ExponentialBackoff` and
`RetryStrategy::Fixed` with a single `RetryStrategy::Progressive` variant.
Add the `MAX_PROGRESSIVE_DELAY_SECS` constant.

**Acceptance Criteria**:
- [ ] `RetryStrategy::ExponentialBackoff` and `RetryStrategy::Fixed` variants removed
- [ ] `RetryStrategy::Progressive` variant added (no fields)
- [ ] `pub(crate) const MAX_PROGRESSIVE_DELAY_SECS: u64 = 100;` added
- [ ] `BASE_RETRY_DELAY` constant preserved (used by corruption path)
- [ ] Compilation succeeds after enum change (callers will fail — that's expected for T2)

---

## T2: Update retry_strategy_for

**Files**: `crates/agent/src/thread.rs`

**Description**: Change every `Some(RetryStrategy::Fixed { ... })` and
`Some(RetryStrategy::ExponentialBackoff { ... })` branch in
`retry_strategy_for` to return `Some(RetryStrategy::Progressive)`. All `None`
branches remain unchanged.

**Acceptance Criteria**:
- [ ] All `Some(...)` arms return `Some(RetryStrategy::Progressive)`
- [ ] All `None` arms unchanged (auth, payload too large, payment required, etc.)
- [ ] Match arm structure preserved for readability
- [ ] No `max_attempts` or `delay` values in any return

---

## T3: Update handle_completion_error

**Files**: `crates/agent/src/thread.rs`

**Description**: Remove the `max_attempts` check and the per-strategy delay
computation. Compute delay as `min(attempt, MAX_PROGRESSIVE_DELAY_SECS)`
seconds. Set `max_attempts: 0` in the returned `RetryStatus`.

**Acceptance Criteria**:
- [ ] No `max_attempts` comparison / early return
- [ ] Delay computed as `Duration::from_secs((attempt as u64).min(MAX_PROGRESSIVE_DELAY_SECS))`
- [ ] `RetryStatus.max_attempts` set to `0`
- [ ] `RetryStatus.duration` set to computed progressive delay
- [ ] Zed Cloud auto-retry condition preserved
- [ ] Non-retryable error path (returns `Err`) preserved

---

## T4: Update retry_completion_error

**Files**: `crates/agent/src/thread.rs`

**Description**: Verify no changes are needed. The function delegates to
`handle_completion_error` and uses the returned `retry.duration` for the timer.
Confirm it still works with the new return shape.

**Acceptance Criteria**:
- [ ] No structural changes to `retry_completion_error`
- [ ] Timer uses `retry.duration` (progressive delay from T3)
- [ ] Cancellation check via `cancellation_rx` still works
- [ ] Returns `Continue(())` to signal retry

---

## T5: Update run_turn_internal attempt counter

**Files**: `crates/agent/src/thread.rs`

**Description**: Verify the attempt counter increment in `run_turn_internal`
works correctly with infinite retries. The counter should keep incrementing
without overflow (u8 wraps at 255; using u16 or u32 if needed).

**Acceptance Criteria**:
- [ ] Attempt counter type supports > 4 retries (u8 is fine up to 255; verify no logic breaks if attempt > 4)
- [ ] No `if attempt > max_attempts` check remains in the turn loop
- [ ] Corruption retry counter and cap remain unchanged

---

## T6: Add tests for forever-retry behavior

**Files**: `crates/agent/src/tests/mod.rs` (or new test file)

**Description**: Add integration tests verifying:
- Retryable error retries indefinitely (5+ attempts without giving up)
- Delay increases linearly (1s, 2s, 3s, ...)
- Delay caps at 100s
- Non-retryable error fails immediately (no retry)
- Cancellation during retry delay exits cleanly
- `RetryStatus.max_attempts == 0` for forever-retry

**Acceptance Criteria**:
- [ ] Test: 5 consecutive `UpstreamProviderError` retries → each with correct progressive delay
- [ ] Test: delay at attempt 100 is 100s
- [ ] Test: delay at attempt 101 is still 100s (capped)
- [ ] Test: `AuthenticationError` → no retry, error propagated
- [ ] Test: cancellation during retry → exits turn
- [ ] Test: `RetryStatus.max_attempts == 0`

---

## T7: UI considerations for max_attempts = 0

**Files**: `crates/acp_thread/` (rendering code)

**Description**: Verify the UI handles `max_attempts = 0` gracefully. If the
retry status UI currently shows "attempt 2/4", it should show "attempt 2"
(without the "/4" part) when `max_attempts` is 0.

**Acceptance Criteria**:
- [ ] Retry status rendering handles `max_attempts == 0` without panicking
- [ ] Shows "Retrying (attempt 2)..." instead of "Retrying (attempt 2/4)..."
- [ ] No division-by-zero or off-by-one from `max_attempts == 0`

---

## T8: Clean up dead code

**Files**: `crates/agent/src/thread.rs`, `crates/agent/src/tests/`

**Description**: Remove `MAX_RETRY_ATTEMPTS` constant and any other dead code
left over from the old retry strategy. Update references in test files.

**Acceptance Criteria**:
- [ ] `MAX_RETRY_ATTEMPTS` constant removed
- [ ] No remaining references to `ExponentialBackoff` or `Fixed` strategy variants
- [ ] All existing tests pass (corruption retry tests, retry cancellation tests)
- [ ] `./script/clippy` passes
