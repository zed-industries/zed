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
    pub server_id: Option<String>,
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
    pub fn new(name: &str, display_name: Option<&str>, max_tokens: Option<usize>, server_id: Option<String>) -> Self {
        Self {
            name: name.to_owned(),
            display_name: display_name.map(ToOwned::to_owned),
            max_tokens: max_tokens.unwrap_or_else(|| get_max_tokens(name)),
            supports_tools: Some(true),
            server_id,
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
    
    // Debug log the serialized request with size limits
    if log::log_enabled!(log::Level::Debug) {
        // Create a summarized version for logging
        let log_safe_request = ChatCompletionRequest {
            model: request.model.clone(),
            messages: request.messages.iter().map(|msg| {
                match msg {
                    ChatMessage::User { content } => {
                        let truncated = if content.len() > 100 {
                            format!("{}... [truncated, total length: {} chars]", &content[..100], content.len())
                        } else {
                            content.clone()
                        };
                        ChatMessage::User { content: truncated }
                    },
                    ChatMessage::System { content } => {
                        let truncated = if content.len() > 100 {
                            format!("{}... [truncated, total length: {} chars]", &content[..100], content.len())
                        } else {
                            content.clone()
                        };
                        ChatMessage::System { content: truncated }
                    },
                    ChatMessage::Assistant { content, tool_calls } => {
                        let truncated = content.as_ref().map(|c| {
                            if c.len() > 100 {
                                format!("{}... [truncated, total length: {} chars]", &c[..100], c.len())
                            } else {
                                c.clone()
                            }
                        });
                        ChatMessage::Assistant { 
                            content: truncated,
                            tool_calls: tool_calls.clone()
                        }
                    },
                    ChatMessage::Tool { content, tool_call_id } => {
                        let truncated = if content.len() > 100 {
                            format!("{}... [truncated, total length: {} chars]", &content[..100], content.len())
                        } else {
                            content.clone()
                        };
                        ChatMessage::Tool { 
                            content: truncated,
                            tool_call_id: tool_call_id.clone()
                        }
                    }
                }
            }).collect(),
            stream: request.stream,
            max_tokens: request.max_tokens,
            stop: request.stop.clone(),
            temperature: request.temperature,
            tools: vec![],  // Don't log the full tools
            tool_choice: request.tool_choice,
        };
        
        match serde_json::to_string(&log_safe_request) {
            Ok(json) => {
                let tools_count = request.tools.len();
                let total_msg_count = request.messages.len();
                log::debug!(
                    "LMStudio API request to {}: {} messages, {} tools, request size: {} bytes", 
                    endpoint, 
                    total_msg_count,
                    tools_count,
                    request_body.len()
                );
                log::debug!("Request summary: {}", json);
            },
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
    // Try both v1 and v0 endpoints
    let endpoints = vec![
        format!("{}/models", api_url),          // OpenAI-compatible endpoint (v1)
        format!("{}/api/models", api_url),      // Older API endpoint (some versions)
        format!("{}/v1/models", api_url),       // OpenAI format with explicit v1
    ];
    
    let mut last_error = anyhow!("No valid LM Studio endpoint found");
    
    // Try each endpoint in sequence
    for endpoint in endpoints {
        log::info!("Trying to fetch models from endpoint: {}", endpoint);
        
        let http_request = HttpRequest::builder()
            .method(Method::GET)
            .uri(&endpoint)
            .body(AsyncBody::empty())
            .context("Failed to build HTTP request")?;
        
        match client.send(http_request).await {
            Ok(response) => {
                let (status, body_str) = read_response_body_with_status(response).await?;
                
                if !status.is_success() {
                    log::warn!("LM Studio API error at {}: {} - {}", endpoint, status, body_str);
                    continue;
                }
                
                // Log the raw API response for debugging
                log::debug!("LM Studio API models response from {}: {} models, response size: {} bytes", 
                    endpoint, 
                    body_str.chars().filter(|c| *c == '{').count(), // Rough estimate of model count
                    body_str.len()
                );
                
                // First try to parse as ListModelsResponse
                let result = match serde_json::from_str::<ListModelsResponse>(&body_str) {
                    Ok(models) => {
                        log::info!("Successfully parsed response as ListModelsResponse");
                        Ok(models.data)
                    },
                    Err(e) => {
                        log::warn!("Failed to parse as ListModelsResponse: {} - trying OpenAI format", e);
                        
                        // Try parsing as OpenAI-compatible format (simplified structure)
                        log::debug!("Attempting to parse as SimplifiedListModelsResponse");
                        match serde_json::from_str::<SimplifiedListModelsResponse>(&body_str) {
                            Ok(models) => {
                                log::info!("Successfully parsed response as SimplifiedListModelsResponse with {} models", models.data.len());
                                let converted_models: Vec<LocalModelListing> = models.data.into_iter()
                                    .map(|simplified| {
                                        log::debug!("Converting simplified model: id={}", simplified.id);
                                        LocalModelListing {
                                            id: simplified.id,
                                            object: simplified.object,
                                            r#type: ModelType::Llm,  // Assume LLM type
                                            publisher: simplified.owned_by.unwrap_or_else(|| "unknown".to_string()),
                                            arch: None,
                                            compatibility_type: CompatibilityType::Gguf,  // Default
                                            quantization: None,
                                            state: ModelState::Loaded,  // Assume it's loaded
                                            max_context_length: Some(8192),  // Default
                                            loaded_context_length: None,
                                        }
                                    })
                                    .collect();
                                log::info!("Successfully converted {} models to LocalModelListing format", converted_models.len());
                                Ok(converted_models)
                            },
                            Err(e2) => {
                                log::warn!("Failed to parse as SimplifiedListModelsResponse: {} - trying direct JSON extraction", e2);
                                
                                // Try direct JSON parsing approach
                                match serde_json::from_str::<serde_json::Value>(&body_str) {
                                    Ok(json) => {
                                        if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                                            log::info!("Found data array through direct JSON parsing with {} models", data.len());
                                            let mut models = Vec::new();
                                            
                                            for model_json in data {
                                                if let Some(id) = model_json.get("id").and_then(|i| i.as_str()) {
                                                    log::debug!("Directly parsing model with id: {}", id);
                                                    
                                                    // Get publisher/owned_by if it exists
                                                    let publisher = model_json.get("owned_by")
                                                        .and_then(|o| o.as_str())
                                                        .unwrap_or("unknown")
                                                        .to_string();
                                                    
                                                    // Create model with required fields and sensible defaults
                                                    let model = LocalModelListing {
                                                        id: id.to_string(),
                                                        object: model_json.get("object")
                                                            .and_then(|o| o.as_str())
                                                            .unwrap_or("model")
                                                            .to_string(),
                                                        r#type: ModelType::Llm,
                                                        publisher,
                                                        arch: None,
                                                        compatibility_type: CompatibilityType::Gguf,
                                                        quantization: None,
                                                        state: ModelState::Loaded,
                                                        max_context_length: Some(8192),
                                                        loaded_context_length: None,
                                                    };
                                                    
                                                    models.push(model);
                                                }
                                            }
                                            
                                            if !models.is_empty() {
                                                log::info!("Successfully created {} models through direct JSON parsing", models.len());
                                                return Ok(models);
                                            }
                                        }
                                        
                                        log::warn!("No models found through direct JSON parsing, falling back to other methods");
                                    },
                                    Err(e) => {
                                        log::warn!("Failed to parse JSON for direct extraction: {}", e);
                                    }
                                }
                                
                                // Try parsing as direct array of models
                                match serde_json::from_str::<Vec<LocalModelListing>>(&body_str) {
                                    Ok(models) => {
                                        log::info!("Successfully parsed response as direct array of models");
                                        Ok(models)
                                    },
                                    Err(e3) => {
                                        // Try parsing as object with a different structure
                                        match serde_json::from_str::<serde_json::Value>(&body_str) {
                                            Ok(value) => {
                                                log::info!("Got JSON value, attempting to extract models");
                                                // Check if it's an object with a models field
                                                if let Some(models_array) = value.get("models").and_then(|v| v.as_array()) {
                                                    match serde_json::from_value::<Vec<LocalModelListing>>(models_array.clone().into()) {
                                                        Ok(models) => {
                                                            log::info!("Extracted models from 'models' field");
                                                            Ok(models)
                                                        }
                                                        Err(e3) => {
                                                            log::error!("Failed to parse models from 'models' field: {}", e3);
                                                            Err(anyhow!("Failed to parse models response: multiple formats attempted"))
                                                        }
                                                    }
                                                } else if let Some(data_array) = value.get("data").and_then(|v| v.as_array()) {
                                                    // Try extracting from a data field (OpenAI format)
                                                    match serde_json::from_value::<Vec<LocalModelListing>>(data_array.clone().into()) {
                                                        Ok(models) => {
                                                            log::info!("Extracted models from 'data' field");
                                                            Ok(models)
                                                        }
                                                        Err(e3) => {
                                                            log::error!("Failed to parse models from 'data' field: {}", e3);
                                                            Err(anyhow!("Failed to parse models response: multiple formats attempted"))
                                                        }
                                                    }
                                                } else {
                                                    // Try to create a minimal model from whatever data is available
                                                    let mut models = Vec::new();
                                                    
                                                    // Log the structure of what we received
                                                    log::info!("JSON structure received: {}", serde_json::to_string_pretty(&value).unwrap_or_default());
                                                    
                                                    // If we have a map with keys that could be model names, try to extract them
                                                    if value.is_object() {
                                                        for (key, _val) in value.as_object().unwrap() {
                                                            if !key.starts_with("_") {  // Ignore metadata fields
                                                                let model = LocalModelListing {
                                                                    id: key.clone(),
                                                                    object: "model".to_string(),
                                                                    r#type: ModelType::Llm,  // Assume it's an LLM
                                                                    publisher: "unknown".to_string(),
                                                                    arch: None,
                                                                    compatibility_type: CompatibilityType::Gguf,  // Default
                                                                    quantization: None,
                                                                    state: ModelState::Loaded,  // Assume it's loaded
                                                                    max_context_length: Some(8192),  // Default
                                                                    loaded_context_length: None,
                                                                };
                                                                models.push(model);
                                                            }
                                                        }
                                                        
                                                        if !models.is_empty() {
                                                            log::info!("Extracted {} models from object keys", models.len());
                                                            return Ok(models);
                                                        }
                                                    }
                                                    
                                                    log::error!("Could not find models in the response from endpoint {}", endpoint);
                                                    Err(anyhow!("Could not find models in the response"))
                                                }
                                            },
                                            Err(e3) => {
                                                log::error!("Failed to parse response as JSON: {}", e3);
                                                log::error!("Raw response: {}", body_str);
                                                Err(anyhow!("Failed to parse models response: not valid JSON"))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                };
                
                // If successful, return the result, otherwise try the next endpoint
                match &result {
                    Ok(models) => {
                        log::info!("Successfully found {} models from endpoint {}", models.len(), endpoint);
                        return result;
                    },
                    Err(e) => {
                        log::warn!("Failed to parse models from endpoint {}: {}", endpoint, e);
                        last_error = anyhow!("Failed to parse response from {}: {}", endpoint, e);
                    }
                }
            },
            Err(e) => {
                log::warn!("Failed to connect to endpoint {}: {}", endpoint, e);
                last_error = anyhow!("Failed to connect to endpoint {}: {}", endpoint, e);
            }
        }
    }
    
    // If we get here, all endpoints failed
    log::error!("All LM Studio endpoints failed");
    Err(last_error)
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

/// Check if the LM Studio server is running and accessible
pub async fn healthcheck(
    client: &dyn HttpClient,
    api_url: &str,
) -> Result<bool> {
    // Try different health endpoint possibilities
    let endpoints = vec![
        format!("{}/health", api_url),           // Standard health endpoint
        format!("{}/v1/health", api_url),        // v1 explicit health endpoint
        format!("{}/v1/models", api_url),        // Models endpoint as fallback
        format!("{}/models", api_url),           // Base models endpoint as fallback
    ];
    
    for endpoint in &endpoints {
        log::debug!("Sending healthcheck request to LM Studio at {}", endpoint);
        
        let http_request = HttpRequest::builder()
            .method(Method::GET)
            .uri(endpoint)
            .body(AsyncBody::empty())
            .context("Failed to build healthcheck HTTP request")?;
        
        match client.send(http_request).await {
            Ok(response) => {
                let status = response.status();
                log::debug!("LM Studio healthcheck status code from {}: {}", endpoint, status);
                
                // Consider any 2xx response as healthy
                if status.is_success() {
                    log::info!("LM Studio server is healthy (endpoint: {})", endpoint);
                    return Ok(true);
                }
            },
            Err(e) => {
                log::debug!("LM Studio healthcheck connection failed at {}: {}", endpoint, e);
                // Continue trying other endpoints
            }
        }
    }
    
    log::warn!("LM Studio server is not reachable at any endpoint");
    Ok(false)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SimplifiedModelListing {
    pub id: String,
    pub object: String,
    #[serde(default)]
    pub owned_by: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SimplifiedListModelsResponse {
    pub data: Vec<SimplifiedModelListing>,
    pub object: String,
}