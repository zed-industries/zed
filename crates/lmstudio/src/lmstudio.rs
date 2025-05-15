use anyhow::{Context as _, Result, anyhow};
use futures::{AsyncBufReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest, http};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{convert::TryFrom, sync::Arc, time::Duration};
use bytes::Bytes;

pub const LMSTUDIO_API_URL: &str = "http://localhost:1234/api/v0";

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
    pub max_tokens: usize,
    pub supports_tools: Option<bool>,
}

fn get_max_tokens(name: &str) -> usize {
    /// Default context length for unknown models.
    const DEFAULT_TOKENS: usize = 2048;
    /// Maximum allowed context length
    const MAXIMUM_TOKENS: usize = 32768;

    // Map known models to their context sizes
    let tokens = match name.split(':').next().unwrap_or_default() {
        "text-embedding-nomic-embed-text-v1.5" | "granite-code" => 2048,
        "qwen3-32b" => 32768,
        "deepseek-coder-v2-lite-instruct" => 163840,
        _ => DEFAULT_TOKENS,
    };
    
    tokens.clamp(1, MAXIMUM_TOKENS)
}

impl Model {
    pub fn new(name: &str, display_name: Option<&str>, max_tokens: Option<usize>, supports_tools: Option<bool>) -> Self {
        Self {
            name: name.to_owned(),
            display_name: display_name.map(ToOwned::to_owned),
            max_tokens: max_tokens.unwrap_or_else(|| get_max_tokens(name)),
            supports_tools,
        }
    }

    pub fn id(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.name)
    }

    pub fn max_token_count(&self) -> usize {
        self.max_tokens
    }
}

#[derive(Deserialize, Debug)]
pub struct ModelShow {
    #[serde(default)]
    pub capabilities: Vec<String>,
}

impl ModelShow {
    pub fn supports_tools(&self) -> bool {
        self.capabilities.iter().any(|v| v == "tools")
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    Assistant {
        #[serde(default)]
        content: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<LmStudioToolCall>>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LmStudioToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String, // Always "function"
    pub function: LmStudioFunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LmStudioFunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LmStudioFunctionTool {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LmStudioTool {
    Function { function: LmStudioFunctionTool },
}

#[derive(Serialize, Debug)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    pub max_tokens: Option<i32>,
    pub stop: Option<Vec<String>>,
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<LmStudioTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<&'static str>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChoiceDelta>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceDelta {
    pub index: u32,
    #[serde(default)]
    pub delta: serde_json::Value,
    pub finish_reason: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseStreamResult {
    Ok(ResponseStreamEvent),
    Err { error: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamEvent {
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub data: Vec<LocalModelListing>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LocalModelListing {
    pub id: String,
    pub object: String,
    pub r#type: ModelType,
    pub publisher: String,
    pub arch: Option<String>,
    pub compatibility_type: CompatibilityType,
    pub quantization: Option<String>,
    pub state: ModelState,
    pub max_context_length: Option<usize>,
    pub loaded_context_length: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModelType {
    Llm,
    Embeddings,
    Vlm,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ModelState {
    Loaded,
    Loading,
    NotLoaded,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum CompatibilityType {
    Gguf,
    Mlx,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMessageDelta {
    pub role: Option<Role>,
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
}

/// Client for making requests to the LM Studio API
pub struct LmStudioClient {
    client: Arc<dyn HttpClient>,
    api_url: String,
}

impl LmStudioClient {
    /// Create a new LM Studio client
    pub fn new(client: Arc<dyn HttpClient>, api_url: String) -> Self {
        Self { client, api_url }
    }

    /// Stream chat completions from LM Studio
    pub async fn stream_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<BoxStream<'static, Result<ChatResponse>>> {
        stream_chat_completion(self.client.as_ref(), &self.api_url, request).await
    }
    
    /// Make a chat completion request to LM Studio
    pub async fn complete(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatResponse> {
        complete(self.client.as_ref(), &self.api_url, request).await
    }
    
    /// Get the list of available models from LM Studio
    pub async fn get_models(
        &self,
        timeout: Option<Duration>,
    ) -> Result<Vec<LocalModelListing>> {
        get_models(self.client.as_ref(), &self.api_url, timeout).await
    }
    
    /// Get model capabilities from LM Studio
    pub async fn show_model(
        &self,
        model: &str,
    ) -> Result<ModelShow> {
        show_model(self.client.as_ref(), &self.api_url, model).await
    }
    
    /// Preload a model in LM Studio
    pub async fn preload_model(
        &self,
        model: &str,
    ) -> Result<()> {
        preload_model(self.client.clone(), &self.api_url, model).await
    }
}

/// Helper function to read response body into a String and also return the status
async fn read_response_body_with_status(
    response: http_client::Response<AsyncBody>
) -> Result<(http::StatusCode, String)> {
    let status = response.status();
    let mut body = String::new();
    let mut reader = BufReader::new(response.into_body());
    futures::AsyncReadExt::read_to_string(&mut reader, &mut body).await?;
    Ok((status, body))
}

/// Makes a chat completion request to LM Studio API
pub async fn complete(
    client: &dyn HttpClient,
    api_url: &str,
    request: ChatCompletionRequest,
) -> Result<ChatResponse> {
    let endpoint = format!("{}/chat/completions", api_url);
    
    let body = serde_json::to_vec(&request)
        .context("Failed to serialize chat completion request")?;
    
    let http_request = HttpRequest::builder()
        .method(Method::POST)
        .uri(&endpoint)
        .header("Content-Type", "application/json")
        .body(AsyncBody::from_bytes(Bytes::from(body)))
        .context("Failed to build HTTP request")?;
    
    let response = client.send(http_request).await?;
    
    let (status, body_str) = read_response_body_with_status(response).await?;
    
    if !status.is_success() {
        return Err(anyhow!(
            "LM Studio API error ({}): {}",
            status,
            body_str
        ));
    }
    
    serde_json::from_str(&body_str).context("Failed to parse chat completion response")
}

/// Streams chat completions from LM Studio API
pub async fn stream_chat_completion(
    client: &dyn HttpClient,
    api_url: &str,
    request: ChatCompletionRequest,
) -> Result<BoxStream<'static, Result<ChatResponse>>> {
    let endpoint = format!("{}/chat/completions", api_url);
    
    let request_body = serde_json::to_vec(&request)
        .context("Failed to serialize chat completion request")?;
    
    // Debug log the serialized request
    if log::log_enabled!(log::Level::Debug) {
        match serde_json::to_string_pretty(&request) {
            Ok(json) => log::debug!("LMStudio API request to {}: {}", endpoint, json),
            Err(_) => log::debug!("LMStudio API request serialization debug failed"),
        }
    }
    
    let http_request = HttpRequest::builder()
        .method(Method::POST)
        .uri(&endpoint)
        .header("Content-Type", "application/json")
        .body(AsyncBody::from_bytes(Bytes::from(request_body)))
        .context("Failed to build HTTP request")?;
    
    let response = client.send(http_request).await?;
    
    let status = response.status();
    if !status.is_success() {
        let mut body = String::new();
        let mut reader = BufReader::new(response.into_body());
        futures::AsyncReadExt::read_to_string(&mut reader, &mut body).await?;
        
        return Err(anyhow!(
            "LM Studio API error ({}): {}",
            status,
            body
        ));
    }
    
    let reader = BufReader::new(response.into_body());
    let stream = reader
        .lines()
        .filter_map(|line| async move {
            match line {
                Ok(line) => {
                    if line.is_empty() || line.starts_with("data: [DONE]") {
                        return None;
                    }
                    
                    // Remove "data: " prefix if present
                    let json_str = line.strip_prefix("data: ").unwrap_or(&line);
                    
                    match serde_json::from_str::<ChatResponse>(json_str) {
                        Ok(response) => Some(Ok(response)),
                        Err(e) => Some(Err(anyhow!("Failed to parse streaming response: {}", e))),
                    }
                }
                Err(e) => Some(Err(anyhow!("Failed to read streaming response: {}", e))),
            }
        })
        .boxed();
    
    Ok(stream)
}

/// Gets the list of available models from LM Studio API
pub async fn get_models(
    client: &dyn HttpClient,
    api_url: &str,
    _timeout: Option<Duration>,
) -> Result<Vec<LocalModelListing>> {
    let endpoint = format!("{}/models", api_url);
    
    let http_request = HttpRequest::builder()
        .method(Method::GET)
        .uri(&endpoint)
        .body(AsyncBody::empty())
        .context("Failed to build HTTP request")?;
    
    let response = client.send(http_request).await?;
    
    let (status, body_str) = read_response_body_with_status(response).await?;
    
    if !status.is_success() {
        return Err(anyhow!(
            "LM Studio API error ({}): {}",
            status,
            body_str
        ));
    }
    
    let models: ListModelsResponse = serde_json::from_str(&body_str)
        .context("Failed to parse models response")?;
    
    Ok(models.data)
}

/// Gets model capabilities from LM Studio API
pub async fn show_model(client: &dyn HttpClient, api_url: &str, model: &str) -> Result<ModelShow> {
    let endpoint = format!("{}/models/{}", api_url, http::Uri::try_from(model)?);
    
    let http_request = HttpRequest::builder()
        .method(Method::GET)
        .uri(&endpoint)
        .body(AsyncBody::empty())
        .context("Failed to build HTTP request")?;
    
    let response = client.send(http_request).await?;
    
    let (status, body_str) = read_response_body_with_status(response).await?;
    
    if !status.is_success() {
        return Err(anyhow!(
            "LM Studio API error ({}): {}",
            status,
            body_str
        ));
    }
    
    serde_json::from_str(&body_str).context("Failed to parse model show response")
}

/// Preloads a model in LM Studio
pub async fn preload_model(client: Arc<dyn HttpClient>, api_url: &str, model: &str) -> Result<()> {
    let endpoint = format!("{}/models/{}/load", api_url, http::Uri::try_from(model)?);
    
    let http_request = HttpRequest::builder()
        .method(Method::POST)
        .uri(&endpoint)
        .body(AsyncBody::empty())
        .context("Failed to build HTTP request")?;
    
    let response = client.send(http_request).await?;
    
    let (status, body_str) = read_response_body_with_status(response).await?;
    
    if !status.is_success() {
        return Err(anyhow!(
            "LM Studio API error ({}): {}",
            status,
            body_str
        ));
    }
    
    Ok(())
}
