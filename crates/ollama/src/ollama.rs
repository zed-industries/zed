use anyhow::{Context as _, Result, anyhow};
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
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: usize,
    pub keep_alive: Option<KeepAlive>,
    pub supports_tools: bool,
}

fn get_max_tokens(name: &str) -> usize {
    /// Default context length for unknown models.
    const DEFAULT_TOKENS: usize = 2048;
    /// Magic number. Lets many Ollama models work with ~16GB of ram.
    const MAXIMUM_TOKENS: usize = 16384;

    match name.split(':').next().unwrap() {
        "phi" | "tinyllama" | "granite-code" => 2048,
        "llama2" | "yi" | "vicuna" | "stablelm2" => 4096,
        "llama3" | "gemma2" | "gemma" | "codegemma" | "starcoder" | "aya" => 8192,
        "codellama" | "starcoder2" => 16384,
        "mistral" | "codestral" | "mixstral" | "llava" | "qwen2" | "qwen2.5-coder"
        | "dolphin-mixtral" => 32768,
        "llama3.1" | "llama3.2" | "llama3.3" | "phi3" | "phi3.5" | "phi4" | "command-r"
        | "qwen3" | "gemma3" | "deepseek-coder-v2" | "deepseek-v3" | "deepseek-r1" | "yi-coder" => {
            128000
        }
        _ => DEFAULT_TOKENS,
    }
    .clamp(1, MAXIMUM_TOKENS)
}

impl Model {
    pub fn new(
        name: &str,
        display_name: Option<&str>,
        max_tokens: Option<usize>,
        supports_tools: bool,
    ) -> Self {
        Self {
            name: name.to_owned(),
            display_name: display_name
                .map(ToString::to_string)
                .or_else(|| name.strip_suffix(":latest").map(ToString::to_string)),
            max_tokens: max_tokens.unwrap_or_else(|| get_max_tokens(name)),
            keep_alive: Some(KeepAlive::indefinite()),
            supports_tools,
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
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    Assistant {
        content: String,
        tool_calls: Option<Vec<OllamaToolCall>>,
    },
    User {
        content: String,
    },
    System {
        content: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OllamaToolCall {
    Function(OllamaFunctionCall),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaFunctionCall {
    pub name: String,
    pub arguments: Value,
}

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
    pub num_ctx: Option<usize>,
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
        Err(anyhow!(
            "Failed to connect to API: {} {}",
            response.status(),
            body_str
        ))
    }
}

pub async fn stream_chat_completion(
    client: &dyn HttpClient,
    api_url: &str,
    request: ChatRequest,
) -> Result<BoxStream<'static, Result<ChatResponseDelta>>> {
    let uri = format!("{api_url}/api/chat");
    let request_builder = http::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json");

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

        Err(anyhow!(
            "Failed to connect to Ollama API: {} {}",
            response.status(),
            body,
        ))
    }
}

pub async fn get_models(
    client: &dyn HttpClient,
    api_url: &str,
    _: Option<Duration>,
) -> Result<Vec<LocalModelListing>> {
    let uri = format!("{api_url}/api/tags");
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");

    let request = request_builder.body(AsyncBody::default())?;

    let mut response = client.send(request).await?;

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    if response.status().is_success() {
        let response: LocalModelsResponse =
            serde_json::from_str(&body).context("Unable to parse Ollama tag listing")?;

        Ok(response.models)
    } else {
        Err(anyhow!(
            "Failed to connect to Ollama API: {} {}",
            response.status(),
            body,
        ))
    }
}

/// Fetch details of a model, used to determine model capabilities
pub async fn show_model(client: &dyn HttpClient, api_url: &str, model: &str) -> Result<ModelShow> {
    let uri = format!("{api_url}/api/show");
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(AsyncBody::from(
            serde_json::json!({ "model": model }).to_string(),
        ))?;

    let mut response = client.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    if response.status().is_success() {
        let details: ModelShow = serde_json::from_str(body.as_str())?;
        Ok(details)
    } else {
        Err(anyhow!(
            "Failed to connect to Ollama API: {} {}",
            response.status(),
            body,
        ))
    }
}

/// Sends an empty request to Ollama to trigger loading the model
pub async fn preload_model(client: Arc<dyn HttpClient>, api_url: &str, model: &str) -> Result<()> {
    let uri = format!("{api_url}/api/generate");
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(AsyncBody::from(
            serde_json::json!({
                "model": model,
                "keep_alive": "15m",
            })
            .to_string(),
        ))?;

    let mut response = client.send(request).await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        Err(anyhow!(
            "Failed to connect to Ollama API: {} {}",
            response.status(),
            body,
        ))
    }
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
            } => {
                assert!(content.is_empty());
                assert!(tool_calls.is_some_and(|v| !v.is_empty()));
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
}
