# OAuth MCP handover for next session

This document captures the current state of the branch, what I learned by reviewing both this branch and `origin/mcp-auth`, what I already changed, what is currently broken, and the intended plan to finish the work cleanly.

It is meant to be detailed enough that the next session can reconstruct the design without redoing all of the reading.

## High-level verdict

The current branch has the better overall product and lifecycle design than `origin/mcp-auth`:

- explicit `AuthRequired` and `Authenticating` states,
- loopback callback on `127.0.0.1`,
- random OAuth `state` validation,
- startup 401 -> `AuthRequired` transition,
- no custom-scheme callback plumbing.

That should remain the basis.

`origin/mcp-auth` had a better encapsulation boundary and a better persistence/session story:

- more auth logic hidden behind the transport/session layer,
- a persisted auth/session blob,
- expiry-aware refresh.

The right outcome is a hybrid:
- keep this branch’s lifecycle model and loopback callback design,
- steal the stronger session persistence / unified provider idea,
- reduce how much low-level transport assembly `ContextServerStore` does.

## Branch status at handoff

At the moment, I modified:

- `crates/context_server/src/oauth.rs`
- `crates/context_server/src/transport/http.rs`

These two files are in good shape structurally and currently had no diagnostics when last checked.

I did not successfully finish the `crates/project/src/context_server_store.rs` refactor before the context window got tight. That file still contains code written against the old API, and it currently has compile errors because `oauth.rs` changed underneath it.

The current working tree has local modifications and is not in a buildable state until `context_server_store.rs` and downstream consumers are updated.

## The original issues I identified

These were the important issues found in the branch before starting edits.

### 1. DCR registration consistency bug

Before the edits, the branch did DCR twice:

- once during discovery using a placeholder redirect URI,
- once during the actual auth flow using the real loopback redirect URI.

The problem was that the second registration result was not propagated consistently into refresh behavior and persisted state.

Symptoms in the old code:
- token exchange used the second `client_registration`,
- refresh still used `discovery.client_registration`,
- cached DCR registration could remain stale.

Relevant old references:
- `crates/project/src/context_server_store.rs` around `run_oauth_flow`.
- `crates/context_server/src/oauth.rs` old `discover()` behavior.

### 2. `invalid_token` treated like invalid client registration

The branch used `WWW-Authenticate: Bearer error="invalid_token"` as a signal that DCR registration might be invalid, skipping refresh and clearing the DCR cache.

That is too aggressive and likely wrong. `invalid_token` usually means access token expiry/revocation/malformed token, which is exactly where refresh should still be attempted.

Relevant old references:
- `crates/context_server/src/oauth.rs`, `BearerError::indicates_invalid_client`.
- `crates/context_server/src/transport/http.rs`, 401 path.
- `crates/project/src/context_server_store.rs`, discovery / DCR cache clearing path.

### 3. Discovery leaked into public status

Public `ContextServerStatus` exposed `AuthRequired(Arc<OAuthDiscovery>)`.

That forced custom `PartialEq`, `Eq`, and `Hash`, and the UI did not actually need the payload.

Relevant file:
- `crates/project/src/context_server_store.rs`.

### 4. Split token provider design

The branch had:
- `McpOAuthTokenProvider`
- `StaticTokenProvider`

The static provider existed because startup only restored tokens, not a full refresh-capable session.

That led to an awkward design where post-auth and startup used different provider paths.

### 5. DCR cache key used resource URL, not auth issuer

The branch keyed cached DCR registration using the MCP server URL, but DCR registrations belong conceptually to the authorization server.

This should be keyed by auth server issuer instead.

## Architectural comparison with `origin/mcp-auth`

This mattered because some of the requested improvements came from that comparison.

### What this branch does better

- explicit auth lifecycle states instead of just “running with substatus”.
- loopback callback instead of custom URL callback.
- real random `state` validation.
- handles startup auth-required more naturally.

### What `origin/mcp-auth` did better

- persists more complete auth/session state.
- uses expiry-aware refresh.
- pushes more auth mechanics behind a smaller API boundary.

### Recommendation

Do not move back toward the custom-scheme callback design from `origin/mcp-auth`.

Do move toward:
- full persisted session,
- unified refresh-capable provider,
- less low-level transport construction inside the store.

## What I already changed

## 1. `crates/context_server/src/oauth.rs`

This file was refactored significantly.

### New or changed data model

`OAuthDiscovery` is now discovery-only. It no longer includes `client_registration`.

New persisted session type:

- `OAuthSession`
  - `discovery: OAuthDiscovery`
  - `client_registration: OAuthClientRegistration`
  - `tokens: OAuthTokens`

Also:
- `ProtectedResourceMetadata` and `AuthServerMetadata` now derive `Serialize` / `Deserialize` so they can be stored as part of `OAuthSession`.
- `OAuthDiscovery` also now derives `Serialize` / `Deserialize`.

### DCR changes

Placeholder DCR during discovery was removed.

New flow:
- `discover()` now only does:
  - protected resource metadata discovery,
  - auth server metadata discovery,
  - PKCE support validation,
  - registration strategy availability validation,
  - scope selection.
- `resolve_client_registration()` was added to do CIMD or DCR only when the real redirect URI is known.

This is the correct shape for fixing the DCR inconsistency.

### DCR cache key helper

Added:

- `dcr_registration_cache_key(auth_server_issuer: &Url) -> String`

This derives the DCR cache key from the auth server issuer, not the MCP resource URL.

### Invalid token fix

`BearerError::indicates_invalid_client()` was changed so that:

- `InvalidToken` is **not** treated as invalid client.
- only `Other` currently returns true.

This is intentionally conservative.

### Unified provider work

`StaticTokenProvider` was removed.

`McpOAuthTokenProvider` now holds:

- `SyncMutex<OAuthSession>`
- `http_client`
- optional `mpsc::UnboundedSender<OAuthSession>`

Refresh now:
- reads `discovery + client_registration + tokens` from the session,
- refreshes using the correct `client_registration.client_id`,
- preserves the old refresh token if the token endpoint does not return a new one,
- emits a full refreshed `OAuthSession` through the channel.

`access_token()` now checks expiry with a 30-second buffer and returns `None` when the token is effectively expired.

That supports proactive refresh in the transport.

### Test updates already made

The tests in `oauth.rs` were updated to the new model:

- full discover tests now call `discover()` and then `resolve_client_registration()`.
- tests for cached DCR registration now target `resolve_client_registration()`.
- invalid token assertions were updated.
- added a test for `dcr_registration_cache_key()`.

## 2. `crates/context_server/src/transport/http.rs`

This file was also updated.

### Proactive refresh

Before sending a request, if there is a token provider and `access_token()` returns `None`, the transport now attempts a refresh first.

This is important because with the new unified provider, restored sessions can refresh immediately on startup without needing to wait for an initial 401.

### 401 behavior

The transport no longer skips refresh just because `WWW-Authenticate` said `invalid_token`.

On 401:
- parse `WWW-Authenticate`,
- attempt refresh once if there is a provider,
- retry once,
- return `TransportError::AuthRequired` if refresh fails or retry still gets a 401.

### Test updates already made

I updated the fake provider used in transport tests so it can:
- hold a current token,
- optionally install a refreshed token during refresh,
- track refresh count.

Added/updated tests include:
- missing token triggers refresh before first request,
- `invalid_token` still triggers refresh and retry,
- existing 401 refresh/retry behavior still works.

## What is currently broken

`crates/project/src/context_server_store.rs` still targets the old API and currently has compile errors.

The last diagnostic snapshot I saw for that file included errors like:

- calling `discover()` with 4 arguments when it now takes 3,
- trying to access `discovery.client_registration` which no longer exists,
- trying to use `oauth::StaticTokenProvider`, which no longer exists,
- calling `McpOAuthTokenProvider::new(...)` with the old argument shape.

These are expected breakages from the `oauth.rs` refactor and must be resolved next.

## Detailed TODO list for next session

This is the concrete implementation plan, in the recommended order.

### TODO 1: Finish refactoring `ContextServerStatus`

Motivation:
- public status should not expose `OAuthDiscovery`,
- the UI does not need it,
- removing the payload lets us use derived `PartialEq`, `Eq`, and `Hash`.

What to do:
- change `ContextServerStatus::AuthRequired(Arc<OAuthDiscovery>)` to `ContextServerStatus::AuthRequired`,
- remove the handwritten `PartialEq`, `Eq`, and `Hash`,
- update `from_state()` accordingly.

File:
- `crates/project/src/context_server_store.rs`

Downstream consumers to update:
- `crates/agent_ui/src/agent_configuration.rs`
- `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs`
- `crates/agent/src/tools/context_server_registry.rs`
- `crates/assistant_text_thread/src/text_thread_store.rs`
- `crates/agent_ui/src/agent_panel.rs`

Pattern to replace:
- `ContextServerStatus::AuthRequired(_)`
with:
- `ContextServerStatus::AuthRequired`

### TODO 2: Update `run_server()` to the new discovery API

Motivation:
- discovery is now discovery-only,
- DCR must not happen during discovery,
- DCR cache clearing must key off auth issuer and should only happen when warranted.

What to do:
- replace the old `discover(&http_client, &server_url, www_authenticate, cached_dcr)` call with:
  - `discover(&http_client, &server_url, www_authenticate)`
- do **not** load cached DCR registration here,
- do **not** persist DCR registration here,
- if the bearer error indicates invalid client, only clear cached DCR *after* discovery succeeds and use:
  - `discovery.auth_server_metadata.issuer`
  as the DCR cache key input.

Notes:
- now that `invalid_token` is not treated as invalid client, this path should trigger much less often.

File:
- `crates/project/src/context_server_store.rs`

Relevant old area:
- the OAuth 401 handling branch inside `run_server()`.

### TODO 3: Introduce full session persistence in the store

Motivation:
- startup should restore a refresh-capable provider,
- eliminate `StaticTokenProvider`,
- keep one session model for startup and post-auth.

What to do:
- replace token-only keychain helpers with session helpers:
  - `store_session()`
  - `load_session()`
  - `clear_session()`
- serialize/deserialize `OAuthSession`.

Suggested key:
- keep the existing server URL key for the full session:
  - `mcp-oauth:<canonical_server_uri(server_url)>`

This is fine because the session is for the MCP server connection.

File:
- `crates/project/src/context_server_store.rs`

### TODO 4: Unify startup and post-auth token provider creation

Motivation:
- one refresh-capable provider implementation,
- one persistence path for refresh updates.

What to do:
- add a helper in the store, something like:
  - `create_oauth_token_provider(...) -> Arc<dyn oauth::OAuthTokenProvider>`
- this helper should:
  - create the `mpsc` channel,
  - spawn the persistence task that stores refreshed `OAuthSession`,
  - create `McpOAuthTokenProvider::new(session, http_client, Some(sender))`.

Use that helper in two places:
1. startup restoration inside `create_context_server()`,
2. post-auth inside `run_oauth_flow()`.

File:
- `crates/project/src/context_server_store.rs`

### TODO 5: Update `create_context_server()` startup path

Motivation:
- replace `StaticTokenProvider`,
- load full `OAuthSession` instead.

What to do:
- in the HTTP branch before constructing the transport:
  - load `OAuthSession` from keychain,
  - if present, create the unified provider,
  - pass it into `HttpTransport::new_with_token_provider(...)`.

Remove:
- any use of `load_tokens()`,
- any use of `StaticTokenProvider`.

File:
- `crates/project/src/context_server_store.rs`

Relevant current breakage:
- diagnostics point at a missing `oauth::StaticTokenProvider`.

### TODO 6: Refactor `run_oauth_flow()` to use `resolve_client_registration()`

Motivation:
- this is the main DCR correctness fix,
- DCR should happen only with the real loopback redirect URI,
- cached DCR registration should be keyed by auth issuer.

What to do:
- after starting the callback server and computing `redirect_uri`,
- load cached DCR registration using:
  - `load_dcr_registration(&credentials_provider, &discovery.auth_server_metadata.issuer, cx)`
- call:
  - `oauth::resolve_client_registration(&http_client, &discovery, &redirect_uri, cached_dcr_registration)`
- if the strategy is DCR, persist the resulting registration keyed by auth issuer.

Then:
- build `OAuthSession` from:
  - `discovery.clone()`
  - resolved `client_registration`
  - exchanged `tokens`
- persist the full session,
- create unified provider from that session.

Do not:
- refer to `discovery.client_registration` anywhere, because it no longer exists.

File:
- `crates/project/src/context_server_store.rs`

### TODO 7: Fix DCR cache helper signatures in the store

Motivation:
- DCR registration belongs to auth issuer.

What to do:
- change store helper signatures from taking `server_url` to taking `auth_server_issuer`:
  - `store_dcr_registration(...)`
  - `load_dcr_registration(...)`
  - `clear_dcr_registration(...)`
- internally use:
  - `oauth::dcr_registration_cache_key(auth_server_issuer)`

File:
- `crates/project/src/context_server_store.rs`

### TODO 8: Decide logout semantics for DCR registration

My recommendation:
- do **not** clear DCR registration on logout.

Motivation:
- logging out the user is not the same thing as invalidating the OAuth client registration,
- keeping DCR cache reduces needless churn and avoids unnecessary re-registration.

What to do:
- `logout_server()` should clear the stored OAuth session and stop the server,
- probably leave DCR registration intact,
- only clear DCR registration when there is real evidence it is invalid.

This is an intentional design recommendation, not yet implemented.

File:
- `crates/project/src/context_server_store.rs`

### TODO 9: Update `server_may_have_oauth_credentials()`

Motivation:
- once session persistence changes, this method should still reflect UI intent cleanly.

Current logic includes:
- `Running`
- `AuthRequired`
- `Authenticating`

That still seems reasonable because the user may want “Log Out” while auth is required or in progress if a session exists or partially exists.

Just re-check it after refactor.

File:
- `crates/project/src/context_server_store.rs`

### TODO 10: Update all status matches downstream

Required because `AuthRequired(_)` becomes `AuthRequired`.

Files:
- `crates/agent_ui/src/agent_configuration.rs`
- `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs`
- `crates/agent/src/tools/context_server_registry.rs`
- `crates/assistant_text_thread/src/text_thread_store.rs`
- `crates/agent_ui/src/agent_panel.rs`

Specific note:
- the one-time auth toast in `agent_panel.rs` should just switch from matching `AuthRequired(_)` to `AuthRequired`.

### TODO 11: Consider a small architectural cleanup in the store

This is not required to make the branch build again, but it is worth considering while touching the code.

Problem:
- `ContextServerStore` still knows too much about how to assemble authenticated HTTP transports.

Current symptoms:
- startup path and post-auth path each manually build transport + provider wiring.

Suggested cleanup:
- at minimum, add one helper for:
  - constructing HTTP `ContextServer` with optional OAuth provider.
- optionally, later, move more of this behind `context_server`.

This is lower priority than fixing the branch, but it is one of the better simplifications left.

## Concrete references for the next session

These are the files that matter most.

### Files already modified and important to read first

- `crates/context_server/src/oauth.rs`
- `crates/context_server/src/transport/http.rs`

These contain the new model and the updated tests.

### Main unfinished file

- `crates/project/src/context_server_store.rs`

This is the next file to fix.

### Downstream consumers that will need trivial status-match updates

- `crates/agent_ui/src/agent_configuration.rs`
- `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs`
- `crates/agent/src/tools/context_server_registry.rs`
- `crates/assistant_text_thread/src/text_thread_store.rs`
- `crates/agent_ui/src/agent_panel.rs`

## Current compile breakage hints

When resuming, expect these kinds of failures until the store is updated:

- old `discover(..., cached_dcr_registration)` call signatures,
- references to `discovery.client_registration`,
- references to `oauth::StaticTokenProvider`,
- old `McpOAuthTokenProvider::new(tokens, discovery, http_client, sender)` shape.

Those are all expected and should disappear as soon as the store adopts `OAuthSession`.

## Suggested implementation order for next session

1. Open `crates/project/src/context_server_store.rs`.
2. Change `ContextServerStatus::AuthRequired(_)` to payload-free `AuthRequired`.
3. Update `from_state()` and remove manual equality/hash.
4. Add `store_session/load_session/clear_session`.
5. Add `create_oauth_token_provider` helper and the persistence task helper.
6. Update startup restoration in `create_context_server()`.
7. Update `run_server()` to call the new `discover()` API.
8. Update `run_oauth_flow()` to call `resolve_client_registration()`, persist `OAuthSession`, and use the unified provider.
9. Update `logout_server()` semantics.
10. Update downstream pattern matches and UI code.
11. Run diagnostics and fix remaining compile errors.
12. Run focused tests for:
    - `oauth.rs`
    - `transport/http.rs`
    - any relevant project store tests if present.

## Notes on tests

The user’s repo rule asks for TDD and one test at a time. Since the branch already had a large existing implementation and I used static review plus targeted refactoring, the next session should at least add or adjust tests incrementally while finishing the store changes.

Most useful tests to add next if there is time:

- restoring a cached `OAuthSession` at startup uses a refresh-capable provider,
- expired restored session refreshes before first request,
- DCR cache key is based on issuer and not resource URL,
- logout clears the session but, if we choose that design, does not clear cached DCR registration.

## Final architecture recommendation

The target architecture should be:

- `oauth.rs`
  - pure discovery and auth/session logic,
  - persisted `OAuthSession`,
  - unified refresh-capable token provider.
- `transport/http.rs`
  - bearer attachment,
  - proactive refresh when needed,
  - 401 -> refresh -> retry -> `AuthRequired`.
- `project::ContextServerStore`
  - lifecycle state machine,
  - persisted session + DCR cache management,
  - browser auth orchestration,
  - minimal provider/transport assembly through helpers.
- UI
  - only sees status,
  - never needs discovery payload.

That keeps the good parts of this branch and the good internal shape from `origin/mcp-auth` without regressing to custom-scheme callbacks.

## If you need a quick “where to start” note

Start with `crates/project/src/context_server_store.rs`. The `oauth.rs` side is already set up for the intended design. The store is now the lagging piece.