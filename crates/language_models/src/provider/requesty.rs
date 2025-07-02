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
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolChoice, LanguageModelToolResultContent,
    LanguageModelToolUse, MessageContent, RateLimiter, Role, StopReason, TokenUsage,
};
use requesty::{
    Model, ModelMode as RequestyModelMode, ResponseStreamEvent, list_models, stream_completion,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::pin::Pin;
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
        let api_url = AllLanguageModelSettings::get_global(cx)
            .requesty
            .api_url
            .clone();
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
        let api_url = AllLanguageModelSettings::get_global(cx)
            .requesty
            .api_url
            .clone();
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
        let api_url = AllLanguageModelSettings::get_global(cx)
            .requesty
            .api_url
            .clone();
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
        let api_url = settings.api_url.clone();
        let api_key = self.api_key.clone();

        cx.spawn(async move |this, cx| {
            if let Some(api_key) = api_key {
                // For Requesty, we need to pass the API key when fetching models
                let models = list_models_with_key(http_client.as_ref(), &api_url, &api_key).await?;
                this.update(cx, |this, cx| {
                    this.available_models = models;
                    cx.notify();
                })
            } else {
                Ok(())
            }
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
        let mut models_from_api = self.state.read(cx).available_models.clone();
        let mut settings_models = Vec::new();

        for model in &AllLanguageModelSettings::get_global(cx)
            .requesty
            .available_models
        {
            settings_models.push(requesty::Model::new(
                &model.name,
                model.display_name.as_deref(),
                Some(model.max_tokens),
                model.supports_tools,
                model.supports_images,
                model.mode.clone().map(|m| m.into()),
            ));
        }

        for settings_model in &settings_models {
            if let Some(pos) = models_from_api
                .iter()
                .position(|m| m.id() == settings_model.id())
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

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.reset_api_key(cx))
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
        let Ok((api_key, api_url)) = cx.read_entity(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).requesty;
            (state.api_key.clone(), settings.api_url.clone())
        }) else {
            return futures::future::ready(Err(anyhow!(
                "App state dropped: Unable to read API key or API URL from the application state"
            )))
            .boxed();
        };

        let future = self.request_limiter.stream(async move {
            let api_key = api_key.ok_or_else(|| anyhow!("Missing Requesty API Key"))?;
            let request = stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
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

    fn tool_input_format(&self) -> language_model::LanguageModelToolSchemaFormat {
        language_model::LanguageModelToolSchemaFormat::JsonSchema
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
            LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Any => false,
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
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = RequestyEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
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

    let tools = request.tools.into_iter().map(|tool| requesty::ToolDefinition::Function {
        function: requesty::FunctionDefinition {
            name: tool.name,
            description: Some(tool.description),
            parameters: Some(tool.input_schema),
        },
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
        stop: request.stop,
        parallel_tool_calls: None,
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

fn into_requesty_message(msg: LanguageModelRequestMessage) -> requesty::RequestMessage {
    let content = into_requesty_content_list(msg.content);

    match msg.role {
        Role::User => requesty::RequestMessage::User { content },
        Role::Assistant => requesty::RequestMessage::Assistant {
            content: Some(content),
            tool_calls: Vec::new(),
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
                image_url: image.source.to_string(),
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
        requesty::MessageContent::Multipart(parts)
    }
}

fn into_requesty_content(content: MessageContent) -> requesty::MessageContent {
    match content {
        MessageContent::Text(text) => requesty::MessageContent::Plain(text),
        MessageContent::Image(image) => {
            requesty::MessageContent::Multipart(vec![requesty::MessagePart::Image {
                image_url: image.source.to_string(),
            }])
        }
        MessageContent::ToolUse(tool_use) => {
            requesty::MessageContent::Plain(format!("Tool use: {} with input: {}", tool_use.name, tool_use.raw_input))
        }
        MessageContent::ToolResult(result) => {
            let text = match result.content {
                LanguageModelToolResultContent::Text(text) => text.to_string(),
                LanguageModelToolResultContent::Image(_) => "[Image result]".to_string(),
            };
            requesty::MessageContent::Plain(text)
        }
        _ => requesty::MessageContent::Plain(String::new()),
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
            tool_calls_by_index: HashMap::default(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseStreamEvent>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(LanguageModelCompletionError::from(anyhow!(error)))],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: ResponseStreamEvent,
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

        for choice in event.choices {
            let delta = choice.delta;

            if let Some(content) = delta.content {
                events.push(Ok(LanguageModelCompletionEvent::Text(content)));
            }

            if let Some(reasoning) = delta.reasoning {
                events.push(Ok(LanguageModelCompletionEvent::Thinking {
                    text: reasoning,
                    signature: None,
                }));
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
                }
            }

            if let Some(finish_reason) = choice.finish_reason {
                match finish_reason.as_str() {
                    "stop" => {
                        events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
                    }
                    "tool_calls" => {
                        events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                            match serde_json::from_str(&tool_call.arguments) {
                                Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                                    LanguageModelToolUse {
                                        id: tool_call.id.clone().into(),
                                        name: tool_call.name.as_str().into(),
                                        is_input_complete: true,
                                        input,
                                        raw_input: tool_call.arguments.clone().into(),
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
                    stop_reason => {
                        log::error!("Unexpected Requesty stop_reason: {stop_reason:?}",);
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

pub fn count_requesty_tokens(
    request: LanguageModelRequest,
    _model: requesty::Model,
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

async fn list_models_with_key(
    http_client: &dyn HttpClient,
    api_url: &str,
    _api_key: &str,
) -> Result<Vec<requesty::Model>> {
    // Use the correct models endpoint with API key
    let models_url = format!("{}/models", api_url.trim_end_matches('/'));
    list_models(http_client, &models_url).await
}

struct ConfigurationView {
    api_key_editor: Entity<Editor>,
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor
                .set_placeholder_text("rq_00000000000000000000000000000000000000000000000000", cx);
            editor
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
        let api_key = self.api_key_editor.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(api_key, cx))?
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state.update(cx, |state, cx| state.reset_api_key(cx))?.await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn render_api_key_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            white_space: WhiteSpace::Normal,
            ..Default::default()
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
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_from_env;

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials...")).into_any()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Zed's assistant with Requesty, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(InstructionListItem::new(
                            "Create an API key by visiting",
                            Some("Requesty's sign up"),
                            Some("https://app.requesty.ai/sign-up?ref_referrer=Zed"),
                        ))
                        .child(InstructionListItem::text_only(
                            "Ensure your Requesty account has credits",
                        ))
                        .child(InstructionListItem::text_only(
                            "Paste your API key below and hit enter to start using the assistant",
                        )),
                )
                .child(
                    h_flex()
                        .w_full()
                        .my_2()
                        .px_2()
                        .py_1()
                        .bg(cx.theme().colors().editor_background)
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .rounded_sm()
                        .child(self.render_api_key_editor(cx)),
                )
                .child(
                    Label::new(
                        format!("You can also assign the {REQUESTY_API_KEY_VAR} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any()
        } else {
            h_flex()
                .mt_1()
                .p_1()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(if env_var_set {
                            format!("API key set in {REQUESTY_API_KEY_VAR} environment variable.")
                        } else {
                            "API key configured.".to_string()
                        })),
                )
                .child(
                    Button::new("reset-key", "Reset Key")
                        .label_size(LabelSize::Small)
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .disabled(env_var_set)
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your API key, unset the {REQUESTY_API_KEY_VAR} environment variable.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                )
                .into_any()
        }
    }
}
