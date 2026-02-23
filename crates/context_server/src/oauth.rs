use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use url::Url;

/// The CIMD URL where Zed's OAuth client metadata document is hosted.
pub const CIMD_URL: &str = "https://zed.dev/oauth/client-metadata.json";

/// Parsed from the MCP server's WWW-Authenticate header or well-known endpoint
/// per RFC 9728 (OAuth 2.0 Protected Resource Metadata).
#[derive(Debug, Clone)]
pub struct ProtectedResourceMetadata {
    pub resource: Url,
    pub authorization_servers: Vec<Url>,
    pub scopes_supported: Option<Vec<String>>,
}

/// Parsed from the authorization server's .well-known endpoint
/// per RFC 8414 (OAuth 2.0 Authorization Server Metadata).
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

/// Everything needed to kick off the browser flow, obtained during discovery.
/// Cached on the AuthRequired state so we don't re-discover on every attempt.
#[derive(Debug, Clone)]
pub struct OAuthDiscovery {
    pub resource_metadata: ProtectedResourceMetadata,
    pub auth_server_metadata: AuthServerMetadata,
    pub client_registration: OAuthClientRegistration,
    pub scopes: Vec<String>,
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
    /// The `error` parameter, if present (e.g. "insufficient_scope").
    pub error: Option<String>,
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

    // Strip the "Bearer" scheme prefix (case-insensitive).
    let params_str = header
        .strip_prefix("Bearer")
        .or_else(|| header.strip_prefix("bearer"))
        .or_else(|| header.strip_prefix("BEARER"))
        .ok_or_else(|| anyhow!("WWW-Authenticate header does not use Bearer scheme"))?
        .trim();

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

    let error = params.get("error").cloned();
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
    let random_bytes: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
    let verifier = base64_url_encode(&random_bytes);

    let digest = simple_sha256(verifier.as_bytes());
    let challenge = base64_url_encode(&digest);

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
pub fn dcr_registration_body() -> serde_json::Value {
    serde_json::json!({
        "client_name": "Zed",
        "redirect_uris": ["http://127.0.0.1/callback"],
        "grant_types": ["authorization_code"],
        "response_types": ["code"],
        "token_endpoint_auth_method": "none"
    })
}

// -- Helpers (vendored to avoid extra deps) ----------------------------------

/// Base64url-encode without padding, per RFC 4648 Section 5.
fn base64_url_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    let mut out = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        out.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
        out.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[(triple & 0x3F) as usize] as char);
        }
    }
    out
}

/// Minimal SHA-256 implementation (avoids pulling in a crypto crate just for
/// PKCE challenge derivation).
fn simple_sha256(data: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    // Pre-processing: pad the message.
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    // Process each 512-bit (64-byte) block.
    for block in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut result = [0u8; 32];
    for (i, val) in h.iter().enumerate() {
        result[i * 4..i * 4 + 4].copy_from_slice(&val.to_be_bytes());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(result.error.as_deref(), Some("insufficient_scope"));
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
        let expected_digest = simple_sha256(pkce.verifier.as_bytes());
        let expected_challenge = base64_url_encode(&expected_digest);
        assert_eq!(pkce.challenge, expected_challenge);
    }

    #[test]
    fn test_pkce_challenges_are_unique() {
        let a = generate_pkce_challenge();
        let b = generate_pkce_challenge();
        assert_ne!(a.verifier, b.verifier);
    }

    #[test]
    fn test_sha256_known_vector() {
        // SHA-256 of empty string.
        let hash = simple_sha256(b"");
        let hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(
            hex,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_sha256_hello() {
        let hash = simple_sha256(b"hello");
        let hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(
            hex,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
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
        let body = dcr_registration_body();
        assert_eq!(body["client_name"], "Zed");
        assert_eq!(body["redirect_uris"][0], "http://127.0.0.1/callback");
        assert_eq!(body["grant_types"][0], "authorization_code");
        assert_eq!(body["response_types"][0], "code");
        assert_eq!(body["token_endpoint_auth_method"], "none");
    }
}
