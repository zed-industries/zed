use anyhow::{Result, anyhow};
use futures::{
    AsyncBufReadExt, AsyncReadExt,
    io::BufReader,
    stream::{BoxStream, StreamExt},
};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

pub const MINIMAX_API_URL_CHINA: &str = "https://api.minimaxi.com/v1";
pub const MINIMAX_API_URL_INTERNATIONAL: &str = "https://api.minimax.io/v1";

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
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Model {
    #[serde(rename = "MiniMax-M2.1")]
    #[default]
    M2_1,
    #[serde(rename = "MiniMax-M2.1-lightning")]
    M2_1Lightning,
    #[serde(rename = "MiniMax-M2")]
    M2,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Model::M2_1Lightning
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "MiniMax-M2.1" => Ok(Self::M2_1),
            "MiniMax-M2.1-lightning" => Ok(Self::M2_1Lightning),
            "MiniMax-M2" => Ok(Self::M2),
            _ => anyhow::bail!("invalid model id {id}"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::M2_1 => "MiniMax-M2.1",
            Self::M2_1Lightning => "MiniMax-M2.1-lightning",
            Self::M2 => "MiniMax-M2",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::M2_1 => "MiniMax M2.1",
            Self::M2_1Lightning => "MiniMax M2.1 Lightning",
            Self::M2 => "MiniMax M2",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name).as_str(),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            // From MiniMax documentation: M2.1 supports 128K context
            Self::M2_1 | Self::M2_1Lightning | Self::M2 => 128_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            // Using None to let API decide
            Self::M2_1 | Self::M2_1Lightning | Self::M2 => None,
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    pub reasoning_split: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    #[serde(rename = "json_object")]
    JsonObject,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        content: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reasoning_details: Option<Vec<ReasoningDetail>>,
    },
    User {
        content: String,
    },
    System {
        content: String,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ReasoningDetail {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
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
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_details: Option<Vec<ReasoningDetail>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<u64>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDelta {
    pub role: Option<Role>,
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_details: Option<Vec<ReasoningDetail>>,
}
impl std::fmt::Display for StreamDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 优先显示内容，如果有思考标签则标注
        if let Some(content) = &self.content {
            if content.contains("<think>") {
                write!(f, "[contains thinking] ")?;
                // 显示简化的内容预览
                if content.len() > 100 {
                    write!(f, "\"{}\"...", &content[..97].replace("\n", "\\n"))?;
                } else {
                    write!(f, "\"{}\"", content.replace("\n", "\\n"))?;
                }
            } else if !content.trim().is_empty() {
                write!(f, "{}", content)?;
            }
        }

        // 显示其他字段信息
        let mut has_previous = self
            .content
            .as_ref()
            .map(|c| !c.trim().is_empty())
            .unwrap_or(false);

        if let Some(role) = &self.role {
            if has_previous {
                write!(f, " ")?;
            }
            let role_str = match role {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
                Role::Tool => "tool",
            };
            write!(f, "[role: {}]", role_str)?;
            has_previous = true;
        }

        if let Some(tool_calls) = &self.tool_calls {
            if !tool_calls.is_empty() {
                if has_previous {
                    write!(f, " ")?;
                }
                write!(f, "[{} tool call(s)]", tool_calls.len())?;
                has_previous = true;
            }
        }

        if let Some(reasoning_details) = &self.reasoning_details {
            if !reasoning_details.is_empty() {
                if has_previous {
                    write!(f, " ")?;
                }
                write!(f, "[{} reasoning detail(s)]", reasoning_details.len())?;
            }
        }

        // 如果什么都没有，至少显示一些信息
        if !has_previous && self.content.is_none() {
            write!(f, "[empty delta]")?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug)]
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
            "Failed to connect to MiniMax API: {} {}",
            response.status(),
            body,
        );
    }
}
