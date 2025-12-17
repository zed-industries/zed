use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, anyhow};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use collections::HashMap;
use credentials_provider::CredentialsProvider;
use futures::FutureExt;
use gpui::AsyncApp;
use http_client::{AsyncBody, HttpClient, Request, Response, http};
use parking_lot::Mutex;
use rand::{RngCore, rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use smol::Timer;
use smol::channel;
use tiny_http::Server;
use url::Url;
use urlencoding::decode;

const OAUTH_DISCOVERY_HEADER: &str = "MCP-Protocol-Version";
const OAUTH_DISCOVERY_VERSION: &str = "2024-11-05";
const TOKEN_TYPE_BEARER: &str = "bearer";
const DEFAULT_LOGIN_TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredOAuthTokens {
    pub server_name: String,
    pub url: String,
    pub client_id: String,
    pub token_endpoint: String,
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_at: Option<u64>,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthorizationMetadata {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    #[serde(default)]
    pub registration_endpoint: Option<String>,
    #[serde(default)]
    pub scopes_supported: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
struct ClientRegistrationResponse {
    client_id: String,
    #[serde(default)]
    client_secret: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TokenEndpointResponse {
    access_token: String,
    #[serde(default)]
    token_type: Option<String>,
    #[serde(default)]
    expires_in: Option<u64>,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    scope: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OAuthDiscoveryMetadata {
    #[serde(default)]
    authorization_endpoint: Option<String>,
    #[serde(default)]
    token_endpoint: Option<String>,
    #[serde(default)]
    registration_endpoint: Option<String>,
    #[serde(default)]
    scopes_supported: Option<Vec<String>>,
}

/// Lightweight OAuth helper for HTTP MCP servers.
#[derive(Clone)]
pub struct OAuthManager {
    server_name: String,
    server_url: Url,
    http_client: Arc<dyn HttpClient>,
    default_headers: HashMap<String, String>,
    tokens: Arc<Mutex<Option<StoredOAuthTokens>>>,
    discovery_override: Arc<Mutex<Option<Url>>>,
}

impl OAuthManager {
    pub fn new(
        server_name: String,
        server_url: Url,
        default_headers: HashMap<String, String>,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        let tokens = load_oauth_tokens(&server_name, server_url.as_str())
            .ok()
            .flatten();
        Self {
            server_name,
            server_url,
            http_client,
            default_headers,
            tokens: Arc::new(Mutex::new(tokens)),
            discovery_override: Arc::new(Mutex::new(None)),
        }
    }

    /// Load tokens from keychain asynchronously. Call this when you have access to AsyncApp.
    pub async fn load_from_keychain(
        &self,
        credentials_provider: &dyn CredentialsProvider,
        cx: &AsyncApp,
    ) -> Result<bool> {
        if let Some(tokens) = load_oauth_tokens_from_keychain(
            &self.server_name,
            self.server_url.as_str(),
            credentials_provider,
            cx,
        )
        .await?
        {
            *self.tokens.lock() = Some(tokens);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Save current tokens to keychain asynchronously.
    pub async fn save_to_keychain(
        &self,
        credentials_provider: &dyn CredentialsProvider,
        cx: &AsyncApp,
    ) -> Result<()> {
        let tokens = self.tokens.lock().clone();
        if let Some(tokens) = tokens {
            save_oauth_tokens_to_keychain(&self.server_name, &tokens, credentials_provider, cx)
                .await?;
        }
        Ok(())
    }

    /// Manually trigger OAuth authentication. This opens the browser for user to authenticate.
    /// Call this when user explicitly clicks an "Authenticate" button.
    pub async fn authenticate(&self) -> Result<()> {
        self.login(&[]).await
    }

    /// Check if authentication is needed (no valid tokens available)
    pub fn needs_authentication(&self) -> bool {
        let tokens = self.tokens.lock();
        match tokens.as_ref() {
            None => true,
            Some(tokens) => {
                if let Some(expires_at) = tokens.expires_at {
                    let now_ms = current_time_millis();
                    expires_at <= now_ms
                } else {
                    false
                }
            }
        }
    }

    /// Check if the user is currently authenticated (has valid tokens)
    pub fn is_authenticated(&self) -> bool {
        !self.needs_authentication()
    }

    /// Clear tokens (logout). Returns the URL that was used for the tokens.
    pub fn logout(&self) -> Option<String> {
        let tokens = self.tokens.lock().take();
        if let Some(tokens) = &tokens {
            if let Err(e) = delete_oauth_tokens(&self.server_name, &tokens.url) {
                log::warn!(
                    "Failed to delete OAuth tokens from file for '{}': {}",
                    self.server_name,
                    e
                );
            }
            log::info!("Logged out from '{}'", self.server_name);
        }
        tokens.map(|t| t.url)
    }

    /// Returns a bearer token if available, refreshing it when needed.
    pub async fn access_token(&self) -> Result<Option<String>> {
        let needs_refresh = {
            let tokens = self.tokens.lock();
            match tokens.as_ref().and_then(|tokens| tokens.expires_at) {
                None => {
                    log::debug!("No token stored for '{}'", self.server_name);
                    false
                }
                Some(expires_at) => {
                    let now_ms = current_time_millis();
                    let needs_refresh = expires_at <= now_ms.saturating_add(30_000);
                    if needs_refresh {
                        log::debug!(
                            "Token needs refresh for '{}' (expires_at={}, now={})",
                            self.server_name,
                            expires_at,
                            now_ms
                        );
                    }
                    needs_refresh
                }
            }
        };

        if needs_refresh {
            log::debug!("Refreshing token for '{}'...", self.server_name);
            if self.refresh_tokens().await?.is_none() {
                log::warn!("Token refresh returned None for '{}'", self.server_name);
                return Ok(None);
            }
            log::debug!("Token refreshed successfully for '{}'", self.server_name);
        }

        let token = self
            .tokens
            .lock()
            .as_ref()
            .map(|tokens| tokens.access_token.clone());

        if token.is_some() {
            log::debug!("Returning valid token for '{}'", self.server_name);
        } else {
            log::debug!("No token available for '{}'", self.server_name);
        }

        Ok(token)
    }

    /// Returns true if a refresh token is available.
    pub fn can_refresh(&self) -> bool {
        self.tokens
            .lock()
            .as_ref()
            .and_then(|tokens| tokens.refresh_token.as_ref())
            .is_some()
    }

    /// Attempt to refresh the access token without performing an interactive login flow.
    ///
    /// Returns `Ok(true)` if a refresh was performed and succeeded, `Ok(false)` if no refresh token
    /// is available, and `Err` if the refresh attempt failed.
    pub async fn refresh_access_token(&self) -> Result<bool> {
        Ok(self.refresh_tokens().await?.is_some())
    }

    /// Performs the OAuth login flow and persists tokens.
    pub async fn login(&self, scopes: &[String]) -> Result<()> {
        log::info!(
            "Starting OAuth login for '{}' with scopes: {:?}",
            self.server_name,
            scopes
        );

        let metadata = self
            .discover_metadata()
            .await?
            .ok_or_else(|| anyhow!("OAuth authentication required but not supported by this server. Please check the server configuration."))?;

        log::debug!("Discovered OAuth metadata for '{}'", self.server_name);

        let redirect_server = Arc::new(Server::http("127.0.0.1:0").map_err(|err| anyhow!(err))?);
        let redirect_uri = format!("http://{}/callback", redirect_server.server_addr());

        let registration = if let Some(endpoint) = metadata.registration_endpoint.clone() {
            Some(
                self.register_client(&endpoint, &redirect_uri)
                    .await
                    .context("Failed to register OAuth client with the server. The server may not support dynamic client registration.")?,
            )
        } else {
            None
        };

        let client_id = registration
            .as_ref()
            .map(|resp| resp.client_id.clone())
            .unwrap_or_else(|| self.server_name.clone());
        let client_secret = registration.and_then(|resp| resp.client_secret);
        let (code_verifier, code_challenge) = generate_pkce();
        let state = random_url_token();

        let auth_url = build_authorization_url(
            &metadata.authorization_endpoint,
            &client_id,
            &redirect_uri,
            &code_challenge,
            &state,
            scopes,
        )?;

        // Spawn blocking server waiting for callback
        let (tx, rx) = channel::bounded(1);
        spawn_callback_server(redirect_server.clone(), tx).await;

        if let Err(err) = open::that(&auth_url) {
            log::warn!("failed to open browser automatically: {err}");
            println!("Open this URL to authorize the MCP server:\n{auth_url}\n");
        }

        let callback = smol::future::or(rx.recv().map(|result| result.ok()), async {
            Timer::after(DEFAULT_LOGIN_TIMEOUT).await;
            None
        })
        .await
        .ok_or_else(|| anyhow!("OAuth login timed out after {} seconds. Please try again or check if your browser blocked the authorization page.", DEFAULT_LOGIN_TIMEOUT.as_secs()))?;

        anyhow::ensure!(
            callback.state == state,
            "OAuth security validation failed (state mismatch). This may indicate a security issue. Please try again."
        );

        let token = self
            .exchange_code_for_token(
                &metadata.token_endpoint,
                &client_id,
                client_secret.as_deref(),
                &redirect_uri,
                &code_verifier,
                &callback.code,
            )
            .await
            .context("Failed to exchange authorization code for access token. Please try logging in again.")?;

        let scopes = token
            .scope
            .as_ref()
            .map(|scope| scope.split_whitespace().map(str::to_string).collect())
            .unwrap_or_default();

        let expires_at = token.expires_in.map(|seconds| {
            log::debug!(
                "Token for '{}' expires in {} seconds",
                self.server_name,
                seconds
            );
            current_time_millis().saturating_add(seconds * 1000)
        });

        let stored = StoredOAuthTokens {
            server_name: self.server_name.clone(),
            url: self.server_url.to_string(),
            client_id,
            token_endpoint: metadata.token_endpoint.clone(),
            access_token: token.access_token.clone(),
            refresh_token: token.refresh_token,
            expires_at,
            scopes,
            client_secret: client_secret.clone(),
        };

        let token_preview = if stored.access_token.len() > 30 {
            format!(
                "{}...{}",
                stored.access_token.chars().take(15).collect::<String>(),
                stored
                    .access_token
                    .chars()
                    .skip(stored.access_token.len() - 10)
                    .collect::<String>()
            )
        } else {
            stored.access_token.chars().take(20).collect::<String>()
        };
        log::debug!(
            "Saving NEW OAuth token for '{}': {}",
            self.server_name,
            token_preview
        );
        save_oauth_tokens(&self.server_name, &stored)?;
        *self.tokens.lock() = Some(stored);
        log::info!(
            "OAuth login completed successfully for '{}'",
            self.server_name
        );

        Ok(())
    }

    /// Override discovery base URL (e.g., from a WWW-Authenticate resource_metadata hint).
    pub fn set_discovery_override(&self, url: Url) {
        *self.discovery_override.lock() = Some(url);
    }

    /// Handle a WWW-Authenticate header by honoring resource_metadata and scopes, then logging in.
    pub async fn handle_www_authenticate(&self, header: &str) -> Result<()> {
        let (resource_metadata, scopes) = parse_www_authenticate_header(header);

        if let Some(resource) = resource_metadata {
            let mut base = resource;
            base.set_path("/");
            base.set_query(None);
            base.set_fragment(None);
            self.set_discovery_override(base);
        }

        self.login(&scopes).await
    }

    async fn refresh_tokens(&self) -> Result<Option<String>> {
        let token_endpoint = match self.tokens.lock().as_ref() {
            Some(tokens) => tokens.token_endpoint.clone(),
            None => return Ok(None),
        };

        let refresh_token = match self
            .tokens
            .lock()
            .as_ref()
            .and_then(|t| t.refresh_token.clone())
        {
            Some(token) => token,
            None => return Ok(None),
        };

        let client_id = self
            .tokens
            .lock()
            .as_ref()
            .map(|t| t.client_id.clone())
            .unwrap_or_default();
        let client_secret = self
            .tokens
            .lock()
            .as_ref()
            .and_then(|t| t.client_secret.clone());

        let body = form_body(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token.as_str()),
            ("client_id", client_id.as_str()),
            (
                "client_secret",
                client_secret.as_deref().unwrap_or_default(),
            ),
        ]);

        let response = self
            .post_form(&token_endpoint, body)
            .await
            .context("failed to refresh OAuth token")?;

        let token = parse_token_response(response).await?;
        let expires_at = token
            .expires_in
            .map(|seconds| current_time_millis().saturating_add(seconds * 1000));

        let mut tokens = self.tokens.lock();
        let stored = tokens
            .as_mut()
            .ok_or_else(|| anyhow!("OAuth tokens missing during refresh"))?;
        stored.access_token = token.access_token.clone();
        stored.refresh_token = token.refresh_token.clone().or(stored.refresh_token.clone());
        stored.expires_at = expires_at;

        if let Err(err) = save_oauth_tokens(&self.server_name, stored) {
            log::warn!(
                "Failed to persist refreshed OAuth tokens for '{}': {}",
                self.server_name,
                err
            );
        }
        Ok(Some(token.access_token))
    }

    #[cfg(test)]
    pub fn set_tokens_for_test(&self, tokens: StoredOAuthTokens) {
        *self.tokens.lock() = Some(tokens);
    }

    async fn discover_metadata(&self) -> Result<Option<AuthorizationMetadata>> {
        let base = self
            .discovery_override
            .lock()
            .clone()
            .unwrap_or_else(|| self.server_url.clone());

        for path in discovery_paths(self.server_url.path()) {
            let mut url = base.clone();
            url.set_path(&path);
            url.set_query(None);
            if let Some(metadata) = self.fetch_metadata(&url).await? {
                return Ok(Some(metadata));
            }
        }
        Ok(None)
    }

    async fn fetch_metadata(&self, url: &Url) -> Result<Option<AuthorizationMetadata>> {
        let mut request = Request::builder()
            .method(http::Method::GET)
            .uri(url.as_str())
            .header(OAUTH_DISCOVERY_HEADER, OAUTH_DISCOVERY_VERSION);

        for (key, value) in &self.default_headers {
            request = request.header(key.as_str(), value.as_str());
        }

        let response = self
            .http_client
            .send(request.body(AsyncBody::empty())?)
            .await?;
        if !response.status().is_success() {
            return Ok(None);
        }

        let metadata: OAuthDiscoveryMetadata = parse_json_body(response).await?;
        if let (Some(auth_url), Some(token_url)) =
            (metadata.authorization_endpoint, metadata.token_endpoint)
        {
            return Ok(Some(AuthorizationMetadata {
                authorization_endpoint: auth_url,
                token_endpoint: token_url,
                registration_endpoint: metadata.registration_endpoint,
                scopes_supported: metadata.scopes_supported,
            }));
        }

        Ok(None)
    }

    async fn register_client(
        &self,
        endpoint: &str,
        redirect_uri: &str,
    ) -> Result<ClientRegistrationResponse> {
        #[derive(Serialize)]
        struct RegistrationRequest<'a> {
            client_name: &'a str,
            redirect_uris: Vec<&'a str>,
            grant_types: Vec<&'a str>,
            response_types: Vec<&'a str>,
            token_endpoint_auth_method: &'a str,
        }

        let payload = RegistrationRequest {
            client_name: &self.server_name,
            redirect_uris: vec![redirect_uri],
            grant_types: vec!["authorization_code", "refresh_token"],
            response_types: vec!["code"],
            token_endpoint_auth_method: "none",
        };

        let mut request = Request::builder()
            .method(http::Method::POST)
            .uri(endpoint)
            .header(http::header::CONTENT_TYPE, "application/json");

        for (key, value) in &self.default_headers {
            request = request.header(key.as_str(), value.as_str());
        }

        let body = serde_json::to_vec(&payload)?;
        let response = self
            .http_client
            .send(request.body(AsyncBody::from(body))?)
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "OAuth registration failed with status {}",
                response.status()
            );
        }

        parse_json_body(response).await
    }

    async fn exchange_code_for_token(
        &self,
        token_endpoint: &str,
        client_id: &str,
        client_secret: Option<&str>,
        redirect_uri: &str,
        code_verifier: &str,
        code: &str,
    ) -> Result<TokenEndpointResponse> {
        let body = form_body(&[
            ("grant_type", "authorization_code"),
            ("client_id", client_id),
            ("client_secret", client_secret.unwrap_or_default()),
            ("redirect_uri", redirect_uri),
            ("code_verifier", code_verifier),
            ("code", code),
        ]);

        let response = self.post_form(token_endpoint, body).await.context(
            "Failed to exchange authorization code for access token. Please try logging in again.",
        )?;

        parse_token_response(response).await
    }

    async fn post_form(&self, url: &str, body: String) -> Result<Response<AsyncBody>> {
        let mut request = Request::builder()
            .method(http::Method::POST)
            .uri(url)
            .header(
                http::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            );

        for (key, value) in &self.default_headers {
            request = request.header(key.as_str(), value.as_str());
        }

        self.http_client
            .send(request.body(AsyncBody::from(body.into_bytes()))?)
            .await
    }
}

fn discovery_paths(base_path: &str) -> Vec<String> {
    let trimmed = base_path.trim_start_matches('/').trim_end_matches('/');
    let canonical = "/.well-known/oauth-authorization-server".to_string();

    if trimmed.is_empty() {
        return vec![canonical];
    }

    let mut candidates = Vec::new();
    let mut push = |candidate: String| {
        if !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    };

    push(format!("{canonical}/{trimmed}"));
    push(format!("/{trimmed}/.well-known/oauth-authorization-server"));
    push(canonical);

    candidates
}

fn generate_pkce() -> (String, String) {
    let mut verifier_bytes = [0u8; 32];
    let mut rng = rng();
    rng.fill_bytes(&mut verifier_bytes);
    let verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);

    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());

    (verifier, challenge)
}

fn random_url_token() -> String {
    let mut bytes = [0u8; 16];
    let mut rng = rng();
    rng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn parse_www_authenticate_header(header: &str) -> (Option<Url>, Vec<String>) {
    let mut resource_metadata = None;
    let mut scopes: Vec<String> = Vec::new();

    for part in header.split(',') {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("scope=") {
            let trimmed = value.trim_matches('"');
            scopes.extend(trimmed.split_whitespace().map(|s| s.to_string()));
        } else if let Some(value) = part.strip_prefix("resource_metadata=") {
            let trimmed = value.trim_matches('"');
            if let Ok(url) = Url::parse(trimmed) {
                resource_metadata = Some(url);
            }
        }
    }

    (resource_metadata, scopes)
}

fn build_authorization_url(
    authorization_endpoint: &str,
    client_id: &str,
    redirect_uri: &str,
    code_challenge: &str,
    state: &str,
    scopes: &[String],
) -> Result<String> {
    let mut url = Url::parse(authorization_endpoint)?;
    let mut serializer = url.query_pairs_mut();
    serializer.append_pair("response_type", "code");
    serializer.append_pair("client_id", client_id);
    serializer.append_pair("redirect_uri", redirect_uri);
    serializer.append_pair("state", state);
    serializer.append_pair("code_challenge", code_challenge);
    serializer.append_pair("code_challenge_method", "S256");
    if !scopes.is_empty() {
        serializer.append_pair("scope", &scopes.join(" "));
    }
    drop(serializer);
    Ok(url.to_string())
}

struct OAuthCallback {
    code: String,
    state: String,
}

async fn spawn_callback_server(server: Arc<Server>, tx: channel::Sender<OAuthCallback>) {
    smol::spawn(async move {
        loop {
            let server_clone = server.clone();
            let request = smol::unblock(move || server_clone.recv()).await;

            let request = match request {
                Ok(req) => req,
                Err(_) => break,
            };

            let path = request.url().to_string();

            if let Some(callback) = parse_oauth_callback(&path) {
                log::debug!(
                    "Received valid OAuth callback: code={}, state={}",
                    callback.code.chars().take(8).collect::<String>(),
                    callback.state.chars().take(8).collect::<String>()
                );

                let _ = smol::unblock(move || {
                    request.respond(tiny_http::Response::from_string(
                        "Authentication complete. You may close this window.",
                    ))
                })
                .await;

                log::debug!("Sending callback through channel...");
                if tx.send(callback).await.is_err() {
                    log::error!("Failed to send callback through channel - receiver dropped!");
                } else {
                    log::debug!("Callback sent successfully");
                }
                break;
            } else {
                let _ = smol::unblock(move || {
                    request.respond(
                        tiny_http::Response::from_string("Invalid OAuth callback")
                            .with_status_code(400),
                    )
                })
                .await;
            }
        }
    })
    .detach();
}

fn parse_oauth_callback(path: &str) -> Option<OAuthCallback> {
    let (route, query) = path.split_once('?')?;
    if route != "/callback" {
        return None;
    }

    let mut code = None;
    let mut state = None;

    for pair in query.split('&') {
        let (key, value) = pair.split_once('=')?;
        let decoded = decode(value).ok()?.into_owned();
        match key {
            "code" => code = Some(decoded),
            "state" => state = Some(decoded),
            _ => {}
        }
    }

    Some(OAuthCallback {
        code: code?,
        state: state?,
    })
}

fn form_body<'a>(pairs: &[(&'a str, &'a str)]) -> String {
    url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(pairs.iter().map(|(k, v)| (*k, *v)))
        .finish()
}

fn current_time_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn tokens_file_path() -> Result<PathBuf> {
    let mut path = paths::config_dir().clone();
    path.push("mcp_oauth_tokens.json");
    Ok(path)
}

fn read_fallback_file() -> Result<Option<BTreeMap<String, StoredOAuthTokens>>> {
    let path = tokens_file_path()?;
    if !path.exists() {
        return Ok(None);
    }

    let contents = std::fs::read_to_string(&path)?;
    let store = serde_json::from_str(&contents)?;
    Ok(Some(store))
}

fn write_fallback_file(store: &BTreeMap<String, StoredOAuthTokens>) -> Result<()> {
    let path = tokens_file_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let serialized = serde_json::to_string(store)?;
    std::fs::write(path, serialized)?;
    Ok(())
}

fn compute_store_key(server_name: &str, server_url: &str) -> Result<String> {
    let mut payload = serde_json::Map::new();
    payload.insert(
        "type".to_string(),
        serde_json::Value::String("http".to_string()),
    );
    payload.insert(
        "url".to_string(),
        serde_json::Value::String(server_url.to_string()),
    );

    let serialized = serde_json::to_string(&payload)?;
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    let digest = hasher.finalize();
    let hex = format!("{digest:x}");
    let truncated = &hex[..16];
    Ok(format!("{server_name}|{truncated}"))
}

pub fn load_oauth_tokens(server_name: &str, url: &str) -> Result<Option<StoredOAuthTokens>> {
    let Some(store) = read_fallback_file()? else {
        return Ok(None);
    };
    let key = compute_store_key(server_name, url)?;
    Ok(store.get(&key).cloned())
}

pub fn save_oauth_tokens(server_name: &str, tokens: &StoredOAuthTokens) -> Result<()> {
    let mut store = read_fallback_file()?.unwrap_or_default();
    let key = compute_store_key(server_name, &tokens.url)?;
    store.insert(key, tokens.clone());
    write_fallback_file(&store)
}

pub fn delete_oauth_tokens(server_name: &str, url: &str) -> Result<bool> {
    let Some(mut store) = read_fallback_file()? else {
        return Ok(false);
    };
    let key = compute_store_key(server_name, url)?;
    let removed = store.remove(&key).is_some();
    if removed {
        write_fallback_file(&store)?;
    }
    Ok(removed)
}

fn keychain_url_for_mcp_oauth(server_name: &str, server_url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(server_url.as_bytes());
    let digest = hasher.finalize();
    let hex = format!("{digest:x}");
    let truncated = &hex[..16];
    format!("mcp-oauth://{server_name}/{truncated}")
}

pub async fn load_oauth_tokens_from_keychain(
    server_name: &str,
    url: &str,
    credentials_provider: &dyn CredentialsProvider,
    cx: &AsyncApp,
) -> Result<Option<StoredOAuthTokens>> {
    let keychain_url = keychain_url_for_mcp_oauth(server_name, url);

    match credentials_provider
        .read_credentials(&keychain_url, cx)
        .await
    {
        Ok(Some((_username, password_bytes))) => {
            match serde_json::from_slice::<StoredOAuthTokens>(&password_bytes) {
                Ok(tokens) => {
                    log::debug!("Loaded OAuth tokens from keychain for '{}'", server_name);
                    Ok(Some(tokens))
                }
                Err(e) => {
                    log::warn!(
                        "Failed to deserialize OAuth tokens from keychain for '{}': {}",
                        server_name,
                        e
                    );
                    Ok(load_oauth_tokens(server_name, url)?.inspect(|_| {
                        log::debug!("Falling back to file-based tokens for '{}'", server_name);
                    }))
                }
            }
        }
        Ok(None) => {
            log::debug!(
                "No OAuth tokens in keychain for '{}', checking file fallback",
                server_name
            );
            Ok(load_oauth_tokens(server_name, url)?)
        }
        Err(e) => {
            log::warn!(
                "Failed to read OAuth tokens from keychain for '{}': {}",
                server_name,
                e
            );
            Ok(load_oauth_tokens(server_name, url)?)
        }
    }
}

pub async fn save_oauth_tokens_to_keychain(
    server_name: &str,
    tokens: &StoredOAuthTokens,
    credentials_provider: &dyn CredentialsProvider,
    cx: &AsyncApp,
) -> Result<()> {
    let keychain_url = keychain_url_for_mcp_oauth(server_name, &tokens.url);
    let serialized = serde_json::to_vec(tokens)?;

    credentials_provider
        .write_credentials(&keychain_url, server_name, &serialized, cx)
        .await
        .context("Failed to save OAuth tokens to keychain")?;

    log::debug!("Saved OAuth tokens to keychain for '{}'", server_name);

    if let Err(e) = delete_oauth_tokens(server_name, &tokens.url) {
        log::debug!(
            "Could not remove legacy file-based tokens for '{}': {}",
            server_name,
            e
        );
    }

    Ok(())
}

pub async fn delete_oauth_tokens_from_keychain(
    server_name: &str,
    url: &str,
    credentials_provider: &dyn CredentialsProvider,
    cx: &AsyncApp,
) -> Result<bool> {
    let keychain_url = keychain_url_for_mcp_oauth(server_name, url);

    match credentials_provider
        .delete_credentials(&keychain_url, cx)
        .await
    {
        Ok(()) => {
            log::debug!("Deleted OAuth tokens from keychain for '{}'", server_name);
            let _ = delete_oauth_tokens(server_name, url);
            Ok(true)
        }
        Err(e) => {
            log::warn!(
                "Failed to delete OAuth tokens from keychain for '{}': {}",
                server_name,
                e
            );
            delete_oauth_tokens(server_name, url)
        }
    }
}

async fn parse_json_body<T: serde::de::DeserializeOwned>(
    mut response: Response<AsyncBody>,
) -> Result<T> {
    let mut body = Vec::new();
    futures::AsyncReadExt::read_to_end(response.body_mut(), &mut body).await?;
    Ok(serde_json::from_slice(&body)?)
}

async fn parse_token_response(response: Response<AsyncBody>) -> Result<TokenEndpointResponse> {
    if !response.status().is_success() {
        anyhow::bail!("token endpoint returned {}", response.status());
    }

    let token: TokenEndpointResponse = parse_json_body(response).await?;
    if let Some(token_type) = &token.token_type {
        anyhow::ensure!(
            token_type.to_lowercase() == TOKEN_TYPE_BEARER,
            "unsupported token type {token_type}"
        );
    }

    Ok(token)
}
