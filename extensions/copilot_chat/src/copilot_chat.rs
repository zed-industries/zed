use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use zed_extension_api::http_client::{HttpMethod, HttpRequest, HttpResponseStream, RedirectPolicy};
use zed_extension_api::{self as zed, *};

struct CopilotChatProvider {
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
    id: &'static str,
    display_name: &'static str,
    max_tokens: u64,
    max_output_tokens: Option<u64>,
    supports_images: bool,
    is_default: bool,
    is_default_fast: bool,
}

const MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "gpt-4o",
        display_name: "GPT-4o",
        max_tokens: 128_000,
        max_output_tokens: Some(16_384),
        supports_images: true,
        is_default: true,
        is_default_fast: false,
    },
    ModelDefinition {
        id: "gpt-4o-mini",
        display_name: "GPT-4o Mini",
        max_tokens: 128_000,
        max_output_tokens: Some(16_384),
        supports_images: true,
        is_default: false,
        is_default_fast: true,
    },
    ModelDefinition {
        id: "gpt-4.1",
        display_name: "GPT-4.1",
        max_tokens: 1_000_000,
        max_output_tokens: Some(32_768),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        id: "o1",
        display_name: "o1",
        max_tokens: 200_000,
        max_output_tokens: Some(100_000),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        id: "o3-mini",
        display_name: "o3-mini",
        max_tokens: 200_000,
        max_output_tokens: Some(100_000),
        supports_images: false,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        id: "claude-3.5-sonnet",
        display_name: "Claude 3.5 Sonnet",
        max_tokens: 200_000,
        max_output_tokens: Some(8_192),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        id: "claude-3.7-sonnet",
        display_name: "Claude 3.7 Sonnet",
        max_tokens: 200_000,
        max_output_tokens: Some(8_192),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        id: "gemini-2.0-flash-001",
        display_name: "Gemini 2.0 Flash",
        max_tokens: 1_000_000,
        max_output_tokens: Some(8_192),
        supports_images: true,
        is_default: false,
        is_default_fast: false,
    },
];

fn get_model_definition(model_id: &str) -> Option<&'static ModelDefinition> {
    MODELS.iter().find(|m| m.id == model_id)
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<OpenAiTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stop: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<StreamOptions>,
}

#[derive(Serialize)]
struct StreamOptions {
    include_usage: bool,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<OpenAiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
enum OpenAiContent {
    Text(String),
    Parts(Vec<OpenAiContentPart>),
}

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
enum OpenAiContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize, Clone)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize, Clone)]
struct OpenAiToolCall {
    id: String,
    #[serde(rename = "type")]
    call_type: String,
    function: OpenAiFunctionCall,
}

#[derive(Serialize, Clone)]
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
struct OpenAiStreamResponse {
    choices: Vec<OpenAiStreamChoice>,
    #[serde(default)]
    usage: Option<OpenAiUsage>,
}

#[derive(Deserialize, Debug)]
struct OpenAiStreamChoice {
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

#[derive(Deserialize, Debug, Default)]
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

fn convert_request(
    model_id: &str,
    request: &LlmCompletionRequest,
) -> Result<OpenAiRequest, String> {
    let mut messages: Vec<OpenAiMessage> = Vec::new();

    for msg in &request.messages {
        match msg.role {
            LlmMessageRole::System => {
                let mut text_content = String::new();
                for content in &msg.content {
                    if let LlmMessageContent::Text(text) = content {
                        if !text_content.is_empty() {
                            text_content.push('\n');
                        }
                        text_content.push_str(text);
                    }
                }
                if !text_content.is_empty() {
                    messages.push(OpenAiMessage {
                        role: "system".to_string(),
                        content: Some(OpenAiContent::Text(text_content)),
                        tool_calls: None,
                        tool_call_id: None,
                    });
                }
            }
            LlmMessageRole::User => {
                let mut parts: Vec<OpenAiContentPart> = Vec::new();
                let mut tool_result_messages: Vec<OpenAiMessage> = Vec::new();

                for content in &msg.content {
                    match content {
                        LlmMessageContent::Text(text) => {
                            if !text.is_empty() {
                                parts.push(OpenAiContentPart::Text { text: text.clone() });
                            }
                        }
                        LlmMessageContent::Image(img) => {
                            let data_url = format!("data:image/png;base64,{}", img.source);
                            parts.push(OpenAiContentPart::ImageUrl {
                                image_url: ImageUrl { url: data_url },
                            });
                        }
                        LlmMessageContent::ToolResult(result) => {
                            let content_text = match &result.content {
                                LlmToolResultContent::Text(t) => t.clone(),
                                LlmToolResultContent::Image(_) => "[Image]".to_string(),
                            };
                            tool_result_messages.push(OpenAiMessage {
                                role: "tool".to_string(),
                                content: Some(OpenAiContent::Text(content_text)),
                                tool_calls: None,
                                tool_call_id: Some(result.tool_use_id.clone()),
                            });
                        }
                        _ => {}
                    }
                }

                if !parts.is_empty() {
                    let content = if parts.len() == 1 {
                        if let OpenAiContentPart::Text { text } = &parts[0] {
                            OpenAiContent::Text(text.clone())
                        } else {
                            OpenAiContent::Parts(parts)
                        }
                    } else {
                        OpenAiContent::Parts(parts)
                    };

                    messages.push(OpenAiMessage {
                        role: "user".to_string(),
                        content: Some(content),
                        tool_calls: None,
                        tool_call_id: None,
                    });
                }

                messages.extend(tool_result_messages);
            }
            LlmMessageRole::Assistant => {
                let mut text_content = String::new();
                let mut tool_calls: Vec<OpenAiToolCall> = Vec::new();

                for content in &msg.content {
                    match content {
                        LlmMessageContent::Text(text) => {
                            if !text.is_empty() {
                                if !text_content.is_empty() {
                                    text_content.push('\n');
                                }
                                text_content.push_str(text);
                            }
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

                messages.push(OpenAiMessage {
                    role: "assistant".to_string(),
                    content: if text_content.is_empty() {
                        None
                    } else {
                        Some(OpenAiContent::Text(text_content))
                    },
                    tool_calls: if tool_calls.is_empty() {
                        None
                    } else {
                        Some(tool_calls)
                    },
                    tool_call_id: None,
                });
            }
        }
    }

    let tools: Vec<OpenAiTool> = request
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
        .collect();

    let tool_choice = request.tool_choice.as_ref().map(|tc| match tc {
        LlmToolChoice::Auto => "auto".to_string(),
        LlmToolChoice::Any => "required".to_string(),
        LlmToolChoice::None => "none".to_string(),
    });

    let model_def = get_model_definition(model_id);
    let max_tokens = request
        .max_tokens
        .or(model_def.and_then(|m| m.max_output_tokens));

    Ok(OpenAiRequest {
        model: model_id.to_string(),
        messages,
        max_tokens,
        tools,
        tool_choice,
        stop: request.stop_sequences.clone(),
        temperature: request.temperature,
        stream: true,
        stream_options: Some(StreamOptions {
            include_usage: true,
        }),
    })
}

fn parse_sse_line(line: &str) -> Option<OpenAiStreamResponse> {
    let data = line.strip_prefix("data: ")?;
    if data.trim() == "[DONE]" {
        return None;
    }
    serde_json::from_str(data).ok()
}

impl zed::Extension for CopilotChatProvider {
    fn new() -> Self {
        Self {
            streams: Mutex::new(HashMap::new()),
            next_stream_id: Mutex::new(0),
        }
    }

    fn llm_providers(&self) -> Vec<LlmProviderInfo> {
        vec![LlmProviderInfo {
            id: "copilot_chat".into(),
            name: "Copilot Chat".into(),
            icon: Some("icons/copilot.svg".into()),
        }]
    }

    fn llm_provider_models(&self, _provider_id: &str) -> Result<Vec<LlmModelInfo>, String> {
        Ok(MODELS
            .iter()
            .map(|m| LlmModelInfo {
                id: m.id.to_string(),
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
        llm_get_credential("copilot_chat").is_some()
    }

    fn llm_provider_settings_markdown(&self, _provider_id: &str) -> Option<String> {
        Some(
            r#"# Copilot Chat Setup

Welcome to **Copilot Chat**! This extension provides access to GitHub Copilot's chat models.

## Configuration

Enter your GitHub Copilot token below. You need an active GitHub Copilot subscription.

To get your token:
1. Ensure you have a GitHub Copilot subscription
2. Generate a token from your GitHub Copilot settings

## Available Models

| Model | Context | Output |
|-------|---------|--------|
| GPT-4o | 128K | 16K |
| GPT-4o Mini | 128K | 16K |
| GPT-4.1 | 1M | 32K |
| o1 | 200K | 100K |
| o3-mini | 200K | 100K |
| Claude 3.5 Sonnet | 200K | 8K |
| Claude 3.7 Sonnet | 200K | 8K |
| Gemini 2.0 Flash | 1M | 8K |

## Features

- ✅ Full streaming support
- ✅ Tool/function calling
- ✅ Vision (image inputs)
- ✅ Multiple model providers via Copilot

## Note

This extension requires an active GitHub Copilot subscription.
"#
            .to_string(),
        )
    }

    fn llm_provider_authenticate(&mut self, _provider_id: &str) -> Result<(), String> {
        let provided = llm_request_credential(
            "copilot_chat",
            LlmCredentialType::ApiKey,
            "GitHub Copilot Token",
            "ghu_...",
        )?;
        if provided {
            Ok(())
        } else {
            Err("Authentication cancelled".to_string())
        }
    }

    fn llm_provider_reset_credentials(&mut self, _provider_id: &str) -> Result<(), String> {
        llm_delete_credential("copilot_chat")
    }

    fn llm_stream_completion_start(
        &mut self,
        _provider_id: &str,
        model_id: &str,
        request: &LlmCompletionRequest,
    ) -> Result<String, String> {
        let api_key = llm_get_credential("copilot_chat").ok_or_else(|| {
            "No token configured. Please add your GitHub Copilot token in settings.".to_string()
        })?;

        let openai_request = convert_request(model_id, request)?;

        let body = serde_json::to_vec(&openai_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let http_request = HttpRequest {
            method: HttpMethod::Post,
            url: "https://api.githubcopilot.com/chat/completions".to_string(),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Authorization".to_string(), format!("Bearer {}", api_key)),
                (
                    "Copilot-Integration-Id".to_string(),
                    "vscode-chat".to_string(),
                ),
                ("Editor-Version".to_string(), "Zed/1.0.0".to_string()),
            ],
            body: Some(body),
            redirect_policy: RedirectPolicy::FollowAll,
        };

        let response_stream = http_request
            .fetch_stream()
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        let stream_id = {
            let mut id_counter = self.next_stream_id.lock().unwrap();
            let id = format!("copilot-stream-{}", *id_counter);
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
                let line = state.buffer[..newline_pos].to_string();
                state.buffer = state.buffer[newline_pos + 1..].to_string();

                if line.trim().is_empty() {
                    continue;
                }

                if let Some(response) = parse_sse_line(&line) {
                    if let Some(choice) = response.choices.first() {
                        if let Some(content) = &choice.delta.content {
                            if !content.is_empty() {
                                return Ok(Some(LlmCompletionEvent::Text(content.clone())));
                            }
                        }

                        if let Some(tool_calls) = &choice.delta.tool_calls {
                            for tc in tool_calls {
                                let entry = state
                                    .tool_calls
                                    .entry(tc.index)
                                    .or_insert_with(AccumulatedToolCall::default);

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

                        if let Some(finish_reason) = &choice.finish_reason {
                            if !state.tool_calls.is_empty() && !state.tool_calls_emitted {
                                state.tool_calls_emitted = true;
                                let mut tool_calls: Vec<_> = state.tool_calls.drain().collect();
                                tool_calls.sort_by_key(|(idx, _)| *idx);

                                if let Some((_, tc)) = tool_calls.into_iter().next() {
                                    return Ok(Some(LlmCompletionEvent::ToolUse(LlmToolUse {
                                        id: tc.id,
                                        name: tc.name,
                                        input: tc.arguments,
                                        thought_signature: None,
                                    })));
                                }
                            }

                            let stop_reason = match finish_reason.as_str() {
                                "stop" => LlmStopReason::EndTurn,
                                "length" => LlmStopReason::MaxTokens,
                                "tool_calls" => LlmStopReason::ToolUse,
                                "content_filter" => LlmStopReason::Refusal,
                                _ => LlmStopReason::EndTurn,
                            };
                            return Ok(Some(LlmCompletionEvent::Stop(stop_reason)));
                        }
                    }

                    if let Some(usage) = response.usage {
                        return Ok(Some(LlmCompletionEvent::Usage(LlmTokenUsage {
                            input_tokens: usage.prompt_tokens,
                            output_tokens: usage.completion_tokens,
                            cache_creation_input_tokens: None,
                            cache_read_input_tokens: None,
                        })));
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

zed::register_extension!(CopilotChatProvider);
