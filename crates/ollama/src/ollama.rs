use anyhow::{Context as _, Result};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest, http};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{sync::Arc, time::Duration};

pub const OLLAMA_API_URL: &str = "http://localhost:11434";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum KeepAlive {
    /// Keep model alive for N seconds
    Seconds(isize),
    /// Keep model alive for a fixed duration. Accepts durations like "5m", "10m", "1h", "1d", etc.
    Duration(String),
}

impl KeepAlive {
    /// Keep model alive until a new model is loaded or until Ollama shuts down
    fn indefinite() -> Self {
        Self::Seconds(-1)
    }
}

impl Default for KeepAlive {
    fn default() -> Self {
        Self::indefinite()
    }
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct AvailableModel {
    /// The model name in the Ollama API (e.g. "llama3.2:latest")
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the assistant panel.
    pub display_name: Option<String>,
    /// The Context Length parameter to the model (aka num_ctx or n_ctx)
    pub max_tokens: u64,
    /// The number of seconds to keep the connection open after the last request
    pub keep_alive: Option<KeepAlive>,
    /// Whether the model supports tools
    pub supports_tools: Option<bool>,
    /// Whether the model supports images
    pub supports_images: Option<bool>,
    /// Whether to enable think mode
    pub supports_thinking: Option<bool>,
}

fn get_max_tokens(name: &str) -> u64 {
    /// Default context length for unknown models.
    const DEFAULT_TOKENS: u64 = 4096;
    /// Magic number. Lets many Ollama models work with ~16GB of ram.
    /// Models that support context beyond 16k such as codestral (32k) or devstral (128k) will be clamped down to 16k
    const MAXIMUM_TOKENS: u64 = 16384;

    match name.split(':').next().unwrap() {
        "granite-code" | "phi" | "tinyllama" => 2048,
        "llama2" | "stablelm2" | "vicuna" | "yi" => 4096,
        "aya" | "codegemma" | "gemma" | "gemma2" | "llama3" | "starcoder" => 8192,
        "codellama" | "starcoder2" => 16384,
        "codestral" | "dolphin-mixtral" | "llava" | "magistral" | "mistral" | "mixstral"
        | "qwen2" | "qwen2.5-coder" => 32768,
        "cogito" | "command-r" | "deepseek-coder-v2" | "deepseek-r1" | "deepseek-v3"
        | "devstral" | "gemma3" | "gpt-oss" | "granite3.3" | "llama3.1" | "llama3.2"
        | "llama3.3" | "mistral-nemo" | "phi3" | "phi3.5" | "phi4" | "qwen3" | "yi-coder" => 128000,
        _ => DEFAULT_TOKENS,
    }
    .clamp(1, MAXIMUM_TOKENS)
}

impl AvailableModel {
    pub fn new(
        name: &str,
        display_name: Option<&str>,
        max_tokens: Option<u64>,
        supports_tools: Option<bool>,
        supports_images: Option<bool>,
        supports_thinking: Option<bool>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            display_name: display_name
                .map(ToString::to_string)
                .or_else(|| name.strip_suffix(":latest").map(ToString::to_string)),
            max_tokens: max_tokens.unwrap_or_else(|| get_max_tokens(name)),
            keep_alive: Some(KeepAlive::indefinite()),
            supports_tools,
            supports_images,
            supports_thinking,
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
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    Assistant {
        content: String,
        tool_calls: Option<Vec<OllamaToolCall>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        images: Option<Vec<String>>,
        thinking: Option<String>,
    },
    User {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        images: Option<Vec<String>>,
    },
    System {
        content: String,
    },
    Tool {
        tool_name: String,
        content: String,
    },
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OllamaToolCall {
    Function(OllamaFunctionCall),
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaFunctionCall {
    pub name: String,
    pub arguments: Value,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct OllamaFunctionTool {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum OllamaTool {
    Function { function: OllamaFunctionTool },
}

#[derive(Serialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    pub keep_alive: KeepAlive,
    pub options: Option<ChatOptions>,
    pub tools: Vec<OllamaTool>,
    pub think: Option<bool>,
}

impl ChatRequest {
    pub fn with_tools(mut self, tools: Vec<OllamaTool>) -> Self {
        self.stream = false;
        self.tools = tools;
        self
    }
}

// https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values
#[derive(Serialize, Default, Debug)]
pub struct ChatOptions {
    pub num_ctx: Option<u64>,
    pub num_predict: Option<isize>,
    pub stop: Option<Vec<String>>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponseDelta {
    #[allow(unused)]
    pub model: String,
    #[allow(unused)]
    pub created_at: String,
    pub message: ChatMessage,
    #[allow(unused)]
    pub done_reason: Option<String>,
    #[allow(unused)]
    pub done: bool,
    pub prompt_eval_count: Option<u64>,
    pub eval_count: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct LocalModelsResponse {
    pub models: Vec<LocalModelListing>,
}

#[derive(Serialize, Deserialize)]
pub struct LocalModelListing {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
    pub details: ModelDetails,
}

#[derive(Serialize, Deserialize)]
pub struct LocalModel {
    pub modelfile: String,
    pub parameters: String,
    pub template: String,
    pub details: ModelDetails,
}

#[derive(Serialize, Deserialize)]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Option<Vec<String>>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Deserialize, Debug)]
pub struct ModelShow {
    #[serde(default)]
    pub capabilities: Vec<String>,
}

impl ModelShow {
    pub fn supports_tools(&self) -> bool {
        // .contains expects &String, which would require an additional allocation
        self.capabilities.iter().any(|v| v == "tools")
    }

    pub fn supports_images(&self) -> bool {
        self.capabilities.iter().any(|v| v == "vision")
    }

    pub fn supports_thinking(&self) -> bool {
        self.capabilities.iter().any(|v| v == "thinking")
    }
}

pub async fn complete(
    client: &dyn HttpClient,
    api_url: &str,
    request: ChatRequest,
) -> Result<ChatResponseDelta> {
    let uri = format!("{api_url}/api/chat");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json");

    let serialized_request = serde_json::to_string(&request)?;
    let request = request_builder.body(AsyncBody::from(serialized_request))?;

    let mut response = client.send(request).await?;

    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;

    if response.status().is_success() {
        let response_message: ChatResponseDelta = serde_json::from_slice(&body)?;
        Ok(response_message)
    } else {
        let body_str = std::str::from_utf8(&body)?;
        anyhow::bail!(
            "Failed to connect to API: {} {}",
            response.status(),
            body_str
        );
    }
}

pub async fn stream_chat_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    request: ChatRequest,
) -> Result<BoxStream<'static, Result<ChatResponseDelta>>> {
    let uri = format!("{api_url}/api/chat");
    let mut request_builder = http::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json");

    if let Some(api_key) = api_key {
        request_builder = request_builder.header("Authorization", format!("Bearer {api_key}"))
    }

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());

        Ok(reader
            .lines()
            .map(|line| match line {
                Ok(line) => serde_json::from_str(&line).context("Unable to parse chat response"),
                Err(e) => Err(e.into()),
            })
            .boxed())
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::bail!(
            "Failed to connect to Ollama API: {} {}",
            response.status(),
            body,
        );
    }
}

pub async fn get_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    _: Option<Duration>,
) -> Result<Vec<LocalModelListing>> {
    let uri = format!("{api_url}/api/tags");
    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");

    if let Some(api_key) = api_key {
        request_builder = request_builder.header("Authorization", format!("Bearer {api_key}"));
    }

    let request = request_builder.body(AsyncBody::default())?;

    let mut response = client.send(request).await?;

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to connect to Ollama API: {} {}",
        response.status(),
        body,
    );
    let response: LocalModelsResponse =
        serde_json::from_str(&body).context("Unable to parse Ollama tag listing")?;
    Ok(response.models)
}

/// Fetch details of a model, used to determine model capabilities
pub async fn show_model(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    model: &str,
) -> Result<ModelShow> {
    let uri = format!("{api_url}/api/show");
    let mut request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json");

    if let Some(api_key) = api_key {
        request_builder = request_builder.header("Authorization", format!("Bearer {api_key}"));
    }

    let request = request_builder.body(AsyncBody::from(
        serde_json::json!({ "model": model }).to_string(),
    ))?;

    let mut response = client.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to connect to Ollama API: {} {}",
        response.status(),
        body,
    );
    let details: ModelShow = serde_json::from_str(body.as_str())?;
    Ok(details)
}

/// Discover available models from the Ollama API with their capabilities
pub async fn discover_available_models(
    client: Arc<dyn HttpClient>,
    api_url: &str,
    api_key: Option<Arc<str>>,
) -> Result<Vec<AvailableModel>> {
    let models = get_models(client.as_ref(), api_url, api_key.as_deref(), None).await?;

    let tasks = models
        .into_iter()
        // Since there is no metadata from the Ollama API
        // indicating which models are embedding models,
        // simply filter out models with "-embed" in their name
        .filter(|model| !model.name.contains("-embed"))
        .map(|model| {
            let client = client.clone();
            let api_key = api_key.clone();
            let api_url = api_url.to_string();
            async move {
                let capabilities =
                    show_model(client.as_ref(), &api_url, api_key.as_deref(), &model.name).await?;
                Ok(AvailableModel::new(
                    &model.name,
                    None,
                    None,
                    Some(capabilities.supports_tools()),
                    Some(capabilities.supports_images()),
                    Some(capabilities.supports_thinking()),
                ))
            }
        });

    let mut available_models: Vec<_> = futures::stream::iter(tasks)
        .buffer_unordered(5)
        .collect::<Vec<Result<_>>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    available_models.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(available_models)
}

#[cfg(any(test, feature = "test-support"))]
pub mod fake {
    use super::*;
    use http_client::{AsyncBody, Response, Url};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    pub struct FakeHttpClient {
        responses: Arc<Mutex<HashMap<String, String>>>,
        requests: Arc<Mutex<Vec<(String, String)>>>, // (path, body)
    }

    impl FakeHttpClient {
        pub fn new() -> Self {
            Self {
                responses: Arc::new(Mutex::new(HashMap::new())),
                requests: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn set_response(&self, path: &str, response: String) {
            self.responses
                .lock()
                .unwrap()
                .insert(path.to_string(), response);
        }

        pub fn set_generate_response(&self, completion_text: &str) {
            let response = serde_json::json!({
                "response": completion_text,
                "done": true,
                "context": [],
                "total_duration": 1000000_u64,
                "load_duration": 1000000_u64,
                "prompt_eval_count": 10,
                "prompt_eval_duration": 1000000_u64,
                "eval_count": 20,
                "eval_duration": 1000000_u64
            });
            self.set_response("/api/generate", response.to_string());
        }

        pub fn set_error(&self, path: &str) {
            // Remove any existing response to force an error
            self.responses.lock().unwrap().remove(path);
        }

        pub fn get_requests(&self) -> Vec<(String, String)> {
            self.requests.lock().unwrap().clone()
        }

        pub fn clear_requests(&self) {
            self.requests.lock().unwrap().clear();
        }
    }

    impl HttpClient for FakeHttpClient {
        fn type_name(&self) -> &'static str {
            "FakeHttpClient"
        }

        fn user_agent(&self) -> Option<&http::HeaderValue> {
            None
        }

        fn proxy(&self) -> Option<&Url> {
            None
        }

        fn send(
            &self,
            req: http_client::Request<AsyncBody>,
        ) -> futures::future::BoxFuture<'static, Result<Response<AsyncBody>, anyhow::Error>>
        {
            let path = req.uri().path().to_string();
            let responses = Arc::clone(&self.responses);
            let requests = Arc::clone(&self.requests);

            Box::pin(async move {
                // Store the request
                requests.lock().unwrap().push((path.clone(), String::new()));

                let responses = responses.lock().unwrap();

                if let Some(response_body) = responses.get(&path).cloned() {
                    let response = Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(AsyncBody::from(response_body))
                        .unwrap();
                    Ok(response)
                } else {
                    Err(anyhow::anyhow!("No mock response set for {}", path))
                }
            })
        }
    }

    pub struct Ollama;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_completion() {
        let response = serde_json::json!({
        "model": "llama3.2",
        "created_at": "2023-12-12T14:13:43.416799Z",
        "message": {
            "role": "assistant",
            "content": "Hello! How are you today?"
        },
        "done": true,
        "total_duration": 5191566416u64,
        "load_duration": 2154458,
        "prompt_eval_count": 26,
        "prompt_eval_duration": 383809000,
        "eval_count": 298,
        "eval_duration": 4799921000u64
        });
        let _: ChatResponseDelta = serde_json::from_value(response).unwrap();
    }

    #[test]
    fn parse_streaming_completion() {
        let partial = serde_json::json!({
        "model": "llama3.2",
        "created_at": "2023-08-04T08:52:19.385406455-07:00",
        "message": {
            "role": "assistant",
            "content": "The",
            "images": null
        },
        "done": false
        });

        let _: ChatResponseDelta = serde_json::from_value(partial).unwrap();

        let last = serde_json::json!({
        "model": "llama3.2",
        "created_at": "2023-08-04T19:22:45.499127Z",
        "message": {
            "role": "assistant",
            "content": ""
        },
        "done": true,
        "total_duration": 4883583458u64,
        "load_duration": 1334875,
        "prompt_eval_count": 26,
        "prompt_eval_duration": 342546000,
        "eval_count": 282,
        "eval_duration": 4535599000u64
        });

        let _: ChatResponseDelta = serde_json::from_value(last).unwrap();
    }

    #[test]
    fn parse_tool_call() {
        let response = serde_json::json!({
            "model": "llama3.2:3b",
            "created_at": "2025-04-28T20:02:02.140489Z",
            "message": {
                "role": "assistant",
                "content": "",
                "tool_calls": [
                    {
                        "function": {
                            "name": "weather",
                            "arguments": {
                                "city": "london",
                            }
                        }
                    }
                ]
            },
            "done_reason": "stop",
            "done": true,
            "total_duration": 2758629166u64,
            "load_duration": 1770059875,
            "prompt_eval_count": 147,
            "prompt_eval_duration": 684637583,
            "eval_count": 16,
            "eval_duration": 302561917,
        });

        let result: ChatResponseDelta = serde_json::from_value(response).unwrap();
        match result.message {
            ChatMessage::Assistant {
                content,
                tool_calls,
                images: _,
                thinking,
            } => {
                assert!(content.is_empty());
                assert!(tool_calls.is_some_and(|v| !v.is_empty()));
                assert!(thinking.is_none());
            }
            _ => panic!("Deserialized wrong role"),
        }
    }

    #[test]
    fn parse_show_model() {
        let response = serde_json::json!({
            "license": "LLAMA 3.2 COMMUNITY LICENSE AGREEMENT...",
            "details": {
                "parent_model": "",
                "format": "gguf",
                "family": "llama",
                "families": ["llama"],
                "parameter_size": "3.2B",
                "quantization_level": "Q4_K_M"
            },
            "model_info": {
                "general.architecture": "llama",
                "general.basename": "Llama-3.2",
                "general.file_type": 15,
                "general.finetune": "Instruct",
                "general.languages": ["en", "de", "fr", "it", "pt", "hi", "es", "th"],
                "general.parameter_count": 3212749888u64,
                "general.quantization_version": 2,
                "general.size_label": "3B",
                "general.tags": ["facebook", "meta", "pytorch", "llama", "llama-3", "text-generation"],
                "general.type": "model",
                "llama.attention.head_count": 24,
                "llama.attention.head_count_kv": 8,
                "llama.attention.key_length": 128,
                "llama.attention.layer_norm_rms_epsilon": 0.00001,
                "llama.attention.value_length": 128,
                "llama.block_count": 28,
                "llama.context_length": 131072,
                "llama.embedding_length": 3072,
                "llama.feed_forward_length": 8192,
                "llama.rope.dimension_count": 128,
                "llama.rope.freq_base": 500000,
                "llama.vocab_size": 128256,
                "tokenizer.ggml.bos_token_id": 128000,
                "tokenizer.ggml.eos_token_id": 128009,
                "tokenizer.ggml.merges": null,
                "tokenizer.ggml.model": "gpt2",
                "tokenizer.ggml.pre": "llama-bpe",
                "tokenizer.ggml.token_type": null,
                "tokenizer.ggml.tokens": null
            },
            "tensors": [
                { "name": "rope_freqs.weight", "type": "F32", "shape": [64] },
                { "name": "token_embd.weight", "type": "Q4_K_S", "shape": [3072, 128256] }
            ],
            "capabilities": ["completion", "tools"],
            "modified_at": "2025-04-29T21:24:41.445877632+03:00"
        });

        let result: ModelShow = serde_json::from_value(response).unwrap();
        assert!(result.supports_tools());
        assert!(result.capabilities.contains(&"tools".to_string()));
        assert!(result.capabilities.contains(&"completion".to_string()));
    }

    #[test]
    fn serialize_chat_request_with_images() {
        let base64_image = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";

        let request = ChatRequest {
            model: "llava".to_string(),
            messages: vec![ChatMessage::User {
                content: "What do you see in this image?".to_string(),
                images: Some(vec![base64_image.to_string()]),
            }],
            stream: false,
            keep_alive: KeepAlive::default(),
            options: None,
            think: None,
            tools: vec![],
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("images"));
        assert!(serialized.contains(base64_image));
    }

    #[test]
    fn serialize_chat_request_without_images() {
        let request = ChatRequest {
            model: "llama3.2".to_string(),
            messages: vec![ChatMessage::User {
                content: "Hello, world!".to_string(),
                images: None,
            }],
            stream: false,
            keep_alive: KeepAlive::default(),
            options: None,
            think: None,
            tools: vec![],
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(!serialized.contains("images"));
    }

    #[test]
    fn test_json_format_with_images() {
        let base64_image = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";

        let request = ChatRequest {
            model: "llava".to_string(),
            messages: vec![ChatMessage::User {
                content: "What do you see?".to_string(),
                images: Some(vec![base64_image.to_string()]),
            }],
            stream: false,
            keep_alive: KeepAlive::default(),
            options: None,
            think: None,
            tools: vec![],
        };

        let serialized = serde_json::to_string(&request).unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        let message_images = parsed["messages"][0]["images"].as_array().unwrap();
        assert_eq!(message_images.len(), 1);
        assert_eq!(message_images[0].as_str().unwrap(), base64_image);
    }

    #[test]
    fn test_discover_available_models() {
        use crate::fake::FakeHttpClient;
        use std::sync::Arc;

        let fake_http_client = Arc::new(FakeHttpClient::new());

        // Mock /api/tags response (list models)
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "qwen2.5-coder:3b",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "qwen2",
                        "families": ["qwen2"],
                        "parameter_size": "3B",
                        "quantization_level": "Q4_0"
                    }
                },
                {
                    "name": "codellama:7b-code",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 2000000,
                    "digest": "def456",
                    "details": {
                        "format": "gguf",
                        "family": "codellama",
                        "families": ["codellama"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                },
                {
                    "name": "nomic-embed-text",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 500000,
                    "digest": "ghi789",
                    "details": {
                        "format": "gguf",
                        "family": "nomic-embed",
                        "families": ["nomic-embed"],
                        "parameter_size": "137M",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());

        // Mock /api/show responses for model capabilities
        let capabilities_response = serde_json::json!({
            "capabilities": ["tools", "thinking"]
        });
        fake_http_client.set_response("/api/show", capabilities_response.to_string());

        // Test the shared discover_available_models function
        let models = futures::executor::block_on(discover_available_models(
            fake_http_client,
            "http://localhost:11434",
            None,
        ))
        .unwrap();

        // Should have 2 models (excluding the embedding model)
        assert_eq!(models.len(), 2);

        let model_names: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
        assert!(model_names.contains(&"codellama:7b-code"));
        assert!(model_names.contains(&"qwen2.5-coder:3b"));
        assert!(!model_names.contains(&"nomic-embed-text"));

        // Verify models are sorted alphabetically
        assert_eq!(models[0].name, "codellama:7b-code");
        assert_eq!(models[1].name, "qwen2.5-coder:3b");

        // Verify capabilities were detected
        assert_eq!(models[0].supports_tools, Some(true));
        assert_eq!(models[0].supports_thinking, Some(true));
        assert_eq!(models[1].supports_tools, Some(true));
        assert_eq!(models[1].supports_thinking, Some(true));
    }

    #[test]
    fn test_discover_available_models_api_failure() {
        use crate::fake::FakeHttpClient;
        use std::sync::Arc;

        let fake_http_client = Arc::new(FakeHttpClient::new());
        fake_http_client.set_error("Connection refused");

        // Test that API failure is properly handled
        let result = futures::executor::block_on(discover_available_models(
            fake_http_client,
            "http://localhost:11434",
            None,
        ));

        assert!(result.is_err());
    }
}
