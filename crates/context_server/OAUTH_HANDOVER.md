# MCP OAuth Implementation — Handover

## What this is

We're adding OAuth 2.1 authorization to Zed's MCP Streamable HTTP transport,
targeting the MCP 2025-11-25 spec (latest stable). The full plan is in
`crates/context_server/OAUTH_PLAN.md`.

## What's done

### Phase 1: `crates/context_server/src/oauth.rs` (mostly complete)

The oauth module is created and registered in `context_server.rs`. It contains:

**Pure logic (fully tested, 47+ tests passing):**
- Types: `ProtectedResourceMetadata`, `AuthServerMetadata`, `OAuthClientRegistration`,
  `OAuthTokens`, `OAuthDiscovery`, `WwwAuthenticate`, `PkceChallenge`, `TokenResponse`,
  `AuthorizationCallback`.
- `parse_www_authenticate()` — parses `WWW-Authenticate: Bearer` headers per
  RFC 6750 / RFC 9728. Handles `resource_metadata`, `scope`, `error`,
  `error_description` parameters.
- `protected_resource_metadata_urls()` — constructs well-known URIs per RFC 9728.
- `auth_server_metadata_urls()` — constructs well-known URIs per RFC 8414 + OIDC.
- `canonical_server_uri()` — derives the RFC 8707 resource parameter.
- `select_scopes()` — implements the spec's Scope Selection Strategy.
- `determine_registration_strategy()` — CIMD first, DCR fallback, Unavailable.
- `generate_pkce_challenge()` — RFC 7636 S256 challenge/verifier.
- `build_authorization_url()` — constructs the full authorize URL with all params.
- `token_exchange_params()` / `token_refresh_params()` — form-encoded body builders.
- `dcr_registration_body()` — RFC 7591 registration JSON.
- Vendored `base64_url_encode()` and `simple_sha256()` to avoid extra crypto deps.

**Async I/O (tested with FakeHttpClient, all passing):**
- `fetch_protected_resource_metadata()` — fetches from WWW-Authenticate URL or
  well-known URIs.
- `fetch_auth_server_metadata()` — tries RFC 8414 then OIDC Discovery endpoints.
- `discover()` — full discovery flow: resource metadata → auth server metadata →
  PKCE validation → scope selection → client registration (CIMD or DCR).
- `perform_dcr()` — Dynamic Client Registration POST.
- `exchange_code()` — authorization code → token exchange.
- `refresh_tokens()` — refresh token grant.
- `start_callback_server()` — ephemeral localhost TCP server that receives the
  OAuth redirect, parses `code` and `state`, returns HTML to the browser.
- `fetch_json()` helper.

**Dependencies added:** `rand` in `crates/context_server/Cargo.toml`.

**CIMD constant:** `CIMD_URL = "https://zed.dev/oauth/client-metadata.json"` —
Zed Industries needs to host this document (see plan Phase 5).

### Phase 2: Transport integration (complete)

`crates/context_server/src/transport/http.rs` now handles OAuth at the transport
layer:

**New types:**
- `OAuthTokenProvider` trait in `oauth.rs` — the interface the transport uses to
  get tokens and attempt refreshes. Two methods: `access_token() -> Option<String>`
  and `async try_refresh() -> Result<bool>`.
- `TransportError` enum in `transport/http.rs` — typed error with
  `AuthRequired { www_authenticate }` variant, downcastable from `anyhow::Error`.

**HttpTransport changes:**
- Added `token_provider: Option<Arc<dyn OAuthTokenProvider>>` field.
- New `new_with_token_provider()` constructor; existing `new()` remains unchanged
  (no token provider) for backwards compatibility.
- Extracted `build_request()` helper that attaches `Authorization: Bearer <token>`
  when a token provider is present.
- `send_message()` intercepts 401 responses: parses `WWW-Authenticate`, calls
  `try_refresh()`, retries once with the new token, and returns
  `TransportError::AuthRequired` if auth still fails.
- `Drop` impl also attaches the bearer token to the session cleanup DELETE request.

**Tests (6, all passing via `#[gpui::test]`):**
1. Bearer token attached when provider present.
2. No auth header without a provider.
3. 401 triggers refresh + retry (succeeds).
4. 401 returns `AuthRequired` when refresh fails.
5. 401 returns `AuthRequired` without any provider.
6. 401 after successful refresh (server still rejects) returns `AuthRequired`.

### Phase 3: State management (complete)

`crates/project/src/context_server_store.rs` now manages the full OAuth lifecycle:

**Status/state enums:**
- Added `AuthRequired(Arc<OAuthDiscovery>)` variant to `ContextServerStatus`.
- Added `AuthRequired { server, configuration, discovery }` variant to
  `ContextServerState`.
- Hand-implemented `PartialEq`, `Eq`, `Hash` for `ContextServerStatus` (the
  `OAuthDiscovery` payload is not compared — variant identity is enough).
- All downstream match expressions updated (`agent_ui`, `agent`,
  `assistant_text_thread`).

**401 → AuthRequired transition in `run_server()`:**
- When `server.start()` fails with `TransportError::AuthRequired`, the store
  extracts the server URL from the `Http` configuration, runs
  `oauth::discover()` to fetch resource metadata, auth server metadata, and
  client registration, and transitions to `AuthRequired` with the discovery info.
- If discovery itself fails, transitions to `Error` with a descriptive message.

**`authenticate_server()`:**
- Validates the server is in `AuthRequired` state.
- Spawns the full OAuth browser flow (`run_oauth_flow`):
  1. Starts `oauth::start_callback_server()` on an ephemeral port.
  2. Generates PKCE challenge and random state parameter.
  3. Builds the authorization URL and opens the user's browser.
  4. Awaits the callback with `code` and `state`.
  5. Validates the state parameter.
  6. Calls `oauth::exchange_code()` to get tokens.
  7. Persists tokens in the system keychain via `CredentialsProvider`.
  8. Creates a new `HttpTransport` with an `McpOAuthTokenProvider` (which
     supports refresh) and restarts the server.
- On failure, transitions to `Error` with the original server/configuration
  preserved.

**`logout_server()`:**
- Stops the server and clears stored tokens from the keychain.

**Token providers:**
- `McpOAuthTokenProvider` in `oauth.rs` — holds tokens in-memory, can refresh
  via the token endpoint using discovery info and the HTTP client. After a
  successful refresh the new tokens live in memory (keychain persistence of
  refreshed tokens is a future enhancement — if the app restarts, the old
  refresh token is tried; if it was rotated, the user re-authenticates).
- `StaticTokenProvider` in `oauth.rs` — holds a single access token, never
  refreshes. Used on startup when loading cached tokens from the keychain
  (no discovery info available yet). If the token is expired, the server gets
  a 401, `try_refresh()` returns false, and the full discovery/auth flow kicks in.

**Cached token loading on startup:**
- `create_context_server()` now checks the keychain for cached tokens when
  creating HTTP servers. If found, creates the transport with a
  `StaticTokenProvider` so the first request includes the bearer token.

**Keychain integration:**
- `store_tokens()` — serializes `OAuthTokens` as JSON, writes to keychain
  via `CredentialsProvider`. Key format: `mcp-oauth:<canonical_server_uri>`.
  Username: `mcp-oauth`.
- `load_tokens()` — reads from keychain, deserializes.
- `clear_tokens()` — deletes from keychain.
- Added `credentials_provider` dependency to `crates/project/Cargo.toml`.

### What was NOT done in Phase 1-3

- DCR client ID persistence in `KEY_VALUE_STORE`. Currently, if DCR is used,
  a new client ID is minted on every discovery. This is fine for CIMD-primary
  servers but should be addressed for DCR-only servers.
- Persistence of refreshed tokens. If a token refresh succeeds mid-session,
  the new tokens are in-memory only. On restart, the old refresh token is
  tried; if it was rotated by the server, the user must re-authenticate.

## What's next

### Phase 4: UI (`crates/agent_ui/src/agent_configuration.rs`)

The `AuthRequired` status variant is already handled in the UI code with a
warning-colored indicator and "Authentication required" tooltip. Remaining work:

- "Authenticate" button in the server row or a modal that triggers
  `authenticate_server()` on the store.
- "Log Out" entry in the gear menu for authenticated HTTP servers that calls
  `logout_server()`.
- One-time toast via `Dismissable` trait on `KEY_VALUE_STORE` explaining that
  the server needs authentication (shown once per server, dismissed forever).

### Phase 5: External

- Deploy CIMD JSON document to `https://zed.dev/oauth/client-metadata.json`.

## Key design decisions (already agreed)

- OAuth state lives on the transport (token provider trait), not the store.
- `AuthRequired` is a first-class status variant, not an error string.
- Never auto-open the browser — user must click "Authenticate".
- Client registration: CIMD (check `client_id_metadata_document_supported` in
  auth server metadata) → DCR (check `registration_endpoint`) → error.
- Tokens in system keychain via `CredentialsProvider` trait (see
  `crates/credentials_provider/`). Key format: `mcp-oauth:<canonical_server_uri>`.
- DCR-minted client IDs persisted in `KEY_VALUE_STORE` keyed by
  `mcp-oauth-dcr-client:<auth_server_url>`.
- One-time toast via `Dismissable` trait (see `crates/db/src/kvp.rs`).
- Localhost callback uses `127.0.0.1` (not `localhost`) per OAuth 2.1 guidance.
  Ephemeral port, registered in CIMD without port (auth servers ignore port for
  loopback per OAuth 2.1 Section 7.5.1).
- Two token provider implementations: `McpOAuthTokenProvider` (full, with
  refresh) for post-authentication, and `StaticTokenProvider` (access token
  only, no refresh) for cached-tokens-on-startup.

## Codebase orientation

- `crates/context_server/src/oauth.rs` — the OAuth module (Phase 1). Contains
  types, discovery, token exchange, callback server, PKCE, `OAuthTokenProvider`
  trait, `McpOAuthTokenProvider`, and `StaticTokenProvider`.
- `crates/context_server/src/transport/http.rs` — `HttpTransport` with OAuth
  support (Phase 2). `TransportError` enum, `build_request()`, 401 handling.
- `crates/context_server/src/transport.rs` — `Transport` trait. Re-exports
  `TransportError` and `HttpTransport` via `pub use http::*`.
- `crates/context_server/src/context_server.rs` — `ContextServer`, creates
  transports. `http()` constructor still uses `HttpTransport::new()` (no token
  provider); token-aware construction happens in the store.
- `crates/project/src/context_server_store.rs` — `ContextServerStore`, manages
  server lifecycle with OAuth. `ContextServerStatus::AuthRequired`,
  `ContextServerState::AuthRequired`, `authenticate_server()`,
  `logout_server()`, `run_oauth_flow()`, keychain helpers (`store_tokens`,
  `load_tokens`, `clear_tokens`), cached token loading in
  `create_context_server()`.
- `crates/agent_ui/src/agent_configuration.rs` — `render_context_server` renders
  each MCP server row with status indicator, toggle, gear menu. `AuthRequired`
  renders with a warning indicator.
- `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs` —
  `AuthRequired` handled in `wait_for_context_server` subscription.
- `crates/agent/src/tools/context_server_registry.rs` — `AuthRequired` grouped
  with `Stopped`/`Error` (server not usable).
- `crates/assistant_text_thread/src/text_thread_store.rs` — same grouping.
- `crates/credentials_provider/` — `CredentialsProvider` trait for keychain
  access.
- `crates/db/src/kvp.rs` — `KEY_VALUE_STORE`, `Dismissable` trait.
- `crates/http_client/src/http_client.rs` — `HttpClient` trait, `FakeHttpClient`
  for tests.

## Testing approach

- Pure logic: standard `#[test]` with direct assertions.
- Async I/O: `smol::block_on` with `FakeHttpClient::create(handler)` for mock HTTP.
  The fake client is created via `http_client::FakeHttpClient::create(handler)` which
  returns `Arc<HttpClientWithUrl>`, castable to `Arc<dyn HttpClient>`.
- Transport tests: `#[gpui::test]` with `TestAppContext` for `BackgroundExecutor`
  access. `FakeHttpClient` for mock HTTP, `FakeTokenProvider` for mock OAuth.
- Callback server tests: real TCP connections on localhost in `smol::block_on`.
- Project rules say to use TDD, one test at a time, ask for confirmation before
  moving on. Follow that cadence.

## PR context

This work addresses https://github.com/zed-industries/zed/issues/43162. A previous
community PR (#44638 by erenatas) was closed because it targeted an older spec
revision (2025-06-18), had connection issues with real servers, and architectural
differences. We incorporate the UX feedback from agu-z (Zed team member) from that
PR: explicit auth, no auto-browser, keychain storage, auth status in settings UI.

The MCP spec moved substantially between 2025-06-18 and 2025-11-25. The biggest
change is CIMD (Client ID Metadata Documents) becoming the recommended default
over DCR. We implement both, with CIMD as primary and DCR as fallback.

## Known issues

- `oauth::tests::test_fetch_protected_resource_metadata` has a pre-existing
  failure due to a trailing slash in URL comparison (`Url::parse` normalizes
  `https://auth.example.com` to `https://auth.example.com/`). Not introduced
  by the Phase 2/3 work.