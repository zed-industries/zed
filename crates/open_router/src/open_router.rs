use anyhow::{Context, Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

pub const OPEN_ROUTER_API_URL: &str = "https://openrouter.ai/api/v1";

fn is_none_or_empty<T: AsRef<[U]>, U>(opt: &Option<T>) -> bool {
    opt.as_ref().is_none_or(|v| v.as_ref().is_empty())
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

impl TryFrom<String> for Role {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            "tool" => Ok(Self::Tool),
            _ => Err(anyhow!("invalid role '{value}'")),
        }
    }
}

impl From<Role> for String {
    fn from(val: Role) -> Self {
        match val {
            Role::User => "user".to_owned(),
            Role::Assistant => "assistant".to_owned(),
            Role::System => "system".to_owned(),
            Role::Tool => "tool".to_owned(),
        }
    }
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: u64,
    pub supports_tools: Option<bool>,
    pub supports_images: Option<bool>,
    #[serde(default)]
    pub mode: ModelMode,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        budget_tokens: Option<u32>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::new(
            "openrouter/auto",
            Some("Auto Router"),
            Some(2000000),
            Some(true),
            Some(false),
            Some(ModelMode::Default),
        )
    }

    pub fn default() -> Self {
        Self::default_fast()
    }

    pub fn new(
        name: &str,
        display_name: Option<&str>,
        max_tokens: Option<u64>,
        supports_tools: Option<bool>,
        supports_images: Option<bool>,
        mode: Option<ModelMode>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            display_name: display_name.map(|s| s.to_owned()),
            max_tokens: max_tokens.unwrap_or(2000000),
            supports_tools,
            supports_images,
            mode: mode.unwrap_or(ModelMode::Default),
        }
    }

    pub fn id(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_ref().unwrap_or(&self.name)
    }

    pub fn max_token_count(&self) -> u64 {
        self.max_tokens
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        None
    }

    pub fn supports_tool_calls(&self) -> bool {
        self.supports_tools.unwrap_or(false)
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop: Vec<String>,
    pub temperature: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
    pub usage: RequestUsage,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestUsage {
    pub include: bool,
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

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    #[allow(dead_code)]
    Function { function: FunctionDefinition },
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reasoning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        content: Option<MessageContent>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    User {
        content: MessageContent,
    },
    System {
        content: MessageContent,
    },
    Tool {
        content: MessageContent,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    Plain(String),
    Multipart(Vec<MessagePart>),
}

impl MessageContent {
    pub fn empty() -> Self {
        Self::Plain(String::new())
    }

    pub fn push_part(&mut self, part: MessagePart) {
        match self {
            Self::Plain(text) if text.is_empty() => {
                *self = Self::Multipart(vec![part]);
            }
            Self::Plain(text) => {
                let text_part = MessagePart::Text {
                    text: std::mem::take(text),
                };
                *self = Self::Multipart(vec![text_part, part]);
            }
            Self::Multipart(parts) => parts.push(part),
        }
    }
}

impl From<Vec<MessagePart>> for MessageContent {
    fn from(parts: Vec<MessagePart>) -> Self {
        if parts.len() == 1
            && let MessagePart::Text { text } = &parts[0]
        {
            return Self::Plain(text.clone());
        }
        Self::Multipart(parts)
    }
}

impl From<String> for MessageContent {
    fn from(text: String) -> Self {
        Self::Plain(text)
    }
}

impl From<&str> for MessageContent {
    fn from(text: &str) -> Self {
        Self::Plain(text.to_string())
    }
}

impl MessageContent {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Plain(text) => Some(text),
            Self::Multipart(parts) if parts.len() == 1 => {
                if let MessagePart::Text { text } = &parts[0] {
                    Some(text)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn to_text(&self) -> String {
        match self {
            Self::Plain(text) => text.clone(),
            Self::Multipart(parts) => parts
                .iter()
                .filter_map(|part| {
                    if let MessagePart::Text { text } = part {
                        Some(text.as_str())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessagePart {
    Text {
        text: String,
    },
    #[serde(rename = "image_url")]
    Image {
        image_url: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCall {
    pub id: String,
    #[serde(flatten)]
    pub content: ToolCallContent,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolCallContent {
    Function { function: FunctionContent },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionContent {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseMessageDelta {
    pub role: Option<Role>,
    pub content: Option<String>,
    pub reasoning: Option<String>,
    #[serde(default, skip_serializing_if = "is_none_or_empty")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessageDelta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: RequestMessage,
    pub finish_reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListModelsResponse {
    pub data: Vec<ModelEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModelEntry {
    pub id: String,
    pub name: String,
    pub created: usize,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_length: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_parameters: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<ModelArchitecture>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModelArchitecture {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_modalities: Vec<String>,
}

pub async fn complete(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<Response> {
    let uri = format!("{api_url}/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .header("HTTP-Referer", "https://zed.dev")
        .header("X-Title", "Zed Editor");

    let mut request_body = request;
    request_body.stream = false;

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request_body)?))?;
    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        let response: Response = serde_json::from_str(&body)?;
        Ok(response)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct OpenRouterResponse {
            error: OpenRouterError,
        }

        #[derive(Deserialize)]
        struct OpenRouterError {
            message: String,
            #[serde(default)]
            code: String,
        }

        match serde_json::from_str::<OpenRouterResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => {
                let error_message = if !response.error.code.is_empty() {
                    format!("{}: {}", response.error.code, response.error.message)
                } else {
                    response.error.message
                };

                Err(anyhow!(
                    "Failed to connect to OpenRouter API: {}",
                    error_message
                ))
            }
            _ => Err(anyhow!(
                "Failed to connect to OpenRouter API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>> {
    let uri = format!("{api_url}/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "https://zed.dev")
        .header("X-Title", "Zed Editor");

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        if line.starts_with(':') {
                            return None;
                        }

                        let line = line.strip_prefix("data: ")?;
                        if line == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str::<ResponseStreamEvent>(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => {
                                    #[derive(Deserialize)]
                                    struct ErrorResponse {
                                        error: String,
                                    }

                                    match serde_json::from_str::<ErrorResponse>(line) {
                                        Ok(err_response) => Some(Err(anyhow!(err_response.error))),
                                        Err(_) => {
                                            if line.trim().is_empty() {
                                                None
                                            } else {
                                                Some(Err(anyhow!(
                                                    "Failed to parse response: {}. Original content: '{}'",
                                                    error, line
                                                )))
                                            }
                                        }
                                    }
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
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct OpenRouterResponse {
            error: OpenRouterError,
        }

        #[derive(Deserialize)]
        struct OpenRouterError {
            message: String,
            #[serde(default)]
            code: String,
        }

        match serde_json::from_str::<OpenRouterResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => {
                let error_message = if !response.error.code.is_empty() {
                    format!("{}: {}", response.error.code, response.error.message)
                } else {
                    response.error.message
                };

                Err(anyhow!(
                    "Failed to connect to OpenRouter API: {}",
                    error_message
                ))
            }
            _ => Err(anyhow!(
                "Failed to connect to OpenRouter API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}

pub async fn list_models(client: &dyn HttpClient, api_url: &str) -> Result<Vec<Model>> {
    let uri = format!("{api_url}/models");
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");

    let request = request_builder.body(AsyncBody::default())?;
    let mut response = client.send(request).await?;

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    if response.status().is_success() {
        let response: ListModelsResponse =
            serde_json::from_str(&body).context("Unable to parse OpenRouter models response")?;

        let models = response
            .data
            .into_iter()
            .map(|entry| Model {
                name: entry.id,
                // OpenRouter returns display names in the format "provider_name: model_name".
                // When displayed in the UI, these names can get truncated from the right.
                // Since users typically already know the provider, we extract just the model name
                // portion (after the colon) to create a more concise and user-friendly label
                // for the model dropdown in the agent panel.
                display_name: Some(
                    entry
                        .name
                        .split(':')
                        .next_back()
                        .unwrap_or(&entry.name)
                        .trim()
                        .to_string(),
                ),
                max_tokens: entry.context_length.unwrap_or(2000000),
                supports_tools: Some(entry.supported_parameters.contains(&"tools".to_string())),
                supports_images: Some(
                    entry
                        .architecture
                        .as_ref()
                        .map(|arch| arch.input_modalities.contains(&"image".to_string()))
                        .unwrap_or(false),
                ),
                mode: if entry
                    .supported_parameters
                    .contains(&"reasoning".to_string())
                {
                    ModelMode::Thinking {
                        budget_tokens: Some(4_096),
                    }
                } else {
                    ModelMode::Default
                },
            })
            .collect();

        Ok(models)
    } else {
        Err(anyhow!(
            "Failed to connect to OpenRouter API: {} {}",
            response.status(),
            body,
        ))
    }
}
