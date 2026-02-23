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

### What was NOT done yet in Phase 1

- Token keychain storage functions (`store_tokens`, `load_tokens`, `clear_tokens`
  using `CredentialsProvider`). These need `AsyncApp` context so they'll be
  easier to write when wiring up Phase 3.
- DCR client ID persistence in `KEY_VALUE_STORE`. Same reason.

## What's next

### Phase 2: Transport integration (`crates/context_server/src/transport/http.rs`)

The current `HttpTransport` sends requests with static headers and treats all
non-success responses as opaque string errors. Changes needed:

- Add `Option<Arc<dyn OAuthTokenProvider>>` to `HttpTransport`. The trait has
  `access_token() -> Option<String>` and `try_refresh() -> Future<Result<bool>>`.
- In `send_message()`: attach `Authorization: Bearer <token>` when available.
- On 401 response: parse `WWW-Authenticate`, try `try_refresh()` + retry once,
  then return a typed `TransportError::AuthRequired { www_authenticate }`.
- The `Transport` trait's `send` returns `Result<()>` via anyhow; use downcast
  to `TransportError` where needed upstream.

### Phase 3: State management (`crates/project/src/context_server_store.rs`)

- Add `AuthRequired` variant to `ContextServerStatus` and `ContextServerState`
  (with `discovery: Arc<OAuthDiscovery>` on the state).
- In `run_server()`, catch `TransportError::AuthRequired` from `server.start()`,
  run discovery, transition to `AuthRequired`.
- Add `authenticate_server()` — starts callback server, builds auth URL, opens
  browser, awaits code, exchanges tokens, stores in keychain, restarts server.
- Add `logout_server()` — clears keychain tokens, stops server, disables in settings.
- On startup with cached tokens: load from keychain, create token provider, pass
  to transport.
- Concrete `OAuthTokenProvider` implementation backed by keychain + discovery info.

### Phase 4: UI (`crates/agent_ui/src/agent_configuration.rs`)

- `AuthRequired` status indicator + "Authenticate" button.
- "Log Out" entry in gear menu for authenticated servers.
- One-time toast via `Dismissable` trait on `KEY_VALUE_STORE`.

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

## Codebase orientation

- `crates/context_server/src/oauth.rs` — the new module (Phase 1).
- `crates/context_server/src/transport/http.rs` — `HttpTransport` (Phase 2 target).
- `crates/context_server/src/transport.rs` — `Transport` trait.
- `crates/context_server/src/context_server.rs` — `ContextServer`, creates transports.
- `crates/project/src/context_server_store.rs` — `ContextServerStore`, manages
  server lifecycle. `ContextServerStatus` / `ContextServerState` enums. `run_server`,
  `start_server`, `stop_server`, `create_context_server`.
- `crates/agent_ui/src/agent_configuration.rs` — `render_context_server` renders
  each MCP server row with status indicator, toggle, gear menu.
- `crates/credentials_provider/` — `CredentialsProvider` trait for keychain access.
- `crates/db/src/kvp.rs` — `KEY_VALUE_STORE`, `Dismissable` trait.
- `crates/http_client/src/http_client.rs` — `HttpClient` trait, `FakeHttpClient`
  for tests.

## Testing approach

- Pure logic: standard `#[test]` with direct assertions.
- Async I/O: `smol::block_on` with `FakeHttpClient::create(handler)` for mock HTTP.
  The fake client is created via `http_client::FakeHttpClient::create(handler)` which
  returns `Arc<HttpClientWithUrl>`, castable to `Arc<dyn HttpClient>`.
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