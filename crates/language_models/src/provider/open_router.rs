use anyhow::{Result, anyhow};
use collections::HashMap;
use futures::{FutureExt, Stream, StreamExt, future, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolResultContent, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, MessageContent, RateLimiter, Role, StopReason, TokenUsage,
};
use open_router::{
    Model, ModelMode as OpenRouterModelMode, OPEN_ROUTER_API_URL, ResponseStreamEvent, list_models,
};
use settings::{OpenRouterAvailableModel as AvailableModel, Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr as _;
use std::sync::{Arc, LazyLock};
use ui::{List, prelude::*};
use ui_input::InputField;
use util::ResultExt;
use zed_env_vars::{EnvVar, env_var};

use crate::ui::ConfiguredApiCard;
use crate::{api_key::ApiKeyState, ui::InstructionListItem};

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("openrouter");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("OpenRouter");

const API_KEY_ENV_VAR_NAME: &str = "OPENROUTER_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenRouterSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct OpenRouterLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<open_router::Model>,
    fetch_models_task: Option<Task<Result<(), LanguageModelCompletionError>>>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = OpenRouterLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = OpenRouterLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.load_if_needed(
            api_url,
            &API_KEY_ENV_VAR,
            |this| &mut this.api_key_state,
            cx,
        );

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn fetch_models(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<Result<(), LanguageModelCompletionError>> {
        let http_client = self.http_client.clone();
        let api_url = OpenRouterLanguageModelProvider::api_url(cx);
        let Some(api_key) = self.api_key_state.key(&api_url) else {
            return Task::ready(Err(LanguageModelCompletionError::NoApiKey {
                provider: PROVIDER_NAME,
            }));
        };
        cx.spawn(async move |this, cx| {
            let models = list_models(http_client.as_ref(), &api_url, &api_key)
                .await
                .map_err(|e| {
                    LanguageModelCompletionError::Other(anyhow::anyhow!(
                        "OpenRouter error: {:?}",
                        e
                    ))
                })?;

            this.update(cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
            .map_err(|e| LanguageModelCompletionError::Other(e))?;

            Ok(())
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        if self.is_authenticated() {
            let task = self.fetch_models(cx);
            self.fetch_models_task.replace(task);
        } else {
            self.available_models = Vec::new();
        }
    }
}

impl OpenRouterLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>({
                let mut last_settings = OpenRouterLanguageModelProvider::settings(cx).clone();
                move |this: &mut State, cx| {
                    let current_settings = OpenRouterLanguageModelProvider::settings(cx);
                    let settings_changed = current_settings != &last_settings;
                    if settings_changed {
                        last_settings = current_settings.clone();
                        this.authenticate(cx).detach();
                        cx.notify();
                    }
                }
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx)),
                http_client: http_client.clone(),
                available_models: Vec::new(),
                fetch_models_task: None,
            }
        });

        Self { http_client, state }
    }

    fn settings(cx: &App) -> &OpenRouterSettings {
        &crate::AllLanguageModelSettings::get_global(cx).open_router
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            OPEN_ROUTER_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }

    fn create_language_model(&self, model: open_router::Model) -> Arc<dyn LanguageModel> {
        Arc::new(OpenRouterLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for OpenRouterLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenRouterLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconName {
        IconName::AiOpenRouter
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_router::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_router::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models_from_api = self.state.read(cx).available_models.clone();
        let mut settings_models = Vec::new();

        for model in &Self::settings(cx).available_models {
            settings_models.push(open_router::Model {
                name: model.name.clone(),
                display_name: model.display_name.clone(),
                max_tokens: model.max_tokens,
                supports_tools: model.supports_tools,
                supports_images: model.supports_images,
                mode: model.mode.unwrap_or_default(),
                provider: model.provider.clone(),
            });
        }

        for settings_model in &settings_models {
            if let Some(pos) = models_from_api
                .iter()
                .position(|m| m.name == settings_model.name)
            {
                models_from_api[pos] = settings_model.clone();
            } else {
                models_from_api.push(settings_model.clone());
            }
        }

        models_from_api
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

pub struct OpenRouterLanguageModel {
    id: LanguageModelId,
    model: open_router::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenRouterLanguageModel {
    fn stream_completion(
        &self,
        request: open_router::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<ResponseStreamEvent, open_router::OpenRouterError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();
        let Ok((api_key, api_url)) = self.state.read_with(cx, |state, cx| {
            let api_url = OpenRouterLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        }) else {
            return future::ready(Err(anyhow!("App state dropped").into())).boxed();
        };

        async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request =
                open_router::stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            request.await.map_err(Into::into)
        }
        .boxed()
    }
}

impl LanguageModel for OpenRouterLanguageModel {
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
        self.model.supports_tool_calls()
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        let model_id = self.model.id().trim().to_lowercase();
        if model_id.contains("gemini") || model_id.contains("grok") {
            LanguageModelToolSchemaFormat::JsonSchemaSubset
        } else {
            LanguageModelToolSchemaFormat::JsonSchema
        }
    }

    fn telemetry_id(&self) -> String {
        format!("openrouter/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Any => true,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images.unwrap_or(false)
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        count_open_router_tokens(request, self.model.clone(), cx)
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
        let request = into_open_router(request, &self.model, self.max_output_tokens());
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await?;
            Ok(OpenRouterEventMapper::new().map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

pub fn into_open_router(
    request: LanguageModelRequest,
    model: &Model,
    max_output_tokens: Option<u64>,
) -> open_router::Request {
    let mut messages = Vec::new();
    for message in request.messages {
        for content in message.content {
            match content {
                MessageContent::Text(text) => add_message_content_part(
                    open_router::MessagePart::Text { text },
                    message.role,
                    &mut messages,
                ),
                MessageContent::Thinking { .. } => {}
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Image(image) => {
                    add_message_content_part(
                        open_router::MessagePart::Image {
                            image_url: image.to_base64_url(),
                        },
                        message.role,
                        &mut messages,
                    );
                }
                MessageContent::ToolUse(tool_use) => {
                    let tool_call = open_router::ToolCall {
                        id: tool_use.id.to_string(),
                        content: open_router::ToolCallContent::Function {
                            function: open_router::FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                            },
                        },
                    };

                    if let Some(open_router::RequestMessage::Assistant { tool_calls, .. }) =
                        messages.last_mut()
                    {
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(open_router::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                        });
                    }
                }
                MessageContent::ToolResult(tool_result) => {
                    let content = match &tool_result.content {
                        LanguageModelToolResultContent::Text(text) => {
                            vec![open_router::MessagePart::Text {
                                text: text.to_string(),
                            }]
                        }
                        LanguageModelToolResultContent::Image(image) => {
                            vec![open_router::MessagePart::Image {
                                image_url: image.to_base64_url(),
                            }]
                        }
                    };

                    messages.push(open_router::RequestMessage::Tool {
                        content: content.into(),
                        tool_call_id: tool_result.tool_use_id.to_string(),
                    });
                }
            }
        }
    }

    open_router::Request {
        model: model.id().into(),
        messages,
        stream: true,
        stop: request.stop,
        temperature: request.temperature.unwrap_or(0.4),
        max_tokens: max_output_tokens,
        parallel_tool_calls: if model.supports_parallel_tool_calls() && !request.tools.is_empty() {
            Some(false)
        } else {
            None
        },
        usage: open_router::RequestUsage { include: true },
        reasoning: if request.thinking_allowed
            && let OpenRouterModelMode::Thinking { budget_tokens } = model.mode
        {
            Some(open_router::Reasoning {
                effort: None,
                max_tokens: budget_tokens,
                exclude: Some(false),
                enabled: Some(true),
            })
        } else {
            None
        },
        tools: request
            .tools
            .into_iter()
            .map(|tool| open_router::ToolDefinition::Function {
                function: open_router::FunctionDefinition {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect(),
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => open_router::ToolChoice::Auto,
            LanguageModelToolChoice::Any => open_router::ToolChoice::Required,
            LanguageModelToolChoice::None => open_router::ToolChoice::None,
        }),
        provider: model.provider.clone(),
    }
}

fn add_message_content_part(
    new_part: open_router::MessagePart,
    role: Role,
    messages: &mut Vec<open_router::RequestMessage>,
) {
    match (role, messages.last_mut()) {
        (Role::User, Some(open_router::RequestMessage::User { content }))
        | (Role::System, Some(open_router::RequestMessage::System { content })) => {
            content.push_part(new_part);
        }
        (
            Role::Assistant,
            Some(open_router::RequestMessage::Assistant {
                content: Some(content),
                ..
            }),
        ) => {
            content.push_part(new_part);
        }
        _ => {
            messages.push(match role {
                Role::User => open_router::RequestMessage::User {
                    content: open_router::MessageContent::from(vec![new_part]),
                },
                Role::Assistant => open_router::RequestMessage::Assistant {
                    content: Some(open_router::MessageContent::from(vec![new_part])),
                    tool_calls: Vec::new(),
                },
                Role::System => open_router::RequestMessage::System {
                    content: open_router::MessageContent::from(vec![new_part]),
                },
            });
        }
    }
}

pub struct OpenRouterEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl OpenRouterEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<
            Box<
                dyn Send + Stream<Item = Result<ResponseStreamEvent, open_router::OpenRouterError>>,
            >,
        >,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(error.into())],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: ResponseStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let Some(choice) = event.choices.first() else {
            return vec![Err(LanguageModelCompletionError::from(anyhow!(
                "Response contained no choices"
            )))];
        };

        let mut events = Vec::new();
        if let Some(reasoning) = choice.delta.reasoning.clone() {
            events.push(Ok(LanguageModelCompletionEvent::Thinking {
                text: reasoning,
                signature: None,
            }));
        }

        if let Some(content) = choice.delta.content.clone() {
            // OpenRouter send empty content string with the reasoning content
            // This is a workaround for the OpenRouter API bug
            if !content.is_empty() {
                events.push(Ok(LanguageModelCompletionEvent::Text(content)));
            }
        }

        if let Some(tool_calls) = choice.delta.tool_calls.as_ref() {
            for tool_call in tool_calls {
                let entry = self.tool_calls_by_index.entry(tool_call.index).or_default();

                if let Some(tool_id) = tool_call.id.clone() {
                    entry.id = tool_id;
                }

                if let Some(function) = tool_call.function.as_ref() {
                    if let Some(name) = function.name.clone() {
                        entry.name = name;
                    }

                    if let Some(arguments) = function.arguments.clone() {
                        entry.arguments.push_str(&arguments);
                    }
                }
            }
        }

        if let Some(usage) = event.usage {
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            })));
        }

        match choice.finish_reason.as_deref() {
            Some("stop") => {
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            Some("tool_calls") => {
                events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                    match serde_json::Value::from_str(&tool_call.arguments) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_call.id.clone().into(),
                                name: tool_call.name.as_str().into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_call.arguments.clone(),
                            },
                        )),
                        Err(error) => Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                            id: tool_call.id.clone().into(),
                            tool_name: tool_call.name.as_str().into(),
                            raw_input: tool_call.arguments.clone().into(),
                            json_parse_error: error.to_string(),
                        }),
                    }
                }));

                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
            }
            Some(stop_reason) => {
                log::error!("Unexpected OpenRouter stop_reason: {stop_reason:?}",);
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            None => {}
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

pub fn count_open_router_tokens(
    request: LanguageModelRequest,
    _model: open_router::Model,
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

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            InputField::new(
                window,
                cx,
                "sk_or_000000000000000000000000000000000000000000000000",
            )
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
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

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))?
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
                .update(cx, |state, cx| state.set_api_key(None, cx))?
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
            let api_url = OpenRouterLanguageModelProvider::api_url(cx);
            if api_url == OPEN_ROUTER_API_URL {
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
                .child(Label::new("To use Zed's agent with OpenRouter, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(InstructionListItem::new(
                            "Create an API key by visiting",
                            Some("OpenRouter's console"),
                            Some("https://openrouter.ai/keys"),
                        ))
                        .child(InstructionListItem::text_only(
                            "Ensure your OpenRouter account has credits",
                        ))
                        .child(InstructionListItem::text_only(
                            "Paste your API key below and hit enter to start using the assistant",
                        )),
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(
                        format!("You can also assign the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!("To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."))
                })
                .into_any_element()
        }
    }
}
