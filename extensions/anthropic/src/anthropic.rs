use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use zed_extension_api::http_client::{HttpMethod, HttpRequest, HttpResponseStream, RedirectPolicy};
use zed_extension_api::{self as zed, *};

struct AnthropicProvider {
    streams: Mutex<HashMap<String, StreamState>>,
    next_stream_id: Mutex<u64>,
}

struct StreamState {
    response_stream: Option<HttpResponseStream>,
    buffer: String,
    started: bool,
    current_tool_use: Option<ToolUseState>,
    stop_reason: Option<LlmStopReason>,
    pending_signature: Option<String>,
}

struct ToolUseState {
    id: String,
    name: String,
    input_json: String,
}

struct ModelDefinition {
    real_id: &'static str,
    display_name: &'static str,
    max_tokens: u64,
    max_output_tokens: u64,
    supports_images: bool,
    supports_thinking: bool,
    is_default: bool,
    is_default_fast: bool,
}

const MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        real_id: "claude-opus-4-5-20251101",
        display_name: "Claude Opus 4.5",
        max_tokens: 200_000,
        max_output_tokens: 8_192,
        supports_images: true,
        supports_thinking: false,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "claude-opus-4-5-20251101",
        display_name: "Claude Opus 4.5 Thinking",
        max_tokens: 200_000,
        max_output_tokens: 8_192,
        supports_images: true,
        supports_thinking: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "claude-sonnet-4-5-20250929",
        display_name: "Claude Sonnet 4.5",
        max_tokens: 200_000,
        max_output_tokens: 8_192,
        supports_images: true,
        supports_thinking: false,
        is_default: true,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "claude-sonnet-4-5-20250929",
        display_name: "Claude Sonnet 4.5 Thinking",
        max_tokens: 200_000,
        max_output_tokens: 8_192,
        supports_images: true,
        supports_thinking: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "claude-sonnet-4-20250514",
        display_name: "Claude Sonnet 4",
        max_tokens: 200_000,
        max_output_tokens: 8_192,
        supports_images: true,
        supports_thinking: false,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "claude-sonnet-4-20250514",
        display_name: "Claude Sonnet 4 Thinking",
        max_tokens: 200_000,
        max_output_tokens: 8_192,
        supports_images: true,
        supports_thinking: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "claude-haiku-4-5-20251001",
        display_name: "Claude Haiku 4.5",
        max_tokens: 200_000,
        max_output_tokens: 64_000,
        supports_images: true,
        supports_thinking: false,
        is_default: false,
        is_default_fast: true,
    },
    ModelDefinition {
        real_id: "claude-haiku-4-5-20251001",
        display_name: "Claude Haiku 4.5 Thinking",
        max_tokens: 200_000,
        max_output_tokens: 64_000,
        supports_images: true,
        supports_thinking: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "claude-3-5-sonnet-latest",
        display_name: "Claude 3.5 Sonnet",
        max_tokens: 200_000,
        max_output_tokens: 8_192,
        supports_images: true,
        supports_thinking: false,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "claude-3-5-haiku-latest",
        display_name: "Claude 3.5 Haiku",
        max_tokens: 200_000,
        max_output_tokens: 8_192,
        supports_images: true,
        supports_thinking: false,
        is_default: false,
        is_default_fast: false,
    },
];

fn get_model_definition(display_name: &str) -> Option<&'static ModelDefinition> {
    MODELS.iter().find(|m| m.display_name == display_name)
}

// Anthropic API Request Types

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u64,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking: Option<AnthropicThinking>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<AnthropicTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<AnthropicToolChoice>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stop_sequences: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
}

#[derive(Serialize)]
struct AnthropicThinking {
    #[serde(rename = "type")]
    thinking_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    budget_tokens: Option<u32>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContent>,
}

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
enum AnthropicContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String, signature: String },
    #[serde(rename = "redacted_thinking")]
    RedactedThinking { data: String },
    #[serde(rename = "image")]
    Image { source: AnthropicImageSource },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        is_error: bool,
        content: String,
    },
}

#[derive(Serialize, Clone)]
struct AnthropicImageSource {
    #[serde(rename = "type")]
    source_type: String,
    media_type: String,
    data: String,
}

#[derive(Serialize)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum AnthropicToolChoice {
    Auto,
    Any,
    None,
}

// Anthropic API Response Types

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum AnthropicEvent {
    #[serde(rename = "message_start")]
    MessageStart { message: AnthropicMessageResponse },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: usize,
        content_block: AnthropicContentBlock,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta { index: usize, delta: AnthropicDelta },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: usize },
    #[serde(rename = "message_delta")]
    MessageDelta {
        delta: AnthropicMessageDelta,
        usage: AnthropicUsage,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "error")]
    Error { error: AnthropicApiError },
}

#[derive(Deserialize, Debug)]
struct AnthropicMessageResponse {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    role: String,
    #[serde(default)]
    usage: AnthropicUsage,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String },
    #[serde(rename = "redacted_thinking")]
    RedactedThinking { data: String },
    #[serde(rename = "tool_use")]
    ToolUse { id: String, name: String },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum AnthropicDelta {
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "thinking_delta")]
    ThinkingDelta { thinking: String },
    #[serde(rename = "signature_delta")]
    SignatureDelta { signature: String },
    #[serde(rename = "input_json_delta")]
    InputJsonDelta { partial_json: String },
}

#[derive(Deserialize, Debug)]
struct AnthropicMessageDelta {
    stop_reason: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct AnthropicUsage {
    #[serde(default)]
    input_tokens: Option<u64>,
    #[serde(default)]
    output_tokens: Option<u64>,
    #[serde(default)]
    cache_creation_input_tokens: Option<u64>,
    #[serde(default)]
    cache_read_input_tokens: Option<u64>,
}

#[derive(Deserialize, Debug)]
struct AnthropicApiError {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    error_type: String,
    message: String,
}

fn convert_request(
    model_id: &str,
    request: &LlmCompletionRequest,
) -> Result<AnthropicRequest, String> {
    let model_def =
        get_model_definition(model_id).ok_or_else(|| format!("Unknown model: {}", model_id))?;

    let mut messages: Vec<AnthropicMessage> = Vec::new();
    let mut system_message = String::new();

    for msg in &request.messages {
        match msg.role {
            LlmMessageRole::System => {
                for content in &msg.content {
                    if let LlmMessageContent::Text(text) = content {
                        if !system_message.is_empty() {
                            system_message.push('\n');
                        }
                        system_message.push_str(text);
                    }
                }
            }
            LlmMessageRole::User => {
                let mut contents: Vec<AnthropicContent> = Vec::new();

                for content in &msg.content {
                    match content {
                        LlmMessageContent::Text(text) => {
                            if !text.is_empty() {
                                contents.push(AnthropicContent::Text { text: text.clone() });
                            }
                        }
                        LlmMessageContent::Image(img) => {
                            contents.push(AnthropicContent::Image {
                                source: AnthropicImageSource {
                                    source_type: "base64".to_string(),
                                    media_type: "image/png".to_string(),
                                    data: img.source.clone(),
                                },
                            });
                        }
                        LlmMessageContent::ToolResult(result) => {
                            let content_text = match &result.content {
                                LlmToolResultContent::Text(t) => t.clone(),
                                LlmToolResultContent::Image(_) => "[Image]".to_string(),
                            };
                            contents.push(AnthropicContent::ToolResult {
                                tool_use_id: result.tool_use_id.clone(),
                                is_error: result.is_error,
                                content: content_text,
                            });
                        }
                        _ => {}
                    }
                }

                if !contents.is_empty() {
                    messages.push(AnthropicMessage {
                        role: "user".to_string(),
                        content: contents,
                    });
                }
            }
            LlmMessageRole::Assistant => {
                let mut contents: Vec<AnthropicContent> = Vec::new();

                for content in &msg.content {
                    match content {
                        LlmMessageContent::Text(text) => {
                            if !text.is_empty() {
                                contents.push(AnthropicContent::Text { text: text.clone() });
                            }
                        }
                        LlmMessageContent::ToolUse(tool_use) => {
                            let input: serde_json::Value =
                                serde_json::from_str(&tool_use.input).unwrap_or_default();
                            contents.push(AnthropicContent::ToolUse {
                                id: tool_use.id.clone(),
                                name: tool_use.name.clone(),
                                input,
                            });
                        }
                        LlmMessageContent::Thinking(thinking) => {
                            if !thinking.text.is_empty() {
                                contents.push(AnthropicContent::Thinking {
                                    thinking: thinking.text.clone(),
                                    signature: thinking.signature.clone().unwrap_or_default(),
                                });
                            }
                        }
                        LlmMessageContent::RedactedThinking(data) => {
                            if !data.is_empty() {
                                contents.push(AnthropicContent::RedactedThinking {
                                    data: data.clone(),
                                });
                            }
                        }
                        _ => {}
                    }
                }

                if !contents.is_empty() {
                    messages.push(AnthropicMessage {
                        role: "assistant".to_string(),
                        content: contents,
                    });
                }
            }
        }
    }

    let tools: Vec<AnthropicTool> = request
        .tools
        .iter()
        .map(|t| AnthropicTool {
            name: t.name.clone(),
            description: t.description.clone(),
            input_schema: serde_json::from_str(&t.input_schema)
                .unwrap_or(serde_json::Value::Object(Default::default())),
        })
        .collect();

    let tool_choice = request.tool_choice.as_ref().map(|tc| match tc {
        LlmToolChoice::Auto => AnthropicToolChoice::Auto,
        LlmToolChoice::Any => AnthropicToolChoice::Any,
        LlmToolChoice::None => AnthropicToolChoice::None,
    });

    let thinking = if model_def.supports_thinking && request.thinking_allowed {
        Some(AnthropicThinking {
            thinking_type: "enabled".to_string(),
            budget_tokens: Some(4096),
        })
    } else {
        None
    };

    Ok(AnthropicRequest {
        model: model_def.real_id.to_string(),
        max_tokens: model_def.max_output_tokens,
        messages,
        system: if system_message.is_empty() {
            None
        } else {
            Some(system_message)
        },
        thinking,
        tools,
        tool_choice,
        stop_sequences: request.stop_sequences.clone(),
        temperature: request.temperature,
        stream: true,
    })
}

fn parse_sse_line(line: &str) -> Option<AnthropicEvent> {
    let data = line.strip_prefix("data: ")?;
    serde_json::from_str(data).ok()
}

impl zed::Extension for AnthropicProvider {
    fn new() -> Self {
        Self {
            streams: Mutex::new(HashMap::new()),
            next_stream_id: Mutex::new(0),
        }
    }

    fn llm_providers(&self) -> Vec<LlmProviderInfo> {
        vec![LlmProviderInfo {
            id: "anthropic".into(),
            name: "Anthropic".into(),
            icon: Some("anthropic".into()),
        }]
    }

    fn llm_provider_models(&self, _provider_id: &str) -> Result<Vec<LlmModelInfo>, String> {
        Ok(MODELS
            .iter()
            .map(|m| LlmModelInfo {
                id: m.display_name.to_string(),
                name: m.display_name.to_string(),
                max_token_count: m.max_tokens,
                max_output_tokens: Some(m.max_output_tokens),
                capabilities: LlmModelCapabilities {
                    supports_images: m.supports_images,
                    supports_tools: true,
                    supports_tool_choice_auto: true,
                    supports_tool_choice_any: true,
                    supports_tool_choice_none: true,
                    supports_thinking: m.supports_thinking,
                    tool_input_format: LlmToolInputFormat::JsonSchema,
                },
                is_default: m.is_default,
                is_default_fast: m.is_default_fast,
            })
            .collect())
    }

    fn llm_provider_is_authenticated(&self, _provider_id: &str) -> bool {
        llm_get_credential("anthropic").is_some()
    }

    fn llm_provider_settings_markdown(&self, _provider_id: &str) -> Option<String> {
        Some(
            r#"# Anthropic Setup

Welcome to **Anthropic**! This extension provides access to Claude models.

## Configuration

Enter your Anthropic API key below. You can get your API key at [console.anthropic.com](https://console.anthropic.com/).

## Available Models

| Display Name | Real Model | Context | Output |
|--------------|------------|---------|--------|
| Claude Opus 4.5 | claude-opus-4-5 | 200K | 8K |
| Claude Opus 4.5 Thinking | claude-opus-4-5 | 200K | 8K |
| Claude Sonnet 4.5 | claude-sonnet-4-5 | 200K | 8K |
| Claude Sonnet 4.5 Thinking | claude-sonnet-4-5 | 200K | 8K |
| Claude Sonnet 4 | claude-sonnet-4 | 200K | 8K |
| Claude Sonnet 4 Thinking | claude-sonnet-4 | 200K | 8K |
| Claude Haiku 4.5 | claude-haiku-4-5 | 200K | 64K |
| Claude Haiku 4.5 Thinking | claude-haiku-4-5 | 200K | 64K |
| Claude 3.5 Sonnet | claude-3-5-sonnet | 200K | 8K |
| Claude 3.5 Haiku | claude-3-5-haiku | 200K | 8K |

## Features

- ✅ Full streaming support
- ✅ Tool/function calling
- ✅ Vision (image inputs)
- ✅ Extended thinking support
- ✅ All Claude models

## Pricing

Uses your Anthropic API credits. See [Anthropic pricing](https://www.anthropic.com/pricing) for details.
"#
            .to_string(),
        )
    }

    fn llm_provider_authenticate(&mut self, _provider_id: &str) -> Result<(), String> {
        let provided = llm_request_credential(
            "anthropic",
            LlmCredentialType::ApiKey,
            "Anthropic API Key",
            "sk-ant-...",
        )?;
        if provided {
            Ok(())
        } else {
            Err("Authentication cancelled".to_string())
        }
    }

    fn llm_provider_reset_credentials(&mut self, _provider_id: &str) -> Result<(), String> {
        llm_delete_credential("anthropic")
    }

    fn llm_stream_completion_start(
        &mut self,
        _provider_id: &str,
        model_id: &str,
        request: &LlmCompletionRequest,
    ) -> Result<String, String> {
        let api_key = llm_get_credential("anthropic").ok_or_else(|| {
            "No API key configured. Please add your Anthropic API key in settings.".to_string()
        })?;

        let anthropic_request = convert_request(model_id, request)?;

        let body = serde_json::to_vec(&anthropic_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let http_request = HttpRequest {
            method: HttpMethod::Post,
            url: "https://api.anthropic.com/v1/messages".to_string(),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("x-api-key".to_string(), api_key),
                ("anthropic-version".to_string(), "2023-06-01".to_string()),
            ],
            body: Some(body),
            redirect_policy: RedirectPolicy::FollowAll,
        };

        let response_stream = http_request
            .fetch_stream()
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        let stream_id = {
            let mut id_counter = self.next_stream_id.lock().unwrap();
            let id = format!("anthropic-stream-{}", *id_counter);
            *id_counter += 1;
            id
        };

        self.streams.lock().unwrap().insert(
            stream_id.clone(),
            StreamState {
                response_stream: Some(response_stream),
                buffer: String::new(),
                started: false,
                current_tool_use: None,
                stop_reason: None,
                pending_signature: None,
            },
        );

        Ok(stream_id)
    }

    fn llm_stream_completion_next(
        &mut self,
        stream_id: &str,
    ) -> Result<Option<LlmCompletionEvent>, String> {
        let mut streams = self.streams.lock().unwrap();
        let state = streams
            .get_mut(stream_id)
            .ok_or_else(|| format!("Unknown stream: {}", stream_id))?;

        if !state.started {
            state.started = true;
            return Ok(Some(LlmCompletionEvent::Started));
        }

        let response_stream = state
            .response_stream
            .as_mut()
            .ok_or_else(|| "Stream already closed".to_string())?;

        loop {
            if let Some(newline_pos) = state.buffer.find('\n') {
                let line = state.buffer[..newline_pos].to_string();
                state.buffer = state.buffer[newline_pos + 1..].to_string();

                if line.trim().is_empty() || line.starts_with("event:") {
                    continue;
                }

                if let Some(event) = parse_sse_line(&line) {
                    match event {
                        AnthropicEvent::MessageStart { message } => {
                            if let (Some(input), Some(output)) =
                                (message.usage.input_tokens, message.usage.output_tokens)
                            {
                                return Ok(Some(LlmCompletionEvent::Usage(LlmTokenUsage {
                                    input_tokens: input,
                                    output_tokens: output,
                                    cache_creation_input_tokens: message
                                        .usage
                                        .cache_creation_input_tokens,
                                    cache_read_input_tokens: message.usage.cache_read_input_tokens,
                                })));
                            }
                        }
                        AnthropicEvent::ContentBlockStart { content_block, .. } => {
                            match content_block {
                                AnthropicContentBlock::Text { text } => {
                                    if !text.is_empty() {
                                        return Ok(Some(LlmCompletionEvent::Text(text)));
                                    }
                                }
                                AnthropicContentBlock::Thinking { thinking } => {
                                    return Ok(Some(LlmCompletionEvent::Thinking(
                                        LlmThinkingContent {
                                            text: thinking,
                                            signature: None,
                                        },
                                    )));
                                }
                                AnthropicContentBlock::RedactedThinking { data } => {
                                    return Ok(Some(LlmCompletionEvent::RedactedThinking(data)));
                                }
                                AnthropicContentBlock::ToolUse { id, name } => {
                                    state.current_tool_use = Some(ToolUseState {
                                        id,
                                        name,
                                        input_json: String::new(),
                                    });
                                }
                            }
                        }
                        AnthropicEvent::ContentBlockDelta { delta, .. } => match delta {
                            AnthropicDelta::TextDelta { text } => {
                                if !text.is_empty() {
                                    return Ok(Some(LlmCompletionEvent::Text(text)));
                                }
                            }
                            AnthropicDelta::ThinkingDelta { thinking } => {
                                return Ok(Some(LlmCompletionEvent::Thinking(
                                    LlmThinkingContent {
                                        text: thinking,
                                        signature: None,
                                    },
                                )));
                            }
                            AnthropicDelta::SignatureDelta { signature } => {
                                state.pending_signature = Some(signature.clone());
                                return Ok(Some(LlmCompletionEvent::Thinking(
                                    LlmThinkingContent {
                                        text: String::new(),
                                        signature: Some(signature),
                                    },
                                )));
                            }
                            AnthropicDelta::InputJsonDelta { partial_json } => {
                                if let Some(ref mut tool_use) = state.current_tool_use {
                                    tool_use.input_json.push_str(&partial_json);
                                }
                            }
                        },
                        AnthropicEvent::ContentBlockStop { .. } => {
                            if let Some(tool_use) = state.current_tool_use.take() {
                                return Ok(Some(LlmCompletionEvent::ToolUse(LlmToolUse {
                                    id: tool_use.id,
                                    name: tool_use.name,
                                    input: tool_use.input_json,
                                    thought_signature: state.pending_signature.take(),
                                })));
                            }
                        }
                        AnthropicEvent::MessageDelta { delta, usage } => {
                            if let Some(reason) = delta.stop_reason {
                                state.stop_reason = Some(match reason.as_str() {
                                    "end_turn" => LlmStopReason::EndTurn,
                                    "max_tokens" => LlmStopReason::MaxTokens,
                                    "tool_use" => LlmStopReason::ToolUse,
                                    _ => LlmStopReason::EndTurn,
                                });
                            }
                            if let Some(output) = usage.output_tokens {
                                return Ok(Some(LlmCompletionEvent::Usage(LlmTokenUsage {
                                    input_tokens: usage.input_tokens.unwrap_or(0),
                                    output_tokens: output,
                                    cache_creation_input_tokens: usage.cache_creation_input_tokens,
                                    cache_read_input_tokens: usage.cache_read_input_tokens,
                                })));
                            }
                        }
                        AnthropicEvent::MessageStop => {
                            if let Some(stop_reason) = state.stop_reason.take() {
                                return Ok(Some(LlmCompletionEvent::Stop(stop_reason)));
                            }
                            return Ok(Some(LlmCompletionEvent::Stop(LlmStopReason::EndTurn)));
                        }
                        AnthropicEvent::Ping => {}
                        AnthropicEvent::Error { error } => {
                            return Err(format!("API error: {}", error.message));
                        }
                    }
                }

                continue;
            }

            match response_stream.next_chunk() {
                Ok(Some(chunk)) => {
                    let text = String::from_utf8_lossy(&chunk);
                    state.buffer.push_str(&text);
                }
                Ok(None) => {
                    if let Some(stop_reason) = state.stop_reason.take() {
                        return Ok(Some(LlmCompletionEvent::Stop(stop_reason)));
                    }
                    return Ok(None);
                }
                Err(e) => {
                    return Err(format!("Stream error: {}", e));
                }
            }
        }
    }

    fn llm_stream_completion_close(&mut self, stream_id: &str) {
        self.streams.lock().unwrap().remove(stream_id);
    }
}

zed::register_extension!(AnthropicProvider);
