use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

pub const PROVIDER_ID: &str = "x_ai";
pub const PROVIDER_NAME: &str = "xAI";
pub const API_URL: &str = "https://api.x.ai/v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "grok-3")]
    Grok3,
    #[serde(rename = "grok-3-mini-fast")]
    Grok3MiniFast,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        display_name: Option<String>,
        max_tokens: usize,
        max_output_tokens: Option<usize>,
        max_completion_tokens: Option<usize>,
    },
}

impl Model {
    pub fn id(&self) -> &str {
        match self {
            Model::Grok3 => "grok-3",
            Model::Grok3MiniFast => "grok-3-mini-fast",
            Model::Custom { name, .. } => name,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Model::Grok3 => "Grok 3",
            Model::Grok3MiniFast => "Grok 3 Mini Fast",
            Model::Custom {
                display_name, name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn display_name(&self) -> &str {
        self.name()
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Model::Grok3 => 2_000_000,
            Model::Grok3MiniFast => 1_000_000,
            Model::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_tokens(&self) -> usize {
        self.max_token_count()
    }

    pub fn max_output_tokens(&self) -> Option<usize> {
        Some(4096)
    }

    pub fn from_id(id: &str) -> anyhow::Result<Self> {
        match id {
            "grok-3" => Ok(Self::Grok3),
            "grok-3-mini-fast" => Ok(Self::Grok3MiniFast),
            _ => Err(anyhow::anyhow!("invalid model id: {}", id)),
        }
    }

    pub fn default_fast() -> Self {
        Self::Grok3MiniFast
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Model::Grok3 => true,
            Model::Grok3MiniFast => true,
            Model::Custom { .. } => false,
        }
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Clone)]
pub struct XaiApiClient {
    http_client: std::sync::Arc<dyn http_client::HttpClient>,
    api_url: String,
    api_key: String,
}

impl XaiApiClient {
    pub fn new(
        http_client: std::sync::Arc<dyn http_client::HttpClient>,
        api_url: &str,
        api_key: &str,
    ) -> Result<Self> {
        Ok(Self {
            http_client,
            api_url: api_url.to_string(),
            api_key: api_key.to_string(),
        })
    }

    pub async fn send_chat_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: Option<f32>,
        max_completion_tokens: Option<u32>,
        stream: bool,
    ) -> Result<Response> {
        complete(
            &*self.http_client,
            &self.api_url,
            &self.api_key,
            Request {
                messages: messages
                    .into_iter()
                    .map(|msg| match msg {
                        Message::System { content, .. } => RequestMessage {
                            role: Role::System,
                            content: match content {
                                MessageContent::Text(text) => text,
                                MessageContent::Parts(_) => "".to_string(), // Simplified
                            },
                            tool_calls: None,
                            tool_call_id: None,
                        },
                        Message::User { content, .. } => RequestMessage {
                            role: Role::User,
                            content: match content {
                                MessageContent::Text(text) => text,
                                MessageContent::Parts(_) => "".to_string(), // Simplified
                            },
                            tool_calls: None,
                            tool_call_id: None,
                        },
                        Message::Assistant { content, .. } => RequestMessage {
                            role: Role::Assistant,
                            content: content
                                .map(|c| match c {
                                    MessageContent::Text(text) => text,
                                    MessageContent::Parts(_) => "".to_string(), // Simplified
                                })
                                .unwrap_or_default(),
                            tool_calls: None,
                            tool_call_id: None,
                        },
                        Message::Tool {
                            content,
                            tool_call_id,
                        } => RequestMessage {
                            role: Role::Tool,
                            content,
                            tool_calls: None,
                            tool_call_id: Some(tool_call_id),
                        },
                    })
                    .collect(),
                model: model.to_string(),
                stream,
                max_tokens: max_completion_tokens,
                temperature,
                top_p: None,
                frequency_penalty: None,
                presence_penalty: None,
                stop: None,
                tools: None,
                tool_choice: None,
            },
        )
        .await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "role")]
pub enum Message {
    #[serde(rename = "system")]
    System {
        content: MessageContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "user")]
    User {
        content: MessageContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "assistant")]
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<MessageContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    #[serde(rename = "tool")]
    Tool {
        content: String,
        tool_call_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrlObject },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrlObject {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<Message>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<std::collections::HashMap<String, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: serde_json::Value },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamOptions {
    pub include_usage: bool,
}

#[derive(Debug, Serialize)]
pub struct Request {
    pub messages: Vec<RequestMessage>,
    pub model: String,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestMessage {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolFunction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    String(String),
    Object {
        #[serde(rename = "type")]
        tool_type: String,
        function: ToolChoiceFunction,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolChoiceFunction {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolCallFunction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: ResponseMessage,
    pub delta: Option<ResponseMessage>,
    pub finish_reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamingChoice {
    pub index: u32,
    pub delta: ResponseMessage,
    pub finish_reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamingResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamingChoice>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

pub async fn stream_completion(
    http_client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<StreamingResponse>>> {
    let uri = format!("{}/chat/completions", api_url);

    let http_request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(AsyncBody::from(serde_json::to_string(&request)?))?;

    let mut response = http_client.send(http_request).await?;

    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        if line == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str::<StreamingResponse>(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => Some(Err(anyhow!(error))),
                            }
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        Err(anyhow!("HTTP error {}: {}", response.status(), body))
    }
}

pub async fn complete(
    http_client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<Response> {
    let uri = format!("{}/chat/completions", api_url);
    println!("Request URI: {}", uri);
    let body = serde_json::to_string(&request)?;
    println!("Request body: {}", body);

    let http_request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(AsyncBody::from(body))?;

    let mut response = http_client.send(http_request).await?;

    if response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        let response: Response = serde_json::from_str(&body)?;
        Ok(response)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        println!("Response status: {}, body: {}", response.status(), body);
        Err(anyhow!("HTTP error {}: {}", response.status(), body))
    }
}

pub fn count_tokens(text: &str) -> usize {
    // Simple token counting approximation
    // In a real implementation, you might want to use a proper tokenizer
    text.split_whitespace().count() + text.chars().filter(|c| c.is_ascii_punctuation()).count()
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for Model {
    fn schema_name() -> String {
        "XaiModel".to_string()
    }

    fn json_schema(r#gen: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::JsonSchema;

        #[derive(JsonSchema)]
        #[allow(dead_code)]
        struct XaiModel {
            id: String,
            name: String,
            max_tokens: usize,
            max_output_tokens: Option<usize>,
        }

        XaiModel::json_schema(r#gen)
    }
}
