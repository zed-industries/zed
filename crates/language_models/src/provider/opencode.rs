use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use fs::Fs;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{App, AsyncApp, Context, Entity, SharedString, Task, TaskExt, Window};
use http_client::{AsyncBody, CustomHeaders, HttpClient, RequestBuilderExt, http};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, InlineDescription, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelEffortLevel,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, ProviderSettingsView, RateLimiter, ReasoningEffort,
    SubPageProviderSettings, env_var,
};
use opencode::{ApiProtocol, OPENCODE_API_URL, OpenCodeSubscription};
pub use settings::OpenCodeApiProtocol;
pub use settings::OpenCodeAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore, update_settings_file};
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::{
    Banner, ButtonLink, ConfiguredApiCard, Divider, List, ListBulletItem, Severity, Switch,
    SwitchLabelPosition, ToggleState, prelude::*,
};
use ui_input::InputField;
use util::ResultExt;

use crate::provider::anthropic::{AnthropicEventMapper, into_anthropic};
use crate::provider::google::{GoogleEventMapper, into_google};
use crate::provider::open_ai::{
    ChatCompletionMaxTokensParameter, OpenAiResponseEventMapper, into_open_ai,
    into_open_ai_response,
};
use language_model::chat_completion::{ChatCompletionEventMapper, ResponseStreamEvent};

fn normalize_reasoning_effort(effort: &str) -> Option<ReasoningEffort> {
    match effort.trim().to_ascii_lowercase().as_str() {
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

fn reasoning_effort_display(effort: ReasoningEffort) -> (&'static str, &'static str) {
    match effort {
        ReasoningEffort::None => ("None", "none"),
        ReasoningEffort::Minimal => ("Minimal", "minimal"),
        ReasoningEffort::Low => ("Low", "low"),
        ReasoningEffort::Medium => ("Medium", "medium"),
        ReasoningEffort::High => ("High", "high"),
        ReasoningEffort::XHigh => ("XHigh", "xhigh"),
        ReasoningEffort::Max => ("Max", "max"),
    }
}

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("opencode");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("OpenCode");

const API_KEY_ENV_VAR_NAME: &str = "OPENCODE_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);
const OPENCODE_SESSION_HEADER_NAME: &str = "x-opencode-session";
pub(crate) const RESERVED_HEADER_NAMES: &[&str] = &[OPENCODE_SESSION_HEADER_NAME];

/// The models.dev registry, which OpenCode keeps up to date with newly
/// released models for its Zen and Go subscriptions.
const MODELS_DEV_API_URL: &str = "https://models.dev/api.json";

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
struct RegistryProvider {
    #[serde(default)]
    npm: Option<String>,
    #[serde(default)]
    models: BTreeMap<String, RegistryModel>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
struct RegistryModel {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    family: Option<String>,
    #[serde(default)]
    reasoning_options: Vec<RegistryReasoningOption>,
    #[serde(default)]
    interleaved: Option<RegistryInterleaved>,
    #[serde(default)]
    modalities: Option<RegistryModalities>,
    #[serde(default)]
    limit: Option<RegistryLimit>,
    #[serde(default)]
    provider: Option<RegistryModelProvider>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
struct RegistryReasoningOption {
    #[serde(rename = "type")]
    kind: Option<String>,
    #[serde(default)]
    values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
struct RegistryInterleaved {
    #[serde(default)]
    field: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
struct RegistryModalities {
    #[serde(default)]
    input: Option<Vec<String>>,
    #[serde(default)]
    output: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
struct RegistryLimit {
    #[serde(default)]
    context: Option<u64>,
    #[serde(default)]
    output: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
struct RegistryModelProvider {
    #[serde(default)]
    npm: Option<String>,
}

impl RegistryModel {
    /// Converts a registry model into a custom OpenCode model, resolving the
    /// API protocol from the registry's per-model SDK, the model family, and
    /// finally the provider-level SDK default.
    fn into_custom(self, id: String, provider_npm: Option<&str>) -> Option<opencode::Model> {
        let protocol = self
            .provider
            .as_ref()
            .and_then(|provider| provider.npm.as_deref())
            .and_then(protocol_from_npm)
            .or_else(|| self.family.as_deref().and_then(protocol_from_family))
            .or_else(|| provider_npm.and_then(protocol_from_npm))
            .unwrap_or(ApiProtocol::OpenAiChat);
        Some(opencode::Model::Custom {
            name: id,
            display_name: self.name,
            max_tokens: self
                .limit
                .as_ref()
                .and_then(|limit| limit.context)
                .unwrap_or(128_000),
            max_output_tokens: self.limit.as_ref().and_then(|limit| limit.output),
            protocol,
            reasoning_effort_levels: registry_reasoning_effort_levels(&self.reasoning_options),
            custom_model_api_url: None,
            interleaved_reasoning: self.interleaved.is_some_and(|interleaved| {
                interleaved.field.as_deref() == Some("reasoning_content")
            }),
        })
    }
}

fn protocol_from_npm(npm: &str) -> Option<ApiProtocol> {
    match npm {
        "@ai-sdk/anthropic" => Some(ApiProtocol::Anthropic),
        "@ai-sdk/openai" => Some(ApiProtocol::OpenAiResponses),
        "@ai-sdk/google" => Some(ApiProtocol::Google),
        "@ai-sdk/openai-compatible" => Some(ApiProtocol::OpenAiChat),
        _ => None,
    }
}

fn protocol_from_family(family: &str) -> Option<ApiProtocol> {
    let family = family.to_ascii_lowercase();
    if family.starts_with("claude") {
        Some(ApiProtocol::Anthropic)
    } else if family.starts_with("gpt") || family.starts_with("grok") || family.starts_with("muse")
    {
        Some(ApiProtocol::OpenAiResponses)
    } else if family.starts_with("gemini") {
        Some(ApiProtocol::Google)
    } else if family.starts_with("qwen") {
        Some(ApiProtocol::Anthropic)
    } else if family.starts_with("deepseek")
        || family.starts_with("glm")
        || family.starts_with("kimi")
        || family.starts_with("minimax")
        || family.starts_with("mimo")
        || family.starts_with("hy")
        || family.starts_with("longcat")
    {
        Some(ApiProtocol::OpenAiChat)
    } else {
        None
    }
}

fn registry_reasoning_effort_levels(
    options: &[RegistryReasoningOption],
) -> Option<Vec<ReasoningEffort>> {
    let effort_levels: Vec<ReasoningEffort> = options
        .iter()
        .filter(|option| option.kind.as_deref() == Some("effort"))
        .flat_map(|option| option.values.iter().flatten())
        .filter_map(|value| normalize_reasoning_effort(value))
        .collect();
    if !effort_levels.is_empty() {
        Some(effort_levels)
    } else if options
        .iter()
        .any(|option| option.kind.as_deref() == Some("toggle"))
    {
        // Toggle-only reasoning models can't express effort levels; enabling
        // reasoning means requesting the highest effort (see Kimi K3).
        Some(vec![ReasoningEffort::Max])
    } else {
        None
    }
}

fn parse_registry_models(body: &str) -> Result<Vec<(OpenCodeSubscription, opencode::Model)>> {
    let registry: serde_json::Value = serde_json::from_str(body)?;
    let mut models = Vec::new();
    for (provider_id, subscription) in [
        ("opencode", OpenCodeSubscription::Zen),
        ("opencode-go", OpenCodeSubscription::Go),
    ] {
        let Some(provider_value) = registry.get(provider_id) else {
            log::warn!("OpenCode model registry is missing the {provider_id} provider");
            continue;
        };
        let provider: RegistryProvider = match serde_json::from_value(provider_value.clone()) {
            Ok(provider) => provider,
            Err(error) => {
                log::warn!(
                    "failed to parse the {provider_id} provider in the OpenCode model registry: {error:?}"
                );
                continue;
            }
        };
        let provider_npm = provider.npm.clone();
        for (id, model) in provider.models {
            if let Some(model) = model.into_custom(id, provider_npm.as_deref()) {
                models.push((subscription, model));
            }
        }
    }
    Ok(models)
}

async fn fetch_registry_models(
    http_client: &dyn HttpClient,
    extra_headers: &CustomHeaders,
) -> Result<Vec<(OpenCodeSubscription, opencode::Model)>> {
    let request = http::Request::builder()
        .method(http::Method::GET)
        .uri(MODELS_DEV_API_URL)
        .header("Content-Type", "application/json")
        .extra_headers(extra_headers)
        .body(AsyncBody::empty())?;
    let mut response = http_client.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "failed to fetch the OpenCode model registry, status code: {:?}, body: {}",
            response.status(),
            body
        ));
    }
    parse_registry_models(&body)
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenCodeSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
    pub show_zen_models: bool,
    pub show_go_models: bool,
    pub fetch_registry_models: bool,
}

pub struct OpenCodeLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    registry_models: Option<Vec<(OpenCodeSubscription, opencode::Model)>>,
    registry_fetch_task: Option<Task<()>>,
    registry_fetch_attempted: bool,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenCodeLanguageModelProvider::api_url(cx);
        self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenCodeLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }

    /// Fetches the models.dev registry so that newly released OpenCode models
    /// show up without a Zed release. Built-in models take precedence; this
    /// only fills in models that Zed doesn't bundle yet.
    fn maybe_fetch_registry_models(&mut self, cx: &mut Context<Self>) {
        if self.registry_fetch_attempted
            || !OpenCodeLanguageModelProvider::settings(cx).fetch_registry_models
        {
            return;
        }
        self.registry_fetch_attempted = true;
        let http_client = self.http_client.clone();
        let extra_headers = OpenCodeLanguageModelProvider::settings(cx)
            .custom_headers
            .clone();
        let task = cx.spawn(async move |this, cx| {
            let registry_models = fetch_registry_models(http_client.as_ref(), &extra_headers).await;
            this.update(cx, |state, cx| {
                state.registry_fetch_task.take();
                match registry_models {
                    Ok(registry_models) => {
                        state.registry_models = Some(registry_models);
                        cx.notify();
                    }
                    Err(error) => {
                        log::warn!("failed to fetch the OpenCode model registry: {error:?}");
                    }
                }
            })
            .ok();
        });
        self.registry_fetch_task = Some(task);
    }
}

impl OpenCodeLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let credentials_provider = this.credentials_provider.clone();
                let api_url = Self::api_url(cx);
                this.api_key_state.handle_url_change(
                    api_url,
                    |this| &mut this.api_key_state,
                    credentials_provider,
                    cx,
                );
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                credentials_provider,
                http_client: http_client.clone(),
                registry_models: None,
                registry_fetch_task: None,
                registry_fetch_attempted: false,
            }
        });
        state.update(cx, |state, cx| state.maybe_fetch_registry_models(cx));

        Self { http_client, state }
    }

    fn create_language_model(
        &self,
        model: opencode::Model,
        subscription: OpenCodeSubscription,
    ) -> Arc<dyn LanguageModel> {
        let id_str = format!("{}/{}", subscription.id_prefix(), model.id());
        Arc::new(OpenCodeLanguageModel {
            id: LanguageModelId::from(id_str),
            model,
            subscription,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    pub fn settings(cx: &App) -> &OpenCodeSettings {
        &crate::AllLanguageModelSettings::get_global(cx).opencode
    }

    fn subscription_enabled(subscription: OpenCodeSubscription, cx: &App) -> bool {
        let settings = Self::settings(cx);
        match subscription {
            OpenCodeSubscription::Zen => settings.show_zen_models,
            OpenCodeSubscription::Go => settings.show_go_models,
        }
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            OPENCODE_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for OpenCodeLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenCodeLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenCode)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        if Self::subscription_enabled(OpenCodeSubscription::Go, cx) {
            // If both Go and Zen are enabled, prefer Go since it's not pay-as-you-go
            Some(
                self.create_language_model(opencode::Model::default_go(), OpenCodeSubscription::Go),
            )
        } else if Self::subscription_enabled(OpenCodeSubscription::Zen, cx) {
            Some(self.create_language_model(opencode::Model::default(), OpenCodeSubscription::Zen))
        } else {
            None
        }
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        if Self::subscription_enabled(OpenCodeSubscription::Go, cx) {
            // If both Go and Zen are enabled, prefer Go since it's not pay-as-you-go
            Some(self.create_language_model(
                opencode::Model::default_go_fast(),
                OpenCodeSubscription::Go,
            ))
        } else if Self::subscription_enabled(OpenCodeSubscription::Zen, cx) {
            Some(
                self.create_language_model(
                    opencode::Model::default_fast(),
                    OpenCodeSubscription::Zen,
                ),
            )
        } else {
            None
        }
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: BTreeMap<String, (opencode::Model, OpenCodeSubscription)> =
            BTreeMap::default();
        let settings = Self::settings(cx);

        for model in opencode::Model::iter() {
            if matches!(model, opencode::Model::Custom { .. }) {
                continue;
            }
            for &subscription in model.available_subscriptions() {
                if Self::subscription_enabled(subscription, cx) {
                    let key = format!("{}/{}", subscription.id_prefix(), model.id());
                    models.insert(key, (model.clone(), subscription));
                }
            }
        }

        if settings.fetch_registry_models {
            if let Some(registry_models) = self.state.read(cx).registry_models.clone() {
                for (subscription, model) in registry_models {
                    if Self::subscription_enabled(subscription, cx) {
                        let key = format!("{}/{}", subscription.id_prefix(), model.id());
                        models.entry(key).or_insert((model, subscription));
                    }
                }
            }
        }

        for model in &settings.available_models {
            let protocol = match model.protocol {
                Some(OpenCodeApiProtocol::Anthropic) => ApiProtocol::Anthropic,
                Some(OpenCodeApiProtocol::OpenAiResponses) => ApiProtocol::OpenAiResponses,
                Some(OpenCodeApiProtocol::OpenAiChat) => ApiProtocol::OpenAiChat,
                Some(OpenCodeApiProtocol::Google) => ApiProtocol::Google,
                None => ApiProtocol::OpenAiChat, // default fallback
            };
            let subscription = match model.subscription {
                Some(settings::OpenCodeModelSubscription::Go) => OpenCodeSubscription::Go,
                Some(settings::OpenCodeModelSubscription::Zen) | None => OpenCodeSubscription::Zen,
            };
            if !Self::subscription_enabled(subscription, cx) {
                continue;
            }
            let custom_model = opencode::Model::Custom {
                name: model.name.clone(),
                display_name: model.display_name.clone(),
                max_tokens: model.max_tokens,
                max_output_tokens: model.max_output_tokens,
                protocol,
                reasoning_effort_levels: model.reasoning_effort_levels.clone(),
                custom_model_api_url: model.custom_model_api_url.clone(),
                interleaved_reasoning: model.interleaved_reasoning,
            };
            let key = format!("{}/{}", subscription.id_prefix(), model.name);
            models.insert(key, (custom_model, subscription));
        }

        models
            .into_values()
            .map(|(model, subscription)| self.create_language_model(model, subscription))
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
        Some(ProviderSettingsView::SubPage(
            SubPageProviderSettings::new(move |window, cx| {
                cx.new(|cx| ConfigurationView::new(state.clone(), window, cx))
                    .into()
            })
            .description(InlineDescription::Text(
                "To use OpenCode models in Zed, you need an API key.".into(),
            )),
        ))
    }
}

pub struct OpenCodeLanguageModel {
    id: LanguageModelId,
    model: opencode::Model,
    subscription: OpenCodeSubscription,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

struct InjectHeaderClient {
    inner: Arc<dyn HttpClient>,
    name: http::HeaderName,
    value: http::HeaderValue,
}

impl HttpClient for InjectHeaderClient {
    fn user_agent(&self) -> Option<&http::HeaderValue> {
        self.inner.user_agent()
    }

    fn proxy(&self) -> Option<&http_client::Url> {
        self.inner.proxy()
    }

    fn send(
        &self,
        mut req: http::Request<AsyncBody>,
    ) -> futures::future::BoxFuture<'static, anyhow::Result<http::Response<AsyncBody>>> {
        req.headers_mut()
            .insert(self.name.clone(), self.value.clone());
        self.inner.send(req)
    }
}

// Standalone requests do not have a conversation ID, but OpenCode requires a
// non-empty session header.
fn opencode_session_header_value(thread_id: Option<&str>) -> http::HeaderValue {
    thread_id
        .filter(|thread_id| !thread_id.is_empty())
        .and_then(|thread_id| http::HeaderValue::from_str(thread_id).ok())
        .unwrap_or_else(|| http::HeaderValue::from(rand::random::<u64>()))
}

impl OpenCodeLanguageModel {
    fn base_api_url(&self, cx: &AsyncApp) -> SharedString {
        // Custom models can override the API URL
        if let opencode::Model::Custom {
            custom_model_api_url: Some(url),
            ..
        } = &self.model
        {
            if !url.is_empty() {
                return url.clone().into();
            }
        }

        // Combine base URL with subscription path suffix
        let base = self
            .state
            .read_with(cx, |_, cx| OpenCodeLanguageModelProvider::api_url(cx));

        let suffix = self.subscription.api_path_suffix();
        let base_str = base.as_ref().trim_end_matches('/');
        format!("{}{}", base_str, suffix).into()
    }

    fn api_key(&self, cx: &AsyncApp) -> Option<Arc<str>> {
        self.state.read_with(cx, |state, cx| {
            let api_url = OpenCodeLanguageModelProvider::api_url(cx);
            state.api_key_state.key(&api_url)
        })
    }

    fn custom_headers(&self, cx: &AsyncApp) -> CustomHeaders {
        self.state.read_with(cx, |_, cx| {
            OpenCodeLanguageModelProvider::settings(cx)
                .custom_headers
                .clone()
        })
    }

    fn stream_anthropic(
        &self,
        request: anthropic::Request,
        http_client: Arc<dyn HttpClient>,
        extra_headers: CustomHeaders,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<anthropic::Event, anthropic::AnthropicError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        // Anthropic crate appends /v1/messages to api_url
        let api_url = self.base_api_url(cx);
        let api_key = self.api_key(cx);

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = anthropic::stream_completion(
                http_client.as_ref(),
                &api_url,
                &api_key,
                request,
                None,
                &extra_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn stream_openai_chat(
        &self,
        request: open_ai::Request,
        http_client: Arc<dyn HttpClient>,
        extra_headers: CustomHeaders,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>>>
    {
        // OpenAI crate appends /chat/completions to api_url, so we pass base + "/v1"
        let base_url = self.base_api_url(cx);
        let api_url: SharedString = format!("{base_url}/v1").into();
        let api_key = self.api_key(cx);
        let provider_name = PROVIDER_NAME.0.to_string();

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = open_ai::stream_completion(
                http_client.as_ref(),
                &provider_name,
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

    fn stream_openai_response(
        &self,
        request: open_ai::responses::Request,
        http_client: Arc<dyn HttpClient>,
        extra_headers: CustomHeaders,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<open_ai::responses::StreamEvent>>>,
    > {
        // Responses crate appends /responses to api_url, so we pass base + "/v1"
        let base_url = self.base_api_url(cx);
        let api_url: SharedString = format!("{base_url}/v1").into();
        let api_key = self.api_key(cx);
        let provider_name = PROVIDER_NAME.0.to_string();

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = open_ai::responses::stream_response(
                http_client.as_ref(),
                &provider_name,
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

    fn stream_google(
        &self,
        request: google_ai::GenerateContentRequest,
        http_client: Arc<dyn HttpClient>,
        extra_headers: CustomHeaders,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<google_ai::GenerateContentResponse>>>,
    > {
        let api_url = self.base_api_url(cx);
        let api_key = self.api_key(cx);

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = opencode::stream_generate_content(
                http_client.as_ref(),
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

impl LanguageModel for OpenCodeLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(format!(
            "{}: {}",
            self.subscription.display_name(),
            self.model.display_name()
        ))
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

    fn supports_thinking(&self) -> bool {
        self.model
            .supported_reasoning_effort_levels()
            .is_some_and(|levels| levels.iter().any(|effort| *effort != ReasoningEffort::None))
    }

    fn supports_disabling_thinking(&self) -> bool {
        self.model
            .supported_reasoning_effort_levels()
            .is_some_and(|levels| levels.contains(&ReasoningEffort::None))
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        self.model
            .supported_reasoning_effort_levels()
            .map(|levels| {
                let levels = levels
                    .into_iter()
                    .filter(|effort| *effort != ReasoningEffort::None)
                    .collect::<Vec<_>>();
                if levels.is_empty() {
                    return Vec::new();
                }
                let default_index = levels.len() - 1;
                levels
                    .into_iter()
                    .enumerate()
                    .map(|(i, effort)| {
                        let (name, value) = reasoning_effort_display(effort);
                        LanguageModelEffortLevel {
                            name: name.into(),
                            value: value.into(),
                            is_default: i == default_index,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto | LanguageModelToolChoice::Any => true,
            LanguageModelToolChoice::None => {
                // Google models don't support None tool choice
                self.model.protocol(self.subscription) != ApiProtocol::Google
            }
        }
    }

    fn telemetry_id(&self) -> String {
        format!(
            "opencode/{}/{}",
            self.subscription.id_prefix(),
            self.model.id()
        )
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count(self.subscription)
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens(self.subscription)
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
        let http_client: Arc<dyn HttpClient> = Arc::new(InjectHeaderClient {
            inner: self.http_client.clone(),
            name: http::HeaderName::from_static(OPENCODE_SESSION_HEADER_NAME),
            value: opencode_session_header_value(request.thread_id.as_deref()),
        });
        let extra_headers = self.custom_headers(cx);

        match self.model.protocol(self.subscription) {
            ApiProtocol::Anthropic => {
                let mode = if self.supports_thinking() && request.thinking_allowed {
                    anthropic::AnthropicModelMode::AdaptiveThinking
                } else {
                    anthropic::AnthropicModelMode::Default
                };
                let anthropic_request = match into_anthropic(
                    request,
                    self.model.id().to_string(),
                    1.0,
                    self.model
                        .max_output_tokens(self.subscription)
                        .unwrap_or(8192),
                    mode,
                    anthropic::completion::AnthropicPromptCacheMode::Automatic,
                    &PROVIDER_ID,
                ) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };
                let stream =
                    self.stream_anthropic(anthropic_request, http_client, extra_headers, cx);
                let executor = cx.background_executor().clone();
                async move {
                    let mapper = AnthropicEventMapper::new(PROVIDER_NAME, PROVIDER_ID);
                    Ok(language_model::stream_in_background(
                        mapper.map_stream(stream.await?).boxed(),
                        executor,
                    ))
                }
                .boxed()
            }
            ApiProtocol::OpenAiChat => {
                let reasoning_effort = if request.thinking_allowed {
                    request
                        .thinking_effort
                        .as_deref()
                        .and_then(normalize_reasoning_effort)
                } else {
                    None
                };
                let openai_request = match into_open_ai(
                    request,
                    self.model.id(),
                    true,
                    false,
                    self.model.max_output_tokens(self.subscription),
                    ChatCompletionMaxTokensParameter::MaxCompletionTokens,
                    reasoning_effort,
                    self.model.interleaved_reasoning(),
                ) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };
                let stream =
                    self.stream_openai_chat(openai_request, http_client, extra_headers, cx);
                let executor = cx.background_executor().clone();
                async move {
                    let mapper = ChatCompletionEventMapper::new();
                    Ok(language_model::stream_in_background(
                        mapper.map_stream(stream.await?).boxed(),
                        executor,
                    ))
                }
                .boxed()
            }
            ApiProtocol::OpenAiResponses => {
                let supports_none_reasoning_effort = self
                    .model
                    .supported_reasoning_effort_levels()
                    .is_some_and(|levels| levels.contains(&ReasoningEffort::None));
                let response_request = match into_open_ai_response(
                    request,
                    self.model.id(),
                    true,
                    false,
                    self.model.max_output_tokens(self.subscription),
                    None,
                    supports_none_reasoning_effort,
                    &PROVIDER_ID,
                ) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };
                let stream =
                    self.stream_openai_response(response_request, http_client, extra_headers, cx);
                let executor = cx.background_executor().clone();
                async move {
                    let mapper = OpenAiResponseEventMapper::new(PROVIDER_ID);
                    Ok(language_model::stream_in_background(
                        mapper.map_stream(stream.await?).boxed(),
                        executor,
                    ))
                }
                .boxed()
            }
            ApiProtocol::Google => {
                let mode = if self.supports_thinking() && request.thinking_allowed {
                    google_ai::GoogleModelMode::Thinking {
                        budget_tokens: None,
                    }
                } else {
                    google_ai::GoogleModelMode::Default
                };
                let google_request = match into_google(request, self.model.id().to_string(), mode) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };
                let stream = self.stream_google(google_request, http_client, extra_headers, cx);
                async move {
                    let mapper = GoogleEventMapper::new();
                    Ok(mapper.map_stream(stream.await?.boxed()).boxed())
                }
                .boxed()
            }
        }
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            InputField::new(window, cx, "sk-00000000000000000000000000000000").label("API key")
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

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
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

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
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn set_subscription_enabled(
        &mut self,
        subscription: OpenCodeSubscription,
        is_enabled: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let fs = <dyn Fs>::global(cx);

        update_settings_file(fs, cx, move |settings, _| {
            let opencode_settings = settings
                .language_models
                .get_or_insert_default()
                .opencode
                .get_or_insert_default();

            match subscription {
                OpenCodeSubscription::Zen => opencode_settings.show_zen_models = Some(is_enabled),
                OpenCodeSubscription::Go => opencode_settings.show_go_models = Some(is_enabled),
            }
        });
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable")
        } else {
            let api_url = OpenCodeLanguageModelProvider::api_url(cx);
            if api_url == OPENCODE_API_URL {
                "API key configured".to_string()
            } else {
                format!("API key configured for {}", api_url)
            }
        };

        let is_editing = self.should_render_editor(cx);

        let api_key_control = if is_editing {
            self.api_key_editor.clone().into_any_element()
        } else {
            ConfiguredApiCard::new("opencode-reset-key", configured_card_label)
                .disabled(env_var_set)
                .when(env_var_set, |this| {
                    this.tooltip_label(format!(
                        "To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."
                    ))
                })
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .into_any_element()
        };

        let api_key_section = v_flex()
            .on_action(cx.listener(Self::save_api_key))
            .child(Label::new(
                "To use OpenCode models in Zed, you need an API key:",
            ).color(Color::Muted))
            .child(
                List::new()
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("Sign in and get your key at").color(Color::Muted))
                            .child(ButtonLink::new(
                                "OpenCode Console",
                                "https://opencode.ai/auth",
                            )),
                    )
                    .when(is_editing, |this| {
                        this.child(ListBulletItem::new(
                            "Paste your API key below and hit enter to start using OpenCode",
                        ).label_color(Color::Muted))
                    }),
            )
            .child(api_key_control)
            .child(
                Label::new(format!(
                    "You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."
                ))
                .size(LabelSize::Small)
                .color(Color::Muted).mt_1p5(),
            )
            .into_any_element();

        if self.load_credentials_task.is_some() {
            Label::new("Loading Credentials…").into_any_element()
        } else {
            let settings = OpenCodeLanguageModelProvider::settings(cx);
            let show_zen = settings.show_zen_models;
            let show_go = settings.show_go_models;

            let subscription_toggles = v_flex()
                .gap_2()
                .child(Label::new("Subscriptions"))
                .child(
                    Switch::new("opencode-show-zen-models", show_zen.into())
                        .full_width(true)
                        .label("Show Zen models")
                        .label_position(SwitchLabelPosition::Start)
                        .on_click(cx.listener(|this, state, window, cx| {
                            this.set_subscription_enabled(
                                OpenCodeSubscription::Zen,
                                matches!(state, ToggleState::Selected),
                                window,
                                cx,
                            );
                        })),
                )
                .child(Divider::horizontal_dashed())
                .child(
                    Switch::new("opencode-show-go-models", show_go.into())
                        .full_width(true)
                        .label("Show Go models")
                        .label_position(SwitchLabelPosition::Start)
                        .on_click(cx.listener(|this, state, window, cx| {
                            this.set_subscription_enabled(
                                OpenCodeSubscription::Go,
                                matches!(state, ToggleState::Selected),
                                window,
                                cx,
                            );
                        })),
                );

            let no_subscriptions_warning = if !show_zen && !show_go {
                Some(Banner::new().severity(Severity::Warning).child(Label::new(
                    "No subscriptions enabled. Enable at least one subscription to use OpenCode.",
                )))
            } else {
                None
            };

            v_flex()
                .size_full()
                .gap_2p5()
                .child(Headline::new("OpenCode").size(HeadlineSize::Small))
                .child(api_key_section)
                .child(Divider::horizontal())
                .child(subscription_toggles)
                .children(no_subscriptions_warning)
                .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collections::HashMap;
    use http_client::{FakeHttpClient, Response};
    use language_model::{LanguageModelRequestMessage, MessageContent, Role};
    use parking_lot::Mutex;

    #[test]
    fn test_opencode_session_header_uses_thread_id() {
        let value = opencode_session_header_value(Some("thread-123"));

        assert_eq!(value, "thread-123");
    }

    #[test]
    fn test_opencode_session_header_without_thread_id() {
        let value = opencode_session_header_value(None);

        assert_generated_session_id(&value);
    }

    #[test]
    fn test_opencode_session_header_with_empty_thread_id() {
        let value = opencode_session_header_value(Some(""));

        assert_generated_session_id(&value);
    }

    #[test]
    fn test_opencode_session_header_with_invalid_thread_id() {
        let value = opencode_session_header_value(Some("thread\n123"));

        assert_generated_session_id(&value);
    }

    #[test]
    fn test_parse_registry_models_resolves_protocols_and_metadata() {
        let body = r#"
        {
          "opencode": {
            "npm": "@ai-sdk/openai-compatible",
            "models": {
              "claude-opus-5": {
                "provider": { "npm": "@ai-sdk/anthropic" },
                "limit": { "context": 1000000, "output": 128000 }
              },
              "glm-5.3": {
                "family": "glm",
                "interleaved": { "field": "reasoning_content" },
                "limit": { "context": 1000000, "output": 131072 }
              }
            }
          },
          "opencode-go": {
            "npm": "@ai-sdk/openai-compatible",
            "models": {
              "kimi-k3": {
                "reasoning_options": [{ "type": "toggle" }],
                "limit": { "context": 1048576, "output": 131072 }
              },
              "qwen3.8-flash": {
                "provider": { "npm": "@ai-sdk/anthropic" },
                "limit": { "context": 1000000 }
              }
            }
          },
          "other-provider": {
            "models": { "should-be-ignored": {} }
          }
        }"#;

        let models = parse_registry_models(body).unwrap();
        assert_eq!(models.len(), 4);
        let by_id: HashMap<_, _> = models
            .iter()
            .map(|(subscription, model)| {
                (
                    format!("{}/{}", subscription.id_prefix(), model.id()),
                    model,
                )
            })
            .collect();

        let claude = by_id["zen/claude-opus-5"].clone();
        let opencode::Model::Custom {
            name,
            max_tokens,
            max_output_tokens,
            protocol,
            interleaved_reasoning,
            ..
        } = claude
        else {
            panic!("registry models should be custom models");
        };
        assert_eq!(name, "claude-opus-5");
        assert_eq!(max_tokens, 1_000_000);
        assert_eq!(max_output_tokens, Some(128_000));
        assert_eq!(protocol, ApiProtocol::Anthropic);
        assert!(!interleaved_reasoning);

        let opencode::Model::Custom {
            protocol,
            interleaved_reasoning,
            ..
        } = &by_id["zen/glm-5.3"]
        else {
            panic!("registry models should be custom models");
        };
        assert_eq!(*protocol, ApiProtocol::OpenAiChat);
        assert!(*interleaved_reasoning);

        let opencode::Model::Custom {
            protocol,
            reasoning_effort_levels,
            ..
        } = &by_id["go/kimi-k3"]
        else {
            panic!("registry models should be custom models");
        };
        assert_eq!(*protocol, ApiProtocol::OpenAiChat);
        assert_eq!(
            reasoning_effort_levels.as_ref().unwrap(),
            &vec![ReasoningEffort::Max]
        );

        let opencode::Model::Custom {
            protocol,
            max_output_tokens,
            ..
        } = &by_id["go/qwen3.8-flash"]
        else {
            panic!("registry models should be custom models");
        };
        assert_eq!(*protocol, ApiProtocol::Anthropic);
        assert_eq!(*max_output_tokens, None);
    }

    #[gpui::test]
    async fn test_stream_completion_sends_session_header_without_thread_id(
        cx: &mut gpui::TestAppContext,
    ) {
        let captured_header = Arc::new(Mutex::new(None));
        let http_client = FakeHttpClient::create({
            let captured_header = captured_header.clone();
            move |request| {
                let captured_header = captured_header.clone();
                async move {
                    *captured_header.lock() =
                        Some(request.headers().get(OPENCODE_SESSION_HEADER_NAME).cloned());
                    Ok(Response::builder().status(200).body(AsyncBody::from(
                        "event: message_stop\ndata: {\"type\":\"message_stop\"}\n\n",
                    ))?)
                }
            }
        });
        let provider = cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            OpenCodeLanguageModelProvider::new(http_client, Arc::new(TestCredentialsProvider), cx)
        });
        let store_key = provider.state.update(cx, |state, cx| {
            state.set_api_key(Some("test-key".to_string()), cx)
        });
        store_key.await.unwrap();
        let model =
            provider.create_language_model(opencode::Model::default(), OpenCodeSubscription::Go);
        let request = LanguageModelRequest {
            thread_id: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".to_string())],
                cache: false,
                reasoning_details: None,
            }],
            ..Default::default()
        };

        let stream = model
            .stream_completion(request, &cx.to_async())
            .await
            .unwrap();
        drop(stream);

        let captured_header = captured_header
            .lock()
            .take()
            .expect("request should reach the http client")
            .expect("request should carry the session header");
        assert_generated_session_id(&captured_header);
    }

    fn assert_generated_session_id(value: &http::HeaderValue) {
        value
            .to_str()
            .unwrap()
            .parse::<u64>()
            .expect("generated session id should be a u64");
    }

    struct TestCredentialsProvider;

    impl CredentialsProvider for TestCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>,
        > {
            Box::pin(async { Ok(None) })
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            _username: &'a str,
            _password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }
    }
}
