use anyhow::{Context as _, Result};
use collections::HashMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, TaskExt, Window};
use http_client::{AsyncBody, CustomHeaders, HttpClient, HttpRequestExt, Method, Request as HttpRequest, RequestBuilderExt};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice, RateLimiter, env_var,
};
use menu;
use open_ai::stream_completion;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use ui::{ConfiguredApiCard, IconName, prelude::*};
use ui_input::InputField;
use util::ResultExt;

use crate::provider::open_ai::{OpenAiEventMapper, into_open_ai};
pub use settings::HyperAvailableModel as AvailableModel;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("hyper");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Charm Hyper");

const API_KEY_ENV_VAR_NAME: &str = "HYPER_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

const DEFAULT_API_URL: &str = "https://hyper.charm.land/v1";

const API_KEY_PLACEHOLDER: &str = "hx-00000000000000000000000000000000";

fn normalize_model_id(id: &str) -> String {
    let prefix = "Hyper:";
    if !id.starts_with(prefix) {
        format!("{prefix}{id}")
    } else {
        id.to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HyperModelResponse {
    id: String,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    context_window: Option<u64>,
    #[serde(default)]
    max_output_tokens: Option<u64>,
    #[serde(default)]
    supports_reasoning: Option<bool>,
    #[serde(default)]
    supports_attachments: Option<bool>,
    #[serde(default)]
    supports_tool_call: Option<bool>,
    #[serde(default)]
    supports_reasoning_effort: Option<bool>,
    #[serde(default)]
    reasoning_effort_levels: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HyperModelsListResponse {
    data: Vec<HyperModelResponse>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HyperModel {
    /// Display/navigation ID (e.g. `hyper:deepseek-v4-flash`).
    id: String,
    /// Raw model name Hyper's API expects in chat requests (e.g. `deepseek-v4-flash`).
    api_model_id: String,
    display_name: Option<String>,
    context_window: u64,
    max_output_tokens: Option<u64>,
    supports_reasoning: bool,
    supports_attachments: bool,
    supports_tool_call: bool,
    supports_reasoning_effort: bool,
    reasoning_effort_levels: Vec<String>,
}

impl HyperModel {
    fn display_name(&self) -> String {
        let base = self.display_name.as_deref().unwrap_or(&self.id);
        format!("Hyper:{base}")
    }

    fn max_token_count(&self) -> u64 {
        self.context_window
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct HyperSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct HyperLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    fetched_models: Vec<HyperModel>,
    fetch_model_task: Option<Task<Result<()>>>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        !self.fetched_models.is_empty()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let has_key = api_key.is_some();
        let credentials_provider = self.credentials_provider.clone();
        let api_url = HyperLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        self.fetched_models.clear();
        if has_key {
            cx.spawn(async move |this, cx| {
                let result = task.await;
                if result.is_ok() {
                    this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                        .ok();
                }
                result
            })
        } else {
            task
        }
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = HyperLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        // Always try to fetch models — if a key is available (env var or
        // stored credential) this will succeed and populate the model list.
        // If no key is configured the fetch will fail gracefully and the
        // provider stays in the unauthenticated state.
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let http_client = Arc::clone(&self.http_client);
        let settings = HyperLanguageModelProvider::settings(cx);
        let api_url = HyperLanguageModelProvider::api_url(cx);
        let api_key = self.api_key_state.key(&api_url);
        let extra_headers = settings.custom_headers.clone();

        cx.spawn(async move |this, cx| {
            let uri = format!("{api_url}/models");
            let request = HttpRequest::builder()
                .method(Method::GET)
                .uri(uri)
                .header("Accept", "application/json")
                .when_some(api_key.as_deref(), |builder, api_key| {
                    builder.header("Authorization", format!("Bearer {api_key}"))
                })
                .extra_headers(&extra_headers)
                .body(AsyncBody::default())
                .map_err(|e| anyhow::anyhow!("{e}"))?;

            let mut response = http_client.send(request).await.map_err(|e| anyhow::anyhow!("{e}"))?;

            let status = response.status();
            let mut body = String::new();
            futures::AsyncReadExt::read_to_string(response.body_mut(), &mut body).await?;

            anyhow::ensure!(
                status.is_success(),
                "Hyper API error: {status} {body}",
            );

            let list: HyperModelsListResponse =
                serde_json::from_str(&body).context("failed to parse Hyper models list")?;

            let models: Vec<HyperModel> = list
                .data
                .into_iter()
                .map(|m| HyperModel {
                    id: normalize_model_id(&m.id),
                    api_model_id: m.id,
                    display_name: m.display_name,
                    context_window: m.context_window.unwrap_or(8192),
                    max_output_tokens: m.max_output_tokens,
                    supports_reasoning: m.supports_reasoning.unwrap_or(false),
                    supports_attachments: m.supports_attachments.unwrap_or(false),
                    supports_tool_call: m.supports_tool_call.unwrap_or(true),
                    supports_reasoning_effort: m.supports_reasoning_effort.unwrap_or(false),
                    reasoning_effort_levels: m.reasoning_effort_levels,
                })
                .collect();

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

impl HyperLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let this = Self {
            http_client: http_client.clone(),
            state: cx.new(|cx| {
                cx.observe_global::<SettingsStore>({
                    let mut last_settings = HyperLanguageModelProvider::settings(cx).clone();
                    move |this: &mut State, cx| {
                        let current_settings = HyperLanguageModelProvider::settings(cx);
                        let settings_changed = current_settings != &last_settings;
                        if settings_changed {
                            let url_changed =
                                last_settings.api_url != current_settings.api_url;
                            last_settings = current_settings.clone();
                            if url_changed {
                                let credentials_provider = this.credentials_provider.clone();
                                let api_url = HyperLanguageModelProvider::api_url(cx);
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
                    api_key_state: ApiKeyState::new(
                        HyperLanguageModelProvider::api_url(cx),
                        (*API_KEY_ENV_VAR).clone(),
                    ),
                    credentials_provider,
                }
            }),
        };
        this
    }

    fn settings(cx: &App) -> &HyperSettings {
        &crate::AllLanguageModelSettings::get_global(cx).hyper
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            DEFAULT_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }

    fn create_language_model(&self, model: HyperModel) -> Arc<dyn LanguageModel> {
        Arc::new(HyperLanguageModel {
            id: LanguageModelId::from(model.id.clone()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for HyperLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for HyperLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAiCompat)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let mut models: HashMap<String, HyperModel> = HashMap::default();

        for model in state.fetched_models.iter() {
            models.insert(model.id.clone(), model.clone());
        }

        let settings = Self::settings(cx);
        for available in &settings.available_models {
            let raw_name = available.name.clone();
            let entry = models.entry(normalize_model_id(&raw_name));
            entry.or_insert(HyperModel {
                api_model_id: raw_name,
                id: normalize_model_id(&available.name),
                display_name: available.display_name.clone(),
                context_window: available.max_tokens,
                max_output_tokens: available.max_output_tokens,
                supports_reasoning: false,
                supports_attachments: false,
                supports_tool_call: true,
                supports_reasoning_effort: false,
                reasoning_effort_levels: Vec::new(),
            });
        }

        let mut models: Vec<_> = models
            .into_values()
            .map(|model| self.create_language_model(model))
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

    fn configuration_view_v2(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> language_model::ProviderConfigurationView {
        let state = self.state.clone();
        language_model::ProviderConfigurationView::Inline(
            cx.new(|cx| {
                crate::ApiKeyEditor::new(
                    state,
                    "https://hyper.charm.land/",
                    API_KEY_PLACEHOLDER,
                    |state, _cx| {
                        if state.api_key_state.is_from_env_var() {
                            crate::api_key_editor::ApiKeyStatus::FromEnvVar(
                                state.api_key_state.env_var_name().clone(),
                            )
                        } else if !state.fetched_models.is_empty() {
                            crate::api_key_editor::ApiKeyStatus::Configured
                        } else {
                            crate::api_key_editor::ApiKeyStatus::Unset
                        }
                    },
                    |state, key, cx| {
                        state.update(cx, |state, cx| state.set_api_key(Some(key), cx))
                    },
                    |state, cx| state.update(cx, |state, cx| state.set_api_key(None, cx)),
                    window,
                    cx,
                )
            })
            .into(),
        )
    }
}

pub struct HyperLanguageModel {
    id: LanguageModelId,
    model: HyperModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl HyperLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<open_ai::ResponseStreamEvent>>, LanguageModelCompletionError>>
    {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = HyperLanguageModelProvider::api_url(cx);
            let extra_headers = HyperLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let provider_name = PROVIDER_NAME;
            let request = stream_completion(
                http_client.as_ref(),
                provider_name.0.as_str(),
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

impl LanguageModel for HyperLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tool_call
    }

    fn tool_input_format(&self) -> language_model::LanguageModelToolSchemaFormat {
        language_model::LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn supports_images(&self) -> bool {
        self.model.supports_attachments
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => self.model.supports_tool_call,
            LanguageModelToolChoice::Any => self.model.supports_tool_call,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        self.model.supports_reasoning_effort
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        if !self.model.supports_reasoning_effort {
            return Vec::new();
        }
        self.model
            .reasoning_effort_levels
            .iter()
            .map(|level| {
                let (name, value) = match level.as_str() {
                    "low" => ("Low", "low"),
                    "medium" => ("Medium", "medium"),
                    "high" => ("High", "high"),
                    "xhigh" => ("Extra High", "xhigh"),
                    "max" => ("Max", "max"),
                    other => (other, other),
                };
                LanguageModelEffortLevel {
                    name: name.into(),
                    value: value.into(),
                    is_default: level == "medium",
                }
            })
            .collect()
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("hyper/{}", self.model.id)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
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
        let request = into_open_ai(
            request,
            &self.model.api_model_id,
            true,
            false,
            self.max_output_tokens(),
            None,
            false,
        );
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor =
            cx.new(|cx| InputField::new(window, cx, API_KEY_PLACEHOLDER));

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
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable")
        } else {
            let api_url = HyperLanguageModelProvider::api_url(cx);
            format!("API key configured for {}", api_url)
        };

        if self.load_credentials_task.is_some() {
            div()
                .child(Label::new("Loading credentials..."))
                .into_any_element()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(
                    "To use Charm Hyper in Zed, enter your API key:",
                ))
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
                .into_any_element()
        }
    }
}
