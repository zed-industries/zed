use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;
use strum::EnumIter;

pub const MISTRAL_API_URL: &str = "https://api.mistral.ai/v1";

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
            _ => anyhow::bail!("invalid role '{value}'"),
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
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(rename = "codestral-latest", alias = "codestral-latest")]
    #[default]
    CodestralLatest,

    #[serde(rename = "mistral-large-latest", alias = "mistral-large-latest")]
    MistralLargeLatest,
    #[serde(rename = "mistral-medium-latest", alias = "mistral-medium-latest")]
    MistralMediumLatest,
    #[serde(rename = "mistral-small-latest", alias = "mistral-small-latest")]
    MistralSmallLatest,

    #[serde(rename = "magistral-medium-latest", alias = "magistral-medium-latest")]
    MagistralMediumLatest,
    #[serde(rename = "magistral-small-latest", alias = "magistral-small-latest")]
    MagistralSmallLatest,

    #[serde(rename = "open-mistral-nemo", alias = "open-mistral-nemo")]
    OpenMistralNemo,
    #[serde(rename = "open-codestral-mamba", alias = "open-codestral-mamba")]
    OpenCodestralMamba,

    #[serde(rename = "devstral-medium-latest", alias = "devstral-medium-latest")]
    DevstralMediumLatest,
    #[serde(rename = "devstral-small-latest", alias = "devstral-small-latest")]
    DevstralSmallLatest,

    #[serde(rename = "pixtral-12b-latest", alias = "pixtral-12b-latest")]
    Pixtral12BLatest,
    #[serde(rename = "pixtral-large-latest", alias = "pixtral-large-latest")]
    PixtralLargeLatest,

    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
        supports_tools: Option<bool>,
        supports_images: Option<bool>,
        supports_thinking: Option<bool>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Model::MistralSmallLatest
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "codestral-latest" => Ok(Self::CodestralLatest),
            "mistral-large-latest" => Ok(Self::MistralLargeLatest),
            "mistral-medium-latest" => Ok(Self::MistralMediumLatest),
            "mistral-small-latest" => Ok(Self::MistralSmallLatest),
            "magistral-medium-latest" => Ok(Self::MagistralMediumLatest),
            "magistral-small-latest" => Ok(Self::MagistralSmallLatest),
            "open-mistral-nemo" => Ok(Self::OpenMistralNemo),
            "open-codestral-mamba" => Ok(Self::OpenCodestralMamba),
            "devstral-medium-latest" => Ok(Self::DevstralMediumLatest),
            "devstral-small-latest" => Ok(Self::DevstralSmallLatest),
            "pixtral-12b-latest" => Ok(Self::Pixtral12BLatest),
            "pixtral-large-latest" => Ok(Self::PixtralLargeLatest),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::CodestralLatest => "codestral-latest",
            Self::MistralLargeLatest => "mistral-large-latest",
            Self::MistralMediumLatest => "mistral-medium-latest",
            Self::MistralSmallLatest => "mistral-small-latest",
            Self::MagistralMediumLatest => "magistral-medium-latest",
            Self::MagistralSmallLatest => "magistral-small-latest",
            Self::OpenMistralNemo => "open-mistral-nemo",
            Self::OpenCodestralMamba => "open-codestral-mamba",
            Self::DevstralMediumLatest => "devstral-medium-latest",
            Self::DevstralSmallLatest => "devstral-small-latest",
            Self::Pixtral12BLatest => "pixtral-12b-latest",
            Self::PixtralLargeLatest => "pixtral-large-latest",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::CodestralLatest => "codestral-latest",
            Self::MistralLargeLatest => "mistral-large-latest",
            Self::MistralMediumLatest => "mistral-medium-latest",
            Self::MistralSmallLatest => "mistral-small-latest",
            Self::MagistralMediumLatest => "magistral-medium-latest",
            Self::MagistralSmallLatest => "magistral-small-latest",
            Self::OpenMistralNemo => "open-mistral-nemo",
            Self::OpenCodestralMamba => "open-codestral-mamba",
            Self::DevstralMediumLatest => "devstral-medium-latest",
            Self::DevstralSmallLatest => "devstral-small-latest",
            Self::Pixtral12BLatest => "pixtral-12b-latest",
            Self::PixtralLargeLatest => "pixtral-large-latest",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::CodestralLatest => 256000,
            Self::MistralLargeLatest => 131000,
            Self::MistralMediumLatest => 128000,
            Self::MistralSmallLatest => 32000,
            Self::MagistralMediumLatest => 40000,
            Self::MagistralSmallLatest => 40000,
            Self::OpenMistralNemo => 131000,
            Self::OpenCodestralMamba => 256000,
            Self::DevstralMediumLatest => 128000,
            Self::DevstralSmallLatest => 262144,
            Self::Pixtral12BLatest => 128000,
            Self::PixtralLargeLatest => 128000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            _ => None,
        }
    }

    pub fn supports_tools(&self) -> bool {
        match self {
            Self::CodestralLatest
            | Self::MistralLargeLatest
            | Self::MistralMediumLatest
            | Self::MistralSmallLatest
            | Self::MagistralMediumLatest
            | Self::MagistralSmallLatest
            | Self::OpenMistralNemo
            | Self::OpenCodestralMamba
            | Self::DevstralMediumLatest
            | Self::DevstralSmallLatest
            | Self::Pixtral12BLatest
            | Self::PixtralLargeLatest => true,
            Self::Custom { supports_tools, .. } => supports_tools.unwrap_or(false),
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::Pixtral12BLatest
            | Self::PixtralLargeLatest
            | Self::MistralMediumLatest
            | Self::MistralSmallLatest => true,
            Self::CodestralLatest
            | Self::MistralLargeLatest
            | Self::MagistralMediumLatest
            | Self::MagistralSmallLatest
            | Self::OpenMistralNemo
            | Self::OpenCodestralMamba
            | Self::DevstralMediumLatest
            | Self::DevstralSmallLatest => false,
            Self::Custom {
                supports_images, ..
            } => supports_images.unwrap_or(false),
        }
    }

    pub fn supports_thinking(&self) -> bool {
        match self {
            Self::MagistralMediumLatest | Self::MagistralSmallLatest => true,
            Self::Custom {
                supports_thinking, ..
            } => supports_thinking.unwrap_or(false),
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    #[serde(rename = "json_object")]
    JsonObject,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    Function { function: FunctionDefinition },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: u32,
    pub temperature: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prediction: Option<Prediction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite_speculation: Option<bool>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Prediction {
    Content { content: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    Any,
    Function(ToolDefinition),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        #[serde(flatten)]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        content: Option<MessageContent>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    User {
        #[serde(flatten)]
        content: MessageContent,
    },
    System {
        #[serde(flatten)]
        content: MessageContent,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    #[serde(rename = "content")]
    Plain { content: String },
    #[serde(rename = "content")]
    Multipart { content: Vec<MessagePart> },
}

impl MessageContent {
    pub fn empty() -> Self {
        Self::Plain {
            content: String::new(),
        }
    }

    pub fn push_part(&mut self, part: MessagePart) {
        match self {
            Self::Plain { content } => match part {
                MessagePart::Text { text } => {
                    content.push_str(&text);
                }
                part => {
                    let mut parts = if content.is_empty() {
                        Vec::new()
                    } else {
                        vec![MessagePart::Text {
                            text: content.clone(),
                        }]
                    };
                    parts.push(part);
                    *self = Self::Multipart { content: parts };
                }
            },
            Self::Multipart { content } => {
                content.push(part);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessagePart {
    Text { text: String },
    ImageUrl { image_url: String },
    Thinking { thinking: Vec<ThinkingPart> },
}

// Backwards-compatibility alias for provider code that refers to ContentPart
pub type ContentPart = MessagePart;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ThinkingPart {
    Text { text: String },
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionChoice {
    pub text: String,
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
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: RequestMessage,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamChoice {
    pub index: u32,
    pub delta: StreamDelta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamDelta {
    pub role: Option<Role>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<MessageContentDelta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum MessageContentDelta {
    Text(String),
    Parts(Vec<MessagePart>),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<StreamResponse>>> {
    let uri = format!("{api_url}/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()));

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;

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
                            match serde_json::from_str(line) {
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
        anyhow::bail!(
            "Failed to connect to Mistral API: {} {}",
            response.status(),
            body,
        );
    }
}
