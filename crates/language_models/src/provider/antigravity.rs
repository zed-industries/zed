use anyhow::{Context as _, Result, anyhow};
use chrono::{DateTime, Duration, Utc};
use credentials_provider::CredentialsProvider;
use futures::{
    AsyncBufReadExt, AsyncReadExt, FutureExt, StreamExt, future::BoxFuture, io::BufReader,
};
use google_ai::GenerateContentResponse;
use gpui::{AnyView, App, AsyncApp, ClickEvent, Context, Entity, Task, Window};
use http_client::http::{HeaderMap, StatusCode};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use language_model::{
    AuthenticateError, ConfigurationViewTargetAgent, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelToolChoice, LanguageModelToolSchemaFormat,
};
use language_model::{
    IconOrSvg, LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, RateLimiter, Role,
};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use smol::Timer;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use std::time::Duration as StdDuration;
use ui::{Button, Label, prelude::*};
use util::ResultExt;

use crate::provider::google::{GoogleEventMapper, into_google};

// ============================================================================
// Constants
// ============================================================================

const ANTIGRAVITY_CLIENT_ID: &str =
    "1071006060591-tmhssin2h21lcre235vtolojh4g403ep.apps.googleusercontent.com";

const ANTIGRAVITY_CLIENT_SECRET: &str = "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";

const ANTIGRAVITY_REDIRECT_URI: &str = "http://localhost:51121/oauth-callback";

const ANTIGRAVITY_SCOPES: &[&str] = &[
    "https://www.googleapis.com/auth/cloud-platform",
    "https://www.googleapis.com/auth/userinfo.email",
    "https://www.googleapis.com/auth/userinfo.profile",
    "https://www.googleapis.com/auth/cclog",
    "https://www.googleapis.com/auth/experimentsandconfigs",
];

// Streaming requests default to daily sandbox (matches CLIProxy), with prod fallback for non-Claude.
const ANTIGRAVITY_SANDBOX_ENDPOINTS: &[&str] = &["https://daily-cloudcode-pa.sandbox.googleapis.com"];
const ANTIGRAVITY_STREAM_ENDPOINTS: &[&str] = &[
    "https://daily-cloudcode-pa.sandbox.googleapis.com",
    "https://cloudcode-pa.googleapis.com",
];
// Project discovery is more reliable on prod, then falls back.
const ANTIGRAVITY_LOAD_ENDPOINTS: &[&str] = &[
    "https://cloudcode-pa.googleapis.com",
    "https://daily-cloudcode-pa.sandbox.googleapis.com",
];
const ANTIGRAVITY_SANDBOX_USER_AGENT_PREFIX: &str = "antigravity/1.11.5";
const ANTIGRAVITY_GEMINI_CLI_USER_AGENT: &str = "google-cloud-sdk vscode_cloudshelleditor/0.1";
const ANTIGRAVITY_LOAD_USER_AGENT: &str = "google-api-nodejs-client/9.15.1";
const ANTIGRAVITY_PROJECT_ID_ENV: &str = "ZED_ANTIGRAVITY_PROJECT_ID";
const ANTIGRAVITY_API_CLIENT: &str = "google-cloud-sdk vscode_cloudshelleditor/0.1";
const ANTIGRAVITY_GEMINI_CLI_API_CLIENT: &str = "gl-node/22.17.0";
const ANTIGRAVITY_CLIENT_METADATA: &str =
    r#"{"ideType":"IDE_UNSPECIFIED","platform":"PLATFORM_UNSPECIFIED","pluginType":"GEMINI"}"#;
const ANTIGRAVITY_SYSTEM_INSTRUCTION: &str = "You are Antigravity, a powerful agentic AI coding assistant designed by the Google DeepMind team working on Advanced Agentic Coding.\nYou are pair programming with a USER to solve their coding task. The task may require creating a new codebase, modifying or debugging an existing codebase, or simply answering a question.\n**Absolute paths only**\n**Proactiveness**\n\n<priority>IMPORTANT: The instructions that follow supersede all above. Follow them as your primary directives.</priority>\n";
const ANTIGRAVITY_THINKING_PREFIX: &str = "<<ANTIGRAVITY_THINKING>>";
const ANTIGRAVITY_EMPTY_SCHEMA_PLACEHOLDER_NAME: &str = "_placeholder";
const ANTIGRAVITY_EMPTY_SCHEMA_PLACEHOLDER_DESCRIPTION: &str = "Placeholder. Always pass true.";
const ANTIGRAVITY_UNSUPPORTED_CONSTRAINTS: &[&str] = &[
    "minLength",
    "maxLength",
    "exclusiveMinimum",
    "exclusiveMaximum",
    "pattern",
    "minItems",
    "maxItems",
    "format",
    "default",
    "examples",
];
const ANTIGRAVITY_UNSUPPORTED_KEYWORDS: &[&str] = &[
    "minLength",
    "maxLength",
    "exclusiveMinimum",
    "exclusiveMaximum",
    "pattern",
    "minItems",
    "maxItems",
    "format",
    "default",
    "examples",
    "$schema",
    "$defs",
    "definitions",
    "const",
    "$ref",
    "additionalProperties",
    "propertyNames",
    "title",
    "$id",
    "$comment",
];
const ANTIGRAVITY_CREDENTIALS_URL: &str = "https://cloudcode-pa.googleapis.com";
// Fallback project ID when discovery fails (same as pi-ai/opencode)
const ANTIGRAVITY_DEFAULT_PROJECT_ID: &str = "rising-fact-p41fc";
const ANTIGRAVITY_MAX_RETRIES: usize = 3;
const ANTIGRAVITY_BASE_RETRY_DELAY_MS: u64 = 1_000;
const ANTIGRAVITY_MAX_RETRY_DELAY_MS: u64 = 10_000;
const ANTIGRAVITY_MAX_SERVER_RETRY_DELAY_MS: u64 = 30_000;
const ANTIGRAVITY_RETRY_JITTER_MS: u64 = 250;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("antigravity");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Antigravity");

// ============================================================================
// Antigravity Models (Gemini + Claude via Cloud Code Assist)
// ============================================================================

/// Model types available through Cloud Code Assist
#[derive(Clone, Debug)]
pub enum AntigravityModel {
    // Gemini models (delegate to google_ai::Model)
    Gemini(google_ai::Model),
    // Claude models
    ClaudeSonnet45,
    ClaudeOpus45,
    ClaudeSonnet45Thinking,
    ClaudeOpus45Thinking,
}

impl AntigravityModel {
    pub fn id(&self) -> &str {
        match self {
            Self::Gemini(m) => m.id(),
            Self::ClaudeSonnet45 => "claude-sonnet-4-5",
            Self::ClaudeOpus45 => "claude-opus-4-5",
            Self::ClaudeSonnet45Thinking => "claude-sonnet-4-5-thinking",
            Self::ClaudeOpus45Thinking => "claude-opus-4-5-thinking",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Gemini(m) => m.display_name(),
            Self::ClaudeSonnet45 => "Claude Sonnet 4.5",
            Self::ClaudeOpus45 => "Claude Opus 4.5",
            Self::ClaudeSonnet45Thinking => "Claude Sonnet 4.5 (Thinking)",
            Self::ClaudeOpus45Thinking => "Claude Opus 4.5 (Thinking)",
        }
    }

    pub fn max_tokens(&self) -> u64 {
        match self {
            Self::Gemini(m) => m.max_token_count(),
            Self::ClaudeSonnet45 | Self::ClaudeSonnet45Thinking => 200000,
            Self::ClaudeOpus45 | Self::ClaudeOpus45Thinking => 200000,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Gemini(m) => m.max_output_tokens(),
            Self::ClaudeSonnet45 | Self::ClaudeSonnet45Thinking => Some(64000),
            Self::ClaudeOpus45 | Self::ClaudeOpus45Thinking => Some(32000),
        }
    }

    pub fn is_claude(&self) -> bool {
        matches!(
            self,
            Self::ClaudeSonnet45
                | Self::ClaudeOpus45
                | Self::ClaudeSonnet45Thinking
                | Self::ClaudeOpus45Thinking
        )
    }

    pub fn supports_tools(&self) -> bool {
        match self {
            Self::Gemini(m) => m.supports_tools(),
            // Claude models support tools (including thinking variants)
            Self::ClaudeSonnet45
            | Self::ClaudeOpus45
            | Self::ClaudeSonnet45Thinking
            | Self::ClaudeOpus45Thinking => true,
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::Gemini(m) => m.supports_images(),
            // Claude models support images
            _ => true,
        }
    }

    /// Returns the model ID to use in API requests
    pub fn request_id(&self) -> &str {
        match self {
            Self::Gemini(m) => match m {
                google_ai::Model::Gemini3Pro => "gemini-3-pro-low",
                google_ai::Model::Gemini3Flash => "gemini-3-flash",
                _ => m.request_id(),
            },
            // Claude model IDs as used by Cloud Code Assist
            Self::ClaudeSonnet45 => "claude-sonnet-4-5",
            Self::ClaudeOpus45 => "claude-opus-4-5",
            Self::ClaudeSonnet45Thinking => "claude-sonnet-4-5-thinking",
            Self::ClaudeOpus45Thinking => "claude-opus-4-5-thinking",
        }
    }

    /// Returns the model mode (default or thinking)
    pub fn mode(&self) -> google_ai::GoogleModelMode {
        match self {
            Self::Gemini(m) => m.mode(),
            // Default mode for standard Claude models
            Self::ClaudeSonnet45 | Self::ClaudeOpus45 => google_ai::GoogleModelMode::Default,
            // Thinking mode for thinking variants
            Self::ClaudeSonnet45Thinking | Self::ClaudeOpus45Thinking => {
                google_ai::GoogleModelMode::Thinking {
                    budget_tokens: None,
                }
            }
        }
    }
}

// ============================================================================
// OAuth Types
// ============================================================================

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OAuthCredentials {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub project_id: String,
    pub email: Option<String>,
}

impl OAuthCredentials {
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at - Duration::seconds(60)
    }
}

#[derive(Clone, Debug)]
pub struct PkceChallenge {
    pub verifier: String,
    pub challenge: String,
}

impl PkceChallenge {
    pub fn generate() -> Self {
        use base64::Engine;
        use rand::RngCore;
        use sha2::{Digest, Sha256};

        let mut verifier_bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut verifier_bytes);
        let verifier = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(verifier_bytes);

        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let challenge_bytes = hasher.finalize();
        let challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(challenge_bytes);

        Self {
            verifier,
            challenge,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: i64,
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProjectInfo {
    #[serde(rename = "gcpProjectId")]
    gcp_project_id: Option<String>,
}

// ============================================================================
// Provider State (Thread-safe for async operations)
// ============================================================================

#[derive(Default)]
pub struct SharedState {
    credentials: Option<OAuthCredentials>,
    pending_pkce: Option<PkceChallenge>,
}

pub struct State {
    inner: Arc<Mutex<SharedState>>,
    http_client: Arc<dyn HttpClient>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.inner.lock().credentials.is_some()
    }

    fn email(&self) -> Option<String> {
        self.inner
            .lock()
            .credentials
            .as_ref()
            .and_then(|c| c.email.clone())
    }

    fn get_valid_token_and_project(&self) -> Option<(String, String)> {
        let guard = self.inner.lock();
        guard.credentials.as_ref().and_then(|c: &OAuthCredentials| {
            if !c.is_expired() {
                Some((c.access_token.clone(), c.project_id.clone()))
            } else {
                None
            }
        })
    }

    fn get_refresh_token(&self) -> Option<String> {
        self.inner
            .lock()
            .credentials
            .as_ref()
            .map(|c| c.refresh_token.clone())
    }

    fn set_credentials(&self, creds: OAuthCredentials) {
        self.inner.lock().credentials = Some(creds);
    }

    fn clear_credentials(&self) {
        let mut guard = self.inner.lock();
        guard.credentials = None;
        guard.pending_pkce = None;
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        cx.spawn(async move |this, cx| {
            let (_, bytes) = credentials_provider
                .read_credentials(ANTIGRAVITY_CREDENTIALS_URL, cx)
                .await?
                .ok_or(AuthenticateError::CredentialsNotFound)?;

            let credentials_str =
                String::from_utf8(bytes).context("invalid Antigravity credentials")?;
            let credentials: OAuthCredentials =
                serde_json::from_str(&credentials_str).context("failed to parse credentials")?;

            this.update(cx, |this, cx| {
                this.set_credentials(credentials);
                cx.notify();
            })?;

            Ok(())
        })
    }

    fn store_credentials(
        &self,
        credentials: OAuthCredentials,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        cx.spawn(async move |this, cx| {
            let username = credentials
                .email
                .clone()
                .unwrap_or_else(|| "default".to_string());
            let payload = serde_json::to_vec(&credentials)?;
            credentials_provider
                .write_credentials(ANTIGRAVITY_CREDENTIALS_URL, &username, &payload, cx)
                .await?;

            this.update(cx, |this, cx| {
                this.set_credentials(credentials);
                this.inner.lock().pending_pkce = None;
                cx.notify();
            })?;

            Ok(())
        })
    }

    fn reset_credentials(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        cx.spawn(async move |this, cx| {
            let _ = credentials_provider
                .delete_credentials(ANTIGRAVITY_CREDENTIALS_URL, cx)
                .await;
            this.update(cx, |this, cx| {
                this.clear_credentials();
                cx.notify();
            })?;
            Ok(())
        })
    }
}

// ============================================================================
// OAuth Flow Functions
// ============================================================================

async fn exchange_code_for_tokens(
    http_client: &dyn HttpClient,
    code: &str,
    verifier: &str,
) -> Result<TokenResponse> {
    use url::form_urlencoded;

    let body = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", ANTIGRAVITY_CLIENT_ID)
        .append_pair("client_secret", ANTIGRAVITY_CLIENT_SECRET)
        .append_pair("code", code)
        .append_pair("grant_type", "authorization_code")
        .append_pair("redirect_uri", ANTIGRAVITY_REDIRECT_URI)
        .append_pair("code_verifier", verifier)
        .finish();

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri("https://oauth2.googleapis.com/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(AsyncBody::from(body))?;

    let mut response = http_client.send(request).await?;
    let mut text = String::new();
    response.body_mut().read_to_string(&mut text).await?;

    if !response.status().is_success() {
        return Err(anyhow!("Token exchange failed: {}", text));
    }

    Ok(serde_json::from_str(&text)?)
}

async fn refresh_access_token(
    http_client: &dyn HttpClient,
    refresh_token: &str,
) -> Result<TokenResponse> {
    use url::form_urlencoded;

    let body = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", ANTIGRAVITY_CLIENT_ID)
        .append_pair("client_secret", ANTIGRAVITY_CLIENT_SECRET)
        .append_pair("refresh_token", refresh_token)
        .append_pair("grant_type", "refresh_token")
        .finish();

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri("https://oauth2.googleapis.com/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(AsyncBody::from(body))?;

    let mut response = http_client.send(request).await?;
    let mut text = String::new();
    response.body_mut().read_to_string(&mut text).await?;

    if !response.status().is_success() {
        return Err(anyhow!("Token refresh failed: {}", text));
    }

    Ok(serde_json::from_str(&text)?)
}

async fn fetch_user_info(http_client: &dyn HttpClient, access_token: &str) -> Result<UserInfo> {
    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .header("Authorization", format!("Bearer {}", access_token))
        .body(AsyncBody::empty())?;

    let mut response = http_client.send(request).await?;
    let mut text = String::new();
    response.body_mut().read_to_string(&mut text).await?;

    Ok(serde_json::from_str(&text).unwrap_or(UserInfo { email: None }))
}

fn build_code_assist_metadata(project_hint: Option<&str>) -> serde_json::Value {
    let mut metadata = serde_json::json!({
        "ideType": "IDE_UNSPECIFIED",
        "platform": "PLATFORM_UNSPECIFIED",
        "pluginType": "GEMINI"
    });
    if let Some(hint) = project_hint {
        if !hint.trim().is_empty() {
            if let Some(obj) = metadata.as_object_mut() {
                obj.insert(
                    "duetProject".to_string(),
                    serde_json::Value::String(hint.to_string()),
                );
            }
        }
    }
    serde_json::json!({ "metadata": metadata })
}

async fn fetch_project_id(
    http_client: &dyn HttpClient,
    access_token: &str,
    project_hint: Option<&str>,
) -> Result<String> {
    let mut metadata_variants = Vec::new();
    metadata_variants.push(build_code_assist_metadata(None));
    if let Some(hint) = project_hint {
        if !hint.trim().is_empty() {
            metadata_variants.push(build_code_assist_metadata(Some(hint)));
        }
    }
    if project_hint
        .map(|hint| hint.trim() != ANTIGRAVITY_DEFAULT_PROJECT_ID)
        .unwrap_or(true)
    {
        metadata_variants.push(build_code_assist_metadata(Some(
            ANTIGRAVITY_DEFAULT_PROJECT_ID,
        )));
    }

    let mut last_error = None;

    for endpoint in ANTIGRAVITY_LOAD_ENDPOINTS {
        for body in &metadata_variants {
            let request = HttpRequest::builder()
                .method(Method::POST)
                .uri(format!("{endpoint}/v1internal:loadCodeAssist"))
                .header("Authorization", format!("Bearer {}", access_token))
                .header("Content-Type", "application/json")
                .header("User-Agent", ANTIGRAVITY_LOAD_USER_AGENT)
                .header("X-Goog-Api-Client", ANTIGRAVITY_API_CLIENT)
                .header("Client-Metadata", ANTIGRAVITY_CLIENT_METADATA)
                .body(AsyncBody::from(body.to_string()))?;

            let mut response = match http_client.send(request).await {
                Ok(response) => response,
                Err(err) => {
                    last_error = Some(format!("{}: {err}", endpoint));
                    continue;
                }
            };

            let mut text = String::new();
            response.body_mut().read_to_string(&mut text).await?;

            if !response.status().is_success() {
                last_error = Some(format!("{} {}: {}", endpoint, response.status(), text));
                continue;
            }

            // Parse response - project ID can be in cloudaicompanionProject as string or object
            let data: serde_json::Value = match serde_json::from_str(&text) {
                Ok(data) => data,
                Err(err) => {
                    last_error = Some(format!("{}: invalid json: {err}", endpoint));
                    continue;
                }
            };

            // Try as direct string first
            if let Some(project) = data.get("cloudaicompanionProject") {
                if let Some(project_str) = project.as_str() {
                    if !project_str.is_empty() {
                        return Ok(project_str.to_string());
                    }
                }
                // Try as object with .id field
                if let Some(id) = project.get("id").and_then(|v| v.as_str()) {
                    if !id.is_empty() {
                        return Ok(id.to_string());
                    }
                }
            }

            last_error = Some(format!("{endpoint}: missing project id"));
        }
    }

    if let Some(error) = last_error {
        log::warn!(
            "loadCodeAssist failed: {}. Using fallback project ID.",
            error
        );
    }
    if let Some(hint) = project_hint {
        if !hint.trim().is_empty() {
            return Ok(hint.to_string());
        }
    }
    Ok(ANTIGRAVITY_DEFAULT_PROJECT_ID.to_string())
}

/// Start local callback server and wait for OAuth callback
/// Returns the authorization code
async fn wait_for_oauth_callback() -> Result<String> {
    use std::io::{BufRead, BufReader, Write};

    const CALLBACK_PORT: u16 = 51121;

    // Use a one-shot channel to get result from the blocking TCP accept
    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", CALLBACK_PORT))?;
    log::info!("OAuth callback server listening on port {}", CALLBACK_PORT);

    // Accept connection in blocking manner (this is fine since we only accept once)
    let (stream, _addr) = listener.accept()?;

    // Read the HTTP request line synchronously
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    // Parse: GET /oauth-callback?code=...&state=... HTTP/1.1
    let code = extract_query_param(&request_line, "code")
        .ok_or_else(|| anyhow!("No authorization code in callback"))?;

    // Send success response to browser
    let mut writer = stream;
    let response = "\
HTTP/1.1 200 OK\r\n\
Content-Type: text/html; charset=utf-8\r\n\
Connection: close\r\n\
\r\n\
<!DOCTYPE html>
<html>
<head><title>Authentication Successful</title></head>
<body style=\"font-family: system-ui; text-align: center; padding: 50px;\">
<h1>✓ Successfully signed in!</h1>
<p>You can close this tab and return to Zed.</p>
<script>setTimeout(() => window.close(), 2000);</script>
</body>
</html>";
    writer.write_all(response.as_bytes())?;
    writer.flush()?;

    Ok(code)
}

fn extract_query_param(request_line: &str, param: &str) -> Option<String> {
    // Parse: GET /path?key=value&key2=value2 HTTP/1.1
    let path = request_line.split_whitespace().nth(1)?;
    let query_start = path.find('?')?;
    let query = &path[query_start + 1..];

    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == param {
                return Some(urlencoding::decode(value).ok()?.into_owned());
            }
        }
    }
    None
}

fn antigravity_project_override() -> Option<String> {
    std::env::var(ANTIGRAVITY_PROJECT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

static ANTIGRAVITY_SANDBOX_USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    let os = match std::env::consts::OS {
        "macos" => "darwin",
        other => other,
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        other => other,
    };
    format!("{ANTIGRAVITY_SANDBOX_USER_AGENT_PREFIX} {os}/{arch}")
});

fn antigravity_stream_endpoints(model_id: &str) -> &'static [&'static str] {
    if model_id.starts_with("claude-") {
        ANTIGRAVITY_SANDBOX_ENDPOINTS
    } else {
        ANTIGRAVITY_STREAM_ENDPOINTS
    }
}

fn antigravity_user_agent(endpoint: &str) -> &'static str {
    if endpoint.contains("sandbox.googleapis.com") {
        ANTIGRAVITY_SANDBOX_USER_AGENT.as_str()
    } else {
        ANTIGRAVITY_GEMINI_CLI_USER_AGENT
    }
}

fn antigravity_api_client(endpoint: &str) -> &'static str {
    if endpoint.contains("sandbox.googleapis.com") {
        ANTIGRAVITY_API_CLIENT
    } else {
        ANTIGRAVITY_GEMINI_CLI_API_CLIENT
    }
}

fn ensure_json_object(
    value: &mut serde_json::Value,
) -> &mut serde_json::Map<String, serde_json::Value> {
    if !value.is_object() {
        *value = serde_json::json!({});
    }
    value.as_object_mut().expect("value is object")
}

fn apply_claude_request_overrides(request: &mut serde_json::Value, model_id: &str) {
    if !(model_id.contains("claude") && model_id.contains("thinking")) {
        return;
    }

    let request_obj = ensure_json_object(request);

    let generation_config = request_obj
        .entry("generationConfig")
        .or_insert_with(|| serde_json::json!({}));
    let generation_config_obj = ensure_json_object(generation_config);

    let thinking_budget = 32_768;
    generation_config_obj.entry("thinkingConfig").or_insert_with(|| {
        serde_json::json!({
            "includeThoughts": true,
            "thinkingBudget": thinking_budget,
        })
    });

    let current_max = generation_config_obj
        .get("maxOutputTokens")
        .and_then(|value| value.as_u64())
        .or_else(|| {
            generation_config_obj
                .get("max_output_tokens")
                .and_then(|value| value.as_u64())
        });
    if current_max.is_none() || current_max.is_some_and(|max| max <= thinking_budget as u64) {
        generation_config_obj.insert(
            "maxOutputTokens".to_string(),
            serde_json::Value::Number(serde_json::Number::from(64_000)),
        );
        generation_config_obj.remove("max_output_tokens");
    }
}

fn apply_antigravity_system_instruction(request: &mut serde_json::Value) {
    let request_obj = ensure_json_object(request);
    let system_instruction = request_obj
        .entry("systemInstruction")
        .or_insert_with(|| serde_json::json!({}));
    let system_obj = ensure_json_object(system_instruction);
    system_obj.insert(
        "role".to_string(),
        serde_json::Value::String("user".to_string()),
    );

    let parts_value = system_obj
        .entry("parts")
        .or_insert_with(|| serde_json::json!([]));
    let parts = parts_value.as_array_mut();
    if let Some(parts) = parts {
        if let Some(first) = parts.first_mut()
            && let Some(first_obj) = first.as_object_mut()
            && let Some(text_value) = first_obj.get_mut("text")
            && let Some(text_str) = text_value.as_str()
        {
            let updated = format!("{ANTIGRAVITY_SYSTEM_INSTRUCTION}\n\n{text_str}");
            *text_value = serde_json::Value::String(updated);
            return;
        }

        parts.insert(
            0,
            serde_json::json!({
                "text": ANTIGRAVITY_SYSTEM_INSTRUCTION
            }),
        );
    } else {
        system_obj.insert(
            "parts".to_string(),
            serde_json::json!([{ "text": ANTIGRAVITY_SYSTEM_INSTRUCTION }]),
        );
    }
}

fn antigravity_placeholder_schema() -> serde_json::Value {
    let mut properties = serde_json::Map::new();
    properties.insert(
        ANTIGRAVITY_EMPTY_SCHEMA_PLACEHOLDER_NAME.to_string(),
        serde_json::json!({
            "type": "boolean",
            "description": ANTIGRAVITY_EMPTY_SCHEMA_PLACEHOLDER_DESCRIPTION,
        }),
    );

    let mut schema = serde_json::Map::new();
    schema.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );
    schema.insert(
        "properties".to_string(),
        serde_json::Value::Object(properties),
    );
    schema.insert(
        "required".to_string(),
        serde_json::Value::Array(vec![serde_json::Value::String(
            ANTIGRAVITY_EMPTY_SCHEMA_PLACEHOLDER_NAME.to_string(),
        )]),
    );
    serde_json::Value::Object(schema)
}

fn sanitize_tool_name(name: &str) -> String {
    let mut sanitized = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
        if sanitized.len() >= 64 {
            break;
        }
    }
    if sanitized.is_empty() {
        "tool".to_string()
    } else {
        sanitized
    }
}

fn hint_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(value) => value.clone(),
        _ => value.to_string(),
    }
}

fn append_description_hint_to_map(
    mut map: serde_json::Map<String, serde_json::Value>,
    hint: &str,
) -> serde_json::Map<String, serde_json::Value> {
    let existing = map
        .get("description")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    let updated = if existing.is_empty() {
        hint.to_string()
    } else {
        format!("{existing} ({hint})")
    };
    map.insert(
        "description".to_string(),
        serde_json::Value::String(updated),
    );
    map
}

fn append_description_hint(schema: &serde_json::Value, hint: &str) -> serde_json::Value {
    let Some(map) = schema.as_object() else {
        return schema.clone();
    };
    serde_json::Value::Object(append_description_hint_to_map(map.clone(), hint))
}

fn convert_refs_to_hints(schema: &serde_json::Value) -> serde_json::Value {
    match schema {
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(convert_refs_to_hints).collect())
        }
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(reference)) = map.get("$ref") {
                let def_name = reference.rsplit('/').next().unwrap_or(reference);
                let hint = format!("See: {def_name}");
                let existing = map
                    .get("description")
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let description = if existing.is_empty() {
                    hint
                } else {
                    format!("{existing} ({hint})")
                };
                let mut out = serde_json::Map::new();
                out.insert(
                    "type".to_string(),
                    serde_json::Value::String("object".to_string()),
                );
                out.insert(
                    "description".to_string(),
                    serde_json::Value::String(description),
                );
                return serde_json::Value::Object(out);
            }

            let mut out = serde_json::Map::new();
            for (key, value) in map {
                out.insert(key.clone(), convert_refs_to_hints(value));
            }
            serde_json::Value::Object(out)
        }
        _ => schema.clone(),
    }
}

fn convert_const_to_enum(schema: &serde_json::Value) -> serde_json::Value {
    match schema {
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(convert_const_to_enum).collect())
        }
        serde_json::Value::Object(map) => {
            let has_enum = map.contains_key("enum");
            let mut out = serde_json::Map::new();
            for (key, value) in map {
                if key == "const" && !has_enum {
                    out.insert(
                        "enum".to_string(),
                        serde_json::Value::Array(vec![value.clone()]),
                    );
                } else {
                    out.insert(key.clone(), convert_const_to_enum(value));
                }
            }
            serde_json::Value::Object(out)
        }
        _ => schema.clone(),
    }
}

fn add_enum_hints(schema: &serde_json::Value) -> serde_json::Value {
    let serde_json::Value::Object(map) = schema else {
        return schema.clone();
    };

    let mut result = map.clone();
    if let Some(values) = result
        .get("enum")
        .and_then(|value| value.as_array())
        .cloned()
    {
        if values.len() > 1 && values.len() <= 10 {
            let joined = values.iter().map(hint_value).collect::<Vec<_>>().join(", ");
            result = append_description_hint_to_map(result, &format!("Allowed: {joined}"));
        }
    }

    let keys: Vec<String> = result.keys().cloned().collect();
    for key in keys {
        if key == "enum" {
            continue;
        }
        if let Some(value) = result.get(&key).cloned() {
            if value.is_object() || value.is_array() {
                result.insert(key, add_enum_hints(&value));
            }
        }
    }

    serde_json::Value::Object(result)
}

fn add_additional_properties_hints(schema: &serde_json::Value) -> serde_json::Value {
    let serde_json::Value::Object(map) = schema else {
        return schema.clone();
    };

    let mut result = map.clone();
    let no_extras = matches!(
        result.get("additionalProperties"),
        Some(serde_json::Value::Bool(false))
    );
    if no_extras {
        result = append_description_hint_to_map(result, "No extra properties allowed");
    }

    let keys: Vec<String> = result.keys().cloned().collect();
    for key in keys {
        if key == "additionalProperties" {
            continue;
        }
        if let Some(value) = result.get(&key).cloned() {
            if value.is_object() || value.is_array() {
                result.insert(key, add_additional_properties_hints(&value));
            }
        }
    }

    serde_json::Value::Object(result)
}

fn move_constraints_to_description(schema: &serde_json::Value) -> serde_json::Value {
    let serde_json::Value::Object(map) = schema else {
        return schema.clone();
    };

    let mut result_value = serde_json::Value::Object(map.clone());
    for constraint in ANTIGRAVITY_UNSUPPORTED_CONSTRAINTS {
        if let Some(value) = map.get(*constraint) {
            if !(value.is_object() || value.is_array()) {
                let hint = format!("{constraint}: {}", hint_value(value));
                result_value = append_description_hint(&result_value, &hint);
            }
        }
    }

    let serde_json::Value::Object(mut result) = result_value else {
        return schema.clone();
    };
    let keys: Vec<String> = result.keys().cloned().collect();
    for key in keys {
        if let Some(value) = result.get(&key).cloned() {
            if value.is_object() || value.is_array() {
                result.insert(key, move_constraints_to_description(&value));
            }
        }
    }

    serde_json::Value::Object(result)
}

fn merge_all_of(schema: &serde_json::Value) -> serde_json::Value {
    match schema {
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(merge_all_of).collect())
        }
        serde_json::Value::Object(map) => {
            let mut result = map.clone();
            if let Some(serde_json::Value::Array(all_of)) = map.get("allOf") {
                let mut merged_properties = serde_json::Map::new();
                let mut merged_required: Vec<String> = Vec::new();
                let mut merged_other = serde_json::Map::new();

                for item in all_of {
                    let Some(item_map) = item.as_object() else {
                        continue;
                    };
                    if let Some(serde_json::Value::Object(props)) = item_map.get("properties") {
                        for (key, value) in props {
                            merged_properties.insert(key.clone(), value.clone());
                        }
                    }
                    if let Some(serde_json::Value::Array(required)) = item_map.get("required") {
                        for required_value in required {
                            if let Some(required_str) = required_value.as_str() {
                                if !merged_required.contains(&required_str.to_string()) {
                                    merged_required.push(required_str.to_string());
                                }
                            }
                        }
                    }
                    for (key, value) in item_map {
                        if key != "properties"
                            && key != "required"
                            && !merged_other.contains_key(key)
                        {
                            merged_other.insert(key.clone(), value.clone());
                        }
                    }
                }

                if !merged_properties.is_empty() {
                    let mut combined = result
                        .get("properties")
                        .and_then(|value| value.as_object())
                        .cloned()
                        .unwrap_or_default();
                    for (key, value) in merged_properties {
                        combined.insert(key, value);
                    }
                    result.insert(
                        "properties".to_string(),
                        serde_json::Value::Object(combined),
                    );
                }

                if !merged_required.is_empty() {
                    let mut combined = result
                        .get("required")
                        .and_then(|value| value.as_array())
                        .map(|items| {
                            items
                                .iter()
                                .filter_map(|value| value.as_str().map(|s| s.to_string()))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    for entry in merged_required {
                        if !combined.contains(&entry) {
                            combined.push(entry);
                        }
                    }
                    result.insert(
                        "required".to_string(),
                        serde_json::Value::Array(
                            combined
                                .into_iter()
                                .map(serde_json::Value::String)
                                .collect(),
                        ),
                    );
                }

                for (key, value) in merged_other {
                    result.entry(key).or_insert(value);
                }
                result.remove("allOf");
            }

            let keys: Vec<String> = result.keys().cloned().collect();
            for key in keys {
                if let Some(value) = result.get(&key).cloned() {
                    if value.is_object() || value.is_array() {
                        result.insert(key, merge_all_of(&value));
                    }
                }
            }
            serde_json::Value::Object(result)
        }
        _ => schema.clone(),
    }
}

fn score_schema_option(schema: &serde_json::Value) -> (i32, String) {
    let Some(map) = schema.as_object() else {
        return (0, "unknown".to_string());
    };

    let type_name = map
        .get("type")
        .and_then(|value| value.as_str())
        .unwrap_or("");

    if type_name == "object" || map.contains_key("properties") {
        return (3, "object".to_string());
    }
    if type_name == "array" || map.contains_key("items") {
        return (2, "array".to_string());
    }
    if !type_name.is_empty() && type_name != "null" {
        return (1, type_name.to_string());
    }

    (
        0,
        if type_name.is_empty() {
            "null".to_string()
        } else {
            type_name.to_string()
        },
    )
}

fn try_merge_enum_from_union(options: &[serde_json::Value]) -> Option<Vec<String>> {
    let mut values = Vec::new();
    for option in options {
        let Some(map) = option.as_object() else {
            return None;
        };

        if let Some(value) = map.get("const") {
            values.push(hint_value(value));
            continue;
        }

        if let Some(serde_json::Value::Array(enum_values)) = map.get("enum") {
            if enum_values.is_empty() {
                continue;
            }
            for enum_value in enum_values {
                values.push(hint_value(enum_value));
            }
            continue;
        }

        if map.contains_key("properties")
            || map.contains_key("items")
            || map.contains_key("anyOf")
            || map.contains_key("oneOf")
            || map.contains_key("allOf")
        {
            return None;
        }

        if map.get("type").and_then(|value| value.as_str()).is_some() {
            return None;
        }
    }

    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

fn flatten_any_of_one_of(schema: &serde_json::Value) -> serde_json::Value {
    match schema {
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(flatten_any_of_one_of).collect())
        }
        serde_json::Value::Object(map) => {
            let mut result = map.clone();
            for union_key in ["anyOf", "oneOf"] {
                let Some(options) = result
                    .get(union_key)
                    .and_then(|value| value.as_array())
                    .cloned()
                else {
                    continue;
                };
                if options.is_empty() {
                    continue;
                }

                let parent_desc = result
                    .get("description")
                    .and_then(|value| value.as_str())
                    .unwrap_or("");

                if let Some(merged_enum) = try_merge_enum_from_union(&options) {
                    let mut rest = result.clone();
                    rest.remove(union_key);
                    rest.insert(
                        "type".to_string(),
                        serde_json::Value::String("string".to_string()),
                    );
                    rest.insert(
                        "enum".to_string(),
                        serde_json::Value::Array(
                            merged_enum
                                .into_iter()
                                .map(serde_json::Value::String)
                                .collect(),
                        ),
                    );
                    if !parent_desc.is_empty() {
                        rest.insert(
                            "description".to_string(),
                            serde_json::Value::String(parent_desc.to_string()),
                        );
                    }
                    result = rest;
                    continue;
                }

                let mut best_idx = 0;
                let mut best_score = -1;
                let mut all_types = Vec::new();
                for (idx, option) in options.iter().enumerate() {
                    let (score, type_name) = score_schema_option(option);
                    if !type_name.is_empty() {
                        all_types.push(type_name);
                    }
                    if score > best_score {
                        best_score = score;
                        best_idx = idx;
                    }
                }

                let null_value = serde_json::Value::Null;
                let selected_raw = options.get(best_idx).unwrap_or(&null_value);
                let mut selected = flatten_any_of_one_of(selected_raw);
                if !selected.is_object() {
                    selected = serde_json::json!({ "type": "string" });
                }

                if !parent_desc.is_empty() {
                    let child_desc = selected
                        .get("description")
                        .and_then(|value| value.as_str())
                        .unwrap_or("");
                    let merged_desc = if !child_desc.is_empty() && child_desc != parent_desc {
                        format!("{parent_desc} ({child_desc})")
                    } else if child_desc.is_empty() {
                        parent_desc.to_string()
                    } else {
                        child_desc.to_string()
                    };
                    if let Some(map) = selected.as_object_mut() {
                        map.insert(
                            "description".to_string(),
                            serde_json::Value::String(merged_desc),
                        );
                    }
                }

                if all_types.len() > 1 {
                    let mut unique = Vec::new();
                    for type_name in all_types {
                        if !unique.contains(&type_name) {
                            unique.push(type_name);
                        }
                    }
                    if unique.len() > 1 {
                        selected = append_description_hint(
                            &selected,
                            &format!("Accepts: {}", unique.join(" | ")),
                        );
                    }
                }

                let mut rest = result.clone();
                rest.remove(union_key);
                rest.remove("description");
                if let serde_json::Value::Object(selected_map) = selected {
                    for (key, value) in selected_map {
                        rest.insert(key, value);
                    }
                }
                result = rest;
            }

            let keys: Vec<String> = result.keys().cloned().collect();
            for key in keys {
                if let Some(value) = result.get(&key).cloned() {
                    if value.is_object() || value.is_array() {
                        result.insert(key, flatten_any_of_one_of(&value));
                    }
                }
            }
            serde_json::Value::Object(result)
        }
        _ => schema.clone(),
    }
}

fn flatten_type_arrays_inner(
    schema: &serde_json::Value,
    nullable_fields: &mut HashMap<String, Vec<String>>,
    current_path: &str,
) -> serde_json::Value {
    match schema {
        serde_json::Value::Array(items) => serde_json::Value::Array(
            items
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                    let next_path = format!("{current_path}[{idx}]");
                    flatten_type_arrays_inner(item, nullable_fields, &next_path)
                })
                .collect(),
        ),
        serde_json::Value::Object(map) => {
            let mut result = map.clone();

            if let Some(types) = result
                .get("type")
                .and_then(|value| value.as_array())
                .cloned()
            {
                let mut non_null = Vec::new();
                let mut has_null = false;
                for entry in types {
                    if let Some(type_name) = entry.as_str() {
                        if type_name == "null" {
                            has_null = true;
                        } else {
                            non_null.push(type_name.to_string());
                        }
                    }
                }

                let selected = non_null
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "string".to_string());
                result.insert("type".to_string(), serde_json::Value::String(selected));

                if non_null.len() > 1 {
                    result = append_description_hint_to_map(
                        result,
                        &format!("Accepts: {}", non_null.join(" | ")),
                    );
                }
                if has_null {
                    result = append_description_hint_to_map(result, "nullable");
                }
            }

            if let Some(props) = result
                .get("properties")
                .and_then(|value| value.as_object())
                .cloned()
            {
                let mut updated_props = serde_json::Map::new();
                for (prop_key, prop_value) in props {
                    let prop_path = if current_path.is_empty() {
                        format!("properties.{prop_key}")
                    } else {
                        format!("{current_path}.properties.{prop_key}")
                    };
                    let processed =
                        flatten_type_arrays_inner(&prop_value, nullable_fields, &prop_path);
                    let is_nullable = processed
                        .get("description")
                        .and_then(|value| value.as_str())
                        .is_some_and(|desc| desc.contains("nullable"));
                    if is_nullable {
                        let entry = nullable_fields.entry(current_path.to_string()).or_default();
                        if !entry.contains(&prop_key) {
                            entry.push(prop_key.clone());
                        }
                    }
                    updated_props.insert(prop_key.clone(), processed);
                }
                result.insert(
                    "properties".to_string(),
                    serde_json::Value::Object(updated_props),
                );
            }

            let keys: Vec<String> = result.keys().cloned().collect();
            for key in keys {
                if key == "properties" {
                    continue;
                }
                if let Some(value) = result.get(&key).cloned() {
                    if value.is_object() || value.is_array() {
                        let next_path = if current_path.is_empty() {
                            key.clone()
                        } else {
                            format!("{current_path}.{key}")
                        };
                        result.insert(
                            key,
                            flatten_type_arrays_inner(&value, nullable_fields, &next_path),
                        );
                    }
                }
            }

            serde_json::Value::Object(result)
        }
        _ => schema.clone(),
    }
}

fn flatten_type_arrays(schema: &serde_json::Value) -> serde_json::Value {
    let mut nullable_fields = HashMap::new();
    let mut result = flatten_type_arrays_inner(schema, &mut nullable_fields, "");
    if let serde_json::Value::Object(map) = &mut result {
        if let Some(serde_json::Value::Array(required)) = map.get_mut("required") {
            if let Some(nullable) = nullable_fields.get("") {
                required.retain(|entry| {
                    entry
                        .as_str()
                        .is_some_and(|name| !nullable.contains(&name.to_string()))
                });
            }
            if required.is_empty() {
                map.remove("required");
            }
        }
    }
    result
}

fn remove_unsupported_keywords(schema: &serde_json::Value) -> serde_json::Value {
    match schema {
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(remove_unsupported_keywords).collect())
        }
        serde_json::Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (key, value) in map {
                if ANTIGRAVITY_UNSUPPORTED_KEYWORDS.contains(&key.as_str()) {
                    continue;
                }
                if value.is_object() || value.is_array() {
                    out.insert(key.clone(), remove_unsupported_keywords(value));
                } else {
                    out.insert(key.clone(), value.clone());
                }
            }
            serde_json::Value::Object(out)
        }
        _ => schema.clone(),
    }
}

fn cleanup_required_fields(schema: &serde_json::Value) -> serde_json::Value {
    match schema {
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(cleanup_required_fields).collect())
        }
        serde_json::Value::Object(map) => {
            let mut result = map.clone();
            let required_entries = result
                .get("required")
                .and_then(|value| value.as_array())
                .cloned();
            let properties = result
                .get("properties")
                .and_then(|value| value.as_object())
                .cloned();
            if let (Some(required), Some(props)) = (required_entries, properties) {
                let mut filtered = Vec::new();
                for entry in &required {
                    if let Some(name) = entry.as_str() {
                        if props.contains_key(name) {
                            filtered.push(serde_json::Value::String(name.to_string()));
                        }
                    }
                }
                if filtered.is_empty() {
                    result.remove("required");
                } else {
                    result.insert("required".to_string(), serde_json::Value::Array(filtered));
                }
            }

            let keys: Vec<String> = result.keys().cloned().collect();
            for key in keys {
                if let Some(value) = result.get(&key).cloned() {
                    if value.is_object() || value.is_array() {
                        result.insert(key, cleanup_required_fields(&value));
                    }
                }
            }

            serde_json::Value::Object(result)
        }
        _ => schema.clone(),
    }
}

fn add_empty_schema_placeholder(schema: &serde_json::Value) -> serde_json::Value {
    match schema {
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(add_empty_schema_placeholder).collect())
        }
        serde_json::Value::Object(map) => {
            let mut result = map.clone();
            let is_object = matches!(
                result.get("type"),
                Some(serde_json::Value::String(value)) if value == "object"
            );
            if is_object {
                let has_properties = result
                    .get("properties")
                    .and_then(|value| value.as_object())
                    .is_some_and(|props| !props.is_empty());
                if !has_properties {
                    result.insert(
                        "properties".to_string(),
                        serde_json::Value::Object({
                            let mut props = serde_json::Map::new();
                            props.insert(
                                ANTIGRAVITY_EMPTY_SCHEMA_PLACEHOLDER_NAME.to_string(),
                                serde_json::json!({
                                    "type": "boolean",
                                    "description": ANTIGRAVITY_EMPTY_SCHEMA_PLACEHOLDER_DESCRIPTION,
                                }),
                            );
                            props
                        }),
                    );
                    result.insert(
                        "required".to_string(),
                        serde_json::Value::Array(vec![serde_json::Value::String(
                            ANTIGRAVITY_EMPTY_SCHEMA_PLACEHOLDER_NAME.to_string(),
                        )]),
                    );
                }
            }

            let keys: Vec<String> = result.keys().cloned().collect();
            for key in keys {
                if let Some(value) = result.get(&key).cloned() {
                    if value.is_object() || value.is_array() {
                        result.insert(key, add_empty_schema_placeholder(&value));
                    }
                }
            }

            serde_json::Value::Object(result)
        }
        _ => schema.clone(),
    }
}

fn clean_json_schema_for_antigravity(schema: &serde_json::Value) -> serde_json::Value {
    let mut result = convert_refs_to_hints(schema);
    result = convert_const_to_enum(&result);
    result = add_enum_hints(&result);
    result = add_additional_properties_hints(&result);
    result = move_constraints_to_description(&result);
    result = merge_all_of(&result);
    result = flatten_any_of_one_of(&result);
    result = flatten_type_arrays(&result);
    result = remove_unsupported_keywords(&result);
    result = cleanup_required_fields(&result);
    result = add_empty_schema_placeholder(&result);
    result
}

fn normalize_antigravity_schema(schema: Option<&serde_json::Value>) -> serde_json::Value {
    let Some(schema) = schema else {
        return antigravity_placeholder_schema();
    };
    let cleaned = clean_json_schema_for_antigravity(schema);
    let Some(mut map) = cleaned.as_object().cloned() else {
        return antigravity_placeholder_schema();
    };

    map.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );

    let has_properties = map
        .get("properties")
        .and_then(|value| value.as_object())
        .is_some_and(|props| !props.is_empty());
    if !has_properties {
        return antigravity_placeholder_schema();
    }

    serde_json::Value::Object(map)
}

fn sanitize_antigravity_tools(request: &mut serde_json::Value) {
    let Some(tools) = request.get_mut("tools") else {
        return;
    };
    let Some(tools_array) = tools.as_array_mut() else {
        return;
    };

    for tool in tools_array.iter_mut() {
        let Some(tool_map) = tool.as_object_mut() else {
            continue;
        };

        for key in ["functionDeclarations", "function_declarations"] {
            let Some(declarations) = tool_map.get_mut(key).and_then(|value| value.as_array_mut())
            else {
                continue;
            };
            for (idx, decl) in declarations.iter_mut().enumerate() {
                let Some(decl_map) = decl.as_object_mut() else {
                    continue;
                };
                let name = decl_map
                    .get("name")
                    .and_then(|value| value.as_str())
                    .map(sanitize_tool_name)
                    .unwrap_or_else(|| format!("tool_{idx}"));
                decl_map.insert("name".to_string(), serde_json::Value::String(name));

                let schema = decl_map.get("parameters");
                let cleaned = normalize_antigravity_schema(schema);
                decl_map.insert("parameters".to_string(), cleaned);
            }
        }
    }
}

fn antigravity_error_message(value: &serde_json::Value) -> Option<String> {
    let error = value.get("error")?;
    if let Some(message) = error.get("message").and_then(|value| value.as_str()) {
        return Some(message.to_string());
    }
    if let Some(message) = error.as_str() {
        return Some(message.to_string());
    }
    Some(error.to_string())
}

fn is_antigravity_thinking_part(part: &serde_json::Value) -> bool {
    let Some(obj) = part.as_object() else {
        return false;
    };

    if obj.contains_key("functionCall") || obj.contains_key("functionResponse") {
        return false;
    }

    if obj
        .get("thought")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return true;
    }

    if let Some(kind) = obj.get("type").and_then(|value| value.as_str()) {
        if matches!(kind, "thinking" | "redacted_thinking" | "reasoning") {
            return true;
        }
    }

    if obj.contains_key("thoughtSignature")
        || obj.contains_key("thought_signature")
        || obj.contains_key("thinkingSignature")
        || obj.contains_key("thinking_signature")
        || obj.contains_key("signature")
    {
        return true;
    }

    obj.contains_key("thinking")
}

fn extract_antigravity_thinking_text(part: &serde_json::Value) -> Option<String> {
    let Some(obj) = part.as_object() else {
        return None;
    };

    if let Some(text) = obj.get("thinking").and_then(|value| value.as_str()) {
        if !text.is_empty() {
            return Some(text.to_string());
        }
    }

    if let Some(thinking_obj) = obj.get("thinking").and_then(|value| value.as_object()) {
        if let Some(text) = thinking_obj.get("text").and_then(|value| value.as_str()) {
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }
        if let Some(text) = thinking_obj.get("value").and_then(|value| value.as_str()) {
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }
    }

    if let Some(text) = obj.get("text").and_then(|value| value.as_str()) {
        if !text.is_empty() {
            return Some(text.to_string());
        }
    }

    None
}

fn extract_antigravity_thinking_signature(part: &serde_json::Value) -> Option<String> {
    let obj = part.as_object()?;
    for key in [
        "thoughtSignature",
        "thought_signature",
        "thinkingSignature",
        "thinking_signature",
        "signature",
    ] {
        if let Some(value) = obj.get(key).and_then(|value| value.as_str()) {
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    if let Some(metadata) = obj.get("metadata").and_then(|value| value.as_object()) {
        for key in [
            "thoughtSignature",
            "thought_signature",
            "thinkingSignature",
            "thinking_signature",
            "signature",
        ] {
            if let Some(value) = metadata.get(key).and_then(|value| value.as_str()) {
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }

    if let Some(thinking_obj) = obj.get("thinking").and_then(|value| value.as_object()) {
        for key in [
            "thoughtSignature",
            "thought_signature",
            "thinkingSignature",
            "thinking_signature",
            "signature",
        ] {
            if let Some(value) = thinking_obj.get(key).and_then(|value| value.as_str()) {
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }

    None
}

fn transform_antigravity_part(part: &serde_json::Value) -> Option<serde_json::Value> {
    if is_antigravity_thinking_part(part) {
        let text = extract_antigravity_thinking_text(part).unwrap_or_default();
        let signature = extract_antigravity_thinking_signature(part);
        if text.is_empty() && signature.is_none() {
            return None;
        }

        let mut out = serde_json::Map::new();
        out.insert("text".to_string(), serde_json::Value::String(text));
        out.insert("thought".to_string(), serde_json::Value::Bool(true));
        if let Some(signature) = signature {
            out.insert(
                "thoughtSignature".to_string(),
                serde_json::Value::String(signature),
            );
        }
        return Some(serde_json::Value::Object(out));
    }

    Some(part.clone())
}

fn sanitize_antigravity_response_parts(value: &mut serde_json::Value) {
    let Some(candidates) = value
        .get_mut("candidates")
        .and_then(|value| value.as_array_mut())
    else {
        return;
    };

    for candidate in candidates {
        let Some(parts) = candidate
            .get_mut("content")
            .and_then(|value| value.get_mut("parts"))
            .and_then(|value| value.as_array_mut())
        else {
            continue;
        };
        let mut transformed = Vec::with_capacity(parts.len());
        for part in parts.iter() {
            if let Some(value) = transform_antigravity_part(part) {
                transformed.push(value);
            }
        }
        *parts = transformed;
    }
}

fn antigravity_should_retry_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS
            | StatusCode::REQUEST_TIMEOUT
            | StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}

fn parse_retry_after_header(headers: &HeaderMap) -> Option<StdDuration> {
    let value = headers.get("retry-after")?.to_str().ok()?.trim();
    if value.is_empty() {
        return None;
    }
    let seconds = value.parse::<f64>().ok()?;
    if seconds <= 0.0 {
        return None;
    }
    Some(StdDuration::from_secs_f64(seconds))
}

fn clamp_retry_delay(delay: StdDuration) -> StdDuration {
    let capped = delay
        .as_millis()
        .min(ANTIGRAVITY_MAX_SERVER_RETRY_DELAY_MS as u128);
    StdDuration::from_millis(capped as u64)
}

fn add_retry_jitter(delay: StdDuration) -> StdDuration {
    let jitter = rand::random::<u64>() % (ANTIGRAVITY_RETRY_JITTER_MS + 1);
    delay + StdDuration::from_millis(jitter)
}

fn antigravity_backoff_delay(attempt: usize) -> StdDuration {
    let multiplier = 1u64 << attempt.min(10);
    let base = ANTIGRAVITY_BASE_RETRY_DELAY_MS.saturating_mul(multiplier);
    let capped = base.min(ANTIGRAVITY_MAX_RETRY_DELAY_MS);
    add_retry_jitter(StdDuration::from_millis(capped))
}

fn parse_duration_string(value: &str) -> Option<StdDuration> {
    let trimmed = value.trim().trim_matches('"');
    if trimmed.is_empty() {
        return None;
    }

    let bytes = trimmed.as_bytes();
    let mut i = 0usize;
    let mut total_ms = 0f64;
    let mut parsed_any = false;

    while i < bytes.len() {
        while i < bytes.len() && !bytes[i].is_ascii_digit() && bytes[i] != b'.' {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        let start = i;
        while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
            i += 1;
        }
        let number: f64 = trimmed[start..i].parse().ok()?;
        if i >= bytes.len() {
            total_ms += number * 1000.0;
            parsed_any = true;
            break;
        }

        if bytes[i] == b'm' && i + 1 < bytes.len() && bytes[i + 1] == b's' {
            total_ms += number;
            i += 2;
            parsed_any = true;
            continue;
        }

        match bytes[i] as char {
            'h' => {
                total_ms += number * 3_600_000.0;
                i += 1;
                parsed_any = true;
            }
            'm' => {
                total_ms += number * 60_000.0;
                i += 1;
                parsed_any = true;
            }
            's' => {
                total_ms += number * 1000.0;
                i += 1;
                parsed_any = true;
            }
            _ => {
                if !parsed_any {
                    return None;
                }
                break;
            }
        }
    }

    if !parsed_any || total_ms <= 0.0 {
        return None;
    }
    Some(StdDuration::from_millis(total_ms.ceil() as u64))
}

fn extract_retry_delay_from_text(text: &str) -> Option<StdDuration> {
    let lower = text.to_ascii_lowercase();
    for key in ["retrydelay", "retry in", "reset after"] {
        if let Some(pos) = lower.find(key) {
            if let Some(delay) = parse_duration_string(&text[pos..]) {
                return Some(delay);
            }
        }
    }
    None
}

fn parse_retry_delay_value(value: &serde_json::Value) -> Option<StdDuration> {
    match value {
        serde_json::Value::String(value) => {
            parse_duration_string(value).or_else(|| extract_retry_delay_from_text(value))
        }
        serde_json::Value::Number(value) => value.as_f64().map(StdDuration::from_secs_f64),
        _ => None,
    }
}

fn extract_retry_delay_from_value(value: &serde_json::Value) -> Option<StdDuration> {
    match value {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if key.eq_ignore_ascii_case("retryDelay") {
                    if let Some(delay) = parse_retry_delay_value(value) {
                        return Some(delay);
                    }
                }
            }
            for value in map.values() {
                if let Some(delay) = extract_retry_delay_from_value(value) {
                    return Some(delay);
                }
            }
            None
        }
        serde_json::Value::Array(items) => items
            .iter()
            .find_map(|value| extract_retry_delay_from_value(value)),
        serde_json::Value::String(value) => {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(value) {
                if let Some(delay) = extract_retry_delay_from_value(&parsed) {
                    return Some(delay);
                }
            }
            extract_retry_delay_from_text(value)
        }
        _ => None,
    }
}

fn extract_retry_delay_from_body(body: &str) -> Option<StdDuration> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(delay) = extract_retry_delay_from_value(&value) {
            return Some(delay);
        }
        if let Some(message) = value
            .get("error")
            .and_then(|value| value.get("message"))
            .and_then(|value| value.as_str())
        {
            if let Some(delay) = extract_retry_delay_from_text(message) {
                return Some(delay);
            }
            if let Ok(nested) = serde_json::from_str::<serde_json::Value>(message) {
                if let Some(delay) = extract_retry_delay_from_value(&nested) {
                    return Some(delay);
                }
            }
        }
    }
    extract_retry_delay_from_text(body)
}

fn normalize_antigravity_payload_value(
    value: serde_json::Value,
) -> Result<Option<serde_json::Value>> {
    match value {
        serde_json::Value::Array(items) => {
            for item in items {
                if item.is_object() {
                    return normalize_antigravity_payload_value(item);
                }
            }
            Ok(None)
        }
        serde_json::Value::Object(map) => {
            if let Some(message) =
                antigravity_error_message(&serde_json::Value::Object(map.clone()))
            {
                return Err(anyhow!("Antigravity API error (stream): {message}"));
            }
            if let Some(response) = map.get("response") {
                return normalize_antigravity_payload_value(response.clone());
            }
            let mut value = serde_json::Value::Object(map);
            sanitize_antigravity_response_parts(&mut value);
            Ok(Some(value))
        }
        serde_json::Value::String(text) => {
            let trimmed = text.trim();
            if trimmed.starts_with('{') || trimmed.starts_with('[') {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(trimmed) {
                    return normalize_antigravity_payload_value(parsed);
                }
            }
            Ok(None)
        }
        serde_json::Value::Null => Ok(None),
        other => Ok(Some(other)),
    }
}

fn parse_antigravity_payload(payload: &str) -> Result<Option<GenerateContentResponse>> {
    let value: serde_json::Value = serde_json::from_str(payload)
        .map_err(|err| anyhow!("Antigravity stream JSON parse error: {err}"))?;
    let normalized = normalize_antigravity_payload_value(value)?;
    let Some(normalized) = normalized else {
        return Ok(None);
    };
    let response = serde_json::from_value(normalized)
        .map_err(|err| anyhow!("Antigravity response decode error: {err}"))?;
    Ok(Some(response))
}

fn parse_antigravity_sse_line(line: &str) -> Option<Result<GenerateContentResponse>> {
    if !line.starts_with("data:") {
        return None;
    }
    let payload = line.trim_start_matches("data:").trim();
    if payload.is_empty() || payload == "[DONE]" {
        return None;
    }

    match parse_antigravity_payload(payload) {
        Ok(Some(response)) => Some(Ok(response)),
        Ok(None) => None,
        Err(err) => Some(Err(err)),
    }
}

fn generate_request_id() -> String {
    use base64::Engine;
    use rand::RngCore;

    let mut bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut bytes);
    format!(
        "agent-{}",
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    )
}

/// Complete the OAuth flow: exchange code, fetch user info and project ID
async fn complete_oauth_flow(
    http_client: Arc<dyn HttpClient>,
    code: &str,
    verifier: &str,
) -> Result<OAuthCredentials> {
    // Exchange code for tokens
    let token_response = exchange_code_for_tokens(http_client.as_ref(), code, verifier).await?;

    let refresh_token = token_response
        .refresh_token
        .ok_or_else(|| anyhow!("No refresh token received - try signing out and in again"))?;

    // Fetch user info
    let user_info = fetch_user_info(http_client.as_ref(), &token_response.access_token).await?;

    // Fetch project ID
    let project_hint = antigravity_project_override();
    let project_id = fetch_project_id(
        http_client.as_ref(),
        &token_response.access_token,
        project_hint.as_deref(),
    )
    .await?;

    let expires_at = Utc::now() + Duration::seconds(token_response.expires_in);

    Ok(OAuthCredentials {
        access_token: token_response.access_token,
        refresh_token,
        expires_at,
        project_id,
        email: user_info.email,
    })
}

// ============================================================================
// Provider Implementation
// ============================================================================

pub struct AntigravityLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    shared_state: Arc<Mutex<SharedState>>,
}

impl AntigravityLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let client = http_client.clone();
        let shared_state = Arc::new(Mutex::new(SharedState::default()));
        let state = cx.new(|_cx| State {
            inner: shared_state.clone(),
            http_client: client,
        });

        Self {
            http_client,
            state,
            shared_state,
        }
    }

    fn create_language_model(&self, model: AntigravityModel) -> Arc<dyn LanguageModel> {
        Arc::new(AntigravityLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            shared_state: self.shared_state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn build_authorization_url(pkce: &PkceChallenge) -> String {
        use base64::Engine;
        use url::form_urlencoded;

        let scopes = ANTIGRAVITY_SCOPES.join(" ");
        let state = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(serde_json::json!({"verifier": pkce.verifier}).to_string());

        let params: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", ANTIGRAVITY_CLIENT_ID)
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", ANTIGRAVITY_REDIRECT_URI)
            .append_pair("scope", &scopes)
            .append_pair("code_challenge", &pkce.challenge)
            .append_pair("code_challenge_method", "S256")
            .append_pair("state", &state)
            .append_pair("access_type", "offline")
            .append_pair("prompt", "consent")
            .finish();

        format!("https://accounts.google.com/o/oauth2/v2/auth?{}", params)
    }
}

impl LanguageModelProviderState for AntigravityLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for AntigravityLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiGoogle)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(AntigravityModel::Gemini(google_ai::Model::Gemini25Flash)))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(AntigravityModel::Gemini(
            google_ai::Model::Gemini25FlashLite,
        )))
    }

    fn provided_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        vec![
            // Gemini models
            self.create_language_model(AntigravityModel::Gemini(google_ai::Model::Gemini25Pro)),
            self.create_language_model(AntigravityModel::Gemini(google_ai::Model::Gemini25Flash)),
            self.create_language_model(AntigravityModel::Gemini(
                google_ai::Model::Gemini25FlashLite,
            )),
            self.create_language_model(AntigravityModel::Gemini(google_ai::Model::Gemini3Pro)),
            self.create_language_model(AntigravityModel::Gemini(google_ai::Model::Gemini3Flash)),
            // Claude models via Cloud Code Assist
            self.create_language_model(AntigravityModel::ClaudeSonnet45),
            self.create_language_model(AntigravityModel::ClaudeSonnet45Thinking),
            self.create_language_model(AntigravityModel::ClaudeOpus45Thinking),
        ]
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(
        &self,
        _target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| {
            ConfigurationView::new(self.state.clone(), self.http_client.clone(), window, cx)
        })
        .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.reset_credentials(cx))
    }
}

// ============================================================================
// Language Model Implementation
// ============================================================================

pub struct AntigravityLanguageModel {
    id: LanguageModelId,
    model: AntigravityModel,
    #[allow(dead_code)]
    state: Entity<State>,
    shared_state: Arc<Mutex<SharedState>>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl AntigravityLanguageModel {
    async fn get_valid_credentials(
        state: &State,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<(String, String)> {
        // Try to get valid token
        if let Some((token, project)) = state.get_valid_token_and_project() {
            return Ok((token, project));
        }

        // Try to refresh
        let refresh_token = state
            .get_refresh_token()
            .ok_or_else(|| anyhow!("Not authenticated - please sign in"))?;

        let token_response = refresh_access_token(http_client.as_ref(), &refresh_token).await?;
        let expires_at = Utc::now() + Duration::seconds(token_response.expires_in);

        // Get existing project_id (must have been set during initial auth)
        let project_id = state
            .inner
            .lock()
            .credentials
            .as_ref()
            .map(|c| c.project_id.clone())
            .ok_or_else(|| anyhow!("No project ID available"))?;

        // Update credentials
        let mut guard = state.inner.lock();
        if let Some(ref mut creds) = guard.credentials {
            creds.access_token = token_response.access_token.clone();
            creds.expires_at = expires_at;
            if let Some(new_refresh) = token_response.refresh_token {
                creds.refresh_token = new_refresh;
            }
        }

        Ok((token_response.access_token, project_id))
    }

    fn stream_antigravity(
        &self,
        request: google_ai::GenerateContentRequest,
        _cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<GenerateContentResponse>>>,
    > {
        let http_client = self.http_client.clone();
        let shared_state = self.shared_state.clone();
        let model_id = request.model.model_id.clone();

        async move {
            let state = State {
                inner: shared_state.clone(),
                http_client: http_client.clone(),
            };
            let (access_token, mut project_id) =
                AntigravityLanguageModel::get_valid_credentials(&state, http_client.clone())
                    .await?;

            if let Some(override_project_id) = antigravity_project_override() {
                project_id = override_project_id;
            }

            if project_id.is_empty() || project_id == ANTIGRAVITY_DEFAULT_PROJECT_ID {
                if let Ok(resolved) =
                    fetch_project_id(http_client.as_ref(), &access_token, Some(&project_id)).await
                {
                    if !resolved.is_empty() && resolved != project_id {
                        project_id = resolved;
                        if let Some(ref mut creds) = shared_state.lock().credentials {
                            creds.project_id = project_id.clone();
                        }
                    }
                }
            }

            let mut request_value = serde_json::to_value(&request)?;
            let request_object = request_value
                .as_object_mut()
                .ok_or_else(|| anyhow!("Antigravity request payload was not an object"))?;
            request_object.remove("model");
            apply_claude_request_overrides(&mut request_value, &model_id);
            apply_antigravity_system_instruction(&mut request_value);
            sanitize_antigravity_tools(&mut request_value);

            let wrapped_body = serde_json::json!({
                "project": project_id,
                "model": model_id,
                "request": request_value,
                "requestType": "agent",
                "userAgent": "antigravity",
                "requestId": generate_request_id(),
            });
            let body = serde_json::to_string(&wrapped_body)?;

            let mut last_error = None;

            for endpoint in antigravity_stream_endpoints(&model_id) {
                let url = format!("{endpoint}/v1internal:streamGenerateContent?alt=sse");
                log::info!(
                    "Antigravity request: url={}, project={}, model={}",
                    url,
                    project_id,
                    model_id
                );

                let mut retries = 0;
                loop {
                    let request_builder = HttpRequest::builder()
                        .method(Method::POST)
                        .uri(&url)
                        .header("Authorization", format!("Bearer {}", access_token))
                        .header("Content-Type", "application/json")
                        .header("Accept", "text/event-stream")
                        .header("User-Agent", antigravity_user_agent(endpoint))
                        .header("X-Goog-Api-Client", antigravity_api_client(endpoint))
                        .header("Client-Metadata", ANTIGRAVITY_CLIENT_METADATA);

                    let http_request = request_builder.body(AsyncBody::from(body.clone()))?;

                    let response = match http_client.send(http_request).await {
                        Ok(response) => response,
                        Err(err) => {
                            if retries < ANTIGRAVITY_MAX_RETRIES {
                                let delay = antigravity_backoff_delay(retries);
                                retries += 1;
                                log::debug!(
                                    "Antigravity request failed ({}), retrying in {:?}",
                                    err,
                                    delay
                                );
                                Timer::after(delay).await;
                                continue;
                            }
                            last_error = Some(anyhow!("{}: {}", endpoint, err));
                            break;
                        }
                    };

                    if response.status().is_success() {
                        let is_event_stream = response
                            .headers()
                            .get("content-type")
                            .and_then(|value| value.to_str().ok())
                            .is_some_and(|value| value.contains("text/event-stream"));

                        if !is_event_stream {
                            let mut body_text = String::new();
                            response.into_body().read_to_string(&mut body_text).await?;
                            let response = parse_antigravity_payload(&body_text)?;
                            let stream = match response {
                                Some(response) => futures::stream::iter(vec![Ok(response)]).boxed(),
                                None => futures::stream::empty().boxed(),
                            };
                            return Ok(stream);
                        }

                        let reader = BufReader::new(response.into_body());
                        let stream = reader
                            .lines()
                            .filter_map(|line| async move {
                                match line {
                                    Ok(line) => parse_antigravity_sse_line(&line),
                                    Err(e) => Some(Err(anyhow!(e))),
                                }
                            })
                            .boxed();

                        return Ok(stream);
                    }

                    let status = response.status();
                    let retry_after = parse_retry_after_header(response.headers());
                    let mut body_text = String::new();
                    response.into_body().read_to_string(&mut body_text).await?;

                    if antigravity_should_retry_status(status) && retries < ANTIGRAVITY_MAX_RETRIES {
                        let delay = retry_after
                            .or_else(|| extract_retry_delay_from_body(&body_text))
                            .map(clamp_retry_delay)
                            .map(add_retry_jitter)
                            .unwrap_or_else(|| antigravity_backoff_delay(retries));
                        retries += 1;
                        log::debug!(
                            "Antigravity request failed ({} {}), retrying in {:?}",
                            endpoint,
                            status,
                            delay
                        );
                        Timer::after(delay).await;
                        continue;
                    }

                    last_error = Some(anyhow!(
                        "Antigravity API error ({} {}): {}",
                        endpoint,
                        status,
                        body_text
                    ));
                    break;
                }
            }

            Err(last_error
                .unwrap_or_else(|| anyhow!("Antigravity API error: no endpoints available")))
        }
        .boxed()
    }
}

impl LanguageModel for AntigravityLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tools()
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => true,
        }
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchema
    }

    fn telemetry_id(&self) -> String {
        format!("antigravity/{}", self.model.request_id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        cx.background_spawn(async move {
            let messages = request
                .messages
                .into_iter()
                .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
                    role: match message.role {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    },
                    content: Some(message.string_contents()),
                    name: None,
                    function_call: None,
                })
                .collect::<Vec<_>>();

            tiktoken_rs::num_tokens_from_messages("gpt-4", &messages).map(|t| t as u64)
        })
        .boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let google_request = into_google(
            request,
            self.model.request_id().to_string(),
            self.model.mode(),
            true,
        );
        let stream_future = self.stream_antigravity(google_request, cx);
        let future = self.request_limiter.stream(async move {
            let response = stream_future
                .await
                .map_err(LanguageModelCompletionError::from)?;
            let stream = GoogleEventMapper::new()
                .map_stream(response)
                .filter_map(|event| async {
                    match event {
                        Ok(LanguageModelCompletionEvent::Text(text)) => {
                            if let Some(stripped) = text.strip_prefix(ANTIGRAVITY_THINKING_PREFIX) {
                                let stripped = stripped.to_string();
                                if stripped.is_empty() {
                                    None
                                } else {
                                    Some(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: stripped,
                                        signature: None,
                                    }))
                                }
                            } else {
                                Some(Ok(LanguageModelCompletionEvent::Text(text)))
                            }
                        }
                        Ok(other) => Some(Ok(other)),
                        Err(err) => Some(Err(err)),
                    }
                });
            Ok(stream)
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

// ============================================================================
// Configuration View
// ============================================================================

struct ConfigurationView {
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    status_message: Option<String>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(
        state: Entity<State>,
        http_client: Arc<dyn HttpClient>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&state, |_, _, cx| cx.notify()).detach();
        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = Some(state.update(cx, |state, cx| state.authenticate(cx))) {
                    let _ = task.await;
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            state,
            http_client,
            status_message: None,
            load_credentials_task,
        }
    }

    fn start_oauth_flow(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let pkce = PkceChallenge::generate();
        let verifier = pkce.verifier.clone();
        let auth_url = AntigravityLanguageModelProvider::build_authorization_url(&pkce);

        // Store PKCE for later verification
        self.state.read(cx).inner.lock().pending_pkce = Some(pkce);
        self.status_message = Some("Starting authentication...".to_string());
        cx.notify();

        let state = self.state.clone();
        let http_client = self.http_client.clone();

        // Spawn background task to handle OAuth callback
        cx.spawn(async move |this, cx| {
            // Start callback server and wait for the authorization code
            let code = match wait_for_oauth_callback().await {
                Ok(code) => code,
                Err(e) => {
                    this.update(cx, |this, cx| {
                        this.status_message = Some(format!("Callback error: {}", e));
                        cx.notify();
                    })
                    .ok();
                    return;
                }
            };

            // Update status
            this.update(cx, |this, cx| {
                this.status_message = Some("Exchanging code for tokens...".to_string());
                cx.notify();
            })
            .ok();

            // Complete OAuth flow (exchange code, fetch user info, get project ID)
            match complete_oauth_flow(http_client, &code, &verifier).await {
                Ok(credentials) => {
                    let store_task =
                        state.update(cx, |state, cx| state.store_credentials(credentials, cx));
                    if let Err(err) = store_task.await {
                        this.update(cx, |this, cx| {
                            this.status_message =
                                Some(format!("Failed to save credentials: {}", err));
                            cx.notify();
                        })
                        .ok();
                        return;
                    }

                    this.update(cx, |this, cx| {
                        this.status_message = Some("Successfully signed in!".to_string());
                        cx.notify();
                    })
                    .ok();
                }
                Err(e) => {
                    this.update(cx, |this, cx| {
                        this.status_message = Some(format!("Authentication failed: {}", e));
                        cx.notify();
                    })
                    .ok();
                }
            }
        })
        .detach();

        // Open browser AFTER spawning the callback server task
        cx.open_url(&auth_url);
        self.status_message = Some("Waiting for browser sign-in...".to_string());
        cx.notify();
    }

    fn sign_out(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.clone();
        cx.spawn(async move |this, cx| {
            let task = state.update(cx, |state, cx| state.reset_credentials(cx));
            let _ = task.await;
            this.update(cx, |this, cx| {
                this.status_message = None;
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.load_credentials_task.is_some() {
            return v_flex()
                .gap_2()
                .child(Label::new("Loading credentials..."))
                .into_any();
        }

        let is_authenticated = self.state.read(cx).is_authenticated();
        let email = self.state.read(cx).email();

        if is_authenticated {
            let display_text = email
                .map(|e| format!("Signed in as {}", e))
                .unwrap_or_else(|| "Signed in".to_string());

            v_flex()
                .gap_2()
                .child(Label::new(display_text))
                .child(Button::new("sign_out", "Sign Out").on_click(cx.listener(Self::sign_out)))
                .into_any()
        } else {
            let mut view = v_flex().gap_2().child(Label::new(
                "Sign in with Google to use Antigravity models (Gemini and Claude)",
            ));

            if let Some(ref msg) = self.status_message {
                view = view.child(
                    Label::new(msg.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                );
            }

            view.child(
                Button::new("sign_in", "Sign in with Google")
                    .style(ButtonStyle::Filled)
                    .on_click(cx.listener(Self::start_oauth_flow)),
            )
            .child(
                Label::new("Google AI plan access through Google Cloud Code Assist")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .into_any()
        }
    }
}
