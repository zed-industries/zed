use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use zed_extension_api::http_client::{HttpMethod, HttpRequest, HttpResponseStream, RedirectPolicy};
use zed_extension_api::{self as zed, *};

static TOOL_CALL_COUNTER: AtomicU64 = AtomicU64::new(0);

struct GoogleAiProvider {
    streams: Mutex<HashMap<String, StreamState>>,
    next_stream_id: Mutex<u64>,
}

struct StreamState {
    response_stream: Option<HttpResponseStream>,
    buffer: String,
    started: bool,
    stop_reason: Option<LlmStopReason>,
    wants_tool_use: bool,
}

struct ModelDefinition {
    real_id: &'static str,
    display_name: &'static str,
    max_tokens: u64,
    max_output_tokens: Option<u64>,
    supports_images: bool,
    supports_thinking: bool,
    is_default: bool,
    is_default_fast: bool,
}

const MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        real_id: "gemini-2.5-flash-lite",
        display_name: "Gemini 2.5 Flash-Lite",
        max_tokens: 1_048_576,
        max_output_tokens: Some(65_536),
        supports_images: true,
        supports_thinking: true,
        is_default: false,
        is_default_fast: true,
    },
    ModelDefinition {
        real_id: "gemini-2.5-flash",
        display_name: "Gemini 2.5 Flash",
        max_tokens: 1_048_576,
        max_output_tokens: Some(65_536),
        supports_images: true,
        supports_thinking: true,
        is_default: true,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "gemini-2.5-pro",
        display_name: "Gemini 2.5 Pro",
        max_tokens: 1_048_576,
        max_output_tokens: Some(65_536),
        supports_images: true,
        supports_thinking: true,
        is_default: false,
        is_default_fast: false,
    },
    ModelDefinition {
        real_id: "gemini-3-pro-preview",
        display_name: "Gemini 3 Pro",
        max_tokens: 1_048_576,
        max_output_tokens: Some(65_536),
        supports_images: true,
        supports_thinking: true,
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

fn get_model_supports_thinking(display_name: &str) -> bool {
    MODELS
        .iter()
        .find(|m| m.display_name == display_name)
        .map(|m| m.supports_thinking)
        .unwrap_or(false)
}

/// Adapts a JSON schema to be compatible with Google's API subset.
/// Google only supports a specific subset of JSON Schema fields.
/// See: https://ai.google.dev/api/caching#Schema
fn adapt_schema_for_google(json: &mut serde_json::Value) {
    adapt_schema_for_google_impl(json, true);
}

fn adapt_schema_for_google_impl(json: &mut serde_json::Value, is_schema: bool) {
    if let serde_json::Value::Object(obj) = json {
        // Google's Schema only supports these fields:
        // type, format, title, description, nullable, enum, maxItems, minItems,
        // properties, required, minProperties, maxProperties, minLength, maxLength,
        // pattern, example, anyOf, propertyOrdering, default, items, minimum, maximum
        const ALLOWED_KEYS: &[&str] = &[
            "type",
            "format",
            "title",
            "description",
            "nullable",
            "enum",
            "maxItems",
            "minItems",
            "properties",
            "required",
            "minProperties",
            "maxProperties",
            "minLength",
            "maxLength",
            "pattern",
            "example",
            "anyOf",
            "propertyOrdering",
            "default",
            "items",
            "minimum",
            "maximum",
        ];

        // Convert oneOf to anyOf before filtering keys
        if let Some(one_of) = obj.remove("oneOf") {
            obj.insert("anyOf".to_string(), one_of);
        }

        // If type is an array (e.g., ["string", "null"]), take just the first type
        if let Some(type_field) = obj.get_mut("type") {
            if let serde_json::Value::Array(types) = type_field {
                if let Some(first_type) = types.first().cloned() {
                    *type_field = first_type;
                }
            }
        }

        // Only filter keys if this is a schema object, not a properties map
        if is_schema {
            obj.retain(|key, _| ALLOWED_KEYS.contains(&key.as_str()));
        }

        // Recursively process nested values
        // "properties" contains a map of property names -> schemas
        // "items" and "anyOf" contain schemas directly
        for (key, value) in obj.iter_mut() {
            if key == "properties" {
                // properties is a map of property_name -> schema
                if let serde_json::Value::Object(props) = value {
                    for (_, prop_schema) in props.iter_mut() {
                        adapt_schema_for_google_impl(prop_schema, true);
                    }
                }
            } else if key == "items" {
                // items is a schema
                adapt_schema_for_google_impl(value, true);
            } else if key == "anyOf" {
                // anyOf is an array of schemas
                if let serde_json::Value::Array(arr) = value {
                    for item in arr.iter_mut() {
                        adapt_schema_for_google_impl(item, true);
                    }
                }
            }
        }
    } else if let serde_json::Value::Array(arr) = json {
        for item in arr.iter_mut() {
            adapt_schema_for_google_impl(item, true);
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GoogleRequest {
    contents: Vec<GoogleContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GoogleSystemInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GoogleGenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<GoogleTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_config: Option<GoogleToolConfig>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GoogleSystemInstruction {
    parts: Vec<GooglePart>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GoogleContent {
    parts: Vec<GooglePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum GooglePart {
    Text(GoogleTextPart),
    InlineData(GoogleInlineDataPart),
    FunctionCall(GoogleFunctionCallPart),
    FunctionResponse(GoogleFunctionResponsePart),
    Thought(GoogleThoughtPart),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GoogleTextPart {
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GoogleInlineDataPart {
    inline_data: GoogleBlob,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GoogleBlob {
    mime_type: String,
    data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GoogleFunctionCallPart {
    function_call: GoogleFunctionCall,
    #[serde(skip_serializing_if = "Option::is_none")]
    thought_signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GoogleFunctionCall {
    name: String,
    args: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GoogleFunctionResponsePart {
    function_response: GoogleFunctionResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GoogleFunctionResponse {
    name: String,
    response: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GoogleThoughtPart {
    thought: bool,
    thought_signature: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GoogleGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    candidate_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking_config: Option<GoogleThinkingConfig>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GoogleThinkingConfig {
    thinking_budget: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GoogleTool {
    function_declarations: Vec<GoogleFunctionDeclaration>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GoogleFunctionDeclaration {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GoogleToolConfig {
    function_calling_config: GoogleFunctionCallingConfig,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GoogleFunctionCallingConfig {
    mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_function_names: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GoogleStreamResponse {
    #[serde(default)]
    candidates: Vec<GoogleCandidate>,
    #[serde(default)]
    usage_metadata: Option<GoogleUsageMetadata>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GoogleCandidate {
    #[serde(default)]
    content: Option<GoogleContent>,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GoogleUsageMetadata {
    #[serde(default)]
    prompt_token_count: u64,
    #[serde(default)]
    candidates_token_count: u64,
}

fn convert_request(
    model_id: &str,
    request: &LlmCompletionRequest,
) -> Result<(GoogleRequest, String), String> {
    let real_model_id =
        get_real_model_id(model_id).ok_or_else(|| format!("Unknown model: {}", model_id))?;

    let supports_thinking = get_model_supports_thinking(model_id);

    let mut contents: Vec<GoogleContent> = Vec::new();
    let mut system_parts: Vec<GooglePart> = Vec::new();

    for msg in &request.messages {
        match msg.role {
            LlmMessageRole::System => {
                for content in &msg.content {
                    if let LlmMessageContent::Text(text) = content {
                        if !text.is_empty() {
                            system_parts
                                .push(GooglePart::Text(GoogleTextPart { text: text.clone() }));
                        }
                    }
                }
            }
            LlmMessageRole::User => {
                let mut parts: Vec<GooglePart> = Vec::new();

                for content in &msg.content {
                    match content {
                        LlmMessageContent::Text(text) => {
                            if !text.is_empty() {
                                parts.push(GooglePart::Text(GoogleTextPart { text: text.clone() }));
                            }
                        }
                        LlmMessageContent::Image(img) => {
                            parts.push(GooglePart::InlineData(GoogleInlineDataPart {
                                inline_data: GoogleBlob {
                                    mime_type: "image/png".to_string(),
                                    data: img.source.clone(),
                                },
                            }));
                        }
                        LlmMessageContent::ToolResult(result) => {
                            let response_value = match &result.content {
                                LlmToolResultContent::Text(t) => {
                                    serde_json::json!({ "output": t })
                                }
                                LlmToolResultContent::Image(_) => {
                                    serde_json::json!({ "output": "Tool responded with an image" })
                                }
                            };
                            parts.push(GooglePart::FunctionResponse(GoogleFunctionResponsePart {
                                function_response: GoogleFunctionResponse {
                                    name: result.tool_name.clone(),
                                    response: response_value,
                                },
                            }));
                        }
                        _ => {}
                    }
                }

                if !parts.is_empty() {
                    contents.push(GoogleContent {
                        parts,
                        role: Some("user".to_string()),
                    });
                }
            }
            LlmMessageRole::Assistant => {
                let mut parts: Vec<GooglePart> = Vec::new();

                for content in &msg.content {
                    match content {
                        LlmMessageContent::Text(text) => {
                            if !text.is_empty() {
                                parts.push(GooglePart::Text(GoogleTextPart { text: text.clone() }));
                            }
                        }
                        LlmMessageContent::ToolUse(tool_use) => {
                            let thought_signature =
                                tool_use.thought_signature.clone().filter(|s| !s.is_empty());

                            let args: serde_json::Value =
                                serde_json::from_str(&tool_use.input).unwrap_or_default();

                            parts.push(GooglePart::FunctionCall(GoogleFunctionCallPart {
                                function_call: GoogleFunctionCall {
                                    name: tool_use.name.clone(),
                                    args,
                                },
                                thought_signature,
                            }));
                        }
                        LlmMessageContent::Thinking(thinking) => {
                            if let Some(ref signature) = thinking.signature {
                                if !signature.is_empty() {
                                    parts.push(GooglePart::Thought(GoogleThoughtPart {
                                        thought: true,
                                        thought_signature: signature.clone(),
                                    }));
                                }
                            }
                        }
                        _ => {}
                    }
                }

                if !parts.is_empty() {
                    contents.push(GoogleContent {
                        parts,
                        role: Some("model".to_string()),
                    });
                }
            }
        }
    }

    let system_instruction = if system_parts.is_empty() {
        None
    } else {
        Some(GoogleSystemInstruction {
            parts: system_parts,
        })
    };

    let tools: Option<Vec<GoogleTool>> = if request.tools.is_empty() {
        None
    } else {
        let declarations: Vec<GoogleFunctionDeclaration> = request
            .tools
            .iter()
            .map(|t| {
                let mut parameters: serde_json::Value = serde_json::from_str(&t.input_schema)
                    .unwrap_or(serde_json::Value::Object(Default::default()));
                adapt_schema_for_google(&mut parameters);
                GoogleFunctionDeclaration {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters,
                }
            })
            .collect();
        Some(vec![GoogleTool {
            function_declarations: declarations,
        }])
    };

    let tool_config = request.tool_choice.as_ref().map(|tc| {
        let mode = match tc {
            LlmToolChoice::Auto => "AUTO",
            LlmToolChoice::Any => "ANY",
            LlmToolChoice::None => "NONE",
        };
        GoogleToolConfig {
            function_calling_config: GoogleFunctionCallingConfig {
                mode: mode.to_string(),
                allowed_function_names: None,
            },
        }
    });

    let thinking_config = if supports_thinking && request.thinking_allowed {
        Some(GoogleThinkingConfig {
            thinking_budget: 8192,
        })
    } else {
        None
    };

    let generation_config = Some(GoogleGenerationConfig {
        candidate_count: Some(1),
        stop_sequences: if request.stop_sequences.is_empty() {
            None
        } else {
            Some(request.stop_sequences.clone())
        },
        max_output_tokens: None,
        temperature: request.temperature.map(|t| t as f64).or(Some(1.0)),
        thinking_config,
    });

    Ok((
        GoogleRequest {
            contents,
            system_instruction,
            generation_config,
            tools,
            tool_config,
        },
        real_model_id.to_string(),
    ))
}

fn parse_stream_line(line: &str) -> Option<GoogleStreamResponse> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed == "[" || trimmed == "]" || trimmed == "," {
        return None;
    }

    let json_str = trimmed.strip_prefix("data: ").unwrap_or(trimmed);
    let json_str = json_str.trim_start_matches(',').trim();

    if json_str.is_empty() {
        return None;
    }

    serde_json::from_str(json_str).ok()
}

impl zed::Extension for GoogleAiProvider {
    fn new() -> Self {
        Self {
            streams: Mutex::new(HashMap::new()),
            next_stream_id: Mutex::new(0),
        }
    }

    fn llm_providers(&self) -> Vec<LlmProviderInfo> {
        vec![LlmProviderInfo {
            id: "google-ai".into(),
            name: "Google AI".into(),
            icon: Some("google-ai".into()),
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
                    supports_thinking: m.supports_thinking,
                    tool_input_format: LlmToolInputFormat::JsonSchema,
                },
                is_default: m.is_default,
                is_default_fast: m.is_default_fast,
            })
            .collect())
    }

    fn llm_provider_is_authenticated(&self, _provider_id: &str) -> bool {
        llm_get_credential("google-ai").is_some()
    }

    fn llm_provider_settings_markdown(&self, _provider_id: &str) -> Option<String> {
        Some(
            r#"# Google AI Setup

Welcome to **Google AI**! This extension provides access to Google Gemini models.

## Configuration

Enter your Google AI API key below. You can get your API key at [aistudio.google.com/apikey](https://aistudio.google.com/apikey).

## Available Models

| Display Name | Real Model | Context | Output |
|--------------|------------|---------|--------|
| Gemini 2.5 Flash-Lite | gemini-2.5-flash-lite | 1M | 65K |
| Gemini 2.5 Flash | gemini-2.5-flash | 1M | 65K |
| Gemini 2.5 Pro | gemini-2.5-pro | 1M | 65K |
| Gemini 3 Pro | gemini-3-pro-preview | 1M | 65K |

## Features

- ✅ Full streaming support
- ✅ Tool/function calling with thought signatures
- ✅ Vision (image inputs)
- ✅ Extended thinking support
- ✅ All Gemini models

## Pricing

Uses your Google AI API credits. See [Google AI pricing](https://ai.google.dev/pricing) for details.
"#
            .to_string(),
        )
    }

    fn llm_provider_authenticate(&mut self, _provider_id: &str) -> Result<(), String> {
        let provided = llm_request_credential(
            "google-ai",
            LlmCredentialType::ApiKey,
            "Google AI API Key",
            "AIza...",
        )?;
        if provided {
            Ok(())
        } else {
            Err("Authentication cancelled".to_string())
        }
    }

    fn llm_provider_reset_credentials(&mut self, _provider_id: &str) -> Result<(), String> {
        llm_delete_credential("google-ai")
    }

    fn llm_stream_completion_start(
        &mut self,
        _provider_id: &str,
        model_id: &str,
        request: &LlmCompletionRequest,
    ) -> Result<String, String> {
        let api_key = llm_get_credential("google-ai").ok_or_else(|| {
            "No API key configured. Please add your Google AI API key in settings.".to_string()
        })?;

        let (google_request, real_model_id) = convert_request(model_id, request)?;

        let body = serde_json::to_vec(&google_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            real_model_id, api_key
        );

        let http_request = HttpRequest {
            method: HttpMethod::Post,
            url,
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: Some(body),
            redirect_policy: RedirectPolicy::FollowAll,
        };

        let response_stream = http_request
            .fetch_stream()
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        let stream_id = {
            let mut id_counter = self.next_stream_id.lock().unwrap();
            let id = format!("google-ai-stream-{}", *id_counter);
            *id_counter += 1;
            id
        };

        self.streams.lock().unwrap().insert(
            stream_id.clone(),
            StreamState {
                response_stream: Some(response_stream),
                buffer: String::new(),
                started: false,
                stop_reason: None,
                wants_tool_use: false,
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

                if let Some(response) = parse_stream_line(&line) {
                    for candidate in response.candidates {
                        if let Some(finish_reason) = &candidate.finish_reason {
                            state.stop_reason = Some(match finish_reason.as_str() {
                                "STOP" => {
                                    if state.wants_tool_use {
                                        LlmStopReason::ToolUse
                                    } else {
                                        LlmStopReason::EndTurn
                                    }
                                }
                                "MAX_TOKENS" => LlmStopReason::MaxTokens,
                                "SAFETY" => LlmStopReason::Refusal,
                                _ => LlmStopReason::EndTurn,
                            });
                        }

                        if let Some(content) = candidate.content {
                            for part in content.parts {
                                match part {
                                    GooglePart::Text(text_part) => {
                                        if !text_part.text.is_empty() {
                                            return Ok(Some(LlmCompletionEvent::Text(
                                                text_part.text,
                                            )));
                                        }
                                    }
                                    GooglePart::FunctionCall(fc_part) => {
                                        state.wants_tool_use = true;
                                        let next_tool_id =
                                            TOOL_CALL_COUNTER.fetch_add(1, Ordering::SeqCst);
                                        let id = format!(
                                            "{}-{}",
                                            fc_part.function_call.name, next_tool_id
                                        );

                                        let thought_signature =
                                            fc_part.thought_signature.filter(|s| !s.is_empty());

                                        return Ok(Some(LlmCompletionEvent::ToolUse(LlmToolUse {
                                            id,
                                            name: fc_part.function_call.name,
                                            input: fc_part.function_call.args.to_string(),
                                            thought_signature,
                                        })));
                                    }
                                    GooglePart::Thought(thought_part) => {
                                        return Ok(Some(LlmCompletionEvent::Thinking(
                                            LlmThinkingContent {
                                                text: "(Encrypted thought)".to_string(),
                                                signature: Some(thought_part.thought_signature),
                                            },
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    if let Some(usage) = response.usage_metadata {
                        return Ok(Some(LlmCompletionEvent::Usage(LlmTokenUsage {
                            input_tokens: usage.prompt_token_count,
                            output_tokens: usage.candidates_token_count,
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
                    // Stream ended - check if we have a stop reason
                    if let Some(stop_reason) = state.stop_reason.take() {
                        return Ok(Some(LlmCompletionEvent::Stop(stop_reason)));
                    }

                    // No stop reason - this is unexpected. Check if buffer contains error info
                    let mut error_msg = String::from("Stream ended unexpectedly.");

                    // Try to parse remaining buffer as potential error response
                    if !state.buffer.is_empty() {
                        error_msg.push_str(&format!(
                            "\nRemaining buffer: {}",
                            &state.buffer[..state.buffer.len().min(1000)]
                        ));
                    }

                    return Err(error_msg);
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

zed::register_extension!(GoogleAiProvider);
