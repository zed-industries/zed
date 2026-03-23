use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ReasoningEffort, RequestError, Role, ToolChoice};

#[derive(Serialize, Debug)]
pub struct Request {
    pub model: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<ResponseInputItem>,
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseInputItem {
    Message(ResponseMessageItem),
    FunctionCall(ResponseFunctionCallItem),
    FunctionCallOutput(ResponseFunctionCallOutputItem),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessageItem {
    pub role: Role,
    pub content: Vec<ResponseInputContent>,
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
    pub output: String,
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
}

#[derive(Serialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Error {
    pub message: String,
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
    Error { error: Error },
    #[serde(rename = "error")]
    GenericError { error: Error },
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
    pub status_details: Option<ResponseStatusDetails>,
    #[serde(default)]
    pub usage: Option<ResponseUsage>,
    #[serde(default)]
    pub output: Vec<ResponseOutputItem>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseStatusDetails {
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub error: Option<Value>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseUsage {
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub output_tokens: Option<u64>,
    #[serde(default)]
    pub total_tokens: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseOutputItem {
    Message(ResponseOutputMessage),
    FunctionCall(ResponseFunctionToolCall),
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

pub async fn stream_response(
    client: &dyn HttpClient,
    provider_name: &str,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<StreamEvent>>, RequestError> {
    let uri = format!("{api_url}/responses");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()));

    let is_streaming = request.stream;
    let request = request_builder
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
                            ResponseOutputItem::Unknown => {}
                        }

                        all_events.push(StreamEvent::OutputItemDone {
                            output_index,
                            sequence_number: None,
                            item: item.clone(),
                        });
                    }

                    all_events.push(StreamEvent::Completed {
                        response: response_summary,
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
