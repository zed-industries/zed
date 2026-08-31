use anyhow::{Result, anyhow};
use collections::HashMap;
use futures::{Stream, StreamExt};
use http_client::StatusCode;
use language_model_core::{
    CompactedContext, CompactionUpdate, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelCustomToolFormat, LanguageModelCustomToolGrammarSyntax, LanguageModelImage,
    LanguageModelProviderId, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelRequestToolInput, LanguageModelToolChoice, LanguageModelToolResultContent,
    LanguageModelToolUse, LanguageModelToolUseId, LanguageModelToolUseInput, MessageContent,
    ProviderErrorCategory, Role, StopReason, TokenUsage, provider_name_for_id,
    util::{fix_streamed_json, parse_tool_arguments},
};
use std::pin::Pin;
use std::sync::Arc;

use crate::responses::{
    ContextManagement, Request as ResponseRequest, ResponseCustomToolCallItem,
    ResponseCustomToolCallOutputItem, ResponseError, ResponseFunctionCallItem,
    ResponseFunctionCallOutputContent, ResponseFunctionCallOutputItem, ResponseIncludable,
    ResponseInputContent, ResponseInputItem, ResponseMessageItem, ResponseOutputItem,
    ResponseOutputMessage, ResponseReasoningInputItem, ResponseReasoningItem,
    ResponseReasoningSummaryPart, ResponseSummary as ResponsesSummary,
    ResponseUsage as ResponsesUsage, StreamEvent as ResponsesStreamEvent,
    provider_compaction_items, provider_compaction_state_from_items,
};
use crate::{
    FunctionContent, FunctionDefinition, ImageUrl, MessagePart, ReasoningEffort,
    ResponseStreamEvent, ServiceTier, ToolCall, ToolCallContent,
};

const RESPONSE_MESSAGE_PHASE_COMMENTARY: &str = "commentary";
const RESPONSE_MESSAGE_PHASE_FINAL_ANSWER: &str = "final_answer";

/// Translates the request's `Speed` into the corresponding OpenAI service tier.
/// Only `Fast` produces a value; `Standard` leaves the field unset so that the
/// project's default tier applies.
fn service_tier_for(speed: Option<language_model_core::Speed>) -> Option<ServiceTier> {
    match speed? {
        language_model_core::Speed::Fast => Some(ServiceTier::Priority),
        language_model_core::Speed::Standard => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChatCompletionMaxTokensParameter {
    MaxCompletionTokens,
    MaxTokens,
}

pub fn into_open_ai(
    request: LanguageModelRequest,
    model_id: &str,
    supports_parallel_tool_calls: bool,
    supports_prompt_cache_key: bool,
    max_output_tokens: Option<u64>,
    max_tokens_parameter: ChatCompletionMaxTokensParameter,
    reasoning_effort: Option<ReasoningEffort>,
    interleaved_reasoning: bool,
) -> Result<crate::Request> {
    if request
        .tools
        .iter()
        .any(|tool| matches!(tool.input, LanguageModelRequestToolInput::Custom { .. }))
    {
        return Err(anyhow!(
            "OpenAI Chat Completions does not support custom tools; use Responses API instead"
        ));
    }

    let stream = !model_id.starts_with("o1-");
    let service_tier = service_tier_for(request.speed);

    let mut messages = Vec::new();
    let mut current_reasoning: Option<String> = None;
    for message in request.messages {
        let reasoning_details = interleaved_reasoning
            .then(|| message.reasoning_details.clone())
            .flatten();
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
                            reasoning_details.clone(),
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
                MessageContent::RedactedThinking(_) | MessageContent::Compaction(_) => {}
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
                        reasoning_details.clone(),
                    );
                }
                MessageContent::ToolUse(tool_use) => {
                    let LanguageModelToolUseInput::Json(input) = &tool_use.input else {
                        return Err(anyhow!(
                            "OpenAI Chat Completions cannot replay custom tool call `{}`",
                            tool_use.name
                        ));
                    };
                    let tool_call = ToolCall {
                        id: tool_use.id.to_string(),
                        content: ToolCallContent::Function {
                            function: FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(input).unwrap_or_default(),
                                thought_signature: interleaved_reasoning
                                    .then(|| tool_use.thought_signature.clone())
                                    .flatten(),
                            },
                        },
                    };

                    if let Some(crate::RequestMessage::Assistant {
                        tool_calls,
                        reasoning_details: existing_reasoning_details,
                        ..
                    }) = messages.last_mut()
                        && existing_reasoning_details == &reasoning_details
                    {
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(crate::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                            reasoning_content: current_reasoning.take(),
                            reasoning_details: reasoning_details.clone(),
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

    Ok(crate::Request {
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
        max_completion_tokens: match max_tokens_parameter {
            ChatCompletionMaxTokensParameter::MaxCompletionTokens => max_output_tokens,
            ChatCompletionMaxTokensParameter::MaxTokens => None,
        },
        max_tokens: match max_tokens_parameter {
            ChatCompletionMaxTokensParameter::MaxCompletionTokens => None,
            ChatCompletionMaxTokensParameter::MaxTokens => max_output_tokens,
        },
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
            .map(|tool| match tool.input {
                LanguageModelRequestToolInput::Function { input_schema, .. } => {
                    Ok(crate::ToolDefinition::Function {
                        function: FunctionDefinition {
                            name: tool.name,
                            description: Some(tool.description),
                            parameters: Some(input_schema),
                        },
                    })
                }
                LanguageModelRequestToolInput::Custom { .. } => Err(anyhow!(
                    "OpenAI Chat Completions does not support custom tools; use Responses API instead"
                )),
            })
            .collect::<Result<_>>()?,
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => crate::ToolChoice::Auto,
            LanguageModelToolChoice::Any => crate::ToolChoice::Required,
            LanguageModelToolChoice::None => crate::ToolChoice::None,
        }),
        reasoning_effort,
        service_tier,
    })
}

/// `compaction_state_owner` identifies the backend this request will be sent
/// to for the purpose of replaying provider-native compaction state. Multiple
/// backends share this conversion, but their encrypted compaction items are
/// not interchangeable, so only state owned by this id is replayed; state
/// owned by any other backend falls back to replaying the full transcript.
pub fn into_open_ai_response(
    request: LanguageModelRequest,
    model_id: &str,
    supports_parallel_tool_calls: bool,
    supports_prompt_cache_key: bool,
    max_output_tokens: Option<u64>,
    default_reasoning_effort: Option<ReasoningEffort>,
    supports_none_reasoning_effort: bool,
    compaction_state_owner: &LanguageModelProviderId,
) -> Result<ResponseRequest> {
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
        thinking_allowed,
        thinking_effort,
        speed,
        compact_at_tokens,
    } = request;

    let service_tier = service_tier_for(speed);

    let mut provider_items = Vec::new();
    let mut input_items = Vec::new();
    let mut replayed_reasoning_item_indexes = HashMap::default();
    let mut tool_use_kinds_by_id = HashMap::default();
    let mut system_instructions = Vec::new();
    for (index, message) in messages.into_iter().enumerate() {
        // System messages go to the top-level `instructions` field rather
        // than the input item list. `instructions` is applied per request
        // (the Responses API documents it as a system message inserted into
        // the model's context), so the current system prompt survives when
        // replaying provider compaction state replaces the accumulated input
        // items below. This also matches the Codex backend, which rejects
        // system-role input items outright.
        if message.role == Role::System {
            for content in message.content {
                if let MessageContent::Text(text) = content
                    && !text.trim().is_empty()
                {
                    system_instructions.push(text);
                }
            }
            continue;
        }
        append_message_to_response_items(
            message,
            index,
            compaction_state_owner,
            &mut replayed_reasoning_item_indexes,
            &mut tool_use_kinds_by_id,
            &mut provider_items,
            &mut input_items,
        )?;
    }

    let tools: Vec<_> = tools
        .into_iter()
        .map(|tool| match tool.input {
            LanguageModelRequestToolInput::Function { input_schema, .. } => {
                crate::responses::ToolDefinition::Function {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(input_schema),
                    strict: None,
                }
            }
            LanguageModelRequestToolInput::Custom { format } => {
                crate::responses::ToolDefinition::Custom {
                    name: tool.name,
                    description: if tool.description.is_empty() {
                        None
                    } else {
                        Some(tool.description)
                    },
                    format: format.map(custom_tool_format_into_open_ai),
                }
            }
        })
        .collect();

    let default_reasoning_effort =
        default_reasoning_effort.filter(|effort| *effort != ReasoningEffort::None);
    let reasoning_effort = if thinking_allowed {
        thinking_effort
            .as_deref()
            .and_then(|effort| effort.parse::<ReasoningEffort>().ok())
            .filter(|effort| *effort != ReasoningEffort::None)
            .or(default_reasoning_effort)
    } else if supports_none_reasoning_effort {
        Some(ReasoningEffort::None)
    } else {
        None
    };

    let reasoning = reasoning_effort.map(|effort| crate::responses::ReasoningConfig {
        effort,
        summary: if effort == ReasoningEffort::None {
            None
        } else {
            Some(crate::responses::ReasoningSummaryMode::Auto)
        },
    });

    let include = if reasoning
        .as_ref()
        .is_some_and(|reasoning| reasoning.effort != ReasoningEffort::None)
        || input_items
            .iter()
            .any(|item| matches!(item, ResponseInputItem::Reasoning(_)))
    {
        vec![ResponseIncludable::ReasoningEncryptedContent]
    } else {
        Vec::new()
    };

    Ok(ResponseRequest {
        model: model_id.into(),
        instructions: if system_instructions.is_empty() {
            None
        } else {
            Some(system_instructions.join("\n\n"))
        },
        input: crate::responses::ResponseInput::new(provider_items, input_items),
        store: Some(false),
        include,
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
        reasoning,
        service_tier,
        context_management: compact_at_tokens
            .map(|compact_threshold| vec![ContextManagement::Compaction { compact_threshold }]),
    })
}

fn append_message_to_response_items(
    message: LanguageModelRequestMessage,
    index: usize,
    compaction_state_owner: &LanguageModelProviderId,
    replayed_reasoning_item_indexes: &mut HashMap<String, usize>,
    tool_use_kinds_by_id: &mut HashMap<LanguageModelToolUseId, ReplayToolKind>,
    provider_items: &mut Vec<serde_json::Value>,
    input_items: &mut Vec<ResponseInputItem>,
) -> Result<()> {
    let mut content_parts: Vec<ResponseInputContent> = Vec::new();

    let LanguageModelRequestMessage {
        role,
        content,
        reasoning_details,
        ..
    } = message;
    let phase = if role == Role::Assistant {
        response_message_phase_from_details(reasoning_details.as_deref())
    } else {
        None
    };

    if role == Role::Assistant {
        append_reasoning_details_to_response_items(
            reasoning_details.as_deref(),
            replayed_reasoning_item_indexes,
            input_items,
        );
    }

    for content in content {
        match content {
            MessageContent::Text(text) => {
                push_response_text_part(&role, text, &mut content_parts);
            }
            MessageContent::Thinking { .. } | MessageContent::RedactedThinking(_) => {}
            MessageContent::Compaction(CompactedContext::ProviderState(state)) => {
                // The canonical replacement window already encodes all
                // context retained at the compaction point, so everything
                // accumulated before it -- earlier input items, earlier parts
                // of this same message, and replay bookkeeping -- is
                // superseded and must not be resent. When a transcript
                // contains multiple compactions, each later window supersedes
                // the previous one, so the last compaction wins. State owned
                // by another backend yields `None`, in which case we fall
                // back to replaying the full transcript.
                if let Some(items) = provider_compaction_items(&state, compaction_state_owner)? {
                    content_parts.clear();
                    input_items.clear();
                    replayed_reasoning_item_indexes.clear();
                    tool_use_kinds_by_id.clear();
                    provider_items.clear();
                    provider_items.extend(items);
                }
            }
            MessageContent::Compaction(CompactedContext::Summary { .. }) => {}
            MessageContent::Image(image) => {
                push_response_image_part(&role, image, &mut content_parts);
            }
            MessageContent::ToolUse(tool_use) => {
                flush_response_parts(
                    &role,
                    index,
                    phase.as_deref(),
                    &mut content_parts,
                    input_items,
                );
                let call_id = tool_use.id.to_string();
                match tool_use.input {
                    LanguageModelToolUseInput::Json(_) => {
                        tool_use_kinds_by_id.insert(tool_use.id, ReplayToolKind::Function);
                        input_items.push(ResponseInputItem::FunctionCall(
                            ResponseFunctionCallItem {
                                call_id,
                                name: tool_use.name.to_string(),
                                arguments: tool_use.raw_input,
                            },
                        ));
                    }
                    LanguageModelToolUseInput::Text(_) => {
                        tool_use_kinds_by_id.insert(tool_use.id, ReplayToolKind::Custom);
                        input_items.push(ResponseInputItem::CustomToolCall(
                            ResponseCustomToolCallItem {
                                id: None,
                                call_id,
                                name: tool_use.name.to_string(),
                                input: tool_use.raw_input,
                            },
                        ));
                    }
                }
            }
            MessageContent::ToolResult(tool_result) => {
                flush_response_parts(
                    &role,
                    index,
                    phase.as_deref(),
                    &mut content_parts,
                    input_items,
                );
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
                match tool_use_kinds_by_id.get(&tool_result.tool_use_id) {
                    Some(ReplayToolKind::Custom) => {
                        input_items.push(ResponseInputItem::CustomToolCallOutput(
                            ResponseCustomToolCallOutputItem {
                                call_id: tool_result.tool_use_id.to_string(),
                                output,
                            },
                        ));
                    }
                    Some(ReplayToolKind::Function) | None => {
                        input_items.push(ResponseInputItem::FunctionCallOutput(
                            ResponseFunctionCallOutputItem {
                                call_id: tool_result.tool_use_id.to_string(),
                                output,
                            },
                        ));
                    }
                }
            }
        }
    }

    flush_response_parts(
        &role,
        index,
        phase.as_deref(),
        &mut content_parts,
        input_items,
    );
    Ok(())
}

#[derive(Clone, Copy)]
enum ReplayToolKind {
    Function,
    Custom,
}

fn custom_tool_format_into_open_ai(
    format: LanguageModelCustomToolFormat,
) -> crate::responses::CustomToolFormat {
    match format {
        LanguageModelCustomToolFormat::Text => crate::responses::CustomToolFormat::Text,
        LanguageModelCustomToolFormat::Grammar { syntax, definition } => {
            crate::responses::CustomToolFormat::Grammar {
                syntax: match syntax {
                    LanguageModelCustomToolGrammarSyntax::Lark => {
                        crate::responses::CustomToolGrammarSyntax::Lark
                    }
                    LanguageModelCustomToolGrammarSyntax::Regex => {
                        crate::responses::CustomToolGrammarSyntax::Regex
                    }
                },
                definition,
            }
        }
    }
}

fn append_reasoning_details_to_response_items(
    reasoning_details: Option<&serde_json::Value>,
    replayed_reasoning_item_indexes: &mut HashMap<String, usize>,
    input_items: &mut Vec<ResponseInputItem>,
) {
    let Some(reasoning_details) = reasoning_details else {
        return;
    };

    let Some(metadata) = response_message_metadata_from_details(reasoning_details) else {
        return;
    };

    for reasoning_item in metadata.reasoning_items {
        push_replayed_reasoning_item(reasoning_item, replayed_reasoning_item_indexes, input_items);
    }
}

fn push_replayed_reasoning_item(
    reasoning_item: ResponseReasoningInputItem,
    replayed_reasoning_item_indexes: &mut HashMap<String, usize>,
    input_items: &mut Vec<ResponseInputItem>,
) {
    if let Some(id) = reasoning_item.id.as_ref() {
        if let Some(index) = replayed_reasoning_item_indexes.get(id) {
            input_items[*index] = ResponseInputItem::Reasoning(reasoning_item);
            return;
        }

        replayed_reasoning_item_indexes.insert(id.clone(), input_items.len());
    }

    input_items.push(ResponseInputItem::Reasoning(reasoning_item));
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
    phase: Option<&str>,
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
        phase: match role {
            Role::Assistant => phase.map(str::to_string),
            Role::User | Role::System => None,
        },
    });

    input_items.push(item);
    parts.clear();
}

fn add_message_content_part(
    new_part: MessagePart,
    role: Role,
    messages: &mut Vec<crate::RequestMessage>,
    reasoning_details: Option<Arc<serde_json::Value>>,
) {
    match (role, messages.last_mut()) {
        (Role::User, Some(crate::RequestMessage::User { content }))
        | (Role::System, Some(crate::RequestMessage::System { content, .. })) => {
            content.push_part(new_part);
        }
        (
            Role::Assistant,
            Some(crate::RequestMessage::Assistant {
                content: Some(content),
                reasoning_details: existing_reasoning_details,
                ..
            }),
        ) if existing_reasoning_details == &reasoning_details => {
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
                    reasoning_details,
                },
                Role::System => crate::RequestMessage::System {
                    content: crate::MessageContent::from(vec![new_part]),
                },
            });
        }
    }
}

/// Accumulates structured reasoning metadata from compatible providers.
///
/// Array entries are matched by `index` and then `id`. Fragmented `text`,
/// `summary`, and `data` fields are concatenated while other non-null fields
/// replace their previous values.
///
/// # Examples
///
/// ```
/// use open_ai::completion::ReasoningDetailsAccumulator;
/// use serde_json::json;
///
/// let mut accumulator = ReasoningDetailsAccumulator::default();
/// accumulator.push(json!([{"index": 0, "text": "first "}]));
/// let details = accumulator
///     .push(json!([{"index": 0, "text": "second"}]))
///     .expect("non-empty reasoning details");
///
/// assert_eq!(details[0]["text"], "first second");
/// ```
#[derive(Debug, Default)]
pub struct ReasoningDetailsAccumulator {
    accumulated: Option<serde_json::Value>,
}

impl ReasoningDetailsAccumulator {
    /// Merges `chunk` and returns the updated metadata snapshot.
    ///
    /// `null` and empty arrays do not replace previously accumulated metadata
    /// and return `None`.
    pub fn push(&mut self, chunk: serde_json::Value) -> Option<serde_json::Value> {
        match chunk {
            serde_json::Value::Null => None,
            serde_json::Value::Array(chunks) if chunks.is_empty() => None,
            serde_json::Value::Array(chunks) => {
                let mut details = match self.accumulated.take() {
                    Some(serde_json::Value::Array(details)) => details,
                    _ => Vec::new(),
                };
                for chunk in chunks {
                    merge_reasoning_detail(&mut details, chunk);
                }
                let accumulated = serde_json::Value::Array(details);
                self.accumulated = Some(accumulated.clone());
                Some(accumulated)
            }
            chunk => {
                self.accumulated = Some(chunk.clone());
                Some(chunk)
            }
        }
    }
}

pub struct OpenAiEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
    reasoning_details: ReasoningDetailsAccumulator,
}

impl OpenAiEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
            reasoning_details: ReasoningDetailsAccumulator::default(),
        }
    }

    pub fn map_stream<E>(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseStreamEvent, E>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    where
        E: Into<LanguageModelCompletionError>,
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
        let mut events = Vec::new();
        if let Some(token_usage) = event.usage.as_ref().and_then(|usage| usage.token_usage()) {
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(token_usage)));
        }

        let Some(choice) = event.choices.first() else {
            return events;
        };

        if let Some(delta) = choice.delta.as_ref() {
            if let Some(reasoning_details) = delta.reasoning_details.clone()
                && let Some(reasoning_details) = self.reasoning_details.push(reasoning_details)
            {
                events.push(Ok(LanguageModelCompletionEvent::ReasoningDetails(
                    reasoning_details,
                )));
            }
            if let Some(reasoning) = delta.reasoning.clone() {
                push_thinking_event(reasoning, &mut events);
            }
            if let Some(reasoning_content) = delta.reasoning_content.clone() {
                push_thinking_event(reasoning_content, &mut events);
            }
            if let Some(content) = delta.content.clone() {
                if !content.is_empty() {
                    events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                }
            }

            if let Some(tool_calls) = delta.tool_calls.as_ref() {
                for tool_call in tool_calls {
                    let entry = self.tool_calls_by_index.entry(tool_call.index).or_default();

                    if let Some(tool_id) = tool_call.id.clone()
                        && !tool_id.is_empty()
                    {
                        entry.id = tool_id;
                    }

                    if let Some(function) = tool_call.function.as_ref() {
                        if let Some(name) = function.name.clone()
                            && !name.is_empty()
                        {
                            entry.name = name;
                        }

                        if let Some(arguments) = function.arguments.clone() {
                            entry.arguments.push_str(&arguments);
                        }

                        if let Some(thought_signature) = function.thought_signature.clone() {
                            entry.thought_signature = Some(thought_signature);
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
                                    input: LanguageModelToolUseInput::Json(input),
                                    raw_input: entry.arguments.clone(),
                                    thought_signature: entry.thought_signature.clone(),
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
                                input: LanguageModelToolUseInput::Json(input),
                                raw_input: tool_call.arguments.clone(),
                                thought_signature: tool_call.thought_signature.clone(),
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
            Some("length") => {
                events.push(Ok(LanguageModelCompletionEvent::Stop(
                    StopReason::MaxTokens,
                )));
            }
            Some(stop_reason) => {
                log::error!("Unexpected chat completion stop_reason: {stop_reason:?}",);
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            None => {}
        }

        events
    }
}

fn push_thinking_event(
    text: String,
    events: &mut Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
) {
    if !text.is_empty() {
        events.push(Ok(LanguageModelCompletionEvent::Thinking {
            text,
            signature: None,
        }));
    }
}

fn merge_reasoning_detail(details: &mut Vec<serde_json::Value>, chunk: serde_json::Value) {
    let index = chunk.get("index").and_then(serde_json::Value::as_u64);
    let target_index = index
        .and_then(|index| {
            details.iter().position(|detail| {
                detail.get("index").and_then(serde_json::Value::as_u64) == Some(index)
            })
        })
        .or_else(|| {
            let id = chunk.get("id").and_then(serde_json::Value::as_str)?;
            details
                .iter()
                .position(|detail| detail.get("id").and_then(serde_json::Value::as_str) == Some(id))
        });
    let Some(target_index) = target_index else {
        details.push(chunk);
        return;
    };
    let (Some(target), Some(chunk)) = (details[target_index].as_object_mut(), chunk.as_object())
    else {
        return;
    };
    for (key, value) in chunk {
        if matches!(key.as_str(), "text" | "summary" | "data")
            && let Some(fragment) = value.as_str()
            && let Some(existing) = target.get(key).and_then(serde_json::Value::as_str)
        {
            target.insert(
                key.clone(),
                serde_json::Value::String(format!("{existing}{fragment}")),
            );
        } else if !value.is_null() {
            target.insert(key.clone(), value.clone());
        }
    }
}

#[derive(Default)]
struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
    thought_signature: Option<String>,
}

pub struct OpenAiResponseEventMapper {
    /// The backend whose infrastructure produced this stream; stamped on any
    /// compaction state it emits so replay is limited to the same backend.
    compaction_state_owner: LanguageModelProviderId,
    function_calls_by_item: HashMap<String, PendingResponseFunctionCall>,
    custom_tool_calls_by_item: HashMap<String, PendingResponseCustomToolCall>,
    reasoning_items: Vec<ResponseReasoningInputItem>,
    current_reasoning_summary_part: Option<(String, usize)>,
    current_message_phase: Option<String>,
    pending_stop_reason: Option<StopReason>,
    pending_compaction_items: usize,
}

#[derive(Default)]
struct PendingResponseFunctionCall {
    call_id: String,
    name: Arc<str>,
    arguments: String,
}

struct PendingResponseCustomToolCall {
    call_id: String,
    name: Arc<str>,
    input: String,
}

impl OpenAiResponseEventMapper {
    pub fn new(compaction_state_owner: LanguageModelProviderId) -> Self {
        Self {
            compaction_state_owner,
            function_calls_by_item: HashMap::default(),
            custom_tool_calls_by_item: HashMap::default(),
            reasoning_items: Vec::new(),
            current_reasoning_summary_part: None,
            current_message_phase: None,
            pending_stop_reason: None,
            pending_compaction_items: 0,
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
                        events.extend(self.capture_message_phase(message));
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
                    ResponseOutputItem::CustomToolCall(custom_tool_call) => {
                        if let Some(item_id) = custom_tool_call.id.clone() {
                            let call_id = custom_tool_call
                                .call_id
                                .clone()
                                .or_else(|| custom_tool_call.id.clone())
                                .unwrap_or_else(|| item_id.clone());
                            let entry = PendingResponseCustomToolCall {
                                call_id,
                                name: Arc::<str>::from(
                                    custom_tool_call.name.clone().unwrap_or_default(),
                                ),
                                input: custom_tool_call.input.clone(),
                            };
                            self.custom_tool_calls_by_item.insert(item_id, entry);
                        }
                    }
                    ResponseOutputItem::Compaction(_) => {
                        self.pending_compaction_items += 1;
                        events.push(Ok(LanguageModelCompletionEvent::Compaction(
                            CompactionUpdate::Started,
                        )));
                    }
                    ResponseOutputItem::Reasoning(_) | ResponseOutputItem::Unknown => {}
                }
                events
            }
            ResponsesStreamEvent::ReasoningSummaryTextDelta {
                item_id,
                summary_index,
                delta,
                ..
            } => {
                if delta.is_empty() {
                    Vec::new()
                } else {
                    let mut events = self.begin_reasoning_summary_part(&item_id, summary_index);
                    events.push(Ok(LanguageModelCompletionEvent::Thinking {
                        text: delta,
                        signature: None,
                    }));
                    events
                }
            }
            ResponsesStreamEvent::ReasoningDelta { delta, .. } => {
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
            ResponsesStreamEvent::RefusalDelta { .. }
            | ResponsesStreamEvent::RefusalDone { .. } => {
                self.pending_stop_reason = Some(StopReason::Refusal);
                Vec::new()
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
                                input: LanguageModelToolUseInput::Json(input),
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
                                    input: LanguageModelToolUseInput::Json(input),
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
            ResponsesStreamEvent::CustomToolCallInputDelta { item_id, delta, .. } => {
                if let Some(entry) = self.custom_tool_calls_by_item.get_mut(&item_id) {
                    entry.input.push_str(&delta);
                    return vec![Ok(LanguageModelCompletionEvent::ToolUse(
                        LanguageModelToolUse {
                            id: LanguageModelToolUseId::from(entry.call_id.clone()),
                            name: entry.name.clone(),
                            is_input_complete: false,
                            input: LanguageModelToolUseInput::Text(entry.input.clone()),
                            raw_input: entry.input.clone(),
                            thought_signature: None,
                        },
                    ))];
                }
                Vec::new()
            }
            ResponsesStreamEvent::CustomToolCallInputDone { item_id, input, .. } => {
                if let Some(entry) = self.custom_tool_calls_by_item.get_mut(&item_id)
                    && !input.is_empty()
                {
                    entry.input = input;
                }
                self.finish_pending_custom_tool_call(&item_id, None)
            }
            ResponsesStreamEvent::Completed { response } => {
                self.handle_completion(response, StopReason::EndTurn)
            }
            ResponsesStreamEvent::Incomplete { response } => {
                let reason = response
                    .incomplete_details
                    .as_ref()
                    .and_then(|details| details.reason.as_deref());
                let mut stop_reason = match reason {
                    Some("max_tokens" | "max_output_tokens") => StopReason::MaxTokens,
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
                events.extend(self.capture_reasoning_items_from_output(&response.output));
                if response_output_contains_refusal(&response.output)
                    && !matches!(stop_reason, StopReason::MaxTokens)
                {
                    self.pending_stop_reason = Some(StopReason::Refusal);
                    stop_reason = StopReason::Refusal;
                }
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
            ResponsesStreamEvent::Failed { response } => match response.error.as_ref() {
                Some(error) => vec![Err(completion_error_from_response_error(
                    error,
                    provider_name_for_id(&self.compaction_state_owner),
                ))],
                None => vec![Err(LanguageModelCompletionError::from_provider_response(
                    provider_name_for_id(&self.compaction_state_owner),
                    None,
                    Some("response.failed".to_string()),
                    response_failure_message(&response),
                    None,
                    ProviderErrorCategory::Other,
                ))],
            },
            ResponsesStreamEvent::Error { error } => {
                vec![Err(completion_error_from_response_error(
                    &error,
                    provider_name_for_id(&self.compaction_state_owner),
                ))]
            }
            ResponsesStreamEvent::GenericError { error } => {
                let error = error.into_response_error();
                vec![Err(completion_error_from_response_error(
                    &error,
                    provider_name_for_id(&self.compaction_state_owner),
                ))]
            }
            ResponsesStreamEvent::ReasoningSummaryPartAdded {
                item_id,
                summary_index,
                ..
            } => self.begin_reasoning_summary_part(&item_id, summary_index),
            ResponsesStreamEvent::OutputItemDone { item, .. } => match item {
                ResponseOutputItem::Reasoning(reasoning) => self.capture_reasoning_item(&reasoning),
                ResponseOutputItem::Message(message) => self.capture_message_phase(&message),
                ResponseOutputItem::CustomToolCall(custom_tool_call) => {
                    if let Some(item_id) = custom_tool_call.id.as_ref() {
                        self.finish_pending_custom_tool_call(item_id, Some(&custom_tool_call))
                    } else {
                        Vec::new()
                    }
                }
                ResponseOutputItem::Compaction(compaction) => {
                    self.pending_compaction_items = self.pending_compaction_items.saturating_sub(1);
                    match serde_json::to_value(ResponseInputItem::Compaction(compaction))
                        .map_err(anyhow::Error::from)
                        .and_then(|item| {
                            provider_compaction_state_from_items(
                                self.compaction_state_owner.clone(),
                                vec![item],
                            )
                        }) {
                        Ok(state) => vec![Ok(LanguageModelCompletionEvent::Compaction(
                            CompactionUpdate::Finished(CompactedContext::ProviderState(state)),
                        ))],
                        Err(error) => vec![Err(LanguageModelCompletionError::Other(error))],
                    }
                }
                ResponseOutputItem::FunctionCall(_) | ResponseOutputItem::Unknown => Vec::new(),
            },
            ResponsesStreamEvent::OutputTextDone { .. }
            | ResponsesStreamEvent::ContentPartAdded { .. }
            | ResponsesStreamEvent::ContentPartDone { .. }
            | ResponsesStreamEvent::ReasoningSummaryTextDone { .. }
            | ResponsesStreamEvent::ReasoningSummaryPartDone { .. }
            | ResponsesStreamEvent::ReasoningDone { .. }
            | ResponsesStreamEvent::Created { .. }
            | ResponsesStreamEvent::InProgress { .. }
            | ResponsesStreamEvent::Unknown => Vec::new(),
        }
    }

    fn begin_reasoning_summary_part(
        &mut self,
        item_id: &str,
        summary_index: usize,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let part = (item_id.to_string(), summary_index);
        if self.current_reasoning_summary_part.as_ref() == Some(&part) {
            return Vec::new();
        }

        let separator = self.current_reasoning_summary_part.replace(part).is_some();
        if separator {
            vec![Ok(LanguageModelCompletionEvent::Thinking {
                text: "\n\n".to_string(),
                signature: None,
            })]
        } else {
            Vec::new()
        }
    }

    fn handle_completion(
        &mut self,
        response: ResponsesSummary,
        default_reason: StopReason,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        // A compaction item that was added but never done means the server
        // already pruned its context, but we never received the canonical
        // replacement window. Continuing as if the turn succeeded would
        // leave the conversation unable to continue coherently.
        if self.pending_compaction_items > 0 {
            return vec![Err(LanguageModelCompletionError::Other(anyhow!(
                "response completed with an unfinished compaction item"
            )))];
        }

        let mut events = Vec::new();

        events.extend(self.capture_reasoning_items_from_output(&response.output));

        if response_output_contains_refusal(&response.output) {
            self.pending_stop_reason = Some(StopReason::Refusal);
        }

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
            match item {
                ResponseOutputItem::FunctionCall(function_call) => {
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
                                    input: LanguageModelToolUseInput::Json(input),
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
                ResponseOutputItem::CustomToolCall(custom_tool_call) => {
                    events.extend(self.emit_custom_tool_call(custom_tool_call));
                }
                _ => {}
            }
        }
        events
    }

    fn emit_custom_tool_call(
        &mut self,
        custom_tool_call: &crate::responses::ResponseCustomToolCall,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let Some(call_id) = custom_tool_call
            .call_id
            .clone()
            .or_else(|| custom_tool_call.id.clone())
        else {
            log::error!(
                "Custom tool call item missing both call_id and id: {:?}",
                custom_tool_call
            );
            return Vec::new();
        };
        self.pending_stop_reason = Some(StopReason::ToolUse);
        let input = custom_tool_call.input.clone();
        vec![Ok(LanguageModelCompletionEvent::ToolUse(
            LanguageModelToolUse {
                id: LanguageModelToolUseId::from(call_id),
                name: Arc::from(custom_tool_call.name.clone().unwrap_or_default()),
                is_input_complete: true,
                input: LanguageModelToolUseInput::Text(input.clone()),
                raw_input: input,
                thought_signature: None,
            },
        ))]
    }

    fn finish_pending_custom_tool_call(
        &mut self,
        item_id: &str,
        fallback: Option<&crate::responses::ResponseCustomToolCall>,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let Some(mut entry) = self.custom_tool_calls_by_item.remove(item_id) else {
            return Vec::new();
        };
        if let Some(fallback) = fallback
            && !fallback.input.is_empty()
        {
            entry.input = fallback.input.clone();
        }
        self.pending_stop_reason = Some(StopReason::ToolUse);
        vec![Ok(LanguageModelCompletionEvent::ToolUse(
            LanguageModelToolUse {
                id: LanguageModelToolUseId::from(entry.call_id),
                name: entry.name,
                is_input_complete: true,
                input: LanguageModelToolUseInput::Text(entry.input.clone()),
                raw_input: entry.input,
                thought_signature: None,
            },
        ))]
    }

    fn capture_reasoning_items_from_output(
        &mut self,
        output: &[ResponseOutputItem],
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();
        for item in output {
            if let ResponseOutputItem::Reasoning(reasoning) = item {
                events.extend(self.capture_reasoning_item(reasoning));
            }
        }
        events
    }

    fn capture_message_phase(
        &mut self,
        message: &ResponseOutputMessage,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        self.current_message_phase = message
            .phase
            .as_deref()
            .and_then(normalize_response_message_phase)
            .map(str::to_string);

        if self.current_message_phase.is_none() && self.reasoning_items.is_empty() {
            return Vec::new();
        }

        self.emit_response_message_metadata()
    }

    fn capture_reasoning_item(
        &mut self,
        reasoning: &ResponseReasoningItem,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let reasoning_item = response_reasoning_input_item_from_output(reasoning);

        if self.reasoning_items.contains(&reasoning_item) {
            return Vec::new();
        }

        if let Some(id) = reasoning_item.id.as_ref()
            && let Some(existing_reasoning_item) = self
                .reasoning_items
                .iter_mut()
                .find(|existing_reasoning_item| existing_reasoning_item.id.as_ref() == Some(id))
        {
            *existing_reasoning_item = reasoning_item;
        } else {
            self.reasoning_items.push(reasoning_item);
        }

        self.emit_response_message_metadata()
    }

    fn emit_response_message_metadata(
        &self,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let details = serde_json::to_value(ResponseMessageMetadata {
            phase: self.current_message_phase.clone(),
            reasoning_items: self.reasoning_items.clone(),
        });

        match details {
            Ok(details) => vec![Ok(LanguageModelCompletionEvent::ReasoningDetails(details))],
            Err(error) => vec![Err(LanguageModelCompletionError::Other(anyhow!(error)))],
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ResponseMessageMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    phase: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    reasoning_items: Vec<ResponseReasoningInputItem>,
}

fn response_message_metadata_from_details(
    details: &serde_json::Value,
) -> Option<ResponseMessageMetadata> {
    serde_json::from_value::<ResponseMessageMetadata>(details.clone()).ok()
}

fn response_message_phase_from_details(details: Option<&serde_json::Value>) -> Option<String> {
    let metadata = response_message_metadata_from_details(details?)?;
    metadata
        .phase
        .as_deref()
        .and_then(normalize_response_message_phase)
        .map(str::to_string)
}

fn normalize_response_message_phase(phase: &str) -> Option<&'static str> {
    match phase {
        RESPONSE_MESSAGE_PHASE_COMMENTARY => Some(RESPONSE_MESSAGE_PHASE_COMMENTARY),
        RESPONSE_MESSAGE_PHASE_FINAL_ANSWER => Some(RESPONSE_MESSAGE_PHASE_FINAL_ANSWER),
        _ => None,
    }
}

fn response_failure_message(response: &ResponsesSummary) -> String {
    if let Some(error) = response.error.as_ref() {
        return response_error_message(error);
    }

    response
        .status
        .as_deref()
        .map(|status| format!("response.{status}"))
        .unwrap_or_else(|| "response.failed".to_string())
}

fn completion_error_from_response_error(
    error: &ResponseError,
    provider: language_model_core::LanguageModelProviderName,
) -> LanguageModelCompletionError {
    let category = response_error_category(error.code.as_deref(), None, &error.message);
    LanguageModelCompletionError::from_provider_response(
        provider,
        None,
        error.code.clone(),
        error.message.clone(),
        None,
        category,
    )
}

pub(crate) fn response_error_category(
    code: Option<&str>,
    status: Option<StatusCode>,
    message: &str,
) -> ProviderErrorCategory {
    match code {
        Some("context_length_exceeded" | "request_too_large") => {
            ProviderErrorCategory::PromptTooLarge { tokens: None }
        }
        Some("invalid_encrypted_content") => ProviderErrorCategory::InvalidEncryptedContent,
        Some("invalid_request_error") => ProviderErrorCategory::InvalidRequest,
        Some("authentication_error") => ProviderErrorCategory::Authentication,
        Some(
            "billing_error"
            | "payment_required_error"
            | "credit_balance_exhausted"
            | "insufficient_quota"
            | "organization_spend_limit_exceeded"
            | "project_spend_limit_exceeded"
            | "organization_usage_limit_exceeded",
        ) => ProviderErrorCategory::PaymentRequired,
        Some("permission_error") => ProviderErrorCategory::Permission,
        Some("cyber_policy" | "invalid_prompt") => ProviderErrorCategory::ContentPolicy,
        Some("not_found_error") => ProviderErrorCategory::EndpointNotFound,
        Some("conflict_error") => ProviderErrorCategory::Conflict,
        Some("rate_limit_error" | "rate_limit_exceeded") => ProviderErrorCategory::RateLimit,
        Some("timeout_error" | "request_timed_out") => ProviderErrorCategory::Timeout,
        Some("api_error" | "internal_server_error" | "server_error") => {
            ProviderErrorCategory::InternalServer
        }
        Some("overloaded_error" | "server_is_overloaded" | "slow_down") => {
            ProviderErrorCategory::Overloaded
        }
        Some(_) | None => status
            .map(|status| ProviderErrorCategory::from_http_status(status, message))
            .unwrap_or(ProviderErrorCategory::Other),
    }
}

fn response_error_message(error: &ResponseError) -> String {
    let code = error.code.as_deref().filter(|code| !code.trim().is_empty());
    let message = error.message.trim();

    match (code, message.is_empty()) {
        (Some(code), false) => format!("{code}: {message}"),
        (Some(code), true) => code.to_string(),
        (None, false) => message.to_string(),
        (None, true) => "response error".to_string(),
    }
}

fn response_output_contains_refusal(output: &[ResponseOutputItem]) -> bool {
    output.iter().any(|item| {
        if let ResponseOutputItem::Message(message) = item {
            message.content.iter().any(response_content_is_refusal)
        } else {
            false
        }
    })
}

fn response_content_is_refusal(content: &serde_json::Value) -> bool {
    let content_type = content
        .get("type")
        .and_then(|content_type| content_type.as_str());
    let refusal = content
        .get("refusal")
        .and_then(|refusal| refusal.as_str())
        .unwrap_or_default();

    content_type == Some("refusal") || !refusal.is_empty()
}

pub fn token_usage_from_response_usage(usage: &ResponsesUsage) -> TokenUsage {
    let cache_read_input_tokens = usage.input_tokens_details.cached_tokens;

    TokenUsage {
        input_tokens: usage
            .input_tokens
            .unwrap_or_default()
            .saturating_sub(cache_read_input_tokens),
        output_tokens: usage.output_tokens.unwrap_or_default(),
        cache_creation_input_tokens: 0,
        cache_read_input_tokens,
    }
}

fn response_reasoning_input_item_from_output(
    reasoning: &ResponseReasoningItem,
) -> ResponseReasoningInputItem {
    let encrypted_content = reasoning.encrypted_content.clone();

    let summary = reasoning
        .summary
        .iter()
        .filter_map(|part| match part {
            crate::responses::ReasoningSummaryPart::SummaryText { text } => {
                Some(ResponseReasoningSummaryPart::SummaryText { text: text.clone() })
            }
            crate::responses::ReasoningSummaryPart::Unknown => None,
        })
        .collect();

    ResponseReasoningInputItem {
        id: reasoning.id.clone(),
        summary,
        content: reasoning.content.clone(),
        encrypted_content,
        status: reasoning.status.clone(),
    }
}

#[cfg(test)]
mod tests {
    use crate::responses::{
        ReasoningSummaryPart, ResponseCustomToolCall, ResponseError, ResponseFunctionToolCall,
        ResponseIncompleteDetails, ResponseInputItem, ResponseInputTokensDetails,
        ResponseOutputItem, ResponseOutputMessage, ResponseReasoningItem, ResponseSummary,
        ResponseUsage, StreamEvent as ResponsesStreamEvent, ToolDefinition,
    };
    use futures::{StreamExt, executor::block_on};
    use language_model_core::{
        LanguageModelCustomToolFormat, LanguageModelCustomToolGrammarSyntax, LanguageModelImage,
        LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelRequestToolInput,
        LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolUse,
        LanguageModelToolUseId, LanguageModelToolUseInput, OPEN_AI_PROVIDER_ID,
        OPEN_AI_PROVIDER_NAME, ProviderErrorCategory, SharedString, Speed,
    };
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::{
        ChoiceDelta, FunctionChunk, ResponseMessageDelta, ResponseStreamEvent, ToolCallChunk, Usage,
    };

    fn map_response_events(events: Vec<ResponsesStreamEvent>) -> Vec<LanguageModelCompletionEvent> {
        block_on(async {
            OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID)
                .map_stream(Box::pin(futures::stream::iter(events.into_iter().map(Ok))))
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .map(Result::unwrap)
                .collect()
        })
    }

    fn map_completion_events(
        events: Vec<ResponseStreamEvent>,
    ) -> Vec<LanguageModelCompletionEvent> {
        let mut mapper = OpenAiEventMapper::new();
        let mut all_events = Vec::new();
        for event in events {
            all_events.extend(mapper.map_event(event));
        }
        all_events.into_iter().filter_map(|e| e.ok()).collect()
    }

    fn response_item_message(id: &str) -> ResponseOutputItem {
        ResponseOutputItem::Message(ResponseOutputMessage {
            id: Some(id.to_string()),
            role: Some("assistant".to_string()),
            status: Some("in_progress".to_string()),
            content: vec![],
            phase: None,
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

    fn response_item_custom_tool_call(id: &str, input: &str) -> ResponseOutputItem {
        ResponseOutputItem::CustomToolCall(ResponseCustomToolCall {
            id: Some(id.to_string()),
            status: Some("in_progress".to_string()),
            name: Some("apply_patch".to_string()),
            call_id: Some("call_abc".to_string()),
            input: input.to_string(),
        })
    }

    fn response_reasoning_item(
        id: &str,
        summary: Vec<ReasoningSummaryPart>,
        encrypted_content: Option<&str>,
        status: Option<String>,
    ) -> ResponseReasoningItem {
        ResponseReasoningItem {
            id: Some(id.to_string()),
            summary,
            content: Vec::new(),
            encrypted_content: encrypted_content.map(str::to_string),
            status,
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
                        input_tokens_details: ResponseInputTokensDetails { cached_tokens: 2 },
                        output_tokens: Some(3),
                        total_tokens: Some(8),
                        ..Default::default()
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
                input_tokens: 3,
                output_tokens: 3,
                cache_read_input_tokens: 2,
                ..
            })
        ));
        assert!(matches!(
            mapped[3],
            LanguageModelCompletionEvent::Stop(StopReason::EndTurn)
        ));
    }

    #[test]
    fn responses_stream_maps_mantle_reasoning_delta() {
        let event = serde_json::from_value::<ResponsesStreamEvent>(json!({
            "type": "response.reasoning.delta",
            "delta": "checking the contract terms"
        }))
        .unwrap();

        let mapped = map_response_events(vec![event]);
        assert!(matches!(
            &mapped[0],
            LanguageModelCompletionEvent::Thinking { text, signature: None }
                if text == "checking the contract terms"
        ));
    }

    #[test]
    fn response_usage_deserializes_cached_tokens() -> Result<()> {
        let usage: ResponseUsage = serde_json::from_value(json!({
            "input_tokens": 5,
            "input_tokens_details": {
                "cached_tokens": 2,
            },
            "output_tokens": 3,
            "output_tokens_details": {
                "reasoning_tokens": 1,
            },
            "total_tokens": 8,
        }))?;

        assert_eq!(usage.output_tokens_details.reasoning_tokens, 1);
        assert_eq!(
            token_usage_from_response_usage(&usage),
            TokenUsage {
                input_tokens: 3,
                output_tokens: 3,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 2,
            }
        );

        Ok(())
    }

    #[test]
    fn responses_custom_tool_wire_types_round_trip() -> Result<()> {
        let tool_json = json!({
            "type": "custom",
            "name": "apply_patch",
            "description": "Apply a patch",
            "format": {
                "type": "grammar",
                "syntax": "lark",
                "definition": "start: /.+/"
            }
        });
        let tool: ToolDefinition = serde_json::from_value(tool_json.clone())?;
        assert_eq!(serde_json::to_value(tool)?, tool_json);

        let text_tool_json = json!({
            "type": "custom",
            "name": "write_text",
            "format": { "type": "text" }
        });
        let text_tool: ToolDefinition = serde_json::from_value(text_tool_json.clone())?;
        assert_eq!(serde_json::to_value(text_tool)?, text_tool_json);

        let input_json = json!({
            "type": "custom_tool_call",
            "id": "ctc_1",
            "call_id": "call_abc",
            "name": "apply_patch",
            "input": "*** Begin Patch\n*** End Patch"
        });
        let input: ResponseInputItem = serde_json::from_value(input_json.clone())?;
        assert_eq!(serde_json::to_value(input)?, input_json);

        let output_json = json!({
            "type": "custom_tool_call_output",
            "call_id": "call_abc",
            "output": "ok"
        });
        let output: ResponseInputItem = serde_json::from_value(output_json.clone())?;
        assert_eq!(serde_json::to_value(output)?, output_json);

        let output_item_json = json!({
            "id": "ctc_1",
            "type": "custom_tool_call",
            "status": "completed",
            "call_id": "call_abc",
            "name": "apply_patch",
            "input": "*** Begin Patch\n*** End Patch"
        });
        let output_item: ResponseOutputItem = serde_json::from_value(output_item_json.clone())?;
        assert_eq!(serde_json::to_value(output_item)?, output_item_json);

        let delta_json = json!({
            "type": "response.custom_tool_call_input.delta",
            "output_index": 0,
            "item_id": "ctc_1",
            "sequence_number": 5,
            "delta": "chunk"
        });
        let delta: ResponsesStreamEvent = serde_json::from_value(delta_json)?;
        assert!(matches!(
            delta,
            ResponsesStreamEvent::CustomToolCallInputDelta {
                output_index: 0,
                sequence_number: Some(5),
                ref item_id,
                ref delta,
            } if item_id == "ctc_1" && delta == "chunk"
        ));

        let done_json = json!({
            "type": "response.custom_tool_call_input.done",
            "output_index": 0,
            "item_id": "ctc_1",
            "sequence_number": 6,
            "input": "full text"
        });
        let done: ResponsesStreamEvent = serde_json::from_value(done_json)?;
        assert!(matches!(
            done,
            ResponsesStreamEvent::CustomToolCallInputDone {
                output_index: 0,
                sequence_number: Some(6),
                ref item_id,
                ref input,
            } if item_id == "ctc_1" && input == "full text"
        ));

        Ok(())
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
            input: LanguageModelToolUseInput::Json(tool_input),
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
            tools: vec![LanguageModelRequestTool::function(
                "get_weather".into(),
                "Fetches the weather".into(),
                json!({ "type": "object" }),
                false,
            )],
            tool_choice: Some(LanguageModelToolChoice::Any),
            stop: vec!["<STOP>".into()],
            temperature: None,
            thinking_allowed: true,
            thinking_effort: Some("high".into()),
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "custom-model",
            true,
            true,
            Some(2048),
            Some(ReasoningEffort::Low),
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        let serialized = serde_json::to_value(&response).unwrap();
        let expected = json!({
            "model": "custom-model",
            "instructions": "System context",
            "input": [
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
            "store": false,
            "include": ["reasoning.encrypted_content"],
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
            "reasoning": { "effort": "high", "summary": "auto" }
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn responses_stream_maps_custom_tool_input() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: response_item_custom_tool_call("ctc_1", ""),
            },
            ResponsesStreamEvent::CustomToolCallInputDelta {
                item_id: "ctc_1".into(),
                output_index: 0,
                delta: "*** Begin".into(),
                sequence_number: Some(1),
            },
            ResponsesStreamEvent::CustomToolCallInputDelta {
                item_id: "ctc_1".into(),
                output_index: 0,
                delta: " Patch".into(),
                sequence_number: Some(2),
            },
            ResponsesStreamEvent::CustomToolCallInputDone {
                item_id: "ctc_1".into(),
                output_index: 0,
                input: "*** Begin Patch".into(),
                sequence_number: Some(3),
            },
            ResponsesStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: response_item_custom_tool_call("ctc_1", "*** Begin Patch"),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        assert_eq!(
            mapped,
            vec![
                LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                    id: LanguageModelToolUseId::from("call_abc"),
                    name: Arc::from("apply_patch"),
                    raw_input: "*** Begin".into(),
                    input: LanguageModelToolUseInput::Text("*** Begin".into()),
                    is_input_complete: false,
                    thought_signature: None,
                }),
                LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                    id: LanguageModelToolUseId::from("call_abc"),
                    name: Arc::from("apply_patch"),
                    raw_input: "*** Begin Patch".into(),
                    input: LanguageModelToolUseInput::Text("*** Begin Patch".into()),
                    is_input_complete: false,
                    thought_signature: None,
                }),
                LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                    id: LanguageModelToolUseId::from("call_abc"),
                    name: Arc::from("apply_patch"),
                    raw_input: "*** Begin Patch".into(),
                    input: LanguageModelToolUseInput::Text("*** Begin Patch".into()),
                    is_input_complete: true,
                    thought_signature: None,
                }),
                LanguageModelCompletionEvent::Stop(StopReason::ToolUse),
            ]
        );
    }

    #[test]
    fn into_open_ai_response_replays_custom_tool_calls() {
        let tool_call_id = LanguageModelToolUseId::from("call_abc");
        let raw_input = "*** Begin Patch\n*** End Patch".to_string();
        let tool_use = LanguageModelToolUse {
            id: tool_call_id.clone(),
            name: Arc::from("apply_patch"),
            raw_input: raw_input.clone(),
            input: LanguageModelToolUseInput::Text(raw_input.clone()),
            is_input_complete: true,
            thought_signature: None,
        };
        let tool_result = LanguageModelToolResult {
            tool_use_id: tool_call_id,
            tool_name: Arc::from("apply_patch"),
            is_error: false,
            content: vec![LanguageModelToolResultContent::Text(Arc::from("ok"))],
            output: None,
        };

        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![
                    MessageContent::ToolUse(tool_use),
                    MessageContent::ToolResult(tool_result),
                ],
                cache: false,
                reasoning_details: None,
            }],
            tools: vec![LanguageModelRequestTool {
                name: "apply_patch".into(),
                description: "Apply a patch".into(),
                input: LanguageModelRequestToolInput::Custom {
                    format: Some(LanguageModelCustomToolFormat::Grammar {
                        syntax: LanguageModelCustomToolGrammarSyntax::Lark,
                        definition: "start: /.+/".into(),
                    }),
                },
            }],
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: false,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "custom-model",
            false,
            false,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();
        let serialized = serde_json::to_value(response).unwrap();
        assert_eq!(
            serialized,
            json!({
                "model": "custom-model",
                "input": [
                    {
                        "type": "custom_tool_call",
                        "call_id": "call_abc",
                        "name": "apply_patch",
                        "input": raw_input
                    },
                    {
                        "type": "custom_tool_call_output",
                        "call_id": "call_abc",
                        "output": "ok"
                    }
                ],
                "store": false,
                "stream": true,
                "parallel_tool_calls": false,
                "tools": [
                    {
                        "type": "custom",
                        "name": "apply_patch",
                        "description": "Apply a patch",
                        "format": {
                            "type": "grammar",
                            "syntax": "lark",
                            "definition": "start: /.+/"
                        }
                    }
                ]
            })
        );
    }

    #[test]
    fn into_open_ai_response_replays_encrypted_reasoning_details() {
        let tool_call_id = LanguageModelToolUseId::from("call-42");
        let tool_arguments = "{\"city\":\"Boston\"}".to_string();
        let tool_use = LanguageModelToolUse {
            id: tool_call_id,
            name: Arc::from("get_weather"),
            raw_input: tool_arguments.clone(),
            input: LanguageModelToolUseInput::Json(json!({ "city": "Boston" })),
            is_input_complete: true,
            thought_signature: None,
        };

        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::ToolUse(tool_use)],
                cache: false,
                reasoning_details: Some(Arc::new(json!({
                    "reasoning_items": [
                        {
                            "id": "rs_123",
                            "summary": [
                                {
                                    "type": "summary_text",
                                    "text": "Checked what information is needed."
                                }
                            ],
                            "content": [
                                {
                                    "type": "reasoning_text",
                                    "text": "Internal reasoning text."
                                }
                            ],
                            "encrypted_content": "ENC",
                            "status": "completed",
                        }
                    ]
                }))),
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: false,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "gpt-5",
            true,
            true,
            None,
            Some(ReasoningEffort::Low),
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(
            serialized["input"],
            json!([
                {
                    "type": "reasoning",
                    "id": "rs_123",
                    "summary": [
                        {
                            "type": "summary_text",
                            "text": "Checked what information is needed."
                        }
                    ],
                    "content": [
                        {
                            "type": "reasoning_text",
                            "text": "Internal reasoning text."
                        }
                    ],
                    "encrypted_content": "ENC",
                    "status": "completed"
                },
                {
                    "type": "function_call",
                    "call_id": "call-42",
                    "name": "get_weather",
                    "arguments": tool_arguments
                }
            ])
        );
        assert_eq!(
            serialized["include"],
            json!(["reasoning.encrypted_content"])
        );
        assert_eq!(serialized.get("reasoning"), None);
    }

    #[test]
    fn into_open_ai_response_replays_reasoning_without_encrypted_content() {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::Text("Done.".into())],
                cache: false,
                reasoning_details: Some(Arc::new(json!({
                    "reasoning_items": [
                        {
                            "id": "rs_123",
                            "summary": [],
                            "status": "completed"
                        },
                        {
                            "id": "rs_456",
                            "summary": [],
                            "encrypted_content": "",
                            "status": "completed"
                        }
                    ]
                }))),
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: false,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "custom-model",
            false,
            false,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();
        let serialized = serde_json::to_value(&response).unwrap();

        assert_eq!(
            serialized["input"],
            json!([
                {
                    "type": "reasoning",
                    "id": "rs_123",
                    "summary": [],
                    "status": "completed"
                },
                {
                    "type": "reasoning",
                    "id": "rs_456",
                    "summary": [],
                    "encrypted_content": "",
                    "status": "completed"
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "Done.",
                            "annotations": []
                        }
                    ]
                }
            ])
        );
    }

    #[test]
    fn into_open_ai_response_omits_reasoning_when_thinking_is_disabled_and_none_is_unsupported() {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".into())],
                cache: false,
                reasoning_details: None,
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: false,
            thinking_effort: Some("high".into()),
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "gpt-5",
            true,
            true,
            None,
            Some(ReasoningEffort::Medium),
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(serialized.get("reasoning"), None);
    }

    /// `Speed::Fast` should translate to `service_tier: "priority"` on the
    /// outgoing Responses request, while `Standard` / `None` should leave the
    /// field unset so the project's default tier wins.
    #[test]
    fn into_open_ai_response_sets_service_tier_for_fast_speed() -> Result<()> {
        for (speed, expected) in [
            (None, None),
            (Some(Speed::Standard), None),
            (Some(Speed::Fast), Some("priority")),
        ] {
            let request = LanguageModelRequest {
                thread_id: None,
                prompt_id: None,
                intent: None,
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hello".into())],
                    cache: false,
                    reasoning_details: None,
                }],
                tools: Vec::new(),
                tool_choice: None,
                stop: Vec::new(),
                temperature: None,
                thinking_allowed: false,
                thinking_effort: None,
                speed,
                compact_at_tokens: None,
            };

            let response = into_open_ai_response(
                request,
                "gpt-5.4",
                true,
                true,
                None,
                None,
                true,
                &OPEN_AI_PROVIDER_ID,
            )
            .unwrap();

            let serialized = serde_json::to_value(&response)?;
            assert_eq!(
                serialized
                    .get("service_tier")
                    .and_then(|value| value.as_str()),
                expected,
                "speed = {speed:?} should produce service_tier = {expected:?}",
            );
        }
        Ok(())
    }

    /// Same as above but for the Chat Completions code path.
    #[test]
    fn into_open_ai_sets_service_tier_for_fast_speed() -> Result<()> {
        for (speed, expected) in [
            (None, None),
            (Some(Speed::Standard), None),
            (Some(Speed::Fast), Some("priority")),
        ] {
            let request = LanguageModelRequest {
                thread_id: None,
                prompt_id: None,
                intent: None,
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hello".into())],
                    cache: false,
                    reasoning_details: None,
                }],
                tools: Vec::new(),
                tool_choice: None,
                stop: Vec::new(),
                temperature: None,
                thinking_allowed: false,
                thinking_effort: None,
                speed,
                compact_at_tokens: None,
            };

            let chat = into_open_ai(
                request,
                "gpt-5.4",
                true,
                true,
                None,
                ChatCompletionMaxTokensParameter::MaxCompletionTokens,
                None,
                false,
            )?;

            let serialized = serde_json::to_value(&chat)?;
            assert_eq!(
                serialized
                    .get("service_tier")
                    .and_then(|value| value.as_str()),
                expected,
                "speed = {speed:?} should produce service_tier = {expected:?}",
            );
        }
        Ok(())
    }

    #[test]
    fn into_open_ai_can_send_max_tokens_parameter() -> Result<()> {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".into())],
                cache: false,
                reasoning_details: None,
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: false,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let chat = into_open_ai(
            request,
            "compatible-model",
            false,
            false,
            Some(4096),
            ChatCompletionMaxTokensParameter::MaxTokens,
            None,
            false,
        )?;

        let serialized = serde_json::to_value(&chat)?;
        assert_eq!(serialized.get("max_completion_tokens"), None);
        assert_eq!(serialized["max_tokens"], json!(4096));
        Ok(())
    }

    #[test]
    fn into_open_ai_response_sends_none_reasoning_when_thinking_is_disabled() -> Result<()> {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".into())],
                cache: false,
                reasoning_details: None,
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: false,
            thinking_effort: Some("high".into()),
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.1",
            true,
            true,
            None,
            Some(ReasoningEffort::Medium),
            true,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        let serialized = serde_json::to_value(&response)?;
        assert_eq!(serialized["reasoning"], json!({ "effort": "none" }));
        assert_eq!(serialized.get("include"), None);

        Ok(())
    }

    #[test]
    fn into_open_ai_response_uses_default_effort_when_selected_effort_is_none() -> Result<()> {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".into())],
                cache: false,
                reasoning_details: None,
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: true,
            thinking_effort: Some("none".into()),
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.1",
            true,
            true,
            None,
            Some(ReasoningEffort::Medium),
            true,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        let serialized = serde_json::to_value(&response)?;
        assert_eq!(
            serialized["reasoning"],
            json!({ "effort": "medium", "summary": "auto" })
        );

        Ok(())
    }

    #[test]
    fn into_open_ai_response_replays_assistant_phase() {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::Text("Done.".into())],
                cache: false,
                reasoning_details: Some(Arc::new(json!({
                    "phase": "final_answer",
                    "reasoning_items": [
                        {
                            "id": "rs_123",
                            "summary": [],
                            "encrypted_content": "ENC",
                            "status": "completed"
                        }
                    ]
                }))),
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: true,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.3-codex",
            true,
            true,
            None,
            Some(ReasoningEffort::Medium),
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(
            serialized["input"],
            json!([
                {
                    "type": "reasoning",
                    "id": "rs_123",
                    "summary": [],
                    "encrypted_content": "ENC",
                    "status": "completed"
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "content": [
                        { "type": "output_text", "text": "Done.", "annotations": [] }
                    ],
                    "phase": "final_answer"
                }
            ])
        );
    }

    #[test]
    fn into_open_ai_response_deduplicates_replayed_reasoning_items() {
        let first_reasoning_details = json!({
            "phase": "final_answer",
            "reasoning_items": [
                {
                    "id": "rs_123",
                    "summary": [],
                    "encrypted_content": "ENC_OLD",
                    "status": "in_progress"
                }
            ]
        });
        let second_reasoning_details = json!({
            "phase": "final_answer",
            "reasoning_items": [
                {
                    "id": "rs_123",
                    "summary": [
                        {
                            "type": "summary_text",
                            "text": "Later metadata has the complete summary."
                        }
                    ],
                    "encrypted_content": "ENC_NEW",
                    "status": "completed"
                }
            ]
        });
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::Text("First.".into())],
                    cache: false,
                    reasoning_details: Some(Arc::new(first_reasoning_details)),
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::Text("Second.".into())],
                    cache: false,
                    reasoning_details: Some(Arc::new(second_reasoning_details)),
                },
            ],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: true,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.3-codex",
            true,
            true,
            None,
            Some(ReasoningEffort::Medium),
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(
            serialized["input"],
            json!([
                {
                    "type": "reasoning",
                    "id": "rs_123",
                    "summary": [
                        {
                            "type": "summary_text",
                            "text": "Later metadata has the complete summary."
                        }
                    ],
                    "encrypted_content": "ENC_NEW",
                    "status": "completed"
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "content": [
                        { "type": "output_text", "text": "First.", "annotations": [] }
                    ],
                    "phase": "final_answer"
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "content": [
                        { "type": "output_text", "text": "Second.", "annotations": [] }
                    ],
                    "phase": "final_answer"
                }
            ])
        );
    }

    #[test]
    fn into_open_ai_response_replays_reasoning_details_but_not_thinking_text() {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![
                    MessageContent::Thinking {
                        text: "This is a reasoning summary, not assistant output.".into(),
                        signature: None,
                    },
                    MessageContent::Text("This is visible assistant output.".into()),
                ],
                cache: false,
                reasoning_details: Some(Arc::new(json!({
                    "reasoning_items": [
                        {
                            "id": "rs_123",
                            "summary": [
                                {
                                    "type": "summary_text",
                                    "text": "This is the reasoning summary to preserve."
                                }
                            ],
                            "encrypted_content": "ENC",
                            "status": "completed"
                        }
                    ]
                }))),
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: false,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let response = into_open_ai_response(
            request,
            "custom-model",
            false,
            false,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();
        let serialized = serde_json::to_value(&response).unwrap();

        assert_eq!(
            serialized["input"],
            json!([
                {
                    "type": "reasoning",
                    "id": "rs_123",
                    "summary": [
                        {
                            "type": "summary_text",
                            "text": "This is the reasoning summary to preserve."
                        }
                    ],
                    "encrypted_content": "ENC",
                    "status": "completed"
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "This is visible assistant output.",
                            "annotations": []
                        }
                    ]
                }
            ])
        );
        assert_eq!(
            serialized["include"],
            json!(["reasoning.encrypted_content"])
        );
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
                incomplete_details: Some(ResponseIncompleteDetails {
                    reason: Some("max_tokens".into()),
                }),
                usage: Some(ResponseUsage {
                    input_tokens: Some(10),
                    output_tokens: Some(20),
                    total_tokens: Some(30),
                    ..Default::default()
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
    fn responses_stream_failed_preserves_provider_rejection() {
        let mut mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
        let mapped = mapper.map_event(ResponsesStreamEvent::Failed {
            response: ResponseSummary {
                status: Some("failed".into()),
                error: Some(ResponseError {
                    code: Some("server_error".into()),
                    message: "The model failed to generate a response.".into(),
                    param: None,
                }),
                ..Default::default()
            },
        });

        assert_eq!(mapped.len(), 1);
        let error = mapped.into_iter().next().unwrap().unwrap_err();
        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                provider,
                status: None,
                code: Some(code),
                message,
                retry_after: None,
                ..
            } if provider == OPEN_AI_PROVIDER_NAME
                && code == "server_error"
                && message == "The model failed to generate a response."
        ));
    }

    #[test]
    fn responses_stream_deserializes_documented_error_event() {
        let event = serde_json::from_value::<ResponsesStreamEvent>(json!({
            "type": "error",
            "code": "ERR_SOMETHING",
            "message": "Something went wrong",
            "param": null,
            "sequence_number": 1
        }))
        .expect("documented error event");

        let mut mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
        let mapped = mapper.map_event(event);

        assert_eq!(mapped.len(), 1);
        let error = mapped.into_iter().next().unwrap().unwrap_err();
        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                provider,
                status: None,
                code: Some(code),
                message,
                retry_after: None,
                ..
            } if provider == OPEN_AI_PROVIDER_NAME
                && code == "ERR_SOMETHING"
                && message == "Something went wrong"
        ));
    }

    #[test]
    fn responses_stream_preserves_nested_cyber_policy_rejection() {
        // In practice the Responses API often nests error fields under an
        // `error` object even though the public spec documents them at the top
        // level. Make sure we don't lose the message and code in that case.
        let event = serde_json::from_value::<ResponsesStreamEvent>(json!({
            "type": "error",
            "error": {
                "type": "invalid_request_error",
                "code": "cyber_policy",
                "message": "This content was flagged as potentially violating our terms of use.",
                "param": "input"
            },
            "sequence_number": 2
        }))
        .expect("nested error event");

        let mut mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
        let mapped = mapper.map_event(event);

        assert_eq!(mapped.len(), 1);
        let error = mapped.into_iter().next().unwrap().unwrap_err();
        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                provider,
                status: None,
                code: Some(code),
                message,
                retry_after: None,
                category: ProviderErrorCategory::ContentPolicy,
            } if provider == OPEN_AI_PROVIDER_NAME
                && code == "cyber_policy"
                && message == "This content was flagged as potentially violating our terms of use."
        ));
    }

    #[test]
    fn responses_stream_maps_billing_codes_to_payment_required() {
        for code in [
            "billing_error",
            "payment_required_error",
            "credit_balance_exhausted",
            "insufficient_quota",
            "organization_spend_limit_exceeded",
            "project_spend_limit_exceeded",
            "organization_usage_limit_exceeded",
        ] {
            assert_eq!(
                response_error_category(Some(code), None, ""),
                ProviderErrorCategory::PaymentRequired,
                "{code}"
            );
        }
    }

    #[test]
    fn responses_stream_maps_invalid_prompt_to_content_policy() {
        assert_eq!(
            response_error_category(Some("invalid_prompt"), None, ""),
            ProviderErrorCategory::ContentPolicy
        );
    }

    #[test]
    fn responses_stream_maps_overload_codes_to_overloaded() {
        for code in ["overloaded_error", "server_is_overloaded", "slow_down"] {
            assert_eq!(
                response_error_category(Some(code), None, ""),
                ProviderErrorCategory::Overloaded,
                "{code}"
            );
        }
    }

    #[test]
    fn responses_stream_maps_context_length_exceeded_to_prompt_too_large() {
        let event = serde_json::from_value::<ResponsesStreamEvent>(json!({
            "type": "error",
            "error": {
                "type": "invalid_request_error",
                "code": "context_length_exceeded",
                "message": "Your input exceeds the context window of this model. Please adjust your input and try again.",
                "param": "input"
            },
            "sequence_number": 2
        }))
        .expect("nested error event");

        let mut mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
        let mapped = mapper.map_event(event);

        assert_eq!(mapped.len(), 1);
        let error = mapped.into_iter().next().unwrap().unwrap_err();
        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                category: ProviderErrorCategory::PromptTooLarge { tokens: None },
                ..
            }
        ));
    }

    #[test]
    fn responses_stream_maps_failed_context_length_exceeded_to_prompt_too_large() {
        let mut mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
        let mapped = mapper.map_event(ResponsesStreamEvent::Failed {
            response: ResponseSummary {
                status: Some("failed".into()),
                error: Some(ResponseError {
                    code: Some("context_length_exceeded".into()),
                    message: "Your input exceeds the context window of this model.".into(),
                    param: Some("input".into()),
                }),
                ..Default::default()
            },
        });

        assert_eq!(mapped.len(), 1);
        let error = mapped.into_iter().next().unwrap().unwrap_err();
        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                category: ProviderErrorCategory::PromptTooLarge { tokens: None },
                ..
            }
        ));
    }

    #[test]
    fn responses_stream_deserializes_response_error_event() {
        let event = serde_json::from_value::<ResponsesStreamEvent>(json!({
            "type": "response.error",
            "error": {
                "code": "invalid_request_error",
                "message": "Invalid request."
            }
        }))
        .expect("response error event");

        let mut mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
        let mapped = mapper.map_event(event);

        assert_eq!(mapped.len(), 1);
        let error = mapped.into_iter().next().unwrap().unwrap_err();
        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                provider,
                status: None,
                code: Some(code),
                message,
                retry_after: None,
                ..
            } if provider == OPEN_AI_PROVIDER_NAME
                && code == "invalid_request_error"
                && message == "Invalid request."
        ));
    }

    #[test]
    fn responses_stream_maps_refusal_events_to_refusal_stop() {
        let delta = serde_json::from_value::<ResponsesStreamEvent>(json!({
            "type": "response.refusal.delta",
            "item_id": "msg_123",
            "output_index": 0,
            "content_index": 0,
            "delta": "I can't help",
            "sequence_number": 1
        }))
        .expect("documented refusal delta event");
        let done = serde_json::from_value::<ResponsesStreamEvent>(json!({
            "type": "response.refusal.done",
            "item_id": "msg_123",
            "output_index": 0,
            "content_index": 0,
            "refusal": "I can't help with that.",
            "sequence_number": 2
        }))
        .expect("documented refusal done event");

        let mapped = map_response_events(vec![
            delta,
            done,
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ]);

        assert_eq!(mapped.len(), 1);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::Stop(StopReason::Refusal)
        ));
    }

    #[test]
    fn responses_stream_maps_refusal_output_to_refusal_stop() {
        let mapped = map_response_events(vec![ResponsesStreamEvent::Completed {
            response: ResponseSummary {
                output: vec![ResponseOutputItem::Message(ResponseOutputMessage {
                    id: Some("msg_123".into()),
                    role: Some("assistant".into()),
                    status: Some("completed".into()),
                    content: vec![json!({
                        "type": "refusal",
                        "refusal": "I can't help with that."
                    })],
                    phase: None,
                })],
                ..Default::default()
            },
        }]);

        assert_eq!(mapped.len(), 1);
        assert!(matches!(
            mapped[0],
            LanguageModelCompletionEvent::Stop(StopReason::Refusal)
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
                    incomplete_details: Some(ResponseIncompleteDetails {
                        reason: Some("max_tokens".into()),
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
                    incomplete_details: Some(ResponseIncompleteDetails {
                        reason: Some("max_tokens".into()),
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
                && matches!(input, LanguageModelToolUseInput::Json(value) if value.as_object().is_some_and(|object| object.is_empty()))
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
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_123",
                    vec![],
                    None,
                    None,
                )),
            },
            ResponsesStreamEvent::ReasoningSummaryPartAdded {
                item_id: "rs_123".into(),
                output_index: 0,
                summary_index: 0,
            },
            ResponsesStreamEvent::ReasoningSummaryTextDelta {
                item_id: "rs_123".into(),
                output_index: 0,
                summary_index: 0,
                delta: "Thinking about".into(),
            },
            ResponsesStreamEvent::ReasoningSummaryTextDelta {
                item_id: "rs_123".into(),
                output_index: 0,
                summary_index: 0,
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
            ResponsesStreamEvent::ReasoningSummaryTextDelta {
                item_id: "rs_123".into(),
                output_index: 0,
                summary_index: 1,
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
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_123",
                    vec![
                        ReasoningSummaryPart::SummaryText {
                            text: "Thinking about the answer".into(),
                        },
                        ReasoningSummaryPart::SummaryText {
                            text: "Second part".into(),
                        },
                    ],
                    None,
                    None,
                )),
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
    fn responses_stream_separates_reasoning_summary_items() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_1",
                    vec![],
                    None,
                    None,
                )),
            },
            ResponsesStreamEvent::ReasoningSummaryPartAdded {
                item_id: "rs_1".into(),
                output_index: 0,
                summary_index: 0,
            },
            ResponsesStreamEvent::ReasoningSummaryTextDelta {
                item_id: "rs_1".into(),
                output_index: 0,
                summary_index: 0,
                delta: "**First item**".into(),
            },
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 1,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_2",
                    vec![],
                    None,
                    None,
                )),
            },
            ResponsesStreamEvent::ReasoningSummaryPartAdded {
                item_id: "rs_2".into(),
                output_index: 1,
                summary_index: 0,
            },
            ResponsesStreamEvent::ReasoningSummaryTextDelta {
                item_id: "rs_2".into(),
                output_index: 1,
                summary_index: 0,
                delta: "**Second item**".into(),
            },
        ];

        let mapped = map_response_events(events);
        let thinking = mapped
            .into_iter()
            .filter_map(|event| match event {
                LanguageModelCompletionEvent::Thinking { text, signature: _ } => Some(text),
                _ => None,
            })
            .collect::<String>();

        assert_eq!(thinking, "**First item**\n\n**Second item**");
    }

    #[test]
    fn responses_stream_maps_reasoning_from_done_only() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_789",
                    vec![],
                    None,
                    None,
                )),
            },
            ResponsesStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_789",
                    vec![ReasoningSummaryPart::SummaryText {
                        text: "Summary without deltas".into(),
                    }],
                    None,
                    None,
                )),
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
    fn responses_stream_preserves_encrypted_reasoning_details() {
        let mut reasoning_item = response_reasoning_item(
            "rs_123",
            vec![ReasoningSummaryPart::SummaryText {
                text: "Checked what information is needed.".into(),
            }],
            Some("ENC"),
            Some("completed".into()),
        );
        reasoning_item.content = vec![json!({
            "type": "reasoning_text",
            "text": "Internal reasoning text."
        })];

        let events = vec![
            ResponsesStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(reasoning_item),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        let details = mapped
            .iter()
            .find_map(|event| match event {
                LanguageModelCompletionEvent::ReasoningDetails(details) => Some(details),
                _ => None,
            })
            .expect("reasoning details");

        assert_eq!(
            details,
            &json!({
                "reasoning_items": [
                    {
                        "id": "rs_123",
                        "summary": [
                            {
                                "type": "summary_text",
                                "text": "Checked what information is needed."
                            }
                        ],
                        "content": [
                            {
                                "type": "reasoning_text",
                                "text": "Internal reasoning text."
                            }
                        ],
                        "encrypted_content": "ENC",
                        "status": "completed",
                    }
                ]
            })
        );
    }

    #[test]
    fn responses_stream_replaces_reasoning_details_with_same_id() {
        let events = vec![
            ResponsesStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_123",
                    Vec::new(),
                    Some("ENC_OLD"),
                    Some("in_progress".into()),
                )),
            },
            ResponsesStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_123",
                    vec![ReasoningSummaryPart::SummaryText {
                        text: "Finished reasoning.".into(),
                    }],
                    Some("ENC_NEW"),
                    Some("completed".into()),
                )),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        let details = mapped
            .iter()
            .filter_map(|event| match event {
                LanguageModelCompletionEvent::ReasoningDetails(details) => Some(details),
                _ => None,
            })
            .next_back()
            .expect("reasoning details");

        assert_eq!(
            details,
            &json!({
                "reasoning_items": [
                    {
                        "id": "rs_123",
                        "summary": [
                            {
                                "type": "summary_text",
                                "text": "Finished reasoning."
                            }
                        ],
                        "encrypted_content": "ENC_NEW",
                        "status": "completed"
                    }
                ]
            })
        );
    }

    #[test]
    fn responses_stream_reemits_reasoning_details_after_phase_less_message_start() {
        let events = vec![
            ResponsesStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_123",
                    Vec::new(),
                    Some("ENC"),
                    Some("completed".into()),
                )),
            },
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 1,
                sequence_number: None,
                item: ResponseOutputItem::Message(ResponseOutputMessage {
                    id: Some("msg_123".into()),
                    role: Some("assistant".into()),
                    status: Some("in_progress".into()),
                    content: vec![],
                    phase: None,
                }),
            },
            ResponsesStreamEvent::OutputTextDelta {
                item_id: "msg_123".into(),
                output_index: 1,
                content_index: Some(0),
                delta: "Hello".into(),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        let start_message_index = mapped
            .iter()
            .position(|event| matches!(event, LanguageModelCompletionEvent::StartMessage { .. }))
            .expect("start message");
        let details = mapped
            .iter()
            .skip(start_message_index + 1)
            .find_map(|event| match event {
                LanguageModelCompletionEvent::ReasoningDetails(details) => Some(details),
                _ => None,
            })
            .expect("reasoning details after start message");

        assert_eq!(
            details,
            &json!({
                "reasoning_items": [
                    {
                        "id": "rs_123",
                        "summary": [],
                        "encrypted_content": "ENC",
                        "status": "completed"
                    }
                ]
            })
        );
    }

    #[test]
    fn responses_stream_preserves_assistant_phase_with_reasoning_details() {
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: ResponseOutputItem::Message(ResponseOutputMessage {
                    id: Some("msg_123".into()),
                    role: Some("assistant".into()),
                    status: Some("in_progress".into()),
                    content: vec![],
                    phase: Some("commentary".into()),
                }),
            },
            ResponsesStreamEvent::OutputTextDelta {
                item_id: "msg_123".into(),
                output_index: 0,
                content_index: Some(0),
                delta: "I will inspect the workspace.".into(),
            },
            ResponsesStreamEvent::OutputItemDone {
                output_index: 1,
                sequence_number: None,
                item: ResponseOutputItem::Reasoning(response_reasoning_item(
                    "rs_123",
                    Vec::new(),
                    Some("ENC"),
                    Some("completed".into()),
                )),
            },
            ResponsesStreamEvent::Completed {
                response: ResponseSummary::default(),
            },
        ];

        let mapped = map_response_events(events);
        let details = mapped
            .iter()
            .filter_map(|event| match event {
                LanguageModelCompletionEvent::ReasoningDetails(details) => Some(details),
                _ => None,
            })
            .next_back()
            .expect("reasoning details");

        assert_eq!(
            details,
            &json!({
                "phase": "commentary",
                "reasoning_items": [
                    {
                        "id": "rs_123",
                        "summary": [],
                        "encrypted_content": "ENC",
                        "status": "completed"
                    }
                ]
            })
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
            input: LanguageModelToolUseInput::Json(tool_input),
            is_input_complete: true,
            thought_signature: Some("thought-signature".into()),
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
                    reasoning_details: Some(Arc::new(json!([{
                        "id": "reasoning-1",
                        "type": "reasoning.encrypted",
                        "data": "encrypted"
                    }]))),
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
            compact_at_tokens: None,
        };

        let result = into_open_ai(
            request.clone(),
            "model",
            false,
            false,
            None,
            ChatCompletionMaxTokensParameter::MaxCompletionTokens,
            None,
            true,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(&result).unwrap()["messages"],
            json!([
                {"role": "user", "content": "search for something"},
                {
                    "role": "assistant",
                    "content": "Searching now.",
                    "tool_calls": [{"id": "call-1", "type": "function", "function": {"name": "search", "arguments": tool_arguments, "thought_signature": "thought-signature"}}],
                    "reasoning_content": "I should search",
                    "reasoning_details": [{
                        "id": "reasoning-1",
                        "type": "reasoning.encrypted",
                        "data": "encrypted"
                    }]
                },
                {"role": "tool", "content": "result", "tool_call_id": "call-1"}
            ])
        );

        let result = into_open_ai(
            request,
            "model",
            false,
            false,
            None,
            ChatCompletionMaxTokensParameter::MaxCompletionTokens,
            None,
            false,
        )
        .unwrap();
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

    #[test]
    fn stream_maps_reasoning() {
        let events = map_completion_events(vec![ResponseStreamEvent {
            choices: vec![ChoiceDelta {
                index: 0,
                delta: Some(ResponseMessageDelta {
                    content: None,
                    reasoning: Some("thinking".into()),
                    tool_calls: None,
                    reasoning_content: None,
                    reasoning_details: None,
                }),
                finish_reason: None,
            }],
            usage: None,
        }]);

        assert_eq!(
            events,
            vec![LanguageModelCompletionEvent::Thinking {
                text: "thinking".into(),
                signature: None,
            }]
        );
    }

    #[test]
    fn stream_maps_length_finish_reason_to_max_tokens_stop() {
        let events = map_completion_events(vec![ResponseStreamEvent {
            choices: vec![ChoiceDelta {
                index: 0,
                delta: None,
                finish_reason: Some("length".into()),
            }],
            usage: None,
        }]);

        assert_eq!(
            events,
            vec![LanguageModelCompletionEvent::Stop(StopReason::MaxTokens)]
        );
    }

    #[test]
    fn chunk_without_choices_or_usage_maps_to_no_events() {
        let mut mapper = OpenAiEventMapper::new();
        assert!(mapper.map_event(ResponseStreamEvent::default()).is_empty());
    }

    #[test]
    fn usage_update_precedes_text_and_stop_events_from_the_same_chunk() {
        let mut mapper = OpenAiEventMapper::new();
        let events = mapper.map_event(ResponseStreamEvent {
            choices: vec![ChoiceDelta {
                index: 0,
                delta: Some(ResponseMessageDelta {
                    content: Some("Hello!".to_string()),
                    ..Default::default()
                }),
                finish_reason: Some("stop".to_string()),
            }],
            usage: Some(Usage {
                prompt_tokens: Some(11),
                completion_tokens: Some(7),
                total_tokens: Some(18),
                prompt_tokens_details: None,
            }),
        });

        assert!(matches!(
            events.as_slice(),
            [
                Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 11,
                    output_tokens: 7,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                })),
                Ok(LanguageModelCompletionEvent::Text(text)),
                Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)),
            ] if text == "Hello!"
        ));
    }

    #[test]
    fn usage_update_precedes_tool_use_and_stop_events_from_the_same_chunk() {
        let mut mapper = OpenAiEventMapper::new();
        let events = mapper.map_event(ResponseStreamEvent {
            choices: vec![ChoiceDelta {
                index: 0,
                delta: Some(ResponseMessageDelta {
                    tool_calls: Some(vec![ToolCallChunk {
                        index: 0,
                        id: Some("tool-call-id".to_string()),
                        function: Some(FunctionChunk {
                            name: Some("test_tool".to_string()),
                            arguments: Some(r#"{"value":1}"#.to_string()),
                            thought_signature: None,
                        }),
                    }]),
                    ..Default::default()
                }),
                finish_reason: Some("tool_calls".to_string()),
            }],
            usage: Some(Usage {
                prompt_tokens: Some(13),
                completion_tokens: Some(5),
                total_tokens: Some(18),
                prompt_tokens_details: None,
            }),
        });

        assert!(matches!(
            events.as_slice(),
            [
                Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 13,
                    output_tokens: 5,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                })),
                Ok(LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                    is_input_complete: false,
                    ..
                })),
                Ok(LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                    id,
                    name,
                    is_input_complete: true,
                    ..
                })),
                Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)),
            ] if id.to_string() == "tool-call-id" && name.as_ref() == "test_tool"
        ));
    }

    #[test]
    fn stream_merges_reasoning_details_and_maps_compatible_usage_and_signatures() {
        let response_events = serde_json::from_value(json!([
            {
                "choices": [{
                    "index": 0,
                    "delta": {
                        "reasoning_details": [{
                            "id": "reasoning-1",
                            "index": 0,
                            "type": "reasoning.text",
                            "text": "first "
                        }],
                        "tool_calls": [{
                            "index": 0,
                            "id": "call-1",
                            "function": {
                                "name": "search",
                                "arguments": "{",
                                "thought_signature": "signature"
                            }
                        }]
                    },
                    "finish_reason": null
                }],
                "usage": null
            },
            {
                "choices": [{
                    "index": 0,
                    "delta": {
                        "reasoning_details": [{
                            "id": "reasoning-1",
                            "index": 0,
                            "text": "second"
                        }],
                        "tool_calls": [{
                            "index": 0,
                            "function": {
                                "arguments": "}"
                            }
                        }]
                    },
                    "finish_reason": "tool_calls"
                }],
                "usage": null
            },
            {
                "choices": [],
                "usage": {
                    "prompt_tokens": 10000,
                    "completion_tokens": 500,
                    "total_tokens": 10500,
                    "prompt_tokens_details": {
                        "cached_tokens": 6000,
                        "cache_write_tokens": 1000
                    }
                }
            }
        ]))
        .expect("valid compatible Chat Completions events");
        let events = map_completion_events(response_events);

        assert!(events.iter().any(|event| {
            matches!(
                event,
                LanguageModelCompletionEvent::ReasoningDetails(details)
                    if details[0]["text"] == "first second"
            )
        }));
        assert!(events.iter().any(|event| {
            matches!(
                event,
                LanguageModelCompletionEvent::ToolUse(tool_use)
                    if tool_use.is_input_complete
                        && tool_use.thought_signature.as_deref() == Some("signature")
            )
        }));
        assert!(events.iter().any(|event| {
            matches!(
                event,
                LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 3_000,
                    output_tokens: 500,
                    cache_creation_input_tokens: 1_000,
                    cache_read_input_tokens: 6_000,
                })
            )
        }));
    }

    #[test]
    fn reasoning_details_accumulator_replaces_an_incompatible_previous_shape() {
        let mut accumulator = ReasoningDetailsAccumulator::default();
        assert_eq!(
            accumulator.push(json!({"summary": "provider-defined"})),
            Some(json!({"summary": "provider-defined"}))
        );
        assert_eq!(
            accumulator.push(json!([{"index": 0, "text": "reasoning"}])),
            Some(json!([{"index": 0, "text": "reasoning"}]))
        );
    }

    // OpenRouter sends an empty `reasoning_details` array in the finish chunk;
    // it must not wipe out details accumulated from earlier chunks.
    #[test]
    fn reasoning_details_accumulator_ignores_null_and_empty_array_chunks() {
        let mut accumulator = ReasoningDetailsAccumulator::default();
        assert!(
            accumulator
                .push(json!([{"index": 0, "type": "reasoning.text", "text": "thinking"}]))
                .is_some()
        );
        assert!(
            accumulator
                .push(
                    json!([{"index": 0, "type": "reasoning.encrypted", "data": "encrypted-blob"}])
                )
                .is_some()
        );

        assert_eq!(accumulator.push(json!([])), None);
        assert_eq!(accumulator.push(serde_json::Value::Null), None);

        let details = accumulator
            .push(json!([{"index": 0, "text": " more"}]))
            .expect("accumulated reasoning details");
        assert_eq!(details[0]["text"], "thinking more");
        assert_eq!(details[0]["data"], "encrypted-blob");
    }

    #[test]
    fn stream_maps_preserves_tool_id_and_name_across_empty_deltas() {
        // DashScope sends id="" and name="" in subsequent tool_calls delta
        // chunks after the first chunk. OpenAiEventMapper must not overwrite
        // the accumulated id and name with these empty strings.

        let events = vec![
            // First chunk: id and name are present
            ResponseStreamEvent {
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: Some(ResponseMessageDelta {
                        content: None,
                        reasoning: None,
                        tool_calls: Some(vec![ToolCallChunk {
                            index: 0,
                            id: Some("call_dashscope_test".into()),
                            function: Some(FunctionChunk {
                                name: Some("list_directory".into()),
                                arguments: Some("".into()),
                                thought_signature: None,
                            }),
                        }]),
                        reasoning_content: None,
                        reasoning_details: None,
                    }),
                    finish_reason: None,
                }],
                usage: None,
            },
            // Subsequent chunks: DashScope sends id="" and name=""
            ResponseStreamEvent {
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: Some(ResponseMessageDelta {
                        content: None,
                        reasoning: None,
                        tool_calls: Some(vec![ToolCallChunk {
                            index: 0,
                            id: Some("".into()),
                            function: Some(FunctionChunk {
                                name: Some("".into()),
                                arguments: Some("{\"path\": \"".into()),
                                thought_signature: None,
                            }),
                        }]),
                        reasoning_content: None,
                        reasoning_details: None,
                    }),
                    finish_reason: None,
                }],
                usage: None,
            },
            ResponseStreamEvent {
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: Some(ResponseMessageDelta {
                        content: None,
                        reasoning: None,
                        tool_calls: Some(vec![ToolCallChunk {
                            index: 0,
                            id: Some("".into()),
                            function: Some(FunctionChunk {
                                name: Some("".into()),
                                arguments: Some("blog-scraper\"}".into()),
                                thought_signature: None,
                            }),
                        }]),
                        reasoning_content: None,
                        reasoning_details: None,
                    }),
                    finish_reason: None,
                }],
                usage: None,
            },
            // Final chunk: finish_reason = "tool_calls"
            ResponseStreamEvent {
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: None,
                    finish_reason: Some("tool_calls".into()),
                }],
                usage: None,
            },
        ];

        let mapped = map_completion_events(events);

        // Events emitted:
        //   1. Partial ToolUse from chunk 1 (fix_json("") → "{}", parseable)
        //   2. Partial ToolUse from chunk 3 (arguments fully assembled)
        //   3. Complete ToolUse from finish_reason="tool_calls" drain
        //   4. Stop(ToolUse)
        assert_eq!(mapped.len(), 4);

        // Verify the complete ToolUse event (from finish_reason drain)
        // has the correct id, name, and accumulated arguments.
        let complete_tool_use = mapped.iter().find_map(|event| {
            if let LanguageModelCompletionEvent::ToolUse(tool_use) = event {
                if tool_use.is_input_complete {
                    return Some(tool_use);
                }
            }
            None
        });
        assert!(
            complete_tool_use.is_some(),
            "expected a completed ToolUse event"
        );
        let tool_use = complete_tool_use.unwrap();
        assert_eq!(
            tool_use.id.to_string(),
            "call_dashscope_test",
            "id must survive empty-string overwrites"
        );
        assert_eq!(
            tool_use.name.as_ref(),
            "list_directory",
            "name must survive empty-string overwrites"
        );
        assert_eq!(
            tool_use.raw_input, "{\"path\": \"blog-scraper\"}",
            "arguments should accumulate across chunks"
        );

        // Verify the Stop event
        assert!(mapped.iter().any(|event| {
            matches!(
                event,
                LanguageModelCompletionEvent::Stop(StopReason::ToolUse)
            )
        }));
    }

    #[test]
    fn into_open_ai_response_prepends_provider_input_unchanged() {
        let provider_input = json!([
            {
                "type": "message",
                "role": "user",
                "content": "Retained user context.",
                "provider_extension": {"preserve": true}
            },
            {
                "type": "compaction",
                "id": "cmp_manual",
                "encrypted_content": "opaque-state"
            }
        ]);
        let request = LanguageModelRequest {
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::Compaction(CompactedContext::ProviderState(
                        provider_compaction_state_from_items(
                            OPEN_AI_PROVIDER_ID,
                            provider_input.as_array().unwrap().clone(),
                        )
                        .unwrap(),
                    ))],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Continue.".into())],
                    cache: false,
                    reasoning_details: None,
                },
            ],
            ..Default::default()
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.4",
            true,
            true,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_value(&response).unwrap()["input"],
            json!([
                {
                    "type": "message",
                    "role": "user",
                    "content": "Retained user context.",
                    "provider_extension": {"preserve": true}
                },
                {
                    "type": "compaction",
                    "id": "cmp_manual",
                    "encrypted_content": "opaque-state"
                },
                {
                    "type": "message",
                    "role": "user",
                    "content": [{"type": "input_text", "text": "Continue."}]
                }
            ])
        );
    }

    #[test]
    fn open_ai_response_converts_to_compact_request_with_supported_controls() {
        let request = LanguageModelRequest {
            thread_id: Some("thread-123".to_string()),
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Retain this context.".into())],
                cache: false,
                reasoning_details: None,
            }],
            speed: Some(Speed::Fast),
            compact_at_tokens: Some(100_000),
            ..Default::default()
        };

        let mut response_request = into_open_ai_response(
            request,
            "gpt-5.4",
            true,
            true,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();
        response_request.instructions = Some("Preserve implementation details.".to_string());
        let compact_request = response_request.into_compact_request();

        assert_eq!(
            serde_json::to_value(compact_request).unwrap(),
            json!({
                "model": "gpt-5.4",
                "instructions": "Preserve implementation details.",
                "input": [{
                    "type": "message",
                    "role": "user",
                    "content": [{
                        "type": "input_text",
                        "text": "Retain this context."
                    }]
                }],
                "prompt_cache_key": "thread-123",
                "service_tier": "priority"
            })
        );
    }

    #[test]
    fn into_open_ai_response_maps_compact_at_tokens_to_context_management() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".into())],
                cache: false,
                reasoning_details: None,
            }],
            compact_at_tokens: Some(100_000),
            ..Default::default()
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.1",
            true,
            true,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_value(&response).unwrap()["context_management"],
            json!([{ "type": "compaction", "compact_threshold": 100_000 }])
        );
    }

    #[test]
    fn into_open_ai_response_omits_context_management_without_compact_at_tokens() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".into())],
                cache: false,
                reasoning_details: None,
            }],
            ..Default::default()
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.1",
            true,
            true,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        assert!(
            serde_json::to_value(&response)
                .unwrap()
                .get("context_management")
                .is_none()
        );
    }

    #[test]
    fn into_open_ai_response_replays_provider_compaction_block() {
        let state = provider_compaction_state_from_items(
            OPEN_AI_PROVIDER_ID,
            vec![json!({
                "type": "compaction",
                "id": "cmp_1",
                "encrypted_content": "encrypted-blob"
            })],
        )
        .unwrap();
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![
                    MessageContent::Compaction(CompactedContext::ProviderState(state)),
                    MessageContent::Text("Done.".into()),
                ],
                cache: false,
                reasoning_details: None,
            }],
            ..Default::default()
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.1",
            true,
            true,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_value(&response).unwrap()["input"],
            json!([
                {
                    "type": "compaction",
                    "id": "cmp_1",
                    "encrypted_content": "encrypted-blob"
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "content": [
                        { "type": "output_text", "text": "Done.", "annotations": [] }
                    ]
                }
            ])
        );
    }

    #[test]
    fn into_open_ai_response_hoists_system_messages_into_instructions() {
        let request = LanguageModelRequest {
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("You are a coding assistant.".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hello.".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("Prefer terse answers.".into())],
                    cache: false,
                    reasoning_details: None,
                },
            ],
            ..Default::default()
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.1",
            true,
            true,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(
            serialized["instructions"],
            json!("You are a coding assistant.\n\nPrefer terse answers.")
        );
        assert_eq!(
            serialized["input"],
            json!([
                {
                    "type": "message",
                    "role": "user",
                    "content": [
                        { "type": "input_text", "text": "Hello." }
                    ]
                }
            ])
        );
    }

    /// Replaying provider compaction state discards all input items
    /// accumulated before it, but the system prompt must not be lost with
    /// them: it lives in the per-request `instructions` field, outside the
    /// compacted window, so the model keeps running on the current prompt
    /// rather than whatever was frozen into the window at compaction time.
    #[test]
    fn into_open_ai_response_preserves_system_prompt_across_compaction_replay() {
        let state = provider_compaction_state_from_items(
            OPEN_AI_PROVIDER_ID,
            vec![json!({
                "type": "compaction",
                "id": "cmp_1",
                "encrypted_content": "encrypted-blob"
            })],
        )
        .unwrap();
        let request = LanguageModelRequest {
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("Current system prompt.".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Old context.".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::Compaction(CompactedContext::ProviderState(
                        state,
                    ))],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Continue.".into())],
                    cache: false,
                    reasoning_details: None,
                },
            ],
            ..Default::default()
        };

        let response = into_open_ai_response(
            request,
            "gpt-5.1",
            true,
            true,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap();

        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(serialized["instructions"], json!("Current system prompt."));
        assert_eq!(
            serialized["input"],
            json!([
                {
                    "type": "compaction",
                    "id": "cmp_1",
                    "encrypted_content": "encrypted-blob"
                },
                {
                    "type": "message",
                    "role": "user",
                    "content": [
                        { "type": "input_text", "text": "Continue." }
                    ]
                }
            ])
        );
    }

    #[test]
    fn into_open_ai_response_rejects_malformed_provider_compaction_state() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::Compaction(CompactedContext::ProviderState(
                    language_model_core::ProviderCompactionState::new(
                        language_model_core::OPEN_AI_PROVIDER_ID,
                        crate::responses::COMPACTION_STATE_FORMAT,
                        "not valid JSON",
                    ),
                ))],
                cache: false,
                reasoning_details: None,
            }],
            ..Default::default()
        };

        let error = into_open_ai_response(
            request,
            "gpt-5.1",
            true,
            true,
            None,
            None,
            false,
            &OPEN_AI_PROVIDER_ID,
        )
        .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("malformed OpenAI compaction state payload")
        );
    }

    #[test]
    fn into_open_ai_response_ignores_compaction_state_owned_by_another_backend() {
        // Several backends share this request conversion (OpenAI itself,
        // OpenAI-compatible endpoints, Codex, Mantle), but an encrypted
        // compaction item is only decryptable by the backend that produced
        // it. Replaying OpenAI-owned state through a different backend would
        // discard the entire transcript in exchange for an opaque blob that
        // backend cannot read, so the state must be ignored and the full
        // transcript replayed instead.
        let state = provider_compaction_state_from_items(
            OPEN_AI_PROVIDER_ID,
            vec![json!({
                "type": "compaction",
                "id": "cmp_1",
                "encrypted_content": "encrypted-blob"
            })],
        )
        .unwrap();
        let request = LanguageModelRequest {
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Set up the project.".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![
                        MessageContent::Compaction(CompactedContext::ProviderState(state)),
                        MessageContent::Text("Done.".into()),
                    ],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Continue.".into())],
                    cache: false,
                    reasoning_details: None,
                },
            ],
            ..Default::default()
        };

        let response = into_open_ai_response(
            request,
            "compatible-model",
            true,
            true,
            None,
            None,
            false,
            &LanguageModelProviderId::new("my-compatible-endpoint"),
        )
        .unwrap();

        assert_eq!(
            serde_json::to_value(&response).unwrap()["input"],
            json!([
                {
                    "type": "message",
                    "role": "user",
                    "content": [{ "type": "input_text", "text": "Set up the project." }]
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "content": [
                        { "type": "output_text", "text": "Done.", "annotations": [] }
                    ]
                },
                {
                    "type": "message",
                    "role": "user",
                    "content": [{ "type": "input_text", "text": "Continue." }]
                }
            ])
        );
    }

    #[test]
    fn responses_stream_stamps_compaction_state_with_owning_backend() {
        let owner = LanguageModelProviderId::new("my-compatible-endpoint");
        let mut mapper = OpenAiResponseEventMapper::new(owner.clone());
        let item: ResponseOutputItem = serde_json::from_value(json!({
            "type": "compaction",
            "id": "cmp_1",
            "encrypted_content": "encrypted-blob"
        }))
        .unwrap();

        let mut events = mapper.map_event(ResponsesStreamEvent::OutputItemDone {
            output_index: 0,
            sequence_number: None,
            item,
        });

        let Some(Ok(LanguageModelCompletionEvent::Compaction(CompactionUpdate::Finished(
            CompactedContext::ProviderState(state),
        )))) = events.pop()
        else {
            panic!("expected finished provider compaction state");
        };
        assert_eq!(state.provider_id(), &owner);
        assert!(
            crate::responses::provider_compaction_items(&state, &owner)
                .unwrap()
                .is_some()
        );
        // OpenAI proper must not attempt to replay a window produced by a
        // different backend's infrastructure.
        assert_eq!(
            crate::responses::provider_compaction_items(&state, &OPEN_AI_PROVIDER_ID).unwrap(),
            None
        );
    }

    #[test]
    fn responses_stream_rejects_completion_with_unfinished_compaction() {
        let mut mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
        let item: ResponseOutputItem = serde_json::from_value(json!({
            "type": "compaction",
            "id": "cmp_1",
            "encrypted_content": "encrypted-blob"
        }))
        .unwrap();

        let started = mapper.map_event(ResponsesStreamEvent::OutputItemAdded {
            output_index: 0,
            sequence_number: None,
            item,
        });
        assert!(matches!(
            started.as_slice(),
            [Ok(LanguageModelCompletionEvent::Compaction(
                CompactionUpdate::Started
            ))]
        ));

        let mut completed = mapper.map_event(ResponsesStreamEvent::Completed {
            response: ResponseSummary::default(),
        });
        let error = completed.pop().unwrap().unwrap_err();

        assert!(error.to_string().contains("unfinished compaction"));
    }

    #[test]
    fn responses_stream_maps_compaction_output_item() {
        let item: ResponseOutputItem = serde_json::from_value(json!({
            "type": "compaction",
            "id": "cmp_1",
            "encrypted_content": "encrypted-blob"
        }))
        .unwrap();
        let events = vec![
            ResponsesStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: item.clone(),
            },
            ResponsesStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item,
            },
        ];

        let mapped = map_response_events(events);

        assert_eq!(
            mapped,
            vec![
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::Started),
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::Finished(
                    CompactedContext::ProviderState(
                        provider_compaction_state_from_items(
                            OPEN_AI_PROVIDER_ID,
                            vec![json!({
                                "type": "compaction",
                                "id": "cmp_1",
                                "encrypted_content": "encrypted-blob"
                            })],
                        )
                        .unwrap()
                    )
                )),
            ]
        );
    }
}
