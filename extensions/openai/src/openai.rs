use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use zed_extension_api::http_client::{HttpMethod, HttpRequest, HttpResponseStream, RedirectPolicy};
use zed_extension_api::{self as zed, *};

struct OpenAiProvider {
    streams: Mutex<HashMap<String, StreamState>>,
    next_stream_id: Mutex<u64>,
}

struct StreamState {
    response_stream: Option<HttpResponseStream>,
    buffer: String,
    started: bool,
    tool_calls: HashMap<usize, AccumulatedToolCall>,
    tool_calls_emitted: bool,
}

#[derive(Clone, Default)]
struct AccumulatedToolCall {
    id: String,
    name: String,
    arguments: String,
}

struct ModelDefinition {
    real_id: &'static str,
    display_name: &'static str,
    max_tokens: u64,
    max_output_tokens: Option<u64>,
    supports_images: bool,
    is_default: bool,
    is_default_fast: bool,
}

const MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        real_id: "gpt-4o",
        display_name: "GPT-4o",
        max_tokens: 128_000,
        max_output_tokens: Some(16_384),
        supports_images: true,
        is_default: true,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "gpt-4o-mini",
        display_name: "GPT-4o-mini",
        max_tokens: 128_000,
        max_output_tokens: Some(16_384),
        supports_images: true,
        is_default: false,
        is_default_fast: true,
    },
    ModelDefinition {
        real_id: "gpt-4.1",
        display_name: "GPT-4.1",
        max_tokens: 1_047_576,
        max_output_tokens: Some(32_768),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "gpt-4.1-mini",
        display_name: "GPT-4.1-mini",
        max_tokens: 1_047_576,
        max_output_tokens: Some(32_768),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "gpt-4.1-nano",
        display_name: "GPT-4.1-nano",
        max_tokens: 1_047_576,
        max_output_tokens: Some(32_768),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "gpt-5",
        display_name: "GPT-5",
        max_tokens: 272_000,
        max_output_tokens: Some(32_768),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "gpt-5-mini",
        display_name: "GPT-5-mini",
        max_tokens: 272_000,
        max_output_tokens: Some(32_768),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "o1",
        display_name: "o1",
        max_tokens: 200_000,
        max_output_tokens: Some(100_000),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "o3",
        display_name: "o3",
        max_tokens: 200_000,
        max_output_tokens: Some(100_000),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "o3-mini",
        display_name: "o3-mini",
        max_tokens: 200_000,
        max_output_tokens: Some(100_000),
        supports_images: false,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "o4-mini",
        display_name: "o4-mini",
        max_tokens: 200_000,
        max_output_tokens: Some(100_000),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
];

fn get_real_model_id(display_name: &str) -> Option<&'static str> {
    MODELS
        .iter()
        .find(|m| m.display_name == display_name)
        .map(|m| m.real_id)
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stop: Vec<String>,
    stream: bool,
    stream_options: Option<StreamOptions>,
}

#[derive(Serialize)]
struct StreamOptions {
    include_usage: bool,
}

#[derive(Serialize)]
#[serde(tag = "role")]
enum OpenAiMessage {
    #[serde(rename = "system")]
    System { content: String },
    #[serde(rename = "user")]
    User { content: Vec<OpenAiContentPart> },
    #[serde(rename = "assistant")]
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<OpenAiToolCall>>,
    },
    #[serde(rename = "tool")]
    Tool {
        tool_call_id: String,
        content: String,
    },
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum OpenAiContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct OpenAiToolCall {
    id: String,
    #[serde(rename = "type")]
    call_type: String,
    function: OpenAiFunctionCall,
}

#[derive(Serialize, Deserialize, Clone)]
struct OpenAiFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Serialize)]
struct OpenAiTool {
    #[serde(rename = "type")]
    tool_type: String,
    function: OpenAiFunctionDef,
}

#[derive(Serialize)]
struct OpenAiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct OpenAiStreamEvent {
    choices: Vec<OpenAiChoice>,
    #[serde(default)]
    usage: Option<OpenAiUsage>,
}

#[derive(Deserialize, Debug)]
struct OpenAiChoice {
    delta: OpenAiDelta,
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct OpenAiDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<OpenAiToolCallDelta>>,
}

#[derive(Deserialize, Debug)]
struct OpenAiToolCallDelta {
    index: usize,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    function: Option<OpenAiFunctionDelta>,
}

#[derive(Deserialize, Debug)]
struct OpenAiFunctionDelta {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
}

#[derive(Deserialize, Debug)]
struct OpenAiUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OpenAiError {
    error: OpenAiErrorDetail,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OpenAiErrorDetail {
    message: String,
}

fn convert_request(
    model_id: &str,
    request: &LlmCompletionRequest,
) -> Result<OpenAiRequest, String> {
    let real_model_id =
        get_real_model_id(model_id).ok_or_else(|| format!("Unknown model: {}", model_id))?;

    let mut messages = Vec::new();

    for msg in &request.messages {
        match msg.role {
            LlmMessageRole::System => {
                let text: String = msg
                    .content
                    .iter()
                    .filter_map(|c| match c {
                        LlmMessageContent::Text(t) => Some(t.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                if !text.is_empty() {
                    messages.push(OpenAiMessage::System { content: text });
                }
            }
            LlmMessageRole::User => {
                let parts: Vec<OpenAiContentPart> = msg
                    .content
                    .iter()
                    .filter_map(|c| match c {
                        LlmMessageContent::Text(t) => {
                            Some(OpenAiContentPart::Text { text: t.clone() })
                        }
                        LlmMessageContent::Image(img) => Some(OpenAiContentPart::ImageUrl {
                            image_url: ImageUrl {
                                url: format!("data:image/png;base64,{}", img.source),
                            },
                        }),
                        LlmMessageContent::ToolResult(_) => None,
                        _ => None,
                    })
                    .collect();

                for content in &msg.content {
                    if let LlmMessageContent::ToolResult(result) = content {
                        let content_text = match &result.content {
                            LlmToolResultContent::Text(t) => t.clone(),
                            LlmToolResultContent::Image(_) => "[Image]".to_string(),
                        };
                        messages.push(OpenAiMessage::Tool {
                            tool_call_id: result.tool_use_id.clone(),
                            content: content_text,
                        });
                    }
                }

                if !parts.is_empty() {
                    messages.push(OpenAiMessage::User { content: parts });
                }
            }
            LlmMessageRole::Assistant => {
                let mut content_text: Option<String> = None;
                let mut tool_calls: Vec<OpenAiToolCall> = Vec::new();

                for c in &msg.content {
                    match c {
                        LlmMessageContent::Text(t) => {
                            content_text = Some(t.clone());
                        }
                        LlmMessageContent::ToolUse(tool_use) => {
                            tool_calls.push(OpenAiToolCall {
                                id: tool_use.id.clone(),
                                call_type: "function".to_string(),
                                function: OpenAiFunctionCall {
                                    name: tool_use.name.clone(),
                                    arguments: tool_use.input.clone(),
                                },
                            });
                        }
                        _ => {}
                    }
                }

                messages.push(OpenAiMessage::Assistant {
                    content: content_text,
                    tool_calls: if tool_calls.is_empty() {
                        None
                    } else {
                        Some(tool_calls)
                    },
                });
            }
        }
    }

    let tools: Option<Vec<OpenAiTool>> = if request.tools.is_empty() {
        None
    } else {
        Some(
            request
                .tools
                .iter()
                .map(|t| OpenAiTool {
                    tool_type: "function".to_string(),
                    function: OpenAiFunctionDef {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: serde_json::from_str(&t.input_schema)
                            .unwrap_or(serde_json::Value::Object(Default::default())),
                    },
                })
                .collect(),
        )
    };

    let tool_choice = request.tool_choice.as_ref().map(|tc| match tc {
        LlmToolChoice::Auto => "auto".to_string(),
        LlmToolChoice::Any => "required".to_string(),
        LlmToolChoice::None => "none".to_string(),
    });

    Ok(OpenAiRequest {
        model: real_model_id.to_string(),
        messages,
        tools,
        tool_choice,
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stop: request.stop_sequences.clone(),
        stream: true,
        stream_options: Some(StreamOptions {
            include_usage: true,
        }),
    })
}

fn parse_sse_line(line: &str) -> Option<OpenAiStreamEvent> {
    if let Some(data) = line.strip_prefix("data: ") {
        if data == "[DONE]" {
            return None;
        }
        serde_json::from_str(data).ok()
    } else {
        None
    }
}

impl zed::Extension for OpenAiProvider {
    fn new() -> Self {
        Self {
            streams: Mutex::new(HashMap::new()),
            next_stream_id: Mutex::new(0),
        }
    }

    fn llm_providers(&self) -> Vec<LlmProviderInfo> {
        vec![LlmProviderInfo {
            id: "openai".into(),
            name: "OpenAI".into(),
            icon: Some("icons/openai.svg".into()),
        }]
    }

    fn llm_provider_models(&self, _provider_id: &str) -> Result<Vec<LlmModelInfo>, String> {
        Ok(MODELS
            .iter()
            .map(|m| LlmModelInfo {
                id: m.display_name.to_string(),
                name: m.display_name.to_string(),
                max_token_count: m.max_tokens,
                max_output_tokens: m.max_output_tokens,
                capabilities: LlmModelCapabilities {
                    supports_images: m.supports_images,
                    supports_tools: true,
                    supports_tool_choice_auto: true,
                    supports_tool_choice_any: true,
                    supports_tool_choice_none: true,
                    supports_thinking: false,
                    tool_input_format: LlmToolInputFormat::JsonSchema,
                },
                is_default: m.is_default,
                is_default_fast: m.is_default_fast,
            })
            .collect())
    }

    fn llm_provider_is_authenticated(&self, _provider_id: &str) -> bool {
        llm_get_credential("openai").is_some()
    }

    fn llm_provider_settings_markdown(&self, _provider_id: &str) -> Option<String> {
        Some(
            r#"# OpenAI Setup

Welcome to **OpenAI**! This extension provides access to OpenAI GPT models.

## Configuration

Enter your OpenAI API key below. You can find your API key at [platform.openai.com/api-keys](https://platform.openai.com/api-keys).

## Available Models

| Display Name | Real Model | Context | Output |
|--------------|------------|---------|--------|
| GPT-4o | gpt-4o | 128K | 16K |
| GPT-4o-mini | gpt-4o-mini | 128K | 16K |
| GPT-4.1 | gpt-4.1 | 1M | 32K |
| GPT-4.1-mini | gpt-4.1-mini | 1M | 32K |
| GPT-5 | gpt-5 | 272K | 32K |
| GPT-5-mini | gpt-5-mini | 272K | 32K |
| o1 | o1 | 200K | 100K |
| o3 | o3 | 200K | 100K |
| o3-mini | o3-mini | 200K | 100K |

## Features

- ✅ Full streaming support
- ✅ Tool/function calling
- ✅ Vision (image inputs)
- ✅ All OpenAI models

## Pricing

Uses your OpenAI API credits. See [OpenAI pricing](https://openai.com/pricing) for details.
"#
            .to_string(),
        )
    }

    fn llm_provider_authenticate(&mut self, _provider_id: &str) -> Result<(), String> {
        let provided = llm_request_credential(
            "openai",
            LlmCredentialType::ApiKey,
            "OpenAI API Key",
            "sk-...",
        )?;
        if provided {
            Ok(())
        } else {
            Err("Authentication cancelled".to_string())
        }
    }

    fn llm_provider_reset_credentials(&mut self, _provider_id: &str) -> Result<(), String> {
        llm_delete_credential("openai")
    }

    fn llm_stream_completion_start(
        &mut self,
        _provider_id: &str,
        model_id: &str,
        request: &LlmCompletionRequest,
    ) -> Result<String, String> {
        let api_key = llm_get_credential("openai").ok_or_else(|| {
            "No API key configured. Please add your OpenAI API key in settings.".to_string()
        })?;

        let openai_request = convert_request(model_id, request)?;

        let body = serde_json::to_vec(&openai_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let http_request = HttpRequest {
            method: HttpMethod::Post,
            url: "https://api.openai.com/v1/chat/completions".to_string(),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Authorization".to_string(), format!("Bearer {}", api_key)),
            ],
            body: Some(body),
            redirect_policy: RedirectPolicy::FollowAll,
        };

        let response_stream = http_request
            .fetch_stream()
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        let stream_id = {
            let mut id_counter = self.next_stream_id.lock().unwrap();
            let id = format!("openai-stream-{}", *id_counter);
            *id_counter += 1;
            id
        };

        self.streams.lock().unwrap().insert(
            stream_id.clone(),
            StreamState {
                response_stream: Some(response_stream),
                buffer: String::new(),
                started: false,
                tool_calls: HashMap::new(),
                tool_calls_emitted: false,
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
                let line = state.buffer[..newline_pos].trim().to_string();
                state.buffer = state.buffer[newline_pos + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                if let Some(event) = parse_sse_line(&line) {
                    if let Some(choice) = event.choices.first() {
                        if let Some(tool_calls) = &choice.delta.tool_calls {
                            for tc in tool_calls {
                                let entry = state.tool_calls.entry(tc.index).or_default();

                                if let Some(id) = &tc.id {
                                    entry.id = id.clone();
                                }

                                if let Some(func) = &tc.function {
                                    if let Some(name) = &func.name {
                                        entry.name = name.clone();
                                    }
                                    if let Some(args) = &func.arguments {
                                        entry.arguments.push_str(args);
                                    }
                                }
                            }
                        }

                        if let Some(reason) = &choice.finish_reason {
                            if reason == "tool_calls" && !state.tool_calls_emitted {
                                state.tool_calls_emitted = true;
                                if let Some((&index, _)) = state.tool_calls.iter().next() {
                                    if let Some(tool_call) = state.tool_calls.remove(&index) {
                                        return Ok(Some(LlmCompletionEvent::ToolUse(LlmToolUse {
                                            id: tool_call.id,
                                            name: tool_call.name,
                                            input: tool_call.arguments,
                                            thought_signature: None,
                                        })));
                                    }
                                }
                            }

                            let stop_reason = match reason.as_str() {
                                "stop" => LlmStopReason::EndTurn,
                                "length" => LlmStopReason::MaxTokens,
                                "tool_calls" => LlmStopReason::ToolUse,
                                "content_filter" => LlmStopReason::Refusal,
                                _ => LlmStopReason::EndTurn,
                            };

                            if let Some(usage) = event.usage {
                                return Ok(Some(LlmCompletionEvent::Usage(LlmTokenUsage {
                                    input_tokens: usage.prompt_tokens,
                                    output_tokens: usage.completion_tokens,
                                    cache_creation_input_tokens: None,
                                    cache_read_input_tokens: None,
                                })));
                            }

                            return Ok(Some(LlmCompletionEvent::Stop(stop_reason)));
                        }

                        if let Some(content) = &choice.delta.content {
                            if !content.is_empty() {
                                return Ok(Some(LlmCompletionEvent::Text(content.clone())));
                            }
                        }
                    }

                    if event.choices.is_empty() {
                        if let Some(usage) = event.usage {
                            return Ok(Some(LlmCompletionEvent::Usage(LlmTokenUsage {
                                input_tokens: usage.prompt_tokens,
                                output_tokens: usage.completion_tokens,
                                cache_creation_input_tokens: None,
                                cache_read_input_tokens: None,
                            })));
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
                    if !state.tool_calls.is_empty() && !state.tool_calls_emitted {
                        state.tool_calls_emitted = true;
                        let keys: Vec<usize> = state.tool_calls.keys().copied().collect();
                        if let Some(&key) = keys.first() {
                            if let Some(tool_call) = state.tool_calls.remove(&key) {
                                return Ok(Some(LlmCompletionEvent::ToolUse(LlmToolUse {
                                    id: tool_call.id,
                                    name: tool_call.name,
                                    input: tool_call.arguments,
                                    thought_signature: None,
                                })));
                            }
                        }
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

zed::register_extension!(OpenAiProvider);
