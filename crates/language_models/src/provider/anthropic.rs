pub mod telemetry;

use anthropic::{ANTHROPIC_API_URL, AnthropicError, AnthropicModelMode};
use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Entity, Task};
use http_client::HttpClient;
use language_model::{
    ANTHROPIC_PROVIDER_ID, ANTHROPIC_PROVIDER_NAME, ApiKeyState, AuthenticateError,
    ConfigurationViewTargetAgent, EnvVar, IconOrSvg, LanguageModel,
    LanguageModelCacheConfiguration, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, RateLimiter, env_var,
};
use settings::{Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

pub use anthropic::completion::{
    AnthropicEventMapper, count_anthropic_tokens_with_tiktoken, into_anthropic,
    into_anthropic_count_tokens_request,
};
pub use settings::AnthropicAvailableModel as AvailableModel;

const PROVIDER_ID: LanguageModelProviderId = ANTHROPIC_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = ANTHROPIC_PROVIDER_NAME;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AnthropicSettings {
    pub api_url: String,
    /// Extend Zed's list of Anthropic models.
    pub available_models: Vec<AvailableModel>,
}

pub struct AnthropicLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

const API_KEY_ENV_VAR_NAME: &str = "ANTHROPIC_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

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
        let api_url = AnthropicLanguageModelProvider::api_url(cx);
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
        let api_url = AnthropicLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }
}

impl AnthropicLanguageModelProvider {
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

    fn create_language_model(&self, model: anthropic::Model) -> Arc<dyn LanguageModel> {
        Arc::new(AnthropicModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &AnthropicSettings {
        &crate::AllLanguageModelSettings::get_global(cx).anthropic
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            ANTHROPIC_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for AnthropicLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for AnthropicLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiAnthropic)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic::Model::default_fast()))
    }

    fn recommended_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        [anthropic::Model::ClaudeSonnet4_6]
            .into_iter()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from anthropic::Model::iter()
        for model in anthropic::Model::iter() {
            if !matches!(model, anthropic::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in &AnthropicLanguageModelProvider::settings(cx).available_models {
            models.insert(
                model.name.clone(),
                anthropic::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    tool_override: model.tool_override.clone(),
                    cache_configuration: model.cache_configuration.as_ref().map(|config| {
                        anthropic::AnthropicModelCacheConfiguration {
                            max_cache_anchors: config.max_cache_anchors,
                            should_speculate: config.should_speculate,
                            min_total_token: config.min_total_token,
                        }
                    }),
                    max_output_tokens: model.max_output_tokens,
                    default_temperature: model.default_temperature,
                    extra_beta_headers: model.extra_beta_headers.clone(),
                    mode: match model.mode.unwrap_or_default() {
                        settings::ModelMode::Default => AnthropicModelMode::Default,
                        settings::ModelMode::Thinking { budget_tokens } => {
                            AnthropicModelMode::Thinking { budget_tokens }
                        }
                    },
                },
            );
        }

        models
            .into_values()
            .map(|model| self.create_language_model(model))
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
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), target_agent, window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct AnthropicModel {
    id: LanguageModelId,
    model: anthropic::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl AnthropicModel {
    fn stream_completion(
        &self,
        request: anthropic::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<anthropic::Event, AnthropicError>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url) = self.state.read_with(cx, |state, cx| {
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        });

        let beta_headers = self.model.beta_headers();

        async move {
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
                beta_headers,
            );
            request.await.map_err(Into::into)
        }
        .boxed()
    }
}

impl LanguageModel for AnthropicModel {
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
        true
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
        self.model.supports_thinking()
    }

    fn supported_effort_levels(&self) -> Vec<language_model::LanguageModelEffortLevel> {
        if self.model.supports_adaptive_thinking() {
            vec![
                language_model::LanguageModelEffortLevel {
                    name: "Low".into(),
                    value: "low".into(),
                    is_default: false,
                },
                language_model::LanguageModelEffortLevel {
                    name: "Medium".into(),
                    value: "medium".into(),
                    is_default: false,
                },
                language_model::LanguageModelEffortLevel {
                    name: "High".into(),
                    value: "high".into(),
                    is_default: true,
                },
                language_model::LanguageModelEffortLevel {
                    name: "Max".into(),
                    value: "max".into(),
                    is_default: false,
                },
            ]
        } else {
            Vec::new()
        }
    }

    fn telemetry_id(&self) -> String {
        format!("anthropic/{}", self.model.id())
    }

    fn api_key(&self, cx: &App) -> Option<String> {
        self.state.read_with(cx, |state, cx| {
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            state.api_key_state.key(&api_url).map(|key| key.to_string())
        })
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens())
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        let http_client = self.http_client.clone();
        let model_id = self.model.request_id().to_string();
        let mode = self.model.mode();

        let (api_key, api_url) = self.state.read_with(cx, |state, cx| {
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            (
                state.api_key_state.key(&api_url).map(|k| k.to_string()),
                api_url.to_string(),
            )
        });

        let background = cx.background_executor().clone();
        async move {
            // If no API key, fall back to tiktoken estimation
            let Some(api_key) = api_key else {
                return background
                    .spawn(async move { count_anthropic_tokens_with_tiktoken(request) })
                    .await;
            };

            let count_request =
                into_anthropic_count_tokens_request(request.clone(), model_id, mode);

            match anthropic::count_tokens(http_client.as_ref(), &api_url, &api_key, count_request)
                .await
            {
                Ok(response) => Ok(response.input_tokens),
                Err(err) => {
                    log::error!(
                        "Anthropic count_tokens API failed, falling back to tiktoken: {err:?}"
                    );
                    background
                        .spawn(async move { count_anthropic_tokens_with_tiktoken(request) })
                        .await
                }
            }
        }
        .boxed()
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
        let request = into_anthropic(
            request,
            self.model.request_id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
            self.model.mode(),
        );
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await?;
            Ok(AnthropicEventMapper::new().map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        self.model
            .cache_configuration()
            .map(|config| LanguageModelCacheConfiguration {
                max_cache_anchors: config.max_cache_anchors,
                should_speculate: config.should_speculate,
                min_total_token: config.min_total_token,
            })
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
    target_agent: ConfigurationViewTargetAgent,
}

impl ConfigurationView {
    const PLACEHOLDER_TEXT: &'static str = "sk-ant-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";

    fn new(
        state: Entity<State>,
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn({
            let state = state.clone();
            async move |this, cx| {
                let task = state.update(cx, |state, cx| state.authenticate(cx));
                // We don't log an error, because "not signed in" is also an error.
                let _ = task.await;
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor: cx.new(|cx| InputField::new(window, cx, Self::PLACEHOLDER_TEXT)),
            state,
            load_credentials_task,
            target_agent,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
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
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            if api_url == ANTHROPIC_API_URL {
                "API key configured".to_string()
            } else {
                format!("API key configured for {}", api_url)
            }
        };

        if self.load_credentials_task.is_some() {
            div()
                .child(Label::new("Loading credentials..."))
                .into_any_element()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(format!("To use {}, you need to add an API key. Follow these steps:", match &self.target_agent {
                    ConfigurationViewTargetAgent::ZedAgent => "Zed's agent with Anthropic".into(),
                    ConfigurationViewTargetAgent::Other(agent) => agent.clone(),
                })))
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Create one by visiting"))
                                .child(ButtonLink::new("Anthropic's settings", "https://console.anthropic.com/settings/keys"))
                        )
                        .child(
                            ListBulletItem::new("Paste your API key below and hit enter to start using the agent")
                        )
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(
                        format!("You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_0p5(),
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
}
