use anyhow::{Context as _, Result, anyhow};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, future::Shared};
use gpui::{App, AsyncApp, Context, Entity, SharedString, Task, WeakEntity};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt as _,
    http::{HeaderName, HeaderValue},
};
use language_model::{
    CompactionResult, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelEffortLevel, LanguageModelId, LanguageModelName, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelRequest, LanguageModelToolChoice, RateLimiter,
};
use open_ai::{
    ReasoningEffort,
    responses::{ResponseInputItem, stream_response},
};
use rand::RngCore as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use url::form_urlencoded;
use util::ResultExt as _;

use open_ai::completion::{OpenAiResponseEventMapper, into_open_ai_response};

pub const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("openai-subscribed");
pub const PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("ChatGPT Subscription");

const CODEX_BASE_URL: &str = "https://chatgpt.com/backend-api/codex";
const OPENAI_TOKEN_URL: &str = "https://auth.openai.com/oauth/token";
const OPENAI_AUTHORIZE_URL: &str = "https://auth.openai.com/oauth/authorize";
const CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";

const CREDENTIALS_KEY: &str = "https://chatgpt.com/backend-api/codex";
const TOKEN_REFRESH_BUFFER_MS: u64 = 5 * 60 * 1000;
/// Requests the complete account catalog without Codex CLI version filtering.
///
/// The backend treats this exact version as an ungated sentinel. Other versions
/// are compared with each model's `minimal_client_version`.
const UNGATED_MODEL_CATALOG_CLIENT_VERSION: &str = "0.0.0";
// Codex applies the same bound because model discovery is a startup-critical request.
const MODEL_CATALOG_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CodexCredentials {
    access_token: String,
    refresh_token: String,
    expires_at_ms: u64,
    account_id: Option<String>,
    email: Option<String>,
}

impl CodexCredentials {
    fn is_expired(&self) -> bool {
        let now = now_ms();
        now + TOKEN_REFRESH_BUFFER_MS >= self.expires_at_ms
    }
}

pub struct State {
    credentials: Option<CodexCredentials>,
    sign_in_task: Option<Task<Result<()>>>,
    refresh_task: Option<Shared<Task<Result<CodexCredentials, Arc<anyhow::Error>>>>>,
    load_task: Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    client_version: SharedString,
    available_models: Vec<ChatGptModel>,
    auth_generation: u64,
    model_catalog_generation: u64,
    last_auth_error: Option<SharedString>,
    last_model_catalog_error: Option<SharedString>,
}

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
    /// Creates state and starts loading persisted credentials.
    ///
    /// Model discovery requests the ungated account catalog because host
    /// application versions are unrelated to Codex CLI compatibility versions.
    ///
    /// [`State::load_task`] resolves once the load finishes.
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
                    let refresh_models_task = this.update(cx, |state, cx| {
                        match result {
                            Ok(Some((_, bytes))) => {
                                match serde_json::from_slice::<CodexCredentials>(&bytes) {
                                    Ok(credentials) => {
                                        state.auth_generation =
                                            state.auth_generation.wrapping_add(1);
                                        state.credentials = Some(credentials);
                                    }
                                    Err(error) => {
                                        log::warn!(
                                            "Failed to deserialize ChatGPT subscription credentials: {error}"
                                        );
                                    }
                                }
                            }
                            Ok(None) => {}
                            Err(error) => {
                                log::error!(
                                    "Failed to load ChatGPT subscription credentials: {error:#}"
                                );
                            }
                        }
                        state
                            .is_authenticated()
                            .then(|| state.refresh_model_catalog(cx))
                    })?;
                    if let Some(refresh_models_task) = refresh_models_task
                        && let Err(error) = refresh_models_task.await
                    {
                        log::warn!("Failed to refresh ChatGPT models: {error:#}");
                    }
                    this.update(cx, |state, cx| {
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
            client_version: UNGATED_MODEL_CATALOG_CLIENT_VERSION.into(),
            available_models: ChatGptModel::all(),
            auth_generation: 0,
            model_catalog_generation: 0,
            last_auth_error: None,
            last_model_catalog_error: None,
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

    pub fn model_catalog_error(&self) -> Option<SharedString> {
        self.last_model_catalog_error.clone()
    }

    pub fn available_models(&self) -> &[ChatGptModel] {
        &self.available_models
    }

    pub fn default_model(&self) -> Option<ChatGptModel> {
        self.available_models.first().cloned()
    }

    pub fn default_fast_model(&self) -> Option<ChatGptModel> {
        self.available_models
            .iter()
            .find(|model| model.id() == "gpt-5.6-luna")
            .cloned()
    }

    /// The in-flight task loading persisted credentials, or `None` once the
    /// initial load has finished.
    pub fn load_task(&self) -> Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>> {
        self.load_task.clone()
    }

    fn refresh_model_catalog(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<Result<(), Arc<anyhow::Error>>> {
        self.model_catalog_generation = self.model_catalog_generation.wrapping_add(1);
        let model_catalog_generation = self.model_catalog_generation;
        let http_client = self.http_client.clone();
        let client_version = self.client_version.clone();

        cx.spawn(async move |this, cx| {
            let result = async {
                let credentials = get_fresh_credentials(&this, &http_client, cx)
                    .await
                    .map_err(|error| anyhow!("{error}"))?;
                let request =
                    list_models(http_client.as_ref(), &credentials, client_version.as_ref());
                let timeout = cx
                    .background_executor()
                    .timer(MODEL_CATALOG_REQUEST_TIMEOUT);
                futures::select! {
                    result = request.fuse() => result,
                    () = timeout.fuse() => Err(anyhow!(
                        "ChatGPT models request timed out after {MODEL_CATALOG_REQUEST_TIMEOUT:?}"
                    )),
                }
            }
            .await;

            match result {
                Ok(models) => this
                    .update(cx, |state, cx| {
                        if state.model_catalog_generation == model_catalog_generation {
                            state.available_models = models;
                            state.last_model_catalog_error = None;
                            cx.notify();
                        }
                    })
                    .map_err(Arc::new),
                Err(error) => {
                    let error = Arc::new(error);
                    this.update(cx, |state, cx| {
                        if state.model_catalog_generation == model_catalog_generation {
                            state.last_model_catalog_error =
                                Some(format!("Failed to load models: {error:#}").into());
                            cx.notify();
                        }
                    })
                    .map_err(Arc::new)?;
                    Err(error)
                }
            }
        })
    }

    fn reset_model_catalog(&mut self) {
        self.model_catalog_generation = self.model_catalog_generation.wrapping_add(1);
        self.available_models = ChatGptModel::all();
        self.last_model_catalog_error = None;
    }

    /// Starts the browser-based OAuth sign-in flow. No-op while a sign-in is
    /// already in progress; observe the entity to react to the outcome.
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
                            let refresh_models_task = this.update(cx, |state, cx| {
                                state.auth_generation = state.auth_generation.wrapping_add(1);
                                state.credentials = Some(creds);
                                state.last_auth_error = None;
                                state.refresh_model_catalog(cx)
                            })?;
                            if let Err(error) = refresh_models_task.await {
                                log::warn!("Failed to refresh ChatGPT models: {error:#}");
                            }
                            this.update(cx, |state, cx| {
                                state.sign_in_task = None;
                                cx.notify();
                            })?;
                        }
                        Err(err) => {
                            log::error!(
                                "ChatGPT subscription sign-in failed to persist credentials: {err:?}"
                            );
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
                    log::error!("ChatGPT subscription sign-in failed: {err:?}");
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

    /// Clears credentials and in-flight work immediately (so observers see the
    /// sign-out right away); the returned task deletes the persisted
    /// credentials.
    pub fn sign_out(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.auth_generation += 1;
        self.credentials = None;
        self.sign_in_task = None;
        self.refresh_task = None;
        self.last_auth_error = None;
        self.reset_model_catalog();
        cx.notify();

        let credentials_provider = self.credentials_provider.clone();
        cx.spawn(async move |_this, cx| {
            credentials_provider
                .delete_credentials(CREDENTIALS_KEY, cx)
                .await
                .context("Failed to delete ChatGPT subscription credentials from keychain")?;
            anyhow::Ok(())
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChatGptModel {
    Gpt56Sol,
    Gpt56Terra,
    Gpt56Luna,
    Gpt55,
    Gpt54,
    Gpt54Mini,
    Discovered(Box<DiscoveredChatGptModel>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiscoveredChatGptModel {
    id: String,
    display_name: String,
    max_token_count: u64,
    supports_images: bool,
    default_reasoning_effort: Option<ReasoningEffort>,
    supported_reasoning_efforts: Vec<ReasoningEffort>,
    supports_priority: bool,
}

impl ChatGptModel {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Gpt56Sol,
            Self::Gpt56Terra,
            Self::Gpt56Luna,
            Self::Gpt55,
            Self::Gpt54,
            Self::Gpt54Mini,
        ]
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Gpt56Sol => "gpt-5.6-sol",
            Self::Gpt56Terra => "gpt-5.6-terra",
            Self::Gpt56Luna => "gpt-5.6-luna",
            Self::Gpt55 => "gpt-5.5",
            Self::Gpt54 => "gpt-5.4",
            Self::Gpt54Mini => "gpt-5.4-mini",
            Self::Discovered(model) => &model.id,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Gpt56Sol => "GPT-5.6 Sol",
            Self::Gpt56Terra => "GPT-5.6 Terra",
            Self::Gpt56Luna => "GPT-5.6 Luna",
            Self::Gpt55 => "GPT-5.5",
            Self::Gpt54 => "GPT-5.4",
            Self::Gpt54Mini => "GPT-5.4 Mini",
            Self::Discovered(model) => &model.display_name,
        }
    }

    fn max_token_count(&self) -> u64 {
        match self {
            Self::Gpt56Sol | Self::Gpt56Terra | Self::Gpt56Luna => 372_000,
            Self::Gpt55 | Self::Gpt54 | Self::Gpt54Mini => 272_000,
            Self::Discovered(model) => model.max_token_count,
        }
    }

    fn max_output_tokens(&self) -> Option<u64> {
        // Codex model metadata does not expose a max output token cap for these
        // models. Source: openai/codex models-manager/models.json.
        None
    }

    fn supports_images(&self) -> bool {
        match self {
            Self::Discovered(model) => model.supports_images,
            _ => true,
        }
    }

    fn default_reasoning_effort(&self) -> Option<ReasoningEffort> {
        match self {
            Self::Gpt56Sol => Some(ReasoningEffort::Low),
            Self::Gpt56Terra | Self::Gpt56Luna | Self::Gpt55 | Self::Gpt54 | Self::Gpt54Mini => {
                Some(ReasoningEffort::Medium)
            }
            Self::Discovered(model) => model.default_reasoning_effort,
        }
    }

    fn supported_reasoning_efforts(&self) -> &[ReasoningEffort] {
        match self {
            Self::Gpt56Sol | Self::Gpt56Terra | Self::Gpt56Luna => &[
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
                ReasoningEffort::Max,
            ],
            Self::Gpt55 | Self::Gpt54 | Self::Gpt54Mini => &[
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ],
            Self::Discovered(model) => &model.supported_reasoning_efforts,
        }
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    fn supports_prompt_cache_key(&self) -> bool {
        true
    }

    pub fn supports_priority(&self) -> bool {
        match self {
            Self::Gpt56Sol | Self::Gpt56Terra | Self::Gpt56Luna | Self::Gpt55 | Self::Gpt54 => true,
            Self::Gpt54Mini => false,
            Self::Discovered(model) => model.supports_priority,
        }
    }
}

/// Creates a [`LanguageModel`] for `model` that authenticates through
/// `state`'s credentials, refreshing them as needed.
pub fn create_language_model(
    model: ChatGptModel,
    state: &Entity<State>,
    cx: &App,
) -> Arc<dyn LanguageModel> {
    Arc::new(OpenAiSubscribedLanguageModel {
        id: LanguageModelId::from(model.id().to_string()),
        http_client: state.read(cx).http_client.clone(),
        model,
        state: state.clone(),
        request_limiter: RateLimiter::new(4),
    })
}

struct OpenAiSubscribedLanguageModel {
    id: LanguageModelId,
    model: ChatGptModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiSubscribedLanguageModel {
    fn codex_responses_request(
        &self,
        mut request: LanguageModelRequest,
    ) -> Result<open_ai::responses::Request> {
        if !self.model.supports_priority() {
            request.speed = None;
        }
        let mut responses_request = into_open_ai_response(
            request,
            self.model.id(),
            self.model.supports_parallel_tool_calls(),
            self.model.supports_prompt_cache_key(),
            None,
            self.model.default_reasoning_effort(),
            self.model
                .supported_reasoning_efforts()
                .contains(&ReasoningEffort::None),
            &PROVIDER_ID,
        )?;
        responses_request.store = Some(false);
        responses_request.instructions.get_or_insert_default();
        Ok(responses_request)
    }
}

fn codex_extra_headers(
    credentials: &CodexCredentials,
    routing_cache_key: Option<&str>,
) -> CustomHeaders {
    let mut header_pairs: Vec<(HeaderName, HeaderValue)> = vec![
        (
            HeaderName::from_static("originator"),
            HeaderValue::from_static("zed"),
        ),
        (
            HeaderName::from_static("openai-beta"),
            HeaderValue::from_static("responses=experimental"),
        ),
    ];
    if let Some(id) = &credentials.account_id
        && !id.is_empty()
        && let Ok(value) = HeaderValue::from_str(id)
    {
        header_pairs.push((HeaderName::from_static("chatgpt-account-id"), value));
    }
    if let Some(routing_cache_key) = routing_cache_key
        && let Ok(value) = HeaderValue::from_str(routing_cache_key)
    {
        header_pairs.push((HeaderName::from_static("session-id"), value.clone()));
        header_pairs.push((HeaderName::from_static("thread-id"), value));
    }
    CustomHeaders::new(header_pairs)
}

#[derive(Deserialize)]
struct ModelsResponse {
    models: Vec<CatalogModel>,
}

#[derive(Deserialize)]
struct CatalogModel {
    slug: String,
    display_name: String,
    default_reasoning_level: Option<String>,
    #[serde(default)]
    supported_reasoning_levels: Vec<ReasoningEffortPreset>,
    visibility: ModelVisibility,
    priority: i32,
    #[serde(default)]
    additional_speed_tiers: Vec<String>,
    #[serde(default)]
    service_tiers: Vec<ModelServiceTier>,
    context_window: Option<u64>,
    max_context_window: Option<u64>,
    input_modalities: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct ReasoningEffortPreset {
    effort: String,
}

#[derive(Deserialize)]
struct ModelServiceTier {
    id: String,
}

#[derive(Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ModelVisibility {
    List,
    Hide,
    None,
}

impl From<CatalogModel> for ChatGptModel {
    fn from(model: CatalogModel) -> Self {
        let default_reasoning_effort = model
            .default_reasoning_level
            .as_deref()
            .and_then(parse_reasoning_effort);
        let mut supported_reasoning_efforts = model
            .supported_reasoning_levels
            .iter()
            .filter_map(|preset| parse_reasoning_effort(&preset.effort))
            .collect::<Vec<_>>();
        if supported_reasoning_efforts.is_empty()
            && let Some(default_reasoning_effort) = default_reasoning_effort
        {
            supported_reasoning_efforts.push(default_reasoning_effort);
        }
        let supports_priority = model
            .additional_speed_tiers
            .iter()
            .any(|tier| tier == "fast")
            || model.service_tiers.iter().any(|tier| tier.id == "priority");
        let supports_images = model
            .input_modalities
            .as_ref()
            .is_none_or(|modalities| modalities.iter().any(|modality| modality == "image"));

        Self::Discovered(Box::new(DiscoveredChatGptModel {
            id: model.slug,
            display_name: model.display_name,
            max_token_count: model
                .context_window
                .or(model.max_context_window)
                .unwrap_or(272_000),
            supports_images,
            default_reasoning_effort,
            supported_reasoning_efforts,
            supports_priority,
        }))
    }
}

fn parse_reasoning_effort(effort: &str) -> Option<ReasoningEffort> {
    match effort {
        "none" => Some(ReasoningEffort::None),
        "minimal" => Some(ReasoningEffort::Minimal),
        "low" => Some(ReasoningEffort::Low),
        "medium" => Some(ReasoningEffort::Medium),
        "high" => Some(ReasoningEffort::High),
        "xhigh" => Some(ReasoningEffort::XHigh),
        "max" => Some(ReasoningEffort::Max),
        _ => None,
    }
}

async fn list_models(
    http_client: &dyn HttpClient,
    credentials: &CodexCredentials,
    client_version: &str,
) -> Result<Vec<ChatGptModel>> {
    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_version", client_version)
        .finish();
    let uri = format!("{CODEX_BASE_URL}/models?{query}");
    let extra_headers = codex_extra_headers(credentials, None);
    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", credentials.access_token),
        )
        .extra_headers(&extra_headers)
        .body(AsyncBody::default())
        .context("failed to build ChatGPT models request")?;
    let mut response = http_client
        .send(request)
        .await
        .context("failed to request ChatGPT models")?;
    let status = response.status();
    let mut body = String::new();
    smol::io::AsyncReadExt::read_to_string(response.body_mut(), &mut body)
        .await
        .context("failed to read ChatGPT models response")?;
    if !status.is_success() {
        return Err(anyhow!(
            "ChatGPT models request failed (HTTP {status}): {body}"
        ));
    }

    let mut models = serde_json::from_str::<ModelsResponse>(&body)
        .context("failed to parse ChatGPT models response")?
        .models;
    models.retain(|model| model.visibility == ModelVisibility::List);
    models.sort_by_key(|model| model.priority);
    if models.is_empty() {
        return Err(anyhow!(
            "ChatGPT models response did not contain any picker-visible models"
        ));
    }
    Ok(models.into_iter().map(ChatGptModel::from).collect())
}

impl LanguageModel for OpenAiSubscribedLanguageModel {
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
        true
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        true
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        true
    }

    fn supports_fast_mode(&self) -> bool {
        self.model.supports_priority()
    }

    fn supports_server_side_compaction(&self) -> bool {
        true
    }

    fn supports_explicit_compaction(&self) -> bool {
        true
    }

    fn compact(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<CompactionResult, LanguageModelCompletionError>> {
        let mut responses_request = match self.codex_responses_request(request) {
            Ok(responses_request) => responses_request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        responses_request.context_management = None;
        responses_request
            .input
            .push(ResponseInputItem::CompactionTrigger);

        let state = self.state.downgrade();
        let http_client = self.http_client.clone();
        let request_limiter = self.request_limiter.clone();

        cx.spawn(async move |cx| {
            let creds = get_fresh_credentials(&state, &http_client, cx).await?;
            let extra_headers =
                codex_extra_headers(&creds, responses_request.prompt_cache_key.as_deref());
            let access_token = creds.access_token.clone();
            let response_stream = request_limiter
                .stream(async move {
                    stream_response(
                        http_client.as_ref(),
                        PROVIDER_NAME.0.as_str(),
                        CODEX_BASE_URL,
                        &access_token,
                        responses_request,
                        &extra_headers,
                    )
                    .await
                    .map_err(LanguageModelCompletionError::from)
                })
                .await?;
            let mapper = OpenAiResponseEventMapper::new(PROVIDER_ID);
            let mut event_stream = mapper.map_stream(response_stream.boxed());
            let mut compacted_context = None;
            let mut usage = language_model::TokenUsage::default();

            while let Some(event) = event_stream.next().await {
                match event? {
                    LanguageModelCompletionEvent::Compaction(
                        language_model::CompactionUpdate::Finished(context),
                    ) => {
                        if compacted_context.replace(context).is_some() {
                            return Err(LanguageModelCompletionError::Other(anyhow!(
                                "ChatGPT subscription compaction returned multiple replacement contexts"
                            )));
                        }
                    }
                    LanguageModelCompletionEvent::UsageUpdate(updated_usage) => {
                        usage = updated_usage;
                    }
                    _ => {}
                }
            }

            let context = compacted_context.ok_or_else(|| {
                LanguageModelCompletionError::Other(anyhow!(
                    "ChatGPT subscription compaction returned no replacement context"
                ))
            })?;
            Ok(CompactionResult { context, usage })
        })
        .boxed()
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        let default_effort = self.model.default_reasoning_effort();
        self.model
            .supported_reasoning_efforts()
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
                    ReasoningEffort::Max => ("Max", "max"),
                };

                Some(LanguageModelEffortLevel {
                    name: name.into(),
                    value: value.into(),
                    is_default: Some(effort) == default_effort,
                })
            })
            .collect()
    }

    fn telemetry_id(&self) -> String {
        format!("openai-subscribed/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
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
        let responses_request = match self.codex_responses_request(request) {
            Ok(responses_request) => responses_request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };

        let state = self.state.downgrade();
        let http_client = self.http_client.clone();
        let request_limiter = self.request_limiter.clone();

        let future = cx.spawn(async move |cx| {
            let creds = get_fresh_credentials(&state, &http_client, cx).await?;
            let extra_headers =
                codex_extra_headers(&creds, responses_request.prompt_cache_key.as_deref());

            let access_token = creds.access_token.clone();
            request_limiter
                .stream(async move {
                    stream_response(
                        http_client.as_ref(),
                        PROVIDER_NAME.0.as_str(),
                        CODEX_BASE_URL,
                        &access_token,
                        responses_request,
                        &extra_headers,
                    )
                    .await
                    .map_err(LanguageModelCompletionError::from)
                })
                .await
        });

        async move {
            let mapper = OpenAiResponseEventMapper::new(PROVIDER_ID);
            Ok(mapper.map_stream(future.await?.boxed()).boxed())
        }
        .boxed()
    }
}

async fn get_fresh_credentials(
    state: &WeakEntity<State>,
    http_client: &Arc<dyn HttpClient>,
    cx: &mut AsyncApp,
) -> Result<CodexCredentials, LanguageModelCompletionError> {
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
                    let persist_result: Result<CodexCredentials, Arc<anyhow::Error>> = async {
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
                            .ok();
                    }

                    persist_result
                }
                Err(RefreshError::Fatal(e)) => {
                    log::error!("ChatGPT subscription token refresh failed fatally: {e:?}");
                    state_clone
                        .update(cx, |s, cx| {
                            s.refresh_task = None;
                            s.credentials = None;
                            s.last_auth_error =
                                Some("Your session has expired. Please sign in again.".into());
                            s.reset_model_catalog();
                            cx.notify();
                        })
                        .ok();
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
                    log::warn!("ChatGPT subscription token refresh failed transiently: {e:?}");
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
    refresh_token: String,
    #[serde(default)]
    id_token: Option<String>,
    expires_in: u64,
    #[serde(default)]
    email: Option<String>,
}

// The OAuth client registered for `CLIENT_ID` (the Codex CLI's client) only allows
// `http://localhost:1455/auth/callback` and `http://localhost:1457/auth/callback`
// as redirect URIs; using anything else (different host, port, or path) causes
// auth.openai.com to reject the authorize request with a generic `unknown_error`
// before redirecting back. Keep these in sync with the Codex CLI's redirect URI
// allow-list (see codex-rs/login/src/server.rs in openai/codex).
const CODEX_CALLBACK_HOST: &str = "localhost";
const CODEX_CALLBACK_PORT: u16 = 1455;
const CODEX_CALLBACK_FALLBACK_PORT: u16 = 1457;
const CODEX_CALLBACK_PATH: &str = "/auth/callback";

async fn do_oauth_flow(
    http_client: Arc<dyn HttpClient>,
    cx: &AsyncApp,
) -> Result<CodexCredentials> {
    // Start the callback server FIRST so the redirect URI is ready
    let (redirect_uri, callback_rx) =
        oauth_callback_server::start_oauth_callback_server_with_config(
            oauth_callback_server::OAuthCallbackServerConfig {
                host: CODEX_CALLBACK_HOST,
                preferred_port: CODEX_CALLBACK_PORT,
                fallback_port: Some(CODEX_CALLBACK_FALLBACK_PORT),
                path: CODEX_CALLBACK_PATH,
            },
        )
        .context("Failed to start OAuth callback server")?;

    // PKCE verifier: 32 random bytes → base64url (no padding)
    let mut verifier_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut verifier_bytes);
    let verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);

    // PKCE challenge: SHA-256(verifier) → base64url
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let challenge = URL_SAFE_NO_PAD.encode(hasher.finalize().as_slice());

    // CSRF state: 16 random bytes → hex string
    let mut state_bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut state_bytes);
    let oauth_state: String = state_bytes.iter().map(|b| format!("{b:02x}")).collect();

    let mut auth_url = url::Url::parse(OPENAI_AUTHORIZE_URL).expect("valid base URL");
    auth_url
        .query_pairs_mut()
        .append_pair("client_id", CLIENT_ID)
        .append_pair("redirect_uri", &redirect_uri)
        // Deliberately excludes `api.connectors.read api.connectors.invoke`
        // (which Codex CLI requests): extra scopes inflate the
        // access-token JWT, and the serialized credentials must fit within
        // Windows Credential Manager's 2560-byte blob limit
        // (CRED_MAX_CREDENTIAL_BLOB_SIZE). See #58541.
        .append_pair("scope", "openid profile email offline_access")
        .append_pair("response_type", "code")
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("id_token_add_organizations", "true")
        .append_pair("state", &oauth_state)
        .append_pair("codex_cli_simplified_flow", "true")
        .append_pair("originator", "zed");

    // Open browser AFTER the listener is ready
    cx.update(|cx| cx.open_url(auth_url.as_str()));

    // Await the callback
    let callback = callback_rx
        .await
        .map_err(|_| anyhow!("OAuth callback was cancelled"))?
        .context("OAuth callback failed")?;

    // Validate CSRF state
    if callback.state != oauth_state {
        return Err(anyhow!("OAuth state mismatch"));
    }

    let tokens = exchange_code(&http_client, &callback.code, &verifier, &redirect_uri)
        .await
        .context("Token exchange failed")?;

    let jwt = tokens
        .id_token
        .as_deref()
        .unwrap_or(tokens.access_token.as_str());
    let claims = extract_jwt_claims(jwt);

    Ok(CodexCredentials {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at_ms: now_ms() + tokens.expires_in * 1000,
        account_id: claims.account_id,
        email: claims.email.or(tokens.email),
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
        .uri(OPENAI_TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
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

async fn refresh_token(
    client: &Arc<dyn HttpClient>,
    refresh_token: &str,
) -> Result<CodexCredentials, RefreshError> {
    let body = form_urlencoded::Serializer::new(String::new())
        .append_pair("grant_type", "refresh_token")
        .append_pair("client_id", CLIENT_ID)
        .append_pair("refresh_token", refresh_token)
        .finish();

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(OPENAI_TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
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
    let jwt = tokens
        .id_token
        .as_deref()
        .unwrap_or(tokens.access_token.as_str());
    let claims = extract_jwt_claims(jwt);

    Ok(CodexCredentials {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at_ms: now_ms() + tokens.expires_in * 1000,
        account_id: claims.account_id,
        email: claims.email.or(tokens.email),
    })
}

struct JwtClaims {
    account_id: Option<String>,
    email: Option<String>,
}

/// Extract claims from a JWT payload (base64url middle segment).
/// Extracts `chatgpt_account_id` from three possible locations (matching Roo Code's
/// implementation) and the `email` claim.
fn extract_jwt_claims(jwt: &str) -> JwtClaims {
    let Some(payload_b64) = jwt.split('.').nth(1) else {
        return JwtClaims {
            account_id: None,
            email: None,
        };
    };
    let Ok(payload) = URL_SAFE_NO_PAD.decode(payload_b64) else {
        return JwtClaims {
            account_id: None,
            email: None,
        };
    };
    let Ok(claims) = serde_json::from_slice::<serde_json::Value>(&payload) else {
        return JwtClaims {
            account_id: None,
            email: None,
        };
    };

    let account_id = claims
        .get("chatgpt_account_id")
        .and_then(|v| v.as_str())
        .or_else(|| {
            claims
                .get("https://api.openai.com/auth")
                .and_then(|v| v.get("chatgpt_account_id"))
                .and_then(|v| v.as_str())
        })
        .or_else(|| {
            claims
                .get("organizations")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|org| org.get("id"))
                .and_then(|v| v.as_str())
        })
        .map(|s| s.to_owned());

    let email = claims
        .get("email")
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());

    JwtClaims { account_id, email }
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

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http.clone(), Some(make_expired_credentials()), cx);

        let weak_state = cx.read(|_cx| state.downgrade());

        // Spawn two concurrent refresh attempts.
        let weak1 = weak_state.clone();
        let http1 = http.clone();
        let task1 =
            cx.spawn(async move |mut cx| get_fresh_credentials(&weak1, &http1, &mut cx).await);

        let weak2 = weak_state.clone();
        let http2 = http.clone();
        let task2 =
            cx.spawn(async move |mut cx| get_fresh_credentials(&weak2, &http2, &mut cx).await);

        // Drive both to completion.
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

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http.clone(), Some(make_fresh_credentials()), cx);

        let weak_state = cx.read(|_cx| state.downgrade());

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

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http.clone(), None, cx);

        let weak_state = cx.read(|_cx| state.downgrade());

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

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http.clone(), Some(make_expired_credentials()), cx);

        let weak_state = cx.read(|_cx| state.downgrade());

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

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http.clone(), Some(make_expired_credentials()), cx);

        let weak_state = cx.read(|_cx| state.downgrade());

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
                // Wait until the gate is opened, simulating a slow network.
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

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http.clone(), Some(make_expired_credentials()), cx);

        let weak_state = cx.read(|_cx| state.downgrade());

        // Start a refresh
        let weak = weak_state.clone();
        let http_clone = http.clone();
        let refresh_task =
            cx.spawn(async move |mut cx| get_fresh_credentials(&weak, &http_clone, &mut cx).await);

        cx.run_until_parked();

        // Sign out while the refresh is in-flight
        state.update(cx, |state, cx| {
            state.sign_out(cx).detach();
        });
        cx.run_until_parked();

        // Now let the refresh respond by opening the gate
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
        // Pre-populate the credential store
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
    async fn test_initial_load_restores_persisted_credentials(cx: &mut TestAppContext) {
        let creds = make_fresh_credentials();
        let creds_json = serde_json::to_vec(&creds).unwrap();
        let creds_provider = Arc::new(FakeCredentialsProvider::new());
        creds_provider
            .storage
            .lock()
            .replace(("Bearer".to_string(), creds_json));

        let http: Arc<dyn HttpClient> = FakeHttpClient::create(|request| async move {
            assert_eq!(
                request.uri().to_string(),
                "https://chatgpt.com/backend-api/codex/models?client_version=0.0.0"
            );
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::from(
                    serde_json::json!({
                        "models": [{
                            "slug": "gpt-account-default",
                            "display_name": "Account Default",
                            "default_reasoning_level": "medium",
                            "supported_reasoning_levels": [{
                                "effort": "medium",
                                "description": "Medium"
                            }],
                            "visibility": "list",
                            "priority": 0,
                            "additional_speed_tiers": [],
                            "service_tiers": [],
                            "context_window": 128_000,
                            "max_context_window": null,
                            "input_modalities": ["text"]
                        }]
                    })
                    .to_string(),
                ))?)
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
            assert_eq!(
                state.available_models().first().map(ChatGptModel::id),
                Some("gpt-account-default")
            );
        });
    }

    #[gpui::test]
    async fn test_model_catalog_uses_account_visible_models(cx: &mut TestAppContext) {
        let http_client = FakeHttpClient::create(|request| async move {
            assert_eq!(request.method(), Method::GET);
            assert_eq!(
                request.uri().to_string(),
                "https://chatgpt.com/backend-api/codex/models?client_version=1.2.3"
            );
            assert_eq!(
                request
                    .headers()
                    .get("authorization")
                    .and_then(|value| value.to_str().ok()),
                Some("Bearer fresh_access")
            );
            assert_eq!(
                request
                    .headers()
                    .get("chatgpt-account-id")
                    .and_then(|value| value.to_str().ok()),
                Some("account-123")
            );
            assert_eq!(
                request
                    .headers()
                    .get("originator")
                    .and_then(|value| value.to_str().ok()),
                Some("zed")
            );
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::from(
                    serde_json::json!({
                        "models": [
                            {
                                "slug": "gpt-5.6-sol",
                                "display_name": "GPT-5.6 Sol",
                                "default_reasoning_level": "low",
                                "supported_reasoning_levels": [{
                                    "effort": "low",
                                    "description": "Low"
                                }],
                                "visibility": "hide",
                                "priority": 0,
                                "additional_speed_tiers": ["fast"],
                                "service_tiers": [],
                                "context_window": 372_000,
                                "max_context_window": null,
                                "input_modalities": ["text", "image"]
                            },
                            {
                                "slug": "gpt-5.6-luna",
                                "display_name": "GPT-5.6 Luna",
                                "default_reasoning_level": "medium",
                                "supported_reasoning_levels": [{
                                    "effort": "medium",
                                    "description": "Medium"
                                }],
                                "visibility": "list",
                                "priority": 2,
                                "additional_speed_tiers": [],
                                "service_tiers": [{"id": "priority"}],
                                "context_window": 372_000,
                                "max_context_window": null,
                                "input_modalities": ["text", "image"]
                            },
                            {
                                "slug": "gpt-5.5",
                                "display_name": "GPT-5.5",
                                "default_reasoning_level": "medium",
                                "supported_reasoning_levels": [{
                                    "effort": "medium",
                                    "description": "Medium"
                                }],
                                "visibility": "list",
                                "priority": 1,
                                "additional_speed_tiers": [],
                                "service_tiers": [],
                                "context_window": 272_000,
                                "max_context_window": null,
                                "input_modalities": ["text"]
                            }
                        ]
                    })
                    .to_string(),
                ))?)
        });
        let mut credentials = make_fresh_credentials();
        credentials.account_id = Some("account-123".to_string());
        let state = make_state(http_client, Some(credentials), cx);
        state.update(cx, |state, _cx| {
            state.client_version = "1.2.3".into();
        });

        state
            .update(cx, |state, cx| state.refresh_model_catalog(cx))
            .await
            .expect("model discovery should succeed");

        cx.read(|cx| {
            let state = state.read(cx);
            let model_ids = state
                .available_models()
                .iter()
                .map(ChatGptModel::id)
                .collect::<Vec<_>>();
            assert_eq!(model_ids, ["gpt-5.5", "gpt-5.6-luna"]);
            assert_eq!(
                state.default_model().as_ref().map(ChatGptModel::id),
                Some("gpt-5.5")
            );
            assert_eq!(
                state.default_fast_model().as_ref().map(ChatGptModel::id),
                Some("gpt-5.6-luna")
            );
            let default_model = state
                .available_models()
                .iter()
                .find(|model| model.id() == "gpt-5.5")
                .expect("default model should be present");
            let fast_model = state
                .available_models()
                .iter()
                .find(|model| model.id() == "gpt-5.6-luna")
                .expect("fast model should be present");
            assert!(!default_model.supports_images());
            assert!(fast_model.supports_priority());
            assert!(state.model_catalog_error().is_none());
        });
    }

    #[gpui::test]
    async fn test_model_catalog_failure_preserves_fallback_models(cx: &mut TestAppContext) {
        let http_client = FakeHttpClient::create(|_| async move {
            Ok(http_client::Response::builder()
                .status(500)
                .body(http_client::AsyncBody::from("backend unavailable"))?)
        });
        let state = make_state(http_client, Some(make_fresh_credentials()), cx);
        let fallback_model_ids = cx.read(|cx| {
            state
                .read(cx)
                .available_models()
                .iter()
                .map(ChatGptModel::id)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        });

        state
            .update(cx, |state, cx| state.refresh_model_catalog(cx))
            .await
            .expect_err("model discovery should fail");

        cx.read(|cx| {
            let state = state.read(cx);
            let model_ids = state
                .available_models()
                .iter()
                .map(ChatGptModel::id)
                .map(str::to_owned)
                .collect::<Vec<_>>();
            assert_eq!(model_ids, fallback_model_ids);
            assert!(
                state
                    .model_catalog_error()
                    .is_some_and(|error| error.contains("backend unavailable"))
            );
        });
    }

    #[gpui::test]
    async fn test_model_catalog_request_times_out(cx: &mut TestAppContext) {
        let http_client = FakeHttpClient::create(|_| {
            futures::future::pending::<Result<http_client::Response<AsyncBody>>>()
        });
        let state = make_state(http_client, Some(make_fresh_credentials()), cx);

        let refresh_task = state.update(cx, |state, cx| state.refresh_model_catalog(cx));
        cx.run_until_parked();
        cx.executor().advance_clock(MODEL_CATALOG_REQUEST_TIMEOUT);
        cx.run_until_parked();

        let error = refresh_task
            .await
            .expect_err("model discovery should time out");
        assert!(
            error.to_string().contains("timed out after 5s"),
            "unexpected model discovery error: {error:#}"
        );
    }

    #[gpui::test]
    async fn test_obsolete_model_catalog_cannot_replace_newer_models(cx: &mut TestAppContext) {
        let (release_obsolete_request, obsolete_request) =
            futures::channel::oneshot::channel::<()>();
        let obsolete_request = Arc::new(Mutex::new(Some(obsolete_request)));
        let request_count = Arc::new(AtomicUsize::new(0));
        let http_client = FakeHttpClient::create({
            let obsolete_request = obsolete_request.clone();
            let request_count = request_count.clone();
            move |_| {
                let obsolete_request = obsolete_request.clone();
                let request_count = request_count.clone();
                async move {
                    let request_index = request_count.fetch_add(1, Ordering::SeqCst);
                    if request_index == 0 {
                        let receiver = obsolete_request.lock().take();
                        if let Some(receiver) = receiver {
                            receiver.await.expect("obsolete request should be released");
                        }
                    }
                    let model_id = if request_index == 0 {
                        "obsolete-model"
                    } else {
                        "current-model"
                    };
                    Ok(http_client::Response::builder().status(200).body(
                        http_client::AsyncBody::from(
                            serde_json::json!({
                                "models": [{
                                    "slug": model_id,
                                    "display_name": model_id,
                                    "default_reasoning_level": "medium",
                                    "supported_reasoning_levels": [],
                                    "visibility": "list",
                                    "priority": 0,
                                    "additional_speed_tiers": [],
                                    "service_tiers": [],
                                    "context_window": 128_000,
                                    "max_context_window": null,
                                    "input_modalities": ["text"]
                                }]
                            })
                            .to_string(),
                        ),
                    )?)
                }
            }
        });
        let state = make_state(http_client, Some(make_fresh_credentials()), cx);

        let obsolete_refresh = state.update(cx, |state, cx| state.refresh_model_catalog(cx));
        cx.run_until_parked();
        state
            .update(cx, |state, cx| state.refresh_model_catalog(cx))
            .await
            .expect("newer model discovery should succeed");
        release_obsolete_request
            .send(())
            .expect("obsolete request should remain connected");
        obsolete_refresh
            .await
            .expect("obsolete model discovery may still complete");

        cx.read(|cx| {
            assert_eq!(
                state
                    .read(cx)
                    .available_models()
                    .first()
                    .map(ChatGptModel::id),
                Some("current-model")
            );
        });
    }

    #[gpui::test]
    async fn test_server_side_compaction_streams_from_codex_responses(cx: &mut TestAppContext) {
        let compaction_request_count = Arc::new(AtomicUsize::new(0));
        let http_client = FakeHttpClient::create({
            let compaction_request_count = compaction_request_count.clone();
            move |request| {
                let compaction_request_count = compaction_request_count.clone();
                async move {
                    assert_eq!(
                        request.uri().to_string(),
                        "https://chatgpt.com/backend-api/codex/responses"
                    );
                    assert_eq!(
                        request
                            .headers()
                            .get("authorization")
                            .and_then(|value| value.to_str().ok()),
                        Some("Bearer fresh_access")
                    );
                    assert_eq!(
                        request
                            .headers()
                            .get("chatgpt-account-id")
                            .and_then(|value| value.to_str().ok()),
                        Some("account-123")
                    );
                    assert_eq!(
                        request
                            .headers()
                            .get("session-id")
                            .and_then(|value| value.to_str().ok()),
                        Some("thread-123")
                    );
                    assert_eq!(
                        request
                            .headers()
                            .get("thread-id")
                            .and_then(|value| value.to_str().ok()),
                        Some("thread-123")
                    );
                    let mut request_body = String::new();
                    smol::io::AsyncReadExt::read_to_string(
                        &mut request.into_body(),
                        &mut request_body,
                    )
                    .await?;
                    let request_body: serde_json::Value = serde_json::from_str(&request_body)?;
                    assert_eq!(
                        request_body["context_management"],
                        serde_json::json!([{
                            "type": "compaction",
                            "compact_threshold": 100_000,
                        }])
                    );
                    compaction_request_count.fetch_add(1, Ordering::SeqCst);
                    Ok(http_client::Response::builder()
                        .status(200)
                        .body(http_client::AsyncBody::from(compaction_response_stream()))?)
                }
            }
        });

        let http: Arc<dyn HttpClient> = http_client;
        let mut credentials = make_fresh_credentials();
        credentials.account_id = Some("account-123".to_string());
        let state = make_state(http, Some(credentials), cx);
        let model = cx.read(|cx| create_language_model(ChatGptModel::Gpt55, &state, cx));
        assert!(model.supports_server_side_compaction());
        assert!(model.supports_explicit_compaction());

        let request = LanguageModelRequest {
            messages: vec![language_model::LanguageModelRequestMessage {
                role: language_model::Role::User,
                content: vec![language_model::MessageContent::Text("Hello".into())],
                cache: false,
                reasoning_details: None,
            }],
            compact_at_tokens: Some(100_000),
            thread_id: Some("thread-123".to_string()),
            ..Default::default()
        };
        let async_cx = cx.to_async();
        let events = model
            .stream_completion(request, &async_cx)
            .await
            .expect("the response stream should start")
            .collect::<Vec<_>>()
            .await;

        assert_eq!(compaction_request_count.load(Ordering::SeqCst), 1);
        assert!(matches!(
            events.first(),
            Some(Ok(LanguageModelCompletionEvent::Compaction(
                language_model::CompactionUpdate::Started
            )))
        ));
        let Some(Ok(LanguageModelCompletionEvent::Compaction(
            language_model::CompactionUpdate::Finished(
                language_model::CompactedContext::ProviderState(compaction_state),
            ),
        ))) = events.get(1)
        else {
            panic!("expected the streamed provider compaction state");
        };
        assert_eq!(compaction_state.provider_id(), &PROVIDER_ID);
        let items = open_ai::responses::provider_compaction_items(&compaction_state, &PROVIDER_ID)
            .expect("the compacted state should parse")
            .expect("the compacted state should be owned by the subscription provider");
        assert_eq!(
            items,
            vec![serde_json::json!({
                "type": "compaction",
                "id": "cmp_1",
                "encrypted_content": "opaque-state",
            })]
        );
    }

    #[gpui::test]
    async fn test_explicit_compaction_streams_with_codex_compaction_trigger(
        cx: &mut TestAppContext,
    ) {
        let http_client = FakeHttpClient::create(move |request| async move {
            assert_eq!(
                request.uri().to_string(),
                "https://chatgpt.com/backend-api/codex/responses"
            );
            assert_eq!(
                request
                    .headers()
                    .get("session-id")
                    .and_then(|value| value.to_str().ok()),
                Some("thread-123")
            );
            assert_eq!(
                request
                    .headers()
                    .get("thread-id")
                    .and_then(|value| value.to_str().ok()),
                Some("thread-123")
            );
            let mut request_body = String::new();
            smol::io::AsyncReadExt::read_to_string(&mut request.into_body(), &mut request_body)
                .await?;
            let request_body: serde_json::Value = serde_json::from_str(&request_body)?;
            assert!(request_body.get("context_management").is_none());
            assert_eq!(
                request_body["input"]
                    .as_array()
                    .and_then(|input| input.last()),
                Some(&serde_json::json!({"type": "compaction_trigger"}))
            );
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::from(compaction_response_stream()))?)
        });

        let http: Arc<dyn HttpClient> = http_client;
        let state = make_state(http, Some(make_fresh_credentials()), cx);
        let model = cx.read(|cx| create_language_model(ChatGptModel::Gpt55, &state, cx));
        let request = LanguageModelRequest {
            messages: vec![language_model::LanguageModelRequestMessage {
                role: language_model::Role::User,
                content: vec![language_model::MessageContent::Text("Hello".into())],
                cache: false,
                reasoning_details: None,
            }],
            compact_at_tokens: Some(100_000),
            thread_id: Some("thread-123".to_string()),
            ..Default::default()
        };

        let result = model
            .compact(request, &cx.to_async())
            .await
            .expect("manual compaction should succeed");
        let language_model::CompactedContext::ProviderState(compaction_state) = result.context
        else {
            panic!("expected provider compaction state");
        };
        let items = open_ai::responses::provider_compaction_items(&compaction_state, &PROVIDER_ID)
            .expect("the compacted state should parse")
            .expect("the compacted state should be owned by the subscription provider");
        assert_eq!(
            items,
            vec![serde_json::json!({
                "type": "compaction",
                "id": "cmp_1",
                "encrypted_content": "opaque-state",
            })]
        );
    }

    fn compaction_response_stream() -> String {
        let compaction_item = serde_json::json!({
            "type": "compaction",
            "id": "cmp_1",
            "encrypted_content": "opaque-state",
        });
        [
            serde_json::json!({
                "type": "response.output_item.added",
                "output_index": 0,
                "item": compaction_item,
            }),
            serde_json::json!({
                "type": "response.output_item.done",
                "output_index": 0,
                "item": compaction_item,
            }),
            serde_json::json!({
                "type": "response.completed",
                "response": {
                    "status": "completed",
                    "output": [],
                },
            }),
        ]
        .into_iter()
        .map(|event| format!("data: {event}\n\n"))
        .collect()
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
        credentials: Option<CodexCredentials>,
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
        credentials: Option<CodexCredentials>,
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
            client_version: "0.0.0".into(),
            available_models: ChatGptModel::all(),
            auth_generation: 0,
            model_catalog_generation: 0,
            last_auth_error: None,
            last_model_catalog_error: None,
        })
    }

    fn make_expired_credentials() -> CodexCredentials {
        CodexCredentials {
            access_token: "old_access".to_string(),
            refresh_token: "old_refresh".to_string(),
            expires_at_ms: 0,
            account_id: None,
            email: None,
        }
    }

    fn make_fresh_credentials() -> CodexCredentials {
        CodexCredentials {
            access_token: "fresh_access".to_string(),
            refresh_token: "fresh_refresh".to_string(),
            expires_at_ms: now_ms() + 3_600_000,
            account_id: None,
            email: None,
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
}
