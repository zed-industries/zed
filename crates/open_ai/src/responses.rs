use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
};
use serde::{Deserialize, Serialize, ser::SerializeSeq as _};
use serde_json::Value;
use std::sync::Arc;

use crate::{ReasoningEffort, RequestError, Role, ServiceTier, ToolChoice};

#[derive(Serialize, Debug)]
pub struct Request {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "ResponseInput::is_empty")]
    pub input: ResponseInput,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<ResponseIncludable>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_management: Option<Vec<ContextManagement>>,
}

impl Request {
    pub fn into_compact_request(self) -> CompactRequest {
        CompactRequest {
            model: self.model,
            instructions: self.instructions,
            input: self.input,
            prompt_cache_key: self.prompt_cache_key,
            service_tier: self.service_tier,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CompactRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    pub input: ResponseInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
}

#[derive(Deserialize, Debug)]
pub struct CompactedResponse {
    pub id: String,
    pub created_at: u64,
    pub object: String,
    pub output: Vec<Value>,
    pub usage: ResponseUsage,
}

impl CompactedResponse {
    pub fn into_compaction_items(self) -> Result<Vec<Value>> {
        if self.output.is_empty() {
            return Err(anyhow!("OpenAI returned an empty compaction output"));
        }
        if !self.output.iter().any(|item| {
            item.get("type")
                .and_then(Value::as_str)
                .is_some_and(|item_type| item_type == "compaction")
        }) {
            return Err(anyhow!(
                "OpenAI compaction output did not contain a compaction item"
            ));
        }
        Ok(self.output)
    }
}

#[derive(Debug, Default)]
pub struct ResponseInput {
    provider_items: Vec<Value>,
    generated_items: Vec<ResponseInputItem>,
}

impl ResponseInput {
    pub fn new(provider_items: Vec<Value>, generated_items: Vec<ResponseInputItem>) -> Self {
        Self {
            provider_items,
            generated_items,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.provider_items.is_empty() && self.generated_items.is_empty()
    }

    pub fn retain(&mut self, predicate: impl FnMut(&ResponseInputItem) -> bool) {
        self.generated_items.retain(predicate);
    }
}

impl Serialize for ResponseInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sequence = serializer
            .serialize_seq(Some(self.provider_items.len() + self.generated_items.len()))?;
        for item in &self.provider_items {
            sequence.serialize_element(item)?;
        }
        for item in &self.generated_items {
            sequence.serialize_element(item)?;
        }
        sequence.end()
    }
}

/// Server-side context management configuration.
///
/// <https://developers.openai.com/api/docs/guides/compaction>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContextManagement {
    Compaction { compact_threshold: u64 },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseIncludable {
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseInputItem {
    Message(ResponseMessageItem),
    FunctionCall(ResponseFunctionCallItem),
    FunctionCallOutput(ResponseFunctionCallOutputItem),
    CustomToolCall(ResponseCustomToolCallItem),
    CustomToolCallOutput(ResponseCustomToolCallOutputItem),
    Reasoning(ResponseReasoningInputItem),
    Compaction(ResponseCompactionItem),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseCompactionItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Arc<str>>,
    pub encrypted_content: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessageItem {
    pub role: Role,
    pub content: Vec<ResponseInputContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFunctionCallItem {
    pub call_id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFunctionCallOutputItem {
    pub call_id: String,
    pub output: ResponseFunctionCallOutputContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCustomToolCallItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub call_id: String,
    pub name: String,
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCustomToolCallOutputItem {
    pub call_id: String,
    pub output: ResponseFunctionCallOutputContent,
}

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseFunctionCallOutputContent {
    List(Vec<ResponseInputContent>),
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseInputContent {
    #[serde(rename = "input_text")]
    Text { text: String },
    #[serde(rename = "input_image")]
    Image { image_url: String },
    #[serde(rename = "output_text")]
    OutputText {
        text: String,
        #[serde(default)]
        annotations: Vec<serde_json::Value>,
    },
    #[serde(rename = "refusal")]
    Refusal { refusal: String },
}

#[derive(Serialize, Debug)]
pub struct ReasoningConfig {
    pub effort: ReasoningEffort,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummaryMode>,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummaryMode {
    Auto,
    Concise,
    Detailed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    Function {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },
    Custom {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<CustomToolFormat>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CustomToolFormat {
    Text,
    Grammar {
        syntax: CustomToolGrammarSyntax,
        definition: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CustomToolGrammarSyntax {
    Lark,
    Regex,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseError {
    #[serde(default)]
    pub code: Option<String>,
    pub message: String,
    #[serde(default)]
    pub param: Option<Value>,
}

/// Payload of the top-level `error` SSE event from the Responses API.
///
/// OpenAI's spec documents the error fields as being at the top level of the
/// event, but in practice the API often nests them under an `error` object.
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
    pub fn into_response_error(self) -> ResponseError {
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
    #[serde(rename = "response.custom_tool_call_input.delta")]
    CustomToolCallInputDelta {
        item_id: String,
        output_index: usize,
        delta: String,
        #[serde(default)]
        sequence_number: Option<u64>,
    },
    #[serde(rename = "response.custom_tool_call_input.done")]
    CustomToolCallInputDone {
        item_id: String,
        output_index: usize,
        input: String,
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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
    #[serde(default)]
    pub service_tier: Option<crate::ServiceTier>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ResponseIncompleteDetails {
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ResponseUsage {
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub input_tokens_details: ResponseInputTokensDetails,
    #[serde(default)]
    pub output_tokens: Option<u64>,
    #[serde(default)]
    pub output_tokens_details: ResponseOutputTokensDetails,
    #[serde(default)]
    pub total_tokens: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ResponseInputTokensDetails {
    #[serde(default)]
    pub cached_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ResponseOutputTokensDetails {
    #[serde(default)]
    pub reasoning_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseOutputItem {
    Message(ResponseOutputMessage),
    FunctionCall(ResponseFunctionToolCall),
    CustomToolCall(ResponseCustomToolCall),
    Reasoning(ResponseReasoningItem),
    Compaction(ResponseCompactionItem),
    /// Deserialization-only catch-all; must never be serialized back to the API.
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReasoningSummaryPart {
    SummaryText {
        text: String,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseCustomToolCall {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub call_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub input: String,
}

pub async fn compact_response(
    client: &dyn HttpClient,
    provider_name: &str,
    api_url: &str,
    api_key: &str,
    request: CompactRequest,
    extra_headers: &CustomHeaders,
) -> Result<CompactedResponse, RequestError> {
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(format!("{api_url}/responses/compact"))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .extra_headers(extra_headers)
        .body(AsyncBody::from(
            serde_json::to_string(&request).map_err(|error| RequestError::Other(error.into()))?,
        ))
        .map_err(|error| RequestError::Other(error.into()))?;

    let mut response = client.send(request).await?;
    let mut body = String::new();
    response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .map_err(|error| RequestError::Other(error.into()))?;

    if response.status().is_success() {
        serde_json::from_str(&body).map_err(|error| RequestError::Other(error.into()))
    } else {
        Err(RequestError::HttpResponseError {
            provider: provider_name.to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

pub async fn stream_response(
    client: &dyn HttpClient,
    provider_name: &str,
    api_url: &str,
    api_key: &str,
    request: Request,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<StreamEvent>>, RequestError> {
    let uri = format!("{api_url}/responses");
    let is_streaming = request.stream;
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .extra_headers(extra_headers)
        .body(AsyncBody::from(
            serde_json::to_string(&request).map_err(|e| RequestError::Other(e.into()))?,
        ))
        .map_err(|e| RequestError::Other(e.into()))?;

    let mut response = client.send(request).await?;
    if response.status().is_success() {
        if is_streaming {
            let reader = BufReader::new(response.into_body());
            Ok(reader
                .lines()
                .filter_map(|line| async move {
                    match line {
                        Ok(line) => {
                            let line = line
                                .strip_prefix("data: ")
                                .or_else(|| line.strip_prefix("data:"))?;
                            if line == "[DONE]" || line.is_empty() {
                                None
                            } else {
                                match serde_json::from_str::<StreamEvent>(line) {
                                    Ok(event) => Some(Ok(event)),
                                    Err(error) => {
                                        log::error!(
                                            "Failed to parse OpenAI responses stream event: `{}`\nResponse: `{}`",
                                            error,
                                            line,
                                        );
                                        Some(Err(anyhow!(error)))
                                    }
                                }
                            }
                        }
                        Err(error) => Some(Err(anyhow!(error))),
                    }
                })
                .boxed())
        } else {
            let mut body = String::new();
            response
                .body_mut()
                .read_to_string(&mut body)
                .await
                .map_err(|e| RequestError::Other(e.into()))?;

            match serde_json::from_str::<ResponseSummary>(&body) {
                Ok(response_summary) => {
                    let events = vec![
                        StreamEvent::Created {
                            response: response_summary.clone(),
                        },
                        StreamEvent::InProgress {
                            response: response_summary.clone(),
                        },
                    ];

                    let mut all_events = events;
                    for (output_index, item) in response_summary.output.iter().enumerate() {
                        all_events.push(StreamEvent::OutputItemAdded {
                            output_index,
                            sequence_number: None,
                            item: item.clone(),
                        });

                        match item {
                            ResponseOutputItem::Message(message) => {
                                for content_item in &message.content {
                                    if let Some(text) = content_item.get("text") {
                                        if let Some(text_str) = text.as_str() {
                                            if let Some(ref item_id) = message.id {
                                                all_events.push(StreamEvent::OutputTextDelta {
                                                    item_id: item_id.clone(),
                                                    output_index,
                                                    content_index: None,
                                                    delta: text_str.to_string(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            ResponseOutputItem::FunctionCall(function_call) => {
                                if let Some(ref item_id) = function_call.id {
                                    all_events.push(StreamEvent::FunctionCallArgumentsDone {
                                        item_id: item_id.clone(),
                                        output_index,
                                        arguments: function_call.arguments.clone(),
                                        sequence_number: None,
                                    });
                                }
                            }
                            ResponseOutputItem::CustomToolCall(custom_tool_call) => {
                                if let Some(ref item_id) = custom_tool_call.id {
                                    all_events.push(StreamEvent::CustomToolCallInputDone {
                                        item_id: item_id.clone(),
                                        output_index,
                                        input: custom_tool_call.input.clone(),
                                        sequence_number: None,
                                    });
                                }
                            }
                            ResponseOutputItem::Reasoning(reasoning) => {
                                if let Some(ref item_id) = reasoning.id {
                                    for part in &reasoning.summary {
                                        if let ReasoningSummaryPart::SummaryText { text } = part {
                                            all_events.push(
                                                StreamEvent::ReasoningSummaryTextDelta {
                                                    item_id: item_id.clone(),
                                                    output_index,
                                                    delta: text.clone(),
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                            // No synthesized deltas; the `OutputItemDone`
                            // event pushed below carries the full item.
                            ResponseOutputItem::Compaction(_) => {}
                            ResponseOutputItem::Unknown => {}
                        }

                        all_events.push(StreamEvent::OutputItemDone {
                            output_index,
                            sequence_number: None,
                            item: item.clone(),
                        });
                    }

                    let status = response_summary.status.clone();
                    all_events.push(match status.as_deref() {
                        Some("incomplete") => StreamEvent::Incomplete {
                            response: response_summary,
                        },
                        Some("failed") => StreamEvent::Failed {
                            response: response_summary,
                        },
                        _ => StreamEvent::Completed {
                            response: response_summary,
                        },
                    });

                    Ok(futures::stream::iter(all_events.into_iter().map(Ok)).boxed())
                }
                Err(error) => {
                    log::error!(
                        "Failed to parse OpenAI non-streaming response: `{}`\nResponse: `{}`",
                        error,
                        body,
                    );
                    Err(RequestError::Other(anyhow!(error)))
                }
            }
        }
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Err(RequestError::HttpResponseError {
            provider: provider_name.to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use http_client::FakeHttpClient;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    #[test]
    fn compact_response_posts_supported_request_fields() {
        let captured_request = Arc::new(Mutex::new(None));
        let captured_request_for_handler = captured_request.clone();
        let http_client = FakeHttpClient::create(move |request| {
            let captured_request = captured_request_for_handler.clone();
            async move {
                let method = request.method().clone();
                let uri = request.uri().to_string();
                let authorization = request
                    .headers()
                    .get("Authorization")
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_string);
                let mut body = request.into_body();
                let mut body_text = String::new();
                body.read_to_string(&mut body_text).await?;
                *captured_request.lock().unwrap() = Some((method, uri, authorization, body_text));

                Ok(http_client::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(
                        json!({
                            "id": "resp_compact",
                            "created_at": 1_700_000_000,
                            "object": "response.compaction",
                            "output": [{
                                "type": "compaction",
                                "id": "cmp_manual",
                                "encrypted_content": "opaque-state"
                            }],
                            "usage": {
                                "input_tokens": 100,
                                "input_tokens_details": {"cached_tokens": 20},
                                "output_tokens": 10,
                                "output_tokens_details": {"reasoning_tokens": 5},
                                "total_tokens": 110
                            }
                        })
                        .to_string(),
                    ))?)
            }
        });
        let response = block_on(compact_response(
            http_client.as_ref(),
            "OpenAI",
            "https://api.openai.com/v1",
            "secret",
            compact_test_request(),
            &CustomHeaders::default(),
        ))
        .unwrap();

        assert_eq!(
            response.into_compaction_items().unwrap(),
            vec![json!({
                "type": "compaction",
                "id": "cmp_manual",
                "encrypted_content": "opaque-state"
            })]
        );
        let (method, uri, authorization, body) = captured_request.lock().unwrap().take().unwrap();
        assert_eq!(method, Method::POST);
        assert_eq!(uri, "https://api.openai.com/v1/responses/compact");
        assert_eq!(authorization.as_deref(), Some("Bearer secret"));
        assert_eq!(
            serde_json::from_str::<Value>(&body).unwrap(),
            json!({
                "model": "gpt-5.4",
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
    fn compact_response_reports_http_and_deserialization_errors() {
        let http_client = FakeHttpClient::create(|_| async move {
            Ok(http_client::Response::builder()
                .status(429)
                .header("retry-after", "5")
                .body(AsyncBody::from("rate limited"))?)
        });

        let error = block_on(compact_response(
            http_client.as_ref(),
            "OpenAI",
            "https://api.openai.com/v1",
            "secret",
            compact_test_request(),
            &CustomHeaders::default(),
        ))
        .unwrap_err();

        match error {
            RequestError::HttpResponseError {
                provider,
                status_code,
                body,
                headers,
            } => {
                assert_eq!(provider, "OpenAI");
                assert_eq!(status_code, 429);
                assert_eq!(body, "rate limited");
                assert_eq!(headers["retry-after"], "5");
            }
            error => panic!("expected an HTTP response error, got {error:?}"),
        }

        let http_client = FakeHttpClient::create(|_| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body(AsyncBody::from("not valid JSON"))?)
        });

        let error = block_on(compact_response(
            http_client.as_ref(),
            "OpenAI",
            "https://api.openai.com/v1",
            "secret",
            compact_test_request(),
            &CustomHeaders::default(),
        ))
        .unwrap_err();

        assert!(
            matches!(error, RequestError::Other(_)),
            "expected malformed JSON to produce a request error, got {error:?}"
        );
    }

    #[test]
    fn compacted_response_preserves_canonical_output_items() {
        let output = vec![
            json!({
                "type": "message",
                "role": "user",
                "content": "Retained user context.",
                "provider_extension": {"preserve": true}
            }),
            json!({
                "type": "compaction",
                "id": "cmp_manual",
                "encrypted_content": "opaque-state"
            }),
        ];
        let response: CompactedResponse = serde_json::from_value(json!({
            "id": "resp_compact",
            "created_at": 1_700_000_000,
            "object": "response.compaction",
            "output": &output,
            "usage": {
                "input_tokens": 100,
                "input_tokens_details": {"cached_tokens": 20},
                "output_tokens": 10,
                "output_tokens_details": {"reasoning_tokens": 5},
                "total_tokens": 110
            }
        }))
        .unwrap();

        assert_eq!(response.into_compaction_items().unwrap(), output);
    }

    #[test]
    fn compacted_response_rejects_output_without_compaction_item() {
        let response: CompactedResponse = serde_json::from_value(json!({
            "id": "resp_compact",
            "created_at": 1_700_000_000,
            "object": "response.compaction",
            "output": [{
                "type": "message",
                "role": "user",
                "content": "Retained user context."
            }],
            "usage": {
                "input_tokens": 100,
                "input_tokens_details": {"cached_tokens": 20},
                "output_tokens": 10,
                "output_tokens_details": {"reasoning_tokens": 5},
                "total_tokens": 110
            }
        }))
        .unwrap();

        assert!(
            response
                .into_compaction_items()
                .unwrap_err()
                .to_string()
                .contains("compaction item")
        );
    }

    #[test]
    fn compacted_response_rejects_empty_output() {
        let response: CompactedResponse = serde_json::from_value(json!({
            "id": "resp_compact",
            "created_at": 1_700_000_000,
            "object": "response.compaction",
            "output": [],
            "usage": {
                "input_tokens": 100,
                "input_tokens_details": {"cached_tokens": 20},
                "output_tokens": 10,
                "output_tokens_details": {"reasoning_tokens": 5},
                "total_tokens": 110
            }
        }))
        .unwrap();

        assert!(
            response
                .into_compaction_items()
                .unwrap_err()
                .to_string()
                .contains("empty")
        );
    }

    fn compact_test_request() -> CompactRequest {
        CompactRequest {
            model: "gpt-5.4".to_string(),
            instructions: None,
            input: ResponseInput::new(
                Vec::new(),
                vec![ResponseInputItem::Message(ResponseMessageItem {
                    role: Role::User,
                    content: vec![ResponseInputContent::Text {
                        text: "Retain this context.".to_string(),
                    }],
                    phase: None,
                })],
            ),
            prompt_cache_key: Some("thread-123".to_string()),
            service_tier: Some(ServiceTier::Priority),
        }
    }
}
