use anyhow::{anyhow, Result};
use futures::{
    io::BufReader,
    stream::{BoxStream, StreamExt},
    AsyncBufReadExt, AsyncReadExt,
};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

pub const DEEPSEEK_API_URL: &str = "https://api.deepseek.com";

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
pub enum Model {
    #[serde(rename = "deepseek-chat")]
    #[default]
    Chat,
    #[serde(rename = "deepseek-reasoner")]
    Reasoner,
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "deepseek-chat" => Ok(Self::Chat),
            "deepseek-reasoner" => Ok(Self::Reasoner),
            _ => Err(anyhow!("invalid model id")),
        }
    }

    pub const fn id(&self) -> &str {
        match self {
            Self::Chat => "deepseek-chat",
            Self::Reasoner => "deepseek-reasoner",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Chat => "DeepSeek Chat",
            Self::Reasoner => "DeepSeek Reasoner",
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Chat | Self::Reasoner => 64_000,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u32> {
        match self {
            Self::Chat => Some(8_192),
            Self::Reasoner => Some(8_192),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        content: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
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
    pub reasoning_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    #[serde(default)]
    pub prompt_cache_hit_tokens: u32,
    #[serde(default)]
    pub prompt_cache_miss_tokens: u32,
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
    pub reasoning_content: Option<String>,
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
pub async fn complete(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    mut request: Request,
) -> Result<Response> {
    if &request.model == Model::Reasoner.id() {
        request.messages = merge_consecutive_messages(request.messages)?;
    }

    let uri = format!("{api_url}/v1/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

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

        Err(anyhow!(
            "Failed to connect to DeepSeek API: {} {}",
            response.status(),
            body,
        ))
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    mut request: Request,
) -> Result<BoxStream<'static, Result<StreamResponse>>> {
    if &request.model == Model::Reasoner.id() {
        request.messages = merge_consecutive_messages(request.messages)?;
    }

    let uri = format!("{api_url}/v1/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

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
        Err(anyhow!(
            "Failed to connect to DeepSeek API: {} {}",
            response.status(),
            body,
        ))
    }
}

fn merge_consecutive_messages(messages: Vec<RequestMessage>) -> Result<Vec<RequestMessage>> {
    if messages.len() < 2 {
        return Ok(messages);
    }

    let mut messages = messages.into_iter().peekable();
    let mut merged = Vec::new();

    while let Some(current_msg) = messages.next() {
        let mut merged_msg = current_msg;
        while let Some(next_msg) = messages.peek() {
            if can_merge(&merged_msg, next_msg) {
                merged_msg = merge_messages(merged_msg, messages.next().unwrap())?;
            } else {
                break;
            }
        }
        merged.push(merged_msg);
    }

    Ok(merged)
}

fn merge_messages(a: RequestMessage, b: RequestMessage) -> Result<RequestMessage> {
    Ok(match (a, b) {
        (
            RequestMessage::User { content: a_content },
            RequestMessage::User { content: b_content },
        ) => {
            let mut content = String::with_capacity(a_content.len() + b_content.len() + 1);
            content.push_str(&a_content);
            content.push(' ');
            content.push_str(&b_content);
            RequestMessage::User { content }
        }
        (
            RequestMessage::Assistant {
                content: a_content,
                tool_calls: mut a_tools,
            },
            RequestMessage::Assistant {
                content: b_content,
                tool_calls: b_tools,
            },
        ) => {
            let merged_content = match (a_content, b_content) {
                (Some(a), Some(b)) => {
                    let mut merged = String::with_capacity(a.len() + b.len() + 1);
                    merged.push_str(&a);
                    merged.push(' ');
                    merged.push_str(&b);
                    Some(merged)
                }
                (a, b) => a.or(b),
            };
            a_tools.reserve(b_tools.len());
            a_tools.extend(b_tools);
            RequestMessage::Assistant {
                content: merged_content,
                tool_calls: a_tools,
            }
        }
        _ => unreachable!(),
    })
}

#[inline]
fn can_merge(a: &RequestMessage, b: &RequestMessage) -> bool {
    match (a, b) {
        (RequestMessage::User { .. }, RequestMessage::User { .. }) => true,
        (RequestMessage::Assistant { .. }, RequestMessage::Assistant { .. }) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_message(content: &str) -> RequestMessage {
        RequestMessage::User {
            content: content.to_string(),
        }
    }

    fn system_message(content: &str) -> RequestMessage {
        RequestMessage::System {
            content: content.to_string(),
        }
    }

    fn assistant_message(content: Option<&str>) -> RequestMessage {
        RequestMessage::Assistant {
            content: content.map(|s| s.to_string()),
            tool_calls: vec![],
        }
    }

    fn tool_message(content: &str, id: &str) -> RequestMessage {
        RequestMessage::Tool {
            content: content.to_string(),
            tool_call_id: id.to_string(),
        }
    }

    #[test]
    fn test_no_consecutive_messages() {
        let messages = vec![
            user_message("Hello"),
            system_message("System prompt"),
            user_message("How are you?"),
        ];

        let merged = merge_consecutive_messages(messages).unwrap();
        assert_eq!(merged.len(), 3);
    }

    #[test]
    fn test_merge_user_messages() {
        let messages = vec![
            user_message("Hello"),
            user_message("How are you?"),
            system_message("System"),
            user_message("Another question"),
            user_message("And another"),
        ];

        let merged = merge_consecutive_messages(messages).unwrap();
        assert_eq!(merged.len(), 3);
        assert_eq!(merged[0], user_message("Hello How are you?"));
        assert_eq!(merged[2], user_message("Another question And another"));
    }

    #[test]
    fn test_merge_system_messages() {
        let messages = vec![
            system_message("System 1"),
            system_message("System 2"),
            user_message("User message"),
            system_message("System 3"),
        ];

        let merged = merge_consecutive_messages(messages).unwrap();
        assert_eq!(merged.len(), 4);
    }

    #[test]
    fn test_merge_assistant_messages() {
        let messages = vec![
            assistant_message(Some("First")),
            assistant_message(Some("Second")),
            assistant_message(None),
            user_message("User"),
            assistant_message(Some("Alone")),
        ];

        let merged = merge_consecutive_messages(messages).unwrap();
        assert_eq!(merged.len(), 3);
        assert_eq!(merged[0], assistant_message(Some("First Second")));
        assert_eq!(merged[2], assistant_message(Some("Alone")));
    }

    #[test]
    fn test_merge_tool_messages() {
        let messages = vec![
            tool_message("part1", "id1"),
            tool_message("part2", "id1"),
            tool_message("partA", "id2"),
            tool_message("partB", "id1"),
            user_message("test"),
        ];

        let merged = merge_consecutive_messages(messages).unwrap();
        assert_eq!(merged.len(), 5);
    }

    #[test]
    fn test_mixed_roles() {
        let messages = vec![
            user_message("A"),
            user_message("B"),
            system_message("C"),
            system_message("D"),
            assistant_message(Some("E")),
            tool_message("F", "id1"),
            tool_message("G", "id1"),
            user_message("H"),
        ];

        let merged = merge_consecutive_messages(messages).unwrap();
        assert_eq!(merged.len(), 7);
        assert_eq!(merged[0], user_message("A B"));
    }
    #[test]
    fn test_empty_messages() {
        let messages = vec![];
        let merged = merge_consecutive_messages(messages);
        assert!(merged.is_ok());
        assert!(merged.unwrap().is_empty());
    }

    #[test]
    fn test_tool_calls_retention() {
        let tool_call = ToolCall {
            id: "test_id".to_string(),
            content: ToolCallContent::Function {
                function: FunctionContent {
                    name: "test".to_string(),
                    arguments: "{}".to_string(),
                },
            },
        };

        let messages = vec![
            RequestMessage::Assistant {
                content: Some("First".into()),
                tool_calls: vec![tool_call],
            },
            RequestMessage::Assistant {
                content: Some("Second".into()),
                tool_calls: vec![],
            },
        ];

        let merged = merge_consecutive_messages(messages).unwrap();
        assert_eq!(merged.len(), 1);
        match &merged[0] {
            RequestMessage::Assistant { tool_calls, .. } => {
                assert_eq!(tool_calls.len(), 1);
            }
            _ => panic!("Invalid message type"),
        }
    }
}
