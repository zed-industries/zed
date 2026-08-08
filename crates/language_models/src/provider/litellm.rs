use anyhow::{Context as _, Result};
use collections::HashMap;
use credentials_provider::CredentialsProvider;
use fs::Fs;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{App, AsyncApp, Context, Entity, SharedString, Task, TaskExt, Window};
use http_client::{AsyncBody, CustomHeaders, HttpClient, HttpRequestExt, Method, Request as HttpRequest, RequestBuilderExt};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, InlineDescription, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelEffortLevel,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolSchemaFormat, ProviderSettingsView, RateLimiter,
    SubPageProviderSettings, env_var,
};
use menu;
use open_ai::{
    ResponseStreamEvent,
    responses::{Request as ResponseRequest, StreamEvent as ResponsesStreamEvent, stream_response},
    stream_completion,
};
use serde::Deserialize;
use settings::{Settings, SettingsStore, update_settings_file};
use std::sync::{Arc, LazyLock};
use ui::{ButtonLike, ButtonLink, Divider, Tooltip, prelude::*};
use ui_input::InputField;

use crate::provider::open_ai::{
    OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai, into_open_ai_response,
};
use crate::AllLanguageModelSettings;

pub use settings::{OpenAiCompatibleAvailableModel as AvailableModel};
pub use settings::OpenAiCompatibleModelCapabilities as ModelCapabilities;
pub use settings::OpenAiReasoningEffort;

const LITELLM_API_URL: &str = "http://localhost:4000/v1";
const LITELLM_DOCS_URL: &str = "https://docs.litellm.ai/docs/proxy/quick_start";
const LITELLM_SITE: &str = "https://www.litellm.ai/";
const LITELLM_MODELS_URL: &str = "https://models.litellm.ai/";

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("litellm");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("LiteLLM");

const API_KEY_ENV_VAR_NAME: &str = "LITELLM_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

// Context window size applied to discovered models when the proxy's `/model/info`
// doesn't report one (e.g. passthrough models without metadata).
const DEFAULT_MAX_TOKENS: u64 = 128_000;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LiteLlmSettings {
    pub api_url: String,
    pub custom_headers: CustomHeaders,
}

pub struct LiteLlmLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    fetched_models: Vec<AvailableModel>,
    fetch_model_task: Option<Task<Result<()>>>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        !self.fetched_models.is_empty()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = LiteLlmLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        self.fetched_models.clear();
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = LiteLlmLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let http_client = Arc::clone(&self.http_client);
        let api_url = LiteLlmLanguageModelProvider::api_url(cx);
        let api_key = self.api_key_state.key(&api_url);
        let extra_headers = LiteLlmLanguageModelProvider::settings(cx).custom_headers.clone();

        // Fetching the model list doubles as an authentication check: if the
        // master key is wrong or the proxy is down, this fails and the provider
        // stays unauthenticated.
        cx.spawn(async move |this, cx| {
            let models = fetch_models(
                http_client.as_ref(),
                &api_url,
                api_key.as_deref(),
                &extra_headers,
            )
            .await?;

            this.update(cx, |this, cx| {
                this.fetched_models = models;
                cx.notify();
            })
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        let task = self.fetch_models(cx);
        self.fetch_model_task.replace(task);
    }
}

impl LiteLlmLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        Self {
            http_client: http_client.clone(),
            state: cx.new(|cx| {
                cx.observe_global::<SettingsStore>({
                    let mut last_settings = LiteLlmLanguageModelProvider::settings(cx).clone();
                    move |this: &mut State, cx| {
                        let current_settings = LiteLlmLanguageModelProvider::settings(cx);
                        let settings_changed = current_settings != &last_settings;
                        if settings_changed {
                            let url_changed = last_settings.api_url != current_settings.api_url;
                            last_settings = current_settings.clone();
                            if url_changed {
                                let credentials_provider = this.credentials_provider.clone();
                                let api_url = Self::api_url(cx);
                                this.api_key_state.handle_url_change(
                                    api_url,
                                    |this| &mut this.api_key_state,
                                    credentials_provider,
                                    cx,
                                );
                                this.fetched_models.clear();
                                this.authenticate(cx).detach();
                            }
                            cx.notify();
                        }
                    }
                })
                .detach();

                State {
                    http_client,
                    fetched_models: Default::default(),
                    fetch_model_task: None,
                    api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                    credentials_provider,
                }
            }),
        }
    }

    fn settings(cx: &App) -> &LiteLlmSettings {
        &AllLanguageModelSettings::get_global(cx).litellm
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            SharedString::from(LITELLM_API_URL)
        } else {
            SharedString::new(api_url.as_str())
        }
    }

    fn has_custom_url(cx: &App) -> bool {
        Self::settings(cx).api_url != LITELLM_API_URL
    }
}

impl LanguageModelProviderState for LiteLlmLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for LiteLlmLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiLiteLlm)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.state
            .read(cx)
            .fetched_models
            .first()
            .map(|model| self.create_language_model(model.clone()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: Vec<_> = self
            .state
            .read(cx)
            .fetched_models
            .iter()
            .map(|model| self.create_language_model(model.clone()))
            .collect();
        models.sort_by_key(|model| model.name());
        models
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn settings_view(&self, _cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.clone();
        Some(ProviderSettingsView::SubPage(
            SubPageProviderSettings::new(move |window, cx| {
                cx.new(|cx| ConfigurationView::new(state.clone(), window, cx))
                    .into()
            })
            .description(InlineDescription::Text(
                "Connect to a LiteLLM proxy to use many LLM providers through one OpenAI-compatible API.".into(),
            )),
        ))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }
}

impl LiteLlmLanguageModelProvider {
    fn create_language_model(&self, model: AvailableModel) -> Arc<dyn LanguageModel> {
        Arc::new(LiteLlmLanguageModel {
            id: LanguageModelId::from(model.name.clone()),
            provider_id: PROVIDER_ID,
            provider_name: PROVIDER_NAME,
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

pub struct LiteLlmLanguageModel {
    id: LanguageModelId,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    model: AvailableModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl LiteLlmLanguageModel {
    fn stream_completion(
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

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = LiteLlmLanguageModelProvider::api_url(cx);
            let extra_headers = LiteLlmLanguageModelProvider::settings(cx).custom_headers.clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let provider = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_completion(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn stream_response(
        &self,
        request: ResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<ResponsesStreamEvent>>>,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = LiteLlmLanguageModelProvider::api_url(cx);
            let extra_headers = LiteLlmLanguageModelProvider::settings(cx).custom_headers.clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let provider = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_response(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

fn default_thinking_reasoning_effort(model: &AvailableModel) -> Option<open_ai::ReasoningEffort> {
    model
        .reasoning_effort
        .filter(|effort| *effort != open_ai::ReasoningEffort::None)
}

fn supported_thinking_effort_levels(model: &AvailableModel) -> Vec<LanguageModelEffortLevel> {
    let Some(default_effort) = default_thinking_reasoning_effort(model) else {
        return Vec::new();
    };

    open_ai::ReasoningEffort::OPENAI_COMPATIBLE_SELECTABLE
        .into_iter()
        .map(|effort| LanguageModelEffortLevel {
            name: effort.label().into(),
            value: effort.value().into(),
            is_default: effort == default_effort,
        })
        .collect()
}

fn selected_thinking_reasoning_effort(
    request: &LanguageModelRequest,
) -> Option<open_ai::ReasoningEffort> {
    request
        .thinking_effort
        .as_deref()
        .and_then(|effort| effort.parse::<open_ai::ReasoningEffort>().ok())
        .filter(|effort| *effort != open_ai::ReasoningEffort::None)
}

fn chat_completion_max_tokens_parameter(
    model: &AvailableModel,
) -> crate::provider::open_ai::ChatCompletionMaxTokensParameter {
    if model.capabilities.max_tokens_parameter {
        crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxTokens
    } else {
        crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxCompletionTokens
    }
}

fn supports_none_reasoning_effort(model: &AvailableModel) -> bool {
    model.reasoning_effort.is_some()
}

fn chat_completion_reasoning_effort(
    request: &LanguageModelRequest,
    model: &AvailableModel,
) -> Option<open_ai::ReasoningEffort> {
    if model.reasoning_effort == Some(open_ai::ReasoningEffort::None) {
        return Some(open_ai::ReasoningEffort::None);
    }

    if request.thinking_allowed {
        selected_thinking_reasoning_effort(request)
            .or_else(|| default_thinking_reasoning_effort(model))
    } else if supports_none_reasoning_effort(model) {
        Some(open_ai::ReasoningEffort::None)
    } else {
        None
    }
}

fn disable_response_thinking_for_none_effort(
    request: &mut LanguageModelRequest,
    model: &AvailableModel,
) {
    if model.reasoning_effort == Some(open_ai::ReasoningEffort::None) {
        request.thinking_allowed = false;
        request.thinking_effort = None;
    }
}

impl LanguageModel for LiteLlmLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(
            self.model
                .display_name
                .clone()
                .unwrap_or_else(|| self.model.name.clone()),
        )
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        self.provider_id.clone()
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        self.provider_name.clone()
    }

    fn supports_tools(&self) -> bool {
        self.model.capabilities.tools
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn supports_images(&self) -> bool {
        self.model.capabilities.images
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => self.model.capabilities.tools,
            LanguageModelToolChoice::Any => self.model.capabilities.tools,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        default_thinking_reasoning_effort(&self.model).is_some()
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        supported_thinking_effort_levels(&self.model)
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("litellm/{}", self.model.name)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens
    }

    fn stream_completion(
        &self,
        mut request: LanguageModelRequest,
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
        // `speed` can leak in from a parent thread's model; arbitrary endpoints
        // reject `service_tier`, so drop it unless fast mode is supported.
        if !self.supports_fast_mode() {
            request.speed = None;
        }

        if self.model.capabilities.chat_completions {
            let reasoning_effort = chat_completion_reasoning_effort(&request, &self.model);
            let request = match into_open_ai(
                request,
                &self.model.name,
                self.model.capabilities.parallel_tool_calls,
                self.model.capabilities.prompt_cache_key,
                self.max_output_tokens(),
                chat_completion_max_tokens_parameter(&self.model),
                reasoning_effort,
                self.model.capabilities.interleaved_reasoning,
            ) {
                Ok(request) => request,
                Err(error) => return async move { Err(error.into()) }.boxed(),
            };
            let completions = self.stream_completion(request, cx);
            async move {
                let mapper = OpenAiEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        } else {
            disable_response_thinking_for_none_effort(&mut request, &self.model);
            let request = match into_open_ai_response(
                request,
                &self.model.name,
                self.model.capabilities.parallel_tool_calls,
                self.model.capabilities.prompt_cache_key,
                self.max_output_tokens(),
                default_thinking_reasoning_effort(&self.model),
                supports_none_reasoning_effort(&self.model),
                &self.provider_id,
            ) {
                Ok(request) => request,
                Err(error) => return async move { Err(error.into()) }.boxed(),
            };
            let completions = self.stream_response(request, cx);
            let compaction_state_owner = self.provider_id.clone();
            async move {
                let mapper = OpenAiResponseEventMapper::new(compaction_state_owner);
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        }
    }
}

// ── Discovery: /v1/models + /model/info ───────────────────────────────────

/// Response of `GET /v1/models` (standard OpenAI format).
#[derive(Deserialize)]
struct ListModelsResponse {
    #[serde(default)]
    data: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    id: String,
}

/// Response of LiteLLM's `GET /model/info`, which augments each model with
/// capabilities and token limits not present in `/v1/models`.
#[derive(Deserialize)]
struct ModelInfoResponse {
    #[serde(default)]
    data: Vec<ModelInfoEntry>,
}

#[derive(Deserialize)]
struct ModelInfoEntry {
    model_name: String,
    #[serde(default)]
    model_info: ModelInfo,
}

#[derive(Default, Deserialize)]
struct ModelInfo {
    #[serde(default)]
    max_tokens: Option<u64>,
    #[serde(default)]
    max_input_tokens: Option<u64>,
    #[serde(default)]
    max_output_tokens: Option<u64>,
    #[serde(default)]
    supports_function_calling: Option<bool>,
    #[serde(default)]
    supports_vision: Option<bool>,
    #[serde(default)]
    supports_reasoning: Option<bool>,
    #[serde(default)]
    supports_prompt_caching: Option<bool>,
}

/// Build a `AvailableModel` from a `/v1/models` entry, enriched with
/// `/model/info` when available. The mapping follows LiteLLM's field
/// semantics: `max_input_tokens` is the context window, `max_tokens` is the
/// output limit.
fn model_from_entry(id: String, info: Option<&ModelInfo>) -> AvailableModel {
    let max_tokens = info
        .and_then(|i| i.max_input_tokens)
        .or(info.and_then(|i| i.max_tokens))
        .unwrap_or(DEFAULT_MAX_TOKENS);

    let max_output_tokens = info
        .and_then(|i| i.max_output_tokens)
        .or(info.and_then(|i| i.max_tokens));

    let tools = info
        .and_then(|i| i.supports_function_calling)
        .unwrap_or(true);

    let images = info.and_then(|i| i.supports_vision).unwrap_or(false);

    let prompt_cache_key = info
        .and_then(|i| i.supports_prompt_caching)
        .unwrap_or(false);

    let reasoning_effort = info
        .and_then(|i| i.supports_reasoning)
        .and_then(|supports| supports.then_some(OpenAiReasoningEffort::Medium));

    AvailableModel {
        name: id,
        display_name: None,
        max_tokens,
        max_output_tokens,
        max_completion_tokens: None,
        reasoning_effort,
        capabilities: ModelCapabilities {
            tools,
            images,
            parallel_tool_calls: false,
            prompt_cache_key,
            chat_completions: true,
            interleaved_reasoning: false,
            max_tokens_parameter: false,
        },
    }
}

/// Fetch the proxy's model list from `/v1/models` and enrich it with
/// capabilities from `/model/info`.
///
/// `api_url` includes the `/v1` prefix, so `/models` is appended directly.
/// LiteLLM serves `/model/info` at the API root (without `/v1`), so the
/// trailing `/v1` is stripped for that request.
async fn fetch_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    extra_headers: &CustomHeaders,
) -> Result<Vec<AvailableModel>> {
    let models_url = format!("{api_url}/models");
    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(&models_url)
        .header("Accept", "application/json")
        .when_some(api_key, |builder, key| {
            builder.header("Authorization", format!("Bearer {key}"))
        })
        .extra_headers(extra_headers)
        .body(AsyncBody::default())?;

    let mut response = client.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    anyhow::ensure!(
        response.status().is_success(),
        "Failed to fetch models: {} {}",
        response.status(),
        body,
    );
    let models_response: ListModelsResponse =
        serde_json::from_str(&body).context("Unable to parse /v1/models response")?;

    // /model/info is optional and LiteLLM-specific; ignore failures.
    let mut model_info: HashMap<String, ModelInfo> = HashMap::default();
    let root_url = api_url.strip_suffix("/v1").unwrap_or(api_url);
    let info_url = format!("{root_url}/model/info");
    let info_request = HttpRequest::builder()
        .method(Method::GET)
        .uri(&info_url)
        .header("Accept", "application/json")
        .when_some(api_key, |builder, key| {
            builder.header("Authorization", format!("Bearer {key}"))
        })
        .extra_headers(extra_headers)
        .body(AsyncBody::default())?;

    if let Ok(mut info_response) = client.send(info_request).await {
        if info_response.status().is_success() {
            let mut info_body = String::new();
            if info_response.body_mut().read_to_string(&mut info_body).await.is_ok() {
                if let Ok(parsed) = serde_json::from_str::<ModelInfoResponse>(&info_body) {
                    for entry in parsed.data {
                        model_info.insert(entry.model_name, entry.model_info);
                    }
                }
            }
        }
    }

    let mut models = Vec::new();
    for entry in models_response.data {
        let info = model_info.get(&entry.id);
        models.push(model_from_entry(entry.id, info));
    }

    models.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(models)
}

// ── Configuration view ────────────────────────────────────────────────────

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    api_url_editor: Entity<InputField>,
    state: Entity<State>,
}

impl ConfigurationView {
    pub fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| InputField::new(window, cx, "sk-...").label("API key"));

        let api_url_editor = cx.new(|cx| {
            let input = InputField::new(window, cx, LITELLM_API_URL).label("API URL");
            input.set_text(&LiteLlmLanguageModelProvider::api_url(cx), window, cx);
            input
        });

        Self {
            api_key_editor,
            api_url_editor,
            state,
        }
    }

    fn retry_connection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let has_api_url = LiteLlmLanguageModelProvider::has_custom_url(cx);
        let has_api_key = self
            .state
            .read_with(cx, |state, _| state.api_key_state.has_key());
        if !has_api_url {
            self.save_api_url(cx);
        }
        if !has_api_key {
            self.save_api_key(&Default::default(), window, cx);
        }

        self.state.update(cx, |state, cx| state.restart_fetch_models_task(cx));
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn save_api_url(&self, cx: &mut Context<Self>) {
        let api_url = self.api_url_editor.read(cx).text(cx).trim().to_string();
        let current_url = LiteLlmLanguageModelProvider::api_url(cx);
        if !api_url.is_empty() && &api_url != &current_url {
            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .litellm
                    .get_or_insert_default()
                    .api_url = Some(api_url);
            });
        }
    }

    fn reset_api_url(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_url_editor
            .update(cx, |input, cx| input.set_text("", window, cx));
        let fs = <dyn Fs>::global(cx);
        update_settings_file(fs, cx, |settings, _cx| {
            if let Some(settings) = settings
                .language_models
                .as_mut()
                .and_then(|models| models.litellm.as_mut())
            {
                settings.api_url = Some(LITELLM_API_URL.into());
            }
        });
        cx.notify();
    }

    fn render_instructions(_cx: &App) -> Div {
        v_flex()
            .gap_2()
            .child(
                Label::new(
                    "LiteLLM is a proxy that exposes many LLM providers (OpenAI, Anthropic, \
                     Bedrock, Vertex, Ollama, and more) through a single OpenAI-compatible API. \
                     To connect to the proxy, enter the URL and optionally provide a master key.",
                )
                .color(Color::Muted),
            )
            .child(
                h_flex()
                    .gap_0p5()
                    .child(
                        Label::new("To set up a proxy, see the").color(Color::Muted),
                    )
                    .child(ButtonLink::new("LiteLLM docs", LITELLM_DOCS_URL)),
            )
    }

    fn render_api_key_editor(&self, cx: &Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable.")
        } else {
            "API key configured".to_string()
        };

        if state.api_key_state.has_key() {
            h_flex()
                .p_1()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().background.opacity(0.5))
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(configured_card_label)),
                )
                .child(
                    Button::new("reset-api-key", "Reset")
                        .style(ButtonStyle::Outlined)
                        .label_size(LabelSize::Small)
                        .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                )
        } else {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(self.api_key_editor.clone())
                .gap_1p5()
                .child(
                    Label::new(format!(
                        "You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed.",
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
        }
    }

    fn render_api_url_editor(&self, cx: &Context<Self>) -> Div {
        let api_url = LiteLlmLanguageModelProvider::api_url(cx);
        let custom_api_url_set = api_url.as_ref() != LITELLM_API_URL;

        if custom_api_url_set {
            h_flex()
                .p_1()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().background.opacity(0.5))
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(api_url)),
                )
                .child(
                    Button::new("reset-api-url", "Reset API URL")
                        .style(ButtonStyle::Outlined)
                        .label_size(LabelSize::Small)
                        .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_url(window, cx))),
                )
        } else {
            v_flex()
                .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| {
                    this.save_api_url(cx);
                    cx.notify();
                }))
                .gap_2()
                .child(self.api_url_editor.clone())
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

        v_flex()
            .gap_2()
            .child(Headline::new("LiteLLM").size(HeadlineSize::Small))
            .child(Self::render_instructions(cx))
            .child(self.render_api_url_editor(cx))
            .child(self.render_api_key_editor(cx))
            .child(Divider::horizontal())
            .child(
                h_flex()
                    .pt_2()
                    .w_full()
                    .justify_between()
                    .gap_1()
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .map(|this| {
                                if is_authenticated {
                                    this.child(
                                        Button::new("litellm-site", "LiteLLM")
                                            .style(ButtonStyle::OutlinedGhost)
                                            .size(ButtonSize::Medium)
                                            .end_icon(
                                                Icon::new(IconName::ArrowUpRight)
                                                    .size(IconSize::XSmall)
                                                    .color(Color::Muted),
                                            )
                                            .on_click(move |_, _, cx| cx.open_url(LITELLM_SITE))
                                            .into_any_element(),
                                    )
                                } else {
                                    this.child(
                                        Button::new("get-litellm", "Get LiteLLM")
                                            .style(ButtonStyle::OutlinedGhost)
                                            .size(ButtonSize::Medium)
                                            .end_icon(
                                                Icon::new(IconName::ArrowUpRight)
                                                    .size(IconSize::XSmall)
                                                    .color(Color::Muted),
                                            )
                                            .on_click(move |_, _, cx| cx.open_url(LITELLM_SITE))
                                            .into_any_element(),
                                    )
                                }
                            })
                            .child(
                                Button::new("browse-models", "Browse Models")
                                    .style(ButtonStyle::OutlinedGhost)
                                    .size(ButtonSize::Medium)
                                    .end_icon(
                                        Icon::new(IconName::ArrowUpRight)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                    .on_click(move |_, _, cx| cx.open_url(LITELLM_MODELS_URL)),
                            ),
                    )
                    .map(|this| {
                        if is_authenticated {
                            this.child(
                                ButtonLike::new("connected")
                                    .size(ButtonSize::Medium)
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .child(Icon::new(IconName::Check).color(Color::Success))
                                            .child(Label::new("Connected")),
                                    )
                                    .child(
                                        IconButton::new("refresh-models", IconName::RotateCcw)
                                            .icon_size(IconSize::Small)
                                            .tooltip(Tooltip::text("Refresh Models"))
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.retry_connection(window, cx)
                                            })),
                                    ),
                            )
                        } else {
                            this.child(
                                Button::new("connect", "Connect")
                                    .style(ButtonStyle::OutlinedGhost)
                                    .size(ButtonSize::Medium)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.retry_connection(window, cx)
                                    })),
                            )
                        }
                    }),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_from_entry_uses_input_tokens_as_context_window() {
        // LiteLLM reports `max_tokens` as the output limit and `max_input_tokens`
        // as the context window; Zed's `max_tokens` is the context window.
        let info = ModelInfo {
            max_tokens: Some(65535),
            max_input_tokens: Some(1048576),
            max_output_tokens: Some(65535),
            supports_function_calling: Some(true),
            supports_vision: Some(true),
            supports_reasoning: Some(true),
            supports_prompt_caching: Some(true),
        };
        let model = model_from_entry("gemini-2.5-pro".into(), Some(&info));

        assert_eq!(model.name, "gemini-2.5-pro");
        assert_eq!(model.max_tokens, 1_048_576);
        assert_eq!(model.max_output_tokens, Some(65535));
        assert!(model.capabilities.tools);
        assert!(model.capabilities.images);
        assert!(model.capabilities.prompt_cache_key);
        assert_eq!(model.reasoning_effort, Some(OpenAiReasoningEffort::Medium));
    }

    #[test]
    fn model_from_entry_falls_back_to_defaults_without_info() {
        let model = model_from_entry("unknown-model".into(), None);

        assert_eq!(model.name, "unknown-model");
        assert_eq!(model.max_tokens, DEFAULT_MAX_TOKENS);
        assert_eq!(model.max_output_tokens, None);
        // Tools default on: most OpenAI-compatible endpoints support function calling.
        assert!(model.capabilities.tools);
        assert!(!model.capabilities.images);
        assert_eq!(model.reasoning_effort, None);
    }

    #[test]
    fn model_from_entry_falls_back_to_max_tokens_for_context_window() {
        // Some entries report only `max_tokens` (no `max_input_tokens`); use it as
        // the context window rather than leaving the model with no limit.
        let info = ModelInfo {
            max_tokens: Some(200_000),
            ..Default::default()
        };
        let model = model_from_entry("model".into(), Some(&info));

        assert_eq!(model.max_tokens, 200_000);
        assert_eq!(model.max_output_tokens, Some(200_000));
    }
}
