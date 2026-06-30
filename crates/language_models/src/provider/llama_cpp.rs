use anyhow::Result;
use collections::{HashMap, HashSet};
use credentials_provider::CredentialsProvider;
use fs::Fs;
use futures::Stream;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, CursorStyle, Entity, Task, TaskExt};
use http_client::{CustomHeaders, HttpClient};
use language_model::util::parse_tool_arguments;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolResultContent,
    LanguageModelToolUse, MessageContent, RateLimiter, Role, StopReason, TokenUsage, env_var,
};
use llama_cpp::{
    LLAMA_CPP_API_URL, ModelEntry, Props, get_models, get_props, stream_chat_completion,
    stream_model_events,
};
pub use settings::LlamaCppAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore, update_settings_file};
use std::pin::Pin;
use std::sync::LazyLock;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;
use ui::{
    ButtonLike, ButtonLink, ConfiguredApiCard, ElevationIndex, List, ListBulletItem, Tooltip,
    prelude::*,
};
use ui_input::InputField;
use util::ResultExt;

use crate::AllLanguageModelSettings;

const LLAMA_CPP_DOWNLOAD_URL: &str = "https://llama.app";
const LLAMA_CPP_MODELS_URL: &str = "https://huggingface.co/models?library=gguf&sort=trending";

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("llama.cpp");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("llama.cpp");

const API_KEY_ENV_VAR_NAME: &str = "LLAMACPP_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

/// How long to wait before reconnecting to `/models/sse` after the stream ends.
const MODEL_EVENT_RECONNECT_INTERVAL: Duration = Duration::from_secs(5);

/// Context length assumed for an unloaded router model (it can't be probed
/// without loading it). Generous so early messages work; re-discovery refines
/// it once the model loads.
const ASSUMED_UNLOADED_CONTEXT: u64 = 131_072;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LlamaCppSettings {
    pub api_url: String,
    pub auto_discover: bool,
    pub available_models: Vec<AvailableModel>,
    pub context_window: Option<u64>,
    pub custom_headers: CustomHeaders,
}

pub struct LlamaCppLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    /// Live capabilities shared with the agent's models (see [`LiveCapabilities`]).
    capability_cells: CapabilityCells,
    /// Live model-load progress shared with the models (see [`LoadingProgress`]).
    loading_progress: LoadingProgress,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    fetched_models: Vec<llama_cpp::Model>,
    fetch_model_task: Option<Task<Result<()>>>,
    /// Router-mode task on `/models/sse`; re-runs discovery as models load/unload.
    model_event_task: Option<Task<()>>,
    /// Same `Arc` as the provider's; re-discovery keeps these cells in sync.
    capability_cells: CapabilityCells,
    /// Same `Arc` as the provider's; the event stream updates it as a model loads.
    loading_progress: LoadingProgress,
}

impl State {
    fn is_authenticated(&self) -> bool {
        !self.fetched_models.is_empty()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = LlamaCppLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        self.fetched_models.clear();
        // Drop the event stream so it reconnects with the new key (re-fetch
        // below restarts it).
        self.model_event_task = None;
        write_recover(&self.loading_progress).clear();
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = LlamaCppLanguageModelProvider::api_url(cx);
        let load_key_task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        cx.spawn(async move |this, cx| {
            match load_key_task.await {
                Ok(()) | Err(AuthenticateError::CredentialsNotFound) => {}
                Err(error) => {
                    log::warn!("failed to load llama.cpp API key: {error}");
                }
            }
            let fetch_models_task = this.update(cx, |this, cx| this.fetch_models(cx))?;
            match fetch_models_task.await {
                Ok(()) => Ok(()),
                Err(err) => {
                    // A refused connection means the server isn't running yet, not an error.
                    let connection_refused = err.chain().any(|cause| {
                        cause
                            .downcast_ref::<std::io::Error>()
                            .is_some_and(|io_err| {
                                io_err.kind() == std::io::ErrorKind::ConnectionRefused
                            })
                    });
                    if connection_refused {
                        Err(AuthenticateError::ConnectionRefused)
                    } else {
                        Err(AuthenticateError::Other(err))
                    }
                }
            }
        })
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let http_client = Arc::clone(&self.http_client);
        let settings = LlamaCppLanguageModelProvider::settings(cx);
        let api_url = LlamaCppLanguageModelProvider::api_url(cx);
        let api_key = self.api_key_state.key(&api_url);
        let extra_headers = settings.custom_headers.clone();

        cx.spawn(async move |this, cx| {
            let entries = get_models(
                http_client.as_ref(),
                &api_url,
                api_key.as_deref(),
                &extra_headers,
            )
            .await?;

            let is_router = entries.iter().any(ModelEntry::is_router_entry);

            // Models the server reports as loading, used below to prune stale
            // progress labels by reconciling against the live listing (a preempted
            // load or missed SSE event can skip the terminal event).
            let loading_ids: HashSet<String> = entries
                .iter()
                .filter(|entry| entry.is_loading())
                .map(|entry| entry.id.clone())
                .collect();

            let models: Vec<llama_cpp::Model> = if is_router {
                // Router mode: metadata comes from `/v1/models`. We probe
                // `/props` only for loaded models so listing never triggers a
                // load; unloaded models use the listing's hints and overrides.
                let tasks = entries.into_iter().map(|entry| {
                    let http_client = Arc::clone(&http_client);
                    let api_url = api_url.clone();
                    let api_key = api_key.clone();
                    let extra_headers = extra_headers.clone();
                    async move {
                        let props = if entry.is_loaded() {
                            get_props(
                                http_client.as_ref(),
                                &api_url,
                                api_key.as_deref(),
                                Some(&entry.id),
                                &extra_headers,
                            )
                            .await
                            .log_err()
                        } else {
                            None
                        };
                        model_from_entry(&entry, props.as_ref())
                    }
                });
                futures::stream::iter(tasks)
                    .buffer_unordered(5)
                    .collect()
                    .await
            } else {
                // Single-model mode: one `/props` call describes the loaded model.
                let props = get_props(
                    http_client.as_ref(),
                    &api_url,
                    api_key.as_deref(),
                    None,
                    &extra_headers,
                )
                .await
                .log_err();
                entries
                    .iter()
                    .map(|entry| model_from_entry(entry, props.as_ref()))
                    .collect()
            };

            this.update(cx, |this, cx| {
                this.fetched_models = models;
                let effective = compute_effective_models(
                    &this.fetched_models,
                    LlamaCppLanguageModelProvider::settings(cx),
                );
                sync_capability_cells(&this.capability_cells, &effective);
                // Drop progress labels for models no longer loading, so a stale
                // "Loading …" can't stick after a preempted load or missed event.
                write_recover(&this.loading_progress).retain(|id, _| loading_ids.contains(id));
                // Router mode loads models on demand: subscribe so capabilities
                // self-correct as they load/unload. Start it once (events trigger
                // re-discovery, not a re-spawn); single-model mode needs no stream.
                if is_router {
                    if this.model_event_task.is_none() {
                        this.start_model_event_stream(cx);
                    }
                } else {
                    this.model_event_task = None;
                }
                cx.notify();
            })
        })
    }

    /// Subscribes to `/models/sse` and re-runs discovery as models load, unload,
    /// or the list changes, so capabilities stay current. Reconnects if the stream
    /// drops; on builds without `/models/sse` the refresh is simply skipped.
    fn start_model_event_stream(&mut self, cx: &mut Context<Self>) {
        let http_client = Arc::clone(&self.http_client);
        let api_url = LlamaCppLanguageModelProvider::api_url(cx);
        let api_key = self.api_key_state.key(&api_url);
        let extra_headers = LlamaCppLanguageModelProvider::settings(cx)
            .custom_headers
            .clone();

        self.model_event_task = Some(cx.spawn(async move |this, cx| {
            loop {
                match stream_model_events(
                    http_client.as_ref(),
                    &api_url,
                    api_key.as_deref(),
                    &extra_headers,
                )
                .await
                {
                    Ok(mut events) => {
                        while let Some(event) = events.next().await {
                            let Some(event) = event.log_err() else {
                                continue;
                            };
                            if let Some(exit_code) = event.load_failure() {
                                log::error!(
                                    "llama.cpp model {} failed to load (exit code {exit_code})",
                                    event.model
                                );
                            }
                            // Loading-progress tick: record it for the selector (no
                            // re-discovery). `cx.notify()` drives `ProviderStateChanged`.
                            if let Some(progress) = event.load_progress() {
                                let label = SharedString::from(progress.progress_label());
                                if this
                                    .update(cx, |this, cx| {
                                        write_recover(&this.loading_progress)
                                            .insert(event.model.clone(), label);
                                        cx.notify();
                                    })
                                    .is_err()
                                {
                                    return;
                                }
                                continue;
                            }
                            if !event.changes_model_state() {
                                continue;
                            }
                            // Terminal load/unload (or list change): drop the
                            // progress label and re-discover to refresh capabilities.
                            if this
                                .update(cx, |this, cx| {
                                    write_recover(&this.loading_progress).remove(&event.model);
                                    this.restart_fetch_models_task(cx);
                                })
                                .is_err()
                            {
                                return;
                            }
                        }
                    }
                    // Endpoint missing (older build) or connection failed; retry after a backoff.
                    Err(error) => {
                        log::warn!("llama.cpp model event stream unavailable: {error:#}");
                    }
                }

                cx.background_executor()
                    .timer(MODEL_EVENT_RECONNECT_INTERVAL)
                    .await;
                if this.update(cx, |_, _| ()).is_err() {
                    return;
                }
            }
        }));
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        let task = self.fetch_models(cx);
        self.fetch_model_task.replace(task);
    }
}

/// Capabilities that only become known once a router model loads. `LanguageModel`'s
/// methods take no `cx`, yet the agent reads them live each turn, so we share
/// them through a map that re-discovery updates — an already-selected model
/// picks up real values without re-selection. Image support is available from
/// `/v1/models` hints, so it stays a field.
#[derive(Clone, Copy, Debug, PartialEq)]
struct LiveCapabilities {
    max_tokens: u64,
    supports_tools: bool,
    supports_thinking: bool,
}

impl LiveCapabilities {
    fn of(model: &llama_cpp::Model) -> Self {
        Self {
            max_tokens: model.max_tokens,
            supports_tools: model.supports_tools,
            supports_thinking: model.supports_thinking,
        }
    }
}

/// Live capabilities keyed by model name, shared by the provider and its models.
type CapabilityCells = Arc<RwLock<HashMap<String, LiveCapabilities>>>;

/// Model name → load-status label (e.g. `"Loading weights 42%"`) while a router
/// model loads, shared so the model selector can show progress. Absent once loaded.
type LoadingProgress = Arc<RwLock<HashMap<String, SharedString>>>;

/// Locks for reading, recovering instead of panicking on a poisoned lock. The
/// critical sections are infallible map ops, so poisoning is unreachable anyway.
fn read_recover<T>(lock: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    lock.read().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Locks for writing; see [`read_recover`].
fn write_recover<T>(lock: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    lock.write()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The exact set of models `provided_models` exposes: discovery merged with the
/// `available_models` and `context_window` overrides. Shared with re-discovery.
fn compute_effective_models(
    fetched_models: &[llama_cpp::Model],
    settings: &LlamaCppSettings,
) -> HashMap<String, llama_cpp::Model> {
    let mut models: HashMap<String, llama_cpp::Model> = HashMap::default();
    if settings.auto_discover {
        for model in fetched_models {
            let mut model = model.clone();
            if let Some(context_window) = settings.context_window {
                model.max_tokens = context_window;
            }
            models.insert(model.name.clone(), model);
        }
    }
    merge_settings_into_models(
        &mut models,
        &settings.available_models,
        settings.context_window,
    );
    models
}

/// Updates the shared capability map from the effective models, so a model held
/// by an open conversation observes the new values (it reads the map by name).
fn sync_capability_cells(cells: &CapabilityCells, effective: &HashMap<String, llama_cpp::Model>) {
    let mut cells = write_recover(cells);
    for model in effective.values() {
        cells.insert(model.name.clone(), LiveCapabilities::of(model));
    }
}

/// Builds a model from a `/v1/models` entry, refined by `/props` when the model
/// is loaded. An unloaded router model can't be probed, so we assume optimistic
/// capabilities and let re-discovery refine them on load.
fn model_from_entry(entry: &ModelEntry, props: Option<&Props>) -> llama_cpp::Model {
    let max_tokens = props
        .and_then(Props::context_length)
        .or_else(|| entry.meta.as_ref().and_then(|meta| meta.n_ctx))
        .or_else(|| entry.meta.as_ref().and_then(|meta| meta.n_ctx_train))
        .unwrap_or(ASSUMED_UNLOADED_CONTEXT);
    // Trust `/props` when present. Without it, assume tools for an unloaded model
    // (re-discovery corrects on load) but not for a loaded model whose probe failed.
    let supports_tools = match props {
        Some(props) => props.supports_tools(),
        None => !entry.is_loaded(),
    };
    let supports_images = props.is_some_and(Props::supports_images) || entry.supports_images_hint();
    let supports_thinking = props.is_some_and(Props::supports_thinking);

    llama_cpp::Model::new(
        &entry.id,
        Some(&display_name_for(&entry.id)),
        Some(max_tokens),
        supports_tools,
        supports_images,
        supports_thinking,
    )
}

/// Friendly display name from a model id, which is often a `.gguf` file path.
fn display_name_for(id: &str) -> String {
    let base = id.rsplit(['/', '\\']).next().unwrap_or(id);
    base.strip_suffix(".gguf").unwrap_or(base).to_string()
}

fn telemetry_id_for(id: &str) -> String {
    format!("{PROVIDER_ID}/{}", display_name_for(id))
}

impl LlamaCppLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let capability_cells: CapabilityCells = Arc::new(RwLock::new(HashMap::default()));
        let loading_progress: LoadingProgress = Arc::new(RwLock::new(HashMap::default()));
        let this = Self {
            http_client: http_client.clone(),
            capability_cells: capability_cells.clone(),
            loading_progress: loading_progress.clone(),
            state: cx.new(|cx| {
                cx.observe_global::<SettingsStore>({
                    let mut last_settings = LlamaCppLanguageModelProvider::settings(cx).clone();
                    move |this: &mut State, cx| {
                        let current_settings = LlamaCppLanguageModelProvider::settings(cx);
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
                                // Drop the event stream so it reconnects against
                                // the new URL (re-auth below restarts it).
                                this.model_event_task = None;
                                write_recover(&this.loading_progress).clear();
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
                    model_event_task: None,
                    capability_cells,
                    loading_progress,
                    api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                    credentials_provider,
                }
            }),
        };
        // Discover eagerly so a running server is picked up without opening settings.
        this.state
            .update(cx, |state, cx| state.restart_fetch_models_task(cx));
        this
    }

    fn settings(cx: &App) -> &LlamaCppSettings {
        &AllLanguageModelSettings::get_global(cx).llama_cpp
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            LLAMA_CPP_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }

    fn has_custom_url(cx: &App) -> bool {
        Self::settings(cx).api_url != LLAMA_CPP_API_URL
    }
}

impl LanguageModelProviderState for LlamaCppLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for LlamaCppLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiLlamaCpp)
    }

    fn default_model(&self, _: &App) -> Option<Arc<dyn LanguageModel>> {
        // No default model: in router mode it could trigger an expensive load of
        // an unloaded model on a constrained machine.
        None
    }

    fn default_fast_model(&self, _: &App) -> Option<Arc<dyn LanguageModel>> {
        // See explanation for default_model.
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let settings = LlamaCppLanguageModelProvider::settings(cx);
        let effective = compute_effective_models(&self.state.read(cx).fetched_models, settings);

        // Refresh the shared capability map so open conversations pick up settings changes.
        sync_capability_cells(&self.capability_cells, &effective);
        let mut models = effective
            .into_values()
            .map(|model| {
                Arc::new(LlamaCppLanguageModel {
                    id: LanguageModelId::from(model.name.clone()),
                    name: model.name.clone(),
                    display_name: model.display_name().to_string(),
                    fallback_capabilities: LiveCapabilities::of(&model),
                    supports_images: model.supports_images,
                    capability_cells: self.capability_cells.clone(),
                    loading_progress: self.loading_progress.clone(),
                    http_client: self.http_client.clone(),
                    request_limiter: RateLimiter::new(4),
                    state: self.state.clone(),
                }) as Arc<dyn LanguageModel>
            })
            .collect::<Vec<_>>();
        models.sort_by_key(|model| model.name());
        models
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        let state = self.state.clone();
        cx.new(|cx| ConfigurationView::new(state, window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct LlamaCppLanguageModel {
    id: LanguageModelId,
    /// The model id sent to the server.
    name: String,
    display_name: String,
    /// Live capabilities shared with the provider, read fresh on each access so an
    /// open conversation reflects the model's real values once it has loaded.
    capability_cells: CapabilityCells,
    /// Used when `capability_cells` has no entry (e.g. model removed mid-conversation).
    fallback_capabilities: LiveCapabilities,
    /// Available from `/v1/models` hints, so captured at build time.
    supports_images: bool,
    /// Shared with the provider; this model's load progress, read by `name` so the
    /// selector can show a loading indicator.
    loading_progress: LoadingProgress,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
    state: Entity<State>,
}

impl LlamaCppLanguageModel {
    /// The model's live capabilities, or the build-time fallback if the map lacks it.
    fn capabilities(&self) -> LiveCapabilities {
        read_recover(&self.capability_cells)
            .get(&self.name)
            .copied()
            .unwrap_or(self.fallback_capabilities)
    }

    /// This model's load-status label while loading, read live from the shared map.
    fn loading_label(&self) -> Option<SharedString> {
        read_recover(&self.loading_progress)
            .get(&self.name)
            .cloned()
    }

    fn to_llama_cpp_request(
        &self,
        request: LanguageModelRequest,
    ) -> llama_cpp::ChatCompletionRequest {
        build_llama_cpp_request(
            &self.name,
            self.supports_images,
            self.capabilities(),
            request,
        )
    }

    fn stream_completion(
        &self,
        request: llama_cpp::ChatCompletionRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<llama_cpp::ResponseStreamEvent>>>,
    > {
        let http_client = self.http_client.clone();
        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = LlamaCppLanguageModelProvider::api_url(cx);
            let extra_headers = LlamaCppLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let future = self.request_limiter.stream(async move {
            let stream = stream_chat_completion(
                http_client.as_ref(),
                &api_url,
                api_key.as_deref(),
                request,
                &extra_headers,
            )
            .await?;
            Ok(stream)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

fn build_llama_cpp_request(
    model_name: &str,
    supports_images: bool,
    capabilities: LiveCapabilities,
    request: LanguageModelRequest,
) -> llama_cpp::ChatCompletionRequest {
    let supports_tools = capabilities.supports_tools;
    let supports_thinking = capabilities.supports_thinking;
    let mut messages = Vec::new();

    for message in request.messages {
        let mut reasoning_content: Option<String> = None;
        for content in message.content {
            match content {
                MessageContent::Text(text) => add_message_content_part(
                    llama_cpp::MessagePart::Text { text },
                    message.role,
                    &mut messages,
                    if supports_thinking && message.role == Role::Assistant {
                        reasoning_content.take()
                    } else {
                        None
                    },
                ),
                MessageContent::Thinking { text, .. } => {
                    if supports_thinking && message.role == Role::Assistant && !text.is_empty() {
                        reasoning_content.get_or_insert_default().push_str(&text);
                    }
                }
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Compaction(_) => {}
                MessageContent::Image(image) => {
                    if supports_images {
                        add_message_content_part(
                            llama_cpp::MessagePart::Image {
                                image_url: llama_cpp::ImageUrl {
                                    url: image.to_base64_url(),
                                    detail: None,
                                },
                            },
                            message.role,
                            &mut messages,
                            if supports_thinking && message.role == Role::Assistant {
                                reasoning_content.take()
                            } else {
                                None
                            },
                        );
                    }
                }
                MessageContent::ToolUse(tool_use) => {
                    let tool_call = llama_cpp::ToolCall {
                        id: tool_use.id.to_string(),
                        content: llama_cpp::ToolCallContent::Function {
                            function: llama_cpp::FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                            },
                        },
                    };

                    if let Some(llama_cpp::ChatMessage::Assistant {
                        tool_calls,
                        reasoning_content: message_reasoning_content,
                        ..
                    }) = messages.last_mut()
                    {
                        append_reasoning_content(
                            message_reasoning_content,
                            reasoning_content.take(),
                        );
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(llama_cpp::ChatMessage::Assistant {
                            content: None,
                            reasoning_content: reasoning_content.take(),
                            tool_calls: vec![tool_call],
                        });
                    }
                }
                MessageContent::ToolResult(tool_result) => {
                    let content: Vec<llama_cpp::MessagePart> = tool_result
                        .content
                        .iter()
                        .filter_map(|part| match part {
                            LanguageModelToolResultContent::Text(text) => {
                                Some(llama_cpp::MessagePart::Text {
                                    text: text.to_string(),
                                })
                            }
                            LanguageModelToolResultContent::Image(image) => {
                                if supports_images {
                                    Some(llama_cpp::MessagePart::Image {
                                        image_url: llama_cpp::ImageUrl {
                                            url: image.to_base64_url(),
                                            detail: None,
                                        },
                                    })
                                } else {
                                    None
                                }
                            }
                        })
                        .collect();

                    messages.push(llama_cpp::ChatMessage::Tool {
                        content: content.into(),
                        tool_call_id: tool_result.tool_use_id.to_string(),
                    });
                }
            }
        }
    }

    let tools: Vec<llama_cpp::ToolDefinition> = if supports_tools {
        request
            .tools
            .into_iter()
            .map(|tool| llama_cpp::ToolDefinition::Function {
                function: llama_cpp::FunctionDefinition {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect()
    } else {
        Vec::new()
    };
    // Only send `tool_choice` with actual tools; some OpenAI-compatible servers
    // reject it otherwise.
    let tool_choice = if tools.is_empty() {
        None
    } else {
        request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => llama_cpp::ToolChoice::Auto,
            LanguageModelToolChoice::Any => llama_cpp::ToolChoice::Required,
            LanguageModelToolChoice::None => llama_cpp::ToolChoice::None,
        })
    };

    llama_cpp::ChatCompletionRequest {
        model: model_name.to_string(),
        messages,
        stream: true,
        // Let the server decide the output length (its `n_predict` default).
        max_tokens: None,
        stop: if request.stop.is_empty() {
            None
        } else {
            Some(request.stop)
        },
        // llama.cpp models often ship recommended sampler settings, so override
        // temperature only when the request sets one.
        temperature: request.temperature,
        tools,
        tool_choice,
        stream_options: Some(llama_cpp::StreamOptions {
            include_usage: true,
        }),
    }
}

impl LanguageModel for LlamaCppLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        match self.loading_label() {
            // Surface load progress in the display name so it shows wherever the
            // model is named, without provider-agnostic UI changes. The agent
            // rebuilds the name on `ProviderStateChanged`, which our ticks emit.
            Some(label) => LanguageModelName::from(format!("{} · {}", self.display_name, label)),
            None => LanguageModelName::from(self.display_name.clone()),
        }
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        self.capabilities().supports_tools
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        self.supports_tools()
            && match choice {
                LanguageModelToolChoice::Auto => true,
                LanguageModelToolChoice::Any => true,
                LanguageModelToolChoice::None => true,
            }
    }

    fn supports_images(&self) -> bool {
        self.supports_images
    }

    fn supports_thinking(&self) -> bool {
        self.capabilities().supports_thinking
    }

    fn telemetry_id(&self) -> String {
        telemetry_id_for(&self.name)
    }

    fn max_token_count(&self) -> u64 {
        self.capabilities().max_tokens
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let request = self.to_llama_cpp_request(request);
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = LlamaCppEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

struct LlamaCppEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl LlamaCppEventMapper {
    fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<llama_cpp::ResponseStreamEvent>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(LanguageModelCompletionError::from(error))],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: llama_cpp::ResponseStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();

        if let Some(usage) = event.usage {
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            })));
        }

        if let Some(choice) = event.choices.into_iter().next() {
            if let Some(reasoning_content) = choice.delta.reasoning_content {
                events.push(Ok(LanguageModelCompletionEvent::Thinking {
                    text: reasoning_content,
                    signature: None,
                }));
            }

            if let Some(content) = choice.delta.content {
                if !content.is_empty() {
                    events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                }
            }

            if let Some(tool_calls) = choice.delta.tool_calls {
                for tool_call in tool_calls {
                    let entry = self.tool_calls_by_index.entry(tool_call.index).or_default();

                    if let Some(tool_id) = tool_call.id {
                        entry.id = tool_id;
                    }

                    if let Some(function) = tool_call.function {
                        if let Some(name) = function.name {
                            // Only the first chunk carries the function name;
                            // later chunks send an empty name with arguments.
                            if !name.is_empty() {
                                entry.name = name;
                            }
                        }

                        if let Some(arguments) = function.arguments {
                            entry.arguments.push_str(&arguments);
                        }
                    }
                }
            }

            if let Some(finish_reason) = choice.finish_reason.as_deref() {
                match finish_reason {
                    "stop" => {
                        events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
                    }
                    "tool_calls" => {
                        events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                            match parse_tool_arguments(&tool_call.arguments) {
                                Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                                    LanguageModelToolUse {
                                        id: tool_call.id.into(),
                                        name: tool_call.name.into(),
                                        is_input_complete: true,
                                        input,
                                        raw_input: tool_call.arguments,
                                        thought_signature: None,
                                    },
                                )),
                                Err(error) => {
                                    Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                        id: tool_call.id.into(),
                                        tool_name: tool_call.name.into(),
                                        raw_input: tool_call.arguments.into(),
                                        json_parse_error: error.to_string(),
                                    })
                                }
                            }
                        }));

                        events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
                    }
                    "length" => {
                        events.push(Ok(LanguageModelCompletionEvent::Stop(
                            StopReason::MaxTokens,
                        )));
                    }
                    unexpected => {
                        log::warn!("Unexpected llama.cpp finish_reason: {unexpected:?}");
                        events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
                    }
                }
            }
        }

        events
    }
}

#[derive(Default)]
struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
}

fn add_message_content_part(
    new_part: llama_cpp::MessagePart,
    role: Role,
    messages: &mut Vec<llama_cpp::ChatMessage>,
    reasoning_content: Option<String>,
) {
    match (role, messages.last_mut()) {
        (Role::User, Some(llama_cpp::ChatMessage::User { content }))
        | (Role::System, Some(llama_cpp::ChatMessage::System { content })) => {
            content.push_part(new_part);
        }
        (
            Role::Assistant,
            Some(llama_cpp::ChatMessage::Assistant {
                content: Some(content),
                reasoning_content: message_reasoning_content,
                ..
            }),
        ) => {
            append_reasoning_content(message_reasoning_content, reasoning_content);
            content.push_part(new_part);
        }
        _ => {
            messages.push(match role {
                Role::User => llama_cpp::ChatMessage::User {
                    content: llama_cpp::MessageContent::from(vec![new_part]),
                },
                Role::Assistant => llama_cpp::ChatMessage::Assistant {
                    content: Some(llama_cpp::MessageContent::from(vec![new_part])),
                    reasoning_content,
                    tool_calls: Vec::new(),
                },
                Role::System => llama_cpp::ChatMessage::System {
                    content: llama_cpp::MessageContent::from(vec![new_part]),
                },
            });
        }
    }
}

fn append_reasoning_content(target: &mut Option<String>, content: Option<String>) {
    let Some(content) = content else {
        return;
    };
    if content.is_empty() {
        return;
    }
    target.get_or_insert_default().push_str(&content);
}

fn merge_settings_into_models(
    models: &mut HashMap<String, llama_cpp::Model>,
    available_models: &[AvailableModel],
    context_window: Option<u64>,
) {
    for setting_model in available_models {
        if let Some(model) = models.get_mut(&setting_model.name) {
            if context_window.is_none() {
                model.max_tokens = setting_model.max_tokens;
            }
            if setting_model.display_name.is_some() {
                model.display_name = setting_model.display_name.clone();
            }
            if let Some(supports_tools) = setting_model.supports_tools {
                model.supports_tools = supports_tools;
            }
            if let Some(supports_images) = setting_model.supports_images {
                model.supports_images = supports_images;
            }
            if let Some(supports_thinking) = setting_model.supports_thinking {
                model.supports_thinking = supports_thinking;
            }
        } else {
            models.insert(
                setting_model.name.clone(),
                llama_cpp::Model {
                    name: setting_model.name.clone(),
                    display_name: setting_model.display_name.clone(),
                    max_tokens: context_window.unwrap_or(setting_model.max_tokens),
                    supports_tools: setting_model.supports_tools.unwrap_or(false),
                    supports_images: setting_model.supports_images.unwrap_or(false),
                    supports_thinking: setting_model.supports_thinking.unwrap_or(false),
                },
            );
        }
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    api_url_editor: Entity<InputField>,
    context_window_editor: Entity<InputField>,
    state: Entity<State>,
}

impl ConfigurationView {
    pub fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| InputField::new(window, cx, "sk-...").label("API key"));

        let api_url_editor = cx.new(|cx| {
            let input = InputField::new(window, cx, LLAMA_CPP_API_URL).label("API URL");
            input.set_text(&LlamaCppLanguageModelProvider::api_url(cx), window, cx);
            input
        });

        let context_window_editor = cx.new(|cx| {
            let input = InputField::new(window, cx, "8192").label("Context Window");
            if let Some(context_window) = LlamaCppLanguageModelProvider::settings(cx).context_window
            {
                input.set_text(&context_window.to_string(), window, cx);
            }
            input
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        Self {
            api_key_editor,
            api_url_editor,
            context_window_editor,
            state,
        }
    }

    fn retry_connection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let has_api_url = LlamaCppLanguageModelProvider::has_custom_url(cx);
        let has_api_key = self
            .state
            .read_with(cx, |state, _| state.api_key_state.has_key());
        if !has_api_url {
            self.save_api_url(cx);
        }
        if !has_api_key {
            self.save_api_key(&Default::default(), window, cx);
        }

        self.state.update(cx, |state, cx| {
            state.restart_fetch_models_task(cx);
        });
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        // A URL change can cause the editor to be shown again.
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
        let current_url = LlamaCppLanguageModelProvider::api_url(cx);
        if !api_url.is_empty() && &api_url != &current_url {
            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .llama_cpp
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
                .and_then(|models| models.llama_cpp.as_mut())
            {
                settings.api_url = Some(LLAMA_CPP_API_URL.into());
            }
        });
        cx.notify();
    }

    fn save_context_window(&mut self, cx: &mut Context<Self>) {
        let context_window_str = self
            .context_window_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let current_context_window = LlamaCppLanguageModelProvider::settings(cx).context_window;

        if let Ok(context_window) = context_window_str.parse::<u64>() {
            if Some(context_window) != current_context_window {
                let fs = <dyn Fs>::global(cx);
                update_settings_file(fs, cx, move |settings, _| {
                    settings
                        .language_models
                        .get_or_insert_default()
                        .llama_cpp
                        .get_or_insert_default()
                        .context_window = Some(context_window);
                });
            }
        } else if context_window_str.is_empty() && current_context_window.is_some() {
            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .llama_cpp
                    .get_or_insert_default()
                    .context_window = None;
            });
        }
    }

    fn reset_context_window(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.context_window_editor
            .update(cx, |input, cx| input.set_text("", window, cx));
        let fs = <dyn Fs>::global(cx);
        update_settings_file(fs, cx, |settings, _cx| {
            if let Some(settings) = settings
                .language_models
                .as_mut()
                .and_then(|models| models.llama_cpp.as_mut())
            {
                settings.context_window = None;
            }
        });
        cx.notify();
    }

    fn render_instructions(cx: &App) -> Div {
        v_flex()
            .gap_2()
            .child(Label::new(
                "Run open models locally with llama.cpp's built-in server, or connect to a \
                remote llama.cpp server.",
            ))
            .child(Label::new("To use a local llama.cpp server:"))
            .child(
                List::new()
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("Install llama.cpp from"))
                            .child(ButtonLink::new("llama.app", LLAMA_CPP_DOWNLOAD_URL)),
                    )
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("Start the server in router mode:"))
                            .child(Label::new("llama serve").inline_code(cx)),
                    )
                    .child(ListBulletItem::new(
                        "Click 'Connect' below to start using llama.cpp in Zed",
                    )),
            )
            .child(Label::new(
                "Alternatively, you can connect to a remote llama.cpp server by specifying its \
                URL and API key (set with --api-key, may not be required):",
            ))
    }

    fn render_api_key_editor(&self, cx: &Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable.")
        } else {
            "API key configured".to_string()
        };

        if !state.api_key_state.has_key() {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(format!(
                        "You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!(
                        "To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."
                    ))
                })
                .into_any_element()
        }
    }

    fn render_context_window_editor(&self, cx: &Context<Self>) -> Div {
        let settings = LlamaCppLanguageModelProvider::settings(cx);
        let custom_context_window_set = settings.context_window.is_some();

        if custom_context_window_set {
            h_flex()
                .p_3()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().elevated_surface_background)
                .child(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(v_flex().gap_1().child(Label::new(format!(
                            "Context Window: {}",
                            settings.context_window.unwrap_or_default()
                        )))),
                )
                .child(
                    Button::new("reset-context-window", "Reset")
                        .label_size(LabelSize::Small)
                        .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                        .layer(ElevationIndex::ModalSurface)
                        .on_click(
                            cx.listener(|this, _, window, cx| {
                                this.reset_context_window(window, cx)
                            }),
                        ),
                )
        } else {
            v_flex()
                .on_action(
                    cx.listener(|this, _: &menu::Confirm, _window, cx| {
                        this.save_context_window(cx)
                    }),
                )
                .child(self.context_window_editor.clone())
                .child(
                    Label::new("Default: discovered from the server")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
        }
    }

    fn render_api_url_editor(&self, cx: &Context<Self>) -> Div {
        let api_url = LlamaCppLanguageModelProvider::api_url(cx);
        let custom_api_url_set = api_url != LLAMA_CPP_API_URL;

        if custom_api_url_set {
            h_flex()
                .p_3()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().elevated_surface_background)
                .child(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(v_flex().gap_1().child(Label::new(api_url))),
                )
                .child(
                    Button::new("reset-api-url", "Reset API URL")
                        .label_size(LabelSize::Small)
                        .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                        .layer(ElevationIndex::ModalSurface)
                        .on_click(
                            cx.listener(|this, _, window, cx| this.reset_api_url(window, cx)),
                        ),
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
            .child(Self::render_instructions(cx))
            .child(self.render_api_url_editor(cx))
            .child(self.render_context_window_editor(cx))
            .child(self.render_api_key_editor(cx))
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .gap_2()
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .map(|this| {
                                if is_authenticated {
                                    this.child(
                                        Button::new("llama-cpp-webui", "Open WebUI")
                                            .style(ButtonStyle::Subtle)
                                            .end_icon(
                                                Icon::new(IconName::ArrowUpRight)
                                                    .size(IconSize::XSmall)
                                                    .color(Color::Muted),
                                            )
                                            .on_click(move |_, _, cx| {
                                                let url =
                                                    LlamaCppLanguageModelProvider::api_url(cx);
                                                cx.open_url(&url);
                                            })
                                            .into_any_element(),
                                    )
                                    .child(
                                        Button::new("llama-cpp-site", "llama.cpp")
                                            .style(ButtonStyle::Subtle)
                                            .end_icon(
                                                Icon::new(IconName::ArrowUpRight)
                                                    .size(IconSize::XSmall)
                                                    .color(Color::Muted),
                                            )
                                            .on_click(move |_, _, cx| {
                                                cx.open_url(LLAMA_CPP_DOWNLOAD_URL)
                                            })
                                            .into_any_element(),
                                    )
                                } else {
                                    this.child(
                                        Button::new("download_llama_cpp_button", "Get llama.cpp")
                                            .style(ButtonStyle::Subtle)
                                            .end_icon(
                                                Icon::new(IconName::ArrowUpRight)
                                                    .size(IconSize::XSmall)
                                                    .color(Color::Muted),
                                            )
                                            .on_click(move |_, _, cx| {
                                                cx.open_url(LLAMA_CPP_DOWNLOAD_URL)
                                            })
                                            .into_any_element(),
                                    )
                                }
                            })
                            .child(
                                Button::new("view-models", "Browse GGUF Models")
                                    .style(ButtonStyle::Subtle)
                                    .end_icon(
                                        Icon::new(IconName::ArrowUpRight)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                    .on_click(move |_, _, cx| cx.open_url(LLAMA_CPP_MODELS_URL)),
                            ),
                    )
                    .map(|this| {
                        if is_authenticated {
                            this.child(
                                ButtonLike::new("connected")
                                    .disabled(true)
                                    .cursor_style(CursorStyle::Arrow)
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Icon::new(IconName::Check).color(Color::Success))
                                            .child(Label::new("Connected"))
                                            .into_any_element(),
                                    )
                                    .child(
                                        IconButton::new("refresh-models", IconName::RotateCcw)
                                            .tooltip(Tooltip::text("Refresh Models"))
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.state.update(cx, |state, _| {
                                                    state.fetched_models.clear();
                                                });
                                                this.retry_connection(window, cx);
                                            })),
                                    ),
                            )
                        } else {
                            this.child(
                                Button::new("retry_llama_cpp_models", "Connect")
                                    .start_icon(
                                        Icon::new(IconName::PlayOutlined).size(IconSize::XSmall),
                                    )
                                    .on_click(cx.listener(move |this, _, window, cx| {
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
    use gpui::TestAppContext;
    use http_client::FakeHttpClient;
    use parking_lot::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct FakeCredentialsProvider {
        api_key: Vec<u8>,
    }

    impl CredentialsProvider for FakeCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>,
        > {
            let api_key = self.api_key.clone();
            Box::pin(async move { Ok(Some(("Bearer".to_string(), api_key))) })
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

    fn entry(id: &str, n_ctx: Option<u64>, n_ctx_train: Option<u64>) -> ModelEntry {
        ModelEntry {
            id: id.to_string(),
            meta: Some(llama_cpp::ModelMeta { n_ctx, n_ctx_train }),
            architecture: None,
            status: None,
        }
    }

    #[test]
    fn display_name_strips_path_and_extension() {
        assert_eq!(
            display_name_for("../models/Qwen2.5-Coder-7B-Instruct-Q4_K_M.gguf"),
            "Qwen2.5-Coder-7B-Instruct-Q4_K_M"
        );
        assert_eq!(display_name_for("my-alias"), "my-alias");
    }

    #[test]
    fn telemetry_id_strips_local_model_paths() {
        assert_eq!(
            telemetry_id_for("/Users/alice/models/Qwen2.5-Coder-7B-Instruct-Q4_K_M.gguf"),
            "llama.cpp/Qwen2.5-Coder-7B-Instruct-Q4_K_M"
        );
        assert_eq!(
            telemetry_id_for(r"C:\Users\alice\models\Qwen2.5-Coder-7B-Instruct-Q4_K_M.gguf"),
            "llama.cpp/Qwen2.5-Coder-7B-Instruct-Q4_K_M"
        );
        assert_eq!(telemetry_id_for("my-alias"), "llama.cpp/my-alias");
    }

    #[test]
    fn model_uses_props_then_meta_for_context() {
        let props = Props {
            default_generation_settings: Some(llama_cpp::GenerationSettings { n_ctx: Some(8192) }),
            modalities: Some(llama_cpp::Modalities { vision: true }),
            chat_template_caps: Some(llama_cpp::ChatTemplateCaps {
                supports_tool_calls: true,
                supports_preserve_reasoning: true,
                ..Default::default()
            }),
        };
        // /props wins when present.
        let model = model_from_entry(&entry("m", Some(4096), Some(131072)), Some(&props));
        assert_eq!(model.max_tokens, 8192);
        assert!(model.supports_tools);
        assert!(model.supports_images);
        assert!(model.supports_thinking);

        // Unprobed: falls back to the listing's runtime context, then trained
        // context. Tools are assumed supported until the model loads.
        let model = model_from_entry(&entry("m", Some(4096), Some(131072)), None);
        assert_eq!(model.max_tokens, 4096);
        assert!(model.supports_tools);
        assert!(!model.supports_thinking);

        let model = model_from_entry(&entry("m", None, Some(131072)), None);
        assert_eq!(model.max_tokens, 131072);

        // Nothing reported -> the optimistic unloaded-context assumption.
        let model = model_from_entry(&entry("m", None, None), None);
        assert_eq!(model.max_tokens, ASSUMED_UNLOADED_CONTEXT);
        assert!(model.supports_tools);
    }

    #[test]
    fn router_entry_detects_vision_from_modalities() {
        let router_entry = ModelEntry {
            id: "vlm".to_string(),
            meta: None,
            architecture: Some(llama_cpp::Architecture {
                input_modalities: vec!["text".to_string(), "image".to_string()],
            }),
            status: Some(llama_cpp::ModelStatus {
                value: "unloaded".to_string(),
            }),
        };
        let model = model_from_entry(&router_entry, None);
        assert!(model.supports_images);
        // Unprobed router models optimistically advertise tools until loaded.
        assert!(model.supports_tools);
    }

    #[test]
    fn settings_override_capabilities_and_context() {
        let mut models: HashMap<String, llama_cpp::Model> = HashMap::default();
        models.insert(
            "qwen".to_string(),
            llama_cpp::Model::new("qwen", Some("qwen"), Some(8192), false, false, false),
        );

        let available = vec![AvailableModel {
            name: "qwen".to_string(),
            display_name: Some("Qwen Coder".to_string()),
            max_tokens: 16384,
            supports_tools: Some(true),
            supports_images: None,
            supports_thinking: Some(true),
        }];

        merge_settings_into_models(&mut models, &available, None);

        let model = models.get("qwen").unwrap();
        assert_eq!(model.display_name.as_deref(), Some("Qwen Coder"));
        assert_eq!(model.max_tokens, 16384);
        assert!(model.supports_tools);
        assert!(model.supports_thinking);
        // Unspecified capability keeps the discovered value.
        assert!(!model.supports_images);
    }

    #[test]
    fn capability_cells_update_when_a_model_loads() {
        let cells: CapabilityCells = Arc::new(RwLock::new(HashMap::default()));
        let settings = LlamaCppSettings {
            auto_discover: true,
            ..Default::default()
        };

        // Cold: the optimistic unloaded-context assumption.
        let cold = vec![llama_cpp::Model::new(
            "m",
            Some("m"),
            Some(ASSUMED_UNLOADED_CONTEXT),
            true,
            false,
            false,
        )];
        sync_capability_cells(&cells, &compute_effective_models(&cold, &settings));
        assert_eq!(
            cells.read().unwrap().get("m").unwrap().max_tokens,
            ASSUMED_UNLOADED_CONTEXT
        );

        // The model loads and reports its real context. The shared map must
        // reflect the new value so a model reading it by name (an open
        // conversation) is no longer stuck on the cold-start assumption.
        let loaded = vec![llama_cpp::Model::new(
            "m",
            Some("m"),
            Some(262_144),
            true,
            false,
            true,
        )];
        sync_capability_cells(&cells, &compute_effective_models(&loaded, &settings));
        assert_eq!(cells.read().unwrap().get("m").unwrap().max_tokens, 262_144);
        assert!(cells.read().unwrap().get("m").unwrap().supports_thinking);
    }

    #[test]
    fn request_preserves_assistant_thinking_when_supported() {
        let request = build_llama_cpp_request(
            "test-model",
            false,
            LiveCapabilities {
                max_tokens: 8192,
                supports_tools: false,
                supports_thinking: true,
            },
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![
                        MessageContent::Thinking {
                            text: "reasoning".to_string(),
                            signature: None,
                        },
                        MessageContent::Text("answer".to_string()),
                    ],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
        );

        assert_eq!(request.messages.len(), 1);
        match &request.messages[0] {
            llama_cpp::ChatMessage::Assistant {
                content: Some(llama_cpp::MessageContent::Plain(content)),
                reasoning_content: Some(reasoning_content),
                tool_calls,
            } => {
                assert_eq!(content, "answer");
                assert_eq!(reasoning_content, "reasoning");
                assert!(tool_calls.is_empty());
            }
            message => panic!("unexpected message: {message:?}"),
        }
    }

    #[test]
    fn request_skips_assistant_thinking_when_unsupported() {
        let request = build_llama_cpp_request(
            "test-model",
            false,
            LiveCapabilities {
                max_tokens: 8192,
                supports_tools: false,
                supports_thinking: false,
            },
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![
                        MessageContent::Thinking {
                            text: "reasoning".to_string(),
                            signature: None,
                        },
                        MessageContent::RedactedThinking("encrypted".to_string()),
                        MessageContent::Text("answer".to_string()),
                    ],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
        );

        assert_eq!(request.messages.len(), 1);
        match &request.messages[0] {
            llama_cpp::ChatMessage::Assistant {
                content: Some(llama_cpp::MessageContent::Plain(content)),
                reasoning_content,
                tool_calls,
            } => {
                assert_eq!(content, "answer");
                assert_eq!(reasoning_content, &None);
                assert!(tool_calls.is_empty());
            }
            message => panic!("unexpected message: {message:?}"),
        }
    }

    #[test]
    fn request_preserves_thinking_for_assistant_tool_calls_when_supported() {
        let request = build_llama_cpp_request(
            "test-model",
            false,
            LiveCapabilities {
                max_tokens: 8192,
                supports_tools: true,
                supports_thinking: true,
            },
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![
                        MessageContent::Thinking {
                            text: "reasoning".to_string(),
                            signature: None,
                        },
                        MessageContent::ToolUse(LanguageModelToolUse {
                            id: "call_1".into(),
                            name: "weather".into(),
                            raw_input: r#"{"city":"Oslo"}"#.to_string(),
                            input: serde_json::json!({ "city": "Oslo" }),
                            is_input_complete: true,
                            thought_signature: None,
                        }),
                    ],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
        );

        assert_eq!(request.messages.len(), 1);
        match &request.messages[0] {
            llama_cpp::ChatMessage::Assistant {
                content: None,
                reasoning_content: Some(reasoning_content),
                tool_calls,
            } => {
                assert_eq!(reasoning_content, "reasoning");
                assert_eq!(tool_calls.len(), 1);
            }
            message => panic!("unexpected message: {message:?}"),
        }
    }

    #[test]
    fn usage_event_precedes_stop_event() {
        let mut mapper = LlamaCppEventMapper::new();
        let events = mapper.map_event(llama_cpp::ResponseStreamEvent {
            model: "test-model".to_string(),
            object: "chat.completion.chunk".to_string(),
            choices: vec![llama_cpp::ChoiceDelta {
                index: 0,
                delta: llama_cpp::ResponseMessageDelta {
                    content: None,
                    reasoning_content: None,
                    tool_calls: None,
                },
                finish_reason: Some("stop".to_string()),
            }],
            usage: Some(llama_cpp::Usage {
                prompt_tokens: 11,
                completion_tokens: 7,
                total_tokens: 18,
            }),
        });

        assert!(matches!(
            events.as_slice(),
            [
                Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 11,
                    output_tokens: 7,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                })),
                Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)),
            ]
        ));
    }

    #[test]
    fn usage_event_precedes_tool_use_stop_event() {
        let mut mapper = LlamaCppEventMapper::new();
        let events = mapper.map_event(llama_cpp::ResponseStreamEvent {
            model: "test-model".to_string(),
            object: "chat.completion.chunk".to_string(),
            choices: vec![llama_cpp::ChoiceDelta {
                index: 0,
                delta: llama_cpp::ResponseMessageDelta {
                    content: None,
                    reasoning_content: None,
                    tool_calls: Some(vec![llama_cpp::ToolCallChunk {
                        index: 0,
                        id: Some("tool-call-id".to_string()),
                        function: Some(llama_cpp::FunctionChunk {
                            name: Some("test_tool".to_string()),
                            arguments: Some(r#"{"value":1}"#.to_string()),
                        }),
                    }]),
                },
                finish_reason: Some("tool_calls".to_string()),
            }],
            usage: Some(llama_cpp::Usage {
                prompt_tokens: 13,
                completion_tokens: 5,
                total_tokens: 18,
            }),
        });

        assert!(matches!(
            events.as_slice(),
            [
                Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 13,
                    output_tokens: 5,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                })),
                Ok(LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                    id,
                    name,
                    ..
                })),
                Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)),
            ] if id.to_string() == "tool-call-id" && name.as_ref() == "test_tool"
        ));
    }

    #[gpui::test]
    async fn authenticate_fetches_models_after_loading_api_key(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        let model_request_authorizations = Arc::new(Mutex::new(Vec::new()));
        let model_request_count = Arc::new(AtomicUsize::new(0));
        let http_client = FakeHttpClient::create({
            let model_request_authorizations = model_request_authorizations.clone();
            let model_request_count = model_request_count.clone();
            move |request| {
                let model_request_authorizations = model_request_authorizations.clone();
                let model_request_count = model_request_count.clone();
                async move {
                    let path = request.uri().path();
                    let authorization = request
                        .headers()
                        .get("Authorization")
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_string);

                    if path == "/v1/models" {
                        model_request_authorizations.lock().push(authorization);
                        let request_index = model_request_count.fetch_add(1, Ordering::SeqCst);
                        if request_index == 0 {
                            return Ok(http_client::Response::builder()
                                .status(503)
                                .body(http_client::AsyncBody::from("not ready"))?);
                        }

                        return Ok(http_client::Response::builder().status(200).body(
                            http_client::AsyncBody::from(
                                r#"{"data":[{"id":"test-model","meta":{"n_ctx":4096}}]}"#,
                            ),
                        )?);
                    }

                    if path == "/props" {
                        return Ok(http_client::Response::builder()
                            .status(200)
                            .body(http_client::AsyncBody::from("{}"))?);
                    }

                    Ok(http_client::Response::builder()
                        .status(404)
                        .body(http_client::AsyncBody::default())?)
                }
            }
        });
        let credentials_provider = Arc::new(FakeCredentialsProvider {
            api_key: b"loaded-key".to_vec(),
        });
        let provider = cx
            .update(|cx| LlamaCppLanguageModelProvider::new(http_client, credentials_provider, cx));

        cx.run_until_parked();

        let result = cx.update(|cx| provider.authenticate(cx)).await;
        assert!(
            result.is_ok(),
            "authenticate should discover models after loading credentials"
        );
        assert_eq!(
            &*model_request_authorizations.lock(),
            &[None, Some("Bearer loaded-key".to_string())]
        );
    }
}
