# MCP OAuth Implementation — Handover

## What this is

We're adding OAuth 2.1 authorization to Zed's MCP Streamable HTTP transport,
targeting the MCP 2025-11-25 spec (latest stable). The full plan is in
`crates/context_server/OAUTH_PLAN.md`.

## What's done

Phases 1 through 4 are complete, along with DCR client ID persistence and
refresh token persistence. The workspace builds cleanly with no warnings from
our code, and all new tests pass (52 in the oauth module alone). The one
pre-existing test failure
(`oauth::tests::test_fetch_protected_resource_metadata` — trailing slash in URL
comparison from `Url::parse`) is unrelated.

### Phase 1: OAuth primitives (`crates/context_server/src/oauth.rs`)

A self-contained OAuth module with ~52 tests. Contains:

- **Types:** `ProtectedResourceMetadata`, `AuthServerMetadata`,
  `OAuthClientRegistration` (with `Serialize`/`Deserialize`), `OAuthTokens`,
  `OAuthDiscovery`, `WwwAuthenticate`, `PkceChallenge`, `TokenResponse`,
  `AuthorizationCallback`.
- **Pure logic:** `parse_www_authenticate()`, `protected_resource_metadata_urls()`,
  `auth_server_metadata_urls()`, `canonical_server_uri()`, `select_scopes()`,
  `determine_registration_strategy()`, `generate_pkce_challenge()`,
  `build_authorization_url()`, `token_exchange_params()`,
  `token_refresh_params()`, `dcr_registration_body()`.
- **Async I/O:** `fetch_protected_resource_metadata()`,
  `fetch_auth_server_metadata()`, `discover()`, `perform_dcr()`,
  `exchange_code()`, `refresh_tokens()`, `start_callback_server()`.
- **Token provider trait and implementations:**
  - `OAuthTokenProvider` trait — `access_token() -> Option<String>` and
    `async try_refresh() -> Result<bool>`.
  - `McpOAuthTokenProvider` — holds tokens in `SyncMutex`, can refresh via the
    token endpoint using discovery info and the HTTP client. Optionally holds an
    `mpsc::UnboundedSender<OAuthTokens>` to notify after successful refreshes
    (used for keychain persistence without requiring GPUI context).
  - `StaticTokenProvider` — holds a single access token, never refreshes. Used
    on startup when loading cached tokens from the keychain before discovery
    info is available.
- **CIMD constant:** `CIMD_URL = "https://zed.dev/oauth/client-metadata.json"`.

### Phase 2: Transport integration (`crates/context_server/src/transport/http.rs`)

- `TransportError::AuthRequired { www_authenticate }` — typed error downcastable
  from `anyhow::Error`.
- `HttpTransport` gained a `token_provider: Option<Arc<dyn OAuthTokenProvider>>`
  field.
- `new_with_token_provider()` constructor alongside the unchanged `new()`.
- `build_request()` attaches `Authorization: Bearer <token>` when a provider is
  present.
- `send_message()` intercepts 401 responses: parses `WWW-Authenticate`, calls
  `try_refresh()`, retries once, returns `TransportError::AuthRequired` if auth
  still fails.
- `Drop` attaches the bearer token to the session cleanup DELETE.
- **6 tests** via `#[gpui::test]` with `FakeHttpClient` and `FakeTokenProvider`.

### Phase 3: State management (`crates/project/src/context_server_store.rs`)

**Status/state enums:**
- `ContextServerStatus::AuthRequired(Arc<OAuthDiscovery>)` — public status the
  UI matches on.
- `ContextServerState::AuthRequired { server, configuration, discovery }` —
  internal state.
- Hand-implemented `PartialEq`/`Eq`/`Hash` for `ContextServerStatus` since
  `OAuthDiscovery` isn't `Eq`.

**401 → AuthRequired transition in `run_server()`:**
- When `server.start()` fails with `TransportError::AuthRequired`, the store
  extracts the server URL, loads any cached DCR registration from the keychain,
  runs `oauth::discover()` (passing the cached registration), persists the
  resulting registration back to the keychain, and transitions to
  `ContextServerState::AuthRequired`. If discovery fails, transitions to `Error`.

**`authenticate_server(&mut self, id, cx) -> Result<()>`:**
- Validates the server is in `AuthRequired` state.
- Spawns `run_oauth_flow()` which:
  1. Starts `oauth::start_callback_server()` on an ephemeral port.
  2. Generates PKCE challenge and random state parameter.
  3. Builds authorization URL and opens the browser via `cx.open_url()`.
  4. Awaits callback, validates state.
  5. Exchanges code for tokens via `oauth::exchange_code()`.
  6. Persists tokens in keychain via `CredentialsProvider`.
  7. Creates an `mpsc::unbounded` channel and passes the sender to
     `McpOAuthTokenProvider`. Spawns a detached foreground task that reads from
     the receiver and persists refreshed tokens to the keychain.
  8. Creates `HttpTransport` with the token provider and restarts the server.
- On failure, transitions to `Error` preserving the original server/configuration.

**`logout_server(&mut self, id, cx) -> Result<()>`:**
- Stops the server, spawns keychain deletion of both OAuth tokens and cached DCR
  registration.

**Cached token loading in `create_context_server()`:**
- For HTTP servers, checks the keychain for cached tokens. If found, wraps them
  in a `StaticTokenProvider` and passes to `HttpTransport::new_with_token_provider()`.
  If the token is expired, the transport gets a 401, refresh returns false, and
  the full discovery flow kicks in.

**Keychain helpers (private):**
- `store_tokens()` / `load_tokens()` / `clear_tokens()` — OAuth tokens.
  Key: `mcp-oauth:<canonical_server_uri>`.
- `store_dcr_registration()` / `load_dcr_registration()` /
  `clear_dcr_registration()` — DCR client registrations.
  Key: `mcp-oauth-dcr-client:<canonical_server_uri>`.

**Downstream match exhaustiveness fixes:**
- `crates/agent_ui/src/agent_configuration.rs` — warning-colored indicator dot
  with "Authentication required." tooltip.
- `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs` —
  sends error through channel (server won't become `Running` without user action).
- `crates/agent/src/tools/context_server_registry.rs` — grouped with
  `Stopped`/`Error` (tools removed).
- `crates/assistant_text_thread/src/text_thread_store.rs` — same (slash commands
  removed).

**Dependencies added:** `credentials_provider` in `crates/project/Cargo.toml`.

### Phase 4: UI (`crates/agent_ui/src/agent_configuration.rs`)

**"Authenticate" button:**
- When a server's status is `AuthRequired`, a row appears below the server name
  with the label "Authentication required." and a filled "Authenticate" button.
- Clicking it calls `store.authenticate_server(&server_id, cx)`. The store
  handles the full OAuth flow; the UI transitions to `Starting` automatically
  via `ServerStatusChangedEvent`.
- Layout mirrors the existing error block pattern — indented to align with the
  server name.

**"Log Out" in the gear menu:**
- For HTTP servers (`is_remote` flag, derived from
  `ContextServerConfiguration::Http`), a "Log Out" entry appears in the gear
  menu between "View Tools" and the separator before "Uninstall".
- Calls `store.logout_server(&server_id, cx)`, which stops the server and clears
  both tokens and cached DCR registration from the keychain.

### DCR client ID persistence

- `discover()` accepts an optional `cached_dcr_registration:
  Option<OAuthClientRegistration>`. When the registration strategy would be DCR
  and a cached registration is provided, the cached value is used and the
  registration endpoint is never hit.
- CIMD still takes priority — a cached DCR registration is ignored when the auth
  server supports CIMD.
- The store loads the cached registration from the keychain before calling
  `discover()` in the 401 handler, and persists the resulting registration after
  discovery succeeds.
- `logout_server()` clears the cached DCR registration alongside tokens, so a
  fresh DCR is performed on re-authentication.
- Two new tests: `test_discover_uses_cached_dcr_registration` (verifies the
  registration endpoint is never called when a cache is provided) and
  `test_discover_ignores_cached_dcr_when_cimd_available` (verifies CIMD
  priority).

### Refresh token persistence

- `McpOAuthTokenProvider` holds an optional
  `mpsc::UnboundedSender<OAuthTokens>`. After a successful `try_refresh()`, new
  tokens are sent through the channel before being stored in memory.
- The store creates the channel in `run_oauth_flow()` and spawns a detached
  foreground task that reads refreshed tokens from the receiver and persists
  them to the keychain via `store_tokens()`.
- This decouples the token provider (which runs on background threads via the
  transport, with no GPUI context) from keychain writes (which need `AsyncApp`).
  The channel bridges the two worlds.
- When the server is stopped, the token provider (and its sender) are dropped,
  the receiver stream ends, and the persistence task terminates naturally.

## What's next

### Phase 5: External

- Deploy CIMD JSON document to `https://zed.dev/oauth/client-metadata.json`.
  This is infrastructure work, not code in this repo.

### Pre-existing test fix (trivial, unrelated)

- `oauth::tests::test_fetch_protected_resource_metadata` — trailing slash in URL
  comparison from `Url::parse`. The assertion compares
  `"https://auth.example.com/"` (from `Url::parse`) with
  `"https://auth.example.com"` (from the test expectation). A one-line fix.

### Optional improvements

- **One-time toast** — via `Dismissable` trait on `KEY_VALUE_STORE`. Shows a
  brief notification the first time a server enters `AuthRequired`. Key format:
  `mcp-oauth-toast-dismissed:<server_id>`. Skipped for now because the
  "Authentication required." label next to the button is clear enough.

## Key design decisions

- OAuth state lives on the transport (token provider trait), not the store.
- `AuthRequired` is a first-class status variant, not an error string.
- Never auto-open the browser — user must click "Authenticate".
- Client registration: CIMD first → DCR fallback → error.
- DCR registrations are cached in the system keychain so the same client_id is
  reused across restarts. Key: `mcp-oauth-dcr-client:<canonical_server_uri>`.
- Tokens in system keychain via `CredentialsProvider`. Key:
  `mcp-oauth:<canonical_server_uri>`.
- Refreshed tokens are persisted back to the keychain via an mpsc channel that
  bridges the background-thread token provider and the main-thread keychain API.
- Localhost callback uses `127.0.0.1` (not `localhost`) per OAuth 2.1 guidance.
  Ephemeral port.
- Two token provider implementations: `McpOAuthTokenProvider` (full, with
  refresh and optional persistence channel) for post-authentication, and
  `StaticTokenProvider` (access token only, no refresh) for
  cached-tokens-on-startup.
- `logout_server()` clears both tokens and DCR registration, forcing a clean
  slate on re-authentication.

## Codebase orientation

- `crates/context_server/src/oauth.rs` — OAuth module. Types, discovery (with
  DCR cache parameter), token exchange, callback server, PKCE,
  `OAuthTokenProvider` trait, `McpOAuthTokenProvider` (with refresh channel),
  `StaticTokenProvider`.
- `crates/context_server/src/transport/http.rs` — `HttpTransport` with OAuth.
  `TransportError`, `build_request()`, 401 handling.
- `crates/context_server/src/transport.rs` — `Transport` trait. Re-exports
  everything from `http.rs` via `pub use http::*`.
- `crates/context_server/src/context_server.rs` — `ContextServer`. The `http()`
  constructor uses `HttpTransport::new()` (no token provider); token-aware
  construction happens in the store via `HttpTransport::new_with_token_provider`.
- `crates/project/src/context_server_store.rs` — `ContextServerStore`. OAuth
  lifecycle: `AuthRequired` status/state, `authenticate_server()`,
  `logout_server()`, `run_oauth_flow()`, keychain helpers for tokens and DCR
  registrations, cached token loading, refresh-token persistence task.
- `crates/agent_ui/src/agent_configuration.rs` — `render_context_server()`.
  "Authenticate" button for `AuthRequired` servers, "Log Out" in gear menu for
  HTTP servers.
- `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs` —
  `AuthRequired` handled as failure in `wait_for_context_server`.
- `crates/agent/src/tools/context_server_registry.rs` — `AuthRequired` grouped
  with `Stopped`/`Error`.
- `crates/assistant_text_thread/src/text_thread_store.rs` — same.
- `crates/credentials_provider/` — `CredentialsProvider` trait for keychain.
- `crates/db/src/kvp.rs` — `KEY_VALUE_STORE`, `Dismissable` trait.
- `crates/http_client/src/http_client.rs` — `HttpClient` trait, `FakeHttpClient`.

## Testing approach

- Pure OAuth logic: `#[test]` with direct assertions (~52 tests).
- Async OAuth I/O: `smol::block_on` with `FakeHttpClient`.
- Transport tests: `#[gpui::test]` with `TestAppContext` for `BackgroundExecutor`.
  `FakeHttpClient` for mock HTTP, `FakeTokenProvider` for mock OAuth.
- Callback server tests: real TCP on localhost in `smol::block_on`.
- DCR caching tests: `test_discover_uses_cached_dcr_registration` and
  `test_discover_ignores_cached_dcr_when_cimd_available` verify cache hit/miss
  behavior and CIMD priority.

## PR context

This work addresses https://github.com/zed-industries/zed/issues/43162. A previous
community PR (#44638 by erenatas) was closed because it targeted an older spec
revision (2025-06-18), had connection issues, and architectural differences. We
incorporate the UX feedback from agu-z: explicit auth, no auto-browser, keychain
storage, auth status in settings UI.

The MCP spec moved substantially between 2025-06-18 and 2025-11-25. The biggest
change is CIMD becoming the recommended default over DCR. We implement both.