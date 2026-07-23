use anyhow::{Context as _, Result};
use collections::HashMap;
use credentials_provider::CredentialsProvider;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Subscription, Task};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
};
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolSchemaFormat, ProviderSettingsView, RateLimiter, SubPageProviderSettings,
};
use open_ai::{
    ResponseStreamEvent,
    responses::{Request as ResponseRequest, StreamEvent as ResponsesStreamEvent, stream_response},
    stream_completion,
};
use serde::Deserialize;
use settings::Settings;
use std::sync::Arc;
use ui::IconName;

use crate::provider::api_compatible::{
    ApiCompatibleProviderConfigurationView, ApiCompatibleProviderSettings,
    ApiCompatibleProviderState,
};
use crate::provider::open_ai::{
    OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai, into_open_ai_response,
};
pub use settings::OpenAiCompatibleAvailableModel as AvailableModel;
pub use settings::OpenAiCompatibleModelCapabilities as ModelCapabilities;

const API_KEY_PLACEHOLDER: &str = "000000000000000000000000000000000000000000000000000";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiCompatibleSettings {
    pub api_url: String,
    pub auto_discover: bool,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

impl ApiCompatibleProviderSettings for OpenAiCompatibleSettings {
    fn api_url(&self) -> &str {
        &self.api_url
    }
}

pub type State = ApiCompatibleProviderState<OpenAiCompatibleSettings>;

pub struct OpenAiCompatibleLanguageModelProvider {
    id: LanguageModelProviderId,
    name: LanguageModelProviderName,
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    discovery_state: Entity<OpenAiCompatibleDiscoveryState>,
}

impl OpenAiCompatibleLanguageModelProvider {
    pub fn new(
        id: Arc<str>,
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = State::new(
            id.clone(),
            credentials_provider,
            |id, cx| {
                crate::AllLanguageModelSettings::get_global(cx)
                    .openai_compatible
                    .get(id)
            },
            cx,
        );

        let discovery_state = cx
            .new(|cx| OpenAiCompatibleDiscoveryState::new(http_client.clone(), state.clone(), cx));
        discovery_state.update(cx, |discovery, cx| discovery.handle_settings_change(cx));

        Self {
            id: id.clone().into(),
            name: id.into(),
            http_client,
            state,
            discovery_state,
        }
    }

    fn create_language_model(&self, model: AvailableModel) -> Arc<dyn LanguageModel> {
        Arc::new(OpenAiCompatibleLanguageModel {
            id: LanguageModelId::from(model.name.clone()),
            provider_id: self.id.clone(),
            provider_name: self.name.clone(),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    /// Returns the effective model list: discovered models (when `auto_discover`
    /// is on) form the base set, and manually-configured `available_models`
    /// override entries with the same name and add any discovery didn't surface.
    fn merged_available_models(&self, cx: &App) -> Vec<AvailableModel> {
        let state = self.state.read(cx);
        let mut models: HashMap<String, AvailableModel> = HashMap::default();

        if state.settings.auto_discover {
            for model in self.discovery_state.read(cx).discovered_models.iter() {
                models.insert(model.name.clone(), model.clone());
            }
        }
        for model in state.settings.available_models.iter() {
            models.insert(model.name.clone(), model.clone());
        }

        let mut models: Vec<_> = models.into_values().collect();
        models.sort_by(|a, b| a.name.cmp(&b.name));
        models
    }
}

impl LanguageModelProviderState for OpenAiCompatibleLanguageModelProvider {
    type ObservableEntity = OpenAiCompatibleDiscoveryState;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.discovery_state.clone())
    }
}

impl LanguageModelProvider for OpenAiCompatibleLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelProviderName {
        self.name.clone()
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAiCompat)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.merged_available_models(cx)
            .into_iter()
            .next()
            .map(|model| self.create_language_model(model))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        self.merged_available_models(cx)
            .into_iter()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn settings_view(&self, _cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.clone();
        Some(ProviderSettingsView::SubPage(SubPageProviderSettings::new(
            move |window, cx| {
                cx.new(|cx| {
                    ApiCompatibleProviderConfigurationView::new(
                        state.clone(),
                        "OpenAI",
                        API_KEY_PLACEHOLDER,
                        window,
                        cx,
                    )
                })
                .into()
            },
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }
}

/// Fallback `max_tokens` (context window) for models discovered from the
/// `/models` endpoint, which does not report context length. Provide an
/// accurate value by listing the model in `available_models`.
const DISCOVERED_MODEL_DEFAULT_MAX_TOKENS: u64 = 128_000;

/// Models discovered from the provider's `/models` endpoint when `auto_discover`
/// is enabled.
///
/// This is a dedicated entity (separate from the shared `ApiCompatibleProviderState`)
/// so the registry can observe it via `LanguageModelProviderState::observable_entity`
/// and re-query `provided_models` once discovery resolves. It observes the
/// settings/api-key state so that URL, credential, or `auto_discover` changes
/// re-run discovery and refresh the model list.
pub struct OpenAiCompatibleDiscoveryState {
    http_client: Arc<dyn HttpClient>,
    settings_state: Entity<State>,
    discovered_models: Vec<AvailableModel>,
    fetch_task: Option<Task<()>>,
    last_api_url: String,
    has_fetched: bool,
    _settings_subscription: Subscription,
}

impl OpenAiCompatibleDiscoveryState {
    fn new(
        http_client: Arc<dyn HttpClient>,
        settings_state: Entity<State>,
        cx: &mut Context<Self>,
    ) -> Self {
        let _settings_subscription = cx.observe(&settings_state, |this, _settings_state, cx| {
            this.handle_settings_change(cx);
        });

        Self {
            http_client,
            settings_state,
            discovered_models: Vec::new(),
            fetch_task: None,
            last_api_url: String::new(),
            has_fetched: false,
            _settings_subscription,
        }
    }

    /// Re-evaluates whether discovery should run based on the current settings
    /// and api-key state. Called once after construction and whenever that state
    /// changes (settings reload, api key loaded, URL change, ...).
    fn handle_settings_change(&mut self, cx: &mut Context<Self>) {
        let (auto_discover, api_url) = {
            let state = self.settings_state.read(cx);
            (state.settings.auto_discover, state.settings.api_url.clone())
        };

        if !auto_discover {
            // Cancel any in-flight discovery and drop previously discovered models.
            self.fetch_task.take();
            self.has_fetched = false;
            self.discovered_models.clear();
            self.last_api_url = api_url;
            cx.notify();
            return;
        }

        let url_changed = self.last_api_url != api_url;
        self.last_api_url = api_url.clone();
        // (Re)run discovery when the URL changed, or when we haven't yet completed
        // a successful fetch (e.g. the API key just loaded after a prior failure).
        if url_changed || !self.has_fetched {
            self.restart_fetch(&api_url, cx);
        }
        cx.notify();
    }

    fn restart_fetch(&mut self, api_url: &str, cx: &mut Context<Self>) {
        let (api_key, extra_headers) = {
            let state = self.settings_state.read(cx);
            (
                state.api_key_state.key(api_url),
                state.settings.custom_headers.clone(),
            )
        };
        let http_client = self.http_client.clone();
        let api_url = api_url.to_string();
        let task = cx.spawn(async move |this, cx| {
            let result = fetch_discovered_models(
                http_client.as_ref(),
                &api_url,
                api_key.as_deref(),
                &extra_headers,
            )
            .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(models) => {
                        this.discovered_models = models;
                        this.has_fetched = true;
                        log::info!(
                            "openai_compatible: discovered {} model(s) from {api_url}",
                            this.discovered_models.len()
                        );
                    }
                    Err(error) => {
                        // Keep any previously discovered models; manually
                        // configured `available_models` remain available
                        // regardless, so discovery failures are non-fatal.
                        log::error!(
                            "openai_compatible: failed to discover models from {api_url}: {error:#}"
                        );
                    }
                }
                cx.notify();
            })
            .ok();
        });
        self.fetch_task.replace(task);
    }
}

/// Fetches `GET {api_url}/models` and maps the OpenAI-style
/// `{"data":[{"id":"..."}]}` response into `AvailableModel`s. Any network,
/// HTTP, or parse error propagates to the caller, which logs it and falls back
/// to the manually-configured `available_models`.
async fn fetch_discovered_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    extra_headers: &CustomHeaders,
) -> Result<Vec<AvailableModel>> {
    let uri = format!("{api_url}/models");
    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");
    if let Some(api_key) = api_key {
        request_builder = request_builder.header("Authorization", format!("Bearer {api_key}"));
    }
    let request = request_builder
        .extra_headers(extra_headers)
        .body(AsyncBody::default())?;

    let mut response = client.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    anyhow::ensure!(
        response.status().is_success(),
        "openai_compatible: models request failed: {} {}",
        response.status(),
        body,
    );

    let parsed: ModelsResponse = serde_json::from_str(&body)
        .context("openai_compatible: unable to parse models response")?;

    let mut models = Vec::new();
    for entry in parsed.data {
        if let Some(id) = entry.id {
            models.push(AvailableModel {
                name: id,
                display_name: None,
                max_tokens: DISCOVERED_MODEL_DEFAULT_MAX_TOKENS,
                max_output_tokens: None,
                max_completion_tokens: None,
                reasoning_effort: None,
                capabilities: ModelCapabilities::default(),
            });
        }
    }
    models.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(models)
}

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    id: Option<String>,
}

pub struct OpenAiCompatibleLanguageModel {
    id: LanguageModelId,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    model: AvailableModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiCompatibleLanguageModel {
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

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, _cx| {
            let api_url = &state.settings.api_url;
            (
                state.api_key_state.key(api_url),
                state.settings.api_url.clone(),
                state.settings.custom_headers.clone(),
            )
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
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponsesStreamEvent>>>>
    {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, _cx| {
            let api_url = &state.settings.api_url;
            (
                state.api_key_state.key(api_url),
                state.settings.api_url.clone(),
                state.settings.custom_headers.clone(),
            )
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

impl LanguageModel for OpenAiCompatibleLanguageModel {
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
        format!("openai/{}", self.model.name)
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
        // `speed` can leak in from a parent thread's model; this provider never
        // supports fast mode, and arbitrary compatible endpoints reject `service_tier`.
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
            let request = into_open_ai_response(
                request,
                &self.model.name,
                self.model.capabilities.parallel_tool_calls,
                self.model.capabilities.prompt_cache_key,
                self.max_output_tokens(),
                default_thinking_reasoning_effort(&self.model),
                supports_none_reasoning_effort(&self.model),
            );
            let completions = self.stream_response(request, cx);
            async move {
                let mapper = OpenAiResponseEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use http_client::FakeHttpClient;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    fn available_model(reasoning_effort: Option<open_ai::ReasoningEffort>) -> AvailableModel {
        AvailableModel {
            name: "custom-model".to_string(),
            display_name: None,
            max_tokens: 128_000,
            max_output_tokens: None,
            max_completion_tokens: None,
            reasoning_effort,
            capabilities: ModelCapabilities {
                chat_completions: false,
                ..Default::default()
            },
        }
    }

    #[test]
    fn configured_reasoning_effort_supports_thinking() {
        assert_eq!(
            default_thinking_reasoning_effort(&available_model(Some(
                open_ai::ReasoningEffort::High
            ))),
            Some(open_ai::ReasoningEffort::High)
        );
    }

    #[test]
    fn missing_or_none_reasoning_effort_does_not_support_thinking() {
        assert_eq!(
            default_thinking_reasoning_effort(&available_model(None)),
            None
        );
        assert_eq!(
            default_thinking_reasoning_effort(&available_model(Some(
                open_ai::ReasoningEffort::None
            ))),
            None
        );
    }

    #[test]
    fn supported_thinking_effort_levels_use_configured_effort_as_default() {
        let effort_levels = supported_thinking_effort_levels(&available_model(Some(
            open_ai::ReasoningEffort::High,
        )));
        let values = effort_levels
            .iter()
            .map(|level| level.value.as_ref())
            .collect::<Vec<_>>();

        assert_eq!(values, ["minimal", "low", "medium", "high", "xhigh", "max"]);
        assert_eq!(
            effort_levels
                .iter()
                .find(|level| level.is_default)
                .map(|level| level.value.as_ref()),
            Some("high")
        );
    }

    #[test]
    fn supported_thinking_effort_levels_hide_missing_or_none_effort() {
        assert!(supported_thinking_effort_levels(&available_model(None)).is_empty());
        assert!(
            supported_thinking_effort_levels(&available_model(Some(
                open_ai::ReasoningEffort::None
            )))
            .is_empty()
        );
    }

    #[test]
    fn chat_completion_reasoning_effort_honors_request_and_configured_effort() {
        let model = available_model(Some(open_ai::ReasoningEffort::Medium));
        let mut request = LanguageModelRequest {
            thinking_allowed: true,
            ..Default::default()
        };

        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::Medium)
        );

        request.thinking_effort = Some("high".to_string());
        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::High)
        );

        request.thinking_effort = Some("not-supported".to_string());
        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::Medium)
        );

        request.thinking_allowed = false;
        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::None)
        );
    }

    #[test]
    fn chat_completion_reasoning_effort_omits_missing_effort() {
        let model = available_model(None);
        let request = LanguageModelRequest {
            thinking_allowed: false,
            ..Default::default()
        };

        assert_eq!(chat_completion_reasoning_effort(&request, &model), None);
    }

    #[test]
    fn chat_completion_reasoning_effort_preserves_explicit_none() {
        let model = available_model(Some(open_ai::ReasoningEffort::None));
        let request = LanguageModelRequest {
            thinking_allowed: true,
            thinking_effort: Some("high".to_string()),
            ..Default::default()
        };

        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::None)
        );
    }

    #[test]
    fn chat_completion_max_tokens_parameter_defaults_to_max_completion_tokens() {
        let model = available_model(Some(open_ai::ReasoningEffort::Medium));

        assert_eq!(
            chat_completion_max_tokens_parameter(&model),
            crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxCompletionTokens
        );
    }

    #[test]
    fn chat_completion_max_tokens_parameter_uses_max_tokens_when_configured() {
        let mut model = available_model(Some(open_ai::ReasoningEffort::Medium));
        model.capabilities.max_tokens_parameter = true;

        assert_eq!(
            chat_completion_max_tokens_parameter(&model),
            crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxTokens
        );
    }

    #[test]
    fn response_request_includes_reasoning_when_effort_is_configured() {
        let model = available_model(Some(open_ai::ReasoningEffort::High));
        let request = LanguageModelRequest {
            thinking_allowed: true,
            ..Default::default()
        };

        let request = into_open_ai_response(
            request,
            &model.name,
            model.capabilities.parallel_tool_calls,
            model.capabilities.prompt_cache_key,
            model.max_output_tokens,
            default_thinking_reasoning_effort(&model),
            supports_none_reasoning_effort(&model),
        );
        let serialized = serde_json::to_value(request).unwrap();

        assert_eq!(
            serialized["reasoning"],
            json!({ "effort": "high", "summary": "auto" })
        );
        assert_eq!(
            serialized["include"],
            json!(["reasoning.encrypted_content"])
        );
    }

    #[test]
    fn response_request_omits_reasoning_when_effort_is_missing() {
        let model = available_model(None);
        let request = LanguageModelRequest {
            thinking_allowed: true,
            ..Default::default()
        };

        let request = into_open_ai_response(
            request,
            &model.name,
            model.capabilities.parallel_tool_calls,
            model.capabilities.prompt_cache_key,
            model.max_output_tokens,
            default_thinking_reasoning_effort(&model),
            supports_none_reasoning_effort(&model),
        );
        let serialized = serde_json::to_value(request).unwrap();

        assert_eq!(serialized.get("reasoning"), None);
        assert_eq!(serialized.get("include"), None);
    }

    #[test]
    fn chat_completion_request_includes_selected_reasoning_effort() {
        let mut model = available_model(Some(open_ai::ReasoningEffort::Medium));
        model.capabilities.chat_completions = true;
        let request = LanguageModelRequest {
            thinking_allowed: true,
            thinking_effort: Some("high".to_string()),
            ..Default::default()
        };
        let reasoning_effort = chat_completion_reasoning_effort(&request, &model);

        let request = into_open_ai(
            request,
            &model.name,
            model.capabilities.parallel_tool_calls,
            model.capabilities.prompt_cache_key,
            model.max_output_tokens,
            chat_completion_max_tokens_parameter(&model),
            reasoning_effort,
            model.capabilities.interleaved_reasoning,
        )
        .unwrap();
        let serialized = serde_json::to_value(request).unwrap();

        assert_eq!(serialized["reasoning_effort"], json!("high"));
    }

    #[test]
    fn configured_reasoning_effort_supports_none_reasoning_effort() {
        assert!(supports_none_reasoning_effort(&available_model(Some(
            open_ai::ReasoningEffort::Medium
        ))));
        assert!(supports_none_reasoning_effort(&available_model(Some(
            open_ai::ReasoningEffort::None
        ))));
        assert!(!supports_none_reasoning_effort(&available_model(None)));
    }

    #[test]
    fn response_thinking_effort_preserves_explicit_none() {
        let model = available_model(Some(open_ai::ReasoningEffort::None));
        let mut request = LanguageModelRequest {
            thinking_allowed: true,
            thinking_effort: Some("high".to_string()),
            ..Default::default()
        };

        disable_response_thinking_for_none_effort(&mut request, &model);
        assert!(!request.thinking_allowed);
        assert_eq!(request.thinking_effort, None);
    }

    #[test]
    fn fetch_discovered_models_parses_ids() {
        let http_client = FakeHttpClient::create(move |_request| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::from(
                    r#"{"data":[{"id":"gpt-4o"},{"id":"gpt-3.5-turbo"}]}"#,
                ))?)
        });

        let models = smol::block_on(fetch_discovered_models(
            http_client.as_ref(),
            "https://example.com/v1",
            None,
            &CustomHeaders::default(),
        ))
        .unwrap();

        assert_eq!(models.len(), 2);
        // Results are sorted by name.
        assert_eq!(models[0].name, "gpt-3.5-turbo");
        assert_eq!(models[1].name, "gpt-4o");
        assert_eq!(models[0].max_tokens, DISCOVERED_MODEL_DEFAULT_MAX_TOKENS);
    }

    #[test]
    fn fetch_discovered_models_skips_entries_without_id() {
        let http_client = FakeHttpClient::create(move |_request| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::from(
                    r#"{"data":[{"id":"gpt-4o"},{"object":"model"},{"id":"claude-3-opus"}]}"#,
                ))?)
        });

        let models = smol::block_on(fetch_discovered_models(
            http_client.as_ref(),
            "https://example.com/v1",
            None,
            &CustomHeaders::default(),
        ))
        .unwrap();

        let names: Vec<_> = models.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["claude-3-opus", "gpt-4o"]);
    }

    #[test]
    fn fetch_discovered_models_errors_on_non_success() {
        let http_client = FakeHttpClient::create(move |_request| async move {
            Ok(http_client::Response::builder()
                .status(404)
                .body(http_client::AsyncBody::from("not found"))?)
        });

        let result = smol::block_on(fetch_discovered_models(
            http_client.as_ref(),
            "https://example.com/v1",
            None,
            &CustomHeaders::default(),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn fetch_discovered_models_errors_on_invalid_json() {
        let http_client = FakeHttpClient::create(move |_request| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::from("not json"))?)
        });

        let result = smol::block_on(fetch_discovered_models(
            http_client.as_ref(),
            "https://example.com/v1",
            None,
            &CustomHeaders::default(),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn fetch_discovered_models_requests_models_endpoint_with_bearer_auth() {
        let captured: Arc<Mutex<Option<(String, Option<String>)>>> = Arc::new(Mutex::new(None));
        let captured_for_handler = captured.clone();
        let http_client = FakeHttpClient::create(move |request| {
            let captured = captured_for_handler.clone();
            async move {
                *captured.lock().unwrap() = Some((
                    request.uri().to_string(),
                    request
                        .headers()
                        .get("Authorization")
                        .map(|value| value.to_str().unwrap().to_string()),
                ));
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(r#"{"data":[]}"#))?)
            }
        });

        let models = smol::block_on(fetch_discovered_models(
            http_client.as_ref(),
            "https://example.com/v1",
            Some("sk-test"),
            &CustomHeaders::default(),
        ))
        .unwrap();
        assert!(models.is_empty());

        let (uri, auth) = captured.lock().unwrap().take().unwrap();
        assert_eq!(uri, "https://example.com/v1/models");
        assert_eq!(auth.as_deref(), Some("Bearer sk-test"));
    }
}
