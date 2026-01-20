# Nested Request Rate Limiting Fix

## Problem

When subagents use the `edit_file` tool, it creates an `EditAgent` that makes its own model request to get the edit instructions. These "nested" requests compete with the parent subagent conversation requests for rate limiter permits.

The rate limiter uses a semaphore with a limit of 4 concurrent requests per model instance. When multiple subagents run in parallel:

1. 3 subagents each hold 1 permit for their ongoing conversation streams (3 permits used)
2. When all 3 try to use `edit_file` simultaneously, their edit agents need permits too
3. Only 1 edit agent can get the 4th permit; the other 2 block waiting
4. The blocked edit agents can't complete, so their parent subagent conversations can't complete
5. The parent conversations hold their permits, so the blocked edit agents stay blocked
6. **Deadlock**

## Current Architecture

- `RateLimiter` is created per model instance with `RateLimiter::new(4)` (see `crates/language_models/src/provider/*.rs`)
- All requests go through `request_limiter.stream()` or `request_limiter.run()` which acquires a semaphore permit
- The permit is held for the duration of the streaming response
- Subagents share the same `Arc<dyn LanguageModel>` as their parent, so they share the rate limiter

## Proposed Solution

Nested requests (like edit agent requests spawned from within a tool call) should not count against the rate limit, since they're already "part of" a rate-limited request.

### Implementation Options

#### Option A: Add a bypass flag to the request

Add a field to `LanguageModelRequest` like `bypass_rate_limit: bool`. The `EditAgent` would set this to `true`. Model implementations would check this flag and skip the rate limiter.

```rust
// In LanguageModelRequest
pub struct LanguageModelRequest {
    // ... existing fields ...
    pub bypass_rate_limit: bool,
}

// In model implementations (e.g., cloud.rs)
fn stream_completion(&self, request: LanguageModelRequest, cx: &AsyncApp) -> ... {
    if request.bypass_rate_limit {
        // Call the API directly without rate limiting
        self.stream_completion_inner(request, cx)
    } else {
        self.request_limiter.stream(self.stream_completion_inner(request, cx))
    }
}
```

#### Option B: Use a thread-local or task-local "already rate limited" flag

When a rate-limited request starts, set a flag. Nested requests check this flag and skip rate limiting if already within a rate-limited context.

#### Option C: Separate rate limiters for different request types

Have one rate limiter for "conversation" requests and another (or none) for "tool-internal" requests like edit agent calls.

### Recommended Approach

**Option A** is the simplest and most explicit. The `EditAgent` knows it's making a nested request, so it can set the flag. This requires:

1. Add `bypass_rate_limit: bool` field to `LanguageModelRequest` (default `false`)
2. Update all model `stream_completion` implementations to check this flag
3. Set `bypass_rate_limit: true` in `EditAgent::request()` (in `crates/agent/src/edit_agent.rs`)

### Files to Modify

1. `crates/language_model/src/request.rs` - Add field to `LanguageModelRequest`
2. `crates/language_models/src/provider/*.rs` - Update all providers to check the flag (~15 files)
3. `crates/agent/src/edit_agent.rs` - Set the flag when building the request

### Testing

The existing tests in `crates/agent/src/tests/edit_file_thread_test.rs` use `FakeLanguageModel` which doesn't have rate limiting, so they won't catch rate limit issues. Consider adding an integration test that verifies multiple concurrent edit operations complete without deadlock, though this may be difficult to test deterministically.