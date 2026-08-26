use anyhow::{Context as _, Result, anyhow};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, future::Shared};
use gpui::{App, AsyncApp, Context, Entity, SharedString, Task, WeakEntity};
use http_client::{AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelEffortLevel, LanguageModelId, LanguageModelName, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolSchemaFormat, ProviderErrorCategory, RateLimiter,
};
use open_ai::{ReasoningEffort, ResponseStreamEvent};
use rand::RngCore as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use url::form_urlencoded;
use util::ResultExt as _;

pub const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("x_ai_subscribed");
pub const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("SuperGrok");

pub use x_ai::XAI_API_URL;
const XAI_AUTHORIZE_URL: &str = "https://auth.x.ai/oauth2/authorize";
const XAI_TOKEN_URL: &str = "https://auth.x.ai/oauth2/token";
const CLIENT_ID: &str = "b1a00492-073a-47ea-816f-4c329264a828";
const OAUTH_SCOPE: &str = "openid profile email offline_access grok-cli:access api:access";

/// Keychain slot. Must not collide with the xAI API-key provider (`https://api.x.ai/v1`).
const CREDENTIALS_KEY: &str = "https://auth.x.ai/zed-supergrok";
const TOKEN_REFRESH_BUFFER_MS: u64 = Duration::from_secs(120).as_millis() as u64;

const CALLBACK_HOST: &str = "127.0.0.1";
const CALLBACK_PORT: u16 = 56121;
const CALLBACK_PATH: &str = "/callback";

const INFERENCE_FORBIDDEN_MESSAGE: &str = "Login succeeded, but this Grok account cannot use the API \
    (HTTP 403). Some plans do not include this access. You can also use the separate xAI provider \
    with an API key from console.x.ai.";

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SuperGrokCredentials {
    access_token: String,
    refresh_token: String,
    expires_at_ms: u64,
    email: Option<String>,
}

impl SuperGrokCredentials {
    fn is_expired(&self) -> bool {
        now_ms() + TOKEN_REFRESH_BUFFER_MS >= self.expires_at_ms
    }
}

pub struct State {
    credentials: Option<SuperGrokCredentials>,
    sign_in_task: Option<Task<Result<()>>>,
    refresh_task: Option<Shared<Task<Result<SuperGrokCredentials, Arc<anyhow::Error>>>>>,
    load_task: Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    auth_generation: u64,
    last_auth_error: Option<SharedString>,
}

#[derive(Debug)]
enum RefreshError {
    Fatal(anyhow::Error),
    Transient(anyhow::Error),
}

impl std::fmt::Display for RefreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefreshError::Fatal(error) => write!(f, "{error}"),
            RefreshError::Transient(error) => write!(f, "{error}"),
        }
    }
}

impl State {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut Context<Self>,
    ) -> Self {
        let load_task = cx
            .spawn({
                let credentials_provider = credentials_provider.clone();
                async move |this, cx| {
                    let result = credentials_provider
                        .read_credentials(CREDENTIALS_KEY, cx)
                        .await;
                    this.update(cx, |state, cx| {
                        match result {
                            Ok(Some((_, bytes))) => {
                                match serde_json::from_slice::<SuperGrokCredentials>(&bytes) {
                                    Ok(credentials) => {
                                        state.auth_generation =
                                            state.auth_generation.wrapping_add(1);
                                        state.credentials = Some(credentials);
                                    }
                                    Err(error) => {
                                        log::warn!(
                                            "Failed to deserialize SuperGrok credentials: {error}"
                                        );
                                    }
                                }
                            }
                            Ok(None) => {}
                            Err(error) => {
                                log::error!("Failed to load SuperGrok credentials: {error:#}");
                            }
                        }
                        state.load_task = None;
                        cx.notify();
                    })?;
                    Ok::<(), Arc<anyhow::Error>>(())
                }
            })
            .shared();

        Self {
            credentials: None,
            sign_in_task: None,
            refresh_task: None,
            load_task: Some(load_task),
            credentials_provider,
            http_client,
            auth_generation: 0,
            last_auth_error: None,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.credentials.is_some()
    }

    pub fn email(&self) -> Option<&str> {
        self.credentials.as_ref().and_then(|c| c.email.as_deref())
    }

    pub fn is_signing_in(&self) -> bool {
        self.sign_in_task.is_some()
    }

    pub fn last_auth_error(&self) -> Option<SharedString> {
        self.last_auth_error.clone()
    }

    pub fn load_task(&self) -> Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>> {
        self.load_task.clone()
    }

    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    pub fn sign_in(&mut self, cx: &mut Context<Self>) {
        if self.is_signing_in() {
            return;
        }

        let http_client = self.http_client.clone();
        let task = cx.spawn(async move |this, cx| {
            match do_oauth_flow(http_client, cx).await {
                Ok(creds) => {
                    let persist_result = async {
                        let credentials_provider =
                            this.read_with(cx, |state, _| state.credentials_provider.clone())?;
                        let json = serde_json::to_vec(&creds)?;
                        credentials_provider
                            .write_credentials(CREDENTIALS_KEY, "Bearer", &json, cx)
                            .await?;
                        anyhow::Ok(())
                    }
                    .await;

                    match persist_result {
                        Ok(()) => {
                            this.update(cx, |state, cx| {
                                state.auth_generation = state.auth_generation.wrapping_add(1);
                                state.credentials = Some(creds);
                                state.last_auth_error = None;
                                state.sign_in_task = None;
                                cx.notify();
                            })?;
                        }
                        Err(err) => {
                            log::error!("SuperGrok sign-in failed to persist credentials: {err:?}");
                            this.update(cx, |state, cx| {
                                state.sign_in_task = None;
                                state.last_auth_error =
                                    Some("Failed to save credentials. Please try again.".into());
                                cx.notify();
                            })
                            .log_err();
                        }
                    }
                }
                Err(err) => {
                    log::error!("SuperGrok sign-in failed: {err:?}");
                    this.update(cx, |state, cx| {
                        state.sign_in_task = None;
                        state.last_auth_error = Some("Sign-in failed. Please try again.".into());
                        cx.notify();
                    })
                    .log_err();
                }
            }
            anyhow::Ok(())
        });

        self.last_auth_error = None;
        self.sign_in_task = Some(task);
        cx.notify();
    }

    pub fn sign_out(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.auth_generation += 1;
        self.credentials = None;
        self.sign_in_task = None;
        self.refresh_task = None;
        self.last_auth_error = None;
        cx.notify();

        let credentials_provider = self.credentials_provider.clone();
        cx.spawn(async move |_this, cx| {
            credentials_provider
                .delete_credentials(CREDENTIALS_KEY, cx)
                .await
                .context("Failed to delete SuperGrok credentials from keychain")?;
            anyhow::Ok(())
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuperGrokModel {
    Grok46,
    Grok45,
    GrokBuild01,
}

impl SuperGrokModel {
    pub fn all() -> Vec<Self> {
        vec![Self::Grok46, Self::Grok45, Self::GrokBuild01]
    }

    fn x_ai_model(&self) -> Option<x_ai::Model> {
        match self {
            Self::Grok46 => Some(x_ai::Model::Grok46),
            Self::Grok45 => Some(x_ai::Model::Grok45),
            // grok-build-0.1 is SuperGrok-only; it is not in the BYOK catalog.
            Self::GrokBuild01 => None,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Grok46 => "grok-4.6",
            Self::Grok45 => "grok-4.5",
            Self::GrokBuild01 => "grok-build-0.1",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Grok46 => "Grok 4.6",
            Self::Grok45 => "Grok 4.5",
            Self::GrokBuild01 => "Grok Build 0.1",
        }
    }

    pub fn max_token_count(&self) -> u64 {
        self.x_ai_model()
            .map(|model| model.max_token_count())
            .unwrap_or(256_000)
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        self.x_ai_model()
            .map(|model| model.max_output_tokens())
            .unwrap_or(Some(64_000))
    }

    pub fn supports_images(&self) -> bool {
        self.x_ai_model()
            .map(|model| model.supports_images())
            .unwrap_or(true)
    }

    pub fn supports_tools(&self) -> bool {
        self.x_ai_model()
            .map(|model| model.supports_tool())
            .unwrap_or(true)
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        self.x_ai_model()
            .map(|model| model.supports_parallel_tool_calls())
            .unwrap_or(true)
    }

    pub fn requires_json_schema_subset(&self) -> bool {
        self.x_ai_model()
            .map(|model| model.requires_json_schema_subset())
            .unwrap_or(true)
    }

    pub fn supports_reasoning_effort(&self) -> bool {
        self.x_ai_model()
            .map(|model| model.supports_reasoning_effort())
            .unwrap_or(false)
    }
}

pub fn create_language_model(
    model: SuperGrokModel,
    state: &Entity<State>,
    cx: &App,
) -> Arc<dyn LanguageModel> {
    Arc::new(SuperGrokLanguageModel {
        id: LanguageModelId::from(model.id().to_string()),
        http_client: state.read(cx).http_client(),
        model,
        state: state.clone(),
        api_url: XAI_API_URL.into(),
        extra_headers: CustomHeaders::default(),
        request_limiter: RateLimiter::new(4),
    })
}

struct SuperGrokLanguageModel {
    id: LanguageModelId,
    model: SuperGrokModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    api_url: SharedString,
    extra_headers: CustomHeaders,
    request_limiter: RateLimiter,
}

fn advertised_reasoning_efforts(model: &SuperGrokModel) -> &'static [ReasoningEffort] {
    // xAI rejects `reasoning_effort: "none"` on grok-4.5/4.6. Compact and title
    // requests disable thinking, so we omit the field instead of sending none.
    match model.x_ai_model() {
        Some(x_ai::Model::Grok45) => &[
            ReasoningEffort::Low,
            ReasoningEffort::Medium,
            ReasoningEffort::High,
        ],
        Some(x_ai::Model::Grok46) => &[
            ReasoningEffort::Low,
            ReasoningEffort::Medium,
            ReasoningEffort::High,
            ReasoningEffort::XHigh,
        ],
        _ => &[],
    }
}

fn default_thinking_reasoning_effort(model: &SuperGrokModel) -> Option<ReasoningEffort> {
    match model.x_ai_model() {
        Some(x_ai::Model::Grok45 | x_ai::Model::Grok46) => Some(ReasoningEffort::High),
        _ => None,
    }
}

fn reasoning_effort_for_request(
    request: &LanguageModelRequest,
    model: &SuperGrokModel,
) -> Option<ReasoningEffort> {
    let supported_efforts = advertised_reasoning_efforts(model);
    if supported_efforts.is_empty() {
        return None;
    }

    if request.thinking_allowed {
        request
            .thinking_effort
            .as_deref()
            .and_then(|effort| effort.parse::<ReasoningEffort>().ok())
            .filter(|effort| supported_efforts.contains(effort))
            .filter(|effort| *effort != ReasoningEffort::None)
            .or_else(|| default_thinking_reasoning_effort(model))
    } else {
        None
    }
}

fn supported_thinking_effort_levels(model: &SuperGrokModel) -> Vec<LanguageModelEffortLevel> {
    let default_effort = default_thinking_reasoning_effort(model);
    advertised_reasoning_efforts(model)
        .iter()
        .copied()
        .filter_map(|effort| {
            let (name, value) = match effort {
                ReasoningEffort::None => return None,
                ReasoningEffort::Minimal => ("Minimal", "minimal"),
                ReasoningEffort::Low => ("Low", "low"),
                ReasoningEffort::Medium => ("Medium", "medium"),
                ReasoningEffort::High => ("High", "high"),
                ReasoningEffort::XHigh => ("Extra High", "xhigh"),
                ReasoningEffort::Max => return None,
            };

            Some(LanguageModelEffortLevel {
                name: name.into(),
                value: value.into(),
                is_default: Some(effort) == default_effort,
            })
        })
        .collect()
}

fn map_completion_error(error: LanguageModelCompletionError) -> LanguageModelCompletionError {
    match error {
        LanguageModelCompletionError::ProviderRejection {
            provider,
            status,
            code,
            retry_after,
            category: ProviderErrorCategory::Permission,
            ..
        } if provider == PROVIDER_NAME => LanguageModelCompletionError::ProviderRejection {
            provider,
            status,
            code,
            message: INFERENCE_FORBIDDEN_MESSAGE.to_string(),
            retry_after,
            category: ProviderErrorCategory::Permission,
        },
        other => other,
    }
}

impl SuperGrokLanguageModel {
    fn stream_open_ai_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();
        let api_url = self.api_url.clone();
        let extra_headers = self.extra_headers.clone();
        let state = self.state.downgrade();
        let request_limiter = self.request_limiter.clone();

        let future = cx.spawn(async move |cx| {
            let credentials = get_fresh_credentials(&state, &http_client, cx).await?;
            let access_token = credentials.access_token.clone();
            request_limiter
                .stream(async move {
                    open_ai::stream_completion(
                        http_client.as_ref(),
                        PROVIDER_NAME.0.as_str(),
                        api_url.as_ref(),
                        &access_token,
                        request,
                        &extra_headers,
                    )
                    .await
                    .map_err(|error| {
                        map_completion_error(LanguageModelCompletionError::from(error))
                    })
                })
                .await
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for SuperGrokLanguageModel {
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
        format!("x_ai_subscribed/{}", self.model.id())
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
        let request = match open_ai::completion::into_open_ai(
            request,
            self.model.id(),
            self.model.supports_parallel_tool_calls(),
            false,
            self.max_output_tokens(),
            open_ai::completion::ChatCompletionMaxTokensParameter::MaxCompletionTokens,
            reasoning_effort,
            false,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let completions = self.stream_open_ai_completion(request, cx);
        async move {
            let mapper = open_ai::completion::OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

async fn get_fresh_credentials(
    state: &WeakEntity<State>,
    http_client: &Arc<dyn HttpClient>,
    cx: &mut AsyncApp,
) -> Result<SuperGrokCredentials, LanguageModelCompletionError> {
    let (creds, existing_task) = state
        .read_with(&*cx, |s, _| (s.credentials.clone(), s.refresh_task.clone()))
        .map_err(LanguageModelCompletionError::Other)?;

    let creds = creds.ok_or(LanguageModelCompletionError::NoApiKey {
        provider: PROVIDER_NAME,
    })?;

    if !creds.is_expired() {
        return Ok(creds);
    }

    if let Some(shared_task) = existing_task {
        return shared_task
            .await
            .map_err(|e| LanguageModelCompletionError::Other(anyhow!("{e}")));
    }

    let http_client_clone = http_client.clone();
    let state_clone = state.clone();
    let previous_refresh_token = creds.refresh_token.clone();
    let previous_email = creds.email.clone();

    let generation = state
        .read_with(&*cx, |s, _| s.auth_generation)
        .map_err(LanguageModelCompletionError::Other)?;

    let shared_task = cx
        .spawn(async move |cx| {
            let result = refresh_token(&http_client_clone, &previous_refresh_token).await;

            match result {
                Ok(tokens) => {
                    let persist_result: Result<SuperGrokCredentials, Arc<anyhow::Error>> = async {
                        let current_generation = state_clone
                            .read_with(&*cx, |s, _| s.auth_generation)
                            .map_err(|e| Arc::new(e))?;
                        if current_generation != generation {
                            return Err(Arc::new(anyhow!(
                                "Sign-out occurred during token refresh"
                            )));
                        }

                        let claims = tokens
                            .id_token
                            .as_deref()
                            .map(extract_email_claim)
                            .unwrap_or(None);
                        let refreshed = SuperGrokCredentials {
                            access_token: tokens.access_token,
                            refresh_token: tokens
                                .refresh_token
                                .unwrap_or(previous_refresh_token.clone()),
                            expires_at_ms: now_ms() + tokens.expires_in * 1000,
                            email: claims.or(tokens.email).or(previous_email.clone()),
                        };

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

                    if persist_result.is_err() {
                        state_clone
                            .update(cx, |s, _| {
                                s.refresh_task = None;
                            })
                            .ok();
                    }

                    persist_result
                }
                Err(RefreshError::Fatal(e)) => {
                    log::error!("SuperGrok token refresh failed fatally: {e:?}");
                    let still_current_generation = state_clone
                        .read_with(&*cx, |s, _| s.auth_generation == generation)
                        .unwrap_or(false);
                    if still_current_generation {
                        state_clone
                            .update(cx, |s, cx| {
                                s.refresh_task = None;
                                s.credentials = None;
                                s.last_auth_error = Some(
                                    "Your SuperGrok session has expired. Sign in again.".into(),
                                );
                                cx.notify();
                            })
                            .ok();
                        if let Ok(credentials_provider) =
                            state_clone.read_with(&*cx, |s, _| s.credentials_provider.clone())
                        {
                            credentials_provider
                                .delete_credentials(CREDENTIALS_KEY, &*cx)
                                .await
                                .log_err();
                        }
                    } else {
                        state_clone
                            .update(cx, |s, _| {
                                s.refresh_task = None;
                            })
                            .ok();
                    }
                    Err(Arc::new(e))
                }
                Err(RefreshError::Transient(e)) => {
                    log::warn!("SuperGrok token refresh failed transiently: {e:?}");
                    state_clone
                        .update(cx, |s, _| {
                            s.refresh_task = None;
                        })
                        .ok();
                    Err(Arc::new(e))
                }
            }
        })
        .shared();

    state
        .update(cx, |s, _| {
            s.refresh_task = Some(shared_task.clone());
        })
        .map_err(LanguageModelCompletionError::Other)?;

    shared_task
        .await
        .map_err(|e| LanguageModelCompletionError::Other(anyhow!("{e}")))
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    id_token: Option<String>,
    expires_in: u64,
    #[serde(default)]
    email: Option<String>,
}

struct PkceAuthorizeRequest {
    authorize_url: String,
    verifier: String,
    state: String,
    redirect_uri: String,
}

fn pkce_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize().as_slice())
}

fn build_authorize_request(
    redirect_uri: &str,
    verifier: &str,
    oauth_state: &str,
) -> Result<String> {
    let challenge = pkce_challenge(verifier);
    let mut auth_url = url::Url::parse(XAI_AUTHORIZE_URL).context("invalid authorize URL")?;
    auth_url
        .query_pairs_mut()
        .append_pair("client_id", CLIENT_ID)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("scope", OAUTH_SCOPE)
        .append_pair("response_type", "code")
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("state", oauth_state)
        .append_pair("nonce", oauth_state);
    Ok(auth_url.to_string())
}

fn new_pkce_authorize_request(redirect_uri: String) -> Result<PkceAuthorizeRequest> {
    let mut verifier_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut verifier_bytes);
    let verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);

    let mut state_bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut state_bytes);
    let oauth_state: String = state_bytes.iter().map(|b| format!("{b:02x}")).collect();

    let authorize_url = build_authorize_request(&redirect_uri, &verifier, &oauth_state)?;
    Ok(PkceAuthorizeRequest {
        authorize_url,
        verifier,
        state: oauth_state,
        redirect_uri,
    })
}

async fn do_oauth_flow(
    http_client: Arc<dyn HttpClient>,
    cx: &AsyncApp,
) -> Result<SuperGrokCredentials> {
    let (redirect_uri, callback_rx) =
        oauth_callback_server::start_oauth_callback_server_with_config(
            oauth_callback_server::OAuthCallbackServerConfig {
                host: CALLBACK_HOST,
                preferred_port: CALLBACK_PORT,
                fallback_port: None,
                path: CALLBACK_PATH,
            },
        )
        .context("Failed to start OAuth callback server")?;

    let pkce = new_pkce_authorize_request(redirect_uri)?;
    cx.update(|cx| cx.open_url(&pkce.authorize_url));

    let callback = callback_rx
        .await
        .map_err(|_| anyhow!("OAuth callback was cancelled"))?
        .context("OAuth callback failed")?;

    if callback.state != pkce.state {
        return Err(anyhow!("OAuth state mismatch"));
    }

    let tokens = exchange_code(
        &http_client,
        &callback.code,
        &pkce.verifier,
        &pkce.redirect_uri,
    )
    .await
    .context("Token exchange failed")?;

    let refresh_token = tokens
        .refresh_token
        .filter(|token| !token.is_empty())
        .context("Token response did not include a refresh_token")?;
    let email = tokens
        .id_token
        .as_deref()
        .and_then(extract_email_claim)
        .or(tokens.email);

    Ok(SuperGrokCredentials {
        access_token: tokens.access_token,
        refresh_token,
        expires_at_ms: now_ms() + tokens.expires_in * 1000,
        email,
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
        .append_pair("client_id", CLIENT_ID)
        .append_pair("code", code)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("code_verifier", verifier)
        .finish();

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(XAI_TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(AsyncBody::from(body))?;

    let mut response = client.send(request).await?;
    let mut body = String::new();
    smol::io::AsyncReadExt::read_to_string(response.body_mut(), &mut body).await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Token exchange failed (HTTP {}): {}",
            response.status(),
            redact_token_body(&body)
        ));
    }

    serde_json::from_str::<TokenResponse>(&body).context("Failed to parse token response")
}

async fn refresh_token(
    client: &Arc<dyn HttpClient>,
    refresh_token: &str,
) -> Result<TokenResponse, RefreshError> {
    let body = form_urlencoded::Serializer::new(String::new())
        .append_pair("grant_type", "refresh_token")
        .append_pair("client_id", CLIENT_ID)
        .append_pair("refresh_token", refresh_token)
        .finish();

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(XAI_TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(AsyncBody::from(body))
        .map_err(|e| RefreshError::Transient(e.into()))?;

    let mut response = client
        .send(request)
        .await
        .map_err(RefreshError::Transient)?;
    let status = response.status();
    let mut body = String::new();
    smol::io::AsyncReadExt::read_to_string(response.body_mut(), &mut body)
        .await
        .map_err(|e| RefreshError::Transient(e.into()))?;

    if !status.is_success() {
        let err = anyhow!(
            "Token refresh failed (HTTP {}): {}",
            status,
            redact_token_body(&body)
        );
        if status == http_client::StatusCode::BAD_REQUEST
            || status == http_client::StatusCode::UNAUTHORIZED
            || status == http_client::StatusCode::FORBIDDEN
        {
            return Err(RefreshError::Fatal(err));
        }
        return Err(RefreshError::Transient(err));
    }

    serde_json::from_str(&body).map_err(|e| RefreshError::Transient(e.into()))
}

fn extract_email_claim(jwt: &str) -> Option<String> {
    let payload_b64 = jwt.split('.').nth(1)?;
    let payload = URL_SAFE_NO_PAD.decode(payload_b64).ok()?;
    let claims = serde_json::from_slice::<serde_json::Value>(&payload).ok()?;
    claims
        .get("email")
        .and_then(|value| value.as_str())
        .map(str::to_owned)
}

fn redact_token_body(body: &str) -> String {
    const MAX_LEN: usize = 240;
    if body.len() <= MAX_LEN {
        body.to_string()
    } else {
        format!("{}…", &body[..MAX_LEN])
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext as _, TestAppContext};
    use http_client::FakeHttpClient;
    use parking_lot::Mutex;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn grok_46_supports_xhigh_and_defaults_to_high() {
        let effort_levels = supported_thinking_effort_levels(&SuperGrokModel::Grok46);
        let values = effort_levels
            .iter()
            .map(|level| level.value.as_ref())
            .collect::<Vec<_>>();

        assert_eq!(values, ["low", "medium", "high", "xhigh"]);
        assert_eq!(
            effort_levels
                .iter()
                .find(|level| level.is_default)
                .map(|level| level.value.as_ref()),
            Some("high")
        );
    }

    #[test]
    fn grok_46_request_uses_selected_reasoning_effort() {
        let request = LanguageModelRequest {
            thinking_allowed: true,
            thinking_effort: Some("xhigh".to_string()),
            ..Default::default()
        };

        assert_eq!(
            reasoning_effort_for_request(&request, &SuperGrokModel::Grok46),
            Some(ReasoningEffort::XHigh)
        );
    }

    #[test]
    fn grok_46_omits_reasoning_effort_when_thinking_is_disabled() {
        let request = LanguageModelRequest {
            thinking_allowed: false,
            thinking_effort: Some("medium".to_string()),
            ..Default::default()
        };

        assert_eq!(
            reasoning_effort_for_request(&request, &SuperGrokModel::Grok46),
            None
        );
    }

    #[test]
    fn grok_build_omits_reasoning_effort() {
        let request = LanguageModelRequest {
            thinking_allowed: true,
            thinking_effort: Some("medium".to_string()),
            ..Default::default()
        };

        assert_eq!(
            reasoning_effort_for_request(&request, &SuperGrokModel::GrokBuild01),
            None
        );
    }

    #[test]
    fn grok_45_does_not_advertise_xhigh() {
        let effort_levels = supported_thinking_effort_levels(&SuperGrokModel::Grok45);
        let values = effort_levels
            .iter()
            .map(|level| level.value.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(values, ["low", "medium", "high"]);
        assert_eq!(
            effort_levels
                .iter()
                .find(|level| level.is_default)
                .map(|level| level.value.as_ref()),
            Some("high")
        );
    }

    #[test]
    fn grok_46_and_45_omit_max_output_tokens() {
        assert_eq!(SuperGrokModel::Grok46.max_output_tokens(), None);
        assert_eq!(SuperGrokModel::Grok45.max_output_tokens(), None);
        assert_eq!(
            SuperGrokModel::GrokBuild01.max_output_tokens(),
            Some(64_000)
        );
    }

    #[test]
    fn authorize_url_uses_public_client_and_s256() {
        let verifier = "test-verifier";
        let url = build_authorize_request("http://127.0.0.1:56121/callback", verifier, "abc123")
            .expect("url should build");
        let parsed = url::Url::parse(&url).unwrap();
        let pairs: Vec<(String, String)> = parsed
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        assert_eq!(parsed.as_str().split('?').next(), Some(XAI_AUTHORIZE_URL));
        assert!(pairs.contains(&("client_id".into(), CLIENT_ID.into())));
        assert!(pairs.contains(&(
            "redirect_uri".into(),
            "http://127.0.0.1:56121/callback".into()
        )));
        assert!(pairs.contains(&("scope".into(), OAUTH_SCOPE.into())));
        assert!(pairs.contains(&("response_type".into(), "code".into())));
        assert!(pairs.contains(&("code_challenge_method".into(), "S256".into())));
        assert!(pairs.contains(&("code_challenge".into(), pkce_challenge(verifier))));
        assert!(pairs.contains(&("state".into(), "abc123".into())));
        assert!(pairs.contains(&("nonce".into(), "abc123".into())));
    }

    #[gpui::test]
    async fn test_concurrent_refresh_deduplicates(cx: &mut TestAppContext) {
        let refresh_count = Arc::new(AtomicUsize::new(0));
        let refresh_count_clone = refresh_count.clone();

        let http_client = FakeHttpClient::create(move |_request| {
            let refresh_count = refresh_count_clone.clone();
            async move {
                refresh_count.fetch_add(1, Ordering::SeqCst);
                let body = fake_token_response(true);
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(body))?)
            }
        });

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http.clone(), Some(make_expired_credentials()), cx);
        let weak_state = cx.read(|_cx| state.downgrade());

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
                let body = fake_token_response(true);
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(body))?)
            }
        });

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http.clone(), Some(make_fresh_credentials()), cx);
        let weak_state = cx.read(|_cx| state.downgrade());
        let http_clone = http.clone();
        let result = cx
            .spawn(async move |mut cx| {
                get_fresh_credentials(&weak_state, &http_clone, &mut cx).await
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().access_token, "fresh_access");
        assert_eq!(refresh_count.load(Ordering::SeqCst), 0);
    }

    #[gpui::test]
    async fn test_no_credentials_returns_no_api_key(cx: &mut TestAppContext) {
        let http: Arc<dyn HttpClient> = FakeHttpClient::create(|_| async {
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::default())?)
        });
        let state = make_state(http.clone(), None, cx);
        let weak_state = cx.read(|_cx| state.downgrade());
        let result = cx
            .spawn(async move |mut cx| get_fresh_credentials(&weak_state, &http, &mut cx).await)
            .await;

        assert!(matches!(
            result,
            Err(LanguageModelCompletionError::NoApiKey { .. })
        ));
    }

    #[gpui::test]
    async fn test_fatal_refresh_clears_auth_state(cx: &mut TestAppContext) {
        let http: Arc<dyn HttpClient> = FakeHttpClient::create(move |_request| async move {
            Ok(http_client::Response::builder()
                .status(401)
                .body(http_client::AsyncBody::from(r#"{"error":"invalid_grant"}"#))?)
        });
        let state = make_state(http.clone(), Some(make_expired_credentials()), cx);
        let weak_state = cx.read(|_cx| state.downgrade());
        let result = cx
            .spawn(async move |mut cx| get_fresh_credentials(&weak_state, &http, &mut cx).await)
            .await;

        cx.run_until_parked();
        assert!(result.is_err());
        cx.read(|cx| {
            let state = state.read(cx);
            assert!(state.credentials.is_none());
            assert!(state.last_auth_error.is_some());
        });
    }

    #[gpui::test]
    async fn test_transient_refresh_keeps_credentials(cx: &mut TestAppContext) {
        let http: Arc<dyn HttpClient> = FakeHttpClient::create(move |_request| async move {
            Ok(http_client::Response::builder()
                .status(500)
                .body(http_client::AsyncBody::from("Internal Server Error"))?)
        });
        let state = make_state(http.clone(), Some(make_expired_credentials()), cx);
        let weak_state = cx.read(|_cx| state.downgrade());
        let result = cx
            .spawn(async move |mut cx| get_fresh_credentials(&weak_state, &http, &mut cx).await)
            .await;

        cx.run_until_parked();
        assert!(result.is_err());
        cx.read(|cx| {
            let state = state.read(cx);
            assert!(state.credentials.is_some());
            assert!(state.last_auth_error.is_none());
        });
    }

    #[gpui::test]
    async fn test_refresh_keeps_previous_refresh_token_when_omitted(cx: &mut TestAppContext) {
        let http: Arc<dyn HttpClient> = FakeHttpClient::create(move |_request| async move {
            let body = fake_token_response(false);
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::from(body))?)
        });
        let state = make_state(http.clone(), Some(make_expired_credentials()), cx);
        let weak_state = cx.read(|_cx| state.downgrade());
        let result = cx
            .spawn(async move |mut cx| get_fresh_credentials(&weak_state, &http, &mut cx).await)
            .await
            .expect("refresh should succeed");

        assert_eq!(result.access_token, "fresh_access");
        assert_eq!(result.refresh_token, "old_refresh");
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
                let body = fake_token_response(true);
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(body))?)
            }
        });

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http.clone(), Some(make_expired_credentials()), cx);
        let weak_state = cx.read(|_cx| state.downgrade());
        let refresh_task =
            cx.spawn(async move |mut cx| get_fresh_credentials(&weak_state, &http, &mut cx).await);

        cx.run_until_parked();
        state.update(cx, |state, cx| {
            state.sign_out(cx).detach();
        });
        cx.run_until_parked();
        let _ = gate_tx.send(());
        cx.run_until_parked();

        assert!(refresh_task.await.is_err());
        cx.read(|cx| {
            assert!(state.read(cx).credentials.is_none());
        });
    }

    #[gpui::test]
    async fn test_fatal_refresh_after_sign_out_keeps_new_session(cx: &mut TestAppContext) {
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
                Ok(http_client::Response::builder()
                    .status(401)
                    .body(http_client::AsyncBody::from(r#"{"error":"invalid_grant"}"#))?)
            }
        });

        let creds_provider = Arc::new(FakeCredentialsProvider::new());
        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state_with_credentials_provider(
            http.clone(),
            Some(make_expired_credentials()),
            creds_provider.clone(),
            cx,
        );
        let weak_state = cx.read(|_cx| state.downgrade());
        let refresh_task =
            cx.spawn(async move |mut cx| get_fresh_credentials(&weak_state, &http, &mut cx).await);

        cx.run_until_parked();
        state.update(cx, |state, cx| {
            state.sign_out(cx).detach();
        });
        cx.run_until_parked();

        let new_creds = make_fresh_credentials();
        let new_creds_json = serde_json::to_vec(&new_creds).unwrap();
        creds_provider
            .storage
            .lock()
            .replace(("Bearer".to_string(), new_creds_json));
        state.update(cx, |state, cx| {
            state.auth_generation = state.auth_generation.wrapping_add(1);
            state.credentials = Some(new_creds);
            cx.notify();
        });

        let _ = gate_tx.send(());
        cx.run_until_parked();

        assert!(refresh_task.await.is_err());
        cx.read(|cx| {
            let state = state.read(cx);
            assert!(state.is_authenticated());
            assert_eq!(
                state.credentials.as_ref().unwrap().access_token,
                "fresh_access"
            );
            assert!(state.last_auth_error.is_none());
        });
        assert!(creds_provider.storage.lock().is_some());
    }

    #[gpui::test]
    async fn test_sign_out_completes_fully(cx: &mut TestAppContext) {
        let creds_provider = Arc::new(FakeCredentialsProvider::new());
        creds_provider
            .storage
            .lock()
            .replace(("Bearer".to_string(), b"some-creds".to_vec()));

        let http: Arc<dyn HttpClient> = FakeHttpClient::create(|_| async {
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::default())?)
        });
        let state = make_state_with_credentials_provider(
            http,
            Some(make_fresh_credentials()),
            creds_provider.clone(),
            cx,
        );

        let sign_out_task = state.update(cx, |state, cx| state.sign_out(cx));
        cx.run_until_parked();
        sign_out_task.await.expect("sign-out should succeed");

        assert!(creds_provider.storage.lock().is_none());
        cx.read(|cx| {
            assert!(!state.read(cx).is_authenticated());
        });
    }

    #[gpui::test]
    async fn test_initial_load_restores_persisted_credentials(cx: &mut TestAppContext) {
        let creds = make_fresh_credentials();
        let creds_json = serde_json::to_vec(&creds).unwrap();
        let creds_provider = Arc::new(FakeCredentialsProvider::new());
        creds_provider
            .storage
            .lock()
            .replace(("Bearer".to_string(), creds_json));

        let http: Arc<dyn HttpClient> = FakeHttpClient::create(|_| async {
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::default())?)
        });
        let state = cx.new(|cx| State::new(http, creds_provider, cx));
        let load_task = cx
            .read(|cx| state.read(cx).load_task())
            .expect("constructor should start the credentials load");

        cx.run_until_parked();
        load_task.await.expect("load should succeed");

        cx.read(|cx| {
            let state = state.read(cx);
            assert!(state.is_authenticated());
            assert!(state.load_task().is_none());
        });
    }

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

    fn make_state(
        http_client: Arc<dyn HttpClient>,
        credentials: Option<SuperGrokCredentials>,
        cx: &mut TestAppContext,
    ) -> Entity<State> {
        make_state_with_credentials_provider(
            http_client,
            credentials,
            Arc::new(FakeCredentialsProvider::new()),
            cx,
        )
    }

    fn make_state_with_credentials_provider(
        http_client: Arc<dyn HttpClient>,
        credentials: Option<SuperGrokCredentials>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut TestAppContext,
    ) -> Entity<State> {
        cx.new(|_cx| State {
            credentials,
            sign_in_task: None,
            refresh_task: None,
            load_task: None,
            credentials_provider,
            http_client,
            auth_generation: 0,
            last_auth_error: None,
        })
    }

    fn make_expired_credentials() -> SuperGrokCredentials {
        SuperGrokCredentials {
            access_token: "old_access".to_string(),
            refresh_token: "old_refresh".to_string(),
            expires_at_ms: 0,
            email: None,
        }
    }

    fn make_fresh_credentials() -> SuperGrokCredentials {
        SuperGrokCredentials {
            access_token: "fresh_access".to_string(),
            refresh_token: "fresh_refresh".to_string(),
            expires_at_ms: now_ms() + 3_600_000,
            email: None,
        }
    }

    fn fake_token_response(include_refresh_token: bool) -> String {
        let mut value = serde_json::json!({
            "access_token": "fresh_access",
            "expires_in": 3600
        });
        if include_refresh_token {
            value["refresh_token"] = serde_json::json!("fresh_refresh");
        }
        value.to_string()
    }
}
