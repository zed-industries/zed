use anyhow::{Result, anyhow};
use collections::{BTreeMap, HashMap};
use futures::Stream;
use futures::{FutureExt, StreamExt, future, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelImage, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelRequestMessage, LanguageModelToolChoice,
    LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolUse,
    LanguageModelToolUseId, MessageContent, RateLimiter, Role, StopReason, TokenUsage,
};
use menu;
use open_ai::{
    ImageUrl, Model, OPEN_AI_API_URL, ReasoningEffort, ResponseStreamEvent,
    responses::{
        Request as ResponseRequest, ResponseItem as ResponsesItem,
        ResponseSummary as ResponsesSummary, ResponseUsage as ResponsesUsage,
        StreamEvent as ResponsesStreamEvent, stream_response,
    },
    stream_completion,
};
use serde_json::{Value, json};
use settings::{OpenAiAvailableModel as AvailableModel, Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr as _;
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::{ElevationIndex, List, Tooltip, prelude::*};
use ui_input::InputField;
use util::{ResultExt, truncate_and_trailoff};
use zed_env_vars::{EnvVar, env_var};

use crate::{api_key::ApiKeyState, ui::InstructionListItem};

const PROVIDER_ID: LanguageModelProviderId = language_model::OPEN_AI_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = language_model::OPEN_AI_PROVIDER_NAME;

const API_KEY_ENV_VAR_NAME: &str = "OPENAI_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct OpenAiLanguageModelProvider {
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
        let api_url = OpenAiLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = OpenAiLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            &API_KEY_ENV_VAR,
            |this| &mut this.api_key_state,
            cx,
        )
    }
}

impl OpenAiLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let api_url = Self::api_url(cx);
                this.api_key_state.handle_url_change(
                    api_url,
                    &API_KEY_ENV_VAR,
                    |this| &mut this.api_key_state,
                    cx,
                );
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx)),
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: open_ai::Model) -> Arc<dyn LanguageModel> {
        Arc::new(OpenAiLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &OpenAiSettings {
        &crate::AllLanguageModelSettings::get_global(cx).openai
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            open_ai::OPEN_AI_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for OpenAiLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenAiLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconName {
        IconName::AiOpenAi
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_ai::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_ai::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from open_ai::Model::iter()
        for model in open_ai::Model::iter() {
            if !matches!(model, open_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in &OpenAiLanguageModelProvider::settings(cx).available_models {
            if !model.capabilities.chat_completions {
                log::debug!(
                    "Model `{}` does not support /chat/completions; falling back to Responses API",
                    model.name
                );
            }
            models.insert(
                model.name.clone(),
                open_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    max_completion_tokens: model.max_completion_tokens,
                    reasoning_effort: model.reasoning_effort.clone(),
                    supports_chat_completions: model.capabilities.chat_completions,
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

pub struct OpenAiLanguageModel {
    id: LanguageModelId,
    model: open_ai::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>>>
    {
        let http_client = self.http_client.clone();

        let Ok((api_key, api_url)) = self.state.read_with(cx, |state, cx| {
            let api_url = OpenAiLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        }) else {
            return future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn stream_response(
        &self,
        request: ResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponsesStreamEvent>>>>
    {
        let http_client = self.http_client.clone();

        let Ok((api_key, api_url)) = self.state.read_with(cx, |state, cx| {
            let api_url = OpenAiLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        }) else {
            return future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = stream_response(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for OpenAiLanguageModel {
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
        use open_ai::Model;
        match &self.model {
            Model::FourOmni
            | Model::FourOmniMini
            | Model::FourPointOne
            | Model::FourPointOneMini
            | Model::FourPointOneNano
            | Model::Five
            | Model::FiveCodex
            | Model::FiveMini
            | Model::FiveNano
            | Model::O1
            | Model::O3
            | Model::O4Mini => true,
            Model::ThreePointFiveTurbo
            | Model::Four
            | Model::FourTurbo
            | Model::O3Mini
            | Model::Custom { .. } => false,
        }
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Any => true,
            LanguageModelToolChoice::None => true,
        }
    }

    fn telemetry_id(&self) -> String {
        format!("openai/{}", self.model.id())
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
        count_open_ai_tokens(request, self.model.clone(), cx)
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
        if self.model.supports_chat_completions() {
            let request = into_open_ai(
                request,
                self.model.id(),
                self.model.supports_parallel_tool_calls(),
                self.model.supports_prompt_cache_key(),
                self.max_output_tokens(),
                self.model.reasoning_effort(),
            );
            let completions = self.stream_completion(request, cx);
            async move {
                let mapper = OpenAiEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        } else {
            let (request, stop_sequences) = into_open_ai_response(
                request,
                self.model.id(),
                self.model.supports_parallel_tool_calls(),
                self.model.supports_prompt_cache_key(),
                self.max_output_tokens(),
                self.model.reasoning_effort(),
            );
            let completions = self.stream_response(request, cx);
            async move {
                let mapper = ResponseEventMapper::new(stop_sequences);
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        }
    }
}

pub fn into_open_ai(
    request: LanguageModelRequest,
    model_id: &str,
    supports_parallel_tool_calls: bool,
    supports_prompt_cache_key: bool,
    max_output_tokens: Option<u64>,
    reasoning_effort: Option<ReasoningEffort>,
) -> open_ai::Request {
    let stream = !model_id.starts_with("o1-");

    let mut messages = Vec::new();
    for message in request.messages {
        for content in message.content {
            match content {
                MessageContent::Text(text) | MessageContent::Thinking { text, .. } => {
                    add_message_content_part(
                        open_ai::MessagePart::Text { text },
                        message.role,
                        &mut messages,
                    )
                }
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Image(image) => {
                    add_message_content_part(
                        open_ai::MessagePart::Image {
                            image_url: ImageUrl {
                                url: image.to_base64_url(),
                                detail: None,
                            },
                        },
                        message.role,
                        &mut messages,
                    );
                }
                MessageContent::ToolUse(tool_use) => {
                    let tool_call = open_ai::ToolCall {
                        id: tool_use.id.to_string(),
                        content: open_ai::ToolCallContent::Function {
                            function: open_ai::FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                            },
                        },
                    };

                    if let Some(open_ai::RequestMessage::Assistant { tool_calls, .. }) =
                        messages.last_mut()
                    {
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(open_ai::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                        });
                    }
                }
                MessageContent::ToolResult(tool_result) => {
                    let content = match &tool_result.content {
                        LanguageModelToolResultContent::Text(text) => {
                            vec![open_ai::MessagePart::Text {
                                text: text.to_string(),
                            }]
                        }
                        LanguageModelToolResultContent::Image(image) => {
                            vec![open_ai::MessagePart::Image {
                                image_url: ImageUrl {
                                    url: image.to_base64_url(),
                                    detail: None,
                                },
                            }]
                        }
                    };

                    messages.push(open_ai::RequestMessage::Tool {
                        content: content.into(),
                        tool_call_id: tool_result.tool_use_id.to_string(),
                    });
                }
            }
        }
    }

    open_ai::Request {
        model: model_id.into(),
        messages,
        stream,
        stop: request.stop,
        temperature: request.temperature.unwrap_or(1.0),
        max_completion_tokens: max_output_tokens,
        parallel_tool_calls: if supports_parallel_tool_calls && !request.tools.is_empty() {
            // Disable parallel tool calls, as the Agent currently expects a maximum of one per turn.
            Some(false)
        } else {
            None
        },
        prompt_cache_key: if supports_prompt_cache_key {
            request.thread_id
        } else {
            None
        },
        tools: request
            .tools
            .into_iter()
            .map(|tool| open_ai::ToolDefinition::Function {
                function: open_ai::FunctionDefinition {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect(),
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => open_ai::ToolChoice::Auto,
            LanguageModelToolChoice::Any => open_ai::ToolChoice::Required,
            LanguageModelToolChoice::None => open_ai::ToolChoice::None,
        }),
        reasoning_effort,
    }
}

pub fn into_open_ai_response(
    request: LanguageModelRequest,
    model_id: &str,
    supports_parallel_tool_calls: bool,
    supports_prompt_cache_key: bool,
    max_output_tokens: Option<u64>,
    reasoning_effort: Option<ReasoningEffort>,
) -> (ResponseRequest, Vec<String>) {
    let stream = !model_id.starts_with("o1-");

    let LanguageModelRequest {
        thread_id,
        prompt_id: _,
        intent: _,
        mode: _,
        messages,
        tools,
        tool_choice,
        stop,
        temperature,
        thinking_allowed: _,
    } = request;

    let mut input_items = Vec::new();
    for (index, message) in messages.into_iter().enumerate() {
        append_message_to_response_items(message, index, &mut input_items);
    }

    let converted_tools: Vec<_> = tools
        .into_iter()
        .map(|tool| open_ai::responses::ToolDefinition::Function {
            name: tool.name,
            description: Some(tool.description),
            parameters: Some(tool.input_schema),
            strict: None,
        })
        .collect();

    let parallel_tool_calls_value = if converted_tools.is_empty() {
        None
    } else {
        if !supports_parallel_tool_calls {
            log::debug!(
                "Model `{}` using Responses API does not support parallel tool calls; calls will be sequential",
                model_id
            );
        }
        Some(false)
    };

    let response_request = ResponseRequest {
        model: model_id.into(),
        input: input_items,
        stream,
        stop_sequences: stop.clone(),
        temperature,
        top_p: None,
        max_output_tokens,
        parallel_tool_calls: parallel_tool_calls_value,
        tool_choice: tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => open_ai::ToolChoice::Auto,
            LanguageModelToolChoice::Any => open_ai::ToolChoice::Required,
            LanguageModelToolChoice::None => open_ai::ToolChoice::None,
        }),
        tools: converted_tools,
        prompt_cache_key: if supports_prompt_cache_key {
            thread_id
        } else {
            None
        },
        reasoning: reasoning_effort.map(|effort| open_ai::responses::ReasoningConfig { effort }),
    };

    (response_request, stop)
}

fn append_message_to_response_items(
    message: LanguageModelRequestMessage,
    index: usize,
    input_items: &mut Vec<Value>,
) {
    let mut content_parts: Vec<Value> = Vec::new();

    for content in message.content {
        match content {
            MessageContent::Text(text) => {
                push_response_text_part(&message.role, text, &mut content_parts);
            }
            MessageContent::Thinking { text, .. } => {
                push_response_text_part(&message.role, text, &mut content_parts);
            }
            MessageContent::RedactedThinking(_) => {
                push_response_text_part(&message.role, "<redacted>", &mut content_parts);
            }
            MessageContent::Image(image) => {
                push_response_image_part(&message.role, image, &mut content_parts);
            }
            MessageContent::ToolUse(tool_use) => {
                flush_response_parts(&message.role, index, &mut content_parts, input_items);
                let call_id = tool_use.id.to_string();
                input_items.push(json!({
                    "type": "function_call",
                    "id": call_id,
                    "call_id": call_id,
                    "name": tool_use.name,
                    "arguments": tool_use.raw_input,
                }));
            }
            MessageContent::ToolResult(tool_result) => {
                flush_response_parts(&message.role, index, &mut content_parts, input_items);
                input_items.push(json!({
                    "type": "function_call_output",
                    "call_id": tool_result.tool_use_id.to_string(),
                    "output": tool_result_output(&tool_result),
                }));
            }
        }
    }

    flush_response_parts(&message.role, index, &mut content_parts, input_items);
}

fn push_response_text_part(role: &Role, text: impl Into<String>, parts: &mut Vec<Value>) {
    let text = text.into();
    if text.trim().is_empty() {
        return;
    }

    match role {
        Role::Assistant => parts.push(json!({
            "type": "output_text",
            "text": text,
            "annotations": [],
        })),
        _ => parts.push(json!({
            "type": "input_text",
            "text": text,
        })),
    }
}

fn push_response_image_part(role: &Role, image: LanguageModelImage, parts: &mut Vec<Value>) {
    match role {
        Role::Assistant => parts.push(json!({
            "type": "output_text",
            "text": "[image omitted]",
            "annotations": [],
        })),
        _ => parts.push(json!({
            "type": "input_image",
            "image_url": image.to_base64_url(),
        })),
    }
}

fn flush_response_parts(
    role: &Role,
    _index: usize,
    parts: &mut Vec<Value>,
    input_items: &mut Vec<Value>,
) {
    if parts.is_empty() {
        return;
    }

    let item = match role {
        Role::Assistant => json!({
            "type": "message",
            "role": "assistant",
            "status": "completed",
            "content": parts.clone(),
        }),
        Role::User => json!({
            "type": "message",
            "role": "user",
            "content": parts.clone(),
        }),
        Role::System => json!({
            "type": "message",
            "role": "system",
            "content": parts.clone(),
        }),
    };

    input_items.push(item);
    parts.clear();
}

fn tool_result_output(result: &LanguageModelToolResult) -> String {
    if let Some(output) = &result.output {
        match output {
            serde_json::Value::String(text) => text.clone(),
            serde_json::Value::Null => String::new(),
            _ => output.to_string(),
        }
    } else {
        match &result.content {
            LanguageModelToolResultContent::Text(text) => text.to_string(),
            LanguageModelToolResultContent::Image(image) => image.to_base64_url(),
        }
    }
}

fn add_message_content_part(
    new_part: open_ai::MessagePart,
    role: Role,
    messages: &mut Vec<open_ai::RequestMessage>,
) {
    match (role, messages.last_mut()) {
        (Role::User, Some(open_ai::RequestMessage::User { content }))
        | (
            Role::Assistant,
            Some(open_ai::RequestMessage::Assistant {
                content: Some(content),
                ..
            }),
        )
        | (Role::System, Some(open_ai::RequestMessage::System { content, .. })) => {
            content.push_part(new_part);
        }
        _ => {
            messages.push(match role {
                Role::User => open_ai::RequestMessage::User {
                    content: open_ai::MessageContent::from(vec![new_part]),
                },
                Role::Assistant => open_ai::RequestMessage::Assistant {
                    content: Some(open_ai::MessageContent::from(vec![new_part])),
                    tool_calls: Vec::new(),
                },
                Role::System => open_ai::RequestMessage::System {
                    content: open_ai::MessageContent::from(vec![new_part]),
                },
            });
        }
    }
}

pub struct OpenAiEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl OpenAiEventMapper {
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

        let Some(choice) = event.choices.first() else {
            return events;
        };

        if let Some(content) = choice.delta.content.clone() {
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
                            id: tool_call.id.into(),
                            tool_name: tool_call.name.into(),
                            raw_input: tool_call.arguments.clone().into(),
                            json_parse_error: error.to_string(),
                        }),
                    }
                }));

                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
            }
            Some(stop_reason) => {
                log::error!("Unexpected OpenAI stop_reason: {stop_reason:?}",);
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

pub struct ResponseEventMapper {
    function_calls_by_item: HashMap<String, PendingResponseFunctionCall>,
    pending_stop_reason: Option<StopReason>,
    stop_sequences: Vec<String>,
    max_stop_sequence_len: usize,
    text_buffer: String,
    stop_triggered: bool,
}

#[derive(Default)]
struct PendingResponseFunctionCall {
    call_id: String,
    name: Arc<str>,
    arguments: String,
}

impl ResponseEventMapper {
    pub fn new(stop_sequences: Vec<String>) -> Self {
        let max_stop_sequence_len = stop_sequences.iter().map(|s| s.len()).max().unwrap_or(0);
        Self {
            function_calls_by_item: HashMap::default(),
            pending_stop_reason: None,
            stop_sequences,
            max_stop_sequence_len,
            text_buffer: String::new(),
            stop_triggered: false,
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponsesStreamEvent>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(LanguageModelCompletionError::from(anyhow!(error)))],
            })
        })
    }

    fn map_event(
        &mut self,
        event: ResponsesStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        match event {
            ResponsesStreamEvent::OutputItemAdded { item, .. } => {
                let mut events = self.flush_text_buffer();
                self.stop_triggered = false;

                match item.item_type.as_str() {
                    "message" => {
                        if let Some(id) = item.id {
                            events.push(Ok(LanguageModelCompletionEvent::StartMessage {
                                message_id: id,
                            }));
                        }
                    }
                    "function_call" => {
                        if let Some(item_id) = item.id.clone() {
                            let call_id = item
                                .call_id
                                .clone()
                                .or_else(|| item.id.clone())
                                .unwrap_or_else(|| item_id.clone());
                            let entry = PendingResponseFunctionCall {
                                call_id,
                                name: Arc::<str>::from(item.name.unwrap_or_default()),
                                arguments: item.arguments.unwrap_or_default(),
                            };
                            self.function_calls_by_item.insert(item_id, entry);
                        }
                    }
                    _ => {}
                }
                events
            }
            ResponsesStreamEvent::OutputTextDelta { delta, .. } => self.handle_text_delta(delta),
            ResponsesStreamEvent::FunctionCallArgumentsDelta { item_id, delta, .. } => {
                if let Some(entry) = self.function_calls_by_item.get_mut(&item_id) {
                    entry.arguments.push_str(&delta);
                }
                Vec::new()
            }
            ResponsesStreamEvent::FunctionCallArgumentsDone {
                item_id, arguments, ..
            } => {
                if let Some(mut entry) = self.function_calls_by_item.remove(&item_id) {
                    if !arguments.is_empty() {
                        entry.arguments = arguments;
                    }
                    let raw_input = entry.arguments.clone();
                    self.pending_stop_reason = Some(StopReason::ToolUse);
                    match serde_json::from_str::<serde_json::Value>(&entry.arguments) {
                        Ok(input) => {
                            vec![Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id: LanguageModelToolUseId::from(entry.call_id.clone()),
                                    name: entry.name.clone(),
                                    is_input_complete: true,
                                    input,
                                    raw_input,
                                },
                            ))]
                        }
                        Err(error) => {
                            vec![Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                id: LanguageModelToolUseId::from(entry.call_id.clone()),
                                tool_name: entry.name.clone(),
                                raw_input: Arc::<str>::from(raw_input),
                                json_parse_error: error.to_string(),
                            })]
                        }
                    }
                } else {
                    Vec::new()
                }
            }
            ResponsesStreamEvent::Completed { response } => {
                self.handle_completion(response, StopReason::EndTurn)
            }
            ResponsesStreamEvent::Incomplete { response } => {
                let reason = response
                    .status_details
                    .as_ref()
                    .and_then(|details| details.reason.as_deref());
                let stop_reason = match reason {
                    Some("max_output_tokens") => StopReason::MaxTokens,
                    Some("content_filter") => {
                        self.pending_stop_reason = Some(StopReason::Refusal);
                        StopReason::Refusal
                    }
                    _ => self
                        .pending_stop_reason
                        .take()
                        .unwrap_or(StopReason::EndTurn),
                };

                let mut events = Vec::new();
                if self.pending_stop_reason.is_none() {
                    events.extend(self.emit_tool_calls_from_output(&response.output));
                }
                if let Some(usage) = response.usage.as_ref() {
                    events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(
                        token_usage_from_response_usage(usage),
                    )));
                }
                events.push(Ok(LanguageModelCompletionEvent::Stop(stop_reason)));
                events
            }
            ResponsesStreamEvent::Failed { response } => {
                let message = response
                    .status_details
                    .and_then(|details| details.error)
                    .map(|error| error.to_string())
                    .unwrap_or_else(|| "response failed".to_string());
                vec![Err(LanguageModelCompletionError::Other(anyhow!(message)))]
            }
            ResponsesStreamEvent::Error { error }
            | ResponsesStreamEvent::GenericError { error } => {
                vec![Err(LanguageModelCompletionError::Other(anyhow!(format!(
                    "{error:?}"
                ))))]
            }
            ResponsesStreamEvent::OutputTextDone { .. } => self.flush_text_buffer(),
            ResponsesStreamEvent::OutputItemDone { .. }
            | ResponsesStreamEvent::ContentPartAdded { .. }
            | ResponsesStreamEvent::ContentPartDone { .. }
            | ResponsesStreamEvent::Created { .. }
            | ResponsesStreamEvent::InProgress { .. }
            | ResponsesStreamEvent::Unknown => Vec::new(),
        }
    }

    fn handle_text_delta(
        &mut self,
        delta: String,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        if delta.is_empty() || self.stop_triggered {
            return Vec::new();
        }

        if self.stop_sequences.is_empty() {
            return vec![Ok(LanguageModelCompletionEvent::Text(delta))];
        }

        self.text_buffer.push_str(&delta);

        let mut events = Vec::new();
        loop {
            if self.stop_triggered {
                self.text_buffer.clear();
                break;
            }
            let mut earliest_stop: Option<(usize, usize)> = None;
            for sequence in &self.stop_sequences {
                if sequence.is_empty() {
                    continue;
                }
                if let Some(idx) = self.text_buffer.find(sequence) {
                    if earliest_stop
                        .map(|(existing_idx, existing_len)| {
                            idx < existing_idx
                                || (idx == existing_idx && sequence.len() < existing_len)
                        })
                        .unwrap_or(true)
                    {
                        earliest_stop = Some((idx, sequence.len()));
                    }
                }
            }

            if let Some((stop_idx, stop_len)) = earliest_stop {
                if stop_idx > 0 {
                    let before_stop = self.text_buffer[..stop_idx].to_string();
                    events.push(Ok(LanguageModelCompletionEvent::Text(before_stop)));
                }
                // Remove up to and including the stop sequence
                self.text_buffer.drain(..stop_idx + stop_len);
                if self.pending_stop_reason.is_none() {
                    self.pending_stop_reason = Some(StopReason::EndTurn);
                }
                self.stop_triggered = true;
                self.text_buffer.clear();
                break;
            } else {
                let retain_len = self.max_stop_sequence_len.saturating_sub(1);
                if self.text_buffer.len() <= retain_len {
                    break;
                }
                let emit_len = self.text_buffer.len() - retain_len;
                let emit_text = self.text_buffer[..emit_len].to_string();
                events.push(Ok(LanguageModelCompletionEvent::Text(emit_text)));
                self.text_buffer.drain(..emit_len);
                break;
            }
        }

        events
    }

    fn flush_text_buffer(
        &mut self,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        if self.stop_triggered || self.text_buffer.is_empty() {
            self.text_buffer.clear();
            return Vec::new();
        }

        let text = std::mem::take(&mut self.text_buffer);
        if text.is_empty() {
            Vec::new()
        } else {
            vec![Ok(LanguageModelCompletionEvent::Text(text))]
        }
    }

    fn handle_completion(
        &mut self,
        response: ResponsesSummary,
        default_reason: StopReason,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();

        events.extend(self.flush_text_buffer());

        if self.pending_stop_reason.is_none() {
            events.extend(self.emit_tool_calls_from_output(&response.output));
        }

        if let Some(usage) = response.usage.as_ref() {
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(
                token_usage_from_response_usage(usage),
            )));
        }

        let stop_reason = self.pending_stop_reason.take().unwrap_or(default_reason);
        events.push(Ok(LanguageModelCompletionEvent::Stop(stop_reason)));
        events
    }

    fn emit_tool_calls_from_output(
        &mut self,
        output: &[ResponsesItem],
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();
        for item in output {
            if item.item_type == "function_call" {
                let Some(call_id) = item.call_id.clone().or_else(|| item.id.clone()) else {
                    log::error!("Function call item missing both call_id and id: {:?}", item);
                    continue;
                };
                let name: Arc<str> = Arc::from(item.name.clone().unwrap_or_default());
                if let Some(arguments) = item.arguments.clone() {
                    self.pending_stop_reason = Some(StopReason::ToolUse);
                    match serde_json::from_str::<serde_json::Value>(&arguments) {
                        Ok(input) => {
                            events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id: LanguageModelToolUseId::from(call_id.clone()),
                                    name: name.clone(),
                                    is_input_complete: true,
                                    input,
                                    raw_input: arguments.clone(),
                                },
                            )));
                        }
                        Err(error) => {
                            events.push(Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                id: LanguageModelToolUseId::from(call_id.clone()),
                                tool_name: name.clone(),
                                raw_input: Arc::<str>::from(arguments.clone()),
                                json_parse_error: error.to_string(),
                            }));
                        }
                    }
                }
            }
        }
        events
    }
}

fn token_usage_from_response_usage(usage: &ResponsesUsage) -> TokenUsage {
    TokenUsage {
        input_tokens: usage.input_tokens.unwrap_or_default(),
        output_tokens: usage.output_tokens.unwrap_or_default(),
        cache_creation_input_tokens: 0,
        cache_read_input_tokens: 0,
    }
}

pub(crate) fn collect_tiktoken_messages(
    request: LanguageModelRequest,
) -> Vec<tiktoken_rs::ChatCompletionRequestMessage> {
    request
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
        .collect::<Vec<_>>()
}

pub fn count_open_ai_tokens(
    request: LanguageModelRequest,
    model: Model,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
    cx.background_spawn(async move {
        let messages = collect_tiktoken_messages(request);

        match model {
            Model::Custom { max_tokens, .. } => {
                let model = if max_tokens >= 100_000 {
                    // If the max tokens is 100k or more, it is likely the o200k_base tokenizer from gpt4o
                    "gpt-4o"
                } else {
                    // Otherwise fallback to gpt-4, since only cl100k_base and o200k_base are
                    // supported with this tiktoken method
                    "gpt-4"
                };
                tiktoken_rs::num_tokens_from_messages(model, &messages)
            }
            // Currently supported by tiktoken_rs
            // Sometimes tiktoken-rs is behind on model support. If that is the case, make a new branch
            // arm with an override. We enumerate all supported models here so that we can check if new
            // models are supported yet or not.
            Model::ThreePointFiveTurbo
            | Model::Four
            | Model::FourTurbo
            | Model::FourOmni
            | Model::FourOmniMini
            | Model::FourPointOne
            | Model::FourPointOneMini
            | Model::FourPointOneNano
            | Model::O1
            | Model::O3
            | Model::O3Mini
            | Model::O4Mini => tiktoken_rs::num_tokens_from_messages(model.id(), &messages),
            // GPT-5 models don't have tiktoken support yet; fall back on gpt-4o tokenizer
            Model::Five | Model::FiveCodex | Model::FiveMini | Model::FiveNano => {
                tiktoken_rs::num_tokens_from_messages("gpt-4o", &messages)
            }
        }
        .map(|tokens| tokens as u64)
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
                "sk-000000000000000000000000000000000000000000000000",
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
                    // We don't log an error, because "not signed in" is also an error.
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
            .update(cx, |input, cx| input.set_text("", window, cx));

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

        let api_key_section = if self.should_render_editor(cx) {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Zed's agent with OpenAI, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(InstructionListItem::new(
                            "Create one by visiting",
                            Some("OpenAI's console"),
                            Some("https://platform.openai.com/api-keys"),
                        ))
                        .child(InstructionListItem::text_only(
                            "Ensure your OpenAI account has credits",
                        ))
                        .child(InstructionListItem::text_only(
                            "Paste your API key below and hit enter to start using the assistant",
                        )),
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(format!(
                        "You can also assign the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .child(
                    Label::new(
                        "Note that having a subscription for another service like GitHub Copilot won't work.",
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
                            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable")
                        } else {
                            let api_url = OpenAiLanguageModelProvider::api_url(cx);
                            if api_url == OPEN_AI_API_URL {
                                "API key configured".to_string()
                            } else {
                                format!("API key configured for {}", truncate_and_trailoff(&api_url, 32))
                            }
                        })),
                )
                .child(
                    Button::new("reset-api-key", "Reset API Key")
                        .label_size(LabelSize::Small)
                        .icon(IconName::Undo)
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .layer(ElevationIndex::ModalSurface)
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                )
                .into_any()
        };

        let compatible_api_section = h_flex()
            .mt_1p5()
            .gap_0p5()
            .flex_wrap()
            .when(self.should_render_editor(cx), |this| {
                this.pt_1p5()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
            })
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Info)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(Label::new("Zed also supports OpenAI-compatible models.")),
            )
            .child(
                Button::new("docs", "Learn More")
                    .icon(IconName::ArrowUpRight)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .on_click(move |_, _window, cx| {
                        cx.open_url("https://zed.dev/docs/ai/llm-providers#openai-api-compatible")
                    }),
            );

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials…")).into_any()
        } else {
            v_flex()
                .size_full()
                .child(api_key_section)
                .child(compatible_api_section)
                .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::{size, DevicePixels, TestAppContext};
    use language_model::{LanguageModelRequestMessage, LanguageModelRequestTool};

    use super::*;
    use futures::{StreamExt, executor::block_on};
    use open_ai::responses::{
        ResponseItem, ResponseStatusDetails, ResponseSummary, ResponseUsage,
        StreamEvent as ResponsesStreamEvent,
    };

    fn map_response_events(events: Vec<ResponsesStreamEvent>) -> Vec<LanguageModelCompletionEvent> {
        block_on(async {
            ResponseEventMapper::new(Vec::new())
                .map_stream(Box::pin(futures::stream::iter(events.into_iter().map(Ok))))
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .map(Result::unwrap)
                .collect()
        })
    }

    fn map_response_events_with_stop(
        events: Vec<ResponsesStreamEvent>,
        stop_sequences: Vec<String>,
    ) -> Vec<LanguageModelCompletionEvent> {
        block_on(async {
            ResponseEventMapper::new(stop_sequences)
                .map_stream(Box::pin(futures::stream::iter(events.into_iter().map(Ok))))
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .map(Result::unwrap)
                .collect()
        })
    }

    fn response_item_message(id: &str) -> ResponseItem {
        ResponseItem {
            id: Some(id.to_string()),
            item_type: "message".to_string(),
            role: Some("assistant".to_string()),
            status: Some("in_progress".to_string()),
            name: None,
            call_id: None,
            arguments: None,
            output: None,
            content: Some(vec![]),
        }
    }

    fn response_item_function_call(id: &str, args: Option<&str>) -> ResponseItem {
        ResponseItem {
            id: Some(id.to_string()),
            item_type: "function_call".to_string(),
            role: None,
            status: Some("in_progress".to_string()),
            name: Some("get_weather".to_string()),
            call_id: Some("call_123".to_string()),
            arguments: args.map(|s| s.to_string()),
            output: None,
            content: None,
        }
    }

    #[gpui::test]
    fn tiktoken_rs_support(cx: &TestAppContext) {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            mode: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("message".into())],
                cache: false,
            }],
            tools: vec![],
            tool_choice: None,
            stop: vec![],
            temperature: None,
            thinking_allowed: true,
        };

        // Validate that all models are supported by tiktoken-rs
        for model in Model::iter() {
            let count = cx
                .executor()
                .block(count_open_ai_tokens(
                    request.clone(),
                    model,
                    &cx.app.borrow(),
                ))
                .unwrap();
            assert!(count > 0);
        }
    }

    #[test]
    fn responses_stream_maps_text_and_usage() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_message("msg_123"),
            },
            ResponsesStreamEvent::OutputTextDelta {
                item_id: "msg_123".into(),
                output_index: 0,
                content_index: Some(0),
                delta: "Hello".into(),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary {
                    usage: Some(ResponseUsage {
                        input_tokens: Some(5),
                        output_tokens: Some(3),
                        total_tokens: Some(8),
                    }),
                    ..Default::default()
                },
            },
        ];

        let mapped = map_response_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::StartMessage { ref message_id } if message_id == "msg_123"
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Text(ref text) if text == "Hello"
        ));
        assert!(matches!(
            mapped[2],
            LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: 5,
                output_tokens: 3,
                ..
            })
        ));
        assert!(matches!(
            mapped[3],
            LanguageModelCompletionEvent::Stop(StopReason::EndTurn)
        ));
    }

    #[test]
    fn responses_stream_respects_stop_sequences() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_message("msg_stop"),
            },
            ResponsesStreamEvent::OutputTextDelta {
                item_id: "msg_stop".into(),
                output_index: 0,
                content_index: Some(0),
                delta: "Hello wor".into(),
            },
            ResponsesStreamEvent::OutputTextDelta {
                item_id: "msg_stop".into(),
                output_index: 0,
                content_index: Some(0),
                delta: "ld<stop>Ignored text".into(),
            },
            ResponsesStreamEvent::OutputTextDone {
                item_id: "msg_stop".into(),
                output_index: 0,
                content_index: Some(0),
                text: "Hello world".into(),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events_with_stop(events, vec!["<stop>".into()]);

        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::StartMessage { .. }
        ));

        let collected_text: String = mapped
            .iter()
            .filter_map(|event| match event {
                LanguageModelCompletionEvent::Text(text) => Some(text.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(collected_text, "Hello world");
        assert!(matches!(
            mapped.last(),
            Some(LanguageModelCompletionEvent::Stop(StopReason::EndTurn))
        ));
    }

    #[test]
    fn responses_stream_allows_new_message_after_stop_sequence() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_message("msg_stop"),
            },
            ResponsesStreamEvent::OutputTextDelta {
                item_id: "msg_stop".into(),
                output_index: 0,
                content_index: Some(0),
                delta: "First chunk<STOP>".into(),
            },
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 1,
                sequence_number: None,
                item: response_item_message("msg_next"),
            },
            ResponsesStreamEvent::OutputTextDelta {
                item_id: "msg_next".into(),
                output_index: 1,
                content_index: Some(0),
                delta: "Second chunk".into(),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events_with_stop(events, vec!["<STOP>".into()]);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::StartMessage { ref message_id }
            if message_id == "msg_stop"
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Text(ref text) if text == "First chunk"
        ));
        assert!(matches!(
            mapped[2],
            LanguageModelCompletionEvent::StartMessage { ref message_id }
            if message_id == "msg_next"
        ));
        assert!(matches!(
            mapped[3],
            LanguageModelCompletionEvent::Text(ref text) if text == "Second chunk"
        ));
        assert!(matches!(
            mapped.last(),
            Some(LanguageModelCompletionEvent::Stop(StopReason::EndTurn))
        ));
    }

    #[test]
    fn into_open_ai_response_disables_parallel_tool_calls_when_tools_present() {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            mode: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("hi".into())],
                cache: false,
            }],
            tools: vec![LanguageModelRequestTool {
                name: "get_weather".into(),
                description: "Retrieve weather".into(),
                input_schema: json!({}),
            }],
            tool_choice: Some(LanguageModelToolChoice::Auto),
            stop: vec!["<stop>".into()],
            temperature: Some(0.5),
            thinking_allowed: false,
        };

        let (response, stops) =
            into_open_ai_response(request, "custom-model", false, false, None, None);

        assert_eq!(response.parallel_tool_calls, Some(false));
        assert_eq!(response.stop_sequences, vec!["<stop>".to_string()]);
        assert_eq!(stops, vec!["<stop>".to_string()]);
    }

    #[test]
    fn into_open_ai_response_omits_parallel_tool_calls_when_no_tools() {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            mode: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("hi".into())],
                cache: false,
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: false,
        };

        let (response, stops) =
            into_open_ai_response(request, "custom-model", true, false, None, None);

        assert_eq!(response.parallel_tool_calls, None);
        assert!(response.stop_sequences.is_empty());
        assert!(stops.is_empty());
    }

    #[test]
    fn into_open_ai_response_builds_complete_payload() {
        let tool_call_id = LanguageModelToolUseId::from("call-42");
        let tool_input = json!({ "city": "Boston" });
        let tool_arguments = serde_json::to_string(&tool_input).unwrap();
        let tool_use = LanguageModelToolUse {
            id: tool_call_id.clone(),
            name: Arc::from("get_weather"),
            raw_input: tool_arguments.clone(),
            input: tool_input.clone(),
            is_input_complete: true,
        };
        let tool_result = LanguageModelToolResult {
            tool_use_id: tool_call_id.clone(),
            tool_name: Arc::from("get_weather"),
            is_error: false,
            content: LanguageModelToolResultContent::Text(Arc::from("Sunny")),
            output: Some(json!({ "forecast": "Sunny" })),
        };
        let user_image = LanguageModelImage {
            source: SharedString::from("aGVsbG8="),
            size: size(DevicePixels(1), DevicePixels(1)),
        };
        let expected_image_url = user_image.to_base64_url();

        let request = LanguageModelRequest {
            thread_id: Some("thread-123".into()),
            prompt_id: None,
            intent: None,
            mode: None,
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("System context".into())],
                    cache: false,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![
                        MessageContent::Text("Please check the weather.".into()),
                        MessageContent::Image(user_image.clone()),
                    ],
                    cache: false,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![
                        MessageContent::Text("Looking that up.".into()),
                        MessageContent::ToolUse(tool_use.clone()),
                    ],
                    cache: false,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolResult(tool_result.clone())],
                    cache: false,
                },
            ],
            tools: vec![LanguageModelRequestTool {
                name: "get_weather".into(),
                description: "Fetches the weather".into(),
                input_schema: json!({ "type": "object" }),
            }],
            tool_choice: Some(LanguageModelToolChoice::Any),
            stop: vec!["<STOP>".into()],
            temperature: Some(0.2),
            thinking_allowed: false,
        };

        let (response, stops) = into_open_ai_response(
            request,
            "custom-model",
            true,
            true,
            Some(2048),
            Some(ReasoningEffort::Low),
        );
        assert_eq!(stops, vec!["<STOP>".to_string()]);

        let serialized = serde_json::to_value(&response).unwrap();
        let expected = json!({
            "model": "custom-model",
            "input": [
                {
                    "type": "message",
                    "role": "system",
                    "content": [
                        { "type": "input_text", "text": "System context" }
                    ]
                },
                {
                    "type": "message",
                    "role": "user",
                    "content": [
                        { "type": "input_text", "text": "Please check the weather." },
                        { "type": "input_image", "image_url": expected_image_url }
                    ]
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "status": "completed",
                    "content": [
                        { "type": "output_text", "text": "Looking that up.", "annotations": [] }
                    ]
                },
                {
                    "type": "function_call",
                    "id": "call-42",
                    "call_id": "call-42",
                    "name": "get_weather",
                    "arguments": tool_arguments
                },
                {
                    "type": "function_call_output",
                    "call_id": "call-42",
                    "output": "{\"forecast\":\"Sunny\"}"
                }
            ],
            "stream": true,
            "stop_sequences": ["<STOP>"],
            "temperature": 0.2,
            "max_output_tokens": 2048,
            "parallel_tool_calls": false,
            "tool_choice": "required",
            "tools": [
                {
                    "type": "function",
                    "name": "get_weather",
                    "description": "Fetches the weather",
                    "parameters": { "type": "object" }
                }
            ],
            "prompt_cache_key": "thread-123",
            "reasoning": { "effort": "low" }
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn responses_stream_maps_tool_calls() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_function_call("item_fn", Some("{\"city\":\"Bos")),
            },
            ResponsesStreamEvent::FunctionCallArgumentsDelta {
                item_id: "item_fn".into(),
                output_index: 0,
                delta: "ton\"}".into(),
                sequence_number: None,
            },
            ResponsesStreamEvent::FunctionCallArgumentsDone {
                item_id: "item_fn".into(),
                output_index: 0,
                arguments: "{\"city\":\"Boston\"}".into(),
                sequence_number: None,
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                ref id,
                ref name,
                ref raw_input,
                ..
            }) if id.to_string() == "call_123"
                && name.as_ref() == "get_weather"
                && raw_input == "{\"city\":\"Boston\"}"
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Stop(StopReason::ToolUse)
        ));
    }

    #[test]
    fn responses_stream_uses_max_tokens_stop_reason() {
        let events = vec![ResponsesStreamEvent::Incomplete {
            response: ResponseSummary {
                status_details: Some(ResponseStatusDetails {
                    reason: Some("max_output_tokens".into()),
                    r#type: Some("incomplete".into()),
                    error: None,
                }),
                usage: Some(ResponseUsage {
                    input_tokens: Some(10),
                    output_tokens: Some(20),
                    total_tokens: Some(30),
                }),
                ..Default::default()
            },
        }];

        let mapped = map_response_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: 10,
                output_tokens: 20,
                ..
            })
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Stop(StopReason::MaxTokens)
        ));
    }

    #[test]
    fn responses_stream_handles_multiple_tool_calls() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_function_call("item_fn1", Some("{\"city\":\"NYC\"}")),
            },
            ResponsesStreamEvent::FunctionCallArgumentsDone {
                item_id: "item_fn1".into(),
                output_index: 0,
                arguments: "{\"city\":\"NYC\"}".into(),
                sequence_number: None,
            },
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 1,
                sequence_number: None,
                item: response_item_function_call("item_fn2", Some("{\"city\":\"LA\"}")),
            },
            ResponsesStreamEvent::FunctionCallArgumentsDone {
                item_id: "item_fn2".into(),
                output_index: 1,
                arguments: "{\"city\":\"LA\"}".into(),
                sequence_number: None,
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        assert_eq!(mapped.len(), 3);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse { ref raw_input, .. })
            if raw_input == "{\"city\":\"NYC\"}"
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse { ref raw_input, .. })
            if raw_input == "{\"city\":\"LA\"}"
        ));
        assert!(matches!(
            mapped[2],
            LanguageModelCompletionEvent::Stop(StopReason::ToolUse)
        ));
    }

    #[test]
    fn responses_stream_handles_mixed_text_and_tool_calls() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_message("msg_123"),
            },
            ResponsesStreamEvent::OutputTextDelta {
                item_id: "msg_123".into(),
                output_index: 0,
                content_index: Some(0),
                delta: "Let me check that".into(),
            },
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 1,
                sequence_number: None,
                item: response_item_function_call("item_fn", Some("{\"query\":\"test\"}")),
            },
            ResponsesStreamEvent::FunctionCallArgumentsDone {
                item_id: "item_fn".into(),
                output_index: 1,
                arguments: "{\"query\":\"test\"}".into(),
                sequence_number: None,
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::StartMessage { .. }
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Text(ref text) if text == "Let me check that"
        ));
        assert!(matches!(
            mapped[2],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse { ref raw_input, .. })
            if raw_input == "{\"query\":\"test\"}"
        ));
        assert!(matches!(
            mapped[3],
            LanguageModelCompletionEvent::Stop(StopReason::ToolUse)
        ));
    }

    #[test]
    fn responses_stream_handles_json_parse_error() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_function_call("item_fn", Some("{invalid json")),
            },
            ResponsesStreamEvent::FunctionCallArgumentsDone {
                item_id: "item_fn".into(),
                output_index: 0,
                arguments: "{invalid json".into(),
                sequence_number: None,
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::ToolUseJsonParseError {
                ref raw_input,
                ..
            } if raw_input.as_ref() == "{invalid json"
        ));
    }

    #[test]
    fn responses_stream_handles_incomplete_function_call() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_function_call("item_fn", Some("{\"city\":")),
            },
            ResponsesStreamEvent::FunctionCallArgumentsDelta {
                item_id: "item_fn".into(),
                output_index: 0,
                delta: "\"Boston\"".into(),
                sequence_number: None,
            },
            ResponsesStreamEvent::Incomplete {
                response: ResponseSummary {
                    status_details: Some(ResponseStatusDetails {
                        reason: Some("max_output_tokens".into()),
                        r#type: Some("incomplete".into()),
                        error: None,
                    }),
                    output: vec![response_item_function_call(
                        "item_fn",
                        Some("{\"city\":\"Boston\"}"),
                    )],
                    ..Default::default()
                },
            },
        ];

        let mapped = map_response_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse { ref raw_input, .. })
            if raw_input == "{\"city\":\"Boston\"}"
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Stop(StopReason::MaxTokens)
        ));
    }

    #[test]
    fn responses_stream_incomplete_does_not_duplicate_tool_calls() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_function_call("item_fn", Some("{\"city\":\"Boston\"}")),
            },
            ResponsesStreamEvent::FunctionCallArgumentsDone {
                item_id: "item_fn".into(),
                output_index: 0,
                arguments: "{\"city\":\"Boston\"}".into(),
                sequence_number: None,
            },
            ResponsesStreamEvent::Incomplete {
                response: ResponseSummary {
                    status_details: Some(ResponseStatusDetails {
                        reason: Some("max_output_tokens".into()),
                        r#type: Some("incomplete".into()),
                        error: None,
                    }),
                    output: vec![response_item_function_call(
                        "item_fn",
                        Some("{\"city\":\"Boston\"}"),
                    )],
                    ..Default::default()
                },
            },
        ];

        let mapped = map_response_events(events);
        assert_eq!(mapped.len(), 2);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse { ref raw_input, .. })
            if raw_input == "{\"city\":\"Boston\"}"
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Stop(StopReason::MaxTokens)
        ));
    }
}
