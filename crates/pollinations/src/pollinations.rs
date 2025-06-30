use anyhow::{Context as _, Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{convert::TryFrom, future::Future};
use strum::EnumIter;

pub const POLLINATIONS_API_URL: &str = "https://text.pollinations.ai/openai";

fn is_none_or_empty<T: AsRef<[U]>, U>(opt: &Option<T>) -> bool {
    opt.as_ref().map_or(true, |v| v.as_ref().is_empty())
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
    #[default]
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "openai-large")]
    OpenAILarge,
    #[serde(rename = "openai-fast")]
    OpenAIFast,
    #[serde(rename = "claude-hybridspace")]
    ClaudeHybridspace,
    #[serde(rename = "mistral")]
    Mistral,
    #[serde(rename = "openai-reasoning")]
    OpenAIReasoning,
    #[serde(rename = "openai-roblox")]
    OpenAIRoblox,
    #[serde(rename = "deepseek")]
    Deepseek,
    #[serde(rename = "deepseek-reasoning")]
    DeepseekReasoning,
    #[serde(rename = "grok")]
    Grok,
    #[serde(rename = "llamascout")]
    Llamascout,
    #[serde(rename = "phi")]
    Phi,
    #[serde(rename = "qwen-coder")]
    QwenCoder,
    #[serde(rename = "searchgpt")]
    SearchGpt,
    #[serde(rename = "bidara")]
    Bidara,
    #[serde(rename = "elixposearch")]
    ElixpoSearch,
    // #[serde(rename = "evil")]
    // Evil,
    // #[serde(rename = "hypnosis-tracy")]
    // HypnosisTracy,
    #[serde(rename = "midijourney")]
    Midijourney,
    #[serde(rename = "mirexa")]
    Mirexa,
    #[serde(rename = "rtist")]
    Rtist,
    #[serde(rename = "sur")]
    Sur,
    #[serde(rename = "unity")]
    Unity,

    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::OpenAI
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "openai" => Ok(Self::OpenAI),
            "openai-large" => Ok(Self::OpenAILarge),
            "openai-fast" => Ok(Self::OpenAIFast),
            "claude-hybridspace" => Ok(Self::ClaudeHybridspace),
            "mistral" => Ok(Self::Mistral),
            "openai-reasoning" => Ok(Self::OpenAIReasoning),
            "openai-roblox" => Ok(Self::OpenAIRoblox),
            "deepseek" => Ok(Self::Deepseek),
            "deepseek-reasoning" => Ok(Self::DeepseekReasoning),
            "grok" => Ok(Self::Grok),
            "llamascout" => Ok(Self::Llamascout),
            "phi" => Ok(Self::Phi),
            "qwen-coder" => Ok(Self::QwenCoder),
            "searchgpt" => Ok(Self::SearchGpt),
            "bidara" => Ok(Self::Bidara),
            "elixposearch" => Ok(Self::ElixpoSearch),
            // "evil" => Ok(Self::Evil),
            // "hypnosis-tracy" => Ok(Self::HypnosisTracy),
            "midijourney" => Ok(Self::Midijourney),
            "mirexa" => Ok(Self::Mirexa),
            "rtist" => Ok(Self::Rtist),
            "sur" => Ok(Self::Sur),
            "unity" => Ok(Self::Unity),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::OpenAI => "openai",
            Self::OpenAILarge => "openai-large",
            Self::OpenAIFast => "openai-fast",
            Self::ClaudeHybridspace => "claude-hybridspace",
            Self::Mistral => "mistral",
            Self::OpenAIReasoning => "openai-reasoning",
            Self::OpenAIRoblox => "openai-roblox",
            Self::Deepseek => "deepseek",
            Self::DeepseekReasoning => "deepseek-reasoning",
            Self::Grok => "grok",
            Self::Llamascout => "llamascout",
            Self::Phi => "phi",
            Self::QwenCoder => "qwen-coder",
            Self::SearchGpt => "searchgpt",
            Self::Bidara => "bidara",
            Self::ElixpoSearch => "elixposearch",
            // Self::Evil => "evil",
            // Self::HypnosisTracy => "hypnosis-tracy",
            Self::Midijourney => "midijourney",
            Self::Mirexa => "mirexa",
            Self::Rtist => "rtist",
            Self::Sur => "sur",
            Self::Unity => "unity",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::OpenAI => "OpenAI GPT-4.1 Mini",
            Self::OpenAILarge => "OpenAI GPT-4.1",
            Self::OpenAIFast => "OpenAI GPT-4.1 Nano",
            Self::ClaudeHybridspace => "Claude Hybridspace",
            Self::Mistral => "Mistral Small 3.1 24B",
            Self::OpenAIReasoning => "OpenAI O3",
            Self::OpenAIRoblox => "OpenAI GPT-4.1 Mini (Roblox)",
            Self::Deepseek => "DeepSeek V3",
            Self::DeepseekReasoning => "DeepSeek R1 0528",
            Self::Grok => "xAI Grok-3 Mini",
            Self::Llamascout => "Llama 4 Scout 17B",
            Self::Phi => "Phi-4 Mini Instruct",
            Self::QwenCoder => "Qwen 2.5 Coder 32B",
            Self::SearchGpt => "OpenAI GPT-4o Mini Search Preview",
            Self::Bidara => "BIDARA (NASA)",
            Self::ElixpoSearch => "Elixpo Search",
            // Self::Evil => "Evil",
            // Self::HypnosisTracy => "Hypnosis Tracy",
            Self::Midijourney => "MIDIjourney",
            Self::Mirexa => "Mirexa AI Companion",
            Self::Rtist => "Rtist",
            Self::Sur => "Sur AI Assistant",
            Self::Unity => "Unity Unrestricted Agent",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::OpenAI => 16_385,
            Self::OpenAILarge => 128_000,
            Self::OpenAIFast => 16_385,
            Self::ClaudeHybridspace => 200_000,
            Self::Mistral => 32_000,
            Self::OpenAIReasoning => 200_000,
            Self::OpenAIRoblox => 16_385,
            Self::Deepseek => 32_000,
            Self::DeepseekReasoning => 32_000,
            Self::Grok => 32_000,
            Self::Llamascout => 32_000,
            Self::Phi => 32_000,
            Self::QwenCoder => 32_000,
            Self::SearchGpt => 16_385,
            Self::Bidara => 32_000,
            Self::ElixpoSearch => 32_000,
            // Self::Evil => 32_000,
            // Self::HypnosisTracy => 32_000,
            Self::Midijourney => 32_000,
            Self::Mirexa => 32_000,
            Self::Rtist => 32_000,
            Self::Sur => 32_000,
            Self::Unity => 32_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            Self::OpenAI => Some(4_096),
            Self::OpenAILarge => Some(4_096),
            Self::OpenAIFast => Some(4_096),
            Self::ClaudeHybridspace => Some(4_096),
            Self::Mistral => Some(4_096),
            Self::OpenAIReasoning => Some(4_096),
            Self::OpenAIRoblox => Some(4_096),
            Self::Deepseek => Some(4_096),
            Self::DeepseekReasoning => Some(4_096),
            Self::Grok => Some(4_096),
            Self::Llamascout => Some(4_096),
            Self::Phi => Some(4_096),
            Self::QwenCoder => Some(4_096),
            Self::SearchGpt => Some(4_096),
            Self::Bidara => Some(4_096),
            Self::ElixpoSearch => Some(4_096),
            // Self::Evil => Some(4_096),
            // Self::HypnosisTracy => Some(4_096),
            Self::Midijourney => Some(4_096),
            Self::Mirexa => Some(4_096),
            Self::Rtist => Some(4_096),
            Self::Sur => Some(4_096),
            Self::Unity => Some(4_096),
        }
    }

    /// Returns whether the given model supports the `parallel_tool_calls` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up, or the API will return an error.
    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::Grok
            | Self::Mistral
            | Self::OpenAI
            | Self::OpenAIFast
            | Self::OpenAILarge
            | Self::OpenAIReasoning
            | Self::OpenAIRoblox
            | Self::QwenCoder
            | Self::SearchGpt
            | Self::Bidara
            | Self::Midijourney
            | Self::Mirexa
            | Self::Rtist
            | Self::Sur
            // | Self::Evil
            // | Self::HypnosisTracy
            | Self::Unity => true,
            Self::Deepseek
            | Self::DeepseekReasoning
            | Self::Llamascout
            | Self::ClaudeHybridspace
            | Self::Phi
            | Self::ElixpoSearch
            | Model::Custom { .. } => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop: Vec<String>,
    pub temperature: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Whether to enable parallel function calling during tool use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    Other(ToolDefinition),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    #[allow(dead_code)]
    Function { function: FunctionDefinition },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    Plain(String),
    Multipart(Vec<MessagePart>),
}

impl MessageContent {
    pub fn empty() -> Self {
        MessageContent::Multipart(vec![])
    }

    pub fn push_part(&mut self, part: MessagePart) {
        match self {
            MessageContent::Plain(text) => {
                *self =
                    MessageContent::Multipart(vec![MessagePart::Text { text: text.clone() }, part]);
            }
            MessageContent::Multipart(parts) if parts.is_empty() => match part {
                MessagePart::Text { text } => *self = MessageContent::Plain(text),
                MessagePart::Image { .. } => *self = MessageContent::Multipart(vec![part]),
            },
            MessageContent::Multipart(parts) => parts.push(part),
        }
    }
}

impl From<Vec<MessagePart>> for MessageContent {
    fn from(mut parts: Vec<MessagePart>) -> Self {
        if let [MessagePart::Text { text }] = parts.as_mut_slice() {
            MessageContent::Plain(std::mem::take(text))
        } else {
            MessageContent::Multipart(parts)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum MessagePart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    Image { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
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
    #[serde(default, skip_serializing_if = "is_none_or_empty")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,

    // There is also an optional `type` field that would determine if a
    // function is there. Sometimes this streams in with the `function` before
    // it streams in the `type`
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessageDelta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseStreamResult {
    Ok(ResponseStreamEvent),
    Err { error: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamEvent {
    pub model: String,
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
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
                                Ok(ResponseStreamResult::Ok(response)) => Some(Ok(response)),
                                Ok(ResponseStreamResult::Err { error }) => {
                                    Some(Err(anyhow!(error)))
                                }
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

        #[derive(Deserialize)]
        struct PollinationsResponse {
            error: PollinationsError,
        }

        #[derive(Deserialize)]
        struct PollinationsError {
            message: String,
        }

        match serde_json::from_str::<PollinationsResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "API request to {} failed: {}",
                api_url,
                response.error.message,
            )),

            _ => anyhow::bail!(
                "API request to {} failed with status {}: {}",
                api_url,
                response.status(),
                body,
            ),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum PollinationsEmbeddingModel {
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
}

#[derive(Serialize)]
struct PollinationsEmbeddingRequest<'a> {
    model: PollinationsEmbeddingModel,
    input: Vec<&'a str>,
}

#[derive(Deserialize)]
pub struct PollinationsEmbeddingResponse {
    pub data: Vec<PollinationsEmbedding>,
}

#[derive(Deserialize)]
pub struct PollinationsEmbedding {
    pub embedding: Vec<f32>,
}

pub fn embed<'a>(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    model: PollinationsEmbeddingModel,
    texts: impl IntoIterator<Item = &'a str>,
) -> impl 'static + Future<Output = Result<PollinationsEmbeddingResponse>> {
    let uri = format!("{api_url}/embeddings");

    let request = PollinationsEmbeddingRequest {
        model,
        input: texts.into_iter().collect(),
    };
    let body = AsyncBody::from(serde_json::to_string(&request).unwrap());
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)
        .map(|request| client.send(request));

    async move {
        let mut response = request?.await?;
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        anyhow::ensure!(
            response.status().is_success(),
            "error during embedding, status: {:?}, body: {:?}",
            response.status(),
            body
        );
        let response: PollinationsEmbeddingResponse = serde_json::from_str(&body)
            .context("failed to parse Pollinations embedding response")?;
        Ok(response)
    }
}
