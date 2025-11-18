use std::pin::Pin;
use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use cloud_llm_client::CompletionIntent;
use collections::HashMap;
use copilot::copilot_chat::{
    ChatMessage, ChatMessageContent, ChatMessagePart, CopilotChat, ImageUrl,
    Model as CopilotChatModel, ModelVendor, Request as CopilotChatRequest, ResponseEvent, Tool,
    ToolCall,
};
use copilot::{Copilot, Status};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, Stream, StreamExt};
use gpui::{Action, AnyView, App, AsyncApp, Entity, Render, Subscription, Task, svg};
use http_client::StatusCode;
use language::language_settings::all_language_settings;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelToolChoice, LanguageModelToolResultContent,
    LanguageModelToolSchemaFormat, LanguageModelToolUse, MessageContent, RateLimiter, Role,
    StopReason, TokenUsage,
};
use settings::SettingsStore;
use ui::{CommonAnimationExt, prelude::*};
use util::debug_panic;

use crate::ui::ConfiguredApiCard;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("copilot_chat");
const PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("GitHub Copilot Chat");

pub struct CopilotChatLanguageModelProvider {
    state: Entity<State>,
}

pub struct State {
    _copilot_chat_subscription: Option<Subscription>,
    _settings_subscription: Subscription,
}

impl State {
    fn is_authenticated(&self, cx: &App) -> bool {
        CopilotChat::global(cx)
            .map(|m| m.read(cx).is_authenticated())
            .unwrap_or(false)
    }
}

impl CopilotChatLanguageModelProvider {
    pub fn new(cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            let copilot_chat_subscription = CopilotChat::global(cx)
                .map(|copilot_chat| cx.observe(&copilot_chat, |_, _, cx| cx.notify()));
            State {
                _copilot_chat_subscription: copilot_chat_subscription,
                _settings_subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                    if let Some(copilot_chat) = CopilotChat::global(cx) {
                        let language_settings = all_language_settings(None, cx);
                        let configuration = copilot::copilot_chat::CopilotChatConfiguration {
                            enterprise_uri: language_settings
                                .edit_predictions
                                .copilot
                                .enterprise_uri
                                .clone(),
                        };
                        copilot_chat.update(cx, |chat, cx| {
                            chat.set_configuration(configuration, cx);
                        });
                    }
                    cx.notify();
                }),
            }
        });

        Self { state }
    }

    fn create_language_model(&self, model: CopilotChatModel) -> Arc<dyn LanguageModel> {
        Arc::new(CopilotChatLanguageModel {
            model,
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for CopilotChatLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for CopilotChatLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconName {
        IconName::Copilot
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let models = CopilotChat::global(cx).and_then(|m| m.read(cx).models())?;
        models
            .first()
            .map(|model| self.create_language_model(model.clone()))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        // The default model should be Copilot Chat's 'base model', which is likely a relatively fast
        // model (e.g. 4o) and a sensible choice when considering premium requests
        self.default_model(cx)
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let Some(models) = CopilotChat::global(cx).and_then(|m| m.read(cx).models()) else {
            return Vec::new();
        };
        models
            .iter()
            .map(|model| self.create_language_model(model.clone()))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated(cx)
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated(cx) {
            return Task::ready(Ok(()));
        };

        let Some(copilot) = Copilot::global(cx) else {
            return Task::ready(Err(anyhow!(concat!(
                "Copilot must be enabled for Copilot Chat to work. ",
                "Please enable Copilot and try again."
            ))
            .into()));
        };

        let err = match copilot.read(cx).status() {
            Status::Authorized => return Task::ready(Ok(())),
            Status::Disabled => anyhow!(
                "Copilot must be enabled for Copilot Chat to work. Please enable Copilot and try again."
            ),
            Status::Error(err) => anyhow!(format!(
                "Received the following error while signing into Copilot: {err}"
            )),
            Status::Starting { task: _ } => anyhow!(
                "Copilot is still starting, please wait for Copilot to start then try again"
            ),
            Status::Unauthorized => anyhow!(
                "Unable to authorize with Copilot. Please make sure that you have an active Copilot and Copilot Chat subscription."
            ),
            Status::SignedOut { .. } => {
                anyhow!("You have signed out of Copilot. Please sign in to Copilot and try again.")
            }
            Status::SigningIn { prompt: _ } => anyhow!("Still signing into Copilot..."),
        };

        Task::ready(Err(err.into()))
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        _: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        let state = self.state.clone();
        cx.new(|cx| ConfigurationView::new(state, cx)).into()
    }

    fn reset_credentials(&self, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Err(anyhow!(
            "Signing out of GitHub Copilot Chat is currently not supported."
        )))
    }
}

fn collect_tiktoken_messages(
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

pub struct CopilotChatLanguageModel {
    model: CopilotChatModel,
    request_limiter: RateLimiter,
}

impl LanguageModel for CopilotChatLanguageModel {
    fn id(&self) -> LanguageModelId {
        LanguageModelId::from(self.model.id().to_string())
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
        self.model.supports_vision()
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        match self.model.vendor() {
            ModelVendor::OpenAI | ModelVendor::Anthropic => {
                LanguageModelToolSchemaFormat::JsonSchema
            }
            ModelVendor::Google | ModelVendor::XAI | ModelVendor::Unknown => {
                LanguageModelToolSchemaFormat::JsonSchemaSubset
            }
        }
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => self.supports_tools(),
        }
    }

    fn telemetry_id(&self) -> String {
        format!("copilot_chat/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        let model = self.model.clone();
        cx.background_spawn(async move {
            let messages = collect_tiktoken_messages(request);
            // Copilot uses OpenAI tiktoken tokenizer for all it's model irrespective of the underlying provider(vendor).
            let tokenizer_model = match model.tokenizer() {
                Some("o200k_base") => "gpt-4o",
                Some("cl100k_base") => "gpt-4",
                _ => "gpt-4o",
            };

            tiktoken_rs::num_tokens_from_messages(tokenizer_model, &messages)
                .map(|tokens| tokens as u64)
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
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let is_user_initiated = request.intent.is_none_or(|intent| match intent {
            CompletionIntent::UserPrompt
            | CompletionIntent::ThreadContextSummarization
            | CompletionIntent::InlineAssist
            | CompletionIntent::TerminalInlineAssist
            | CompletionIntent::GenerateGitCommitMessage => true,

            CompletionIntent::ToolResults
            | CompletionIntent::ThreadSummarization
            | CompletionIntent::CreateFile
            | CompletionIntent::EditFile => false,
        });

        if self.model.supports_response() {
            let responses_request = into_copilot_responses(&self.model, request);
            let request_limiter = self.request_limiter.clone();
            let future = cx.spawn(async move |cx| {
                let request =
                    CopilotChat::stream_response(responses_request, is_user_initiated, cx.clone());
                request_limiter
                    .stream(async move {
                        let stream = request.await?;
                        let mapper = CopilotResponsesEventMapper::new();
                        Ok(mapper.map_stream(stream).boxed())
                    })
                    .await
            });
            return async move { Ok(future.await?.boxed()) }.boxed();
        }

        let copilot_request = match into_copilot_chat(&self.model, request) {
            Ok(request) => request,
            Err(err) => return futures::future::ready(Err(err.into())).boxed(),
        };
        let is_streaming = copilot_request.stream;

        let request_limiter = self.request_limiter.clone();
        let future = cx.spawn(async move |cx| {
            let request =
                CopilotChat::stream_completion(copilot_request, is_user_initiated, cx.clone());
            request_limiter
                .stream(async move {
                    let response = request.await?;
                    Ok(map_to_language_model_completion_events(
                        response,
                        is_streaming,
                    ))
                })
                .await
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

pub fn map_to_language_model_completion_events(
    events: Pin<Box<dyn Send + Stream<Item = Result<ResponseEvent>>>>,
    is_streaming: bool,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
    #[derive(Default)]
    struct RawToolCall {
        id: String,
        name: String,
        arguments: String,
    }

    struct State {
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseEvent>>>>,
        tool_calls_by_index: HashMap<usize, RawToolCall>,
    }

    futures::stream::unfold(
        State {
            events,
            tool_calls_by_index: HashMap::default(),
        },
        move |mut state| async move {
            if let Some(event) = state.events.next().await {
                match event {
                    Ok(event) => {
                        let Some(choice) = event.choices.first() else {
                            return Some((
                                vec![Err(anyhow!("Response contained no choices").into())],
                                state,
                            ));
                        };

                        let delta = if is_streaming {
                            choice.delta.as_ref()
                        } else {
                            choice.message.as_ref()
                        };

                        let Some(delta) = delta else {
                            return Some((
                                vec![Err(anyhow!("Response contained no delta").into())],
                                state,
                            ));
                        };

                        let mut events = Vec::new();
                        if let Some(content) = delta.content.clone() {
                            events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                        }

                        for (index, tool_call) in delta.tool_calls.iter().enumerate() {
                            let tool_index = tool_call.index.unwrap_or(index);
                            let entry = state.tool_calls_by_index.entry(tool_index).or_default();

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

                        if let Some(usage) = event.usage {
                            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(
                                TokenUsage {
                                    input_tokens: usage.prompt_tokens,
                                    output_tokens: usage.completion_tokens,
                                    cache_creation_input_tokens: 0,
                                    cache_read_input_tokens: 0,
                                },
                            )));
                        }

                        match choice.finish_reason.as_deref() {
                            Some("stop") => {
                                events.push(Ok(LanguageModelCompletionEvent::Stop(
                                    StopReason::EndTurn,
                                )));
                            }
                            Some("tool_calls") => {
                                events.extend(state.tool_calls_by_index.drain().map(
                                    |(_, tool_call)| {
                                        // The model can output an empty string
                                        // to indicate the absence of arguments.
                                        // When that happens, create an empty
                                        // object instead.
                                        let arguments = if tool_call.arguments.is_empty() {
                                            Ok(serde_json::Value::Object(Default::default()))
                                        } else {
                                            serde_json::Value::from_str(&tool_call.arguments)
                                        };
                                        match arguments {
                                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                                            LanguageModelToolUse {
                                                id: tool_call.id.into(),
                                                name: tool_call.name.as_str().into(),
                                                is_input_complete: true,
                                                input,
                                                raw_input: tool_call.arguments,
                                            },
                                        )),
                                        Err(error) => Ok(
                                            LanguageModelCompletionEvent::ToolUseJsonParseError {
                                                id: tool_call.id.into(),
                                                tool_name: tool_call.name.as_str().into(),
                                                raw_input: tool_call.arguments.into(),
                                                json_parse_error: error.to_string(),
                                            },
                                        ),
                                    }
                                    },
                                ));

                                events.push(Ok(LanguageModelCompletionEvent::Stop(
                                    StopReason::ToolUse,
                                )));
                            }
                            Some(stop_reason) => {
                                log::error!("Unexpected Copilot Chat stop_reason: {stop_reason:?}");
                                events.push(Ok(LanguageModelCompletionEvent::Stop(
                                    StopReason::EndTurn,
                                )));
                            }
                            None => {}
                        }

                        return Some((events, state));
                    }
                    Err(err) => return Some((vec![Err(anyhow!(err).into())], state)),
                }
            }

            None
        },
    )
    .flat_map(futures::stream::iter)
}

pub struct CopilotResponsesEventMapper {
    pending_stop_reason: Option<StopReason>,
}

impl CopilotResponsesEventMapper {
    pub fn new() -> Self {
        Self {
            pending_stop_reason: None,
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<copilot::copilot_responses::StreamEvent>>>>,
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
        event: copilot::copilot_responses::StreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        match event {
            copilot::copilot_responses::StreamEvent::OutputItemAdded { item, .. } => match item {
                copilot::copilot_responses::ResponseOutputItem::Message { id, .. } => {
                    vec![Ok(LanguageModelCompletionEvent::StartMessage {
                        message_id: id,
                    })]
                }
                _ => Vec::new(),
            },

            copilot::copilot_responses::StreamEvent::OutputTextDelta { delta, .. } => {
                if delta.is_empty() {
                    Vec::new()
                } else {
                    vec![Ok(LanguageModelCompletionEvent::Text(delta))]
                }
            }

            copilot::copilot_responses::StreamEvent::OutputItemDone { item, .. } => match item {
                copilot::copilot_responses::ResponseOutputItem::Message { .. } => Vec::new(),
                copilot::copilot_responses::ResponseOutputItem::FunctionCall {
                    call_id,
                    name,
                    arguments,
                    ..
                } => {
                    let mut events = Vec::new();
                    match serde_json::from_str::<serde_json::Value>(&arguments) {
                        Ok(input) => events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: call_id.into(),
                                name: name.as_str().into(),
                                is_input_complete: true,
                                input,
                                raw_input: arguments.clone(),
                            },
                        ))),
                        Err(error) => {
                            events.push(Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                id: call_id.into(),
                                tool_name: name.as_str().into(),
                                raw_input: arguments.clone().into(),
                                json_parse_error: error.to_string(),
                            }))
                        }
                    }
                    // Record that we already emitted a tool-use stop so we can avoid duplicating
                    // a Stop event on Completed.
                    self.pending_stop_reason = Some(StopReason::ToolUse);
                    events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
                    events
                }
                copilot::copilot_responses::ResponseOutputItem::Reasoning {
                    summary,
                    encrypted_content,
                    ..
                } => {
                    let mut events = Vec::new();

                    if let Some(blocks) = summary {
                        let mut text = String::new();
                        for block in blocks {
                            text.push_str(&block.text);
                        }
                        if !text.is_empty() {
                            events.push(Ok(LanguageModelCompletionEvent::Thinking {
                                text,
                                signature: None,
                            }));
                        }
                    }

                    if let Some(data) = encrypted_content {
                        events.push(Ok(LanguageModelCompletionEvent::RedactedThinking { data }));
                    }

                    events
                }
            },

            copilot::copilot_responses::StreamEvent::Completed { response } => {
                let mut events = Vec::new();
                if let Some(usage) = response.usage {
                    events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                        input_tokens: usage.input_tokens.unwrap_or(0),
                        output_tokens: usage.output_tokens.unwrap_or(0),
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    })));
                }
                if self.pending_stop_reason.take() != Some(StopReason::ToolUse) {
                    events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
                }
                events
            }

            copilot::copilot_responses::StreamEvent::Incomplete { response } => {
                let reason = response
                    .incomplete_details
                    .as_ref()
                    .and_then(|details| details.reason.as_ref());
                let stop_reason = match reason {
                    Some(copilot::copilot_responses::IncompleteReason::MaxOutputTokens) => {
                        StopReason::MaxTokens
                    }
                    Some(copilot::copilot_responses::IncompleteReason::ContentFilter) => {
                        StopReason::Refusal
                    }
                    _ => self
                        .pending_stop_reason
                        .take()
                        .unwrap_or(StopReason::EndTurn),
                };

                let mut events = Vec::new();
                if let Some(usage) = response.usage {
                    events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                        input_tokens: usage.input_tokens.unwrap_or(0),
                        output_tokens: usage.output_tokens.unwrap_or(0),
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    })));
                }
                events.push(Ok(LanguageModelCompletionEvent::Stop(stop_reason)));
                events
            }

            copilot::copilot_responses::StreamEvent::Failed { response } => {
                let provider = PROVIDER_NAME;
                let (status_code, message) = match response.error {
                    Some(error) => {
                        let status_code = StatusCode::from_str(&error.code)
                            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                        (status_code, error.message)
                    }
                    None => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "response.failed".to_string(),
                    ),
                };
                vec![Err(LanguageModelCompletionError::HttpResponseError {
                    provider,
                    status_code,
                    message,
                })]
            }

            copilot::copilot_responses::StreamEvent::GenericError { error } => vec![Err(
                LanguageModelCompletionError::Other(anyhow!(format!("{error:?}"))),
            )],

            copilot::copilot_responses::StreamEvent::Created { .. }
            | copilot::copilot_responses::StreamEvent::Unknown => Vec::new(),
        }
    }
}

fn into_copilot_chat(
    model: &copilot::copilot_chat::Model,
    request: LanguageModelRequest,
) -> Result<CopilotChatRequest> {
    let mut request_messages: Vec<LanguageModelRequestMessage> = Vec::new();
    for message in request.messages {
        if let Some(last_message) = request_messages.last_mut() {
            if last_message.role == message.role {
                last_message.content.extend(message.content);
            } else {
                request_messages.push(message);
            }
        } else {
            request_messages.push(message);
        }
    }

    let mut messages: Vec<ChatMessage> = Vec::new();
    for message in request_messages {
        match message.role {
            Role::User => {
                for content in &message.content {
                    if let MessageContent::ToolResult(tool_result) = content {
                        let content = match &tool_result.content {
                            LanguageModelToolResultContent::Text(text) => text.to_string().into(),
                            LanguageModelToolResultContent::Image(image) => {
                                if model.supports_vision() {
                                    ChatMessageContent::Multipart(vec![ChatMessagePart::Image {
                                        image_url: ImageUrl {
                                            url: image.to_base64_url(),
                                        },
                                    }])
                                } else {
                                    debug_panic!(
                                        "This should be caught at {} level",
                                        tool_result.tool_name
                                    );
                                    "[Tool responded with an image, but this model does not support vision]".to_string().into()
                                }
                            }
                        };

                        messages.push(ChatMessage::Tool {
                            tool_call_id: tool_result.tool_use_id.to_string(),
                            content,
                        });
                    }
                }

                let mut content_parts = Vec::new();
                for content in &message.content {
                    match content {
                        MessageContent::Text(text) | MessageContent::Thinking { text, .. }
                            if !text.is_empty() =>
                        {
                            if let Some(ChatMessagePart::Text { text: text_content }) =
                                content_parts.last_mut()
                            {
                                text_content.push_str(text);
                            } else {
                                content_parts.push(ChatMessagePart::Text {
                                    text: text.to_string(),
                                });
                            }
                        }
                        MessageContent::Image(image) if model.supports_vision() => {
                            content_parts.push(ChatMessagePart::Image {
                                image_url: ImageUrl {
                                    url: image.to_base64_url(),
                                },
                            });
                        }
                        _ => {}
                    }
                }

                if !content_parts.is_empty() {
                    messages.push(ChatMessage::User {
                        content: content_parts.into(),
                    });
                }
            }
            Role::Assistant => {
                let mut tool_calls = Vec::new();
                for content in &message.content {
                    if let MessageContent::ToolUse(tool_use) = content {
                        tool_calls.push(ToolCall {
                            id: tool_use.id.to_string(),
                            content: copilot::copilot_chat::ToolCallContent::Function {
                                function: copilot::copilot_chat::FunctionContent {
                                    name: tool_use.name.to_string(),
                                    arguments: serde_json::to_string(&tool_use.input)?,
                                },
                            },
                        });
                    }
                }

                let text_content = {
                    let mut buffer = String::new();
                    for string in message.content.iter().filter_map(|content| match content {
                        MessageContent::Text(text) | MessageContent::Thinking { text, .. } => {
                            Some(text.as_str())
                        }
                        MessageContent::ToolUse(_)
                        | MessageContent::RedactedThinking(_)
                        | MessageContent::ToolResult(_)
                        | MessageContent::Image(_) => None,
                    }) {
                        buffer.push_str(string);
                    }

                    buffer
                };

                messages.push(ChatMessage::Assistant {
                    content: if text_content.is_empty() {
                        ChatMessageContent::empty()
                    } else {
                        text_content.into()
                    },
                    tool_calls,
                });
            }
            Role::System => messages.push(ChatMessage::System {
                content: message.string_contents(),
            }),
        }
    }

    let tools = request
        .tools
        .iter()
        .map(|tool| Tool::Function {
            function: copilot::copilot_chat::Function {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.input_schema.clone(),
            },
        })
        .collect::<Vec<_>>();

    Ok(CopilotChatRequest {
        intent: true,
        n: 1,
        stream: model.uses_streaming(),
        temperature: 0.1,
        model: model.id().to_string(),
        messages,
        tools,
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => copilot::copilot_chat::ToolChoice::Auto,
            LanguageModelToolChoice::Any => copilot::copilot_chat::ToolChoice::Any,
            LanguageModelToolChoice::None => copilot::copilot_chat::ToolChoice::None,
        }),
    })
}

fn into_copilot_responses(
    model: &copilot::copilot_chat::Model,
    request: LanguageModelRequest,
) -> copilot::copilot_responses::Request {
    use copilot::copilot_responses as responses;

    let LanguageModelRequest {
        thread_id: _,
        prompt_id: _,
        intent: _,
        mode: _,
        messages,
        tools,
        tool_choice,
        stop: _,
        temperature,
        thinking_allowed: _,
    } = request;

    let mut input_items: Vec<responses::ResponseInputItem> = Vec::new();

    for message in messages {
        match message.role {
            Role::User => {
                for content in &message.content {
                    if let MessageContent::ToolResult(tool_result) = content {
                        let output = if let Some(out) = &tool_result.output {
                            match out {
                                serde_json::Value::String(s) => {
                                    responses::ResponseFunctionOutput::Text(s.clone())
                                }
                                serde_json::Value::Null => {
                                    responses::ResponseFunctionOutput::Text(String::new())
                                }
                                other => responses::ResponseFunctionOutput::Text(other.to_string()),
                            }
                        } else {
                            match &tool_result.content {
                                LanguageModelToolResultContent::Text(text) => {
                                    responses::ResponseFunctionOutput::Text(text.to_string())
                                }
                                LanguageModelToolResultContent::Image(image) => {
                                    if model.supports_vision() {
                                        responses::ResponseFunctionOutput::Content(vec![
                                            responses::ResponseInputContent::InputImage {
                                                image_url: Some(image.to_base64_url()),
                                                detail: Default::default(),
                                            },
                                        ])
                                    } else {
                                        debug_panic!(
                                            "This should be caught at {} level",
                                            tool_result.tool_name
                                        );
                                        responses::ResponseFunctionOutput::Text(
                                            "[Tool responded with an image, but this model does not support vision]".into(),
                                        )
                                    }
                                }
                            }
                        };

                        input_items.push(responses::ResponseInputItem::FunctionCallOutput {
                            call_id: tool_result.tool_use_id.to_string(),
                            output,
                            status: None,
                        });
                    }
                }

                let mut parts: Vec<responses::ResponseInputContent> = Vec::new();
                for content in &message.content {
                    match content {
                        MessageContent::Text(text) => {
                            parts.push(responses::ResponseInputContent::InputText {
                                text: text.clone(),
                            });
                        }

                        MessageContent::Image(image) => {
                            if model.supports_vision() {
                                parts.push(responses::ResponseInputContent::InputImage {
                                    image_url: Some(image.to_base64_url()),
                                    detail: Default::default(),
                                });
                            }
                        }
                        _ => {}
                    }
                }

                if !parts.is_empty() {
                    input_items.push(responses::ResponseInputItem::Message {
                        role: "user".into(),
                        content: Some(parts),
                        status: None,
                    });
                }
            }

            Role::Assistant => {
                for content in &message.content {
                    if let MessageContent::ToolUse(tool_use) = content {
                        input_items.push(responses::ResponseInputItem::FunctionCall {
                            call_id: tool_use.id.to_string(),
                            name: tool_use.name.to_string(),
                            arguments: tool_use.raw_input.clone(),
                            status: None,
                        });
                    }
                }

                for content in &message.content {
                    if let MessageContent::RedactedThinking(data) = content {
                        input_items.push(responses::ResponseInputItem::Reasoning {
                            id: None,
                            summary: Vec::new(),
                            encrypted_content: data.clone(),
                        });
                    }
                }

                let mut parts: Vec<responses::ResponseInputContent> = Vec::new();
                for content in &message.content {
                    match content {
                        MessageContent::Text(text) => {
                            parts.push(responses::ResponseInputContent::OutputText {
                                text: text.clone(),
                            });
                        }
                        MessageContent::Image(_) => {
                            parts.push(responses::ResponseInputContent::OutputText {
                                text: "[image omitted]".to_string(),
                            });
                        }
                        _ => {}
                    }
                }

                if !parts.is_empty() {
                    input_items.push(responses::ResponseInputItem::Message {
                        role: "assistant".into(),
                        content: Some(parts),
                        status: Some("completed".into()),
                    });
                }
            }

            Role::System => {
                let mut parts: Vec<responses::ResponseInputContent> = Vec::new();
                for content in &message.content {
                    if let MessageContent::Text(text) = content {
                        parts.push(responses::ResponseInputContent::InputText {
                            text: text.clone(),
                        });
                    }
                }

                if !parts.is_empty() {
                    input_items.push(responses::ResponseInputItem::Message {
                        role: "system".into(),
                        content: Some(parts),
                        status: None,
                    });
                }
            }
        }
    }

    let converted_tools: Vec<responses::ToolDefinition> = tools
        .into_iter()
        .map(|tool| responses::ToolDefinition::Function {
            name: tool.name,
            description: Some(tool.description),
            parameters: Some(tool.input_schema),
            strict: None,
        })
        .collect();

    let mapped_tool_choice = tool_choice.map(|choice| match choice {
        LanguageModelToolChoice::Auto => responses::ToolChoice::Auto,
        LanguageModelToolChoice::Any => responses::ToolChoice::Any,
        LanguageModelToolChoice::None => responses::ToolChoice::None,
    });

    responses::Request {
        model: model.id().to_string(),
        input: input_items,
        stream: model.uses_streaming(),
        temperature,
        tools: converted_tools,
        tool_choice: mapped_tool_choice,
        reasoning: None, // We would need to add support for setting from user settings.
        include: Some(vec![
            copilot::copilot_responses::ResponseIncludable::ReasoningEncryptedContent,
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use copilot::copilot_responses as responses;
    use futures::StreamExt;

    fn map_events(events: Vec<responses::StreamEvent>) -> Vec<LanguageModelCompletionEvent> {
        futures::executor::block_on(async {
            CopilotResponsesEventMapper::new()
                .map_stream(Box::pin(futures::stream::iter(events.into_iter().map(Ok))))
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .map(Result::unwrap)
                .collect()
        })
    }

    #[test]
    fn responses_stream_maps_text_and_usage() {
        let events = vec![
            responses::StreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: responses::ResponseOutputItem::Message {
                    id: "msg_1".into(),
                    role: "assistant".into(),
                    content: Some(Vec::new()),
                },
            },
            responses::StreamEvent::OutputTextDelta {
                item_id: "msg_1".into(),
                output_index: 0,
                delta: "Hello".into(),
            },
            responses::StreamEvent::Completed {
                response: responses::Response {
                    usage: Some(responses::ResponseUsage {
                        input_tokens: Some(5),
                        output_tokens: Some(3),
                        total_tokens: Some(8),
                    }),
                    ..Default::default()
                },
            },
        ];

        let mapped = map_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::StartMessage { ref message_id } if message_id == "msg_1"
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
    fn responses_stream_maps_tool_calls() {
        let events = vec![responses::StreamEvent::OutputItemDone {
            output_index: 0,
            sequence_number: None,
            item: responses::ResponseOutputItem::FunctionCall {
                id: Some("fn_1".into()),
                call_id: "call_1".into(),
                name: "do_it".into(),
                arguments: "{\"x\":1}".into(),
                status: None,
            },
        }];

        let mapped = map_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::ToolUse(ref use_) if use_.id.to_string() == "call_1" && use_.name.as_ref() == "do_it"
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Stop(StopReason::ToolUse)
        ));
    }

    #[test]
    fn responses_stream_handles_json_parse_error() {
        let events = vec![responses::StreamEvent::OutputItemDone {
            output_index: 0,
            sequence_number: None,
            item: responses::ResponseOutputItem::FunctionCall {
                id: Some("fn_1".into()),
                call_id: "call_1".into(),
                name: "do_it".into(),
                arguments: "{not json}".into(),
                status: None,
            },
        }];

        let mapped = map_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::ToolUseJsonParseError { ref id, ref tool_name, .. }
                if id.to_string() == "call_1" && tool_name.as_ref() == "do_it"
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Stop(StopReason::ToolUse)
        ));
    }

    #[test]
    fn responses_stream_maps_reasoning_summary_and_encrypted_content() {
        let events = vec![responses::StreamEvent::OutputItemDone {
            output_index: 0,
            sequence_number: None,
            item: responses::ResponseOutputItem::Reasoning {
                id: "r1".into(),
                summary: Some(vec![responses::ResponseReasoningItem {
                    kind: "summary_text".into(),
                    text: "Chain".into(),
                }]),
                encrypted_content: Some("ENC".into()),
            },
        }];

        let mapped = map_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::Thinking { ref text, signature: None } if text == "Chain"
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::RedactedThinking { ref data } if data == "ENC"
        ));
    }

    #[test]
    fn responses_stream_handles_incomplete_max_tokens() {
        let events = vec![responses::StreamEvent::Incomplete {
            response: responses::Response {
                usage: Some(responses::ResponseUsage {
                    input_tokens: Some(10),
                    output_tokens: Some(0),
                    total_tokens: Some(10),
                }),
                incomplete_details: Some(responses::IncompleteDetails {
                    reason: Some(responses::IncompleteReason::MaxOutputTokens),
                }),
                ..Default::default()
            },
        }];

        let mapped = map_events(events);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: 10,
                output_tokens: 0,
                ..
            })
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Stop(StopReason::MaxTokens)
        ));
    }

    #[test]
    fn responses_stream_handles_incomplete_content_filter() {
        let events = vec![responses::StreamEvent::Incomplete {
            response: responses::Response {
                usage: None,
                incomplete_details: Some(responses::IncompleteDetails {
                    reason: Some(responses::IncompleteReason::ContentFilter),
                }),
                ..Default::default()
            },
        }];

        let mapped = map_events(events);
        assert!(matches!(
            mapped.last().unwrap(),
            LanguageModelCompletionEvent::Stop(StopReason::Refusal)
        ));
    }

    #[test]
    fn responses_stream_completed_no_duplicate_after_tool_use() {
        let events = vec![
            responses::StreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: responses::ResponseOutputItem::FunctionCall {
                    id: Some("fn_1".into()),
                    call_id: "call_1".into(),
                    name: "do_it".into(),
                    arguments: "{}".into(),
                    status: None,
                },
            },
            responses::StreamEvent::Completed {
                response: responses::Response::default(),
            },
        ];

        let mapped = map_events(events);

        let mut stop_count = 0usize;
        let mut saw_tool_use_stop = false;
        for event in mapped {
            if let LanguageModelCompletionEvent::Stop(reason) = event {
                stop_count += 1;
                if matches!(reason, StopReason::ToolUse) {
                    saw_tool_use_stop = true;
                }
            }
        }
        assert_eq!(stop_count, 1, "should emit exactly one Stop event");
        assert!(saw_tool_use_stop, "Stop reason should be ToolUse");
    }

    #[test]
    fn responses_stream_failed_maps_http_response_error() {
        let events = vec![responses::StreamEvent::Failed {
            response: responses::Response {
                error: Some(responses::ResponseError {
                    code: "429".into(),
                    message: "too many requests".into(),
                }),
                ..Default::default()
            },
        }];

        let mapped_results = futures::executor::block_on(async {
            CopilotResponsesEventMapper::new()
                .map_stream(Box::pin(futures::stream::iter(events.into_iter().map(Ok))))
                .collect::<Vec<_>>()
                .await
        });

        assert_eq!(mapped_results.len(), 1);
        match &mapped_results[0] {
            Err(LanguageModelCompletionError::HttpResponseError {
                status_code,
                message,
                ..
            }) => {
                assert_eq!(*status_code, http_client::StatusCode::TOO_MANY_REQUESTS);
                assert_eq!(message, "too many requests");
            }
            other => panic!("expected HttpResponseError, got {:?}", other),
        }
    }
}
struct ConfigurationView {
    copilot_status: Option<copilot::Status>,
    state: Entity<State>,
    _subscription: Option<Subscription>,
}

impl ConfigurationView {
    pub fn new(state: Entity<State>, cx: &mut Context<Self>) -> Self {
        let copilot = Copilot::global(cx);

        Self {
            copilot_status: copilot.as_ref().map(|copilot| copilot.read(cx).status()),
            state,
            _subscription: copilot.as_ref().map(|copilot| {
                cx.observe(copilot, |this, model, cx| {
                    this.copilot_status = Some(model.read(cx).status());
                    cx.notify();
                })
            }),
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.state.read(cx).is_authenticated(cx) {
            ConfiguredApiCard::new("Authorized")
                .button_label("Sign Out")
                .on_click(|_, window, cx| {
                    window.dispatch_action(copilot::SignOut.boxed_clone(), cx);
                })
                .into_any_element()
        } else {
            let loading_icon = Icon::new(IconName::ArrowCircle).with_rotate_animation(4);

            const ERROR_LABEL: &str = "Copilot Chat requires an active GitHub Copilot subscription. Please ensure Copilot is configured and try again, or use a different Assistant provider.";

            match &self.copilot_status {
                Some(status) => match status {
                    Status::Starting { task: _ } => h_flex()
                        .gap_2()
                        .child(loading_icon)
                        .child(Label::new("Starting Copilot…"))
                        .into_any_element(),
                    Status::SigningIn { prompt: _ }
                    | Status::SignedOut {
                        awaiting_signing_in: true,
                    } => h_flex()
                        .gap_2()
                        .child(loading_icon)
                        .child(Label::new("Signing into Copilot…"))
                        .into_any_element(),
                    Status::Error(_) => {
                        const LABEL: &str = "Copilot had issues starting. Please try restarting it. If the issue persists, try reinstalling Copilot.";
                        v_flex()
                            .gap_6()
                            .child(Label::new(LABEL))
                            .child(svg().size_8().path(IconName::CopilotError.path()))
                            .into_any_element()
                    }
                    _ => {
                        const LABEL: &str = "To use Zed's agent with GitHub Copilot, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot Chat subscription.";

                        v_flex()
                            .gap_2()
                            .child(Label::new(LABEL))
                            .child(
                                Button::new("sign_in", "Sign in to use GitHub Copilot")
                                    .full_width()
                                    .style(ButtonStyle::Outlined)
                                    .icon_color(Color::Muted)
                                    .icon(IconName::Github)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
                                    .on_click(|_, window, cx| {
                                        copilot::initiate_sign_in(window, cx)
                                    }),
                            )
                            .into_any_element()
                    }
                },
                None => v_flex()
                    .gap_6()
                    .child(Label::new(ERROR_LABEL))
                    .into_any_element(),
            }
        }
    }
}
