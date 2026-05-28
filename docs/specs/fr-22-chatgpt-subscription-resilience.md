# Spec: FR-22 ChatGPT Subscription Request Resilience

## Assumptions

1. The target provider is Zed's native `ChatGPT Subscription` provider, not the OpenCode ACP integration.
2. The main user-facing failure is a native agent turn that stalls or fails when GPT models are accessed through `openai-subscribed/*`.
3. The first implementation should focus on request resilience and observability, not on changing the model list or OAuth sign-in flow.
4. Header timeout behavior should fail before response headers arrive, but must not abort a valid long-running SSE response after headers have arrived.
5. Any Codex session header change needs a local source of stable session or thread identifiers; if no stable identifier exists at the language-model layer, that part should remain a documented follow-up rather than a guessed header.

## Objective

Make ChatGPT Subscription GPT model requests fail and retry predictably when the Codex backend stalls before response headers, and determine whether Zed should send Codex `session-id` or `thread-id` headers like upstream Codex and OpenCode now do.

Users are Zed agent users who select GPT models under the ChatGPT Subscription provider. Success means long or tool-heavy agent turns either continue streaming normally, retry on transient pre-header stalls, or show an actionable error instead of appearing stuck indefinitely.

## Tech Stack

- Rust workspace in `zed`.
- GPUI foreground/background task primitives for async work.
- `http_client` crate for provider HTTP requests.
- `open_ai` crate for OpenAI Responses request/stream handling.
- `language_models` crate for provider-specific ChatGPT Subscription behavior.
- Native agent retry logic in `crates/agent/src/thread.rs`.

## Commands

- Targeted tests for ChatGPT subscription provider:
  `cargo test -p language_models openai_subscribed`
- Targeted tests for OpenAI response transport:
  `cargo test -p open_ai responses`
- Targeted native agent retry tests:
  `cargo test -p agent retry`
- Broader compile check for touched crates:
  `cargo check -p open_ai -p language_models -p agent`
- Repository lint when implementation is ready:
  `./script/clippy`

## Project Structure

- `crates/language_models/src/provider/openai_subscribed.rs` -> ChatGPT Subscription provider, OAuth token refresh, Codex request shaping.
- `crates/open_ai/src/responses.rs` -> shared OpenAI Responses HTTP request and SSE parsing.
- `crates/open_ai/src/open_ai.rs` -> OpenAI request error types and conversion into `LanguageModelCompletionError`.
- `crates/agent/src/thread.rs` -> native agent retry policy for model completion errors.
- `docs/specs/fr-22-chatgpt-subscription-resilience.md` -> this specification.
- `docs/specs/fr-22-chatgpt-subscription-resilience-plan.md` -> implementation plan and task breakdown.

## Code Style

Prefer provider-specific behavior at the provider boundary unless it is clearly reusable by more than one provider. Keep error handling explicit and preserve visibility when ignoring cleanup failures.

```rust
const CODEX_RESPONSE_HEADER_TIMEOUT: Duration = Duration::from_secs(10);

let response = request_limiter
    .stream(async move {
        stream_response_with_header_timeout(
            http_client.as_ref(),
            PROVIDER_NAME.0.as_str(),
            CODEX_BASE_URL,
            &access_token,
            responses_request,
            extra_headers,
            CODEX_RESPONSE_HEADER_TIMEOUT,
        )
        .await
        .map_err(LanguageModelCompletionError::from)
    })
    .await;
```

Use full names for variables, propagate fallible operations with `?`, and do not introduce broad abstractions until the second caller exists.

## Testing Strategy

- Add a fake HTTP client or local response harness that delays response headers and verifies the request returns a retryable timeout error.
- Add a positive test where response headers arrive promptly but SSE body data arrives later; the header timeout must be cleared and the stream must remain valid.
- Add a provider-level test proving ChatGPT Subscription uses the timeout path and still sends existing auth headers.
- Add or update a retry-policy test only if the new timeout maps to an error variant that is not already retried.
- Avoid live network tests against `chatgpt.com`; all verification should be deterministic.

## Boundaries

- Always:
  - Keep the change scoped to ChatGPT Subscription request resilience unless a shared transport helper is necessary.
  - Preserve existing OAuth refresh semantics and credential storage.
  - Ensure delayed SSE bodies are allowed after headers arrive.
  - Run targeted tests after each implementation slice.
- Ask first:
  - Adding new dependencies.
  - Changing public provider configuration or user-visible settings.
  - Sending new `session-id` or `thread-id` headers if it requires plumbing new identifiers through multiple agent layers.
  - Broadly changing retry policy for all providers.
- Never:
  - Log access tokens, refresh tokens, account IDs, or raw authorization headers.
  - Treat all request inactivity as a full-request timeout; this spec only covers pre-header stalls.
  - Disable or remove existing retry tests to make the change pass.
  - Edit generated docs or mdBook output for this internal spec.

## Success Criteria

- A ChatGPT Subscription request that does not receive response headers within the configured timeout produces a clear retryable error.
- A ChatGPT Subscription request whose headers arrive before the timeout can continue streaming body chunks indefinitely according to existing stream behavior.
- Native agent retry logic retries the timeout failure using the existing retry event path.
- Existing `reasoning.encrypted_content` behavior remains unchanged.
- The implementation includes deterministic tests for pre-header timeout and delayed-body non-timeout behavior.
- The PR notes whether `session-id` or `thread-id` headers were implemented or deferred, with source-backed rationale.

## Open Questions

- Which Zed identifier should map to Codex `session-id`: ACP session ID, native thread/session database ID, or a generated per-request/group ID?
- Does Codex need `thread-id` for the ChatGPT subscription endpoint in Zed's stateless `store=false` flow, or is `session-id` sufficient?
- Should the timeout value be hard-coded for this provider first, or exposed through settings after the behavior is proven?
