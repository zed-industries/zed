use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use fs::Fs;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::{AsyncBody, HttpClient, http};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice, RateLimiter,
    ReasoningEffort, env_var,
};
use opencode::{ApiProtocol, OPENCODE_API_URL, OpenCodeSubscription};
pub use settings::OpenCodeAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore, update_settings_file};
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::{
    Banner, ButtonLink, ConfiguredApiCard, List, ListBulletItem, Severity, Switch,
    SwitchLabelPosition, ToggleState, prelude::*,
};
use ui_input::InputField;
use util::ResultExt;

use crate::provider::anthropic::{AnthropicEventMapper, into_anthropic};
use crate::provider::google::{GoogleEventMapper, into_google};
use crate::provider::open_ai::{
    OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai, into_open_ai_response,
};

fn normalize_reasoning_effort(effort: &str) -> Option<ReasoningEffort> {
    match effort.trim().to_ascii_lowercase().as_str() {
        "minimal" => Some(ReasoningEffort::Minimal),
        "low" => Some(ReasoningEffort::Low),
        "medium" => Some(ReasoningEffort::Medium),
        "high" => Some(ReasoningEffort::High),
        "max" | "xhigh" => Some(ReasoningEffort::XHigh),
        _ => None,
    }
}

fn reasoning_effort_display(effort: ReasoningEffort) -> (&'static str, &'static str) {
    match effort {
        ReasoningEffort::Minimal => ("Minimal", "minimal"),
        ReasoningEffort::Low => ("Low", "low"),
        ReasoningEffort::Medium => ("Medium", "medium"),
        ReasoningEffort::High => ("High", "high"),
        ReasoningEffort::XHigh => ("Max", "max"),
    }
}

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("opencode");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("OpenCode");

const API_KEY_ENV_VAR_NAME: &str = "OPENCODE_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenCodeSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub show_zen_models: bool,
    pub show_go_models: bool,
    pub show_free_models: bool,
}

pub struct OpenCodeLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
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
            }
        });

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

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        if Self::subscription_enabled(OpenCodeSubscription::Go, cx) {
            // If both Go and Zen are enabled, prefer Go since it's not pay-as-you-go
            Some(
                self.create_language_model(opencode::Model::default_go(), OpenCodeSubscription::Go),
            )
        } else if Self::subscription_enabled(OpenCodeSubscription::Zen, cx) {
            Some(self.create_language_model(opencode::Model::default(), OpenCodeSubscription::Zen))
        } else if Self::subscription_enabled(OpenCodeSubscription::Free, cx) {
            Some(
                self.create_language_model(
                    opencode::Model::default_free(),
                    OpenCodeSubscription::Free,
                ),
            )
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
        } else if Self::subscription_enabled(OpenCodeSubscription::Free, cx) {
            Some(self.create_language_model(
                opencode::Model::default_free_fast(),
                OpenCodeSubscription::Free,
            ))
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

        for model in &settings.available_models {
            let protocol = match model.protocol.as_str() {
                "anthropic" => ApiProtocol::Anthropic,
                "openai_responses" => ApiProtocol::OpenAiResponses,
                "openai_chat" => ApiProtocol::OpenAiChat,
                "google" => ApiProtocol::Google,
                _ => ApiProtocol::OpenAiChat, // default fallback
            };
            let subscription = match model.subscription {
                Some(settings::OpenCodeModelSubscription::Go) => OpenCodeSubscription::Go,
                Some(settings::OpenCodeModelSubscription::Free) => OpenCodeSubscription::Free,
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

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
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

    fn stream_anthropic(
        &self,
        request: anthropic::Request,
        http_client: Arc<dyn HttpClient>,
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
            .is_some_and(|levels| !levels.is_empty())
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        self.model
            .supported_reasoning_effort_levels()
            .map(|levels| {
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

        match self.model.protocol(self.subscription) {
            ApiProtocol::Anthropic => {
                let mode = if self.supports_thinking() && request.thinking_allowed {
                    anthropic::AnthropicModelMode::AdaptiveThinking
                } else {
                    anthropic::AnthropicModelMode::Default
                };
                let anthropic_request = into_anthropic(
                    request,
                    self.model.id().to_string(),
                    1.0,
                    self.model.max_output_tokens().unwrap_or(8192),
                    mode,
                );
                let stream = self.stream_anthropic(anthropic_request, http_client, cx);
                async move {
                    let mapper = AnthropicEventMapper::new();
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
                    self.model.id(),
                    false,
                    false,
                    self.model.max_output_tokens(),
                    reasoning_effort,
                    false,
                );
                let stream = self.stream_openai_chat(openai_request, http_client, cx);
                async move {
                    let mapper = OpenAiEventMapper::new();
                    Ok(mapper.map_stream(stream.await?).boxed())
                }
                .boxed()
            }
            ApiProtocol::OpenAiResponses => {
                let reasoning_effort = if request.thinking_allowed {
                    request
                        .thinking_effort
                        .as_deref()
                        .and_then(normalize_reasoning_effort)
                } else {
                    None
                };
                let response_request = into_open_ai_response(
                    request,
                    self.model.id(),
                    false,
                    false,
                    self.model.max_output_tokens(),
                    reasoning_effort,
                );
                let stream = self.stream_openai_response(response_request, http_client, cx);
                async move {
                    let mapper = OpenAiResponseEventMapper::new();
                    Ok(mapper.map_stream(stream.await?).boxed())
                }
                .boxed()
            }
            ApiProtocol::Google => {
                let google_request = into_google(
                    request,
                    self.model.id().to_string(),
                    google_ai::GoogleModelMode::Default,
                );
                let stream = self.stream_google(google_request, http_client, cx);
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

        let api_key_section = if self.should_render_editor(cx) {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(
                    "To use OpenCode models in Zed, you need an API key:",
                ))
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Sign in and get your key at"))
                                .child(ButtonLink::new(
                                    "OpenCode Console",
                                    "https://opencode.ai/auth",
                                )),
                        )
                        .child(ListBulletItem::new(
                            "Paste your API key below and hit enter to start using OpenCode",
                        )),
                )
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
                .when(env_var_set, |this| {
                    this.tooltip_label(format!(
                        "To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."
                    ))
                })
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .into_any_element()
        };

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials...")).into_any()
        } else {
            let settings = OpenCodeLanguageModelProvider::settings(cx);
            let show_zen = settings.show_zen_models;
            let show_go = settings.show_go_models;
            let show_free = settings.show_free_models;

            let subscription_toggles = v_flex()
                .gap_1()
                .child(Label::new("Subscriptions:").color(Color::Muted))
                .child(
                    Switch::new("opencode-show-zen-models", show_zen.into())
                        .label("Show Zen models")
                        .label_position(SwitchLabelPosition::End)
                        .on_click(cx.listener(|this, state, window, cx| {
                            this.set_subscription_enabled(
                                OpenCodeSubscription::Zen,
                                matches!(state, ToggleState::Selected),
                                window,
                                cx,
                            );
                        })),
                )
                .child(
                    Switch::new("opencode-show-go-models", show_go.into())
                        .label("Show Go models")
                        .label_position(SwitchLabelPosition::End)
                        .on_click(cx.listener(|this, state, window, cx| {
                            this.set_subscription_enabled(
                                OpenCodeSubscription::Go,
                                matches!(state, ToggleState::Selected),
                                window,
                                cx,
                            );
                        })),
                )
                .child(
                    Switch::new("opencode-show-free-models", show_free.into())
                        .label("Show Free models")
                        .label_position(SwitchLabelPosition::End)
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

            v_flex()
                .size_full()
                .gap_2()
                .child(api_key_section)
                .child(subscription_toggles)
                .children(no_subscriptions_warning)
                .into_any()
        }
    }
}
