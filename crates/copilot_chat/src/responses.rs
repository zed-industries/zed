use std::sync::Arc;

use super::{ChatLocation, copilot_request_headers};
use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize, ser::SerializeMap};
use serde_json::Value;
pub use settings::OpenAiReasoningEffort as ReasoningEffort;

#[derive(Serialize, Debug)]
pub struct Request {
    pub model: String,
    pub input: Vec<ResponseInputItem>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<ResponseIncludable>>,
    pub store: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseIncludable {
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
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
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    #[serde(untagged)]
    Other(ToolDefinition),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummary {
    Auto,
    Concise,
    Detailed,
}

#[derive(Serialize, Debug)]
pub struct ReasoningConfig {
    pub effort: ReasoningEffort,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummary>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum ResponseImageDetail {
    Low,
    High,
    #[default]
    Auto,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseInputContent {
    InputText {
        text: String,
    },
    OutputText {
        text: String,
    },
    InputImage {
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(default)]
        detail: ResponseImageDetail,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    InProgress,
    Completed,
    Incomplete,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseFunctionOutput {
    Text(String),
    Content(Vec<ResponseInputContent>),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseInputItem {
    Message {
        role: String,
        content: Option<Vec<ResponseInputContent>>,
    },
    FunctionCall {
        call_id: String,
        name: String,
        arguments: String,
        status: Option<ItemStatus>,
        #[serde(default)]
        thought_signature: Option<String>,
    },
    FunctionCallOutput {
        call_id: String,
        output: ResponseFunctionOutput,
        status: Option<ItemStatus>,
    },
    Reasoning(ResponseReasoningInputItem),
}

impl Serialize for ResponseInputItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Message { role, content } => {
                map.serialize_entry("type", "message")?;
                map.serialize_entry("role", role)?;
                if let Some(content) = content {
                    map.serialize_entry("content", content)?;
                }
            }
            Self::FunctionCall {
                call_id,
                name,
                arguments,
                status,
                thought_signature,
            } => {
                map.serialize_entry("type", "function_call")?;
                map.serialize_entry("call_id", call_id)?;
                map.serialize_entry("name", name)?;
                map.serialize_entry("arguments", arguments)?;
                if let Some(status) = status {
                    map.serialize_entry("status", status)?;
                }
                if let Some(thought_signature) = thought_signature {
                    map.serialize_entry("thought_signature", thought_signature)?;
                }
            }
            Self::FunctionCallOutput {
                call_id,
                output,
                status,
            } => {
                map.serialize_entry("type", "function_call_output")?;
                map.serialize_entry("call_id", call_id)?;
                map.serialize_entry("output", output)?;
                if let Some(status) = status {
                    map.serialize_entry("status", status)?;
                }
            }
            Self::Reasoning(reasoning_item) => {
                // Copilot's stateless Responses backend rejects replayed reasoning item IDs,
                // but still needs encrypted content to recover reasoning state.
                map.serialize_entry("type", "reasoning")?;
                map.serialize_entry("summary", &reasoning_item.summary)?;
                if let Some(encrypted_content) = reasoning_item.encrypted_content.as_ref() {
                    map.serialize_entry("encrypted_content", encrypted_content)?;
                }
            }
        }
        map.end()
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IncompleteReason {
    #[serde(rename = "max_output_tokens")]
    MaxOutputTokens,
    #[serde(rename = "content_filter")]
    ContentFilter,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IncompleteDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<IncompleteReason>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ResponseReasoningInputItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub summary: Vec<ResponseReasoningItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ResponseReasoningItem {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum StreamEvent {
    #[serde(rename = "error")]
    GenericError { error: ResponseError },

    #[serde(rename = "response.created")]
    Created { response: Response },

    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        output_index: usize,
        #[serde(default)]
        sequence_number: Option<u64>,
        item: ResponseOutputItem,
    },

    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        item_id: String,
        output_index: usize,
        delta: String,
    },

    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        output_index: usize,
        #[serde(default)]
        sequence_number: Option<u64>,
        item: ResponseOutputItem,
    },

    #[serde(rename = "response.incomplete")]
    Incomplete { response: Response },

    #[serde(rename = "response.completed")]
    Completed { response: Response },

    #[serde(rename = "response.failed")]
    Failed { response: Response },

    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseError {
    pub code: String,
    pub message: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Response {
    pub id: Option<String>,
    pub status: Option<String>,
    pub usage: Option<ResponseUsage>,
    pub output: Vec<ResponseOutputItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseOutputItem {
    Message {
        id: String,
        role: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Vec<ResponseOutputContent>>,
    },
    FunctionCall {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        call_id: String,
        name: String,
        arguments: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<ItemStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thought_signature: Option<String>,
    },
    Reasoning {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        summary: Option<Vec<ResponseReasoningItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        encrypted_content: Option<String>,
    },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseOutputContent {
    OutputText { text: String },
    Refusal { refusal: String },
}

pub async fn stream_response(
    client: Arc<dyn HttpClient>,
    oauth_token: String,
    api_url: String,
    request: Request,
    is_user_initiated: bool,
    location: ChatLocation,
) -> Result<BoxStream<'static, Result<StreamEvent>>> {
    let is_vision_request = request.input.iter().any(|item| match item {
        ResponseInputItem::Message {
            content: Some(parts),
            ..
        } => parts
            .iter()
            .any(|p| matches!(p, ResponseInputContent::InputImage { .. })),
        _ => false,
    });

    let request_builder = copilot_request_headers(
        HttpRequest::builder().method(Method::POST).uri(&api_url),
        &oauth_token,
        Some(is_user_initiated),
        Some(location),
    )
    .when(is_vision_request, |builder| {
        builder.header("Copilot-Vision-Request", "true")
    });

    let is_streaming = request.stream;
    let json = serde_json::to_string(&request)?;
    let request = request_builder.body(AsyncBody::from(json))?;
    let mut response = client.send(request).await?;

    if !response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::bail!("Failed to connect to API: {} {}", response.status(), body);
    }

    if is_streaming {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        if line.starts_with("[DONE]") || line.is_empty() {
                            return None;
                        }

                        match serde_json::from_str::<StreamEvent>(line) {
                            Ok(event) => Some(Ok(event)),
                            Err(error) => {
                                log::error!(
                                    "Failed to parse Copilot responses stream event: `{}`\nResponse: `{}`",
                                    error,
                                    line,
                                );
                                Some(Err(anyhow!(error)))
                            }
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        // Simulate streaming this makes the mapping of this function return more straight-forward to handle if all callers assume it streams.
        // Removes the need of having a method to map StreamEvent and another to map Response to a LanguageCompletionEvent
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => {
                let events = vec![StreamEvent::Created {
                    response: response.clone(),
                }];

                let mut all_events = events;
                for (output_index, item) in response.output.iter().enumerate() {
                    all_events.push(StreamEvent::OutputItemAdded {
                        output_index,
                        sequence_number: None,
                        item: item.clone(),
                    });

                    if let ResponseOutputItem::Message {
                        id,
                        content: Some(content),
                        ..
                    } = item
                    {
                        for part in content {
                            if let ResponseOutputContent::OutputText { text } = part {
                                all_events.push(StreamEvent::OutputTextDelta {
                                    item_id: id.clone(),
                                    output_index,
                                    delta: text.clone(),
                                });
                            }
                        }
                    }

                    all_events.push(StreamEvent::OutputItemDone {
                        output_index,
                        sequence_number: None,
                        item: item.clone(),
                    });
                }

                let final_event = if response.error.is_some() {
                    StreamEvent::Failed { response }
                } else if response.incomplete_details.is_some() {
                    StreamEvent::Incomplete { response }
                } else {
                    StreamEvent::Completed { response }
                };
                all_events.push(final_event);

                Ok(futures::stream::iter(all_events.into_iter().map(Ok)).boxed())
            }
            Err(error) => {
                log::error!(
                    "Failed to parse Copilot non-streaming response: `{}`\nResponse: `{}`",
                    error,
                    body,
                );
                Err(anyhow!(error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_choice_required_serializes_as_required() {
        // Regression test: ToolChoice::Required must serialize as "required" (not "any")
        // for OpenAI Responses API. Reverting the rename would break this.
        assert_eq!(
            serde_json::to_string(&ToolChoice::Required).unwrap(),
            "\"required\""
        );
        assert_eq!(
            serde_json::to_string(&ToolChoice::Auto).unwrap(),
            "\"auto\""
        );
        assert_eq!(
            serde_json::to_string(&ToolChoice::None).unwrap(),
            "\"none\""
        );
    }
}
