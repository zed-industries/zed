# MCP OAuth Implementation Plan

Target spec: **MCP 2025-11-25** (latest stable).

This plan adds OAuth 2.1 authorization support to Zed's MCP Streamable HTTP
transport, following the spec's authorization section. It covers Protected
Resource Metadata discovery (RFC 9728), Authorization Server Metadata discovery
(RFC 8414), client registration via CIMD and DCR, the Authorization Code + PKCE
flow, token lifecycle management, and the UI changes needed to surface auth
status and let users authenticate and log out.

Addresses https://github.com/zed-industries/zed/issues/43162.

---

## Architecture overview

```
┌─────────────────────────────────────────────────────────┐
│                    agent_ui                              │
│  ┌───────────────────────────────────────────────────┐  │
│  │ AgentConfiguration / render_context_server         │  │
│  │  - "Authenticate" button when AuthRequired         │  │
│  │  - "Log Out" entry in gear menu when authenticated │  │
│  │  - One-time toast via Dismissable                  │  │
│  └───────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────┘
                         │ calls authenticate_server / logout_server
┌────────────────────────▼────────────────────────────────┐
│              project::ContextServerStore                 │
│  - AuthRequired state variant                           │
│  - Holds cached discovery info per server               │
│  - authenticate_server(): browser flow → restart        │
│  - logout_server(): clear tokens → stop server          │
└────────────────────────┬────────────────────────────────┘
                         │ creates / restarts
┌────────────────────────▼────────────────────────────────┐
│          context_server::ContextServer                   │
│  - start() → detects 401 → returns typed auth error     │
└────────────────────────┬────────────────────────────────┘
                         │ uses
┌────────────────────────▼────────────────────────────────┐
│        context_server::transport::HttpTransport          │
│  - Attaches Bearer token from OAuthTokenProvider         │
│  - On 401: tries transparent refresh, then signals       │
│    auth required via typed error                         │
└────────────────────────┬────────────────────────────────┘
                         │ delegates to
┌────────────────────────▼────────────────────────────────┐
│            context_server::oauth                         │
│  - Discovery (RFC 9728, RFC 8414)                       │
│  - Client registration (CIMD → DCR fallback)            │
│  - Authorization URL construction + PKCE                │
│  - Local callback server                                │
│  - Token exchange and refresh                           │
│  - Keychain storage via CredentialsProvider              │
└─────────────────────────────────────────────────────────┘
```

---

## Component 1: `context_server::oauth`

New module at `crates/context_server/src/oauth.rs`.

### Types

```rust
/// Parsed from the MCP server's WWW-Authenticate header or well-known endpoint.
struct ProtectedResourceMetadata {
    resource: Url,
    authorization_servers: Vec<Url>,
    scopes_supported: Option<Vec<String>>,
}

/// Parsed from the authorization server's .well-known endpoint.
struct AuthServerMetadata {
    issuer: Url,
    authorization_endpoint: Url,
    token_endpoint: Url,
    registration_endpoint: Option<Url>,
    scopes_supported: Option<Vec<String>>,
    code_challenge_methods_supported: Option<Vec<String>>,
    client_id_metadata_document_supported: bool,
}

/// The result of client registration — either CIMD or DCR.
struct OAuthClientRegistration {
    client_id: String,
    // Only present for DCR-minted registrations.
    client_secret: Option<String>,
}

struct OAuthTokens {
    access_token: String,
    refresh_token: Option<String>,
    expires_at: Option<SystemTime>,
}

/// Everything the store needs to kick off the browser flow, obtained during
/// discovery. Cached on the AuthRequired state so we don't re-discover on
/// every authenticate attempt.
struct OAuthDiscovery {
    resource_metadata: ProtectedResourceMetadata,
    auth_server_metadata: AuthServerMetadata,
    client_registration: OAuthClientRegistration,
    scopes: Vec<String>,
}
```

### Discovery

`discover(http_client, server_url, www_authenticate_header) -> Result<OAuthDiscovery>`

1. **Protected Resource Metadata** (RFC 9728):
   - If a `WWW-Authenticate` header is present, parse it for the
     `resource_metadata` URL. The header format is:
     `Bearer resource_metadata="<url>", scope="<scopes>"`.
   - Otherwise, construct and try well-known URIs in order:
     `https://<host>/.well-known/oauth-protected-resource/<path>` then
     `https://<host>/.well-known/oauth-protected-resource`.
   - Fetch and deserialize the metadata JSON. Extract `authorization_servers`.

2. **Authorization Server Metadata** (RFC 8414 + OIDC Discovery):
   - Pick the first authorization server from the resource metadata.
   - For URLs with path components (e.g. `https://auth.example.com/tenant1`),
     try in order:
     - `https://auth.example.com/.well-known/oauth-authorization-server/tenant1`
     - `https://auth.example.com/.well-known/openid-configuration/tenant1`
     - `https://auth.example.com/tenant1/.well-known/openid-configuration`
   - For URLs without path components, try:
     - `https://auth.example.com/.well-known/oauth-authorization-server`
     - `https://auth.example.com/.well-known/openid-configuration`
   - Parse the response. Verify `code_challenge_methods_supported` includes
     `S256`. If PKCE support is absent, refuse to proceed (spec requirement).

3. **Scope selection** (spec's Scope Selection Strategy):
   - Use `scope` from the `WWW-Authenticate` header if present.
   - Otherwise use `scopes_supported` from the Protected Resource Metadata.
   - If neither is available, omit the scope parameter.

4. **Client registration**:
   - If `client_id_metadata_document_supported` is true in the auth server
     metadata: use the CIMD URL as the `client_id`. No network request needed.
     The CIMD URL is a compile-time constant pointing to a document hosted by
     Zed Industries (see external dependency below).
   - Else if `registration_endpoint` is present: perform DCR — POST to the
     registration endpoint with our client metadata. Store the returned
     `client_id` in the KV store keyed by auth server URL for reuse.
   - Else: return an error indicating no supported registration mechanism.

### Authorization URL + PKCE

`build_authorization_request(discovery, redirect_uri) -> (Url, PkceVerifier)`

- Generate a cryptographically random code verifier (43-128 chars, RFC 7636).
- Derive the S256 code challenge.
- Generate a random `state` parameter.
- Construct the authorization URL with query parameters:
  - `response_type=code`
  - `client_id=<cimd_url_or_dcr_client_id>`
  - `redirect_uri=http://127.0.0.1:<port>/callback`
  - `scope=<selected_scopes>`
  - `resource=<canonical_server_uri>` (RFC 8707)
  - `code_challenge=<challenge>`
  - `code_challenge_method=S256`
  - `state=<random>`

### Local callback server

`start_callback_server() -> Result<(u16, oneshot::Receiver<AuthorizationCode>)>`

- Bind a TCP listener to `127.0.0.1:0` (OS-assigned ephemeral port).
- Return the bound port and a oneshot receiver.
- Spawn a task that:
  - Accepts exactly one connection.
  - Parses the HTTP GET request for `code` and `state` query parameters.
  - Validates `state` matches.
  - Responds with a minimal HTML page ("You can close this tab").
  - Sends the code through the oneshot channel.
  - Shuts down the listener.

We use `127.0.0.1` rather than `localhost` per the OAuth 2.1 recommendation
(avoids DNS lookup issues). The CIMD document registers redirect URIs without
a port; auth servers must ignore port differences for loopback IPs per
OAuth 2.1 Section 7.5.1.

### Token exchange

`exchange_code(http_client, discovery, code, verifier, redirect_uri) -> Result<OAuthTokens>`

- POST to the token endpoint with:
  - `grant_type=authorization_code`
  - `code=<code>`
  - `redirect_uri=<redirect_uri>`
  - `client_id=<client_id>`
  - `code_verifier=<verifier>`
  - `resource=<canonical_server_uri>`
- Parse the response for `access_token`, `refresh_token`, `expires_in`.

### Token refresh

`refresh_tokens(http_client, discovery, refresh_token) -> Result<OAuthTokens>`

- POST to the token endpoint with:
  - `grant_type=refresh_token`
  - `refresh_token=<token>`
  - `client_id=<client_id>`
  - `resource=<canonical_server_uri>`

### Token storage

Uses `CredentialsProvider` (system keychain, or dev file fallback).

- **Key**: `mcp-oauth:<canonical_server_uri>`
- **Username**: the `client_id` used (for informational purposes).
- **Password**: JSON-serialized `OAuthTokens`.
- Functions: `store_tokens(server_url, tokens, cx)`,
  `load_tokens(server_url, cx) -> Option<OAuthTokens>`,
  `clear_tokens(server_url, cx)`.

For DCR-minted client IDs, we persist them separately in `KEY_VALUE_STORE`
keyed by `mcp-oauth-dcr-client:<auth_server_url>` so we reuse the same
registration on subsequent connections.

---

## Component 2: `HttpTransport` changes

File: `crates/context_server/src/transport/http.rs`.

### Token provider

Add an optional token provider to `HttpTransport`:

```rust
pub trait OAuthTokenProvider: Send + Sync {
    /// Returns a valid access token, refreshing transparently if needed.
    /// Returns None if no token is available (server may not require auth).
    fn access_token(&self) -> Option<String>;

    /// Attempt to refresh the token. Returns Ok(true) if successful.
    fn try_refresh(&self) -> impl Future<Output = Result<bool>> + Send;
}
```

`HttpTransport::new` gains an `Option<Arc<dyn OAuthTokenProvider>>` parameter.
Existing callers pass `None` (no behavior change for non-OAuth servers).

### Request flow

Modify `send_message` to:

1. If token provider is present and has a token, add
   `Authorization: Bearer <token>` to the request headers.
2. On 401 response:
   a. Parse the `WWW-Authenticate` header.
   b. If token provider is present, call `try_refresh()`. On success, retry the
      request once with the new token.
   c. If refresh fails or no token provider, return a typed
      `TransportError::AuthRequired { www_authenticate: String }` error
      (not a string through the error channel).
3. On 403 with `error="insufficient_scope"`: propagate as a typed error for
   future step-up auth support. For now, surface as a regular error.

### Typed transport errors

Replace the stringly-typed error channel with a proper error type:

```rust
pub enum TransportError {
    AuthRequired { www_authenticate: String },
    Http { status: u16, body: String },
    Connection(anyhow::Error),
}
```

This is an internal change. The `Transport` trait's `send` method already
returns `Result<()>`; we use `anyhow` with downcasting to `TransportError`
where needed.

---

## Component 3: State management

File: `crates/project/src/context_server_store.rs`.

### New status variant

```rust
pub enum ContextServerStatus {
    Starting,
    Running,
    Stopped,
    AuthRequired,   // new
    Error(Arc<str>),
}
```

And the corresponding internal state:

```rust
enum ContextServerState {
    // ... existing variants ...
    AuthRequired {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        discovery: Arc<OAuthDiscovery>,
    },
}
```

The `discovery` field caches what we learned during the failed initialization
so `authenticate_server` can skip straight to opening the browser.

### Detecting auth requirement

In `run_server`, when `server.start()` fails:

- Try to downcast the error to `TransportError::AuthRequired`.
- If it matches, parse the `www_authenticate` header and run discovery
  (in a background task). On success, transition to
  `ContextServerState::AuthRequired` with the cached discovery info.
- Otherwise, transition to `ContextServerState::Error` as today.

### `authenticate_server`

New public method on `ContextServerStore`:

```rust
pub fn authenticate_server(
    &mut self,
    server_id: &ContextServerId,
    cx: &mut Context<Self>,
) -> Task<Result<()>>
```

1. Look up the server. It must be in `AuthRequired` state. Extract the cached
   `OAuthDiscovery`.
2. Spawn a task that:
   a. Starts the local callback server (get port).
   b. Builds the authorization URL.
   c. Opens the URL in the user's browser (`cx.open_url`).
   d. Awaits the authorization code from the callback server.
   e. Exchanges the code for tokens.
   f. Stores the tokens in the keychain.
   g. Creates a new `HttpTransport` with a token provider backed by the
      stored tokens.
   h. Restarts the server (calls `run_server` again).

### `logout_server`

New public method:

```rust
pub fn logout_server(
    &mut self,
    server_id: &ContextServerId,
    cx: &mut Context<Self>,
) -> Task<Result<()>>
```

1. Clear the tokens from the keychain for this server's URL.
2. Stop the server.
3. Disable the server in settings.

### Token provider implementation

A concrete `OAuthTokenProvider` that:

- Holds the current `OAuthTokens` (behind a lock).
- Holds the `OAuthDiscovery` and `http_client` for refresh.
- `access_token()`: returns the cached token if not expired (with a 30-second
  buffer). Returns `None` if expired and no refresh token.
- `try_refresh()`: uses the refresh token to get new tokens, updates the cache
  and keychain.

This lives in the `context_server` crate so it can be passed to
`HttpTransport`.

### Startup with cached tokens

When creating an HTTP context server, before starting:

1. Check the keychain for stored tokens for this server URL.
2. If tokens exist, create the token provider with them and pass it to the
   transport.
3. Start normally. If the cached token is stale, the transport will try refresh
   first, then surface `AuthRequired` if that also fails.

---

## Component 4: UI changes

File: `crates/agent_ui/src/agent_configuration.rs`.

### Auth status rendering

In `render_context_server`, add handling for `ContextServerStatus::AuthRequired`:

- Status indicator: a key/lock icon in warning color.
- Tooltip: "Authentication required."
- Show an "Authenticate" button that calls
  `context_server_store.authenticate_server(id, cx)`.

### Gear menu changes

In the context server configuration popover menu:

- When the server is `Running` and has OAuth tokens stored (check keychain),
  add a "Log Out" entry that calls
  `context_server_store.logout_server(id, cx)`.

### One-time toast

When a server first transitions to `AuthRequired`:

- Use the `Dismissable` trait with key
  `"mcp-oauth-auth-required-toast:{server_id}"`.
- If not dismissed, show a toast notification:
  "MCP server '{name}' requires authentication. Open settings to sign in."
- Mark as dismissed after showing.

This uses `KEY_VALUE_STORE` (per-workspace), which is fine since the toast is
informational and re-showing after workspace changes is acceptable.

---

## Component 5: External dependency — CIMD document

Zed Industries needs to host a metadata document at a stable URL. Suggested
location:

```
https://zed.dev/oauth/client-metadata.json
```

Contents:

```json
{
  "client_id": "https://zed.dev/oauth/client-metadata.json",
  "client_name": "Zed",
  "client_uri": "https://zed.dev",
  "logo_uri": "https://zed.dev/img/logo.png",
  "redirect_uris": [
    "http://127.0.0.1/callback"
  ],
  "grant_types": ["authorization_code"],
  "response_types": ["code"],
  "token_endpoint_auth_method": "none",
  "scope": "mcp:*"
}
```

The `redirect_uris` entry omits the port. Per OAuth 2.1 Section 7.5.1, auth
servers must not compare ports for loopback IP redirect URIs, so the runtime
redirect URI `http://127.0.0.1:<ephemeral_port>/callback` will match.

Until this document is deployed, CIMD will not work and the flow will fall
through to DCR or error. This is fine for development.

---

## Implementation order

### Phase 1: OAuth module (pure logic, no integration)

Write `oauth.rs` with the discovery, PKCE, token exchange, and token storage
functions. Test with mock HTTP responses. This phase has no UI and no transport
changes.

Rough scope:
- `ProtectedResourceMetadata` / `AuthServerMetadata` types and parsing.
- `WWW-Authenticate` header parsing.
- `discover()` function.
- PKCE verifier/challenge generation.
- `build_authorization_request()`.
- `start_callback_server()`.
- `exchange_code()` / `refresh_tokens()`.
- Token keychain storage functions.
- Client registration (CIMD path + DCR path).
- DCR client ID persistence in KV store.

### Phase 2: Transport integration

Modify `HttpTransport` to support the token provider and 401 detection.
Introduce `TransportError`. Test with mock HTTP client returning 401s.

### Phase 3: State management

Add `AuthRequired` state/status variant. Wire up the store to detect auth
errors on startup, run discovery, and expose `authenticate_server` /
`logout_server`. Wire up cached token loading on startup.

### Phase 4: UI

Add the Authenticate button, Log Out menu entry, and one-time toast. This is
the smallest phase in terms of new code but touches the most files.

### Phase 5: External

Deploy the CIMD document to zed.dev. Until then, all testing uses servers that
support DCR, or a locally-hosted CIMD document.

---

## Scope explicitly deferred

- **Step-up authorization / 403 handling**: the spec defines a flow for
  requesting additional scopes at runtime. We parse 403 responses but surface
  them as errors for now.
- **Preregistered client IDs**: the spec's third registration mechanism. We
  can add a `client_id` field to HTTP server settings later.
- **OpenID Connect specific features**: we support OIDC discovery as a
  fallback for finding auth server metadata, but we don't use ID tokens or
  userinfo.
- **Multiple authorization servers**: the resource metadata can list multiple
  auth servers. We pick the first one. Supporting server selection can come
  later.
- **Private key JWT client authentication**: the CIMD spec allows
  `private_key_jwt` for client auth. We use `none` (public client) for now.