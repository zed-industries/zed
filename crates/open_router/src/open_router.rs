use anyhow::{Context, Result, anyhow};
use futures::join;
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

pub const OPEN_ROUTER_API_URL: &str = "https://openrouter.ai/api/v1";

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

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: usize,
    pub supports_tool_calls: bool,
    pub excels_at_coding: bool,
}

impl Model {
    pub fn default_fast() -> Self {
        Self::new(
            "openrouter/auto",
            Some("Auto Router"),
            Some(2000000),
            true,
            true,
        )
    }

    pub fn default() -> Self {
        Self::default_fast()
    }

    pub fn new(
        name: &str,
        display_name: Option<&str>,
        max_tokens: Option<usize>,
        supports_tool_calls: bool,
        excels_at_coding: bool,
    ) -> Self {
        Self {
            name: name.to_owned(),
            display_name: display_name.map(|s| s.to_owned()),
            max_tokens: max_tokens.unwrap_or(2000000),
            supports_tool_calls,
            excels_at_coding,
        }
    }

    pub fn id(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_ref().unwrap_or(&self.name)
    }

    pub fn max_token_count(&self) -> usize {
        self.max_tokens
    }

    pub fn max_output_tokens(&self) -> Option<u32> {
        u32::try_from(self.max_tokens).ok()
    }

    pub fn supports_tool_calls(&self) -> bool {
        self.supports_tool_calls
    }

    pub fn excels_at_coding(&self) -> bool {
        self.excels_at_coding
    }

    /// Indicates whether the model supports parallel tool calls.
    /// Currently, this always returns `false` as the functionality is not implemented.
    /// This may serve as a placeholder for future enhancements.
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
    pub max_tokens: Option<u32>,
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
    /// HTTP referer header for OpenRouter attribution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_referer: Option<String>,
    /// HTTP user-agent header for OpenRouter attribution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    Auto,
    Required,
    None,
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
    pub context_length: Option<usize>,

    /// Indicates whether the model can handle OpenAI‑style tool/function calls.
    #[serde(default, skip_serializing_if = "is_false")]
    pub supports_tool_calls: bool,
    /// Indicates whether the model is generally strong at code‑related tasks.
    #[serde(default, skip_serializing_if = "is_false")]
    pub excels_at_coding: bool,
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
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "zed.dev")
        .header("X-Title", "Zed Editor");

    let mut request_body = request;
    request_body.stream = false;
    request_body.http_referer = Some("zed.dev".to_string());
    request_body.http_user_agent = Some("Zed Editor".to_string());

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
        .header("HTTP-Referer", "zed.dev")
        .header("X-Title", "Zed Editor");

    // Add OpenRouter-specific fields
    let mut request = request;
    request.http_referer = Some("zed.dev".to_string());
    request.http_user_agent = Some("Zed Editor".to_string());

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        // Handle SSE comments that OpenRouter sends to prevent connection timeouts
                        if line.starts_with(':') {
                            // This is a comment line (e.g., ": OPENROUTER PROCESSING")
                            // We can ignore it per SSE specs
                            return None;
                        }

                        let line = line.strip_prefix("data: ")?;
                        if line == "[DONE]" {
                            None
                        } else {
                            // For OpenRouter, directly parse the stream event rather than expecting
                            // an untagged enum like OpenAI
                            match serde_json::from_str::<ResponseStreamEvent>(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => {
                                    // Try to parse as an error message
                                    #[derive(Deserialize)]
                                    struct ErrorResponse {
                                        error: String,
                                    }

                                    match serde_json::from_str::<ErrorResponse>(line) {
                                        Ok(err_response) => Some(Err(anyhow!(err_response.error))),
                                        Err(_) => {
                                            // Check if it's an empty line or other non-JSON content
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

pub async fn get_models(client: &dyn HttpClient, api_url: &str) -> Result<Vec<ModelEntry>> {
    // Fetch all three model lists in parallel
    let (base_models_result, tool_models_result, coding_models_result) = join!(
        fetch_models(client, api_url, None),
        fetch_models(client, api_url, Some("supported_parameters=tools")),
        fetch_models(client, api_url, Some("category=programming"))
    );

    // Get the base model list
    let mut models = base_models_result?;

    // Create HashSets of model IDs for efficient lookups
    let tool_model_ids = tool_models_result?
        .into_iter()
        .map(|m| m.id)
        .collect::<std::collections::HashSet<_>>();

    let coding_model_ids = coding_models_result?
        .into_iter()
        .map(|m| m.id)
        .collect::<std::collections::HashSet<_>>();

    // Update model flags based on presence in specialized sets
    for model in &mut models {
        model.supports_tool_calls = tool_model_ids.contains(&model.id);
        model.excels_at_coding = coding_model_ids.contains(&model.id);
    }

    Ok(models)
}

async fn fetch_models(
    client: &dyn HttpClient,
    api_url: &str,
    query_params: Option<&str>,
) -> Result<Vec<ModelEntry>> {
    let uri = match query_params {
        Some(params) => format!("{api_url}/models?{params}"),
        None => format!("{api_url}/models"),
    };

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
        Ok(response.data)
    } else {
        Err(anyhow!(
            "Failed to connect to OpenRouter API: {} {}",
            response.status(),
            body,
        ))
    }
}
