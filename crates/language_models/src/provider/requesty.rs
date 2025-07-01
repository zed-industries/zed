use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use credentials_provider::CredentialsProvider;
use editor::{Editor, EditorElement, EditorStyle};
use futures::{FutureExt, Stream, StreamExt, future::BoxFuture};
use gpui::{
    AnyView, App, AsyncApp, Context, Entity, FontStyle, Subscription, Task, TextStyle, WhiteSpace,
};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolResultContent, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, MessageContent, RateLimiter, Role, StopReason, TokenUsage,
};
use requesty::{
    Model, ModelMode as RequestyModelMode, ResponseStreamEvent, list_models, stream_completion,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr as _;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{Icon, IconName, List, Tooltip, prelude::*};
use util::ResultExt;

use crate::{AllLanguageModelSettings, ui::InstructionListItem};

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("requesty");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Requesty");

#[derive(Default, Clone, Debug, PartialEq)]
pub struct RequestySettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: u64,
    pub max_output_tokens: Option<u64>,
    pub max_completion_tokens: Option<u64>,
    pub supports_tools: Option<bool>,
    pub supports_images: Option<bool>,
    pub mode: Option<ModelMode>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        budget_tokens: Option<u32>,
    },
}

impl From<ModelMode> for RequestyModelMode {
    fn from(value: ModelMode) -> Self {
        match value {
            ModelMode::Default => RequestyModelMode::Default,
            ModelMode::Thinking { budget_tokens } => {
                RequestyModelMode::Thinking { budget_tokens }
            }
        }
    }
}

impl From<RequestyModelMode> for ModelMode {
    fn from(value: RequestyModelMode) -> Self {
        match value {
            RequestyModelMode::Default => ModelMode::Default,
            RequestyModelMode::Thinking { budget_tokens } => {
                ModelMode::Thinking { budget_tokens }
            }
        }
    }
}

pub struct RequestyLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    api_key: Option<String>,
    api_key_from_env: bool,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<requesty::Model>,
    fetch_models_task: Option<Task<Result<()>>>,
    settings: RequestySettings,
    _subscription: Subscription,
}

const REQUESTY_API_KEY_VAR: &str = "REQUESTY_API_KEY";

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn reset_api_key(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let settings = &AllLanguageModelSettings::get_global(cx).requesty;
        let api_url = if settings.api_url.is_empty() {
            requesty::REQUESTY_API_URL.to_string()
        } else {
            settings.api_url.clone()
        };
        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(&api_url, &cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.api_key = None;
                this.api_key_from_env = false;
                cx.notify();
            })
        })
    }

    fn set_api_key(&mut self, api_key: String, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let settings = &AllLanguageModelSettings::get_global(cx).requesty;
        let api_url = if settings.api_url.is_empty() {
            requesty::REQUESTY_API_URL.to_string()
        } else {
            settings.api_url.clone()
        };
        cx.spawn(async move |this, cx| {
            credentials_provider
                .write_credentials(&api_url, "Bearer", api_key.as_bytes(), &cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.api_key = Some(api_key);
                this.restart_fetch_models_task(cx);
                cx.notify();
            })
        })
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let settings = &AllLanguageModelSettings::get_global(cx).requesty;
        let api_url = if settings.api_url.is_empty() {
            requesty::REQUESTY_API_URL.to_string()
        } else {
            settings.api_url.clone()
        };
        cx.spawn(async move |this, cx| {
            let (api_key, from_env) = if let Ok(api_key) = std::env::var(REQUESTY_API_KEY_VAR) {
                (api_key, true)
            } else {
                let (_, api_key) = credentials_provider
                    .read_credentials(&api_url, &cx)
                    .await?
                    .ok_or(AuthenticateError::CredentialsNotFound)?;
                (
                    String::from_utf8(api_key)
                        .context(format!("invalid {} API key", PROVIDER_NAME))?,
                    false,
                )
            };
            this.update(cx, |this, cx| {
                this.api_key = Some(api_key);
                this.api_key_from_env = from_env;
                this.restart_fetch_models_task(cx);
                cx.notify();
            })?;

            Ok(())
        })
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let settings = &AllLanguageModelSettings::get_global(cx).requesty;
        let http_client = self.http_client.clone();
        let api_url = if settings.api_url.is_empty() {
            requesty::REQUESTY_API_URL.to_string()
        } else {
            settings.api_url.clone()
        };

        cx.spawn(async move |this, cx| {
            let models = list_models(http_client.as_ref(), &api_url).await?;

            this.update(cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        if self.is_authenticated() {
            let task = self.fetch_models(cx);
            self.fetch_models_task.replace(task);
        }
    }
}

impl RequestyLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            api_key: None,
            api_key_from_env: false,
            http_client: http_client.clone(),
            available_models: Vec::new(),
            fetch_models_task: None,
            settings: RequestySettings::default(),
            _subscription: cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let current_settings = &AllLanguageModelSettings::get_global(cx).requesty;
                let settings_changed = current_settings != &this.settings;
                if settings_changed {
                    this.settings = current_settings.clone();
                    this.restart_fetch_models_task(cx);
                }
                cx.notify();
            }),
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: requesty::Model) -> Arc<dyn LanguageModel> {
        Arc::new(RequestyLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for RequestyLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for RequestyLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconName {
        IconName::AiRequesty
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(requesty::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(requesty::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);

        let mut models = state.available_models.iter().cloned().collect::<Vec<_>>();

        if models.is_empty() {
            models.push(requesty::Model::default());
        }

        let configured_models = state
            .settings
            .available_models
            .iter()
            .map(|model| requesty::Model {
                name: model.name.clone(),
                display_name: model.display_name.clone(),
                max_tokens: model.max_tokens,
                supports_tools: model.supports_tools,
                supports_images: model.supports_images,
                mode: model.mode.clone().map(Into::into).unwrap_or_default(),
            })
            .collect::<Vec<_>>();

        if !configured_models.is_empty() {
            models.extend(configured_models);
        }

        models
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

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        ConfigurationView::new(self.state.clone(), window, cx).into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.read(cx).reset_api_key(cx)
    }
}

pub struct RequestyLanguageModel {
    id: LanguageModelId,
    model: requesty::Model,
    state: gpui::Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl RequestyLanguageModel {
    fn stream_completion(
        &self,
        request: requesty::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>>>
    {
        let state = self.state.clone();
        let http_client = self.http_client.clone();
        async move {
                    let (api_key, api_url) = state
            .update(cx, |state, cx| {
                let settings = &AllLanguageModelSettings::get_global(cx).requesty;
                let api_url = if settings.api_url.is_empty() {
                    requesty::REQUESTY_API_URL.to_string()
                } else {
                    settings.api_url.clone()
                };
                (state.api_key.clone(), api_url)
            })
            .await?;

            let api_key = api_key.ok_or_else(|| anyhow!("API key not set"))?;
            let stream = stream_completion(http_client.as_ref(), &api_url, &api_key, request).await?;
            Ok(stream)
        }
        .boxed()
    }
}

impl LanguageModel for RequestyLanguageModel {
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
        if self.supports_tools() {
            LanguageModelToolSchemaFormat::JsonSchema
        } else {
            LanguageModelToolSchemaFormat::None
        }
    }

    fn telemetry_id(&self) -> String {
        format!("requesty/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::None | LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Required => false,
            LanguageModelToolChoice::Tool(_) => false,
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
        count_requesty_tokens(request, self.model.clone(), cx)
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
        let request = into_requesty(request, &self.model, self.max_output_tokens());
        let future = self.stream_completion(request, cx);
        let request_limiter = self.request_limiter.clone();

        async move {
            request_limiter.acquire().await;
            let stream = future.await?;
            let mapper = RequestyEventMapper::new();
            Ok(mapper.map_stream(stream).boxed())
        }
        .boxed()
    }
}

pub fn into_requesty(
    request: LanguageModelRequest,
    model: &Model,
    max_output_tokens: Option<u64>,
) -> requesty::Request {
    requesty::Request {
        model: model.id().to_string(),
        messages: request
            .messages
            .into_iter()
            .map(|msg| into_requesty_message(msg))
            .collect(),
        stream: true,
        max_tokens: max_output_tokens,
        stop: request.stop,
        temperature: request.temperature,
        tool_choice: request.tool_choice.map(into_requesty_tool_choice),
        parallel_tool_calls: None,
        tools: request
            .tools
            .into_iter()
            .map(into_requesty_tool)
            .collect(),
        reasoning: match model.mode {
            requesty::ModelMode::Thinking { .. } => Some(requesty::Reasoning {
                effort: None,
                max_tokens: None,
                exclude: None,
                enabled: Some(true),
            }),
            _ => None,
        },
        usage: requesty::RequestUsage { include: true },
    }
}

fn into_requesty_message(msg: language_model::RequestMessage) -> requesty::RequestMessage {
    match msg {
        language_model::RequestMessage::User { content } => requesty::RequestMessage::User {
            content: into_requesty_content(content),
        },
        language_model::RequestMessage::Assistant { content, tool_calls } => {
            requesty::RequestMessage::Assistant {
                content: content.map(into_requesty_content),
                tool_calls: tool_calls
                    .into_iter()
                    .map(into_requesty_tool_call)
                    .collect(),
            }
        }
        language_model::RequestMessage::System { content } => requesty::RequestMessage::System {
            content: into_requesty_content(content),
        },
        language_model::RequestMessage::Tool {
            content,
            tool_call_id,
        } => requesty::RequestMessage::Tool {
            content: into_requesty_content(content),
            tool_call_id,
        },
    }
}

fn into_requesty_content(content: MessageContent) -> requesty::MessageContent {
    match content {
        MessageContent::Text(text) => requesty::MessageContent::Plain(text),
        MessageContent::Multipart(parts) => {
            let requesty_parts = parts
                .into_iter()
                .map(|part| match part {
                    language_model::MessagePart::Text(text) => requesty::MessagePart::Text { text },
                    language_model::MessagePart::Image(image) => requesty::MessagePart::Image {
                        image_url: image.base64,
                    },
                })
                .collect();
            requesty::MessageContent::Multipart(requesty_parts)
        }
    }
}

fn into_requesty_tool_call(tool_call: LanguageModelToolUse) -> requesty::ToolCall {
    requesty::ToolCall {
        id: tool_call.id,
        content: requesty::ToolCallContent::Function {
            function: requesty::FunctionContent {
                name: tool_call.name,
                arguments: tool_call.input.to_string(),
            },
        },
    }
}

fn into_requesty_tool_choice(
    choice: LanguageModelToolChoice,
) -> requesty::ToolChoice {
    match choice {
        LanguageModelToolChoice::None => requesty::ToolChoice::None,
        LanguageModelToolChoice::Auto => requesty::ToolChoice::Auto,
        LanguageModelToolChoice::Required => requesty::ToolChoice::Required,
        LanguageModelToolChoice::Tool(_) => requesty::ToolChoice::Auto,
    }
}

fn into_requesty_tool(tool: language_model::LanguageModelTool) -> requesty::ToolDefinition {
    requesty::ToolDefinition::Function {
        function: requesty::FunctionDefinition {
            name: tool.name,
            description: tool.description,
            parameters: Some(tool.input_schema),
        },
    }
}

fn add_message_content_part(
    new_part: requesty::MessagePart,
    role: Role,
    messages: &mut Vec<requesty::RequestMessage>,
) {
    if let Some(last_message) = messages.last_mut() {
        match (last_message, role) {
            (
                requesty::RequestMessage::User { content },
                Role::User,
            ) => {
                content.push_part(new_part);
                return;
            }
            (
                requesty::RequestMessage::Assistant { content: Some(content), .. },
                Role::Assistant,
            ) => {
                content.push_part(new_part);
                return;
            }
            _ => {}
        }
    }

    let content = requesty::MessageContent::Multipart(vec![new_part]);
    let message = match role {
        Role::User => requesty::RequestMessage::User { content },
        Role::Assistant => requesty::RequestMessage::Assistant {
            content: Some(content),
            tool_calls: Vec::new(),
        },
        Role::System => requesty::RequestMessage::System { content },
        Role::Tool => {
            panic!("Tool messages should not be created without a tool_call_id")
        }
    };
    messages.push(message);
}

pub struct RequestyEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl RequestyEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::new(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseStreamEvent>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(
                self.map_event(event)
                    .into_iter()
                    .map(|result| result.map_err(Into::into)),
            )
        })
    }

    pub fn map_event(
        &mut self,
        event: Result<ResponseStreamEvent>,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut mapped_events = Vec::new();

        match event {
            Ok(event) => {
                if let Some(usage) = event.usage {
                    mapped_events.push(Ok(LanguageModelCompletionEvent::Usage(TokenUsage {
                        input_tokens: usage.prompt_tokens,
                        output_tokens: usage.completion_tokens,
                        total_tokens: usage.total_tokens,
                    })));
                }

                for choice in event.choices {
                    let delta = choice.delta;

                    if let Some(content) = delta.content {
                        mapped_events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                    }

                    if let Some(reasoning) = delta.reasoning {
                        mapped_events.push(Ok(LanguageModelCompletionEvent::Reasoning(reasoning)));
                    }

                    if let Some(tool_calls) = delta.tool_calls {
                        for tool_call_chunk in tool_calls {
                            let index = tool_call_chunk.index;
                            let entry = self.tool_calls_by_index.entry(index).or_insert_with(|| {
                                RawToolCall {
                                    id: String::new(),
                                    name: String::new(),
                                    arguments: String::new(),
                                }
                            });

                            if let Some(id) = tool_call_chunk.id {
                                entry.id = id;
                            }

                            if let Some(function) = tool_call_chunk.function {
                                if let Some(name) = function.name {
                                    entry.name = name;
                                }
                                if let Some(arguments) = function.arguments {
                                    entry.arguments.push_str(&arguments);
                                }
                            }

                            if !entry.id.is_empty() && !entry.name.is_empty() && !entry.arguments.is_empty() {
                                let input = serde_json::from_str(&entry.arguments)
                                    .unwrap_or_else(|_| serde_json::Value::String(entry.arguments.clone()));

                                mapped_events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                                    LanguageModelToolUse {
                                        id: entry.id.clone(),
                                        name: entry.name.clone(),
                                        input,
                                    },
                                )));
                            }
                        }
                    }

                    if let Some(finish_reason) = choice.finish_reason {
                        let stop_reason = match finish_reason.as_str() {
                            "stop" => StopReason::EndTurn,
                            "length" => StopReason::MaxTokens,
                            "tool_calls" => StopReason::ToolUse,
                            "content_filter" => StopReason::ContentFilter,
                            _ => StopReason::EndTurn,
                        };
                        mapped_events.push(Ok(LanguageModelCompletionEvent::Stop(stop_reason)));
                    }
                }
            }
            Err(error) => {
                mapped_events.push(Err(LanguageModelCompletionError::Other(error.into())));
            }
        }

        mapped_events
    }
}

struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
}

pub fn count_requesty_tokens(
    request: LanguageModelRequest,
    _model: requesty::Model,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
    let request = into_requesty(request, &_model, None);

    cx.background_spawn(async move {
        let mut total_tokens = 0;

        for message in &request.messages {
            let content = match message {
                requesty::RequestMessage::User { content } => content,
                requesty::RequestMessage::Assistant { content: Some(content), .. } => content,
                requesty::RequestMessage::System { content } => content,
                requesty::RequestMessage::Tool { content, .. } => content,
                _ => continue,
            };

            let text = content.to_text();
            // Rough approximation: 1 token per 4 characters
            total_tokens += (text.len() as f64 / 4.0).ceil() as u64;
        }

        Ok(total_tokens)
    })
}

struct ConfigurationView {
    api_key_editor: Entity<Editor>,
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("sk-...", cx);
            editor
        });

        let load_credentials_task = Self::load_credentials(
            api_key_editor.clone(),
            state.clone(),
            window,
            cx,
        );

        Self {
            api_key_editor,
            state,
            load_credentials_task: Some(load_credentials_task),
        }
    }

    fn load_credentials(
        api_key_editor: Entity<Editor>,
        state: gpui::Entity<State>,
        window: &mut Window,
        cx: &mut Context<ConfigurationView>,
    ) -> Task<()> {
        cx.spawn(async move |this, cx| {
            if let Some(this) = this.upgrade() {
                let credentials_provider = <dyn CredentialsProvider>::global(&cx);
                let api_url = state.read_with(&cx, |state, cx| {
                    let settings = &AllLanguageModelSettings::get_global(cx).requesty;
                    if settings.api_url.is_empty() {
                        requesty::REQUESTY_API_URL.to_string()
                    } else {
                        settings.api_url.clone()
                    }
                }).await?;

                if let Ok(Some((_, api_key))) = credentials_provider
                    .read_credentials(&api_url, &cx)
                    .await
                {
                    if let Ok(api_key) = String::from_utf8(api_key) {
                        api_key_editor.update_in(&cx, |editor, window, cx| {
                            editor.set_text(api_key, window, cx);
                        })?;
                    }
                }

                this.update(&cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })?;
            }
            anyhow::Ok(())
        }).detach_and_log_err(cx)
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        let state = self.state.clone();
        cx.spawn(async move |_, cx| {
            state.update(cx, |state, cx| {
                state.set_api_key(api_key, cx)
            })?
            .await
        }).detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor.update(cx, |editor, window, cx| {
            editor.set_text("", window, cx);
        });

        let state = self.state.clone();
        cx.spawn(async move |_, cx| {
            state.read_with(&cx, |state, cx| {
                state.reset_api_key(cx)
            }).await?
            .await
        }).detach_and_log_err(cx);
    }

    fn render_api_key_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_size: settings.ui_font.size,
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };
        EditorElement::new(
            &self.api_key_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).api_key_from_env && self.load_credentials_task.is_none()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const REQUESTY_SIGN_UP_URL: &str = "https://app.requesty.ai/onboarding";

        if !self.should_render_editor(cx) {
            return v_flex()
                .gap_1()
                .child(
                    InstructionListItem::new("Requesty API Key").child(
                        Label::new("You can use the REQUESTY_API_KEY environment variable or click the button below to manually set your API key.")
                            .size(LabelSize::Small),
                    ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Button::new("sign_up", "Sign up for Requesty")
                                .icon(IconName::ExternalLink)
                                .icon_size(IconSize::XSmall)
                                .icon_position(IconPosition::End)
                                .on_click(|_, cx| cx.open_url(REQUESTY_SIGN_UP_URL)),
                        )
                        .child(
                            Button::new("set_manually", "Set API key manually")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.state.update(cx, |state, cx| {
                                        state.api_key_from_env = false;
                                        cx.notify();
                                    });
                                })),
                        ),
                )
                .into_any_element();
        }

        v_flex()
            .gap_1()
            .child(
                InstructionListItem::new("Requesty API Key").child(
                    Label::new("To use Requesty models, you need to add your Requesty API key.")
                        .size(LabelSize::Small),
                ),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .child(
                                div()
                                    .flex_1()
                                    .h_8()
                                    .px_3()
                                    .py_1()
                                    .bg(cx.theme().colors().editor_background)
                                    .rounded_md()
                                    .child(self.render_api_key_editor(cx)),
                            )
                            .child(
                                Button::new("save_api_key", "Save")
                                    .on_click(cx.listener(Self::save_api_key)),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("sign_up", "Sign up for Requesty")
                                    .icon(IconName::ExternalLink)
                                    .icon_size(IconSize::XSmall)
                                    .icon_position(IconPosition::End)
                                    .on_click(|_, cx| cx.open_url(REQUESTY_SIGN_UP_URL)),
                            )
                            .child(
                                Button::new("reset_key", "Reset key")
                                    .on_click(cx.listener(Self::reset_api_key)),
                            ),
                    ),
            )
            .into_any_element()
    }
}
