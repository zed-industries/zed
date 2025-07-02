use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use credentials_provider::CredentialsProvider;
use editor::Editor;
use futures::{FutureExt, Stream, StreamExt, future::BoxFuture};
use gpui::{
    AnyView, App, AsyncApp, Context, Entity, Subscription, Task, Window,
};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolChoice, LanguageModelToolResultContent, LanguageModelToolUse, MessageContent, RateLimiter, Role, StopReason, TokenUsage,
};
use requesty::{
    Model, ModelMode as RequestyModelMode, ResponseStreamEvent, list_models, stream_completion,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::sync::Arc;
use ui::prelude::*;
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
            })?;

            Ok(())
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        if self.fetch_models_task.is_some() {
            return;
        }
        let task = self.fetch_models(cx);
        self.fetch_models_task = Some(task);
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
            _subscription: cx.observe_global::<SettingsStore>(|_this: &mut State, cx| {
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
        IconName::ZedAssistant
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(_cx);
        state.available_models.first().map(|model| self.create_language_model(model.clone()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(_cx);
        state.available_models.first().map(|model| self.create_language_model(model.clone()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let mut models = state.available_models
            .iter()
            .map(|model| self.create_language_model(model.clone()))
            .collect::<Vec<_>>();

        // Add available models from settings
        let settings = &AllLanguageModelSettings::get_global(cx).requesty;
        for model in &settings.available_models {
            let requesty_model = requesty::Model::new(
                &model.name,
                model.max_tokens,
                model.max_output_tokens,
                model.supports_tools.unwrap_or(false),
                model.supports_images.unwrap_or(false),
                model.mode.clone().map(|m| m.into()).unwrap_or_default(),
            );
            models.push(self.create_language_model(requesty_model));
        }

        models
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        let entity = cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx));
        entity.into()
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
        let http_client = self.http_client.clone();
        let state = self.state.clone();

        async move {
            let (api_key, api_url) = state
                .update_in(cx, |state, cx| {
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

            let response = stream_completion(
                http_client.as_ref(),
                &api_url,
                request,
                &api_key,
            )
            .await?;

            Ok(response.boxed())
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
        self.model.supports_tools()
    }

    fn tool_input_format(&self) -> language_model::LanguageModelToolSchemaFormat {
        language_model::LanguageModelToolSchemaFormat::JsonSchema
    }

    fn telemetry_id(&self) -> String {
        format!("requesty-{}", self.model.id())
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
            LanguageModelToolChoice::Any => false,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        cx.background_spawn(async move {
            let mut total_tokens = 0;

            for message in &request.messages {
                for content in &message.content {
                    match content {
                        MessageContent::Text(text) => {
                            total_tokens += text.len() as u64 / 4; // Rough estimate
                        }
                        MessageContent::Image(_) => {
                            total_tokens += 85; // Rough estimate for images
                        }
                        _ => {}
                    }
                }
            }

            Ok(total_tokens)
        }).boxed()
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
        let request_limiter = self.request_limiter.clone();
        let model = self.model.clone();

        async move {
            request_limiter.acquire().await;

            let requesty_request = into_requesty(request, &model, model.max_output_tokens());

            let events = self.stream_completion(requesty_request, &cx).await?;

            let mapped_events = RequestyEventMapper::new().map_stream(events);

            Ok(mapped_events.boxed())
        }
        .boxed()
    }
}

pub fn into_requesty(
    request: LanguageModelRequest,
    model: &Model,
    max_output_tokens: Option<u64>,
) -> requesty::Request {
    let mut messages = Vec::new();

    for msg in request.messages {
        let requesty_msg = into_requesty_message(msg);
        messages.push(requesty_msg);
    }

    let tools = request.tools.into_iter().map(|tool| requesty::ToolDefinition {
        name: tool.name,
        description: tool.description,
        input_schema: tool.input_schema,
    }).collect();

    let tool_choice = request.tool_choice.map(into_requesty_tool_choice);

    requesty::Request {
        model: model.id().to_string(),
        messages,
        tools,
        tool_choice,
        temperature: request.temperature.unwrap_or(0.7),
        max_tokens: max_output_tokens,
        stream: true,
        stop: if request.stop.is_empty() { None } else { Some(request.stop) },
    }
}

fn into_requesty_message(msg: LanguageModelRequestMessage) -> requesty::RequestMessage {
    let content = into_requesty_content_list(msg.content);

    match msg.role {
        Role::User => requesty::RequestMessage::User { content },
        Role::Assistant => requesty::RequestMessage::Assistant {
            content: Some(content),
            tool_calls: None,
        },
        Role::System => requesty::RequestMessage::System { content },
    }
}

fn into_requesty_content_list(content_list: Vec<MessageContent>) -> requesty::MessageContent {
    if content_list.len() == 1 {
        into_requesty_content(content_list.into_iter().next().unwrap())
    } else {
        let parts = content_list.into_iter().map(|content| match content {
            MessageContent::Text(text) => requesty::MessagePart::Text { text },
            MessageContent::Image(image) => requesty::MessagePart::Image {
                source: image.source.to_string(),
            },
            MessageContent::ToolUse(tool_use) => requesty::MessagePart::Text {
                text: format!("Tool use: {} with input: {}", tool_use.name, tool_use.raw_input),
            },
            MessageContent::ToolResult(result) => requesty::MessagePart::Text {
                text: match result.content {
                    LanguageModelToolResultContent::Text(text) => text.to_string(),
                    LanguageModelToolResultContent::Image(_) => "[Image result]".to_string(),
                },
            },
            _ => requesty::MessagePart::Text { text: String::new() },
        }).collect();
        requesty::MessageContent::MultiPart(parts)
    }
}

fn into_requesty_content(content: MessageContent) -> requesty::MessageContent {
    match content {
        MessageContent::Text(text) => requesty::MessageContent::Text(text),
        MessageContent::Image(image) => {
            requesty::MessageContent::MultiPart(vec![requesty::MessagePart::Image {
                source: image.source.to_string(),
            }])
        }
        MessageContent::ToolUse(tool_use) => {
            requesty::MessageContent::Text(format!("Tool use: {} with input: {}", tool_use.name, tool_use.raw_input))
        }
        MessageContent::ToolResult(result) => {
            let text = match result.content {
                LanguageModelToolResultContent::Text(text) => text.to_string(),
                LanguageModelToolResultContent::Image(_) => "[Image result]".to_string(),
            };
            requesty::MessageContent::Text(text)
        }
        _ => requesty::MessageContent::Text(String::new()),
    }
}

fn into_requesty_tool_choice(
    choice: LanguageModelToolChoice,
) -> requesty::ToolChoice {
    match choice {
        LanguageModelToolChoice::Auto => requesty::ToolChoice::Auto,
        LanguageModelToolChoice::Any => requesty::ToolChoice::Auto,
        LanguageModelToolChoice::None => requesty::ToolChoice::None,
    }
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
        events.filter_map(move |result| {
            let events = match result {
                Ok(event) => self.map_event(Ok(event)),
                Err(error) => vec![Err(LanguageModelCompletionError::Other(error))],
            };

            async move {
                if events.is_empty() {
                    None
                } else {
                    Some(futures::stream::iter(events))
                }
            }
        })
        .flatten()
    }

    pub fn map_event(
        &mut self,
        event: Result<ResponseStreamEvent>,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut mapped_events = Vec::new();

        match event {
            Ok(ResponseStreamEvent::Text(text)) => {
                mapped_events.push(Ok(LanguageModelCompletionEvent::Text(text)));
            }
            Ok(ResponseStreamEvent::Usage(usage)) => {
                mapped_events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: usage.input_tokens,
                    output_tokens: usage.output_tokens,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                })));
            }
            Ok(ResponseStreamEvent::Reasoning(reasoning)) => {
                mapped_events.push(Ok(LanguageModelCompletionEvent::Thinking {
                    text: reasoning,
                    signature: None,
                }));
            }
            Ok(ResponseStreamEvent::Stop(reason)) => {
                let stop_reason = match reason.as_str() {
                    "stop" => StopReason::EndTurn,
                    "length" => StopReason::MaxTokens,
                    "tool_calls" => StopReason::ToolUse,
                    _ => StopReason::EndTurn,
                };
                mapped_events.push(Ok(LanguageModelCompletionEvent::Stop(stop_reason)));
            }
            Ok(ResponseStreamEvent::ToolCallStart { id, name, index }) => {
                self.tool_calls_by_index.insert(index, RawToolCall {
                    id,
                    name,
                    arguments: String::new(),
                });
            }
            Ok(ResponseStreamEvent::ToolCallDelta { index, delta }) => {
                if let Some(tool_call) = self.tool_calls_by_index.get_mut(&index) {
                    tool_call.arguments.push_str(&delta);
                }
            }
            Ok(ResponseStreamEvent::ToolCallEnd { index }) => {
                if let Some(entry) = self.tool_calls_by_index.remove(&index) {
                    if let Ok(input) = serde_json::from_str(&entry.arguments) {
                        mapped_events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: entry.id.into(),
                                name: entry.name.into(),
                                raw_input: entry.arguments,
                                input,
                                is_input_complete: true,
                            }
                        )));
                    }
                }
            }
            Err(error) => {
                mapped_events.push(Err(LanguageModelCompletionError::Other(error)));
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

struct ConfigurationView {
    api_key_editor: Entity<Editor>,
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

const REQUESTY_SIGN_UP_URL: &str = "https://requesty.dev/signup";

impl ConfigurationView {
    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| Editor::single_line(window, cx));

        let load_credentials_task = Some(Self::load_credentials(
            api_key_editor.clone(),
            state.clone(),
            window,
            cx,
        ));

        Self {
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn load_credentials(
        api_key_editor: Entity<Editor>,
        state: gpui::Entity<State>,
        window: &mut Window,
        cx: &mut Context<ConfigurationView>,
    ) -> Task<()> {
        cx.spawn(async move |this, mut cx| {
            if let Some(this) = this.upgrade() {
                let credentials_provider = <dyn CredentialsProvider>::global(cx);
                let api_url = state.read_with(cx, |state, cx| {
                    let settings = &AllLanguageModelSettings::get_global(cx).requesty;
                    if settings.api_url.is_empty() {
                        requesty::REQUESTY_API_URL.to_string()
                    } else {
                        settings.api_url.clone()
                    }
                }).await?;

                if let Ok((_, api_key)) = credentials_provider
                    .read_credentials(&api_url, &cx)
                    .await
                {
                    if let Ok(api_key) = String::from_utf8(api_key) {
                        api_key_editor.update_in(cx, |editor, window, cx| {
                            editor.set_text(api_key, window, cx);
                        }).await?;
                    }
                }

                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })?;
            }
            anyhow::Ok(())
        }).detach_and_log_err(cx);

        Task::ready(())
    }

    fn save_api_key(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();

        if !api_key.is_empty() {
            let state = self.state.clone();
            cx.spawn(async move |_, cx| {
                state.read_with(cx, |state, cx| {
                    state.reset_api_key(cx)
                }).await??;
                anyhow::Ok(())
            }).detach_and_log_err(cx);
        }
    }

    fn reset_api_key(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        // Implementation for reset
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).api_key_from_env
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.should_render_editor(cx) {
            return div()
                .child(
                    InstructionListItem::new("Requesty API Key", None, None).child(
                        Label::new("Configured via REQUESTY_API_KEY environment variable")
                    )
                );
        }

        div()
            .child(
                InstructionListItem::new("Requesty API Key", None, None).child(
                    div()
                        .child(self.api_key_editor.clone())
                        .child(
                            div()
                                .child(
                                    Button::new("save_key", "Save")
                                        .on_click(cx.listener(Self::save_api_key))
                                )
                                .child(
                                    Button::new("reset_key", "Reset")
                                        .on_click(cx.listener(Self::reset_api_key))
                                )
                        )
                )
            )
            .child(
                div()
                    .child(
                        Button::new("sign_up", "Sign up for Requesty")
                            .on_click(|_, _, cx| cx.open_url(REQUESTY_SIGN_UP_URL))
                    )
            )
    }
}
