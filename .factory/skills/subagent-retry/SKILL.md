---
name: subagent-retry
description: Make subagents retry retryable LLM errors (rate limits, server errors, overloads) with backoff instead of immediately failing and propagating the error to the parent agent. Mirrors the main agent's retry_strategy_for logic so subagents are equally resilient to transient failures.
allowed-tools: Read, Write, Edit, Grep
user-invocable: false
---

# Subagent Retry: Transient Error Resilience

Today the main agent (Thread::run_turn_internal) retries retryable LLM completion errors using retry_strategy_for, but subagents do not. When a subagent's model returns a 429, 503, 500, or other transient error, the subagent immediately fails and propagates an error result to the parent. The parent then has to decide what to do — but it has lost the subagent's context and the error message is opaque. The fix is to make subagents retry the same errors the main agent retries, with the same backoff semantics, before giving up.

## Problem

When `SpawnAgentTool::run` calls `subagent.send`, the subagent's Thread runs its own turn internally via `run_turn_internal`, which already has retry logic. However, the `SubagentHandle::send` path in `NativeAgentConnection` converts LLM errors into `SubagentPromptResult::Error(String)` at L3336 in agent.rs, then immediately returns `Err(anyhow!("{message}"))` at L3367. The subagent's internal Thread may have already retried — but errors that escape the thread's max retry count, or errors that hit the subagent outside the stream (e.g. the stream itself errors mid-way), are not retried at the SpawnAgentTool level.

The existing test `test_subagent_error_propagation` only tests a non-retryable error (PromptTooLarge), confirming the subagent correctly fails fast on that. But there is no test for retryable errors on subagents, and the observed behavior is that they also fail fast — meaning a transient 429 on a subagent kills the whole delegated task.

## Solution

Add retry logic to the subagent send path that mirrors `Thread::retry_strategy_for`. When the subagent returns an error, classify it as retryable or non-retryable. For retryable errors, retry the subagent send (re-prompt the subagent with the same message, or resume its session) up to the configured max attempts with backoff, before returning the error to the parent.

### Retryable Errors

These are the same error classes `retry_strategy_for` already defines on Thread. Subagents should treat them identically:

| Error | Strategy | Max Attempts |
|-------|----------|-------------|
| 429 Too Many Requests | Fixed delay (respect retry-after) | 4 |
| 503 Service Unavailable | Fixed delay | 4 |
| 500 Internal Server Error | Fixed delay | 3 |
| Upstream 529 (overloaded) | Fixed delay | 4 |
| Other 4xx/5xx (not 401/403/413) | Fixed delay | 2 |
| RateLimitExceeded | Fixed delay (respect retry-after) | 4 |
| ServerOverloaded | Fixed delay (respect retry-after) | 4 |
| ApiInternalServerError | Fixed delay | 3 |
| ApiReadResponseError | Fixed delay | 3 |
| HttpSend | Fixed delay | 3 |
| DeserializeResponse | Fixed delay | 3 |
| BadRequestFormat | Fixed delay | 3 |
| StreamEndedUnexpectedly | Fixed delay | 1 |
| SerializeRequest / BuildRequestBody | Fixed delay | 1 |

### Non-Retryable Errors (fail immediately)

- 401 Unauthorized, 403 Forbidden, 413 Payload Too Large
- AuthenticationError, PermissionError, NoApiKey
- ApiEndpointNotFound, PromptTooLarge
- User cancellation (SubagentPromptResult::Cancelled)

---

## Implementation Plan

### Phase 1: Extract retry classification from Thread

File: `crates/agent/src/thread.rs`

1. Make `retry_strategy_for` a free function (or move it to a shared module) so it can be called from both Thread and the subagent send path. Currently it is `fn retry_strategy_for(error: &LanguageModelCompletionError) -> Option<RetryStrategy>` on Thread.

2. Alternatively, add a public method `Thread::is_retryable_error(error: &LanguageModelCompletionError) -> bool` that returns `retry_strategy_for(error).is_some()`. This is simpler and sufficient for the subagent path — the subagent's own Thread will handle the actual backoff timing during the retry attempt.

### Phase 2: Add retry loop to subagent send

File: `crates/agent/src/agent.rs`, method `NativeSubagentHandle::send` (around L3227-L3376)

The current flow is:

```
let result = match task.await {
    SubagentPromptResult::Completed => ...,
    SubagentPromptResult::Cancelled => Err(anyhow!("User canceled")),
    SubagentPromptResult::Error(message) => Err(anyhow!("{message}")),
    SubagentPromptResult::ContextWindowWarning => ...,
};
```

Change the `Error` branch to classify the error and retry if retryable:

```rust
SubagentPromptResult::Error(message) => {
    // Parse the error back into a LanguageModelCompletionError to classify it.
    // The error string comes from the subagent's Thread, which converts
    // LanguageModelCompletionError to anyhow::Error.
    if is_subagent_error_retryable(&message) {
        SubagentPromptAction::Retry(message)
    } else {
        SubagentPromptAction::Fail(message)
    }
}
```

Then wrap the entire send in a retry loop:

```rust
let max_attempts = ...; // from retry_strategy, or a constant like MAX_RETRY_ATTEMPTS (4)
let mut attempt: u8 = 0;
loop {
    attempt += 1;
    // ... existing send logic ...
    let result = match task.await { ... };
    match result {
        Ok(output) => break Ok(output),
        Err(error) if attempt < max_attempts && is_subagent_error_retryable(&error) => {
            let delay = retry_delay_for_attempt(attempt);
            // Wait with cancellation awareness, same pattern as Thread::retry_completion_error
            cx.background_executor().timer(delay).await;
            continue;
        }
        Err(error) => break Err(error),
    }
}
```

Key design decisions:

- Use `cx.background_executor().timer(delay)` for backoff (not `smol::Timer`), matching the GPUI timer rule in `.rules`.
- Respect cancellation: check `cancellation_rx` during the backoff wait, same as `retry_completion_error` does with `futures::select!`.
- Use exponential backoff for 429s (BASE_RETRY_DELAY * 2^attempt) and fixed delay for others, matching `retry_strategy_for` semantics.
- The subagent's own Thread already retries internally up to its max. This outer retry catches errors that escaped the inner retry (e.g. the inner retry exhausted its attempts). So the outer retry count should be small (1-2 additional attempts) to avoid very long waits.

### Phase 3: Error classification helper

Since the subagent returns errors as strings (the `SubagentPromptResult::Error(String)` variant loses the typed error), you need a way to classify them.

Option A (preferred): Change `SubagentPromptResult::Error` to carry a structured error type instead of a plain String:

```rust
enum SubagentPromptResult {
    Completed,
    Cancelled,
    ContextWindowWarning,
    Error(SubagentError),
}

enum SubagentError {
    // The subagent's Thread surfaced this LLM error
    LlmError(LanguageModelCompletionError),
    // A generic error we can't classify
    Other(String),
}
```

This lets you call `retry_strategy_for` directly on the `LanguageModelCompletionError` without string parsing.

Option B (simpler, less robust): Parse the error message string for known patterns (e.g. "429", "rate limit", "overloaded"). Fragile but avoids changing the enum.

Go with Option A.

### Phase 4: Thread changes to preserve error type

File: `crates/agent/src/thread.rs`

When the subagent's Thread's `run_turn_internal` exhausts retries on a retryable error, the error is currently converted to `anyhow::Error` and bubbled up. The error chain needs to preserve the original `LanguageModelCompletionError` so the outer retry can classify it.

In the retry exhaustion path (around L2607-2617 in thread.rs), the error is already an `anyhow::Error` wrapping a `LanguageModelCompletionError`. Ensure that when the subagent's turn fails, the `LanguageModelCompletionError` is available to the caller. This might already work via `error.downcast::<LanguageModelCompletionError>()` — verify this.

### Phase 5: Update SpawnAgentTool

File: `crates/agent/src/tools/spawn_agent_tool.rs`

The `run` method (L173) calls `subagent.send(input.message, cx).await`. This already returns `Result<String>`. The retry logic from Phase 2 means that by the time this returns an error, it has already been retried the appropriate number of times. No changes needed here beyond what Phase 2 provides.

### Phase 6: Telemetry

Add telemetry to track subagent retries so you can measure the impact:

```rust
telemetry::event!(
    "Subagent Retry",
    subagent_session = session_id.to_string(),
    attempt,
    error = error.to_string(),
);
```

This mirrors the existing `"Subagent Completed"` event at L180-184 of spawn_agent_tool.rs.

---

## Tests

### 1. Retryable error on subagent triggers retries

File: `crates/agent/src/tests/mod.rs`

Add `test_subagent_retry_on_retryable_error`:

- Set up a parent thread with a subagent, same as existing `test_subagent_error_propagation`.
- Instead of a non-retryable error (PromptTooLarge), send a retryable error (e.g. UpstreamProviderError with 429).
- Verify the subagent retries (the subagent's model receives multiple completion requests).
- After max retries, verify the error propagates to the parent.
- Assert the subagent ultimately leaves the running_subagent_ids set.

### 2. Retryable error on subagent succeeds after retry

Add `test_subagent_retry_succeeds_after_transient_error`:

- Same setup, but after 1-2 retryable errors, the subagent's model returns a successful completion.
- Verify the subagent completes successfully with output.
- Verify the parent receives the tool result as success.

### 3. Non-retryable error on subagent still fails immediately

This is already covered by `test_subagent_error_propagation` (which uses PromptTooLarge). Verify it still passes after the changes.

### 4. Subagent retry respects cancellation

Add `test_subagent_retry_cancelled_on_parent_cancel`:

- Send a retryable error from the subagent's model.
- During the retry backoff, cancel the parent turn.
- Verify the subagent's retry exits promptly without waiting the full backoff.

---

## Constants

Reuse existing constants from `thread.rs`:

- `BASE_RETRY_DELAY` — initial backoff delay
- `MAX_RETRY_ATTEMPTS` — max retry attempts (4)

For the outer subagent retry (on top of the inner Thread retry), use a smaller cap (e.g. 2 additional attempts) to avoid excessive total wait time. Define `MAX_SUBAGENT_OUTER_RETRY_ATTEMPTS: u8 = 2`.

---

## Files to Modify

| File | Change |
|------|--------|
| `crates/agent/src/agent.rs` | Add retry loop to `NativeSubagentHandle::send`; change `SubagentPromptResult::Error` to carry structured error |
| `crates/agent/src/thread.rs` | Move or expose `retry_strategy_for` / `RetryStrategy` for reuse from agent.rs |
| `crates/agent/src/tools/spawn_agent_tool.rs` | No logic changes needed; retry is handled at the send layer |
| `crates/agent/src/tests/mod.rs` | Add 3 new test functions for subagent retry behavior |
| `crates/agent/src/tests/corruption_retry.rs` | No changes needed (main-agent retry tests already cover the internal path) |

---

## Out of Scope

- Retry for context server tool calls inside subagents (those have their own error paths)
- Retry for sandbox authorization failures (user-actionable, not transient)
- Changing the RetryStrategy enum itself (reuse as-is)
- Retry limits on the parent agent for repeated subagent failures (out of scope for this spec)
