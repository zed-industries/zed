# Forever-Retry with Progressive Timeout - Requirements

## Context

The Zed agent retries LLM completion errors using a combination of
`RetryStrategy::ExponentialBackoff` and `RetryStrategy::Fixed`, both capped by
small `max_attempts` values (1–4). Non-retryable errors (bad API key, payload too
large, etc.) correctly return `None` from `retry_strategy_for` and are never
retried. But **retryable** errors — rate limits, server overloads, internal
server errors, upstream provider errors — give up too soon.

In practice, users encounter prolonged transient outages where 4 retries over
~30 seconds is not enough. The agent should keep trying indefinitely for
retryable errors, backing off progressively so the provider has time to
recover.

## Goals

1. **Never give up** on retryable errors — keep retrying until the turn is
   cancelled by the user or a new message is sent.
2. **Progressive backoff** — delay increases linearly each attempt to avoid
   hammering a struggling provider.
3. **Capped delay** — timeout never exceeds 100s regardless of attempt count.
4. **Preserve non-retryable semantics** — errors that won't be fixed by
   retrying (auth, payload too large, etc.) continue to fail immediately.
5. **Minimal behavioral change** — the retry loop, cancellation flow, and
   `RetryStatus` reporting stay intact; only the strategy and attempt cap
   change.

## Non-Goals

- Changing corruption retry behavior (corruption retries are separate and
  capped at `MAX_CORRUPTION_RETRY_ATTEMPTS`).
- Introducing jitter or randomization to the delay sequence.
- Persisting retry state across Zed sessions.
- Adding user-facing settings for the cap or increment.

## Functional Requirements

### FR-1: Linear Progressive Timeout

For all retryable LLM completion errors, the delay before each retry shall be:

```
delay = min(attempt_seconds, MAX_PROGRESSIVE_DELAY_SECS)
```

Where `attempt_seconds` is the 1-based attempt number in seconds, and
`MAX_PROGRESSIVE_DELAY_SECS = 100`.

This produces the sequence: 1s, 2s, 3s, ..., 99s, 100s, 100s, 100s, ...

### FR-2: No Max Attempts on Retryable Errors

Retryable errors shall have no upper bound on the number of retry attempts.
Retrying continues until one of:

- The completion succeeds.
- The turn is cancelled (user sends a new message, manually cancels, or the
  thread is dropped).
- A non-retryable error is returned.

### FR-3: Non-Retryable Errors Unchanged

Errors that `retry_strategy_for` currently returns `None` for shall continue
to not be retried. These include:

- `AuthenticationError`
- `PermissionError`
- `NoApiKey`
- `ApiEndpointNotFound`
- `PromptTooLarge`
- `HttpResponseError` with status 401, 403, 413
- `PaymentRequired`
- `DataRetentionConsentRequired`

### FR-4: Corruption Retries Unchanged

Corruption-detection retries (`MAX_CORRUPTION_RETRY_ATTEMPTS = 2`, fixed
`BASE_RETRY_DELAY = 5s`) are a separate retry path and shall not be affected
by this change.

### FR-5: RetryStatus Reporting

`RetryStatus.max_attempts` shall report `0` when there is no limit, so the
UI can distinguish "forever retry" from "capped retry". The `attempt` and
`duration` fields continue to reflect the current attempt number and the
progressive delay.

### FR-6: Zed Cloud Provider Auto-Retry

The existing Zed Cloud condition — only auto-retry when a plan is present —
shall be preserved. Non-Zed-Cloud providers continue to always auto-retry.

## Quality Attributes

| Attribute | Target |
|-----------|--------|
| Provider-friendliness | Linear backoff avoids hammering; 100s cap prevents unreasonable waits |
| Cancellation latency | < 100ms after user cancels (existing cancellation_rx flow) |
| Memory overhead | No accumulation — single attempt counter |
| UI transparency | `RetryStatus.max_attempts = 0` signals infinite retry |
| Backward compatibility | Non-retryable errors unaffected; corruption path unaffected |

## Constraints

- Must still integrate with `Thread::run_turn_internal` and
  `retry_completion_error`.
- Must respect the existing `cancellation_rx` mechanism for prompt cancellation.
- Must not change the `acp_thread::RetryStatus` struct layout (only the
  semantic meaning of `max_attempts = 0`).

## Related Systems

- `crates/agent/src/thread.rs` — `RetryStrategy`, `retry_strategy_for`,
  `handle_completion_error`, `retry_completion_error`
- `crates/acp_thread/src/acp_thread.rs` — `RetryStatus`
- `crates/agent/src/tests/corruption_retry.rs` — corruption retry tests
  (not modified, but must not break)
