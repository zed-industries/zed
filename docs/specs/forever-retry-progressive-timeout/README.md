# Forever-Retry with Progressive Timeout

Retry all retryable LLM completion errors indefinitely, with a linearly
incrementing timeout: 1s → 2s → 3s → ... → 99s → 100s, then cap at 100s for
all subsequent attempts.

## Motivation

The current retry system caps attempts at small hard limits (`MAX_RETRY_ATTEMPTS = 4`,
`max_attempts: 2` or `3` for many error types). When an upstream provider has a
prolonged outage or rate-limits aggressively, the agent gives up and surfaces an
error to the user — even though the error is transient and would resolve if we
just kept waiting. Users on unstable connections or shared infra frequently hit
this wall.

The fix: **never stop retrying retryable errors**. Instead, back off linearly
each attempt so the provider has time to recover, but never give up.

## Documents

- [`requirements.md`](requirements.md) - Functional and non-functional requirements
- [`design.md`](design.md) - Detailed architecture and code changes
- [`tasks.md`](tasks.md) - Ordered implementation task list

## Retry Delay Sequence

```
attempt  1 → 1s
attempt  2 → 2s
attempt  3 → 3s
...
attempt 99 → 99s
attempt 100 → 100s
attempt 101 → 100s  (capped)
attempt N → 100s    (capped)
```

## Quick Start

1. Replace `RetryStrategy::Fixed` and `RetryStrategy::ExponentialBackoff`
   with `RetryStrategy::Progressive`.
2. Remove `max_attempts` from all retryable error branches.
3. Cap delay at 100s.
4. Only stop retrying when the turn is cancelled (user sends new message
   or manually cancels).

## Status

_Spec complete. Ready for implementation._

## Changelog

- **v1**: Initial spec. Linear progressive timeout 1s→100s. No max attempts on retryable errors.
