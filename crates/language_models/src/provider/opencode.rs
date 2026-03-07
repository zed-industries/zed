use anyhow::Result;
use collections::BTreeMap;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, RateLimiter, Role, env_var,
};
use opencode::{ApiProtocol, OPENCODE_API_URL};
pub use settings::OpenCodeAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

use crate::provider::anthropic::{AnthropicEventMapper, into_anthropic};
use crate::provider::google::{GoogleEventMapper, into_google};
use crate::provider::open_ai::{
    OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai, into_open_ai_response,
};

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("opencode");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("OpenCode Zen");

const API_KEY_ENV_VAR_NAME: &str = "OPENCODE_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenCodeSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct OpenCodeLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = OpenCodeLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = OpenCodeLanguageModelProvider::api_url(cx);
        self.api_key_state
            .load_if_needed(api_url, |this| &mut this.api_key_state, cx)
    }
}

impl OpenCodeLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let api_url = Self::api_url(cx);
                this.api_key_state
                    .handle_url_change(api_url, |this| &mut this.api_key_state, cx);
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: opencode::Model) -> Arc<dyn LanguageModel> {
        Arc::new(OpenCodeLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    pub fn settings(cx: &App) -> &OpenCodeSettings {
        &crate::AllLanguageModelSettings::get_global(cx).opencode
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
        Some(self.create_language_model(opencode::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(opencode::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        for model in opencode::Model::iter() {
            if !matches!(model, opencode::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        for model in &Self::settings(cx).available_models {
            let protocol = match model.protocol.as_str() {
                "anthropic" => ApiProtocol::Anthropic,
                "openai_responses" => ApiProtocol::OpenAiResponses,
                "openai_chat" => ApiProtocol::OpenAiChat,
                "google" => ApiProtocol::Google,
                _ => ApiProtocol::OpenAiChat, // default fallback
            };
            models.insert(
                model.name.clone(),
                opencode::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    protocol,
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
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenCodeLanguageModel {
    /// Returns the base API URL (e.g., "https://opencode.ai/zen").
    fn base_api_url(&self, cx: &AsyncApp) -> SharedString {
        self.state
            .read_with(cx, |_, cx| OpenCodeLanguageModelProvider::api_url(cx))
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
        let http_client = self.http_client.clone();
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
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<open_ai::ResponseStreamEvent>>>,
    > {
        let http_client = self.http_client.clone();
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
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<open_ai::responses::StreamEvent>>>,
    > {
        let http_client = self.http_client.clone();
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

    fn stream_google_zen(
        &self,
        request: google_ai::GenerateContentRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<google_ai::GenerateContentResponse>>>,
    > {
        let http_client = self.http_client.clone();
        let api_url = self.base_api_url(cx);
        let api_key = self.api_key(cx);

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = opencode::stream_generate_content_zen(
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
        LanguageModelName::from(self.model.display_name().to_string())
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
        format!("opencode/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        cx.background_spawn(async move {
            let messages = request
                .messages
                .into_iter()
                .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
                    role: match message.role {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    },
                    content: Some(message.string_contents()),
                    name: None,
                    function_call: None,
                })
                .collect::<Vec<_>>();

            tiktoken_rs::num_tokens_from_messages("gpt-4o", &messages).map(|tokens| tokens as u64)
        })
        .boxed()
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
        match self.model.protocol() {
            ApiProtocol::Anthropic => {
                let anthropic_request = into_anthropic(
                    request,
                    self.model.id().to_string(),
                    1.0,
                    self.model.max_output_tokens().unwrap_or(8192),
                    anthropic::AnthropicModelMode::Default,
                );
                let stream = self.stream_anthropic(anthropic_request, cx);
                async move {
                    let mapper = AnthropicEventMapper::new();
                    Ok(mapper.map_stream(stream.await?).boxed())
                }
                .boxed()
            }
            ApiProtocol::OpenAiChat => {
                let openai_request = into_open_ai(
                    request,
                    self.model.id(),
                    false,
                    false,
                    self.model.max_output_tokens(),
                    None,
                );
                let stream = self.stream_openai_chat(openai_request, cx);
                async move {
                    let mapper = OpenAiEventMapper::new();
                    Ok(mapper.map_stream(stream.await?).boxed())
                }
                .boxed()
            }
            ApiProtocol::OpenAiResponses => {
                let response_request = into_open_ai_response(
                    request,
                    self.model.id(),
                    false,
                    false,
                    self.model.max_output_tokens(),
                    None,
                );
                let stream = self.stream_openai_response(response_request, cx);
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
                let stream = self.stream_google_zen(google_request, cx);
                async move {
                    let mapper = GoogleEventMapper::new();
                    Ok(mapper
                        .map_stream(stream.await?.map(|r| r.map_err(Into::into)).boxed())
                        .boxed())
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
                    "To use OpenCode Zen models in Zed, you need an API key:",
                ))
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Sign in and get your key at"))
                                .child(ButtonLink::new(
                                    "OpenCode Zen Console",
                                    "https://opencode.ai/zen",
                                )),
                        )
                        .child(ListBulletItem::new(
                            "Paste your API key below and hit enter to start using OpenCode Zen",
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
            v_flex().size_full().child(api_key_section).into_any()
        }
    }
}
