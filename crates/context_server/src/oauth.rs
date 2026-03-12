//! OAuth 2.0 authentication for MCP servers using the Authorization Code +
//! PKCE flow, per the MCP spec's OAuth profile.
//!
//! The flow is split into two phases:
//!
//! 1. **Discovery** ([`discover`]) fetches Protected Resource Metadata and
//!    Authorization Server Metadata. This can happen early (e.g. on a 401
//!    during server startup) because it doesn't need the redirect URI yet.
//!
//! 2. **Client registration** ([`resolve_client_registration`]) is separate
//!    because DCR requires the actual loopback redirect URI, which includes an
//!    ephemeral port that only exists once the callback server has started.
//!
//! After authentication, the full state is captured in [`OAuthSession`] which
//! is persisted to the keychain. On next startup, the stored session feeds
//! directly into [`McpOAuthTokenProvider`], giving a refresh-capable provider
//! without requiring another browser flow.

use anyhow::{Context as _, Result, anyhow, bail};
use async_trait::async_trait;
use base64::Engine as _;
use futures::AsyncReadExt as _;
use futures::channel::mpsc;
use http_client::{AsyncBody, HttpClient, Request};
use parking_lot::Mutex as SyncMutex;
use rand::Rng as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use std::sync::Arc;
use std::time::{Duration, SystemTime};
use url::Url;
use util::ResultExt as _;

/// The CIMD URL where Zed's OAuth client metadata document is hosted.
pub const CIMD_URL: &str = "https://zed.dev/oauth/client-metadata.json";

/// Parsed from the MCP server's WWW-Authenticate header or well-known endpoint
/// per RFC 9728 (OAuth 2.0 Protected Resource Metadata).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedResourceMetadata {
    pub resource: Url,
    pub authorization_servers: Vec<Url>,
    pub scopes_supported: Option<Vec<String>>,
}

/// Parsed from the authorization server's .well-known endpoint
/// per RFC 8414 (OAuth 2.0 Authorization Server Metadata).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthServerMetadata {
    pub issuer: Url,
    pub authorization_endpoint: Url,
    pub token_endpoint: Url,
    pub registration_endpoint: Option<Url>,
    pub scopes_supported: Option<Vec<String>>,
    pub code_challenge_methods_supported: Option<Vec<String>>,
    pub client_id_metadata_document_supported: bool,
}

/// The result of client registration — either CIMD or DCR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientRegistration {
    pub client_id: String,
    /// Only present for DCR-minted registrations.
    pub client_secret: Option<String>,
}

/// Access and refresh tokens obtained from the token endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<SystemTime>,
}

/// Everything discovered before the browser flow starts. Client registration is
/// resolved separately, once the real redirect URI is known.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthDiscovery {
    pub resource_metadata: ProtectedResourceMetadata,
    pub auth_server_metadata: AuthServerMetadata,
    pub scopes: Vec<String>,
}

/// The persisted OAuth session for a context server.
///
/// Stored in the keychain so startup can restore a refresh-capable provider
/// without another browser flow. Deliberately excludes the full discovery
/// metadata to keep the serialized size well within keychain item limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthSession {
    pub token_endpoint: Url,
    pub resource: Url,
    pub client_registration: OAuthClientRegistration,
    pub tokens: OAuthTokens,
}

/// Error codes defined by RFC 6750 Section 3.1 for Bearer token authentication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BearerError {
    /// The request is missing a required parameter, includes an unsupported
    /// parameter or parameter value, or is otherwise malformed.
    InvalidRequest,
    /// The access token provided is expired, revoked, malformed, or invalid.
    InvalidToken,
    /// The request requires higher privileges than provided by the access token.
    InsufficientScope,
    /// An unrecognized error code (extension or future spec addition).
    Other,
}

impl BearerError {
    fn parse(value: &str) -> Self {
        match value {
            "invalid_request" => BearerError::InvalidRequest,
            "invalid_token" => BearerError::InvalidToken,
            "insufficient_scope" => BearerError::InsufficientScope,
            _ => BearerError::Other,
        }
    }
}

/// Fields extracted from a `WWW-Authenticate: Bearer` header.
///
/// Per RFC 9728 Section 5.1, MCP servers include `resource_metadata` to point
/// at the Protected Resource Metadata document. The optional `scope` parameter
/// (RFC 6750 Section 3) indicates scopes required for the request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WwwAuthenticate {
    pub resource_metadata: Option<Url>,
    pub scope: Option<Vec<String>>,
    /// The parsed `error` parameter per RFC 6750 Section 3.1.
    pub error: Option<BearerError>,
    pub error_description: Option<String>,
}

/// Parse a `WWW-Authenticate` header value.
///
/// Expects the `Bearer` scheme followed by comma-separated `key="value"` pairs.
/// Per RFC 6750 and RFC 9728, the relevant parameters are:
/// - `resource_metadata` — URL of the Protected Resource Metadata document
/// - `scope` — space-separated list of required scopes
/// - `error` — error code (e.g. "insufficient_scope")
/// - `error_description` — human-readable error description
pub fn parse_www_authenticate(header: &str) -> Result<WwwAuthenticate> {
    let header = header.trim();

    let params_str = if header.len() >= 6 && header[..6].eq_ignore_ascii_case("bearer") {
        header[6..].trim()
    } else {
        bail!("WWW-Authenticate header does not use Bearer scheme");
    };

    if params_str.is_empty() {
        return Ok(WwwAuthenticate {
            resource_metadata: None,
            scope: None,
            error: None,
            error_description: None,
        });
    }

    let params = parse_auth_params(params_str);

    let resource_metadata = params
        .get("resource_metadata")
        .map(|v| Url::parse(v))
        .transpose()
        .map_err(|e| anyhow!("invalid resource_metadata URL: {}", e))?;

    let scope = params
        .get("scope")
        .map(|v| v.split_whitespace().map(String::from).collect());

    let error = params.get("error").map(|v| BearerError::parse(v));
    let error_description = params.get("error_description").cloned();

    Ok(WwwAuthenticate {
        resource_metadata,
        scope,
        error,
        error_description,
    })
}

/// Parse comma-separated `key="value"` or `key=token` parameters from an
/// auth-param list (RFC 7235 Section 2.1).
fn parse_auth_params(input: &str) -> collections::HashMap<String, String> {
    let mut params = collections::HashMap::default();
    let mut remaining = input.trim();

    while !remaining.is_empty() {
        // Skip leading whitespace and commas.
        remaining = remaining.trim_start_matches(|c: char| c == ',' || c.is_whitespace());
        if remaining.is_empty() {
            break;
        }

        // Find the key (everything before '=').
        let eq_pos = match remaining.find('=') {
            Some(pos) => pos,
            None => break,
        };

        let key = remaining[..eq_pos].trim().to_lowercase();
        remaining = &remaining[eq_pos + 1..];
        remaining = remaining.trim_start();

        // Parse the value: either quoted or unquoted (token).
        let value;
        if remaining.starts_with('"') {
            // Quoted string: find the closing quote, handling escaped chars.
            remaining = &remaining[1..]; // skip opening quote
            let mut val = String::new();
            let mut chars = remaining.char_indices();
            loop {
                match chars.next() {
                    Some((_, '\\')) => {
                        // Escaped character — take the next char literally.
                        if let Some((_, c)) = chars.next() {
                            val.push(c);
                        }
                    }
                    Some((i, '"')) => {
                        remaining = &remaining[i + 1..];
                        break;
                    }
                    Some((_, c)) => val.push(c),
                    None => {
                        remaining = "";
                        break;
                    }
                }
            }
            value = val;
        } else {
            // Unquoted token: read until comma or whitespace.
            let end = remaining
                .find(|c: char| c == ',' || c.is_whitespace())
                .unwrap_or(remaining.len());
            value = remaining[..end].to_string();
            remaining = &remaining[end..];
        }

        if !key.is_empty() {
            params.insert(key, value);
        }
    }

    params
}

/// Construct the well-known Protected Resource Metadata URIs for a given MCP
/// server URL, per RFC 9728 Section 3.
///
/// Returns URIs in priority order:
/// 1. Path-specific: `https://<host>/.well-known/oauth-protected-resource/<path>`
/// 2. Root: `https://<host>/.well-known/oauth-protected-resource`
pub fn protected_resource_metadata_urls(server_url: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    let base = format!("{}://{}", server_url.scheme(), server_url.authority());

    let path = server_url.path().trim_start_matches('/');
    if !path.is_empty() {
        if let Ok(url) = Url::parse(&format!(
            "{}/.well-known/oauth-protected-resource/{}",
            base, path
        )) {
            urls.push(url);
        }
    }

    if let Ok(url) = Url::parse(&format!("{}/.well-known/oauth-protected-resource", base)) {
        urls.push(url);
    }

    urls
}

/// Construct the well-known Authorization Server Metadata URIs for a given
/// issuer URL, per RFC 8414 Section 3.1 and Section 5 (OIDC compat).
///
/// Returns URIs in priority order, which differs depending on whether the
/// issuer URL has a path component.
pub fn auth_server_metadata_urls(issuer: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    let base = format!("{}://{}", issuer.scheme(), issuer.authority());
    let path = issuer.path().trim_matches('/');

    if !path.is_empty() {
        // Issuer with path: try path-inserted variants first.
        if let Ok(url) = Url::parse(&format!(
            "{}/.well-known/oauth-authorization-server/{}",
            base, path
        )) {
            urls.push(url);
        }
        if let Ok(url) = Url::parse(&format!(
            "{}/.well-known/openid-configuration/{}",
            base, path
        )) {
            urls.push(url);
        }
        if let Ok(url) = Url::parse(&format!(
            "{}/{}/.well-known/openid-configuration",
            base, path
        )) {
            urls.push(url);
        }
    } else {
        // No path: standard well-known locations.
        if let Ok(url) = Url::parse(&format!("{}/.well-known/oauth-authorization-server", base)) {
            urls.push(url);
        }
        if let Ok(url) = Url::parse(&format!("{}/.well-known/openid-configuration", base)) {
            urls.push(url);
        }
    }

    urls
}

// -- Canonical server URI (RFC 8707) -----------------------------------------

/// Derive the canonical resource URI for an MCP server URL, suitable for the
/// `resource` parameter in authorization and token requests per RFC 8707.
///
/// Lowercases the scheme and host, preserves the path (without trailing slash),
/// strips fragments and query strings.
pub fn canonical_server_uri(server_url: &Url) -> String {
    let mut uri = format!(
        "{}://{}",
        server_url.scheme().to_ascii_lowercase(),
        server_url.host_str().unwrap_or("").to_ascii_lowercase(),
    );
    if let Some(port) = server_url.port() {
        uri.push_str(&format!(":{}", port));
    }
    let path = server_url.path();
    if path != "/" {
        uri.push_str(path.trim_end_matches('/'));
    }
    uri
}

// -- Scope selection ---------------------------------------------------------

/// Select scopes following the MCP spec's Scope Selection Strategy:
/// 1. Use `scope` from the `WWW-Authenticate` challenge if present.
/// 2. Fall back to `scopes_supported` from Protected Resource Metadata.
/// 3. Return empty if neither is available.
pub fn select_scopes(
    www_authenticate: &WwwAuthenticate,
    resource_metadata: &ProtectedResourceMetadata,
) -> Vec<String> {
    if let Some(ref scopes) = www_authenticate.scope {
        if !scopes.is_empty() {
            return scopes.clone();
        }
    }
    resource_metadata
        .scopes_supported
        .clone()
        .unwrap_or_default()
}

// -- Client registration strategy --------------------------------------------

/// The registration approach to use, determined from auth server metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientRegistrationStrategy {
    /// The auth server supports CIMD. Use the CIMD URL as client_id directly.
    Cimd { client_id: String },
    /// The auth server has a registration endpoint. Caller must POST to it.
    Dcr { registration_endpoint: Url },
    /// No supported registration mechanism.
    Unavailable,
}

/// Determine how to register with the authorization server, following the
/// spec's recommended priority: CIMD first, DCR fallback.
pub fn determine_registration_strategy(
    auth_server_metadata: &AuthServerMetadata,
) -> ClientRegistrationStrategy {
    if auth_server_metadata.client_id_metadata_document_supported {
        ClientRegistrationStrategy::Cimd {
            client_id: CIMD_URL.to_string(),
        }
    } else if let Some(ref endpoint) = auth_server_metadata.registration_endpoint {
        ClientRegistrationStrategy::Dcr {
            registration_endpoint: endpoint.clone(),
        }
    } else {
        ClientRegistrationStrategy::Unavailable
    }
}

// -- PKCE (RFC 7636) ---------------------------------------------------------

/// A PKCE code verifier and its S256 challenge.
#[derive(Debug, Clone)]
pub struct PkceChallenge {
    pub verifier: String,
    pub challenge: String,
}

/// Generate a PKCE code verifier and S256 challenge per RFC 7636.
///
/// The verifier is 43 base64url characters derived from 32 random bytes.
/// The challenge is `BASE64URL(SHA256(verifier))`.
pub fn generate_pkce_challenge() -> PkceChallenge {
    let mut random_bytes = [0u8; 32];
    rand::rng().fill(&mut random_bytes);
    let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let verifier = engine.encode(&random_bytes);

    let digest = Sha256::digest(verifier.as_bytes());
    let challenge = engine.encode(digest);

    PkceChallenge {
        verifier,
        challenge,
    }
}

// -- Authorization URL construction ------------------------------------------

/// Build the authorization URL for the OAuth Authorization Code + PKCE flow.
pub fn build_authorization_url(
    auth_server_metadata: &AuthServerMetadata,
    client_id: &str,
    redirect_uri: &str,
    scopes: &[String],
    resource: &str,
    pkce: &PkceChallenge,
    state: &str,
) -> Url {
    let mut url = auth_server_metadata.authorization_endpoint.clone();
    {
        let mut query = url.query_pairs_mut();
        query.append_pair("response_type", "code");
        query.append_pair("client_id", client_id);
        query.append_pair("redirect_uri", redirect_uri);
        if !scopes.is_empty() {
            query.append_pair("scope", &scopes.join(" "));
        }
        query.append_pair("resource", resource);
        query.append_pair("code_challenge", &pkce.challenge);
        query.append_pair("code_challenge_method", "S256");
        query.append_pair("state", state);
    }
    url
}

// -- Token endpoint request bodies -------------------------------------------

/// The JSON body returned by the token endpoint on success.
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<u64>,
    #[serde(default)]
    pub token_type: Option<String>,
}

impl TokenResponse {
    /// Convert into `OAuthTokens`, computing `expires_at` from `expires_in`.
    pub fn into_tokens(self) -> OAuthTokens {
        let expires_at = self
            .expires_in
            .map(|secs| SystemTime::now() + Duration::from_secs(secs));
        OAuthTokens {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            expires_at,
        }
    }
}

/// Build the form-encoded body for an authorization code token exchange.
pub fn token_exchange_params(
    code: &str,
    client_id: &str,
    redirect_uri: &str,
    code_verifier: &str,
    resource: &str,
) -> Vec<(&'static str, String)> {
    vec![
        ("grant_type", "authorization_code".to_string()),
        ("code", code.to_string()),
        ("redirect_uri", redirect_uri.to_string()),
        ("client_id", client_id.to_string()),
        ("code_verifier", code_verifier.to_string()),
        ("resource", resource.to_string()),
    ]
}

/// Build the form-encoded body for a token refresh request.
pub fn token_refresh_params(
    refresh_token: &str,
    client_id: &str,
    resource: &str,
) -> Vec<(&'static str, String)> {
    vec![
        ("grant_type", "refresh_token".to_string()),
        ("refresh_token", refresh_token.to_string()),
        ("client_id", client_id.to_string()),
        ("resource", resource.to_string()),
    ]
}

// -- DCR request body (RFC 7591) ---------------------------------------------

/// Build the JSON body for a Dynamic Client Registration request.
///
/// The `redirect_uri` should be the actual loopback URI with the ephemeral
/// port (e.g. `http://127.0.0.1:12345/callback`). Some auth servers do strict
/// redirect URI matching even for loopback addresses, so we register the
/// exact URI we intend to use.
pub fn dcr_registration_body(redirect_uri: &str) -> serde_json::Value {
    serde_json::json!({
        "client_name": "Zed",
        "redirect_uris": [redirect_uri],
        "grant_types": ["authorization_code"],
        "response_types": ["code"],
        "token_endpoint_auth_method": "none"
    })
}

// -- Discovery (async, hits real endpoints) ----------------------------------

/// Fetch Protected Resource Metadata from the MCP server.
///
/// Tries the `resource_metadata` URL from the `WWW-Authenticate` header first,
/// then falls back to well-known URIs constructed from `server_url`.
pub async fn fetch_protected_resource_metadata(
    http_client: &Arc<dyn HttpClient>,
    server_url: &Url,
    www_authenticate: &WwwAuthenticate,
) -> Result<ProtectedResourceMetadata> {
    let candidate_urls = if let Some(ref url) = www_authenticate.resource_metadata {
        vec![url.clone()]
    } else {
        protected_resource_metadata_urls(server_url)
    };

    for url in &candidate_urls {
        match fetch_json::<ProtectedResourceMetadataResponse>(http_client, url).await {
            Ok(response) => {
                if response.authorization_servers.is_empty() {
                    bail!(
                        "Protected Resource Metadata at {} has no authorization_servers",
                        url
                    );
                }
                return Ok(ProtectedResourceMetadata {
                    resource: response.resource.unwrap_or_else(|| server_url.clone()),
                    authorization_servers: response.authorization_servers,
                    scopes_supported: response.scopes_supported,
                });
            }
            Err(err) => {
                log::debug!(
                    "Failed to fetch Protected Resource Metadata from {}: {}",
                    url,
                    err
                );
            }
        }
    }

    bail!(
        "Could not fetch Protected Resource Metadata for {}",
        server_url
    )
}

/// Fetch Authorization Server Metadata, trying RFC 8414 and OIDC Discovery
/// endpoints in the priority order specified by the MCP spec.
pub async fn fetch_auth_server_metadata(
    http_client: &Arc<dyn HttpClient>,
    issuer: &Url,
) -> Result<AuthServerMetadata> {
    let candidate_urls = auth_server_metadata_urls(issuer);

    for url in &candidate_urls {
        match fetch_json::<AuthServerMetadataResponse>(http_client, url).await {
            Ok(response) => {
                return Ok(AuthServerMetadata {
                    issuer: response.issuer.unwrap_or_else(|| issuer.clone()),
                    authorization_endpoint: response
                        .authorization_endpoint
                        .ok_or_else(|| anyhow!("missing authorization_endpoint"))?,
                    token_endpoint: response
                        .token_endpoint
                        .ok_or_else(|| anyhow!("missing token_endpoint"))?,
                    registration_endpoint: response.registration_endpoint,
                    scopes_supported: response.scopes_supported,
                    code_challenge_methods_supported: response.code_challenge_methods_supported,
                    client_id_metadata_document_supported: response
                        .client_id_metadata_document_supported
                        .unwrap_or(false),
                });
            }
            Err(err) => {
                log::debug!("Failed to fetch Auth Server Metadata from {}: {}", url, err);
            }
        }
    }

    bail!(
        "Could not fetch Authorization Server Metadata for {}",
        issuer
    )
}

/// Run the full discovery flow: fetch resource metadata, then auth server
/// metadata, then select scopes. Client registration is resolved separately,
/// once the real redirect URI is known.
pub async fn discover(
    http_client: &Arc<dyn HttpClient>,
    server_url: &Url,
    www_authenticate: &WwwAuthenticate,
) -> Result<OAuthDiscovery> {
    let resource_metadata =
        fetch_protected_resource_metadata(http_client, server_url, www_authenticate).await?;

    let auth_server_url = resource_metadata
        .authorization_servers
        .first()
        .ok_or_else(|| anyhow!("no authorization servers in resource metadata"))?;

    let auth_server_metadata = fetch_auth_server_metadata(http_client, auth_server_url).await?;

    // Verify PKCE S256 support (spec requirement).
    match &auth_server_metadata.code_challenge_methods_supported {
        Some(methods) if methods.iter().any(|m| m == "S256") => {}
        Some(_) => bail!("authorization server does not support S256 PKCE"),
        None => bail!("authorization server does not advertise code_challenge_methods_supported"),
    }

    // Verify there is at least one supported registration strategy before we
    // present the server as ready to authenticate.
    match determine_registration_strategy(&auth_server_metadata) {
        ClientRegistrationStrategy::Cimd { .. } | ClientRegistrationStrategy::Dcr { .. } => {}
        ClientRegistrationStrategy::Unavailable => {
            bail!("authorization server supports neither CIMD nor DCR")
        }
    }

    let scopes = select_scopes(www_authenticate, &resource_metadata);

    Ok(OAuthDiscovery {
        resource_metadata,
        auth_server_metadata,
        scopes,
    })
}

/// Resolve the OAuth client registration for an authorization flow.
///
/// CIMD uses the static client metadata document directly. For DCR, a fresh
/// registration is performed each time because the loopback redirect URI
/// includes an ephemeral port that changes every flow.
pub async fn resolve_client_registration(
    http_client: &Arc<dyn HttpClient>,
    discovery: &OAuthDiscovery,
    redirect_uri: &str,
) -> Result<OAuthClientRegistration> {
    match determine_registration_strategy(&discovery.auth_server_metadata) {
        ClientRegistrationStrategy::Cimd { client_id } => Ok(OAuthClientRegistration {
            client_id,
            client_secret: None,
        }),
        ClientRegistrationStrategy::Dcr {
            registration_endpoint,
        } => perform_dcr(http_client, &registration_endpoint, redirect_uri).await,
        ClientRegistrationStrategy::Unavailable => {
            bail!("authorization server supports neither CIMD nor DCR")
        }
    }
}

// -- Dynamic Client Registration (RFC 7591) ----------------------------------

/// Perform Dynamic Client Registration with the authorization server.
pub async fn perform_dcr(
    http_client: &Arc<dyn HttpClient>,
    registration_endpoint: &Url,
    redirect_uri: &str,
) -> Result<OAuthClientRegistration> {
    let body = dcr_registration_body(redirect_uri);
    let body_bytes = serde_json::to_vec(&body)?;

    let request = Request::builder()
        .method(http_client::http::Method::POST)
        .uri(registration_endpoint.as_str())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(AsyncBody::from(body_bytes))?;

    let mut response = http_client.send(request).await?;

    if !response.status().is_success() {
        let mut error_body = String::new();
        response.body_mut().read_to_string(&mut error_body).await?;
        bail!(
            "DCR failed with status {}: {}",
            response.status(),
            error_body
        );
    }

    let mut response_body = String::new();
    response
        .body_mut()
        .read_to_string(&mut response_body)
        .await?;

    let dcr_response: DcrResponse =
        serde_json::from_str(&response_body).context("failed to parse DCR response")?;

    Ok(OAuthClientRegistration {
        client_id: dcr_response.client_id,
        client_secret: dcr_response.client_secret,
    })
}

// -- Token exchange and refresh (async) --------------------------------------

/// Exchange an authorization code for tokens at the token endpoint.
pub async fn exchange_code(
    http_client: &Arc<dyn HttpClient>,
    auth_server_metadata: &AuthServerMetadata,
    code: &str,
    client_id: &str,
    redirect_uri: &str,
    code_verifier: &str,
    resource: &str,
) -> Result<OAuthTokens> {
    let params = token_exchange_params(code, client_id, redirect_uri, code_verifier, resource);
    post_token_request(http_client, &auth_server_metadata.token_endpoint, &params).await
}

/// Refresh tokens using a refresh token.
pub async fn refresh_tokens(
    http_client: &Arc<dyn HttpClient>,
    token_endpoint: &Url,
    refresh_token: &str,
    client_id: &str,
    resource: &str,
) -> Result<OAuthTokens> {
    let params = token_refresh_params(refresh_token, client_id, resource);
    post_token_request(http_client, token_endpoint, &params).await
}

/// POST form-encoded parameters to a token endpoint and parse the response.
async fn post_token_request(
    http_client: &Arc<dyn HttpClient>,
    token_endpoint: &Url,
    params: &[(&str, String)],
) -> Result<OAuthTokens> {
    let body = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params.iter().map(|(k, v)| (*k, v.as_str())))
        .finish();

    let request = Request::builder()
        .method(http_client::http::Method::POST)
        .uri(token_endpoint.as_str())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .body(AsyncBody::from(body.into_bytes()))?;

    let mut response = http_client.send(request).await?;

    if !response.status().is_success() {
        let mut error_body = String::new();
        response.body_mut().read_to_string(&mut error_body).await?;
        bail!(
            "token request failed with status {}: {}",
            response.status(),
            error_body
        );
    }

    let mut response_body = String::new();
    response
        .body_mut()
        .read_to_string(&mut response_body)
        .await?;

    let token_response: TokenResponse =
        serde_json::from_str(&response_body).context("failed to parse token response")?;

    Ok(token_response.into_tokens())
}

// -- Loopback HTTP callback server -------------------------------------------

/// An OAuth authorization callback received via the loopback HTTP server.
#[derive(Debug)]
pub struct OAuthCallback {
    pub code: String,
    pub state: String,
}

impl OAuthCallback {
    /// Parse the query string from a callback URL like
    /// `http://127.0.0.1:<port>/callback?code=...&state=...`.
    pub fn parse_query(query: &str) -> Result<Self> {
        let mut code: Option<String> = None;
        let mut state: Option<String> = None;

        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            match key.as_ref() {
                "code" => {
                    if !value.is_empty() {
                        code = Some(value.into_owned());
                    }
                }
                "state" => {
                    if !value.is_empty() {
                        state = Some(value.into_owned());
                    }
                }
                _ => {}
            }
            if code.is_some() && state.is_some() {
                break;
            }
        }

        let code = code.ok_or_else(|| anyhow!("missing 'code' parameter in OAuth callback"))?;
        let state = state.ok_or_else(|| anyhow!("missing 'state' parameter in OAuth callback"))?;

        Ok(Self { code, state })
    }
}

/// How long to wait for the browser to complete the OAuth flow before giving
/// up and releasing the loopback port.
const CALLBACK_TIMEOUT: Duration = Duration::from_secs(5 * 60);

/// The preferred fixed port for the OAuth callback server. This port is listed
/// in Zed's hosted CIMD (Client ID Metadata Document) at `zed.dev`, so using
/// it makes CIMD-based authentication work with authorization servers that do
/// strict redirect URI matching including port. When this port is unavailable
/// (e.g. another Zed instance is mid-auth), we fall back to an ephemeral port,
/// which still works with DCR and with RFC 8252-compliant servers that ignore
/// the port for loopback redirects.
///
/// A fixed port is safe here because PKCE (which we always use) prevents an
/// attacker who binds to this port from exchanging an intercepted authorization
/// code — they don't have the code verifier. See RFC 8252 Section 8.1.
const PREFERRED_CALLBACK_PORT: u16 = 27523;

/// Start a loopback HTTP server to receive the OAuth authorization callback.
///
/// Tries to bind to [`PREFERRED_CALLBACK_PORT`] first for CIMD compatibility,
/// falling back to an ephemeral port if the preferred port is unavailable.
///
/// Returns `(redirect_uri, callback_future)`. The caller should use the
/// redirect URI in the authorization request, open the browser, then await
/// the future to receive the callback.
///
/// The server accepts exactly one request on `/callback`, validates that it
/// contains `code` and `state` query parameters, responds with a minimal
/// HTML page telling the user they can close the tab, and shuts down.
///
/// The callback server shuts down when the returned oneshot receiver is
/// dropped (e.g. because the authentication task was cancelled), or after a
/// 5-minute timeout.
pub async fn start_callback_server() -> Result<(
    String,
    futures::channel::oneshot::Receiver<Result<OAuthCallback>>,
)> {
    let server = tiny_http::Server::http(format!("127.0.0.1:{}", PREFERRED_CALLBACK_PORT))
        .or_else(|_| {
            log::info!(
                "preferred OAuth callback port {} unavailable, using ephemeral port",
                PREFERRED_CALLBACK_PORT,
            );
            tiny_http::Server::http("127.0.0.1:0")
        })
        .map_err(|e| anyhow!(e).context("Failed to bind loopback listener for OAuth callback"))?;
    let port = server.server_addr().port();
    let redirect_uri = format!("http://127.0.0.1:{}/callback", port);

    let (tx, rx) = futures::channel::oneshot::channel();

    // `tiny_http` is blocking, so we run it on a background thread.
    // The `recv_timeout` loop lets us check for cancellation (the receiver
    // being dropped) and enforce an overall timeout.
    std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + CALLBACK_TIMEOUT;

        loop {
            if tx.is_canceled() {
                return;
            }
            let remaining = deadline.saturating_duration_since(std::time::Instant::now());
            if remaining.is_zero() {
                return;
            }

            let timeout = remaining.min(Duration::from_millis(500));
            let Some(request) = (match server.recv_timeout(timeout) {
                Ok(req) => req,
                Err(_) => {
                    let _ = tx.send(Err(anyhow!("OAuth callback server I/O error")));
                    return;
                }
            }) else {
                // Timeout with no request — loop back and check cancellation.
                continue;
            };

            let result = handle_callback_request(&request);

            let (status_code, body) = match &result {
                Ok(_) => (
                    200,
                    "<html><body><h1>Authorization successful</h1>\
                     <p>You can close this tab and return to Zed.</p></body></html>",
                ),
                Err(err) => {
                    log::error!("OAuth callback error: {}", err);
                    (
                        400,
                        "<html><body><h1>Authorization failed</h1>\
                         <p>Something went wrong. Please try again from Zed.</p></body></html>",
                    )
                }
            };

            let response = tiny_http::Response::from_string(body)
                .with_status_code(status_code)
                .with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap(),
                );
            request.respond(response).log_err();

            let _ = tx.send(result);
            return;
        }
    });

    Ok((redirect_uri, rx))
}

/// Extract the `code` and `state` query parameters from an OAuth callback
/// request to `/callback`.
fn handle_callback_request(request: &tiny_http::Request) -> Result<OAuthCallback> {
    let url = Url::parse(&format!("http://localhost{}", request.url()))
        .context("malformed callback request URL")?;

    if url.path() != "/callback" {
        bail!("unexpected path in OAuth callback: {}", url.path());
    }

    let query = url
        .query()
        .ok_or_else(|| anyhow!("OAuth callback has no query string"))?;
    OAuthCallback::parse_query(query)
}

// -- JSON fetch helper -------------------------------------------------------

async fn fetch_json<T: serde::de::DeserializeOwned>(
    http_client: &Arc<dyn HttpClient>,
    url: &Url,
) -> Result<T> {
    let request = Request::builder()
        .method(http_client::http::Method::GET)
        .uri(url.as_str())
        .header("Accept", "application/json")
        .body(AsyncBody::default())?;

    let mut response = http_client.send(request).await?;

    if !response.status().is_success() {
        bail!("HTTP {} fetching {}", response.status(), url);
    }

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    serde_json::from_str(&body).with_context(|| format!("failed to parse JSON from {}", url))
}

// -- Serde response types for discovery --------------------------------------

#[derive(Debug, Deserialize)]
struct ProtectedResourceMetadataResponse {
    #[serde(default)]
    resource: Option<Url>,
    #[serde(default)]
    authorization_servers: Vec<Url>,
    #[serde(default)]
    scopes_supported: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct AuthServerMetadataResponse {
    #[serde(default)]
    issuer: Option<Url>,
    #[serde(default)]
    authorization_endpoint: Option<Url>,
    #[serde(default)]
    token_endpoint: Option<Url>,
    #[serde(default)]
    registration_endpoint: Option<Url>,
    #[serde(default)]
    scopes_supported: Option<Vec<String>>,
    #[serde(default)]
    code_challenge_methods_supported: Option<Vec<String>>,
    #[serde(default)]
    client_id_metadata_document_supported: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct DcrResponse {
    client_id: String,
    #[serde(default)]
    client_secret: Option<String>,
}

/// Provides OAuth tokens to the HTTP transport layer.
///
/// The transport calls `access_token()` before each request. On a 401 response
/// it calls `try_refresh()` and retries once if the refresh succeeds.
#[async_trait]
pub trait OAuthTokenProvider: Send + Sync {
    /// Returns the current access token, if one is available.
    fn access_token(&self) -> Option<String>;

    /// Attempts to refresh the access token. Returns `true` if a new token was
    /// obtained and the request should be retried.
    async fn try_refresh(&self) -> Result<bool>;
}

/// Concrete `OAuthTokenProvider` backed by a full persisted OAuth session and
/// an HTTP client for token refresh. The same provider type is used both after
/// an interactive authentication flow and when restoring a saved session from
/// the keychain on startup.
pub struct McpOAuthTokenProvider {
    session: SyncMutex<OAuthSession>,
    http_client: Arc<dyn HttpClient>,
    token_refresh_tx: Option<mpsc::UnboundedSender<OAuthSession>>,
}

impl McpOAuthTokenProvider {
    pub fn new(
        session: OAuthSession,
        http_client: Arc<dyn HttpClient>,
        token_refresh_tx: Option<mpsc::UnboundedSender<OAuthSession>>,
    ) -> Self {
        Self {
            session: SyncMutex::new(session),
            http_client,
            token_refresh_tx,
        }
    }

    fn access_token_is_expired(tokens: &OAuthTokens) -> bool {
        tokens.expires_at.is_some_and(|expires_at| {
            SystemTime::now()
                .checked_add(Duration::from_secs(30))
                .is_some_and(|now_with_buffer| expires_at <= now_with_buffer)
        })
    }
}

#[async_trait]
impl OAuthTokenProvider for McpOAuthTokenProvider {
    fn access_token(&self) -> Option<String> {
        let session = self.session.lock();
        if Self::access_token_is_expired(&session.tokens) {
            return None;
        }
        Some(session.tokens.access_token.clone())
    }

    async fn try_refresh(&self) -> Result<bool> {
        let (refresh_token, token_endpoint, resource, client_id) = {
            let session = self.session.lock();
            match session.tokens.refresh_token.clone() {
                Some(refresh_token) => (
                    refresh_token,
                    session.token_endpoint.clone(),
                    session.resource.clone(),
                    session.client_registration.client_id.clone(),
                ),
                None => return Ok(false),
            }
        };

        let resource_str = canonical_server_uri(&resource);

        match refresh_tokens(
            &self.http_client,
            &token_endpoint,
            &refresh_token,
            &client_id,
            &resource_str,
        )
        .await
        {
            Ok(mut new_tokens) => {
                if new_tokens.refresh_token.is_none() {
                    new_tokens.refresh_token = Some(refresh_token);
                }

                {
                    let mut session = self.session.lock();
                    session.tokens = new_tokens;

                    if let Some(ref tx) = self.token_refresh_tx {
                        tx.unbounded_send(session.clone()).ok();
                    }
                }

                Ok(true)
            }
            Err(err) => {
                log::warn!("OAuth token refresh failed: {}", err);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_client::Response;

    #[test]
    fn test_parse_www_authenticate_with_resource_metadata_and_scope() {
        let header = r#"Bearer resource_metadata="https://mcp.example.com/.well-known/oauth-protected-resource", scope="files:read user:profile""#;
        let result = parse_www_authenticate(header).unwrap();

        assert_eq!(
            result.resource_metadata.as_ref().map(|u| u.as_str()),
            Some("https://mcp.example.com/.well-known/oauth-protected-resource")
        );
        assert_eq!(
            result.scope,
            Some(vec!["files:read".to_string(), "user:profile".to_string()])
        );
        assert_eq!(result.error, None);
    }

    #[test]
    fn test_parse_www_authenticate_resource_metadata_only() {
        let header = r#"Bearer resource_metadata="https://mcp.example.com/.well-known/oauth-protected-resource""#;
        let result = parse_www_authenticate(header).unwrap();

        assert_eq!(
            result.resource_metadata.as_ref().map(|u| u.as_str()),
            Some("https://mcp.example.com/.well-known/oauth-protected-resource")
        );
        assert_eq!(result.scope, None);
    }

    #[test]
    fn test_parse_www_authenticate_bare_bearer() {
        let result = parse_www_authenticate("Bearer").unwrap();
        assert_eq!(result.resource_metadata, None);
        assert_eq!(result.scope, None);
    }

    #[test]
    fn test_parse_www_authenticate_with_error() {
        let header = r#"Bearer error="insufficient_scope", scope="files:read files:write", resource_metadata="https://mcp.example.com/.well-known/oauth-protected-resource", error_description="Additional file write permission required""#;
        let result = parse_www_authenticate(header).unwrap();

        assert_eq!(result.error, Some(BearerError::InsufficientScope));
        assert_eq!(
            result.error_description.as_deref(),
            Some("Additional file write permission required")
        );
        assert_eq!(
            result.scope,
            Some(vec!["files:read".to_string(), "files:write".to_string()])
        );
        assert!(result.resource_metadata.is_some());
    }

    #[test]
    fn test_parse_www_authenticate_invalid_token_error() {
        let header =
            r#"Bearer error="invalid_token", error_description="The access token expired""#;
        let result = parse_www_authenticate(header).unwrap();
        assert_eq!(result.error, Some(BearerError::InvalidToken));
    }

    #[test]
    fn test_parse_www_authenticate_invalid_request_error() {
        let header = r#"Bearer error="invalid_request""#;
        let result = parse_www_authenticate(header).unwrap();
        assert_eq!(result.error, Some(BearerError::InvalidRequest));
    }

    #[test]
    fn test_parse_www_authenticate_unknown_error() {
        let header = r#"Bearer error="some_future_error""#;
        let result = parse_www_authenticate(header).unwrap();
        assert_eq!(result.error, Some(BearerError::Other));
    }

    #[test]
    fn test_parse_www_authenticate_rejects_non_bearer() {
        let result = parse_www_authenticate("Basic realm=\"example\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_www_authenticate_case_insensitive_scheme() {
        let header = r#"bearer resource_metadata="https://example.com/.well-known/oauth-protected-resource""#;
        let result = parse_www_authenticate(header).unwrap();
        assert!(result.resource_metadata.is_some());
    }

    #[test]
    fn test_parse_www_authenticate_multiline_style() {
        // Some servers emit the header spread across multiple lines joined by
        // whitespace, as shown in the spec examples.
        let header = "Bearer resource_metadata=\"https://mcp.example.com/.well-known/oauth-protected-resource\",\n                         scope=\"files:read\"";
        let result = parse_www_authenticate(header).unwrap();
        assert!(result.resource_metadata.is_some());
        assert_eq!(result.scope, Some(vec!["files:read".to_string()]));
    }

    #[test]
    fn test_protected_resource_metadata_urls_with_path() {
        let server_url = Url::parse("https://api.example.com/v1/mcp").unwrap();
        let urls = protected_resource_metadata_urls(&server_url);

        assert_eq!(urls.len(), 2);
        assert_eq!(
            urls[0].as_str(),
            "https://api.example.com/.well-known/oauth-protected-resource/v1/mcp"
        );
        assert_eq!(
            urls[1].as_str(),
            "https://api.example.com/.well-known/oauth-protected-resource"
        );
    }

    #[test]
    fn test_protected_resource_metadata_urls_without_path() {
        let server_url = Url::parse("https://mcp.example.com").unwrap();
        let urls = protected_resource_metadata_urls(&server_url);

        assert_eq!(urls.len(), 1);
        assert_eq!(
            urls[0].as_str(),
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
    }

    #[test]
    fn test_auth_server_metadata_urls_with_path() {
        let issuer = Url::parse("https://auth.example.com/tenant1").unwrap();
        let urls = auth_server_metadata_urls(&issuer);

        assert_eq!(urls.len(), 3);
        assert_eq!(
            urls[0].as_str(),
            "https://auth.example.com/.well-known/oauth-authorization-server/tenant1"
        );
        assert_eq!(
            urls[1].as_str(),
            "https://auth.example.com/.well-known/openid-configuration/tenant1"
        );
        assert_eq!(
            urls[2].as_str(),
            "https://auth.example.com/tenant1/.well-known/openid-configuration"
        );
    }

    #[test]
    fn test_auth_server_metadata_urls_without_path() {
        let issuer = Url::parse("https://auth.example.com").unwrap();
        let urls = auth_server_metadata_urls(&issuer);

        assert_eq!(urls.len(), 2);
        assert_eq!(
            urls[0].as_str(),
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        assert_eq!(
            urls[1].as_str(),
            "https://auth.example.com/.well-known/openid-configuration"
        );
    }

    // -- Canonical server URI tests ------------------------------------------

    #[test]
    fn test_canonical_server_uri_simple() {
        let url = Url::parse("https://mcp.example.com").unwrap();
        assert_eq!(canonical_server_uri(&url), "https://mcp.example.com");
    }

    #[test]
    fn test_canonical_server_uri_with_path() {
        let url = Url::parse("https://mcp.example.com/v1/mcp").unwrap();
        assert_eq!(canonical_server_uri(&url), "https://mcp.example.com/v1/mcp");
    }

    #[test]
    fn test_canonical_server_uri_strips_trailing_slash() {
        let url = Url::parse("https://mcp.example.com/").unwrap();
        assert_eq!(canonical_server_uri(&url), "https://mcp.example.com");
    }

    #[test]
    fn test_canonical_server_uri_preserves_port() {
        let url = Url::parse("https://mcp.example.com:8443").unwrap();
        assert_eq!(canonical_server_uri(&url), "https://mcp.example.com:8443");
    }

    #[test]
    fn test_canonical_server_uri_lowercases() {
        let url = Url::parse("HTTPS://MCP.Example.COM/Server/MCP").unwrap();
        assert_eq!(
            canonical_server_uri(&url),
            "https://mcp.example.com/Server/MCP"
        );
    }

    // -- Scope selection tests -----------------------------------------------

    #[test]
    fn test_select_scopes_prefers_www_authenticate() {
        let www_auth = WwwAuthenticate {
            resource_metadata: None,
            scope: Some(vec!["files:read".into()]),
            error: None,
            error_description: None,
        };
        let resource_meta = ProtectedResourceMetadata {
            resource: Url::parse("https://example.com").unwrap(),
            authorization_servers: vec![],
            scopes_supported: Some(vec!["files:read".into(), "files:write".into()]),
        };
        assert_eq!(select_scopes(&www_auth, &resource_meta), vec!["files:read"]);
    }

    #[test]
    fn test_select_scopes_falls_back_to_resource_metadata() {
        let www_auth = WwwAuthenticate {
            resource_metadata: None,
            scope: None,
            error: None,
            error_description: None,
        };
        let resource_meta = ProtectedResourceMetadata {
            resource: Url::parse("https://example.com").unwrap(),
            authorization_servers: vec![],
            scopes_supported: Some(vec!["admin".into()]),
        };
        assert_eq!(select_scopes(&www_auth, &resource_meta), vec!["admin"]);
    }

    #[test]
    fn test_select_scopes_empty_when_nothing_available() {
        let www_auth = WwwAuthenticate {
            resource_metadata: None,
            scope: None,
            error: None,
            error_description: None,
        };
        let resource_meta = ProtectedResourceMetadata {
            resource: Url::parse("https://example.com").unwrap(),
            authorization_servers: vec![],
            scopes_supported: None,
        };
        assert!(select_scopes(&www_auth, &resource_meta).is_empty());
    }

    // -- Client registration strategy tests ----------------------------------

    #[test]
    fn test_registration_strategy_prefers_cimd() {
        let metadata = AuthServerMetadata {
            issuer: Url::parse("https://auth.example.com").unwrap(),
            authorization_endpoint: Url::parse("https://auth.example.com/authorize").unwrap(),
            token_endpoint: Url::parse("https://auth.example.com/token").unwrap(),
            registration_endpoint: Some(Url::parse("https://auth.example.com/register").unwrap()),
            scopes_supported: None,
            code_challenge_methods_supported: Some(vec!["S256".into()]),
            client_id_metadata_document_supported: true,
        };
        assert_eq!(
            determine_registration_strategy(&metadata),
            ClientRegistrationStrategy::Cimd {
                client_id: CIMD_URL.to_string(),
            }
        );
    }

    #[test]
    fn test_registration_strategy_falls_back_to_dcr() {
        let reg_endpoint = Url::parse("https://auth.example.com/register").unwrap();
        let metadata = AuthServerMetadata {
            issuer: Url::parse("https://auth.example.com").unwrap(),
            authorization_endpoint: Url::parse("https://auth.example.com/authorize").unwrap(),
            token_endpoint: Url::parse("https://auth.example.com/token").unwrap(),
            registration_endpoint: Some(reg_endpoint.clone()),
            scopes_supported: None,
            code_challenge_methods_supported: Some(vec!["S256".into()]),
            client_id_metadata_document_supported: false,
        };
        assert_eq!(
            determine_registration_strategy(&metadata),
            ClientRegistrationStrategy::Dcr {
                registration_endpoint: reg_endpoint,
            }
        );
    }

    #[test]
    fn test_registration_strategy_unavailable() {
        let metadata = AuthServerMetadata {
            issuer: Url::parse("https://auth.example.com").unwrap(),
            authorization_endpoint: Url::parse("https://auth.example.com/authorize").unwrap(),
            token_endpoint: Url::parse("https://auth.example.com/token").unwrap(),
            registration_endpoint: None,
            scopes_supported: None,
            code_challenge_methods_supported: Some(vec!["S256".into()]),
            client_id_metadata_document_supported: false,
        };
        assert_eq!(
            determine_registration_strategy(&metadata),
            ClientRegistrationStrategy::Unavailable,
        );
    }

    // -- PKCE tests ----------------------------------------------------------

    #[test]
    fn test_pkce_challenge_verifier_length() {
        let pkce = generate_pkce_challenge();
        // 32 random bytes → 43 base64url chars (no padding).
        assert_eq!(pkce.verifier.len(), 43);
    }

    #[test]
    fn test_pkce_challenge_is_valid_base64url() {
        let pkce = generate_pkce_challenge();
        for c in pkce.verifier.chars().chain(pkce.challenge.chars()) {
            assert!(
                c.is_ascii_alphanumeric() || c == '-' || c == '_',
                "invalid base64url character: {}",
                c
            );
        }
    }

    #[test]
    fn test_pkce_challenge_is_s256_of_verifier() {
        let pkce = generate_pkce_challenge();
        let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let expected_digest = Sha256::digest(pkce.verifier.as_bytes());
        let expected_challenge = engine.encode(expected_digest);
        assert_eq!(pkce.challenge, expected_challenge);
    }

    #[test]
    fn test_pkce_challenges_are_unique() {
        let a = generate_pkce_challenge();
        let b = generate_pkce_challenge();
        assert_ne!(a.verifier, b.verifier);
    }

    // -- Authorization URL tests ---------------------------------------------

    #[test]
    fn test_build_authorization_url() {
        let metadata = AuthServerMetadata {
            issuer: Url::parse("https://auth.example.com").unwrap(),
            authorization_endpoint: Url::parse("https://auth.example.com/authorize").unwrap(),
            token_endpoint: Url::parse("https://auth.example.com/token").unwrap(),
            registration_endpoint: None,
            scopes_supported: None,
            code_challenge_methods_supported: Some(vec!["S256".into()]),
            client_id_metadata_document_supported: true,
        };
        let pkce = PkceChallenge {
            verifier: "test_verifier".into(),
            challenge: "test_challenge".into(),
        };
        let url = build_authorization_url(
            &metadata,
            "https://zed.dev/oauth/client-metadata.json",
            "http://127.0.0.1:12345/callback",
            &["files:read".into(), "files:write".into()],
            "https://mcp.example.com",
            &pkce,
            "random_state_123",
        );

        let pairs: std::collections::HashMap<_, _> = url.query_pairs().collect();
        assert_eq!(pairs.get("response_type").unwrap(), "code");
        assert_eq!(
            pairs.get("client_id").unwrap(),
            "https://zed.dev/oauth/client-metadata.json"
        );
        assert_eq!(
            pairs.get("redirect_uri").unwrap(),
            "http://127.0.0.1:12345/callback"
        );
        assert_eq!(pairs.get("scope").unwrap(), "files:read files:write");
        assert_eq!(pairs.get("resource").unwrap(), "https://mcp.example.com");
        assert_eq!(pairs.get("code_challenge").unwrap(), "test_challenge");
        assert_eq!(pairs.get("code_challenge_method").unwrap(), "S256");
        assert_eq!(pairs.get("state").unwrap(), "random_state_123");
    }

    #[test]
    fn test_build_authorization_url_omits_empty_scope() {
        let metadata = AuthServerMetadata {
            issuer: Url::parse("https://auth.example.com").unwrap(),
            authorization_endpoint: Url::parse("https://auth.example.com/authorize").unwrap(),
            token_endpoint: Url::parse("https://auth.example.com/token").unwrap(),
            registration_endpoint: None,
            scopes_supported: None,
            code_challenge_methods_supported: Some(vec!["S256".into()]),
            client_id_metadata_document_supported: false,
        };
        let pkce = PkceChallenge {
            verifier: "v".into(),
            challenge: "c".into(),
        };
        let url = build_authorization_url(
            &metadata,
            "client_123",
            "http://127.0.0.1:9999/callback",
            &[],
            "https://mcp.example.com",
            &pkce,
            "state",
        );

        let pairs: std::collections::HashMap<_, _> = url.query_pairs().collect();
        assert!(!pairs.contains_key("scope"));
    }

    // -- Token exchange / refresh param tests --------------------------------

    #[test]
    fn test_token_exchange_params() {
        let params = token_exchange_params(
            "auth_code_abc",
            "client_xyz",
            "http://127.0.0.1:5555/callback",
            "verifier_123",
            "https://mcp.example.com",
        );
        let map: std::collections::HashMap<&str, &str> =
            params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        assert_eq!(map["grant_type"], "authorization_code");
        assert_eq!(map["code"], "auth_code_abc");
        assert_eq!(map["redirect_uri"], "http://127.0.0.1:5555/callback");
        assert_eq!(map["client_id"], "client_xyz");
        assert_eq!(map["code_verifier"], "verifier_123");
        assert_eq!(map["resource"], "https://mcp.example.com");
    }

    #[test]
    fn test_token_refresh_params() {
        let params =
            token_refresh_params("refresh_token_abc", "client_xyz", "https://mcp.example.com");
        let map: std::collections::HashMap<&str, &str> =
            params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        assert_eq!(map["grant_type"], "refresh_token");
        assert_eq!(map["refresh_token"], "refresh_token_abc");
        assert_eq!(map["client_id"], "client_xyz");
        assert_eq!(map["resource"], "https://mcp.example.com");
    }

    // -- Token response tests ------------------------------------------------

    #[test]
    fn test_token_response_into_tokens_with_expiry() {
        let response: TokenResponse = serde_json::from_str(
            r#"{"access_token": "at_123", "refresh_token": "rt_456", "expires_in": 3600, "token_type": "Bearer"}"#,
        )
        .unwrap();

        let tokens = response.into_tokens();
        assert_eq!(tokens.access_token, "at_123");
        assert_eq!(tokens.refresh_token.as_deref(), Some("rt_456"));
        assert!(tokens.expires_at.is_some());
    }

    #[test]
    fn test_token_response_into_tokens_minimal() {
        let response: TokenResponse =
            serde_json::from_str(r#"{"access_token": "at_789"}"#).unwrap();

        let tokens = response.into_tokens();
        assert_eq!(tokens.access_token, "at_789");
        assert_eq!(tokens.refresh_token, None);
        assert_eq!(tokens.expires_at, None);
    }

    // -- DCR body test -------------------------------------------------------

    #[test]
    fn test_dcr_registration_body_shape() {
        let body = dcr_registration_body("http://127.0.0.1:12345/callback");
        assert_eq!(body["client_name"], "Zed");
        assert_eq!(body["redirect_uris"][0], "http://127.0.0.1:12345/callback");
        assert_eq!(body["grant_types"][0], "authorization_code");
        assert_eq!(body["response_types"][0], "code");
        assert_eq!(body["token_endpoint_auth_method"], "none");
    }

    // -- Test helpers for async/HTTP tests -----------------------------------

    fn make_fake_http_client(
        handler: impl Fn(
            http_client::Request<AsyncBody>,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = anyhow::Result<Response<AsyncBody>>> + Send>,
        > + Send
        + Sync
        + 'static,
    ) -> Arc<dyn HttpClient> {
        http_client::FakeHttpClient::create(handler) as Arc<dyn HttpClient>
    }

    fn json_response(status: u16, body: &str) -> anyhow::Result<Response<AsyncBody>> {
        Ok(Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(body.as_bytes().to_vec()))
            .unwrap())
    }

    // -- Discovery integration tests -----------------------------------------

    #[test]
    fn test_fetch_protected_resource_metadata() {
        smol::block_on(async {
            let client = make_fake_http_client(|req| {
                Box::pin(async move {
                    let uri = req.uri().to_string();
                    if uri.contains(".well-known/oauth-protected-resource") {
                        json_response(
                            200,
                            r#"{
                                "resource": "https://mcp.example.com",
                                "authorization_servers": ["https://auth.example.com"],
                                "scopes_supported": ["read", "write"]
                            }"#,
                        )
                    } else {
                        json_response(404, "{}")
                    }
                })
            });

            let server_url = Url::parse("https://mcp.example.com").unwrap();
            let www_auth = WwwAuthenticate {
                resource_metadata: None,
                scope: None,
                error: None,
                error_description: None,
            };

            let metadata = fetch_protected_resource_metadata(&client, &server_url, &www_auth)
                .await
                .unwrap();

            assert_eq!(metadata.resource.as_str(), "https://mcp.example.com/");
            assert_eq!(metadata.authorization_servers.len(), 1);
            assert_eq!(
                metadata.authorization_servers[0].as_str(),
                "https://auth.example.com/"
            );
            assert_eq!(
                metadata.scopes_supported,
                Some(vec!["read".to_string(), "write".to_string()])
            );
        });
    }

    #[test]
    fn test_fetch_protected_resource_metadata_prefers_www_authenticate_url() {
        smol::block_on(async {
            let client = make_fake_http_client(|req| {
                Box::pin(async move {
                    let uri = req.uri().to_string();
                    if uri == "https://mcp.example.com/custom-resource-metadata" {
                        json_response(
                            200,
                            r#"{
                                "resource": "https://mcp.example.com",
                                "authorization_servers": ["https://auth.example.com"]
                            }"#,
                        )
                    } else {
                        json_response(500, r#"{"error": "should not be called"}"#)
                    }
                })
            });

            let server_url = Url::parse("https://mcp.example.com").unwrap();
            let www_auth = WwwAuthenticate {
                resource_metadata: Some(
                    Url::parse("https://mcp.example.com/custom-resource-metadata").unwrap(),
                ),
                scope: None,
                error: None,
                error_description: None,
            };

            let metadata = fetch_protected_resource_metadata(&client, &server_url, &www_auth)
                .await
                .unwrap();

            assert_eq!(metadata.authorization_servers.len(), 1);
        });
    }

    #[test]
    fn test_fetch_auth_server_metadata() {
        smol::block_on(async {
            let client = make_fake_http_client(|req| {
                Box::pin(async move {
                    let uri = req.uri().to_string();
                    if uri.contains(".well-known/oauth-authorization-server") {
                        json_response(
                            200,
                            r#"{
                                "issuer": "https://auth.example.com",
                                "authorization_endpoint": "https://auth.example.com/authorize",
                                "token_endpoint": "https://auth.example.com/token",
                                "registration_endpoint": "https://auth.example.com/register",
                                "code_challenge_methods_supported": ["S256"],
                                "client_id_metadata_document_supported": true
                            }"#,
                        )
                    } else {
                        json_response(404, "{}")
                    }
                })
            });

            let issuer = Url::parse("https://auth.example.com").unwrap();
            let metadata = fetch_auth_server_metadata(&client, &issuer).await.unwrap();

            assert_eq!(metadata.issuer.as_str(), "https://auth.example.com/");
            assert_eq!(
                metadata.authorization_endpoint.as_str(),
                "https://auth.example.com/authorize"
            );
            assert_eq!(
                metadata.token_endpoint.as_str(),
                "https://auth.example.com/token"
            );
            assert!(metadata.registration_endpoint.is_some());
            assert!(metadata.client_id_metadata_document_supported);
            assert_eq!(
                metadata.code_challenge_methods_supported,
                Some(vec!["S256".to_string()])
            );
        });
    }

    #[test]
    fn test_fetch_auth_server_metadata_falls_back_to_oidc() {
        smol::block_on(async {
            let client = make_fake_http_client(|req| {
                Box::pin(async move {
                    let uri = req.uri().to_string();
                    if uri.contains("openid-configuration") {
                        json_response(
                            200,
                            r#"{
                                "issuer": "https://auth.example.com",
                                "authorization_endpoint": "https://auth.example.com/authorize",
                                "token_endpoint": "https://auth.example.com/token",
                                "code_challenge_methods_supported": ["S256"]
                            }"#,
                        )
                    } else {
                        json_response(404, "{}")
                    }
                })
            });

            let issuer = Url::parse("https://auth.example.com").unwrap();
            let metadata = fetch_auth_server_metadata(&client, &issuer).await.unwrap();

            assert_eq!(
                metadata.authorization_endpoint.as_str(),
                "https://auth.example.com/authorize"
            );
            assert!(!metadata.client_id_metadata_document_supported);
        });
    }

    // -- Full discover integration tests -------------------------------------

    #[test]
    fn test_full_discover_with_cimd() {
        smol::block_on(async {
            let client = make_fake_http_client(|req| {
                Box::pin(async move {
                    let uri = req.uri().to_string();
                    if uri.contains("oauth-protected-resource") {
                        json_response(
                            200,
                            r#"{
                                "resource": "https://mcp.example.com",
                                "authorization_servers": ["https://auth.example.com"],
                                "scopes_supported": ["mcp:read"]
                            }"#,
                        )
                    } else if uri.contains("oauth-authorization-server") {
                        json_response(
                            200,
                            r#"{
                                "issuer": "https://auth.example.com",
                                "authorization_endpoint": "https://auth.example.com/authorize",
                                "token_endpoint": "https://auth.example.com/token",
                                "code_challenge_methods_supported": ["S256"],
                                "client_id_metadata_document_supported": true
                            }"#,
                        )
                    } else {
                        json_response(404, "{}")
                    }
                })
            });

            let server_url = Url::parse("https://mcp.example.com").unwrap();
            let www_auth = WwwAuthenticate {
                resource_metadata: None,
                scope: None,
                error: None,
                error_description: None,
            };

            let discovery = discover(&client, &server_url, &www_auth).await.unwrap();
            let registration =
                resolve_client_registration(&client, &discovery, "http://127.0.0.1:12345/callback")
                    .await
                    .unwrap();

            assert_eq!(registration.client_id, CIMD_URL);
            assert_eq!(registration.client_secret, None);
            assert_eq!(discovery.scopes, vec!["mcp:read"]);
        });
    }

    #[test]
    fn test_full_discover_with_dcr_fallback() {
        smol::block_on(async {
            let client = make_fake_http_client(|req| {
                Box::pin(async move {
                    let uri = req.uri().to_string();
                    if uri.contains("oauth-protected-resource") {
                        json_response(
                            200,
                            r#"{
                                "resource": "https://mcp.example.com",
                                "authorization_servers": ["https://auth.example.com"]
                            }"#,
                        )
                    } else if uri.contains("oauth-authorization-server") {
                        json_response(
                            200,
                            r#"{
                                "issuer": "https://auth.example.com",
                                "authorization_endpoint": "https://auth.example.com/authorize",
                                "token_endpoint": "https://auth.example.com/token",
                                "registration_endpoint": "https://auth.example.com/register",
                                "code_challenge_methods_supported": ["S256"],
                                "client_id_metadata_document_supported": false
                            }"#,
                        )
                    } else if uri.contains("/register") {
                        json_response(
                            201,
                            r#"{
                                "client_id": "dcr-minted-id-123",
                                "client_secret": "dcr-secret-456"
                            }"#,
                        )
                    } else {
                        json_response(404, "{}")
                    }
                })
            });

            let server_url = Url::parse("https://mcp.example.com").unwrap();
            let www_auth = WwwAuthenticate {
                resource_metadata: None,
                scope: Some(vec!["files:read".into()]),
                error: None,
                error_description: None,
            };

            let discovery = discover(&client, &server_url, &www_auth).await.unwrap();
            let registration =
                resolve_client_registration(&client, &discovery, "http://127.0.0.1:9999/callback")
                    .await
                    .unwrap();

            assert_eq!(registration.client_id, "dcr-minted-id-123");
            assert_eq!(
                registration.client_secret.as_deref(),
                Some("dcr-secret-456")
            );
            assert_eq!(discovery.scopes, vec!["files:read"]);
        });
    }

    #[test]
    fn test_discover_fails_without_pkce_support() {
        smol::block_on(async {
            let client = make_fake_http_client(|req| {
                Box::pin(async move {
                    let uri = req.uri().to_string();
                    if uri.contains("oauth-protected-resource") {
                        json_response(
                            200,
                            r#"{
                                "resource": "https://mcp.example.com",
                                "authorization_servers": ["https://auth.example.com"]
                            }"#,
                        )
                    } else if uri.contains("oauth-authorization-server") {
                        json_response(
                            200,
                            r#"{
                                "issuer": "https://auth.example.com",
                                "authorization_endpoint": "https://auth.example.com/authorize",
                                "token_endpoint": "https://auth.example.com/token"
                            }"#,
                        )
                    } else {
                        json_response(404, "{}")
                    }
                })
            });

            let server_url = Url::parse("https://mcp.example.com").unwrap();
            let www_auth = WwwAuthenticate {
                resource_metadata: None,
                scope: None,
                error: None,
                error_description: None,
            };

            let result = discover(&client, &server_url, &www_auth).await;
            assert!(result.is_err());
            let err_msg = result.unwrap_err().to_string();
            assert!(
                err_msg.contains("code_challenge_methods_supported"),
                "unexpected error: {}",
                err_msg
            );
        });
    }

    // -- Token exchange integration tests ------------------------------------

    #[test]
    fn test_exchange_code_success() {
        smol::block_on(async {
            let client = make_fake_http_client(|req| {
                Box::pin(async move {
                    let uri = req.uri().to_string();
                    if uri.contains("/token") {
                        json_response(
                            200,
                            r#"{
                                "access_token": "new_access_token",
                                "refresh_token": "new_refresh_token",
                                "expires_in": 3600,
                                "token_type": "Bearer"
                            }"#,
                        )
                    } else {
                        json_response(404, "{}")
                    }
                })
            });

            let metadata = AuthServerMetadata {
                issuer: Url::parse("https://auth.example.com").unwrap(),
                authorization_endpoint: Url::parse("https://auth.example.com/authorize").unwrap(),
                token_endpoint: Url::parse("https://auth.example.com/token").unwrap(),
                registration_endpoint: None,
                scopes_supported: None,
                code_challenge_methods_supported: Some(vec!["S256".into()]),
                client_id_metadata_document_supported: true,
            };

            let tokens = exchange_code(
                &client,
                &metadata,
                "auth_code_123",
                CIMD_URL,
                "http://127.0.0.1:9999/callback",
                "verifier_abc",
                "https://mcp.example.com",
            )
            .await
            .unwrap();

            assert_eq!(tokens.access_token, "new_access_token");
            assert_eq!(tokens.refresh_token.as_deref(), Some("new_refresh_token"));
            assert!(tokens.expires_at.is_some());
        });
    }

    #[test]
    fn test_refresh_tokens_success() {
        smol::block_on(async {
            let client = make_fake_http_client(|req| {
                Box::pin(async move {
                    let uri = req.uri().to_string();
                    if uri.contains("/token") {
                        json_response(
                            200,
                            r#"{
                                "access_token": "refreshed_token",
                                "expires_in": 1800,
                                "token_type": "Bearer"
                            }"#,
                        )
                    } else {
                        json_response(404, "{}")
                    }
                })
            });

            let token_endpoint = Url::parse("https://auth.example.com/token").unwrap();

            let tokens = refresh_tokens(
                &client,
                &token_endpoint,
                "old_refresh_token",
                CIMD_URL,
                "https://mcp.example.com",
            )
            .await
            .unwrap();

            assert_eq!(tokens.access_token, "refreshed_token");
            assert_eq!(tokens.refresh_token, None);
            assert!(tokens.expires_at.is_some());
        });
    }

    #[test]
    fn test_exchange_code_failure() {
        smol::block_on(async {
            let client = make_fake_http_client(|_req| {
                Box::pin(async move { json_response(400, r#"{"error": "invalid_grant"}"#) })
            });

            let metadata = AuthServerMetadata {
                issuer: Url::parse("https://auth.example.com").unwrap(),
                authorization_endpoint: Url::parse("https://auth.example.com/authorize").unwrap(),
                token_endpoint: Url::parse("https://auth.example.com/token").unwrap(),
                registration_endpoint: None,
                scopes_supported: None,
                code_challenge_methods_supported: Some(vec!["S256".into()]),
                client_id_metadata_document_supported: true,
            };

            let result = exchange_code(
                &client,
                &metadata,
                "bad_code",
                "client",
                "http://127.0.0.1:1/callback",
                "verifier",
                "https://mcp.example.com",
            )
            .await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("400"));
        });
    }

    // -- DCR integration tests -----------------------------------------------

    #[test]
    fn test_perform_dcr() {
        smol::block_on(async {
            let client = make_fake_http_client(|_req| {
                Box::pin(async move {
                    json_response(
                        201,
                        r#"{
                            "client_id": "dynamic-client-001",
                            "client_secret": "dynamic-secret-001"
                        }"#,
                    )
                })
            });

            let endpoint = Url::parse("https://auth.example.com/register").unwrap();
            let registration = perform_dcr(&client, &endpoint, "http://127.0.0.1:9999/callback")
                .await
                .unwrap();

            assert_eq!(registration.client_id, "dynamic-client-001");
            assert_eq!(
                registration.client_secret.as_deref(),
                Some("dynamic-secret-001")
            );
        });
    }

    #[test]
    fn test_perform_dcr_failure() {
        smol::block_on(async {
            let client = make_fake_http_client(|_req| {
                Box::pin(
                    async move { json_response(403, r#"{"error": "registration_not_allowed"}"#) },
                )
            });

            let endpoint = Url::parse("https://auth.example.com/register").unwrap();
            let result = perform_dcr(&client, &endpoint, "http://127.0.0.1:9999/callback").await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("403"));
        });
    }

    // -- OAuthCallback parse tests -------------------------------------------

    #[test]
    fn test_oauth_callback_parse_query() {
        let callback = OAuthCallback::parse_query("code=test_auth_code&state=test_state").unwrap();
        assert_eq!(callback.code, "test_auth_code");
        assert_eq!(callback.state, "test_state");
    }

    #[test]
    fn test_oauth_callback_parse_query_reversed_order() {
        let callback = OAuthCallback::parse_query("state=test_state&code=test_auth_code").unwrap();
        assert_eq!(callback.code, "test_auth_code");
        assert_eq!(callback.state, "test_state");
    }

    #[test]
    fn test_oauth_callback_parse_query_with_extra_params() {
        let callback =
            OAuthCallback::parse_query("code=test_auth_code&state=test_state&extra=ignored")
                .unwrap();
        assert_eq!(callback.code, "test_auth_code");
        assert_eq!(callback.state, "test_state");
    }

    #[test]
    fn test_oauth_callback_parse_query_missing_code() {
        let result = OAuthCallback::parse_query("state=test_state");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("code"));
    }

    #[test]
    fn test_oauth_callback_parse_query_missing_state() {
        let result = OAuthCallback::parse_query("code=test_auth_code");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("state"));
    }

    #[test]
    fn test_oauth_callback_parse_query_empty_code() {
        let result = OAuthCallback::parse_query("code=&state=test_state");
        assert!(result.is_err());
    }

    #[test]
    fn test_oauth_callback_parse_query_empty_state() {
        let result = OAuthCallback::parse_query("code=test_auth_code&state=");
        assert!(result.is_err());
    }

    #[test]
    fn test_oauth_callback_parse_query_url_encoded_values() {
        let callback = OAuthCallback::parse_query("code=abc%20def&state=test%3Dstate").unwrap();
        assert_eq!(callback.code, "abc def");
        assert_eq!(callback.state, "test=state");
    }

    // -- McpOAuthTokenProvider tests -----------------------------------------

    fn make_test_session(
        access_token: &str,
        refresh_token: Option<&str>,
        expires_at: Option<SystemTime>,
    ) -> OAuthSession {
        OAuthSession {
            token_endpoint: Url::parse("https://auth.example.com/token").unwrap(),
            resource: Url::parse("https://mcp.example.com").unwrap(),
            client_registration: OAuthClientRegistration {
                client_id: "test-client".into(),
                client_secret: None,
            },
            tokens: OAuthTokens {
                access_token: access_token.into(),
                refresh_token: refresh_token.map(String::from),
                expires_at,
            },
        }
    }

    #[test]
    fn test_mcp_oauth_provider_returns_none_when_token_expired() {
        let expired = SystemTime::now() - Duration::from_secs(60);
        let session = make_test_session("stale-token", Some("rt"), Some(expired));
        let provider = McpOAuthTokenProvider::new(
            session,
            make_fake_http_client(|_| Box::pin(async { unreachable!() })),
            None,
        );

        assert_eq!(provider.access_token(), None);
    }

    #[test]
    fn test_mcp_oauth_provider_returns_token_when_not_expired() {
        let far_future = SystemTime::now() + Duration::from_secs(3600);
        let session = make_test_session("valid-token", Some("rt"), Some(far_future));
        let provider = McpOAuthTokenProvider::new(
            session,
            make_fake_http_client(|_| Box::pin(async { unreachable!() })),
            None,
        );

        assert_eq!(provider.access_token().as_deref(), Some("valid-token"));
    }

    #[test]
    fn test_mcp_oauth_provider_returns_token_when_no_expiry() {
        let session = make_test_session("no-expiry-token", Some("rt"), None);
        let provider = McpOAuthTokenProvider::new(
            session,
            make_fake_http_client(|_| Box::pin(async { unreachable!() })),
            None,
        );

        assert_eq!(provider.access_token().as_deref(), Some("no-expiry-token"));
    }

    #[test]
    fn test_mcp_oauth_provider_refresh_without_refresh_token_returns_false() {
        smol::block_on(async {
            let session = make_test_session("token", None, None);
            let provider = McpOAuthTokenProvider::new(
                session,
                make_fake_http_client(|_| {
                    Box::pin(async { unreachable!("no HTTP call expected") })
                }),
                None,
            );

            let refreshed = provider.try_refresh().await.unwrap();
            assert!(!refreshed);
        });
    }

    #[test]
    fn test_mcp_oauth_provider_refresh_updates_session_and_notifies_channel() {
        smol::block_on(async {
            let session = make_test_session("old-access", Some("my-refresh-token"), None);
            let (tx, mut rx) = futures::channel::mpsc::unbounded();

            let http_client = make_fake_http_client(|_req| {
                Box::pin(async {
                    json_response(
                        200,
                        r#"{
                            "access_token": "new-access",
                            "refresh_token": "new-refresh",
                            "expires_in": 1800
                        }"#,
                    )
                })
            });

            let provider = McpOAuthTokenProvider::new(session, http_client, Some(tx));

            let refreshed = provider.try_refresh().await.unwrap();
            assert!(refreshed);
            assert_eq!(provider.access_token().as_deref(), Some("new-access"));

            let notified_session = rx
                .try_next()
                .unwrap()
                .expect("channel should have a session");
            assert_eq!(notified_session.tokens.access_token, "new-access");
            assert_eq!(
                notified_session.tokens.refresh_token.as_deref(),
                Some("new-refresh")
            );
        });
    }

    #[test]
    fn test_mcp_oauth_provider_refresh_preserves_old_refresh_token_when_server_omits_it() {
        smol::block_on(async {
            let session = make_test_session("old-access", Some("original-refresh"), None);
            let (tx, mut rx) = futures::channel::mpsc::unbounded();

            let http_client = make_fake_http_client(|_req| {
                Box::pin(async {
                    json_response(
                        200,
                        r#"{
                            "access_token": "new-access",
                            "expires_in": 900
                        }"#,
                    )
                })
            });

            let provider = McpOAuthTokenProvider::new(session, http_client, Some(tx));

            let refreshed = provider.try_refresh().await.unwrap();
            assert!(refreshed);

            let notified_session = rx
                .try_next()
                .unwrap()
                .expect("channel should have a session");
            assert_eq!(notified_session.tokens.access_token, "new-access");
            assert_eq!(
                notified_session.tokens.refresh_token.as_deref(),
                Some("original-refresh"),
            );
        });
    }

    #[test]
    fn test_mcp_oauth_provider_refresh_returns_false_on_http_error() {
        smol::block_on(async {
            let session = make_test_session("old-access", Some("my-refresh"), None);

            let http_client = make_fake_http_client(|_req| {
                Box::pin(async { json_response(401, r#"{"error": "invalid_grant"}"#) })
            });

            let provider = McpOAuthTokenProvider::new(session, http_client, None);

            let refreshed = provider.try_refresh().await.unwrap();
            assert!(!refreshed);
            // The old token should still be in place.
            assert_eq!(provider.access_token().as_deref(), Some("old-access"));
        });
    }
}
