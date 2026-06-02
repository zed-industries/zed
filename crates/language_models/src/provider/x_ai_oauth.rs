//! xAI provider for the SuperGrok subscription, authenticated via OAuth rather
//! than an API key. Completions are streamed against xAI's OpenAI-compatible API.
//!
//! This coexists with the separate API-key xAI provider as a distinct provider;
//! the two never share stored credentials.

use anyhow::{Context as _, Result, anyhow};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, future::Shared};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolSchemaFormat, RateLimiter,
};
use open_ai::ResponseStreamEvent;
use rand::RngCore as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use strum::IntoEnumIterator as _;
use ui::{ConfiguredApiCard, prelude::*};
use url::form_urlencoded;
use util::ResultExt as _;
use x_ai::XAI_API_URL;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("x_ai_oauth");
const PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("SuperGrok Subscription");

// xAI's OAuth endpoints. We reuse the public Grok-CLI OAuth client; xAI's auth
// server rejects loopback OAuth from non-allowlisted clients, which is why the
// redirect URI must bind exactly to `127.0.0.1:56121/callback` and the authorize
// request must include `plan=generic` (see CALLBACK consts and `do_oauth_flow`).
// Mirrors opencode's `packages/opencode/src/plugin/xai.ts`.
const XAI_CLIENT_ID: &str = "b1a00492-073a-47ea-816f-4c329264a828";
const XAI_AUTHORIZE_URL: &str = "https://auth.x.ai/oauth2/authorize";
const XAI_TOKEN_URL: &str = "https://auth.x.ai/oauth2/token";
const XAI_SCOPE: &str = "openid profile email offline_access grok-cli:access api:access";

// The Grok-CLI OAuth client only allow-lists this exact loopback redirect URI, so
// (unlike providers that accept an arbitrary loopback port) we must pin host, port
// and path. There is no fallback port because no other port is registered.
const XAI_CALLBACK_HOST: &str = "127.0.0.1";
const XAI_CALLBACK_PORT: u16 = 56121;
const XAI_CALLBACK_PATH: &str = "/callback";

// Keyed by xAI's auth host (distinct from the API-key provider, which keys the
// keychain entry by the API host `https://api.x.ai/v1`) so the two xAI providers
// never clobber each other's stored credentials.
const CREDENTIALS_KEY: &str = "https://auth.x.ai/oauth2";
// `is_expired` reports a token as expired this long before its real expiry, so a
// refresh is triggered while the current token still works (avoids racing expiry
// mid-request).
const TOKEN_REFRESH_BUFFER_MS: u64 = 5 * 60 * 1000;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct XAiCredentials {
    access_token: String,
    refresh_token: String,
    expires_at_ms: u64,
}

impl XAiCredentials {
    fn is_expired(&self) -> bool {
        now_ms() + TOKEN_REFRESH_BUFFER_MS >= self.expires_at_ms
    }
}

/// The single observable source of truth for SuperGrok authentication, shared
/// (via one `Entity<State>`) between the configuration view, every language
/// model, and the async sign-in/refresh/load tasks.
///
/// `credentials.is_some()` is the authoritative "authenticated" signal.
/// Mutations call `cx.notify()` so the configuration view re-renders.
pub struct State {
    /// Cached OAuth tokens. `None` is ambiguous: it means either "signed out"
    /// or "keychain load (`load_task`) hasn't finished yet". Callers that gate
    /// on authentication must await `load_task` before treating `None` as
    /// unauthenticated (see `authenticate`).
    credentials: Option<XAiCredentials>,
    /// `Some` while an interactive OAuth sign-in is in flight; its presence is
    /// what suppresses a second concurrent sign-in (see `is_signing_in`).
    sign_in_task: Option<Task<Result<()>>>,
    /// Single-flight latch for an in-flight token refresh. `Some` means a
    /// refresh is running; concurrent callers join this shared task instead of
    /// starting their own. Always reset to `None` on completion (success or
    /// failure) so the next expiry can spawn a fresh refresh.
    refresh_task: Option<Shared<Task<Result<XAiCredentials, Arc<anyhow::Error>>>>>,
    /// One-shot read of stored credentials from the keychain, started in `new`.
    /// `authenticate` awaits it so a freshly launched app reports authenticated
    /// once any saved tokens are loaded. Set back to `None` once the read completes.
    load_task: Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    /// Bumped by `do_sign_out`. A refresh captures it before its network
    /// request and re-checks it on completion; a changed value means a
    /// sign-out raced the refresh, so the result is discarded rather than
    /// written back as the current credentials.
    auth_generation: u64,
    /// User-facing message for the most recent sign-in or token-refresh
    /// failure, rendered in the configuration view. Distinct from the
    /// structured errors returned to callers. Cleared whenever a fresh auth
    /// action starts (sign-in, sign-out) and on a successful sign-in.
    last_auth_error: Option<SharedString>,
}

/// Classifies a failed token refresh by what the caller must do about it.
///
/// `Fatal` means the refresh token itself is rejected: the session is
/// unrecoverable, so the caller must clear the in-memory credentials and the
/// keychain entry and force the user to sign in again. `Transient` means the
/// failure is incidental and the existing credentials are kept so a later
/// request can retry. (See `refresh_token` for which conditions map to which.)
#[derive(Debug)]
enum RefreshError {
    Fatal(anyhow::Error),
    Transient(anyhow::Error),
}

impl std::fmt::Display for RefreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefreshError::Fatal(e) => write!(f, "{e}"),
            RefreshError::Transient(e) => write!(f, "{e}"),
        }
    }
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.credentials.is_some()
    }

    fn is_signing_in(&self) -> bool {
        self.sign_in_task.is_some()
    }
}

pub struct XAiOAuthLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

impl XAiOAuthLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|_cx| State {
            credentials: None,
            sign_in_task: None,
            refresh_task: None,
            load_task: None,
            credentials_provider,
            auth_generation: 0,
            last_auth_error: None,
        });

        let provider = Self { http_client, state };

        provider.load_credentials(cx);

        provider
    }

    /// Kicks off the one-time load of persisted credentials into `State`, storing
    /// the shared task in `load_task` so `authenticate` can await it before
    /// concluding the user is signed out. Malformed stored credentials are logged
    /// and treated as absent rather than surfaced as an error.
    fn load_credentials(&self, cx: &mut App) {
        let state = self.state.downgrade();
        let load_task = cx
            .spawn(async move |cx| {
                let credentials_provider =
                    state.read_with(&*cx, |s, _| s.credentials_provider.clone())?;
                let result = credentials_provider
                    .read_credentials(CREDENTIALS_KEY, &*cx)
                    .await;
                state.update(cx, |s, cx| {
                    if let Ok(Some((_, bytes))) = result {
                        match serde_json::from_slice::<XAiCredentials>(&bytes) {
                            Ok(creds) => s.credentials = Some(creds),
                            Err(err) => {
                                log::warn!(
                                    "Failed to deserialize SuperGrok subscription credentials: {err}"
                                );
                            }
                        }
                    }
                    s.load_task = None;
                    cx.notify();
                })?;
                Ok::<(), Arc<anyhow::Error>>(())
            })
            .shared();

        self.state.update(cx, |s, _| {
            s.load_task = Some(load_task);
        });
    }

    fn sign_out(&self, cx: &mut App) -> Task<Result<()>> {
        do_sign_out(&self.state.downgrade(), cx)
    }

    fn create_language_model(&self, model: x_ai::Model) -> Arc<dyn LanguageModel> {
        Arc::new(XAiOAuthLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for XAiOAuthLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for XAiOAuthLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiXAi)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(x_ai::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(x_ai::Model::default_fast()))
    }

    fn provided_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        x_ai::Model::iter()
            .filter(|model| !matches!(model, x_ai::Model::Custom { .. }))
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated(cx) {
            return Task::ready(Ok(()));
        }
        let load_task = self.state.read(cx).load_task.clone();
        if let Some(load_task) = load_task {
            let weak_state = self.state.downgrade();
            cx.spawn(async move |cx| {
                let _ = load_task.await;
                let is_auth = weak_state
                    .read_with(&*cx, |s, _| s.is_authenticated())
                    .unwrap_or(false);
                if is_auth {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "Sign in with your SuperGrok subscription to use this provider."
                    )
                    .into())
                }
            })
        } else {
            Task::ready(Err(anyhow!(
                "Sign in with your SuperGrok subscription to use this provider."
            )
            .into()))
        }
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        _window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        let state = self.state.clone();
        let http_client = self.http_client.clone();
        cx.new(|_cx| ConfigurationView { state, http_client })
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.sign_out(cx)
    }
}

fn x_ai_reasoning_efforts(model: &x_ai::Model) -> &'static [open_ai::ReasoningEffort] {
    if model.supports_reasoning_effort() {
        &[
            open_ai::ReasoningEffort::None,
            open_ai::ReasoningEffort::Low,
            open_ai::ReasoningEffort::Medium,
            open_ai::ReasoningEffort::High,
        ]
    } else {
        &[]
    }
}

fn default_thinking_reasoning_effort(model: &x_ai::Model) -> Option<open_ai::ReasoningEffort> {
    if model.supports_reasoning_effort() {
        Some(open_ai::ReasoningEffort::Low)
    } else {
        None
    }
}

fn reasoning_effort_for_request(
    request: &LanguageModelRequest,
    model: &x_ai::Model,
) -> Option<open_ai::ReasoningEffort> {
    let supported_efforts = x_ai_reasoning_efforts(model);
    if supported_efforts.is_empty() {
        return None;
    }

    if request.thinking_allowed {
        request
            .thinking_effort
            .as_deref()
            .and_then(|effort| effort.parse::<open_ai::ReasoningEffort>().ok())
            .filter(|effort| supported_efforts.contains(effort))
            .filter(|effort| *effort != open_ai::ReasoningEffort::None)
            .or_else(|| default_thinking_reasoning_effort(model))
    } else if supported_efforts.contains(&open_ai::ReasoningEffort::None) {
        Some(open_ai::ReasoningEffort::None)
    } else {
        None
    }
}

fn supported_thinking_effort_levels(model: &x_ai::Model) -> Vec<LanguageModelEffortLevel> {
    let default_effort = default_thinking_reasoning_effort(model);
    x_ai_reasoning_efforts(model)
        .iter()
        .copied()
        .filter_map(|effort| {
            let (name, value) = match effort {
                open_ai::ReasoningEffort::None => return None,
                open_ai::ReasoningEffort::Minimal => ("Minimal", "minimal"),
                open_ai::ReasoningEffort::Low => ("Low", "low"),
                open_ai::ReasoningEffort::Medium => ("Medium", "medium"),
                open_ai::ReasoningEffort::High => ("High", "high"),
                open_ai::ReasoningEffort::XHigh => ("Extra High", "xhigh"),
            };

            Some(LanguageModelEffortLevel {
                name: name.into(),
                value: value.into(),
                is_default: Some(effort) == default_effort,
            })
        })
        .collect()
}

struct XAiOAuthLanguageModel {
    id: LanguageModelId,
    model: x_ai::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl LanguageModel for XAiOAuthLanguageModel {
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
        self.model.supports_tool()
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => true,
        }
    }

    fn supports_thinking(&self) -> bool {
        self.model.supports_reasoning_effort()
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        supported_thinking_effort_levels(&self.model)
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        if self.model.requires_json_schema_subset() {
            LanguageModelToolSchemaFormat::JsonSchemaSubset
        } else {
            LanguageModelToolSchemaFormat::JsonSchema
        }
    }

    fn telemetry_id(&self) -> String {
        format!("x_ai_oauth/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn supports_split_token_display(&self) -> bool {
        true
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
        let reasoning_effort = reasoning_effort_for_request(&request, &self.model);
        let request = crate::provider::open_ai::into_open_ai(
            request,
            self.model.id(),
            self.model.supports_parallel_tool_calls(),
            self.model.supports_prompt_cache_key(),
            self.max_output_tokens(),
            reasoning_effort,
            false,
        );

        let state = self.state.downgrade();
        let http_client = self.http_client.clone();
        let request_limiter = self.request_limiter.clone();

        let future = cx.spawn(async move |cx| {
            let credentials = get_fresh_credentials(&state, &http_client, cx).await?;
            let access_token = credentials.access_token;
            let stream = request_limiter
                .stream(async move {
                    let response = open_ai::stream_completion(
                        http_client.as_ref(),
                        PROVIDER_NAME.0.as_str(),
                        XAI_API_URL,
                        &access_token,
                        request,
                    )
                    .await?;
                    Ok(response)
                })
                .await?;
            Ok::<
                futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>,
                LanguageModelCompletionError,
            >(stream.boxed())
        });

        async move {
            let mapper = crate::provider::open_ai::OpenAiEventMapper::new();
            Ok(mapper.map_stream(future.await?).boxed())
        }
        .boxed()
    }
}

/// Returns usable credentials for a request, refreshing first if the cached
/// token is already expired. Blocks until any needed refresh completes (it does
/// not return stale tokens). Concurrent callers single-flight through
/// `State.refresh_task`, so only one network refresh runs; a successful refresh
/// is written to the keychain. Returns `NoApiKey` when the user is signed out.
async fn get_fresh_credentials(
    state: &gpui::WeakEntity<State>,
    http_client: &Arc<dyn HttpClient>,
    cx: &mut AsyncApp,
) -> Result<XAiCredentials, LanguageModelCompletionError> {
    let (creds, existing_task) = state
        .read_with(&*cx, |s, _| (s.credentials.clone(), s.refresh_task.clone()))
        .map_err(LanguageModelCompletionError::Other)?;

    let creds = creds.ok_or(LanguageModelCompletionError::NoApiKey {
        provider: PROVIDER_NAME,
    })?;

    if !creds.is_expired() {
        return Ok(creds);
    }

    // If another caller is already refreshing, await their result.
    if let Some(shared_task) = existing_task {
        return shared_task
            .await
            .map_err(|e| LanguageModelCompletionError::Other(anyhow::anyhow!("{e}")));
    }

    // We are the first caller to notice expiry — spawn the refresh task.
    let http_client_clone = http_client.clone();
    let state_clone = state.clone();
    let refresh_token_value = creds.refresh_token.clone();

    // Capture the generation so we can detect sign-outs that happened during refresh.
    let generation = state
        .read_with(&*cx, |s, _| s.auth_generation)
        .map_err(LanguageModelCompletionError::Other)?;

    let shared_task = cx
        .spawn(async move |cx| {
            let result = refresh_token(&http_client_clone, &refresh_token_value).await;

            match result {
                Ok(refreshed) => {
                    let persist_result: Result<XAiCredentials, Arc<anyhow::Error>> = async {
                        // Check if auth_generation changed (sign-out during refresh).
                        let current_generation = state_clone
                            .read_with(&*cx, |s, _| s.auth_generation)
                            .map_err(|e| Arc::new(e))?;
                        if current_generation != generation {
                            return Err(Arc::new(anyhow!(
                                "Sign-out occurred during token refresh"
                            )));
                        }

                        let credentials_provider = state_clone
                            .read_with(&*cx, |s, _| s.credentials_provider.clone())
                            .map_err(|e| Arc::new(e))?;

                        let json =
                            serde_json::to_vec(&refreshed).map_err(|e| Arc::new(e.into()))?;

                        credentials_provider
                            .write_credentials(CREDENTIALS_KEY, "Bearer", &json, &*cx)
                            .await
                            .map_err(|e| Arc::new(e))?;

                        state_clone
                            .update(cx, |s, _| {
                                s.credentials = Some(refreshed.clone());
                                s.refresh_task = None;
                            })
                            .map_err(|e| Arc::new(e))?;

                        Ok(refreshed)
                    }
                    .await;

                    // Clear refresh_task on failure too.
                    if persist_result.is_err() {
                        state_clone
                            .update(cx, |s, _| {
                                s.refresh_task = None;
                            })
                            .log_err();
                    }

                    persist_result
                }
                Err(RefreshError::Fatal(e)) => {
                    log::error!("SuperGrok subscription token refresh failed fatally: {e:?}");
                    state_clone
                        .update(cx, |s, cx| {
                            s.refresh_task = None;
                            s.credentials = None;
                            s.last_auth_error =
                                Some("Your session has expired. Please sign in again.".into());
                            cx.notify();
                        })
                        .log_err();
                    // Also clear the keychain so stale credentials aren't loaded next time.
                    if let Ok(credentials_provider) =
                        state_clone.read_with(&*cx, |s, _| s.credentials_provider.clone())
                    {
                        credentials_provider
                            .delete_credentials(CREDENTIALS_KEY, &*cx)
                            .await
                            .log_err();
                    }
                    Err(Arc::new(e))
                }
                Err(RefreshError::Transient(e)) => {
                    log::warn!("SuperGrok subscription token refresh failed transiently: {e:?}");
                    state_clone
                        .update(cx, |s, _| {
                            s.refresh_task = None;
                        })
                        .log_err();
                    Err(Arc::new(e))
                }
            }
        })
        .shared();

    // Store the shared task so concurrent callers can join on it.
    state
        .update(cx, |s, _| {
            s.refresh_task = Some(shared_task.clone());
        })
        .map_err(LanguageModelCompletionError::Other)?;

    shared_task
        .await
        .map_err(|e| LanguageModelCompletionError::Other(anyhow::anyhow!("{e}")))
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    // Optional per RFC 6749 §6: the refresh grant MAY omit a new refresh token,
    // in which case the previously-issued one stays valid.
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    expires_in: Option<u64>,
}

/// Runs the interactive authorization-code-with-PKCE flow and returns the
/// resulting credentials.
///
/// Side effects: binds a loopback HTTP listener for the redirect and opens the
/// system browser to the consent page. This future stays pending until the user
/// completes consent and xAI redirects back, so it can block for an arbitrary
/// time (or be cancelled by dropping it).
///
/// Errors if the token response carries no refresh token: although that is
/// permitted in general, a brand-new session with no way to renew is unusable.
async fn do_oauth_flow(http_client: Arc<dyn HttpClient>, cx: &AsyncApp) -> Result<XAiCredentials> {
    // Start the callback server FIRST so the redirect URI is ready.
    let (redirect_uri, callback_rx) =
        oauth_callback_server::start_oauth_callback_server_with_config(
            oauth_callback_server::OAuthCallbackServerConfig {
                host: XAI_CALLBACK_HOST,
                preferred_port: XAI_CALLBACK_PORT,
                fallback_port: None,
                path: XAI_CALLBACK_PATH,
            },
        )
        .context("Failed to start OAuth callback server")?;

    // PKCE verifier: 32 random bytes → base64url (no padding).
    let mut verifier_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut verifier_bytes);
    let verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);

    // PKCE challenge: SHA-256(verifier) → base64url.
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let challenge = URL_SAFE_NO_PAD.encode(hasher.finalize().as_slice());

    // CSRF state and OIDC nonce: 16 random bytes → hex string each.
    let oauth_state = random_hex_16();
    let nonce = random_hex_16();

    let mut auth_url = url::Url::parse(XAI_AUTHORIZE_URL).expect("valid base URL");
    auth_url
        .query_pairs_mut()
        .append_pair("client_id", XAI_CLIENT_ID)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("scope", XAI_SCOPE)
        .append_pair("response_type", "code")
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("state", &oauth_state)
        .append_pair("nonce", &nonce)
        // `plan=generic` is REQUIRED: without it xAI rejects loopback OAuth from
        // the (non-allowlisted) public Grok-CLI client we authenticate as.
        .append_pair("plan", "generic")
        .append_pair("referrer", "zed");

    // Open browser AFTER the listener is ready.
    cx.update(|cx| cx.open_url(auth_url.as_str()));

    // Await the callback.
    let callback = callback_rx
        .await
        .map_err(|_| anyhow!("OAuth callback was cancelled"))?
        .context("OAuth callback failed")?;

    // Validate CSRF state.
    if callback.state != oauth_state {
        return Err(anyhow!("OAuth state mismatch"));
    }

    let tokens = exchange_code(&http_client, &callback.code, &verifier, &redirect_uri)
        .await
        .context("Token exchange failed")?;

    // The authorization-code grant must include a refresh token; without it we
    // could never refresh and the session would silently die at first expiry.
    let refresh_token = tokens
        .refresh_token
        .filter(|token| !token.is_empty())
        .context("Token response did not include a refresh token")?;

    Ok(XAiCredentials {
        expires_at_ms: access_token_expires_at_ms(&tokens.access_token, tokens.expires_in),
        access_token: tokens.access_token,
        refresh_token,
    })
}

async fn exchange_code(
    client: &Arc<dyn HttpClient>,
    code: &str,
    verifier: &str,
    redirect_uri: &str,
) -> Result<TokenResponse> {
    let body = form_urlencoded::Serializer::new(String::new())
        .append_pair("grant_type", "authorization_code")
        .append_pair("client_id", XAI_CLIENT_ID)
        .append_pair("code", code)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("code_verifier", verifier)
        .finish();

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(XAI_TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .body(AsyncBody::from(body))?;

    let mut response = client.send(request).await?;
    let mut body = String::new();
    smol::io::AsyncReadExt::read_to_string(response.body_mut(), &mut body).await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Token exchange failed (HTTP {}): {body}",
            response.status()
        ));
    }

    serde_json::from_str::<TokenResponse>(&body).context("Failed to parse token response")
}

/// Exchanges the refresh token for a fresh `XAiCredentials` pair.
///
/// The `Err` variant is the caller's contract: `Fatal` means the refresh token
/// is revoked or invalid, so the caller should sign the user out and discard
/// stored credentials; `Transient` means a later retry may succeed, so existing
/// credentials should be kept. On success, if xAI returns no new refresh token
/// the passed-in one is carried over into the returned credentials.
async fn refresh_token(
    client: &Arc<dyn HttpClient>,
    refresh_token: &str,
) -> Result<XAiCredentials, RefreshError> {
    let body = form_urlencoded::Serializer::new(String::new())
        .append_pair("grant_type", "refresh_token")
        .append_pair("client_id", XAI_CLIENT_ID)
        .append_pair("refresh_token", refresh_token)
        .finish();

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(XAI_TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .body(AsyncBody::from(body))
        .map_err(|e| RefreshError::Transient(e.into()))?;

    let mut response = client
        .send(request)
        .await
        .map_err(|e| RefreshError::Transient(e))?;
    let status = response.status();
    let mut body = String::new();
    smol::io::AsyncReadExt::read_to_string(response.body_mut(), &mut body)
        .await
        .map_err(|e| RefreshError::Transient(e.into()))?;

    if !status.is_success() {
        let err = anyhow!("Token refresh failed (HTTP {}): {body}", status);
        // 400/401/403 indicate a revoked or invalid refresh token.
        // 5xx and other errors are treated as transient.
        if status == http_client::StatusCode::BAD_REQUEST
            || status == http_client::StatusCode::UNAUTHORIZED
            || status == http_client::StatusCode::FORBIDDEN
        {
            return Err(RefreshError::Fatal(err));
        }
        return Err(RefreshError::Transient(err));
    }

    let tokens: TokenResponse =
        serde_json::from_str(&body).map_err(|e| RefreshError::Transient(e.into()))?;

    // The refresh grant MAY omit a new refresh token; when it does the existing
    // one remains valid, so reuse it (mirrors opencode's `tokens.refresh_token
    // || refreshToken`). Treating that case as an error would wedge the session
    // even though xAI returned a perfectly usable access token.
    let next_refresh_token = tokens
        .refresh_token
        .filter(|token| !token.is_empty())
        .unwrap_or_else(|| refresh_token.to_string());

    Ok(XAiCredentials {
        expires_at_ms: access_token_expires_at_ms(&tokens.access_token, tokens.expires_in),
        access_token: tokens.access_token,
        refresh_token: next_refresh_token,
    })
}

fn random_hex_16() -> String {
    let mut bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or_else(|err| {
            log::error!("System clock is before UNIX epoch: {err}");
            0
        })
}

/// Absolute access-token expiry in epoch-milliseconds.
///
/// xAI access tokens are JWTs whose `exp` claim is the authoritative expiry, and
/// xAI does not reliably return `expires_in` — so the JWT claim is the
/// load-bearing signal (matching opencode's xAI flow). Falls back to `expires_in`
/// and finally to a conservative one-hour default for opaque tokens.
fn access_token_expires_at_ms(access_token: &str, expires_in: Option<u64>) -> u64 {
    if let Some(exp_seconds) = jwt_exp_seconds(access_token) {
        return exp_seconds.saturating_mul(1000);
    }
    now_ms() + expires_in.unwrap_or(3600) * 1000
}

/// Decode the `exp` (seconds since epoch) claim from a JWT access token, or
/// `None` for opaque (non-JWT) tokens. The signature is intentionally NOT
/// verified — this only informs proactive refresh timing, never trust.
fn jwt_exp_seconds(token: &str) -> Option<u64> {
    let payload_b64 = token.split('.').nth(1)?;
    let payload = URL_SAFE_NO_PAD.decode(payload_b64).ok()?;
    let claims = serde_json::from_slice::<serde_json::Value>(&payload).ok()?;
    claims.get("exp")?.as_u64()
}

/// Starts the OAuth sign-in flow as a fire-and-forget task and returns
/// immediately; the outcome is reported only by mutating `State` (success sets
/// `credentials`, failure sets `last_auth_error`, both call `cx.notify()`), never
/// via a return value. No-op while a sign-in is already in progress. Dropping the
/// `State` entity cancels an in-flight sign-in.
fn do_sign_in(state: &Entity<State>, http_client: &Arc<dyn HttpClient>, cx: &mut App) {
    if state.read(cx).is_signing_in() {
        return;
    }

    let weak_state = state.downgrade();
    let http_client = http_client.clone();

    let task = cx.spawn(async move |cx| {
        match do_oauth_flow(http_client, &*cx).await {
            Ok(creds) => {
                let persist_result = async {
                    let credentials_provider =
                        weak_state.read_with(&*cx, |s, _| s.credentials_provider.clone())?;
                    let json = serde_json::to_vec(&creds)?;
                    credentials_provider
                        .write_credentials(CREDENTIALS_KEY, "Bearer", &json, &*cx)
                        .await?;
                    anyhow::Ok(())
                }
                .await;

                match persist_result {
                    Ok(()) => {
                        weak_state
                            .update(cx, |s, cx| {
                                s.credentials = Some(creds);
                                s.sign_in_task = None;
                                s.last_auth_error = None;
                                cx.notify();
                            })
                            .log_err();
                    }
                    Err(err) => {
                        log::error!(
                            "SuperGrok subscription sign-in failed to persist credentials: {err:?}"
                        );
                        weak_state
                            .update(cx, |s, cx| {
                                s.sign_in_task = None;
                                s.last_auth_error =
                                    Some("Failed to save credentials. Please try again.".into());
                                cx.notify();
                            })
                            .log_err();
                    }
                }
            }
            Err(err) => {
                log::error!("SuperGrok subscription sign-in failed: {err:?}");
                weak_state
                    .update(cx, |s, cx| {
                        s.sign_in_task = None;
                        s.last_auth_error = Some("Sign-in failed. Please try again.".into());
                        cx.notify();
                    })
                    .log_err();
            }
        }
        anyhow::Ok(())
    });

    state.update(cx, |s, cx| {
        s.last_auth_error = None;
        s.sign_in_task = Some(task);
        cx.notify();
    });
}

/// Signs the user out. Bumps `auth_generation` so any token refresh already in
/// flight (see `get_fresh_credentials`) observes the change and discards its
/// result rather than re-persisting now-stale credentials. The returned task
/// performs the keychain deletion; in-memory state is cleared synchronously
/// before it is spawned.
fn do_sign_out(state: &gpui::WeakEntity<State>, cx: &mut App) -> Task<Result<()>> {
    let weak_state = state.clone();
    // Clear credentials and cancel in-flight work immediately so the UI
    // reflects the sign-out right away.
    weak_state
        .update(cx, |s, cx| {
            s.auth_generation += 1;
            s.credentials = None;
            s.sign_in_task = None;
            s.refresh_task = None;
            s.last_auth_error = None;
            cx.notify();
        })
        .log_err();

    cx.spawn(async move |cx| {
        let credentials_provider =
            weak_state.read_with(&*cx, |s, _| s.credentials_provider.clone())?;
        credentials_provider
            .delete_credentials(CREDENTIALS_KEY, &*cx)
            .await
            .context("Failed to delete SuperGrok subscription credentials from keychain")?;
        anyhow::Ok(())
    })
}

struct ConfigurationView {
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);

        if state.is_authenticated() {
            let weak_state = self.state.downgrade();

            return v_flex()
                .child(
                    ConfiguredApiCard::new(SharedString::from("Signed in"))
                        .button_label("Sign Out")
                        .on_click(cx.listener(move |_this, _, _window, cx| {
                            do_sign_out(&weak_state, cx).detach_and_log_err(cx);
                        })),
                )
                .into_any_element();
        }

        let last_auth_error = state.last_auth_error.clone();
        let provider_state = self.state.clone();
        let http_client = self.http_client.clone();

        let is_signing_in = state.is_signing_in();
        let button_label = if is_signing_in {
            "Signing in…"
        } else {
            "Sign in to use SuperGrok Subscription"
        };

        v_flex()
            .gap_2()
            .child(Label::new(
                "Sign in with your xAI SuperGrok subscription to use Grok models in Zed's agent.",
            ))
            .child(
                Button::new("sign-in", button_label)
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .loading(is_signing_in)
                    .disabled(is_signing_in)
                    .when(!is_signing_in, |this| {
                        this.start_icon(
                            Icon::new(IconName::AiXAi)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                    })
                    .on_click(move |_, _window, cx| {
                        do_sign_in(&provider_state, &http_client, cx);
                    }),
            )
            .when_some(last_auth_error, |this, error| {
                this.child(
                    h_flex()
                        .gap_1()
                        .justify_center()
                        .child(
                            Icon::new(IconName::XCircle)
                                .color(Color::Error)
                                .size(IconSize::Small),
                        )
                        .child(Label::new(error).color(Color::Muted)),
                )
            })
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use http_client::FakeHttpClient;
    use parking_lot::Mutex;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct FakeCredentialsProvider {
        storage: Mutex<Option<(String, Vec<u8>)>>,
    }

    impl FakeCredentialsProvider {
        fn new() -> Self {
            Self {
                storage: Mutex::new(None),
            }
        }
    }

    impl CredentialsProvider for FakeCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
            Box::pin(async { Ok(self.storage.lock().clone()) })
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            username: &'a str,
            password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            self.storage
                .lock()
                .replace((username.to_string(), password.to_vec()));
            Box::pin(async { Ok(()) })
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            *self.storage.lock() = None;
            Box::pin(async { Ok(()) })
        }
    }

    fn make_expired_credentials() -> XAiCredentials {
        XAiCredentials {
            access_token: "old_access".to_string(),
            refresh_token: "old_refresh".to_string(),
            expires_at_ms: 0,
        }
    }

    fn make_fresh_credentials() -> XAiCredentials {
        XAiCredentials {
            access_token: "fresh_access".to_string(),
            refresh_token: "fresh_refresh".to_string(),
            expires_at_ms: now_ms() + 3_600_000,
        }
    }

    fn fake_token_response() -> String {
        serde_json::json!({
            "access_token": "fresh_access",
            "refresh_token": "fresh_refresh",
            "expires_in": 3600
        })
        .to_string()
    }

    #[gpui::test]
    async fn test_concurrent_refresh_deduplicates(cx: &mut TestAppContext) {
        let refresh_count = Arc::new(AtomicUsize::new(0));
        let refresh_count_clone = refresh_count.clone();

        let http_client = FakeHttpClient::create(move |_request| {
            let refresh_count = refresh_count_clone.clone();
            async move {
                refresh_count.fetch_add(1, Ordering::SeqCst);
                let body = fake_token_response();
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(body))?)
            }
        });

        let state = cx.new(|_cx| State {
            credentials: Some(make_expired_credentials()),
            sign_in_task: None,
            refresh_task: None,
            load_task: None,
            credentials_provider: Arc::new(FakeCredentialsProvider::new()),
            auth_generation: 0,
            last_auth_error: None,
        });

        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let weak1 = weak_state.clone();
        let http1 = http.clone();
        let task1 =
            cx.spawn(async move |mut cx| get_fresh_credentials(&weak1, &http1, &mut cx).await);

        let weak2 = weak_state.clone();
        let http2 = http.clone();
        let task2 =
            cx.spawn(async move |mut cx| get_fresh_credentials(&weak2, &http2, &mut cx).await);

        cx.run_until_parked();
        let result1 = task1.await;
        let result2 = task2.await;

        assert!(result1.is_ok(), "first refresh should succeed");
        assert!(result2.is_ok(), "second refresh should succeed");
        assert_eq!(result1.unwrap().access_token, "fresh_access");
        assert_eq!(result2.unwrap().access_token, "fresh_access");
        assert_eq!(
            refresh_count.load(Ordering::SeqCst),
            1,
            "refresh_token should only be called once despite two concurrent callers"
        );
    }

    #[gpui::test]
    async fn test_fresh_credentials_skip_refresh(cx: &mut TestAppContext) {
        let refresh_count = Arc::new(AtomicUsize::new(0));
        let refresh_count_clone = refresh_count.clone();

        let http_client = FakeHttpClient::create(move |_request| {
            let refresh_count = refresh_count_clone.clone();
            async move {
                refresh_count.fetch_add(1, Ordering::SeqCst);
                let body = fake_token_response();
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(body))?)
            }
        });

        let state = cx.new(|_cx| State {
            credentials: Some(make_fresh_credentials()),
            sign_in_task: None,
            refresh_task: None,
            load_task: None,
            credentials_provider: Arc::new(FakeCredentialsProvider::new()),
            auth_generation: 0,
            last_auth_error: None,
        });

        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let weak = weak_state.clone();
        let http_clone = http.clone();
        let result = cx
            .spawn(async move |mut cx| get_fresh_credentials(&weak, &http_clone, &mut cx).await)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().access_token, "fresh_access");
        assert_eq!(
            refresh_count.load(Ordering::SeqCst),
            0,
            "no refresh should happen when credentials are fresh"
        );
    }

    #[gpui::test]
    async fn test_no_credentials_returns_no_api_key(cx: &mut TestAppContext) {
        let http_client = FakeHttpClient::create(|_| async {
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::default())?)
        });

        let state = cx.new(|_cx| State {
            credentials: None,
            sign_in_task: None,
            refresh_task: None,
            load_task: None,
            credentials_provider: Arc::new(FakeCredentialsProvider::new()),
            auth_generation: 0,
            last_auth_error: None,
        });

        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let weak = weak_state.clone();
        let http_clone = http.clone();
        let result = cx
            .spawn(async move |mut cx| get_fresh_credentials(&weak, &http_clone, &mut cx).await)
            .await;

        assert!(matches!(
            result,
            Err(LanguageModelCompletionError::NoApiKey { .. })
        ));
    }

    #[gpui::test]
    async fn test_fatal_refresh_clears_auth_state(cx: &mut TestAppContext) {
        let http_client = FakeHttpClient::create(move |_request| async move {
            Ok(http_client::Response::builder()
                .status(401)
                .body(http_client::AsyncBody::from(r#"{"error":"invalid_grant"}"#))?)
        });

        let creds_provider = Arc::new(FakeCredentialsProvider::new());
        let state = cx.new(|_cx| State {
            credentials: Some(make_expired_credentials()),
            sign_in_task: None,
            refresh_task: None,
            load_task: None,
            credentials_provider: creds_provider.clone(),
            auth_generation: 0,
            last_auth_error: None,
        });

        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let weak = weak_state.clone();
        let http_clone = http.clone();
        let result = cx
            .spawn(async move |mut cx| get_fresh_credentials(&weak, &http_clone, &mut cx).await)
            .await;

        cx.run_until_parked();

        assert!(result.is_err(), "fatal refresh should return an error");
        cx.read(|cx| {
            let s = state.read(cx);
            assert!(
                s.credentials.is_none(),
                "credentials should be cleared on fatal refresh failure"
            );
            assert!(
                s.last_auth_error.is_some(),
                "last_auth_error should be set on fatal refresh failure"
            );
        });
    }

    #[gpui::test]
    async fn test_transient_refresh_keeps_credentials(cx: &mut TestAppContext) {
        let http_client = FakeHttpClient::create(move |_request| async move {
            Ok(http_client::Response::builder()
                .status(500)
                .body(http_client::AsyncBody::from("Internal Server Error"))?)
        });

        let state = cx.new(|_cx| State {
            credentials: Some(make_expired_credentials()),
            sign_in_task: None,
            refresh_task: None,
            load_task: None,
            credentials_provider: Arc::new(FakeCredentialsProvider::new()),
            auth_generation: 0,
            last_auth_error: None,
        });

        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let weak = weak_state.clone();
        let http_clone = http.clone();
        let result = cx
            .spawn(async move |mut cx| get_fresh_credentials(&weak, &http_clone, &mut cx).await)
            .await;

        cx.run_until_parked();

        assert!(result.is_err(), "transient refresh should return an error");
        cx.read(|cx| {
            let s = state.read(cx);
            assert!(
                s.credentials.is_some(),
                "credentials should be kept on transient refresh failure"
            );
            assert!(
                s.last_auth_error.is_none(),
                "last_auth_error should not be set on transient refresh failure"
            );
        });
    }

    #[gpui::test]
    async fn test_sign_out_during_refresh_discards_result(cx: &mut TestAppContext) {
        let (gate_tx, gate_rx) = futures::channel::oneshot::channel::<()>();
        let gate_rx = Arc::new(Mutex::new(Some(gate_rx)));
        let gate_rx_clone = gate_rx.clone();

        let http_client = FakeHttpClient::create(move |_request| {
            let gate_rx = gate_rx_clone.clone();
            async move {
                let rx = gate_rx.lock().take();
                if let Some(rx) = rx {
                    let _ = rx.await;
                }
                let body = fake_token_response();
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(body))?)
            }
        });

        let creds_provider = Arc::new(FakeCredentialsProvider::new());
        let state = cx.new(|_cx| State {
            credentials: Some(make_expired_credentials()),
            sign_in_task: None,
            refresh_task: None,
            load_task: None,
            credentials_provider: creds_provider.clone(),
            auth_generation: 0,
            last_auth_error: None,
        });

        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let weak = weak_state.clone();
        let http_clone = http.clone();
        let refresh_task =
            cx.spawn(async move |mut cx| get_fresh_credentials(&weak, &http_clone, &mut cx).await);

        cx.run_until_parked();

        cx.update(|cx| {
            do_sign_out(&weak_state, cx).detach();
        });
        cx.run_until_parked();

        let _ = gate_tx.send(());
        cx.run_until_parked();

        let result = refresh_task.await;
        assert!(result.is_err(), "refresh should fail after sign-out");

        cx.read(|cx| {
            let s = state.read(cx);
            assert!(
                s.credentials.is_none(),
                "sign-out should have cleared credentials"
            );
        });
    }

    #[gpui::test]
    async fn test_sign_out_completes_fully(cx: &mut TestAppContext) {
        let creds_provider = Arc::new(FakeCredentialsProvider::new());
        creds_provider
            .storage
            .lock()
            .replace(("Bearer".to_string(), b"some-creds".to_vec()));

        let state = cx.new(|_cx| State {
            credentials: Some(make_fresh_credentials()),
            sign_in_task: None,
            refresh_task: None,
            load_task: None,
            credentials_provider: creds_provider.clone(),
            auth_generation: 0,
            last_auth_error: None,
        });

        let weak_state = cx.read(|_cx| state.downgrade());
        let sign_out_task = cx.update(|cx| do_sign_out(&weak_state, cx));

        cx.run_until_parked();
        sign_out_task.await.expect("sign-out should succeed");

        assert!(
            creds_provider.storage.lock().is_none(),
            "credential store should be empty after sign-out"
        );
        cx.read(|cx| {
            assert!(
                !state.read(cx).is_authenticated(),
                "state should show not authenticated"
            );
        });
    }

    #[gpui::test]
    async fn test_authenticate_awaits_initial_load(cx: &mut TestAppContext) {
        let creds = make_fresh_credentials();
        let creds_json = serde_json::to_vec(&creds).unwrap();
        let creds_provider = Arc::new(FakeCredentialsProvider::new());
        creds_provider
            .storage
            .lock()
            .replace(("Bearer".to_string(), creds_json));

        let http_client = FakeHttpClient::create(|_| async {
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::default())?)
        });

        let provider =
            cx.update(|cx| XAiOAuthLanguageModelProvider::new(http_client, creds_provider, cx));

        let auth_task = cx.update(|cx| provider.authenticate(cx));

        cx.run_until_parked();

        let result = auth_task.await;
        assert!(
            result.is_ok(),
            "authenticate should succeed after load completes with valid credentials"
        );
    }

    #[test]
    fn grok_43_supports_selectable_thinking_effort_levels() {
        let effort_levels = supported_thinking_effort_levels(&x_ai::Model::Grok43);
        let values = effort_levels
            .iter()
            .map(|level| level.value.as_ref())
            .collect::<Vec<_>>();

        assert_eq!(values, ["low", "medium", "high"]);
    }

    #[gpui::test]
    async fn test_refresh_reuses_existing_refresh_token_when_omitted(cx: &mut TestAppContext) {
        // A non-rotating auth server returns a new access token but no
        // refresh_token; the existing refresh token must be retained.
        let http_client = FakeHttpClient::create(|_| async {
            let body = serde_json::json!({
                "access_token": "new_access",
                "expires_in": 3600
            })
            .to_string();
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::from(body))?)
        });
        let http: Arc<dyn HttpClient> = http_client;

        let creds = cx
            .spawn(async move |_| refresh_token(&http, "kept_refresh").await)
            .await
            .expect("refresh should succeed when the response omits refresh_token");

        assert_eq!(creds.access_token, "new_access");
        assert_eq!(
            creds.refresh_token, "kept_refresh",
            "the existing refresh token should be reused when the response omits one"
        );
    }

    #[test]
    fn jwt_exp_claim_is_decoded_from_access_token() {
        use base64::Engine as _;
        let payload =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(br#"{"exp":1700000000}"#);
        let token = format!("header.{payload}.signature");
        assert_eq!(jwt_exp_seconds(&token), Some(1_700_000_000));
        assert_eq!(
            access_token_expires_at_ms(&token, None),
            1_700_000_000_000,
            "JWT exp must take precedence over the expires_in fallback"
        );
        // Opaque (non-JWT) tokens have no exp and fall back to expires_in.
        assert_eq!(jwt_exp_seconds("opaque-token"), None);
    }
}
