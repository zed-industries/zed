use anyhow::{Context as _, Result, anyhow, bail};
use collections::{BTreeMap, HashMap};
use credentials_provider::CredentialsProvider;
use fs::Fs;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{
    App, AsyncApp, BackgroundExecutor, Context, Entity, SharedString, Task, TaskExt, Window,
};
use http_client::{AsyncBody, CustomHeaders, HttpClient, Method, Request, http};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, InlineDescription, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelEffortLevel,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, ProviderSettingsView, RateLimiter, ReasoningEffort,
    SubPageProviderSettings, env_var,
};
use opencode::{ApiProtocol, OPENCODE_API_URL, OpenCodeSubscription};
use serde::Deserialize;
pub use settings::OpenCodeApiProtocol;
pub use settings::OpenCodeAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore, update_settings_file};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
    time::{Duration, SystemTime},
};
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

fn model_supports_thinking(model: &opencode::Model) -> bool {
    model
        .supported_reasoning_effort_levels()
        .is_some_and(|levels| levels.iter().any(|effort| *effort != ReasoningEffort::None))
}

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("opencode");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("OpenCode");

const API_KEY_ENV_VAR_NAME: &str = "OPENCODE_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);
const OPENCODE_SESSION_HEADER_NAME: &str = "x-opencode-session";
const RICH_MODEL_CATALOG_URL: &str = "https://models.opencode.ai/api.json";
const MODEL_CATALOG_RESPONSE_LIMIT_BYTES: u64 = 8 * 1024 * 1024;
const MODEL_CATALOG_FETCH_TIMEOUT: Duration = Duration::from_secs(15);
const MODEL_CATALOG_DISK_FRESHNESS: Duration = Duration::from_secs(5 * 60);
const MODEL_CATALOG_REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 60);
pub(crate) const RESERVED_HEADER_NAMES: &[&str] = &[OPENCODE_SESSION_HEADER_NAME];

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenCodeSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
    pub show_zen_models: bool,
    pub show_go_models: bool,
}

pub struct OpenCodeLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    discovered_models: HashMap<OpenCodeSubscription, Vec<DiscoveredModel>>,
    fetch_models_errors: HashMap<OpenCodeSubscription, SharedString>,
    fetch_models_task: Option<Task<()>>,
    refresh_models_task: Option<Task<()>>,
    cache_path: PathBuf,
    fs: Arc<dyn Fs>,
}

#[derive(Clone)]
struct DiscoveredModel {
    model: opencode::Model,
    supports_images: bool,
    supports_thinking: bool,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenCodeLanguageModelProvider::api_url(cx);
        self.fetch_models_task = None;
        self.refresh_models_task = None;
        self.discovered_models.clear();
        self.fetch_models_errors.clear();
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = task.await;
            if result.is_ok() {
                this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                    .ok();
            }
            result
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenCodeLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );
        cx.spawn(async move |this, cx| {
            let result = task.await;
            if result.is_ok() {
                this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                    .ok();
            }
            result
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        self.fetch_models_task = None;
        self.refresh_models_task = None;
        let api_url = OpenCodeLanguageModelProvider::api_url(cx);
        if self.api_key_state.key(&api_url).is_none() {
            self.fetch_models_task = None;
            cx.notify();
            return;
        }
        let http_client = self.http_client.clone();
        let fs = self.fs.clone();
        let cache_path = self.cache_path.clone();
        let executor = cx.background_executor().clone();
        self.fetch_models_task = Some(cx.spawn(async move |this, cx| {
            let cached_catalog = load_cached_catalog(fs.as_ref(), &cache_path).await;
            if let Ok(Some((catalog, _))) = &cached_catalog {
                let results = catalog_results(Ok(catalog));
                this.update(cx, |this, cx| {
                    apply_discovery_results(
                        &mut this.discovered_models,
                        &mut this.fetch_models_errors,
                        results,
                    );
                    cx.notify();
                })
                .ok();
            }

            if cached_catalog
                .as_ref()
                .ok()
                .and_then(|catalog| catalog.as_ref())
                .is_some_and(|(_, is_fresh)| *is_fresh)
            {
                this.update(cx, |this, _| this.fetch_models_task = None)
                    .ok();
                return;
            }

            let catalog = fetch_model_catalog(http_client, executor).await;
            let results = catalog_results(catalog.as_ref().map(String::as_str));
            if results.iter().all(|(_, result)| result.is_ok())
                && let Ok(catalog) = catalog
                && let Err(error) = store_cached_catalog(fs.as_ref(), cache_path, catalog).await
            {
                log::warn!("Failed to cache OpenCode model catalog: {error:#}");
            }
            this.update(cx, |this, cx| {
                apply_discovery_results(
                    &mut this.discovered_models,
                    &mut this.fetch_models_errors,
                    results,
                );
                this.fetch_models_task = None;
                cx.notify();
            })
            .ok();
        }));

        let http_client = self.http_client.clone();
        let fs = self.fs.clone();
        let cache_path = self.cache_path.clone();
        let executor = cx.background_executor().clone();
        self.refresh_models_task = Some(cx.spawn(async move |this, cx| {
            loop {
                executor.timer(MODEL_CATALOG_REFRESH_INTERVAL).await;
                let catalog = fetch_model_catalog(http_client.clone(), executor.clone()).await;
                let results = catalog_results(catalog.as_ref().map(String::as_str));
                if results.iter().all(|(_, result)| result.is_ok())
                    && let Ok(catalog) = catalog
                    && let Err(error) =
                        store_cached_catalog(fs.as_ref(), cache_path.clone(), catalog).await
                {
                    log::warn!("Failed to cache OpenCode model catalog: {error:#}");
                }
                if this
                    .update(cx, |this, cx| {
                        apply_discovery_results(
                            &mut this.discovered_models,
                            &mut this.fetch_models_errors,
                            results,
                        );
                        cx.notify();
                    })
                    .is_err()
                {
                    break;
                }
            }
        }));
    }
}

fn catalog_results(
    catalog: Result<&str, &anyhow::Error>,
) -> [(OpenCodeSubscription, Result<Vec<DiscoveredModel>>); 2] {
    [OpenCodeSubscription::Zen, OpenCodeSubscription::Go].map(|subscription| {
        let result = catalog
            .map_err(|error| anyhow!("{error:#}"))
            .and_then(|catalog| parse_discovered_models(catalog, subscription));
        if let Err(error) = &result {
            log::warn!(
                "Failed to fetch OpenCode {} models: {error:#}",
                subscription.display_name()
            );
        }
        (subscription, result)
    })
}

async fn load_cached_catalog(fs: &dyn Fs, path: &Path) -> Result<Option<(String, bool)>> {
    let Some(metadata) = fs.metadata(path).await? else {
        return Ok(None);
    };
    let catalog = fs.load(path).await?;
    for subscription in [OpenCodeSubscription::Zen, OpenCodeSubscription::Go] {
        parse_discovered_models(&catalog, subscription)?;
    }
    let is_fresh = SystemTime::now()
        .duration_since(metadata.mtime.timestamp_for_user())
        .is_ok_and(|age| age < MODEL_CATALOG_DISK_FRESHNESS);
    Ok(Some((catalog, is_fresh)))
}

async fn store_cached_catalog(fs: &dyn Fs, path: PathBuf, catalog: String) -> Result<()> {
    let mut catalog = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&catalog)?;
    catalog.retain(|provider, _| matches!(provider.as_str(), "opencode" | "opencode-go"));
    if let Some(parent) = path.parent() {
        fs.create_dir(parent).await?;
    }
    fs.atomic_write(path, serde_json::to_string(&catalog)?)
        .await
}

fn apply_discovery_results(
    discovered_models: &mut HashMap<OpenCodeSubscription, Vec<DiscoveredModel>>,
    fetch_models_errors: &mut HashMap<OpenCodeSubscription, SharedString>,
    results: [(OpenCodeSubscription, Result<Vec<DiscoveredModel>>); 2],
) {
    for (subscription, result) in results {
        match result {
            Ok(models) => {
                discovered_models.insert(subscription, models);
                fetch_models_errors.remove(&subscription);
            }
            Err(error) => {
                fetch_models_errors.insert(subscription, format!("{error:#}").into());
            }
        }
    }
}

impl OpenCodeLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        Self::new_with_cache(
            http_client,
            credentials_provider,
            <dyn Fs>::global(cx),
            paths::temp_dir().join("opencode").join("models.json"),
            cx,
        )
    }

    fn new_with_cache(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        fs: Arc<dyn Fs>,
        cache_path: PathBuf,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>({
                let mut last_api_url = Self::api_url(cx);
                move |this: &mut State, cx| {
                    let api_url = Self::api_url(cx);
                    if api_url != last_api_url {
                        last_api_url = api_url;
                        this.discovered_models.clear();
                        this.fetch_models_errors.clear();
                        this.fetch_models_task = None;
                        this.refresh_models_task = None;
                        this.authenticate(cx).detach();
                    }
                    cx.notify();
                }
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                credentials_provider,
                http_client: http_client.clone(),
                discovered_models: HashMap::default(),
                fetch_models_errors: HashMap::default(),
                fetch_models_task: None,
                refresh_models_task: None,
                cache_path,
                fs,
            }
        });

        Self { http_client, state }
    }

    fn create_language_model_with_capabilities(
        &self,
        model: opencode::Model,
        subscription: OpenCodeSubscription,
        supports_images: bool,
        supports_thinking: bool,
    ) -> Arc<dyn LanguageModel> {
        let id_str = format!("{}/{}", subscription.id_prefix(), model.id());
        Arc::new(OpenCodeLanguageModel {
            id: LanguageModelId::from(id_str),
            model,
            subscription,
            supports_images,
            supports_thinking,
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

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: BTreeMap<String, (DiscoveredModel, OpenCodeSubscription)> =
            BTreeMap::default();
        let settings = Self::settings(cx);

        let discovered_models = &self.state.read(cx).discovered_models;
        for subscription in [OpenCodeSubscription::Zen, OpenCodeSubscription::Go] {
            if Self::subscription_enabled(subscription, cx) {
                if let Some(discovered) = discovered_models.get(&subscription) {
                    for model in discovered {
                        let key = format!("{}/{}", subscription.id_prefix(), model.model.id());
                        models.insert(key, (model.clone(), subscription));
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
            let custom_model = opencode::Model::new(
                model.name.clone(),
                model.display_name.clone(),
                model.max_tokens,
                model.max_output_tokens,
                protocol,
                model.reasoning_effort_levels.clone(),
                model.custom_model_api_url.clone(),
                model.interleaved_reasoning,
            );
            let key = format!("{}/{}", subscription.id_prefix(), model.name);
            models.insert(
                key,
                (
                    DiscoveredModel {
                        supports_images: true,
                        supports_thinking: model_supports_thinking(&custom_model),
                        model: custom_model,
                    },
                    subscription,
                ),
            );
        }

        models
            .into_values()
            .map(|(model, subscription)| {
                self.create_language_model_with_capabilities(
                    model.model,
                    subscription,
                    model.supports_images,
                    model.supports_thinking,
                )
            })
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
    supports_images: bool,
    supports_thinking: bool,
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
        if let Some(url) = self.model.custom_model_api_url() {
            if !url.is_empty() {
                return url.to_string().into();
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
        self.supports_images
    }

    fn supports_thinking(&self) -> bool {
        self.supports_thinking
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
                    .iter()
                    .copied()
                    .filter(|effort| *effort != ReasoningEffort::None)
                    .collect::<Vec<_>>();
                if levels.is_empty() {
                    return Vec::new();
                }
                let default_index = if levels.contains(&ReasoningEffort::Max) {
                    [
                        ReasoningEffort::High,
                        ReasoningEffort::Medium,
                        ReasoningEffort::Low,
                        ReasoningEffort::Minimal,
                    ]
                    .into_iter()
                    .find_map(|effort| levels.iter().position(|level| *level == effort))
                    .unwrap_or(levels.len() - 1)
                } else {
                    levels.len() - 1
                };
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
                self.model.protocol() != ApiProtocol::Google
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
        let http_client: Arc<dyn HttpClient> = Arc::new(InjectHeaderClient {
            inner: self.http_client.clone(),
            name: http::HeaderName::from_static(OPENCODE_SESSION_HEADER_NAME),
            value: opencode_session_header_value(request.thread_id.as_deref()),
        });
        let extra_headers = self.custom_headers(cx);

        match self.model.protocol() {
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
                    self.model.max_output_tokens().unwrap_or(8192),
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
                    self.model.max_output_tokens(),
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
                    self.model.max_output_tokens(),
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
                let mut request = request;
                if request.max_output_tokens.is_some() {
                    request.max_output_tokens =
                        request.effective_max_output_tokens(self.max_output_tokens());
                }
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

#[derive(Deserialize)]
struct RichCatalogProvider {
    npm: Option<String>,
    models: HashMap<String, RichCatalogModel>,
}

#[derive(Deserialize)]
struct RichCatalogModel {
    id: String,
    name: String,
    #[serde(default)]
    reasoning: bool,
    #[serde(default)]
    reasoning_options: Option<Vec<ReasoningOption>>,
    #[serde(default)]
    tool_call: bool,
    #[serde(default)]
    interleaved: serde_json::Value,
    #[serde(default)]
    modalities: ModelModalities,
    limit: ModelLimits,
    provider: Option<ModelTransport>,
    cost: Option<ModelCost>,
    status: Option<String>,
}

#[derive(Deserialize)]
struct ModelCost {
    input: Option<f64>,
    output: Option<f64>,
}

#[derive(Default, Deserialize)]
struct ModelModalities {
    #[serde(default)]
    input: Vec<String>,
    #[serde(default)]
    output: Vec<String>,
}

#[derive(Deserialize)]
struct ModelLimits {
    context: u64,
    output: Option<u64>,
}

#[derive(Deserialize)]
struct ModelTransport {
    npm: String,
}

#[derive(Deserialize)]
struct ReasoningOption {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    values: Vec<String>,
}

fn parse_discovered_models(
    rich_catalog: &str,
    subscription: OpenCodeSubscription,
) -> Result<Vec<DiscoveredModel>> {
    let provider_key = match subscription {
        OpenCodeSubscription::Zen => "opencode",
        OpenCodeSubscription::Go => "opencode-go",
    };
    let rich_catalog = serde_json::from_str::<serde_json::Value>(rich_catalog)?;
    let provider = rich_catalog
        .get(provider_key)
        .cloned()
        .ok_or_else(|| anyhow!("OpenCode catalog is missing {provider_key}"))?;
    let provider = serde_json::from_value::<RichCatalogProvider>(provider)?;
    let mut compatible_candidate_count = provider.models.len();
    let models = provider
        .models
        .into_iter()
        .filter_map(|(catalog_id, metadata)| {
            if metadata.status.as_deref() == Some("deprecated")
                || (subscription == OpenCodeSubscription::Zen
                    && metadata
                        .cost
                        .as_ref()
                        .is_some_and(|cost| cost.input == Some(0.0) && cost.output == Some(0.0)))
            {
                compatible_candidate_count -= 1;
                return None;
            }
            if metadata.id != catalog_id
                || !metadata.tool_call
                || !metadata
                    .modalities
                    .output
                    .iter()
                    .any(|value| value == "text")
                || metadata.limit.context == 0
            {
                return None;
            }
            let transport = metadata
                .provider
                .as_ref()
                .map(|provider| provider.npm.as_str())
                .or(provider.npm.as_deref());
            let protocol = transport.and_then(protocol_for_transport)?;
            let reasoning_effort_levels = metadata
                .reasoning
                .then(|| {
                    metadata
                        .reasoning_options
                        .as_deref()
                        .unwrap_or_default()
                        .iter()
                        .find(|option| option.kind == "effort")
                        .into_iter()
                        .flat_map(|option| &option.values)
                        .filter_map(|effort| normalize_reasoning_effort(effort))
                        .collect::<Vec<_>>()
                })
                .filter(|levels| !levels.is_empty());
            Some(DiscoveredModel {
                model: opencode::Model::new(
                    metadata.id.clone(),
                    Some(metadata.name.clone()),
                    metadata.limit.context,
                    metadata.limit.output,
                    protocol,
                    reasoning_effort_levels,
                    None,
                    metadata.interleaved == serde_json::Value::Bool(true)
                        || metadata.interleaved.is_object(),
                ),
                supports_images: metadata
                    .modalities
                    .input
                    .iter()
                    .any(|value| value == "image"),
                supports_thinking: metadata.reasoning,
            })
        })
        .collect::<Vec<_>>();
    if compatible_candidate_count > 0 && models.is_empty() {
        bail!("OpenCode model metadata did not contain any compatible models");
    }
    Ok(models)
}

fn protocol_for_transport(transport: &str) -> Option<ApiProtocol> {
    match transport {
        "@ai-sdk/anthropic" => Some(ApiProtocol::Anthropic),
        "@ai-sdk/openai" => Some(ApiProtocol::OpenAiResponses),
        "@ai-sdk/openai-compatible" => Some(ApiProtocol::OpenAiChat),
        "@ai-sdk/google" => Some(ApiProtocol::Google),
        _ => None,
    }
}

async fn fetch_model_catalog(
    client: Arc<dyn HttpClient>,
    executor: BackgroundExecutor,
) -> Result<String> {
    let fetch = fetch_catalog_resource(client.as_ref(), RICH_MODEL_CATALOG_URL).fuse();
    let timeout = executor.timer(MODEL_CATALOG_FETCH_TIMEOUT).fuse();
    futures::pin_mut!(fetch, timeout);
    futures::select_biased! {
        result = fetch => result,
        _ = timeout => bail!("OpenCode model catalog request timed out"),
    }
}

async fn fetch_catalog_resource(client: &dyn HttpClient, url: &str) -> Result<String> {
    // Catalogs are public; completion credentials and custom headers must not be sent here.
    let request = Request::builder().method(Method::GET).uri(url);
    let mut response = client
        .send(request.body(AsyncBody::empty())?)
        .await
        .with_context(|| format!("requesting OpenCode catalog at {url}"))?;
    if !response.status().is_success() {
        bail!(
            "OpenCode catalog request to {url} returned {}",
            response.status()
        );
    }
    let mut body = String::new();
    response
        .body_mut()
        .take(MODEL_CATALOG_RESPONSE_LIMIT_BYTES)
        .read_to_string(&mut body)
        .await?;
    Ok(body)
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
            let fetch_models_warnings =
                self.state
                    .read(cx)
                    .fetch_models_errors
                    .iter()
                    .map(|(subscription, error)| {
                        Banner::new()
                            .severity(Severity::Error)
                            .child(Label::new(format!(
                                "Failed to load OpenCode {} models: {error}",
                                subscription.display_name()
                            )))
                    });

            v_flex()
                .size_full()
                .gap_2p5()
                .child(Headline::new("OpenCode").size(HeadlineSize::Small))
                .child(api_key_section)
                .child(Divider::horizontal())
                .child(subscription_toggles)
                .children(no_subscriptions_warning)
                .children(fetch_models_warnings)
                .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use http_client::{FakeHttpClient, Response};
    use language_model::{LanguageModelRequestMessage, MessageContent, Role};
    use parking_lot::Mutex;

    fn test_catalog() -> &'static str {
        r#"{
            "opencode": {
                "npm": "@ai-sdk/anthropic",
                "models": {
                    "zen-model": {
                        "id": "zen-model",
                        "name": "Zen Model",
                        "tool_call": true,
                        "modalities": {"input":["text"],"output":["text"]},
                        "limit": {"context": 12345}
                    }
                }
            },
            "opencode-go": {
                "npm": "@ai-sdk/openai-compatible",
                "models": {
                    "go-model": {
                        "id": "go-model",
                        "name": "Go Model",
                        "tool_call": true,
                        "modalities": {"input":["text"],"output":["text"]},
                        "limit": {"context": 12345}
                    }
                }
            }
        }"#
    }

    #[gpui::test]
    async fn test_disk_cache_and_hourly_refresh(cx: &mut gpui::TestAppContext) {
        for (cached, stale, fail, initial_requests) in [
            (Some(test_catalog()), false, false, 0),
            (Some("not json"), false, false, 1),
            (Some(test_catalog()), true, true, 1),
            (None, false, false, 1),
        ] {
            let fs = FakeFs::new(cx.background_executor.clone());
            let cache_path = PathBuf::from("/cache/opencode/models.json");
            if let Some(cached) = cached {
                fs.create_dir(Path::new("/cache/opencode")).await.unwrap();
                fs.set_next_mtime(if stale {
                    SystemTime::now() - MODEL_CATALOG_REFRESH_INTERVAL
                } else {
                    SystemTime::now() - Duration::from_secs(1)
                });
                fs.atomic_write(cache_path.clone(), cached.to_string())
                    .await
                    .unwrap();
            }
            let requests = Arc::new(Mutex::new(0));
            let fail_request = Arc::new(Mutex::new(fail));
            let client = FakeHttpClient::create({
                let requests = requests.clone();
                let fail_request = fail_request.clone();
                move |_| {
                    *requests.lock() += 1;
                    let status = if *fail_request.lock() { 503 } else { 200 };
                    async move {
                        Ok(Response::builder().status(status).body(AsyncBody::from(
                            test_catalog().replace("zen-model", "updated-model"),
                        ))?)
                    }
                }
            });
            let provider = cx.update(|cx| {
                let settings_store = SettingsStore::test(cx);
                cx.set_global(settings_store);
                OpenCodeLanguageModelProvider::new_with_cache(
                    client,
                    Arc::new(TestCredentialsProvider),
                    fs.clone(),
                    cache_path.clone(),
                    cx,
                )
            });
            provider
                .state
                .update(cx, |state, cx| {
                    state.set_api_key(Some("secret".to_string()), cx)
                })
                .await
                .unwrap();
            cx.run_until_parked();
            assert_eq!(*requests.lock(), initial_requests);
            let expected = if initial_requests == 0 || fail {
                "zen-model"
            } else {
                "updated-model"
            };
            provider.state.read_with(cx, |state, _cx| {
                assert_eq!(
                    state.discovered_models[&OpenCodeSubscription::Zen][0]
                        .model
                        .id(),
                    expected,
                );
            });
            if fail {
                assert_eq!(fs.load(&cache_path).await.unwrap(), test_catalog());
            }

            *fail_request.lock() = false;
            cx.background_executor
                .advance_clock(MODEL_CATALOG_REFRESH_INTERVAL);
            cx.run_until_parked();
            assert_eq!(*requests.lock(), initial_requests + 1);
            provider.state.read_with(cx, |state, _cx| {
                assert_eq!(
                    state.discovered_models[&OpenCodeSubscription::Zen][0]
                        .model
                        .id(),
                    "updated-model",
                );
                assert!(state.fetch_models_errors.is_empty());
            });
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&fs.load(&cache_path).await.unwrap())
                    .unwrap(),
                serde_json::from_str::<serde_json::Value>(
                    &test_catalog().replace("zen-model", "updated-model")
                )
                .unwrap(),
            );
            drop(provider);
            cx.run_until_parked();
        }
    }

    #[gpui::test]
    async fn test_fresh_disk_catalog_is_usable(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background_executor.clone());
        let cache_path = PathBuf::from("/cache/models.json");
        fs.set_next_mtime(SystemTime::now());
        fs.atomic_write(cache_path.clone(), test_catalog().to_string())
            .await
            .unwrap();

        let (catalog, is_fresh) = load_cached_catalog(fs.as_ref(), &cache_path)
            .await
            .unwrap()
            .unwrap();

        assert!(is_fresh);
        assert_eq!(catalog, test_catalog());
    }

    #[gpui::test]
    async fn test_invalid_disk_catalog_falls_back_to_network(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background_executor.clone());
        let cache_path = PathBuf::from("/cache/models.json");
        fs.atomic_write(cache_path.clone(), "not json".to_string())
            .await
            .unwrap();

        assert!(load_cached_catalog(fs.as_ref(), &cache_path).await.is_err());
        assert_eq!(fs.load(&cache_path).await.unwrap(), "not json");
    }

    #[gpui::test]
    async fn test_credential_change_invalidates_discovery_before_storing_key(
        cx: &mut gpui::TestAppContext,
    ) {
        let provider = cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            OpenCodeLanguageModelProvider::new_with_cache(
                FakeHttpClient::with_404_response(),
                Arc::new(TestCredentialsProvider),
                FakeFs::new(cx.background_executor().clone()),
                PathBuf::from("/cache/models.json"),
                cx,
            )
        });
        provider
            .state
            .update(cx, |state, cx| {
                state.set_api_key(Some("old-key".to_string()), cx)
            })
            .await
            .unwrap();
        provider.state.update(cx, |state, _cx| {
            state.discovered_models.insert(
                OpenCodeSubscription::Zen,
                vec![DiscoveredModel {
                    model: test_model("old-model", ApiProtocol::Anthropic),
                    supports_images: true,
                    supports_thinking: false,
                }],
            );
        });

        let store_task = provider.state.update(cx, |state, cx| {
            let task = state.set_api_key(Some("new-key".to_string()), cx);
            assert!(state.fetch_models_task.is_none());
            assert!(state.discovered_models.is_empty());
            task
        });

        store_task.await.unwrap();
    }

    #[test]
    fn test_refresh_failure_preserves_last_success_per_subscription() {
        let mut discovered_models = HashMap::default();
        discovered_models.insert(
            OpenCodeSubscription::Zen,
            vec![DiscoveredModel {
                model: test_model("old-zen", ApiProtocol::Anthropic),
                supports_images: true,
                supports_thinking: false,
            }],
        );
        discovered_models.insert(
            OpenCodeSubscription::Go,
            vec![DiscoveredModel {
                model: test_model("old-go", ApiProtocol::OpenAiChat),
                supports_images: false,
                supports_thinking: false,
            }],
        );
        let mut errors = HashMap::default();
        let new_go_model = DiscoveredModel {
            model: test_model("new-go", ApiProtocol::OpenAiChat),
            supports_images: true,
            supports_thinking: true,
        };

        apply_discovery_results(
            &mut discovered_models,
            &mut errors,
            [
                (OpenCodeSubscription::Zen, Err(anyhow!("refresh failed"))),
                (OpenCodeSubscription::Go, Ok(vec![new_go_model])),
            ],
        );

        assert_eq!(
            discovered_models[&OpenCodeSubscription::Zen][0].model.id(),
            "old-zen"
        );
        assert_eq!(
            discovered_models[&OpenCodeSubscription::Go][0].model.id(),
            "new-go"
        );
        assert!(errors.contains_key(&OpenCodeSubscription::Zen));
        assert!(!errors.contains_key(&OpenCodeSubscription::Go));
    }

    #[gpui::test]
    fn test_default_reasoning_effort(cx: &mut gpui::TestAppContext) {
        use ReasoningEffort::{High, Low, Max, Medium, Minimal, XHigh};

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            let provider = OpenCodeLanguageModelProvider::new_with_cache(
                FakeHttpClient::with_404_response(),
                Arc::new(TestCredentialsProvider),
                FakeFs::new(cx.background_executor().clone()),
                PathBuf::from("/cache/models.json"),
                cx,
            );
            for (levels, expected) in [
                (vec![Low, Medium, High, XHigh, Max], Some("high")),
                (vec![Max, High, Medium], Some("high")),
                (vec![Low, Medium, Max], Some("medium")),
                (vec![Minimal, Low, Max], Some("low")),
                (vec![Minimal, Max], Some("minimal")),
                (vec![Low, High, XHigh], Some("xhigh")),
                (vec![High, Low], Some("low")),
                (vec![Max], Some("max")),
                (vec![ReasoningEffort::None], None),
                (vec![], None),
            ] {
                let model = provider.create_language_model_with_capabilities(
                    opencode::Model::new(
                        "test-model".to_string(),
                        None,
                        1000,
                        None,
                        ApiProtocol::OpenAiChat,
                        Some(levels),
                        None,
                        false,
                    ),
                    OpenCodeSubscription::Zen,
                    false,
                    true,
                );
                assert_eq!(
                    model
                        .default_effort_level()
                        .map(|effort| effort.value.to_string())
                        .as_deref(),
                    expected,
                );
            }
        });
    }

    #[test]
    fn test_none_only_reasoning_does_not_enable_thinking() {
        let model = opencode::Model::new(
            "none-only".to_string(),
            None,
            1000,
            None,
            ApiProtocol::OpenAiChat,
            Some(vec![ReasoningEffort::None]),
            None,
            false,
        );

        assert!(!model_supports_thinking(&model));
    }

    #[test]
    fn test_discovery_excludes_only_explicitly_free_zen_models() -> Result<()> {
        for (cost, zen_model_count) in [
            (serde_json::json!({"input": 0, "output": 0}), 0),
            (serde_json::json!({"input": 0, "output": 1}), 1),
            (serde_json::json!({"input": 1, "output": 0}), 1),
            (serde_json::json!({"input": 1, "output": 1}), 1),
            (serde_json::json!({"input": 0}), 1),
            (serde_json::json!({"output": 0}), 1),
            (serde_json::json!({}), 1),
            (serde_json::Value::Null, 1),
        ] {
            let provider = serde_json::json!({
                "npm": "@ai-sdk/openai-compatible",
                "models": {
                    "model": {
                        "id": "model",
                        "name": "Model",
                        "tool_call": true,
                        "modalities": {"output": ["text"]},
                        "limit": {"context": 1000},
                        "cost": cost,
                    },
                },
            });
            let catalog = serde_json::json!({
                "opencode": provider,
                "opencode-go": provider,
            })
            .to_string();
            for (subscription, expected_count) in [
                (OpenCodeSubscription::Zen, zen_model_count),
                (OpenCodeSubscription::Go, 1),
            ] {
                let models = parse_discovered_models(&catalog, subscription)?;
                assert_eq!(models.len(), expected_count, "{subscription:?}: {cost}");
            }
        }
        Ok(())
    }

    #[test]
    fn test_discovery_filters_deprecated_status_per_subscription() -> Result<()> {
        for (zen_status, go_status, zen_count, go_count) in [
            (Some("deprecated"), None, 0, 1),
            (None, Some("deprecated"), 1, 0),
            (Some("active"), Some("preview"), 1, 1),
            (Some("deprecated"), Some("deprecated"), 0, 0),
        ] {
            let mut catalog = serde_json::json!({});
            for (provider_key, status) in [("opencode", zen_status), ("opencode-go", go_status)] {
                let mut model = serde_json::json!({
                    "id": "model",
                    "name": "Model",
                    "tool_call": true,
                    "modalities": {"output": ["text"]},
                    "limit": {"context": 1000},
                });
                if let Some(status) = status {
                    model["status"] = status.into();
                }
                catalog[provider_key] = serde_json::json!({
                    "npm": "@ai-sdk/openai-compatible",
                    "models": {"model": model},
                });
            }
            for (subscription, expected_count) in [
                (OpenCodeSubscription::Zen, zen_count),
                (OpenCodeSubscription::Go, go_count),
            ] {
                let models = parse_discovered_models(&catalog.to_string(), subscription)?;
                assert_eq!(models.len(), expected_count, "{subscription:?}: {catalog}",);
            }
        }
        Ok(())
    }

    #[test]
    fn test_parse_discovered_models_uses_catalog_entries_directly() {
        let models = parse_discovered_models(
            r#"{
                "opencode-go": {
                    "npm": "@ai-sdk/openai-compatible",
                    "models": {
                        "new-model": {
                            "id": "new-model",
                            "name": "New Model",
                            "reasoning": true,
                            "reasoning_options": [{"type":"effort","values":["low","high"]}],
                            "tool_call": true,
                            "interleaved": true,
                            "modalities": {"input":["text","image"],"output":["text"]},
                            "limit": {"context": 12345,"output": 678}
                        }
                    }
                }
            }"#,
            OpenCodeSubscription::Go,
        )
        .unwrap();

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].model.id(), "new-model");
        assert_eq!(models[0].model.display_name(), "New Model");
        assert!(models[0].supports_images);
        assert!(models[0].supports_thinking);
        assert_eq!(
            models[0].model.supported_reasoning_effort_levels(),
            Some([ReasoningEffort::Low, ReasoningEffort::High].as_slice())
        );
        assert_eq!(models[0].model.max_token_count(), 12345);
        assert_eq!(models[0].model.max_output_tokens(), Some(678));
        assert_eq!(models[0].model.protocol(), ApiProtocol::OpenAiChat);
    }

    #[test]
    fn test_parse_discovered_models_rejects_catalog_without_compatible_models() {
        let result = parse_discovered_models(
            r#"{
                "opencode": {
                    "npm": "@ai-sdk/anthropic",
                    "models": {
                        "text-only": {
                            "id": "text-only",
                            "name": "Text Only",
                            "tool_call": false,
                            "modalities": {"input":["text"],"output":["text"]},
                            "limit": {"context": 1000}
                        }
                    }
                }
            }"#,
            OpenCodeSubscription::Zen,
        );

        assert!(result.is_err());
    }

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
            OpenCodeLanguageModelProvider::new_with_cache(
                http_client,
                Arc::new(TestCredentialsProvider),
                FakeFs::new(cx.background_executor().clone()),
                PathBuf::from("/cache/models.json"),
                cx,
            )
        });
        let store_key = provider.state.update(cx, |state, cx| {
            state.set_api_key(Some("test-key".to_string()), cx)
        });
        store_key.await.unwrap();
        let model = provider.create_language_model_with_capabilities(
            test_model("test-model", ApiProtocol::Anthropic),
            OpenCodeSubscription::Go,
            true,
            false,
        );
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

    fn test_model(name: &str, protocol: ApiProtocol) -> opencode::Model {
        opencode::Model::new(
            name.to_string(),
            None,
            1000,
            Some(100),
            protocol,
            None,
            None,
            false,
        )
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
