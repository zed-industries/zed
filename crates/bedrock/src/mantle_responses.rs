//! Bedrock Mantle Responses API streaming: wire types plus the event mapper
//! that turns the Responses SSE stream into [`LanguageModelCompletionEvent`]s.
//!
//! This is a faithful port of the OpenAI Responses implementation
//! (`open_ai::responses` types + `open_ai::completion::OpenAiResponseEventMapper`),
//! copied here rather than depended upon so the Bedrock crate owns its own
//! Mantle logic. It differs from the OpenAI original in one way: Bedrock-hosted
//! models (e.g. Gemma) stream reasoning as the non-standard
//! `response.reasoning.delta` event instead of OpenAI's
//! `response.reasoning_summary_text.delta`, so both are handled here.

use anyhow::{Result, anyhow};
use futures::{StreamExt, stream::BoxStream};
use language_model_core::{
    CompactionContent, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelToolUse, LanguageModelToolUseId, StopReason, TokenUsage,
    util::{fix_streamed_json, parse_tool_arguments},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

const RESPONSE_MESSAGE_PHASE_COMMENTARY: &str = "commentary";
const RESPONSE_MESSAGE_PHASE_FINAL_ANSWER: &str = "final_answer";

// -- Wire types (Responses SSE) --------------------------------------------

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum StreamEvent {
    #[serde(rename = "response.created")]
    Created { response: ResponseSummary },
    #[serde(rename = "response.in_progress")]
    InProgress { response: ResponseSummary },
    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        output_index: usize,
        #[serde(default)]
        sequence_number: Option<u64>,
        item: ResponseOutputItem,
    },
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        output_index: usize,
        #[serde(default)]
        sequence_number: Option<u64>,
        item: ResponseOutputItem,
    },
    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        item_id: String,
        output_index: usize,
        content_index: usize,
        part: Value,
    },
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        item_id: String,
        output_index: usize,
        content_index: usize,
        part: Value,
    },
    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        item_id: String,
        output_index: usize,
        #[serde(default)]
        content_index: Option<usize>,
        delta: String,
    },
    #[serde(rename = "response.output_text.done")]
    OutputTextDone {
        item_id: String,
        output_index: usize,
        #[serde(default)]
        content_index: Option<usize>,
        text: String,
    },
    #[serde(rename = "response.refusal.delta")]
    RefusalDelta {
        item_id: String,
        output_index: usize,
        content_index: usize,
        delta: String,
        #[serde(default)]
        sequence_number: Option<u64>,
    },
    #[serde(rename = "response.refusal.done")]
    RefusalDone {
        item_id: String,
        output_index: usize,
        content_index: usize,
        refusal: String,
        #[serde(default)]
        sequence_number: Option<u64>,
    },
    #[serde(rename = "response.reasoning_summary_part.added")]
    ReasoningSummaryPartAdded {
        item_id: String,
        output_index: usize,
        summary_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ReasoningSummaryTextDelta {
        item_id: String,
        output_index: usize,
        delta: String,
    },
    #[serde(rename = "response.reasoning_summary_text.done")]
    ReasoningSummaryTextDone {
        item_id: String,
        output_index: usize,
        text: String,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ReasoningSummaryPartDone {
        item_id: String,
        output_index: usize,
        summary_index: usize,
    },
    /// Bedrock-hosted reasoning models (e.g. Gemma) stream plaintext reasoning
    /// here rather than as an OpenAI-style summary.
    #[serde(rename = "response.reasoning.delta")]
    ReasoningDelta {
        #[serde(default)]
        item_id: Option<String>,
        #[serde(default)]
        output_index: Option<usize>,
        delta: String,
    },
    #[serde(rename = "response.reasoning.done")]
    ReasoningDone {
        #[serde(default)]
        item_id: Option<String>,
        #[serde(default)]
        output_index: Option<usize>,
        #[serde(default)]
        text: Option<String>,
    },
    #[serde(rename = "response.function_call_arguments.delta")]
    FunctionCallArgumentsDelta {
        item_id: String,
        output_index: usize,
        delta: String,
        #[serde(default)]
        sequence_number: Option<u64>,
    },
    #[serde(rename = "response.function_call_arguments.done")]
    FunctionCallArgumentsDone {
        item_id: String,
        output_index: usize,
        arguments: String,
        #[serde(default)]
        sequence_number: Option<u64>,
    },
    #[serde(rename = "response.completed")]
    Completed { response: ResponseSummary },
    #[serde(rename = "response.incomplete")]
    Incomplete { response: ResponseSummary },
    #[serde(rename = "response.failed")]
    Failed { response: ResponseSummary },
    #[serde(rename = "response.error")]
    Error { error: ResponseError },
    #[serde(rename = "error")]
    GenericError {
        #[serde(flatten)]
        error: GenericStreamErrorPayload,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseSummary {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub incomplete_details: Option<ResponseIncompleteDetails>,
    #[serde(default)]
    pub error: Option<ResponseError>,
    #[serde(default)]
    pub usage: Option<ResponseUsage>,
    #[serde(default)]
    pub output: Vec<ResponseOutputItem>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseIncompleteDetails {
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseUsage {
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub input_tokens_details: ResponseInputTokensDetails,
    #[serde(default)]
    pub output_tokens: Option<u64>,
    #[serde(default)]
    pub total_tokens: Option<u64>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseInputTokensDetails {
    #[serde(default)]
    pub cached_tokens: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseOutputItem {
    Message(ResponseOutputMessage),
    FunctionCall(ResponseFunctionToolCall),
    Reasoning(ResponseReasoningItem),
    Compaction(ResponseCompactionItem),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseReasoningItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub summary: Vec<ReasoningSummaryPart>,
    #[serde(default)]
    pub content: Vec<Value>,
    #[serde(default)]
    pub encrypted_content: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReasoningSummaryPart {
    SummaryText { text: String },
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseOutputMessage {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub content: Vec<Value>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub phase: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseFunctionToolCall {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub arguments: String,
    #[serde(default)]
    pub call_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseCompactionItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Arc<str>>,
    pub encrypted_content: Arc<str>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseError {
    #[serde(default)]
    pub code: Option<String>,
    pub message: String,
    #[serde(default)]
    pub param: Option<Value>,
}

/// Payload of the top-level `error` SSE event. OpenAI documents the fields at
/// the top level, but the API often nests them under an `error` object.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct GenericStreamErrorPayload {
    #[serde(flatten)]
    top_level: PartialResponseError,
    #[serde(default)]
    error: Option<PartialResponseError>,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct PartialResponseError {
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    param: Option<Value>,
}

impl GenericStreamErrorPayload {
    fn into_response_error(self) -> ResponseError {
        let nested = self.error.unwrap_or_default();
        ResponseError {
            code: self.top_level.code.or(nested.code),
            message: self
                .top_level
                .message
                .or(nested.message)
                .unwrap_or_default(),
            param: self.top_level.param.or(nested.param),
        }
    }
}

/// Reasoning item as replayed back to the model on subsequent turns. Kept
/// serialization-compatible with the OpenAI request builder so the
/// `ReasoningDetails` we emit round-trip through `into_open_ai_response`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseReasoningInputItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub summary: Vec<ResponseReasoningSummaryPart>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseReasoningSummaryPart {
    SummaryText { text: String },
}

// -- Event mapper -----------------------------------------------------------

pub struct MantleResponseEventMapper {
    function_calls_by_item: HashMap<String, PendingResponseFunctionCall>,
    reasoning_items: Vec<ResponseReasoningInputItem>,
    current_message_phase: Option<String>,
    pending_stop_reason: Option<StopReason>,
}

#[derive(Default)]
struct PendingResponseFunctionCall {
    call_id: String,
    name: Arc<str>,
    arguments: String,
}

impl MantleResponseEventMapper {
    pub fn new() -> Self {
        Self {
            function_calls_by_item: HashMap::default(),
            reasoning_items: Vec::new(),
            current_message_phase: None,
            pending_stop_reason: None,
        }
    }

    pub fn map_stream(
        mut self,
        events: BoxStream<'static, Result<StreamEvent>>,
    ) -> BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        events
            .flat_map(move |event| {
                futures::stream::iter(match event {
                    Ok(event) => self.map_event(event),
                    Err(error) => vec![Err(LanguageModelCompletionError::from(anyhow!(error)))],
                })
            })
            .boxed()
    }

    pub fn map_event(
        &mut self,
        event: StreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        match event {
            StreamEvent::OutputItemAdded { item, .. } => {
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
                    ResponseOutputItem::Compaction(_) => {
                        events.push(Ok(LanguageModelCompletionEvent::Compaction(
                            CompactionContent::Pending,
                        )));
                    }
                    ResponseOutputItem::Reasoning(_) | ResponseOutputItem::Unknown => {}
                }
                events
            }
            StreamEvent::ReasoningSummaryTextDelta { delta, .. }
            | StreamEvent::ReasoningDelta { delta, .. } => {
                if delta.is_empty() {
                    Vec::new()
                } else {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: delta,
                        signature: None,
                    })]
                }
            }
            StreamEvent::OutputTextDelta { delta, .. } => {
                if delta.is_empty() {
                    Vec::new()
                } else {
                    vec![Ok(LanguageModelCompletionEvent::Text(delta))]
                }
            }
            StreamEvent::RefusalDelta { .. } | StreamEvent::RefusalDone { .. } => {
                self.pending_stop_reason = Some(StopReason::Refusal);
                Vec::new()
            }
            StreamEvent::FunctionCallArgumentsDelta { item_id, delta, .. } => {
                if let Some(entry) = self.function_calls_by_item.get_mut(&item_id) {
                    entry.arguments.push_str(&delta);
                    if let Ok(input) =
                        serde_json::from_str::<Value>(&fix_streamed_json(&entry.arguments))
                    {
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
            StreamEvent::FunctionCallArgumentsDone {
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
            StreamEvent::Completed { response } => {
                self.handle_completion(response, StopReason::EndTurn)
            }
            StreamEvent::Incomplete { response } => {
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
            StreamEvent::Failed { response } => {
                let message = response_failure_message(&response);
                vec![Err(LanguageModelCompletionError::Other(anyhow!(message)))]
            }
            StreamEvent::Error { error } => {
                vec![Err(LanguageModelCompletionError::Other(anyhow!(
                    response_error_message(&error)
                )))]
            }
            StreamEvent::GenericError { error } => {
                let error = error.into_response_error();
                vec![Err(LanguageModelCompletionError::Other(anyhow!(
                    response_error_message(&error)
                )))]
            }
            StreamEvent::ReasoningSummaryPartAdded { summary_index, .. } => {
                if summary_index > 0 {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: "\n\n".to_string(),
                        signature: None,
                    })]
                } else {
                    Vec::new()
                }
            }
            StreamEvent::OutputItemDone { item, .. } => match item {
                ResponseOutputItem::Reasoning(reasoning) => self.capture_reasoning_item(&reasoning),
                ResponseOutputItem::Message(message) => self.capture_message_phase(&message),
                ResponseOutputItem::Compaction(compaction) => {
                    vec![Ok(LanguageModelCompletionEvent::Compaction(
                        CompactionContent::Encrypted {
                            id: compaction.id,
                            encrypted_content: compaction.encrypted_content,
                        },
                    ))]
                }
                ResponseOutputItem::FunctionCall(_) | ResponseOutputItem::Unknown => Vec::new(),
            },
            StreamEvent::OutputTextDone { .. }
            | StreamEvent::ContentPartAdded { .. }
            | StreamEvent::ContentPartDone { .. }
            | StreamEvent::ReasoningSummaryTextDone { .. }
            | StreamEvent::ReasoningSummaryPartDone { .. }
            | StreamEvent::ReasoningDone { .. }
            | StreamEvent::Created { .. }
            | StreamEvent::InProgress { .. }
            | StreamEvent::Unknown => Vec::new(),
        }
    }

    fn handle_completion(
        &mut self,
        response: ResponseSummary,
        default_reason: StopReason,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
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

impl Default for MantleResponseEventMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
struct ResponseMessageMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    phase: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    reasoning_items: Vec<ResponseReasoningInputItem>,
}

fn normalize_response_message_phase(phase: &str) -> Option<&'static str> {
    match phase {
        RESPONSE_MESSAGE_PHASE_COMMENTARY => Some(RESPONSE_MESSAGE_PHASE_COMMENTARY),
        RESPONSE_MESSAGE_PHASE_FINAL_ANSWER => Some(RESPONSE_MESSAGE_PHASE_FINAL_ANSWER),
        _ => None,
    }
}

fn response_failure_message(response: &ResponseSummary) -> String {
    if let Some(error) = response.error.as_ref() {
        return response_error_message(error);
    }

    response
        .status
        .as_deref()
        .map(|status| format!("response.{status}"))
        .unwrap_or_else(|| "response.failed".to_string())
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

fn response_content_is_refusal(content: &Value) -> bool {
    let content_type = content
        .get("type")
        .and_then(|content_type| content_type.as_str());
    let refusal = content
        .get("refusal")
        .and_then(|refusal| refusal.as_str())
        .unwrap_or_default();

    content_type == Some("refusal") || !refusal.is_empty()
}

fn token_usage_from_response_usage(usage: &ResponseUsage) -> TokenUsage {
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
    let summary = reasoning
        .summary
        .iter()
        .filter_map(|part| match part {
            ReasoningSummaryPart::SummaryText { text } => {
                Some(ResponseReasoningSummaryPart::SummaryText { text: text.clone() })
            }
            ReasoningSummaryPart::Unknown => None,
        })
        .collect();

    ResponseReasoningInputItem {
        id: reasoning.id.clone(),
        summary,
        content: reasoning.content.clone(),
        encrypted_content: reasoning.encrypted_content.clone(),
        status: reasoning.status.clone(),
    }
}
