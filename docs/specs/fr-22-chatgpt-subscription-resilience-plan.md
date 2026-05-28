# Implementation Plan: FR-22 ChatGPT Subscription Request Resilience

## Overview

Implement the smallest reliable fix for ChatGPT Subscription stalls first: convert pre-header Codex backend hangs into retryable errors while preserving long-running SSE streams after headers arrive. Then investigate and, only if the identifier mapping is clear, add Codex `session-id` or `thread-id` headers in a separate slice.

## Architecture Decisions

- Keep the first timeout slice provider-scoped. The observed issue is on `openai-subscribed/*`, and applying a new timeout globally could change behavior for unrelated OpenAI-compatible providers.
- Model pre-header stalls as transport errors that the native agent retry loop can retry. The user should see existing retry behavior rather than a silent stuck turn.
- Clear the header timeout as soon as `client.send(request).await` returns. A slow model response body is valid for streaming Responses API calls.
- Treat Codex session headers as a follow-up unless the correct stable Zed identifier is proven in code. Guessing IDs risks worse backend behavior and harder debugging.

## Dependency Graph

```text
OpenAI Responses transport timeout helper
    |
    +-- ChatGPT Subscription provider uses timeout helper
    |       |
    |       +-- Provider-level tests prove timeout and delayed-body behavior
    |
    +-- RequestError / LanguageModelCompletionError mapping
            |
            +-- Native agent retry policy test, if needed

Session/thread identifier source investigation
    |
    +-- Optional ChatGPT Subscription Codex headers
            |
            +-- Header serialization tests
```

## Task List

### Phase 1: Pre-Header Timeout Foundation

## Task 1: Add the Responses header-timeout contract

**Description:** Introduce the smallest public contract needed for an opt-in Responses header timeout. This should define the timeout option and timeout error shape, but avoid changing default `stream_response` behavior.

**Acceptance criteria:**

- [ ] A named timeout error exists and carries the configured timeout duration.
- [ ] The existing `stream_response(...)` function remains source-compatible for existing callers.
- [ ] Any new API makes it clear that the timeout applies to response headers only.

**Verification:**

- [ ] Compile check: `cargo check -p open_ai`

**Dependencies:** None

**Files likely touched:**

- `crates/open_ai/src/responses.rs`
- `crates/open_ai/src/open_ai.rs`

**Estimated scope:** Medium: 2 files

## Task 2: Implement and test pre-header timeout behavior

**Description:** Implement the timeout around the `client.send(request).await` wait and add deterministic transport tests. `crates/open_ai/src/responses.rs` currently has no local test module, so this task should add one using `http_client::FakeHttpClient` and GPUI/background timers or controllable futures rather than live networking.

**Acceptance criteria:**

- [ ] A request whose headers never arrive returns the timeout error from Task 1.
- [ ] The timeout is cleared immediately after `client.send(request).await` returns.
- [ ] The delayed-header test fails without the timeout implementation and passes with it.
- [ ] Existing `stream_response(...)` callers still get no header timeout unless they opt in.

**Verification:**

- [ ] Tests pass: `cargo test -p open_ai responses`
- [ ] Compile check: `cargo check -p open_ai`

**Dependencies:** Task 1

**Files likely touched:**

- `crates/open_ai/src/responses.rs`

**Estimated scope:** Medium: 1 file

## Task 3: Test delayed-body streaming remains valid

**Description:** Add the positive streaming regression test: response headers arrive before the timeout, then SSE body data arrives later. This guards the exact failure mode that would incorrectly abort long-running reasoning or tool-heavy responses.

**Acceptance criteria:**

- [ ] The delayed-body test confirms the timeout does not abort after headers arrive.
- [ ] The test consumes at least one parsed `StreamEvent` from the delayed body.
- [ ] Tests do not use live network access.

**Verification:**

- [ ] Tests pass: `cargo test -p open_ai responses`

**Dependencies:** Task 2

**Files likely touched:**

- `crates/open_ai/src/responses.rs`

**Estimated scope:** Medium: 1 file

### Checkpoint: Transport Contract

- [ ] `cargo test -p open_ai responses` passes.
- [ ] `cargo check -p open_ai` passes.
- [ ] The error message is specific enough to diagnose pre-header timeout.
- [ ] No non-ChatGPT provider behavior has changed.

### Phase 2: ChatGPT Subscription Integration

## Task 4: Use the header-timeout path for ChatGPT Subscription Codex requests

**Description:** Wire the provider-scoped timeout into `OpenAiSubscribedLanguageModel::stream_completion` so Codex requests fail promptly when response headers stall.

**Acceptance criteria:**

- [ ] `openai-subscribed/*` requests use the timeout path.
- [ ] Existing headers remain present: `originator`, `OpenAI-Beta`, and `ChatGPT-Account-Id` when available.
- [ ] OAuth refresh and request limiting behavior remain unchanged.
- [ ] The timeout value is a provider-local constant unless settings exposure is explicitly approved.

**Verification:**

- [ ] Tests pass: `cargo test -p language_models openai_subscribed`
- [ ] Compile check: `cargo check -p language_models`

**Dependencies:** Tasks 1-3

**Files likely touched:**

- `crates/language_models/src/provider/openai_subscribed.rs`
- `crates/language_models/src/provider/openai_subscribed.rs` tests

**Estimated scope:** Small: 1 file

## Task 5: Verify native agent retry behavior for the timeout error

**Description:** Confirm the new timeout error is retried by `AgentThread` retry policy. If the existing mapping already lands in a retried error variant, add a focused regression test; otherwise adjust the mapping narrowly.

**Acceptance criteria:**

- [ ] A simulated header timeout produces retry events rather than an immediate terminal failure.
- [ ] Retry count and delay follow the existing transient-error conventions.
- [ ] The change does not broaden non-retryable auth or permission errors.

**Verification:**

- [ ] Tests pass: `cargo test -p agent retry`
- [ ] Compile check: `cargo check -p agent`

**Dependencies:** Task 4

**Files likely touched:**

- `crates/agent/src/thread.rs`
- `crates/agent/src/tests/mod.rs`

**Estimated scope:** Medium: 2 files

### Checkpoint: User-Visible Resilience

- [ ] `cargo test -p open_ai responses` passes.
- [ ] `cargo test -p language_models openai_subscribed` passes.
- [ ] `cargo test -p agent retry` passes if retry-policy code or tests changed.
- [ ] A stalled pre-header request now becomes a retryable failure path.

### Phase 3: Codex Session Header Investigation

## Task 6: Trace available stable session identifiers

**Description:** Inspect the native agent and language model request flow to identify whether a stable session or thread identifier reaches the ChatGPT Subscription provider. This task should produce a code comment, spec update, or small test-only finding before any header is added.

**Acceptance criteria:**

- [ ] The candidate identifier source is documented with file references.
- [ ] The identifier is stable across retries in the same agent turn.
- [ ] The identifier does not expose secrets or local filesystem paths.

**Verification:**

- [ ] Evidence captured in the PR description or spec update.
- [ ] No runtime behavior changes in this task.

**Dependencies:** None; can run in parallel with Phase 1 after branch setup

**Files likely touched:**

- `docs/specs/fr-22-chatgpt-subscription-resilience.md`
- Possibly no code files

**Estimated scope:** Small: 0-1 files

## Task 7: Add Codex session headers only if the identifier mapping is clear

**Description:** If Task 6 identifies the right stable values, pass `session-id` and optionally `thread-id` to the Codex backend for ChatGPT Subscription requests. If not, explicitly defer this in the PR.

**Acceptance criteria:**

- [ ] Headers use hyphenated names only: `session-id`, `thread-id`.
- [ ] Header values are stable across retries for one session.
- [ ] Tests assert header presence and absence of underscored aliases.

**Verification:**

- [ ] Tests pass: `cargo test -p language_models openai_subscribed`
- [ ] Compile check: `cargo check -p language_models`

**Dependencies:** Task 6

**Files likely touched:**

- `crates/language_models/src/provider/openai_subscribed.rs`
- Potentially request/thread plumbing files if a stable identifier is not already present

**Estimated scope:** Medium if identifier is present; Large if plumbing is required and should be split before implementation

### Checkpoint: Header Decision

- [ ] Header implementation is either tested or explicitly deferred.
- [ ] PR summary explains the decision and references upstream Codex/OpenCode behavior.

### Phase 4: Final Verification

## Task 8: Run final targeted verification and PR hygiene

**Description:** Run the targeted test set, inspect the final diff for scope control, and prepare PR notes including release notes and any suggested `.rules` additions only if a repeated non-obvious pattern was validated.

**Acceptance criteria:**

- [ ] All touched-crate targeted tests pass or failures are documented.
- [ ] Final diff is limited to the spec, timeout implementation, tests, and optional header work.
- [ ] PR body includes `Release Notes:` as the final section.

**Verification:**

- [ ] `cargo test -p open_ai responses`
- [ ] `cargo test -p language_models openai_subscribed`
- [ ] `cargo test -p agent retry`
- [ ] `cargo check -p open_ai -p language_models -p agent`
- [ ] `./script/clippy` if time and environment allow

**Dependencies:** Tasks 1-7 as applicable

**Files likely touched:**

- PR description only, unless final docs updates are needed

**Estimated scope:** Small: verification only

## Parallelization Opportunities

- Task 6 can be investigated independently from Tasks 1-3.
- Task 5 can be prepared after the timeout error shape is known, but should not land before Tasks 1-4.
- Task 7 should not be parallelized until Task 6 decides the identifier contract.

## Risks and Mitigations

| Risk                                                       | Impact | Mitigation                                                               |
| ---------------------------------------------------------- | ------ | ------------------------------------------------------------------------ |
| Timeout aborts valid long-running responses                | High   | Clear timeout immediately after headers arrive and add delayed-body test |
| Error maps to non-retryable `Other` unexpectedly           | Medium | Add retry-policy regression test and narrow conversion if needed         |
| Session header value is guessed incorrectly                | Medium | Make header work conditional on Task 6 evidence                          |
| Shared OpenAI transport change affects unrelated providers | Medium | Keep new timeout path opt-in and migrate only ChatGPT Subscription first |
| Tests require live network                                 | Low    | Use fake HTTP client or local deterministic harness only                 |

## Open Questions

- Should the initial timeout be exactly 10 seconds to match OpenCode, or should Zed choose a different provider constant?
- Is there already a native thread/session ID available in `LanguageModelRequest`, or would passing one require a broader request contract change?
- If retry happens after a pre-header timeout, should the retry reuse the same Codex session identifier if one is added?

## Incremental Implementation Rules

- Implement one task at a time.
- After each task, run only the verification command that covers the files changed by that task.
- Commit each passing slice separately.
- Do not mix the timeout fix and session-header work in the same commit.
- If Task 7 becomes larger than 5 files, stop and split it into a separate plan before implementation.

## Planning Gate Checklist

- [x] Every task has acceptance criteria.
- [x] Every task has a verification step.
- [x] Task dependencies are identified and ordered.
- [x] No planned task should touch more than about 5 files unless Task 7 discovers identifier plumbing is larger than expected.
- [x] Checkpoints exist between transport, provider integration, header decision, and final verification phases.
- [ ] Human has reviewed and approved the plan.
