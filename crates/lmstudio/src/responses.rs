use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Check if a model name indicates it's a gpt-oss model
pub fn is_gpt_oss_model(model_name: &str) -> bool {
    model_name.to_lowercase().contains("gpt-oss")
}

#[derive(Serialize, Debug)]
pub struct ReasoningConfig {
    pub effort: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    Function {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    #[serde(untagged)]
    Other(ToolDefinition),
}

#[derive(Serialize, Debug)]
pub struct Request {
    pub model: String,
    pub input: Vec<ResponseInputItem>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum ResponseImageDetail {
    Low,
    High,
    #[default]
    Auto,
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
    Error { error: ResponseError },

    #[serde(rename = "response.created")]
    Created { response: Response },

    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        item_id: String,
        output_index: usize,
        delta: String,
    },

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

    #[serde(rename = "response.completed")]
    Completed { response: Response },

    #[serde(rename = "response.incomplete")]
    Incomplete { response: Response },

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

#[derive(Deserialize, Debug, Clone, Default)]
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

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ResponseUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IncompleteDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
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
        status: Option<String>,
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

/// Stream responses from /v1/responses endpoint
/// Supports both streaming and non-streaming modes
pub async fn stream_response(
    client: &dyn HttpClient,
    api_url: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<StreamEvent>>> {
    // Construct URL: handle /api/v0 suffix and build /v1/responses endpoint
    let base_url = api_url
        .trim_end_matches('/')
        .trim_end_matches("/api/v0")
        .trim_end_matches("/api");
    let url = format!("{}/v1/responses", base_url);

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(&url)
        .header("Content-Type", "application/json");

    let is_streaming = request.stream;
    let json = serde_json::to_string(&request)?;
    
    log::debug!("LM Studio /v1/responses request URL: {}", url);
    log::debug!("LM Studio /v1/responses request body: {}", json);
    
    let request = request_builder.body(AsyncBody::from(json))?;
    let mut response = client.send(request).await?;

    if !response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::bail!("Failed to connect to LM Studio /v1/responses API: {} {}", response.status(), body);
    }

    if is_streaming {
        // Streaming mode: Parse SSE stream
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        if line == "[DONE]" || line.is_empty() {
                            return None;
                        }

                        match serde_json::from_str::<StreamEvent>(line) {
                            Ok(event) => Some(Ok(event)),
                            Err(error) => {
                                log::error!(
                                    "Failed to parse LM Studio /v1/responses stream event: `{}`\nResponse: `{}`",
                                    error,
                                    line,
                                );
                                Some(Err(anyhow!("Failed to parse stream event: {}", error)))
                            }
                        }
                    }
                    Err(error) => {
                        // Handle EOF gracefully - connection may have closed normally
                        // This can happen when the server closes the connection after sending all data
                        let error_msg = error.to_string();
                        if error_msg.contains("EOF") 
                            || error_msg.contains("unexpected end of file")
                            || error_msg.contains("unexpected EOF")
                            || error_msg.contains("chunk size line") {
                            log::debug!("LM Studio /v1/responses stream ended normally: {}", error_msg);
                            None
                        } else {
                            log::error!("LM Studio /v1/responses stream read error: {}", error_msg);
                            Some(Err(anyhow!("Stream read error: {}", error)))
                        }
                    }
                }
            })
            .boxed())
    } else {
        // Non-streaming mode: Convert Response to StreamEvents
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => {
                let mut all_events = vec![StreamEvent::Created {
                    response: response.clone(),
                }];

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
                    "Failed to parse LM Studio non-streaming response: `{}`\nResponse: `{}`",
                    error,
                    body,
                );
                Err(anyhow!("Failed to parse non-streaming response: {}", error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_gpt_oss_model() {
        assert!(is_gpt_oss_model("gpt-oss-1"));
        assert!(is_gpt_oss_model("gpt-oss-2.5"));
        assert!(is_gpt_oss_model("GPT-OSS-3"));
        assert!(is_gpt_oss_model("some-prefix-gpt-oss-suffix"));
        assert!(!is_gpt_oss_model("gpt-4"));
        assert!(!is_gpt_oss_model("gpt-3.5-turbo"));
        assert!(!is_gpt_oss_model("claude-3"));
        assert!(!is_gpt_oss_model(""));
    }
}

