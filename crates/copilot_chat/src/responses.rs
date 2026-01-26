use std::sync::Arc;

use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
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
    Any,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseInputItem {
    Message {
        role: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Vec<ResponseInputContent>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<String>,
    },
    FunctionCall {
        call_id: String,
        name: String,
        arguments: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<ItemStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thought_signature: Option<String>,
    },
    FunctionCallOutput {
        call_id: String,
        output: ResponseFunctionOutput,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<ItemStatus>,
    },
    Reasoning {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        summary: Vec<ResponseReasoningItem>,
        encrypted_content: String,
    },
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    api_key: String,
    api_url: String,
    request: Request,
    is_user_initiated: bool,
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

    let request_initiator = if is_user_initiated { "user" } else { "agent" };

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(&api_url)
        .header(
            "Editor-Version",
            format!(
                "Zed/{}",
                option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
            ),
        )
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("Copilot-Integration-Id", "vscode-chat")
        .header("X-Initiator", request_initiator);

    let request_builder = if is_vision_request {
        request_builder.header("Copilot-Vision-Request", "true")
    } else {
        request_builder
    };

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
