use anyhow::{Result, anyhow};
use chrono::Local;
use collections::HashMap;
use credentials_provider::CredentialsProvider;
use fs::Fs;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{
    App, AsyncApp, Context, Entity, FutureExt as _, SharedString, Task, TaskExt, WeakEntity, Window,
};
use http_client::{AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, http};
use language_model::{
    ApiKeyState, AuthenticateError, DisabledReason, EnvVar, IconOrSvg, InlineDescription,
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelEffortLevel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, ProviderSettingsView, RateLimiter,
    ReasoningEffort, SubPageProviderSettings, env_var,
};
use opencode::{
    ApiProtocol, MODELS_DEV_API_URL, MODELS_DEV_FETCH_TIMEOUT, MODELS_DEV_MAX_FETCH_ATTEMPTS,
    MODELS_DEV_MAX_RESPONSE_SIZE, ModelsDevResponse, OPENCODE_API_URL, OpenCodeSubscription,
    parse_models_json,
};
use serde::{Deserialize, Serialize};
pub use settings::OpenCodeApiProtocol;
pub use settings::OpenCodeAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore, update_settings_file};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use ui::{
    Banner, Button, ButtonLink, ButtonSize, ButtonStyle, ConfiguredApiCard, Divider, List,
    ListBulletItem, Severity, Switch, SwitchLabelPosition, ToggleState, Tooltip, prelude::*,
};
use ui_input::InputField;
use util::ResultExt;

use crate::provider::anthropic::{AnthropicEventMapper, into_anthropic};
use crate::provider::google::{GoogleEventMapper, into_google};
use crate::provider::open_ai::{
    ChatCompletionMaxTokensParameter, OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai,
    into_open_ai_response,
};

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
pub(crate) const RESERVED_HEADER_NAMES: &[&str] = &["x-opencode-session"];

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenCodeSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
    pub show_zen_models: bool,
    pub show_go_models: bool,
    pub show_free_models: bool,
}

pub struct OpenCodeLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

#[derive(Serialize, Deserialize)]
struct CachedModelEntry {
    model: opencode::Model,
    subscription: OpenCodeSubscription,
}

#[derive(Serialize, Deserialize)]
struct ModelsCache {
    #[serde(default)]
    fetched_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    etag: Option<String>,
    #[serde(default)]
    models: HashMap<String, CachedModelEntry>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    fs: Arc<dyn Fs>,
    models_fetched: HashMap<String, (opencode::Model, OpenCodeSubscription)>,
    models_fetch_task: Option<Task<Result<()>>>,
    models_last_fetch_time: Option<chrono::DateTime<chrono::Utc>>,
    models_last_fetch_error: Option<SharedString>,
    models_cache_path: PathBuf,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenCodeLanguageModelProvider::api_url(cx);
        let should_fetch_models = api_key.is_some();
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        cx.spawn(async move |this, cx| {
            let result = task.await;
            if result.is_ok() && should_fetch_models {
                this.update(cx, |this, cx| this.start_fetch_model_task(cx))
                    .ok();
            }
            result
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenCodeLanguageModelProvider::api_url(cx);
        let should_fetch_models = true;
        let task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        cx.spawn(async move |this, cx| {
            let result = task.await;
            if result.is_ok() && should_fetch_models {
                this.update(cx, |this, cx| this.start_fetch_model_task(cx))
                    .ok();
            }
            result
        })
    }

    fn is_refreshing(&self) -> bool {
        self.models_fetch_task.is_some()
    }

    fn last_fetch_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.models_last_fetch_time
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>, force: bool) -> Task<Result<()>> {
        let http_client = Arc::clone(&self.http_client);
        let fs = Arc::clone(&self.fs);
        let cache_path = self.models_cache_path.clone();

        cx.spawn(async move |this, cx| {
            let executor = cx.background_executor().clone();

            let (cached_models, mut cached_etag, cached_fetched_at) =
                Self::read_models_cache(fs.as_ref(), &cache_path).await;
            if force {
                cached_etag = None;
            }

            if !force && !cached_models.is_empty() {
                let (free, zen, go) = count_models_by_tier(&cached_models);
                log::info!(
                    "Loaded {} models from cache ({} Free, {} Zen, {} Go)",
                    cached_models.len(),
                    free,
                    zen,
                    go
                );
                this.update(cx, |this, cx| {
                    this.models_fetched = cached_models.clone();
                    this.models_last_fetch_time =
                        Some(cached_fetched_at.unwrap_or_else(chrono::Utc::now));
                    cx.notify();
                })?;
            }

            let fetched =
                Self::fetch_with_retries(http_client.as_ref(), cached_etag.as_deref(), &executor)
                    .await;

            let result = match fetched {
                Ok(Some((body_bytes, etag))) => {
                    Self::apply_parsed_response(
                        body_bytes,
                        etag,
                        &cached_models,
                        cached_fetched_at,
                        fs.as_ref(),
                        &cache_path,
                        &this,
                        cx,
                    )
                    .await
                }
                Ok(None) => {
                    log::info!("Models cache is up to date");
                    this.update(cx, |this, cx| {
                        this.models_last_fetch_time = Some(chrono::Utc::now());
                        cx.notify();
                    })?;
                    Ok(())
                }
                Err(err) => {
                    log::error!(
                        "All {} attempts to fetch models from models.dev failed: {err}",
                        MODELS_DEV_MAX_FETCH_ATTEMPTS
                    );
                    if !cached_models.is_empty() {
                        let (free, zen, go) = count_models_by_tier(&cached_models);
                        log::info!(
                            "Using cached models after fetch failure ({} Free, {} Zen, {} Go)",
                            free,
                            zen,
                            go
                        );
                        this.update(cx, |this, cx| {
                            this.models_fetched = cached_models.clone();
                            this.models_last_fetch_time =
                                Some(cached_fetched_at.unwrap_or_else(chrono::Utc::now));
                            cx.notify();
                        })?;
                    }
                    Err(err)
                }
            };

            this.update(cx, |this, cx| {
                this.models_last_fetch_error = result.as_ref().err().map(|e| e.to_string().into());
                this.models_fetch_task = None;
                cx.notify();
            })?;

            result
        })
    }

    async fn fetch_models_dev_body(
        http_client: &dyn HttpClient,
        cached_etag: Option<&str>,
    ) -> Result<Option<(Vec<u8>, Option<String>)>> {
        let mut http_request = HttpRequest::builder()
            .method(Method::GET)
            .uri(MODELS_DEV_API_URL);
        if let Some(etag) = cached_etag {
            http_request = http_request.header("If-None-Match", etag);
        }
        let http_request = http_request
            .body(AsyncBody::empty())
            .map_err(|err| anyhow!("Failed to build request body for models.dev: {err}"))?;

        let response = http_client
            .send(http_request)
            .await
            .map_err(|err| anyhow!("Failed to fetch models: {err}"))?;

        if response.status() == http::StatusCode::NOT_MODIFIED {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to fetch models: HTTP {}",
                response.status()
            ));
        }

        let etag = response
            .headers()
            .get(http::header::ETAG)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let mut body_bytes = Vec::new();
        response
            .into_body()
            .take(MODELS_DEV_MAX_RESPONSE_SIZE + 1)
            .read_to_end(&mut body_bytes)
            .await?;
        if body_bytes.len() > MODELS_DEV_MAX_RESPONSE_SIZE as usize {
            return Err(anyhow!(
                "models.dev response body too large: {:.2} MB total, maximum {} MB allowed",
                body_bytes.len() as f64 / (1024 * 1024) as f64,
                MODELS_DEV_MAX_RESPONSE_SIZE / (1024 * 1024),
            ));
        }

        Ok(Some((body_bytes, etag)))
    }

    async fn fetch_with_retries(
        http_client: &dyn HttpClient,
        cached_etag: Option<&str>,
        executor: &gpui::BackgroundExecutor,
    ) -> Result<Option<(Vec<u8>, Option<String>)>> {
        let mut last_error: Option<anyhow::Error> = None;
        for attempt in 0..MODELS_DEV_MAX_FETCH_ATTEMPTS {
            if attempt > 0 {
                let delay = match attempt {
                    1 => Duration::from_secs(1),
                    2 => Duration::from_secs(2),
                    _ => Duration::from_secs(5),
                };
                log::info!(
                    "Retrying models fetch (attempt {}/{}) in {}s...",
                    attempt + 1,
                    MODELS_DEV_MAX_FETCH_ATTEMPTS,
                    delay.as_secs()
                );
                executor.timer(delay).await;
            }

            let fetch_result = Self::fetch_models_dev_body(http_client, cached_etag)
                .with_timeout(MODELS_DEV_FETCH_TIMEOUT, executor)
                .await;

            match fetch_result {
                Ok(Ok(Some(result))) => return Ok(Some(result)),
                Ok(Ok(None)) => return Ok(None),
                Ok(Err(err)) => {
                    if attempt + 1 >= MODELS_DEV_MAX_FETCH_ATTEMPTS {
                        log::error!("Failed to fetch models from models.dev: {err}");
                    } else {
                        log::warn!(
                            "Failed to fetch models from models.dev (attempt {}/{}): {err}",
                            attempt + 1,
                            MODELS_DEV_MAX_FETCH_ATTEMPTS
                        );
                    }
                    last_error = Some(err);
                }
                Err(_timeout) => {
                    if attempt + 1 >= MODELS_DEV_MAX_FETCH_ATTEMPTS {
                        log::error!(
                            "models.dev fetch timed out after {}s",
                            MODELS_DEV_FETCH_TIMEOUT.as_secs()
                        );
                    } else {
                        log::warn!(
                            "models.dev fetch timed out (attempt {}/{}), will retry",
                            attempt + 1,
                            MODELS_DEV_MAX_FETCH_ATTEMPTS
                        );
                    }
                    last_error = Some(anyhow!(
                        "models.dev fetch timed out after {}s",
                        MODELS_DEV_FETCH_TIMEOUT.as_secs()
                    ));
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| anyhow!("Unknown error"))
            .context(format!(
                "Failed to fetch models from models.dev after {} attempts",
                MODELS_DEV_MAX_FETCH_ATTEMPTS
            )))
    }

    async fn apply_parsed_response(
        body_bytes: Vec<u8>,
        etag: Option<String>,
        cached_models: &HashMap<String, (opencode::Model, OpenCodeSubscription)>,
        cached_fetched_at: Option<chrono::DateTime<chrono::Utc>>,
        fs: &dyn Fs,
        cache_path: &Path,
        this: &WeakEntity<State>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        match serde_json::from_slice::<ModelsDevResponse>(&body_bytes) {
            Ok(models_response) => {
                let models = parse_models_json(models_response);

                let mut model_map: HashMap<String, (opencode::Model, OpenCodeSubscription)> =
                    HashMap::default();
                for (model_id, model, subscription) in models {
                    let key = format!("{}/{}", subscription.id_prefix(), model_id);
                    model_map.insert(key, (model, subscription));
                }

                {
                    let (free, zen, go) = count_models_by_tier(&model_map);
                    log::info!(
                        "Fetched {} models from models.dev ({} Free, {} Zen, {} Go)",
                        model_map.len(),
                        free,
                        zen,
                        go
                    );
                }

                if model_map.is_empty() && !cached_models.is_empty() {
                    log::warn!("Fetched 0 models from models.dev; keeping existing cached models");
                    this.update(cx, |this, cx| {
                        this.models_last_fetch_time =
                            Some(cached_fetched_at.unwrap_or_else(chrono::Utc::now));
                        cx.notify();
                    })?;
                    Err(anyhow!(
                        "Fetched 0 models from models.dev, using locally cached models"
                    ))
                } else {
                    if let Err(err) =
                        Self::write_models_cache(fs, cache_path, &model_map, etag.as_deref()).await
                    {
                        log::warn!("Failed to write OpenCode models cache: {err}");
                    }

                    this.update(cx, |this, cx| {
                        this.models_fetched = model_map;
                        this.models_last_fetch_time = Some(chrono::Utc::now());
                        cx.notify();
                    })?;
                    Ok(())
                }
            }
            Err(err) => {
                log::error!("Failed to parse models.dev JSON: {err}");
                if !cached_models.is_empty() {
                    let (free, zen, go) = count_models_by_tier(cached_models);
                    log::info!(
                        "Using cached models due to parse failure ({} Free, {} Zen, {} Go)",
                        free,
                        zen,
                        go
                    );
                    this.update(cx, |this, cx| {
                        this.models_fetched = cached_models.clone();
                        this.models_last_fetch_time =
                            Some(cached_fetched_at.unwrap_or_else(chrono::Utc::now));
                        cx.notify();
                    })?;
                }
                Err(anyhow!("Failed to parse models.dev JSON: {err}"))
            }
        }
    }

    async fn read_models_cache(
        fs: &dyn Fs,
        cache_path: &Path,
    ) -> (
        HashMap<String, (opencode::Model, OpenCodeSubscription)>,
        Option<String>,
        Option<chrono::DateTime<chrono::Utc>>,
    ) {
        let cache_bytes = match fs.load_bytes(cache_path).await {
            Ok(bytes) => bytes,
            Err(err) => {
                if err
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound)
                {
                    log::info!("No models cache found, will download");
                } else {
                    log::warn!("Failed to read models cache ({err}), will re-download");
                }
                return (HashMap::default(), None, None);
            }
        };

        let cache: ModelsCache = match serde_json::from_slice(&cache_bytes) {
            Ok(cache) => cache,
            Err(err) => {
                log::warn!("Models cache is corrupt ({err}), will re-download");
                return (HashMap::default(), None, None);
            }
        };

        let model_map: HashMap<String, (opencode::Model, OpenCodeSubscription)> = cache
            .models
            .into_iter()
            .map(|(key, entry)| (key, (entry.model, entry.subscription)))
            .collect();

        (model_map, cache.etag, cache.fetched_at)
    }

    async fn write_models_cache(
        fs: &dyn Fs,
        cache_path: &Path,
        models: &HashMap<String, (opencode::Model, OpenCodeSubscription)>,
        etag: Option<&str>,
    ) -> Result<()> {
        let cache = ModelsCache {
            etag: etag.map(|s| s.to_string()),
            models: models
                .iter()
                .map(|(key, (model, subscription))| {
                    (
                        key.clone(),
                        CachedModelEntry {
                            model: model.clone(),
                            subscription: *subscription,
                        },
                    )
                })
                .collect(),
            fetched_at: Some(chrono::Utc::now()),
        };

        let cache_json = serde_json::to_string_pretty(&cache)?;

        if let Some(parent) = cache_path.parent() {
            if !fs.is_dir(parent).await {
                fs.create_dir(parent).await?;
            }
        }

        fs.atomic_write(cache_path.to_path_buf(), cache_json)
            .await?;

        Ok(())
    }

    fn start_fetch_model_task(&mut self, cx: &mut Context<Self>) {
        self.models_fetch_task.take();
        self.models_last_fetch_error = None;
        let task = self.fetch_models(cx, false);
        self.models_fetch_task = Some(task);
    }

    fn force_refresh_models(&mut self, cx: &mut Context<Self>) {
        log::info!("Forcing model list refresh");
        self.models_fetch_task.take();
        self.models_last_fetch_error = None;
        let task = self.fetch_models(cx, true);
        self.models_fetch_task = Some(task);
    }
}

fn count_models_by_tier(
    models: &HashMap<String, (opencode::Model, OpenCodeSubscription)>,
) -> (usize, usize, usize) {
    models
        .values()
        .fold((0, 0, 0), |(free, zen, go), (model, subscription)| {
            if model.disabled.is_some() {
                return (free, zen, go);
            }
            match subscription {
                OpenCodeSubscription::Free => (free + 1, zen, go),
                OpenCodeSubscription::Zen => (free, zen + 1, go),
                OpenCodeSubscription::Go => (free, zen, go + 1),
            }
        })
}

impl OpenCodeLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let models_cache_path = paths::data_dir().join("opencode").join("models.json");
        Self::new_internal(http_client, credentials_provider, models_cache_path, cx)
    }

    #[cfg(test)]
    pub fn new_test(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        models_cache_path: PathBuf,
        cx: &mut App,
    ) -> Self {
        Self::new_internal(http_client, credentials_provider, models_cache_path, cx)
    }

    fn new_internal(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        models_cache_path: PathBuf,
        cx: &mut App,
    ) -> Self {
        let state = cx.new({
            let http_client = http_client.clone();
            |cx| {
                cx.observe_global::<SettingsStore>({
                    let mut last_settings = Self::settings(cx).clone();
                    move |this: &mut State, cx| {
                        let current_settings = Self::settings(cx);
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
                            }
                            cx.notify();
                        }
                    }
                })
                .detach();

                let mut state = State {
                    api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                    credentials_provider: credentials_provider.clone(),
                    http_client,
                    fs: <dyn Fs>::global(cx),
                    models_fetched: HashMap::default(),
                    models_fetch_task: None,
                    models_last_fetch_time: None,
                    models_last_fetch_error: None,
                    models_cache_path,
                };

                state.start_fetch_model_task(cx);

                state
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(
        &self,
        model: opencode::Model,
        subscription: OpenCodeSubscription,
        is_custom: bool,
    ) -> Arc<dyn LanguageModel> {
        let id_str = if is_custom {
            format!("custom/{}/{}", subscription.id_prefix(), model.id)
        } else {
            format!("{}/{}", subscription.id_prefix(), model.id)
        };
        let disabled = model
            .disabled
            .as_ref()
            .map(|reason| DisabledReason::new(reason.clone()));
        let name = if is_custom {
            LanguageModelName::from(format!(
                "Custom {}: {}",
                subscription.display_name(),
                model.display_name()
            ))
        } else {
            LanguageModelName::from(format!(
                "{}: {}",
                subscription.display_name(),
                model.display_name()
            ))
        };
        Arc::new(OpenCodeLanguageModel {
            id: LanguageModelId::from(id_str),
            name,
            model,
            subscription,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
            disabled,
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
            OpenCodeSubscription::Free => settings.show_free_models,
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

    fn default_model(&self, _: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn default_fast_model(&self, _: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let settings = Self::settings(cx);
        let state = self.state.read(cx);
        let mut entries: Vec<(opencode::Model, OpenCodeSubscription, bool)> = Vec::new();

        for (model, subscription) in state.models_fetched.values() {
            if Self::subscription_enabled(*subscription, cx) {
                entries.push((model.clone(), *subscription, false));
            }
        }

        for model in &settings.available_models {
            let protocol = match model.protocol {
                OpenCodeApiProtocol::Anthropic => ApiProtocol::Anthropic,
                OpenCodeApiProtocol::OpenAiResponses => ApiProtocol::OpenAiResponses,
                OpenCodeApiProtocol::OpenAiChat => ApiProtocol::OpenAiChat,
                OpenCodeApiProtocol::Google => ApiProtocol::Google,
            };
            let subscription = match model.subscription {
                Some(settings::OpenCodeModelSubscription::Go) => OpenCodeSubscription::Go,
                Some(settings::OpenCodeModelSubscription::Free) => OpenCodeSubscription::Free,
                Some(settings::OpenCodeModelSubscription::Zen) | None => OpenCodeSubscription::Zen,
            };
            if !Self::subscription_enabled(subscription, cx) {
                continue;
            }
            let custom_model = opencode::Model {
                id: model.name.clone(),
                name: model
                    .display_name
                    .clone()
                    .unwrap_or_else(|| model.name.clone()),
                max_tokens: model.max_tokens,
                max_output_tokens: model.max_output_tokens,
                protocol,
                supports_images: true,
                supports_tools: true,
                reasoning_effort_levels: model.reasoning_effort_levels.clone(),
                interleaved_reasoning: model.interleaved_reasoning,
                cost_input: None,
                cost_output: None,
                custom_api_url: model.custom_model_api_url.clone(),
                disabled: None,
            };
            entries.push((custom_model, subscription, true));
        }

        entries.sort_by(
            |(model1, subscription1, is_custom1), (model2, subscription2, is_custom2)| {
                let group1 = if *is_custom1 {
                    "custom"
                } else {
                    subscription1.id_prefix()
                };
                let group2 = if *is_custom2 {
                    "custom"
                } else {
                    subscription2.id_prefix()
                };
                group1.cmp(group2).then(model1.id.cmp(&model2.id))
            },
        );

        entries
            .into_iter()
            .map(|(model, subscription, is_custom)| {
                self.create_language_model(model, subscription, is_custom)
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
    name: LanguageModelName,
    model: opencode::Model,
    subscription: OpenCodeSubscription,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
    disabled: Option<DisabledReason>,
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

impl OpenCodeLanguageModel {
    fn base_api_url(&self, cx: &AsyncApp) -> SharedString {
        if let Some(url) = &self.model.custom_api_url {
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
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<open_ai::ResponseStreamEvent>>>,
    > {
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
        self.name.clone()
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn is_disabled(&self) -> Option<DisabledReason> {
        self.disabled.clone()
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tools
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn supports_thinking(&self) -> bool {
        self.model.reasoning_effort_levels.is_some()
    }

    fn supports_disabling_thinking(&self) -> bool {
        self.model
            .reasoning_effort_levels
            .as_ref()
            .is_some_and(|levels| levels.contains(&ReasoningEffort::None))
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        self.model
            .reasoning_effort_levels
            .as_ref()
            .map(|levels| {
                let levels = levels
                    .iter()
                    .filter(|effort| **effort != ReasoningEffort::None)
                    .collect::<Vec<_>>();
                if levels.is_empty() {
                    return Vec::new();
                }
                let default_index = levels.len() - 1;
                levels
                    .into_iter()
                    .enumerate()
                    .map(|(i, effort)| {
                        let (name, value) = reasoning_effort_display(*effort);
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
            LanguageModelToolChoice::None => self.model.protocol != ApiProtocol::Google,
        }
    }

    fn telemetry_id(&self) -> String {
        format!(
            "opencode/{}/{}",
            self.subscription.id_prefix(),
            self.model.id
        )
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens
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
        let http_client = if let Some(ref thread_id) = request.thread_id
            && let Ok(value) = http::HeaderValue::from_str(thread_id)
        {
            Arc::new(InjectHeaderClient {
                inner: self.http_client.clone(),
                name: http::HeaderName::from_static("x-opencode-session"),
                value,
            })
        } else {
            self.http_client.clone()
        };
        let extra_headers = self.custom_headers(cx);

        match self.model.protocol {
            ApiProtocol::Anthropic => {
                let mode = if self.supports_thinking() && request.thinking_allowed {
                    anthropic::AnthropicModelMode::AdaptiveThinking
                } else {
                    anthropic::AnthropicModelMode::Default
                };
                let anthropic_request = into_anthropic(
                    request,
                    self.model.id.clone(),
                    1.0,
                    self.model.max_output_tokens.unwrap_or(8192),
                    mode,
                    anthropic::completion::AnthropicPromptCacheMode::Automatic,
                );
                let stream =
                    self.stream_anthropic(anthropic_request, http_client, extra_headers, cx);
                async move {
                    let mapper = AnthropicEventMapper::new(PROVIDER_NAME);
                    Ok(mapper.map_stream(stream.await?).boxed())
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
                let openai_request = into_open_ai(
                    request,
                    &self.model.id,
                    true,
                    false,
                    self.model.max_output_tokens,
                    ChatCompletionMaxTokensParameter::MaxCompletionTokens,
                    reasoning_effort,
                    self.model.interleaved_reasoning,
                );
                let stream =
                    self.stream_openai_chat(openai_request, http_client, extra_headers, cx);
                async move {
                    let mapper = OpenAiEventMapper::new();
                    Ok(mapper.map_stream(stream.await?).boxed())
                }
                .boxed()
            }
            ApiProtocol::OpenAiResponses => {
                let supports_none_reasoning_effort = self
                    .model
                    .reasoning_effort_levels
                    .as_ref()
                    .is_some_and(|levels| levels.contains(&ReasoningEffort::None));
                let response_request = into_open_ai_response(
                    request,
                    &self.model.id,
                    true,
                    false,
                    self.model.max_output_tokens,
                    None,
                    supports_none_reasoning_effort,
                );
                let stream =
                    self.stream_openai_response(response_request, http_client, extra_headers, cx);
                async move {
                    let mapper = OpenAiResponseEventMapper::new();
                    Ok(mapper.map_stream(stream.await?).boxed())
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
                let google_request = into_google(request, self.model.id.clone(), mode);
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
                OpenCodeSubscription::Free => opencode_settings.show_free_models = Some(is_enabled),
            }
        });
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }

    fn refresh_models(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.state
            .update(cx, |state, cx| state.force_refresh_models(cx));
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
            let show_free = settings.show_free_models;

            let subscription_toggles = v_flex()
                .gap_2()
                .child(Label::new("Subscriptions"))
                .child(
                    Switch::new("opencode-show-zen-models", show_zen.into())
                        .full_width(true)
                        .label("Show Zen Models")
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
                )
                .child(Divider::horizontal_dashed())
                .child(
                    Switch::new("opencode-show-free-models", show_free.into())
                        .full_width(true)
                        .label("Show Free models")
                        .label_position(SwitchLabelPosition::Start)
                        .on_click(cx.listener(|this, state, window, cx| {
                            this.set_subscription_enabled(
                                OpenCodeSubscription::Free,
                                matches!(state, ToggleState::Selected),
                                window,
                                cx,
                            );
                        })),
                );

            let no_subscriptions_warning = if !show_zen && !show_go && !show_free {
                Some(Banner::new().severity(Severity::Warning).child(Label::new(
                    "No subscriptions enabled. Enable at least one subscription to use OpenCode.",
                )))
            } else {
                None
            };

            let (is_refreshing, last_fetch_text, last_fetch_error) = {
                let state = self.state.read(cx);
                let last_fetch_text = match (state.is_refreshing(), state.last_fetch_time()) {
                    (true, None) => "Fetching models...".to_string(),
                    (_, Some(t)) => t
                        .with_timezone(&Local)
                        .format("%Y-%m-%d at %H:%M:%S")
                        .to_string(),
                    (false, None) => "never".to_string(),
                };
                (
                    state.is_refreshing(),
                    last_fetch_text,
                    state.models_last_fetch_error.clone(),
                )
            };

            let fetch_error_banner = last_fetch_error.map(|error| {
                Banner::new()
                    .severity(Severity::Error)
                    .child(Label::new(format!("Model fetch failed: {error}")))
            });

            let refresh_row = h_flex()
                .gap_2()
                .items_center()
                .child(
                    Button::new("opencode-refresh-models", "Refresh model list")
                        .style(ButtonStyle::Outlined)
                        .size(ButtonSize::Medium)
                        .disabled(is_refreshing)
                        .tooltip(Tooltip::text(
                            "Re-download model configurations from OpenCode's models.dev",
                        ))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.refresh_models(window, cx);
                        })),
                )
                .child(
                    Label::new(format!("Last refreshed: {last_fetch_text}"))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                );

            v_flex()
                .size_full()
                .gap_2p5()
                .child(Headline::new("OpenCode").size(HeadlineSize::Small))
                .child(api_key_section)
                .child(Divider::horizontal())
                .child(subscription_toggles)
                .children(no_subscriptions_warning)
                .children(fetch_error_banner)
                .child(refresh_row)
                .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;

    use super::*;

    struct NoCredentials;

    impl CredentialsProvider for NoCredentials {
        fn read_credentials(
            &self,
            _url: &str,
            _cx: &AsyncApp,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<(String, Vec<u8>)>>>>> {
            Box::pin(async { Ok(None) })
        }

        fn write_credentials(
            &self,
            _url: &str,
            _username: &str,
            _password: &[u8],
            _cx: &AsyncApp,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>>>> {
            Box::pin(async { Ok(()) })
        }

        fn delete_credentials(
            &self,
            _url: &str,
            _cx: &AsyncApp,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>>>> {
            Box::pin(async { Ok(()) })
        }
    }

    fn setup_test_env(cx: &mut gpui::TestAppContext) -> (Arc<fs::FakeFs>, PathBuf) {
        let fake_fs = fs::FakeFs::new(cx.background_executor.clone());
        let cache_path = PathBuf::from("/test/opencode/models.json");
        cx.update(|cx| {
            <dyn Fs>::set_global(fake_fs.clone(), cx);
            settings::init(cx);
        });
        (fake_fs, cache_path)
    }

    fn enable_settings(
        cx: &mut gpui::TestAppContext,
        f: impl FnOnce(&mut settings::OpenCodeSettingsContent),
    ) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |content| {
                    f(content
                        .language_models
                        .get_or_insert_default()
                        .opencode
                        .get_or_insert_default());
                });
            });
        });
    }

    fn make_test_model(id: &str, name: &str) -> opencode::Model {
        opencode::Model {
            id: id.to_string(),
            name: name.to_string(),
            max_tokens: 200_000,
            max_output_tokens: None,
            protocol: ApiProtocol::OpenAiChat,
            supports_images: false,
            supports_tools: true,
            reasoning_effort_levels: None,
            interleaved_reasoning: false,
            cost_input: None,
            cost_output: None,
            custom_api_url: None,
            disabled: None,
        }
    }

    async fn populate_cache(
        fs: &fs::FakeFs,
        cache_path: &Path,
        entries: &[(&str, opencode::Model, OpenCodeSubscription)],
        etag: Option<&str>,
    ) {
        let mut map: HashMap<String, (opencode::Model, OpenCodeSubscription)> = HashMap::default();
        for (key, model, sub) in entries {
            map.insert(key.to_string(), (model.clone(), *sub));
        }
        State::write_models_cache(fs, cache_path, &map, etag)
            .await
            .unwrap();
    }

    #[gpui::test]
    async fn test_cache_etag_304_no_refetch(cx: &mut gpui::TestAppContext) {
        let (fake_fs, cache_path) = setup_test_env(cx);

        populate_cache(
            fake_fs.as_ref(),
            &cache_path,
            &[(
                "zen/cached-model",
                make_test_model("cached-model", "Cached Model"),
                OpenCodeSubscription::Zen,
            )],
            Some("\"etag-abc\""),
        )
        .await;

        let etag_was_sent = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let http_client = http_client::FakeHttpClient::create({
            let etag_was_sent = etag_was_sent.clone();
            move |req| {
                let etag_was_sent = etag_was_sent.clone();
                async move {
                    let if_none_match = req
                        .headers()
                        .get("If-None-Match")
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string());
                    etag_was_sent.store(
                        if_none_match.as_deref() == Some("\"etag-abc\""),
                        std::sync::atomic::Ordering::SeqCst,
                    );
                    if if_none_match.as_deref() == Some("\"etag-abc\"") {
                        Ok(http::Response::builder()
                            .status(http::StatusCode::NOT_MODIFIED)
                            .body(http_client::AsyncBody::empty())
                            .unwrap())
                    } else {
                        Ok(http::Response::builder()
                            .status(http::StatusCode::OK)
                            .header("Content-Type", "application/json")
                            .body(http_client::AsyncBody::from(
                                r#"{"opencode":{"npm":"@ai-sdk/openai-compatible","models":{"fresh-model":{"name":"Fresh Model"}}}}"#,
                            ))
                            .unwrap())
                    }
                }
            }
        });

        let provider = cx.update(|cx| {
            OpenCodeLanguageModelProvider::new_test(
                http_client,
                Arc::new(NoCredentials),
                cache_path,
                cx,
            )
        });

        cx.run_until_parked();

        assert!(
            etag_was_sent.load(std::sync::atomic::Ordering::SeqCst),
            "If-None-Match header should carry the cached ETag"
        );

        cx.update(|cx| {
            let models = provider.provided_models(cx);
            assert_eq!(models.len(), 1, "cached model should be available");
            assert_eq!(models[0].name().0.as_ref(), "Zen: Cached Model");
        });
    }

    #[gpui::test]
    async fn test_200_response_replaces_cached_models(cx: &mut gpui::TestAppContext) {
        let (fake_fs, cache_path) = setup_test_env(cx);

        populate_cache(
            fake_fs.as_ref(),
            &cache_path,
            &[(
                "zen/old-model",
                make_test_model("old-model", "Old Model"),
                OpenCodeSubscription::Zen,
            )],
            Some("\"old-etag\""),
        )
        .await;

        let http_client = http_client::FakeHttpClient::create(|_req| async move {
            Ok(http::Response::builder()
                .status(http::StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("ETag", "\"new-etag\"")
                .body(http_client::AsyncBody::from(
                    r#"{"opencode":{"npm":"@ai-sdk/openai-compatible","models":{"fresh-model":{"name":"Fresh Model","attachment":true,"tool_call":true,"limit":{"context":100000},"cost":{"input":1.0,"output":2.0}}}},"opencode-go":{"npm":"@ai-sdk/openai-compatible","models":{}}}"#,
                ))
                .unwrap())
        });

        let provider = cx.update(|cx| {
            OpenCodeLanguageModelProvider::new_test(
                http_client,
                Arc::new(NoCredentials),
                cache_path.clone(),
                cx,
            )
        });

        cx.run_until_parked();

        let (cached, etag, _) = State::read_models_cache(fake_fs.as_ref(), &cache_path).await;
        assert_eq!(
            etag.as_deref(),
            Some("\"new-etag\""),
            "cache should store the ETag from the 200 response"
        );
        assert!(
            cached.contains_key("zen/fresh-model"),
            "cache should contain the fresh model from the 200 response"
        );
        assert!(
            !cached.contains_key("zen/old-model"),
            "cache should no longer contain the old pre-populated model"
        );

        cx.update(|cx| {
            let models = provider.provided_models(cx);
            assert_eq!(models.len(), 1, "only the fresh model should be available");
            assert_eq!(models[0].name().0.as_ref(), "Zen: Fresh Model");
        });
    }

    #[gpui::test]
    async fn test_network_error_keeps_cached_data(cx: &mut gpui::TestAppContext) {
        let (fake_fs, cache_path) = setup_test_env(cx);

        populate_cache(
            fake_fs.as_ref(),
            &cache_path,
            &[(
                "zen/survivor",
                make_test_model("survivor", "Survivor Model"),
                OpenCodeSubscription::Zen,
            )],
            None,
        )
        .await;

        let http_client = http_client::FakeHttpClient::create(|_req| async move {
            Ok(http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(http_client::AsyncBody::empty())
                .unwrap())
        });

        let provider = cx.update(|cx| {
            OpenCodeLanguageModelProvider::new_test(
                http_client,
                Arc::new(NoCredentials),
                cache_path,
                cx,
            )
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let models = provider.provided_models(cx);
            assert_eq!(models.len(), 1, "cached model survives network error");
            assert_eq!(models[0].name().0.as_ref(), "Zen: Survivor Model");
        });
    }

    #[gpui::test]
    async fn test_force_refresh_network_error_keeps_models(cx: &mut gpui::TestAppContext) {
        let (fake_fs, cache_path) = setup_test_env(cx);

        populate_cache(
            fake_fs.as_ref(),
            &cache_path,
            &[(
                "zen/survivor",
                make_test_model("survivor", "Survivor Model"),
                OpenCodeSubscription::Zen,
            )],
            None,
        )
        .await;

        let http_client = http_client::FakeHttpClient::create(|_req| async move {
            Ok(http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(http_client::AsyncBody::empty())
                .unwrap())
        });

        let provider = cx.update(|cx| {
            OpenCodeLanguageModelProvider::new_test(
                http_client,
                Arc::new(NoCredentials),
                cache_path,
                cx,
            )
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let models = provider.provided_models(cx);
            assert_eq!(
                models.len(),
                1,
                "cached model should survive initial fetch failure"
            );
        });

        cx.update(|cx| {
            provider
                .state
                .update(cx, |state, cx| state.force_refresh_models(cx));
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let models = provider.provided_models(cx);
            assert_eq!(
                models.len(),
                1,
                "models should survive force refresh with network failure"
            );
            assert_eq!(models[0].name().0.as_ref(), "Zen: Survivor Model");
        });
    }

    #[gpui::test]
    async fn test_subscription_filtering_hides_tiers(cx: &mut gpui::TestAppContext) {
        let (fake_fs, cache_path) = setup_test_env(cx);

        populate_cache(
            fake_fs.as_ref(),
            &cache_path,
            &[
                (
                    "zen/zen-model",
                    make_test_model("zen-model", "Zen Model"),
                    OpenCodeSubscription::Zen,
                ),
                (
                    "go/go-model",
                    make_test_model("go-model", "Go Model"),
                    OpenCodeSubscription::Go,
                ),
                (
                    "free/free-model",
                    make_test_model("free-model", "Free Model"),
                    OpenCodeSubscription::Free,
                ),
            ],
            None,
        )
        .await;

        let http_client = http_client::FakeHttpClient::create(|_req| async move {
            Ok(http::Response::builder()
                .status(http::StatusCode::NOT_MODIFIED)
                .body(http_client::AsyncBody::empty())
                .unwrap())
        });

        enable_settings(cx, |opencode| {
            opencode.show_zen_models = Some(true);
            opencode.show_go_models = Some(false);
            opencode.show_free_models = Some(false);
        });

        let provider = cx.update(|cx| {
            OpenCodeLanguageModelProvider::new_test(
                http_client,
                Arc::new(NoCredentials),
                cache_path,
                cx,
            )
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let models = provider.provided_models(cx);
            assert_eq!(
                models.len(),
                1,
                "only Zen models should be visible when Go and Free are hidden"
            );
            assert_eq!(models[0].name().0.as_ref(), "Zen: Zen Model");
        });
    }

    #[gpui::test]
    async fn test_provided_models_custom_adds_alongside_auto(cx: &mut gpui::TestAppContext) {
        let (fake_fs, cache_path) = setup_test_env(cx);

        populate_cache(
            fake_fs.as_ref(),
            &cache_path,
            &[(
                "zen/auto-model",
                make_test_model("auto-model", "Auto Model"),
                OpenCodeSubscription::Zen,
            )],
            None,
        )
        .await;

        let http_client = http_client::FakeHttpClient::create(|_req| async move {
            Ok(http::Response::builder()
                .status(http::StatusCode::NOT_MODIFIED)
                .body(http_client::AsyncBody::empty())
                .unwrap())
        });

        enable_settings(cx, |opencode| {
            opencode.available_models = Some(vec![AvailableModel {
                name: "auto-model".to_string(),
                display_name: Some("My Custom Addition".to_string()),
                max_tokens: 100_000,
                max_output_tokens: Some(8_000),
                protocol: OpenCodeApiProtocol::Anthropic,
                subscription: Some(settings::OpenCodeModelSubscription::Zen),
                reasoning_effort_levels: None,
                interleaved_reasoning: false,
                custom_model_api_url: None,
            }]);
        });

        let provider = cx.update(|cx| {
            OpenCodeLanguageModelProvider::new_test(
                http_client,
                Arc::new(NoCredentials),
                cache_path,
                cx,
            )
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let models = provider.provided_models(cx);
            assert_eq!(
                models.len(),
                2,
                "custom model should appear alongside auto-discovered one"
            );
            let ids: Vec<String> = models.iter().map(|m| m.id().0.to_string()).collect();
            assert!(
                ids.iter().any(|id| id == "zen/auto-model"),
                "auto-discovered model should keep the subscription prefix: got {ids:?}"
            );
            assert!(
                ids.iter().any(|id| id == "custom/zen/auto-model"),
                "custom model should use a custom prefix with the subscription tier: got {ids:?}"
            );
            let names: Vec<String> = models.iter().map(|m| m.name().0.to_string()).collect();
            assert!(
                names.iter().any(|n| n == "Zen: Auto Model"),
                "expected auto-discovered model: got {names:?}"
            );
            assert!(
                names.iter().any(|n| n == "Custom Zen: My Custom Addition"),
                "expected custom model: got {names:?}"
            );
        });
    }
}
