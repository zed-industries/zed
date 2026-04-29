use anyhow::{Result, anyhow};
use collections::HashMap;
use futures::{Stream, StreamExt};
use language_model_core::{
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelImage,
    LanguageModelRequest, LanguageModelRequestMessage, LanguageModelToolChoice,
    LanguageModelToolResultContent, LanguageModelToolUse, LanguageModelToolUseId, MessageContent,
    Role, StopReason, TokenUsage,
    util::{fix_streamed_json, parse_tool_arguments},
};
use std::pin::Pin;
use std::sync::Arc;

use crate::responses::{
    Request as ResponseRequest, ResponseFunctionCallItem, ResponseFunctionCallOutputContent,
    ResponseFunctionCallOutputItem, ResponseInputContent, ResponseInputItem, ResponseMessageItem,
    ResponseOutputItem, ResponseSummary as ResponsesSummary, ResponseUsage as ResponsesUsage,
    StreamEvent as ResponsesStreamEvent,
};
use crate::{
    FunctionContent, FunctionDefinition, ImageUrl, MessagePart, ReasoningEffort,
    ResponseStreamEvent, ToolCall, ToolCallContent,
};

pub fn into_open_ai(
    request: LanguageModelRequest,
    model_id: &str,
    supports_parallel_tool_calls: bool,
    supports_prompt_cache_key: bool,
    max_output_tokens: Option<u64>,
    reasoning_effort: Option<ReasoningEffort>,
    interleaved_reasoning: bool,
) -> crate::Request {
    let stream = !model_id.starts_with("o1-");

    let mut messages = Vec::new();
    let mut current_reasoning: Option<String> = None;
    for message in request.messages {
        for content in message.content {
            match content {
                MessageContent::Thinking { text, .. } if interleaved_reasoning => {
                    current_reasoning.get_or_insert_default().push_str(&text);
                }
                MessageContent::Text(text) | MessageContent::Thinking { text, .. } => {
                    let should_add = if message.role == Role::User {
                        // Including whitespace-only user messages can cause error with OpenAI compatible APIs
                        // See https://github.com/zed-industries/zed/issues/40097
                        !text.trim().is_empty()
                    } else {
                        !text.is_empty()
                    };
                    if should_add {
                        add_message_content_part(
                            MessagePart::Text { text },
                            message.role,
                            &mut messages,
                        );
                        if let Some(reasoning) = current_reasoning.take() {
                            if let Some(crate::RequestMessage::Assistant {
                                reasoning_content,
                                ..
                            }) = messages.last_mut()
                            {
                                *reasoning_content = Some(reasoning);
                            }
                        }
                    }
                }
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Image(image) => {
                    add_message_content_part(
                        MessagePart::Image {
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
                    let tool_call = ToolCall {
                        id: tool_use.id.to_string(),
                        content: ToolCallContent::Function {
                            function: FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                            },
                        },
                    };

                    if let Some(crate::RequestMessage::Assistant { tool_calls, .. }) =
                        messages.last_mut()
                    {
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(crate::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                            reasoning_content: current_reasoning.take(),
                        });
                    }
                }
                MessageContent::ToolResult(tool_result) => {
                    let content: Vec<MessagePart> = tool_result
                        .content
                        .iter()
                        .map(|part| match part {
                            LanguageModelToolResultContent::Text(text) => MessagePart::Text {
                                text: text.to_string(),
                            },
                            LanguageModelToolResultContent::Image(image) => MessagePart::Image {
                                image_url: ImageUrl {
                                    url: image.to_base64_url(),
                                    detail: None,
                                },
                            },
                        })
                        .collect();

                    messages.push(crate::RequestMessage::Tool {
                        content: content.into(),
                        tool_call_id: tool_result.tool_use_id.to_string(),
                    });
                }
            }
        }
    }

    crate::Request {
        model: model_id.into(),
        messages,
        stream,
        stream_options: if stream {
            Some(crate::StreamOptions::default())
        } else {
            None
        },
        stop: request.stop,
        temperature: request.temperature.or(Some(1.0)),
        max_completion_tokens: max_output_tokens,
        parallel_tool_calls: if supports_parallel_tool_calls && !request.tools.is_empty() {
            Some(supports_parallel_tool_calls)
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
            .map(|tool| crate::ToolDefinition::Function {
                function: FunctionDefinition {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect(),
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => crate::ToolChoice::Auto,
            LanguageModelToolChoice::Any => crate::ToolChoice::Required,
            LanguageModelToolChoice::None => crate::ToolChoice::None,
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
) -> ResponseRequest {
    let stream = !model_id.starts_with("o1-");

    let LanguageModelRequest {
        thread_id,
        prompt_id: _,
        intent: _,
        messages,
        tools,
        tool_choice,
        stop: _,
        temperature,
        thinking_allowed: _,
        thinking_effort: _,
        speed: _,
    } = request;

    let mut input_items = Vec::new();
    for (index, message) in messages.into_iter().enumerate() {
        append_message_to_response_items(message, index, &mut input_items);
    }

    let tools: Vec<_> = tools
        .into_iter()
        .map(|tool| crate::responses::ToolDefinition::Function {
            name: tool.name,
            description: Some(tool.description),
            parameters: Some(tool.input_schema),
            strict: None,
        })
        .collect();

    ResponseRequest {
        model: model_id.into(),
        input: input_items,
        stream,
        temperature,
        top_p: None,
        max_output_tokens,
        parallel_tool_calls: if tools.is_empty() {
            None
        } else {
            Some(supports_parallel_tool_calls)
        },
        tool_choice: tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => crate::ToolChoice::Auto,
            LanguageModelToolChoice::Any => crate::ToolChoice::Required,
            LanguageModelToolChoice::None => crate::ToolChoice::None,
        }),
        tools,
        prompt_cache_key: if supports_prompt_cache_key {
            thread_id
        } else {
            None
        },
        reasoning: reasoning_effort.map(|effort| crate::responses::ReasoningConfig {
            effort,
            summary: Some(crate::responses::ReasoningSummaryMode::Auto),
        }),
    }
}

fn append_message_to_response_items(
    message: LanguageModelRequestMessage,
    index: usize,
    input_items: &mut Vec<ResponseInputItem>,
) {
    let mut content_parts: Vec<ResponseInputContent> = Vec::new();

    for content in message.content {
        match content {
            MessageContent::Text(text) => {
                push_response_text_part(&message.role, text, &mut content_parts);
            }
            MessageContent::Thinking { text, .. } => {
                push_response_text_part(&message.role, text, &mut content_parts);
            }
            MessageContent::RedactedThinking(_) => {}
            MessageContent::Image(image) => {
                push_response_image_part(&message.role, image, &mut content_parts);
            }
            MessageContent::ToolUse(tool_use) => {
                flush_response_parts(&message.role, index, &mut content_parts, input_items);
                let call_id = tool_use.id.to_string();
                input_items.push(ResponseInputItem::FunctionCall(ResponseFunctionCallItem {
                    call_id,
                    name: tool_use.name.to_string(),
                    arguments: tool_use.raw_input,
                }));
            }
            MessageContent::ToolResult(tool_result) => {
                flush_response_parts(&message.role, index, &mut content_parts, input_items);
                let output = match tool_result.content.as_slice() {
                    [LanguageModelToolResultContent::Text(text)] => {
                        ResponseFunctionCallOutputContent::Text(text.to_string())
                    }
                    _ => {
                        let parts = tool_result
                            .content
                            .into_iter()
                            .map(|part| match part {
                                LanguageModelToolResultContent::Text(text) => {
                                    ResponseInputContent::Text {
                                        text: text.to_string(),
                                    }
                                }
                                LanguageModelToolResultContent::Image(image) => {
                                    ResponseInputContent::Image {
                                        image_url: image.to_base64_url(),
                                    }
                                }
                            })
                            .collect();
                        ResponseFunctionCallOutputContent::List(parts)
                    }
                };
                input_items.push(ResponseInputItem::FunctionCallOutput(
                    ResponseFunctionCallOutputItem {
                        call_id: tool_result.tool_use_id.to_string(),
                        output,
                    },
                ));
            }
        }
    }

    flush_response_parts(&message.role, index, &mut content_parts, input_items);
}

fn push_response_text_part(
    role: &Role,
    text: impl Into<String>,
    parts: &mut Vec<ResponseInputContent>,
) {
    let text = text.into();
    if text.trim().is_empty() {
        return;
    }

    match role {
        Role::Assistant => parts.push(ResponseInputContent::OutputText {
            text,
            annotations: Vec::new(),
        }),
        _ => parts.push(ResponseInputContent::Text { text }),
    }
}

fn push_response_image_part(
    role: &Role,
    image: LanguageModelImage,
    parts: &mut Vec<ResponseInputContent>,
) {
    match role {
        Role::Assistant => parts.push(ResponseInputContent::OutputText {
            text: "[image omitted]".to_string(),
            annotations: Vec::new(),
        }),
        _ => parts.push(ResponseInputContent::Image {
            image_url: image.to_base64_url(),
        }),
    }
}

fn flush_response_parts(
    role: &Role,
    _index: usize,
    parts: &mut Vec<ResponseInputContent>,
    input_items: &mut Vec<ResponseInputItem>,
) {
    if parts.is_empty() {
        return;
    }

    let item = ResponseInputItem::Message(ResponseMessageItem {
        role: match role {
            Role::User => crate::Role::User,
            Role::Assistant => crate::Role::Assistant,
            Role::System => crate::Role::System,
        },
        content: parts.clone(),
    });

    input_items.push(item);
    parts.clear();
}

fn add_message_content_part(
    new_part: MessagePart,
    role: Role,
    messages: &mut Vec<crate::RequestMessage>,
) {
    match (role, messages.last_mut()) {
        (Role::User, Some(crate::RequestMessage::User { content }))
        | (
            Role::Assistant,
            Some(crate::RequestMessage::Assistant {
                content: Some(content),
                ..
            }),
        )
        | (Role::System, Some(crate::RequestMessage::System { content, .. })) => {
            content.push_part(new_part);
        }
        _ => {
            messages.push(match role {
                Role::User => crate::RequestMessage::User {
                    content: crate::MessageContent::from(vec![new_part]),
                },
                Role::Assistant => crate::RequestMessage::Assistant {
                    content: Some(crate::MessageContent::from(vec![new_part])),
                    tool_calls: Vec::new(),
                    reasoning_content: None,
                },
                Role::System => crate::RequestMessage::System {
                    content: crate::MessageContent::from(vec![new_part]),
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

        if let Some(delta) = choice.delta.as_ref() {
            if let Some(reasoning_content) = delta.reasoning_content.clone() {
                if !reasoning_content.is_empty() {
                    events.push(Ok(LanguageModelCompletionEvent::Thinking {
                        text: reasoning_content,
                        signature: None,
                    }));
                }
            }
            if let Some(content) = delta.content.clone() {
                if !content.is_empty() {
                    events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                }
            }

            if let Some(tool_calls) = delta.tool_calls.as_ref() {
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

                    if !entry.id.is_empty() && !entry.name.is_empty() {
                        if let Ok(input) = serde_json::from_str::<serde_json::Value>(
                            &fix_streamed_json(&entry.arguments),
                        ) {
                            events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id: entry.id.clone().into(),
                                    name: entry.name.as_str().into(),
                                    is_input_complete: false,
                                    input,
                                    raw_input: entry.arguments.clone(),
                                    thought_signature: None,
                                },
                            )));
                        }
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
                    match parse_tool_arguments(&tool_call.arguments) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_call.id.clone().into(),
                                name: tool_call.name.as_str().into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_call.arguments.clone(),
                                thought_signature: None,
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

pub struct OpenAiResponseEventMapper {
    function_calls_by_item: HashMap<String, PendingResponseFunctionCall>,
    pending_stop_reason: Option<StopReason>,
}

#[derive(Default)]
struct PendingResponseFunctionCall {
    call_id: String,
    name: Arc<str>,
    arguments: String,
}

impl OpenAiResponseEventMapper {
    pub fn new() -> Self {
        Self {
            function_calls_by_item: HashMap::default(),
            pending_stop_reason: None,
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

    pub fn map_event(
        &mut self,
        event: ResponsesStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        match event {
            ResponsesStreamEvent::OutputItemAdded { item, .. } => {
                let mut events = Vec::new();

                match &item {
                    ResponseOutputItem::Message(message) => {
                        if let Some(id) = &message.id {
                            events.push(Ok(LanguageModelCompletionEvent::StartMessage {
                                message_id: id.clone(),
                            }));
                        }
                    }
                    ResponseOutputItem::FunctionCall(function_call) => {
                        if let Some(item_id) = function_call.id.clone() {
                            let call_id = function_call
                                .call_id
                                .clone()
                                .or_else(|| function_call.id.clone())
                                .unwrap_or_else(|| item_id.clone());
                            let entry = PendingResponseFunctionCall {
                                call_id,
                                name: Arc::<str>::from(
                                    function_call.name.clone().unwrap_or_default(),
                                ),
                                arguments: function_call.arguments.clone(),
                            };
                            self.function_calls_by_item.insert(item_id, entry);
                        }
                    }
                    ResponseOutputItem::Reasoning(_) | ResponseOutputItem::Unknown => {}
                }
                events
            }
            ResponsesStreamEvent::ReasoningSummaryTextDelta { delta, .. } => {
                if delta.is_empty() {
                    Vec::new()
                } else {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: delta,
                        signature: None,
                    })]
                }
            }
            ResponsesStreamEvent::OutputTextDelta { delta, .. } => {
                if delta.is_empty() {
                    Vec::new()
                } else {
                    vec![Ok(LanguageModelCompletionEvent::Text(delta))]
                }
            }
            ResponsesStreamEvent::FunctionCallArgumentsDelta { item_id, delta, .. } => {
                if let Some(entry) = self.function_calls_by_item.get_mut(&item_id) {
                    entry.arguments.push_str(&delta);
                    if let Ok(input) = serde_json::from_str::<serde_json::Value>(
                        &fix_streamed_json(&entry.arguments),
                    ) {
                        return vec![Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: LanguageModelToolUseId::from(entry.call_id.clone()),
                                name: entry.name.clone(),
                                is_input_complete: false,
                                input,
                                raw_input: entry.arguments.clone(),
                                thought_signature: None,
                            },
                        ))];
                    }
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
                    match parse_tool_arguments(&entry.arguments) {
                        Ok(input) => {
                            vec![Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id: LanguageModelToolUseId::from(entry.call_id.clone()),
                                    name: entry.name.clone(),
                                    is_input_complete: true,
                                    input,
                                    raw_input,
                                    thought_signature: None,
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
                vec![Err(LanguageModelCompletionError::Other(anyhow!(
                    error.message
                )))]
            }
            ResponsesStreamEvent::ReasoningSummaryPartAdded { summary_index, .. } => {
                if summary_index > 0 {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: "\n\n".to_string(),
                        signature: None,
                    })]
                } else {
                    Vec::new()
                }
            }
            ResponsesStreamEvent::OutputTextDone { .. }
            | ResponsesStreamEvent::OutputItemDone { .. }
            | ResponsesStreamEvent::ContentPartAdded { .. }
            | ResponsesStreamEvent::ContentPartDone { .. }
            | ResponsesStreamEvent::ReasoningSummaryTextDone { .. }
            | ResponsesStreamEvent::ReasoningSummaryPartDone { .. }
            | ResponsesStreamEvent::Created { .. }
            | ResponsesStreamEvent::InProgress { .. }
            | ResponsesStreamEvent::Unknown => Vec::new(),
        }
    }

    fn handle_completion(
        &mut self,
        response: ResponsesSummary,
        default_reason: StopReason,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();

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
        output: &[ResponseOutputItem],
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();
        for item in output {
            if let ResponseOutputItem::FunctionCall(function_call) = item {
                let Some(call_id) = function_call
                    .call_id
                    .clone()
                    .or_else(|| function_call.id.clone())
                else {
                    log::error!(
                        "Function call item missing both call_id and id: {:?}",
                        function_call
                    );
                    continue;
                };
                let name: Arc<str> = Arc::from(function_call.name.clone().unwrap_or_default());
                let arguments = &function_call.arguments;
                self.pending_stop_reason = Some(StopReason::ToolUse);
                match parse_tool_arguments(arguments) {
                    Ok(input) => {
                        events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: LanguageModelToolUseId::from(call_id.clone()),
                                name: name.clone(),
                                is_input_complete: true,
                                input,
                                raw_input: arguments.clone(),
                                thought_signature: None,
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

#[cfg(test)]
mod tests {
    use crate::responses::{
        ReasoningSummaryPart, ResponseFunctionToolCall, ResponseOutputItem, ResponseOutputMessage,
        ResponseReasoningItem, ResponseStatusDetails, ResponseSummary, ResponseUsage,
        StreamEvent as ResponsesStreamEvent,
    };
    use futures::{StreamExt, executor::block_on};
    use language_model_core::{
        LanguageModelImage, LanguageModelRequestMessage, LanguageModelRequestTool,
        LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolUse,
        LanguageModelToolUseId, SharedString,
    };
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    fn map_response_events(events: Vec<ResponsesStreamEvent>) -> Vec<LanguageModelCompletionEvent> {
        block_on(async {
            OpenAiResponseEventMapper::new()
                .map_stream(Box::pin(futures::stream::iter(events.into_iter().map(Ok))))
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .map(Result::unwrap)
                .collect()
        })
    }

    fn response_item_message(id: &str) -> ResponseOutputItem {
        ResponseOutputItem::Message(ResponseOutputMessage {
            id: Some(id.to_string()),
            role: Some("assistant".to_string()),
            status: Some("in_progress".to_string()),
            content: vec![],
        })
    }

    fn response_item_function_call(id: &str, args: Option<&str>) -> ResponseOutputItem {
        ResponseOutputItem::FunctionCall(ResponseFunctionToolCall {
            id: Some(id.to_string()),
            status: Some("in_progress".to_string()),
            name: Some("get_weather".to_string()),
            call_id: Some("call_123".to_string()),
            arguments: args.map(|s| s.to_string()).unwrap_or_default(),
        })
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
    fn into_open_ai_response_builds_complete_payload() {
        let tool_call_id = LanguageModelToolUseId::from("call-42");
        let tool_input = json!({ "city": "Boston" });
        let tool_arguments = serde_json::to_string(&tool_input).unwrap();
        let tool_use = LanguageModelToolUse {
            id: tool_call_id.clone(),
            name: Arc::from("get_weather"),
            raw_input: tool_arguments.clone(),
            input: tool_input,
            is_input_complete: true,
            thought_signature: None,
        };
        let tool_result = LanguageModelToolResult {
            tool_use_id: tool_call_id,
            tool_name: Arc::from("get_weather"),
            is_error: false,
            content: vec![LanguageModelToolResultContent::Text(Arc::from("Sunny"))],
            output: Some(json!({ "forecast": "Sunny" })),
        };
        let user_image = LanguageModelImage {
            source: SharedString::from("aGVsbG8="),
            size: None,
        };
        let expected_image_url = user_image.to_base64_url();

        let request = LanguageModelRequest {
            thread_id: Some("thread-123".into()),
            prompt_id: None,
            intent: None,
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("System context".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![
                        MessageContent::Text("Please check the weather.".into()),
                        MessageContent::Image(user_image),
                    ],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![
                        MessageContent::Text("Looking that up.".into()),
                        MessageContent::ToolUse(tool_use),
                    ],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolResult(tool_result)],
                    cache: false,
                    reasoning_details: None,
                },
            ],
            tools: vec![LanguageModelRequestTool {
                name: "get_weather".into(),
                description: "Fetches the weather".into(),
                input_schema: json!({ "type": "object" }),
                use_input_streaming: false,
            }],
            tool_choice: Some(LanguageModelToolChoice::Any),
            stop: vec!["<STOP>".into()],
            temperature: None,
            thinking_allowed: false,
            thinking_effort: None,
            speed: None,
        };

        let response = into_open_ai_response(
            request,
            "custom-model",
            true,
            true,
            Some(2048),
            Some(ReasoningEffort::Low),
        );

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
                    "content": [
                        { "type": "output_text", "text": "Looking that up.", "annotations": [] }
                    ]
                },
                {
                    "type": "function_call",
                    "call_id": "call-42",
                    "name": "get_weather",
                    "arguments": tool_arguments
                },
                {
                    "type": "function_call_output",
                    "call_id": "call-42",
                    "output": "Sunny"
                }
            ],
            "stream": true,
            "max_output_tokens": 2048,
            "parallel_tool_calls": true,
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
            "reasoning": { "effort": "low", "summary": "auto" }
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
        assert_eq!(mapped.len(), 3);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                is_input_complete: false,
                ..
            })
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                ref id,
                ref name,
                ref raw_input,
                is_input_complete: true,
                ..
            }) if id.to_string() == "call_123"
                && name.as_ref() == "get_weather"
                && raw_input == "{\"city\":\"Boston\"}"
        ));
        assert!(matches!(
            mapped[2],
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
        assert!(
            matches!(mapped[1], LanguageModelCompletionEvent::Text(ref text) if text == "Let me check that")
        );
        assert!(
            matches!(mapped[2], LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse { ref raw_input, .. }) if raw_input == "{\"query\":\"test\"}")
        );
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
            LanguageModelCompletionEvent::ToolUseJsonParseError { ref raw_input, .. }
            if raw_input.as_ref() == "{invalid json"
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
        assert_eq!(mapped.len(), 3);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                is_input_complete: false,
                ..
            })
        ));
        assert!(
            matches!(mapped[1], LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse { ref raw_input, is_input_complete: true, .. }) if raw_input == "{\"city\":\"Boston\"}")
        );
        assert!(matches!(
            mapped[2],
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
        assert!(
            matches!(mapped[0], LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse { ref raw_input, .. }) if raw_input == "{\"city\":\"Boston\"}")
        );
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Stop(StopReason::MaxTokens)
        ));
    }

    #[test]
    fn responses_stream_handles_empty_tool_arguments() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_function_call("item_fn", Some("")),
            },
            ResponsesStreamEvent::FunctionCallArgumentsDone {
                item_id: "item_fn".into(),
                output_index: 0,
                arguments: "".into(),
                sequence_number: None,
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        assert_eq!(mapped.len(), 2);
        assert!(matches!(
            &mapped[0],
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                id, name, raw_input, input, ..
            }) if id.to_string() == "call_123"
                && name.as_ref() == "get_weather"
                && raw_input == ""
                && input.is_object()
                && input.as_object().unwrap().is_empty()
        ));
        assert!(matches!(
            mapped[1],
            LanguageModelCompletionEvent::Stop(StopReason::ToolUse)
        ));
    }

    #[test]
    fn responses_stream_emits_partial_tool_use_events() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::FunctionCall(
                    crate::responses::ResponseFunctionToolCall {
                        id: Some("item_fn".to_string()),
                        status: Some("in_progress".to_string()),
                        name: Some("get_weather".to_string()),
                        call_id: Some("call_abc".to_string()),
                        arguments: String::new(),
                    },
                ),
            },
            ResponsesStreamEvent::FunctionCallArgumentsDelta {
                item_id: "item_fn".into(),
                output_index: 0,
                delta: "{\"city\":\"Bos".into(),
                sequence_number: None,
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
        assert!(mapped.len() >= 3);

        let complete_tool_use = mapped.iter().find(|e| {
            matches!(
                e,
                LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                    is_input_complete: true,
                    ..
                })
            )
        });
        assert!(
            complete_tool_use.is_some(),
            "should have a complete tool use event"
        );

        let tool_uses: Vec<_> = mapped
            .iter()
            .filter(|e| matches!(e, LanguageModelCompletionEvent::ToolUse(_)))
            .collect();
        assert!(
            tool_uses.len() >= 2,
            "should have at least one partial and one complete event"
        );
        assert!(matches!(
            tool_uses.last().unwrap(),
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                is_input_complete: true,
                ..
            })
        ));
    }

    #[test]
    fn responses_stream_maps_reasoning_summary_deltas() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(ResponseReasoningItem {
                    id: Some("rs_123".into()),
                    summary: vec![],
                }),
            },
            ResponsesStreamEvent::ReasoningSummaryPartAdded {
                item_id: "rs_123".into(),
                output_index: 0,
                summary_index: 0,
            },
            ResponsesStreamEvent::ReasoningSummaryTextDelta {
                item_id: "rs_123".into(),
                output_index: 0,
                delta: "Thinking about".into(),
            },
            ResponsesStreamEvent::ReasoningSummaryTextDelta {
                item_id: "rs_123".into(),
                output_index: 0,
                delta: " the answer".into(),
            },
            ResponsesStreamEvent::ReasoningSummaryTextDone {
                item_id: "rs_123".into(),
                output_index: 0,
                text: "Thinking about the answer".into(),
            },
            ResponsesStreamEvent::ReasoningSummaryPartDone {
                item_id: "rs_123".into(),
                output_index: 0,
                summary_index: 0,
            },
            ResponsesStreamEvent::ReasoningSummaryPartAdded {
                item_id: "rs_123".into(),
                output_index: 0,
                summary_index: 1,
            },
            ResponsesStreamEvent::ReasoningSummaryTextDelta {
                item_id: "rs_123".into(),
                output_index: 0,
                delta: "Second part".into(),
            },
            ResponsesStreamEvent::ReasoningSummaryTextDone {
                item_id: "rs_123".into(),
                output_index: 0,
                text: "Second part".into(),
            },
            ResponsesStreamEvent::ReasoningSummaryPartDone {
                item_id: "rs_123".into(),
                output_index: 0,
                summary_index: 1,
            },
            ResponsesStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(ResponseReasoningItem {
                    id: Some("rs_123".into()),
                    summary: vec![
                        ReasoningSummaryPart::SummaryText {
                            text: "Thinking about the answer".into(),
                        },
                        ReasoningSummaryPart::SummaryText {
                            text: "Second part".into(),
                        },
                    ],
                }),
            },
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 1,
                sequence_number: None,
                item: response_item_message("msg_456"),
            },
            ResponsesStreamEvent::OutputTextDelta {
                item_id: "msg_456".into(),
                output_index: 1,
                content_index: Some(0),
                delta: "The answer is 42".into(),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);

        let thinking_events: Vec<_> = mapped
            .iter()
            .filter(|e| matches!(e, LanguageModelCompletionEvent::Thinking { .. }))
            .collect();
        assert_eq!(
            thinking_events.len(),
            4,
            "expected 4 thinking events, got {:?}",
            thinking_events
        );
        assert!(
            matches!(&thinking_events[0], LanguageModelCompletionEvent::Thinking { text, .. } if text == "Thinking about")
        );
        assert!(
            matches!(&thinking_events[1], LanguageModelCompletionEvent::Thinking { text, .. } if text == " the answer")
        );
        assert!(
            matches!(&thinking_events[2], LanguageModelCompletionEvent::Thinking { text, .. } if text == "\n\n"),
            "expected separator between summary parts"
        );
        assert!(
            matches!(&thinking_events[3], LanguageModelCompletionEvent::Thinking { text, .. } if text == "Second part")
        );

        assert!(mapped.iter().any(
            |e| matches!(e, LanguageModelCompletionEvent::Text(t) if t == "The answer is 42")
        ));
    }

    #[test]
    fn responses_stream_maps_reasoning_from_done_only() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(ResponseReasoningItem {
                    id: Some("rs_789".into()),
                    summary: vec![],
                }),
            },
            ResponsesStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(ResponseReasoningItem {
                    id: Some("rs_789".into()),
                    summary: vec![ReasoningSummaryPart::SummaryText {
                        text: "Summary without deltas".into(),
                    }],
                }),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        assert!(
            !mapped
                .iter()
                .any(|e| matches!(e, LanguageModelCompletionEvent::Thinking { .. })),
            "OutputItemDone reasoning should not produce Thinking events"
        );
    }

    #[test]
    fn into_open_ai_interleaved_reasoning() {
        let tool_use_id = LanguageModelToolUseId::from("call-1");
        let tool_input = json!({"query": "foo"});
        let tool_arguments = serde_json::to_string(&tool_input).unwrap();
        let tool_use = LanguageModelToolUse {
            id: tool_use_id.clone(),
            name: Arc::from("search"),
            raw_input: tool_arguments.clone(),
            input: tool_input,
            is_input_complete: true,
            thought_signature: None,
        };
        let tool_result = LanguageModelToolResult {
            tool_use_id: tool_use_id,
            tool_name: Arc::from("search"),
            is_error: false,
            content: vec![LanguageModelToolResultContent::Text(Arc::from("result"))],
            output: None,
        };
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("search for something".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![
                        MessageContent::Thinking {
                            text: "I should search".into(),
                            signature: None,
                        },
                        MessageContent::Text("Searching now.".into()),
                        MessageContent::ToolUse(tool_use),
                    ],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolResult(tool_result)],
                    cache: false,
                    reasoning_details: None,
                },
            ],
            tools: vec![],
            tool_choice: None,
            stop: vec![],
            temperature: None,
            thinking_allowed: true,
            thinking_effort: None,
            speed: None,
        };

        let result = into_open_ai(request.clone(), "model", false, false, None, None, true);
        assert_eq!(
            serde_json::to_value(&result).unwrap()["messages"],
            json!([
                {"role": "user", "content": "search for something"},
                {
                    "role": "assistant",
                    "content": "Searching now.",
                    "tool_calls": [{"id": "call-1", "type": "function", "function": {"name": "search", "arguments": tool_arguments}}],
                    "reasoning_content": "I should search"
                },
                {"role": "tool", "content": "result", "tool_call_id": "call-1"}
            ])
        );

        let result = into_open_ai(request, "model", false, false, None, None, false);
        assert_eq!(
            serde_json::to_value(&result).unwrap()["messages"],
            json!([
                {"role": "user", "content": "search for something"},
                {
                    "role": "assistant",
                    "content": [
                        {"type": "text", "text": "I should search"},
                        {"type": "text", "text": "Searching now."}
                    ],
                    "tool_calls": [{"id": "call-1", "type": "function", "function": {"name": "search", "arguments": tool_arguments}}]
                },
                {"role": "tool", "content": "result", "tool_call_id": "call-1"}
            ])
        );
    }
}
