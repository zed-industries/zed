use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zed_extension_api::{
    self as zed, http_client::HttpMethod, http_client::HttpRequest, llm_get_env_var,
    LlmCacheConfiguration, LlmCompletionEvent, LlmCompletionRequest, LlmMessageContent,
    LlmMessageRole, LlmModelCapabilities, LlmModelInfo, LlmProviderInfo, LlmStopReason,
    LlmThinkingContent, LlmTokenUsage, LlmToolInputFormat, LlmToolUse,
};

pub const API_URL: &str = "https://generativelanguage.googleapis.com";

fn stream_generate_content(
    model_id: &str,
    request: &LlmCompletionRequest,
    streams: &mut HashMap<String, StreamState>,
    next_stream_id: &mut u64,
) -> Result<String, String> {
    let api_key = get_api_key().ok_or_else(|| "API key not configured".to_string())?;

    let generate_content_request = build_generate_content_request(model_id, request)?;
    validate_generate_content_request(&generate_content_request)?;

    let uri = format!(
        "{}/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
        API_URL, model_id, api_key
    );

    let body = serde_json::to_vec(&generate_content_request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?;

    let http_request = HttpRequest::builder()
        .method(HttpMethod::Post)
        .url(&uri)
        .header("Content-Type", "application/json")
        .body(body)
        .build()?;

    let response_stream = http_request.fetch_stream()?;

    let stream_id = format!("stream-{}", *next_stream_id);
    *next_stream_id += 1;

    streams.insert(
        stream_id.clone(),
        StreamState {
            response_stream,
            buffer: String::new(),
            usage: None,
        },
    );

    Ok(stream_id)
}

fn count_tokens(model_id: &str, request: &LlmCompletionRequest) -> Result<u64, String> {
    let api_key = get_api_key().ok_or_else(|| "API key not configured".to_string())?;

    let generate_content_request = build_generate_content_request(model_id, request)?;
    validate_generate_content_request(&generate_content_request)?;
    let count_request = CountTokensRequest {
        generate_content_request,
    };

    let uri = format!(
        "{}/v1beta/models/{}:countTokens?key={}",
        API_URL, model_id, api_key
    );

    let body = serde_json::to_vec(&count_request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?;

    let http_request = HttpRequest::builder()
        .method(HttpMethod::Post)
        .url(&uri)
        .header("Content-Type", "application/json")
        .body(body)
        .build()?;

    let response = http_request.fetch()?;
    let response_body: CountTokensResponse = serde_json::from_slice(&response.body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(response_body.total_tokens)
}

fn validate_generate_content_request(request: &GenerateContentRequest) -> Result<(), String> {
    if request.model.is_empty() {
        return Err("Model must be specified".to_string());
    }

    if request.contents.is_empty() {
        return Err("Request must contain at least one content item".to_string());
    }

    if let Some(user_content) = request
        .contents
        .iter()
        .find(|content| content.role == Role::User)
    {
        if user_content.parts.is_empty() {
            return Err("User content must contain at least one part".to_string());
        }
    }

    Ok(())
}

// Extension implementation

const PROVIDER_ID: &str = "google-ai";
const PROVIDER_NAME: &str = "Google AI";

struct GoogleAiExtension {
    streams: HashMap<String, StreamState>,
    next_stream_id: u64,
}

struct StreamState {
    response_stream: zed::http_client::HttpResponseStream,
    buffer: String,
    usage: Option<UsageMetadata>,
}

impl zed::Extension for GoogleAiExtension {
    fn new() -> Self {
        Self {
            streams: HashMap::new(),
            next_stream_id: 0,
        }
    }

    fn llm_providers(&self) -> Vec<LlmProviderInfo> {
        vec![LlmProviderInfo {
            id: PROVIDER_ID.to_string(),
            name: PROVIDER_NAME.to_string(),
            icon: Some("icons/google-ai.svg".to_string()),
        }]
    }

    fn llm_provider_models(&self, provider_id: &str) -> Result<Vec<LlmModelInfo>, String> {
        if provider_id != PROVIDER_ID {
            return Err(format!("Unknown provider: {}", provider_id));
        }
        Ok(get_models())
    }

    fn llm_provider_settings_markdown(&self, provider_id: &str) -> Option<String> {
        if provider_id != PROVIDER_ID {
            return None;
        }

        Some(
            r#"## Google AI Setup

To use Google AI models in Zed, you need a Gemini API key.

1. Go to [Google AI Studio](https://aistudio.google.com/apikey)
2. Create or select a project
3. Generate an API key
4. Set the `GEMINI_API_KEY` or `GOOGLE_AI_API_KEY` environment variable

You can set this in your shell profile or use a `.envrc` file with [direnv](https://direnv.net/).
"#
            .to_string(),
        )
    }

    fn llm_provider_is_authenticated(&self, provider_id: &str) -> bool {
        if provider_id != PROVIDER_ID {
            return false;
        }
        get_api_key().is_some()
    }

    fn llm_provider_reset_credentials(&mut self, provider_id: &str) -> Result<(), String> {
        if provider_id != PROVIDER_ID {
            return Err(format!("Unknown provider: {}", provider_id));
        }
        Ok(())
    }

    fn llm_count_tokens(
        &self,
        provider_id: &str,
        model_id: &str,
        request: &LlmCompletionRequest,
    ) -> Result<u64, String> {
        if provider_id != PROVIDER_ID {
            return Err(format!("Unknown provider: {}", provider_id));
        }
        count_tokens(model_id, request)
    }

    fn llm_stream_completion_start(
        &mut self,
        provider_id: &str,
        model_id: &str,
        request: &LlmCompletionRequest,
    ) -> Result<String, String> {
        if provider_id != PROVIDER_ID {
            return Err(format!("Unknown provider: {}", provider_id));
        }
        stream_generate_content(model_id, request, &mut self.streams, &mut self.next_stream_id)
    }

    fn llm_stream_completion_next(
        &mut self,
        stream_id: &str,
    ) -> Result<Option<LlmCompletionEvent>, String> {
        stream_generate_content_next(stream_id, &mut self.streams)
    }

    fn llm_stream_completion_close(&mut self, stream_id: &str) {
        self.streams.remove(stream_id);
    }

    fn llm_cache_configuration(
        &self,
        provider_id: &str,
        _model_id: &str,
    ) -> Option<LlmCacheConfiguration> {
        if provider_id != PROVIDER_ID {
            return None;
        }

        Some(LlmCacheConfiguration {
            max_cache_anchors: 1,
            should_cache_tool_definitions: false,
            min_total_token_count: 32768,
        })
    }
}

zed::register_extension!(GoogleAiExtension);

// Helper functions

fn get_api_key() -> Option<String> {
    llm_get_env_var("GEMINI_API_KEY").or_else(|| llm_get_env_var("GOOGLE_AI_API_KEY"))
}

fn get_models() -> Vec<LlmModelInfo> {
    vec![
        LlmModelInfo {
            id: "gemini-2.5-flash-lite".to_string(),
            name: "Gemini 2.5 Flash-Lite".to_string(),
            max_token_count: 1_048_576,
            max_output_tokens: Some(65_536),
            capabilities: LlmModelCapabilities {
                supports_images: true,
                supports_tools: true,
                supports_tool_choice_auto: true,
                supports_tool_choice_any: true,
                supports_tool_choice_none: true,
                supports_thinking: true,
                tool_input_format: LlmToolInputFormat::JsonSchema,
            },
            is_default: false,
            is_default_fast: true,
        },
        LlmModelInfo {
            id: "gemini-2.5-flash".to_string(),
            name: "Gemini 2.5 Flash".to_string(),
            max_token_count: 1_048_576,
            max_output_tokens: Some(65_536),
            capabilities: LlmModelCapabilities {
                supports_images: true,
                supports_tools: true,
                supports_tool_choice_auto: true,
                supports_tool_choice_any: true,
                supports_tool_choice_none: true,
                supports_thinking: true,
                tool_input_format: LlmToolInputFormat::JsonSchema,
            },
            is_default: true,
            is_default_fast: false,
        },
        LlmModelInfo {
            id: "gemini-2.5-pro".to_string(),
            name: "Gemini 2.5 Pro".to_string(),
            max_token_count: 1_048_576,
            max_output_tokens: Some(65_536),
            capabilities: LlmModelCapabilities {
                supports_images: true,
                supports_tools: true,
                supports_tool_choice_auto: true,
                supports_tool_choice_any: true,
                supports_tool_choice_none: true,
                supports_thinking: true,
                tool_input_format: LlmToolInputFormat::JsonSchema,
            },
            is_default: false,
            is_default_fast: false,
        },
        LlmModelInfo {
            id: "gemini-3-pro-preview".to_string(),
            name: "Gemini 3 Pro".to_string(),
            max_token_count: 1_048_576,
            max_output_tokens: Some(65_536),
            capabilities: LlmModelCapabilities {
                supports_images: true,
                supports_tools: true,
                supports_tool_choice_auto: true,
                supports_tool_choice_any: true,
                supports_tool_choice_none: true,
                supports_thinking: true,
                tool_input_format: LlmToolInputFormat::JsonSchema,
            },
            is_default: false,
            is_default_fast: false,
        },
        LlmModelInfo {
            id: "gemini-3-flash-preview".to_string(),
            name: "Gemini 3 Flash".to_string(),
            max_token_count: 1_048_576,
            max_output_tokens: Some(65_536),
            capabilities: LlmModelCapabilities {
                supports_images: true,
                supports_tools: true,
                supports_tool_choice_auto: true,
                supports_tool_choice_any: true,
                supports_tool_choice_none: true,
                supports_thinking: true,
                tool_input_format: LlmToolInputFormat::JsonSchema,
            },
            is_default: false,
            is_default_fast: false,
        },
    ]
}

fn stream_generate_content_next(
    stream_id: &str,
    streams: &mut HashMap<String, StreamState>,
) -> Result<Option<LlmCompletionEvent>, String> {
    let state = streams
        .get_mut(stream_id)
        .ok_or_else(|| format!("Unknown stream: {}", stream_id))?;

    loop {
        if let Some(newline_pos) = state.buffer.find('\n') {
            let line = state.buffer[..newline_pos].to_string();
            state.buffer = state.buffer[newline_pos + 1..].to_string();

            if let Some(data) = line.strip_prefix("data: ") {
                if data.trim().is_empty() {
                    continue;
                }

                let response: GenerateContentResponse = serde_json::from_str(data)
                    .map_err(|e| format!("Failed to parse SSE data: {} - {}", e, data))?;

                if let Some(usage) = response.usage_metadata {
                    state.usage = Some(usage);
                }

                if let Some(candidates) = response.candidates {
                    for candidate in candidates {
                        for part in candidate.content.parts {
                            match part {
                                Part::TextPart(text_part) => {
                                    return Ok(Some(LlmCompletionEvent::Text(text_part.text)));
                                }
                                Part::ThoughtPart(thought_part) => {
                                    return Ok(Some(LlmCompletionEvent::Thinking(
                                        LlmThinkingContent {
                                            text: String::new(),
                                            signature: Some(thought_part.thought_signature),
                                        },
                                    )));
                                }
                                Part::FunctionCallPart(fc_part) => {
                                    return Ok(Some(LlmCompletionEvent::ToolUse(LlmToolUse {
                                        id: fc_part.function_call.name.clone(),
                                        name: fc_part.function_call.name,
                                        input: serde_json::to_string(&fc_part.function_call.args)
                                            .unwrap_or_default(),
                                        is_input_complete: true,
                                        thought_signature: fc_part.thought_signature,
                                    })));
                                }
                                _ => {}
                            }
                        }

                        if let Some(finish_reason) = candidate.finish_reason {
                            let stop_reason = match finish_reason.as_str() {
                                "STOP" => LlmStopReason::EndTurn,
                                "MAX_TOKENS" => LlmStopReason::MaxTokens,
                                "TOOL_USE" | "FUNCTION_CALL" => LlmStopReason::ToolUse,
                                "SAFETY" | "RECITATION" | "OTHER" => LlmStopReason::Refusal,
                                _ => LlmStopReason::EndTurn,
                            };

                            if let Some(usage) = state.usage.take() {
                                return Ok(Some(LlmCompletionEvent::Usage(LlmTokenUsage {
                                    input_tokens: usage.prompt_token_count.unwrap_or(0),
                                    output_tokens: usage.candidates_token_count.unwrap_or(0),
                                    cache_creation_input_tokens: None,
                                    cache_read_input_tokens: usage.cached_content_token_count,
                                })));
                            }

                            return Ok(Some(LlmCompletionEvent::Stop(stop_reason)));
                        }
                    }
                }
            }

            continue;
        }

        match state.response_stream.next_chunk() {
            Ok(Some(chunk)) => {
                let chunk_str = String::from_utf8_lossy(&chunk);
                state.buffer.push_str(&chunk_str);
            }
            Ok(None) => {
                streams.remove(stream_id);
                return Ok(None);
            }
            Err(e) => {
                streams.remove(stream_id);
                return Err(e);
            }
        }
    }
}

fn build_generate_content_request(
    model_id: &str,
    request: &LlmCompletionRequest,
) -> Result<GenerateContentRequest, String> {
    let mut contents: Vec<Content> = Vec::new();
    let mut system_instruction: Option<SystemInstruction> = None;

    for message in &request.messages {
        match message.role {
            LlmMessageRole::System => {
                let parts = convert_content_to_parts(&message.content)?;
                system_instruction = Some(SystemInstruction { parts });
            }
            LlmMessageRole::User | LlmMessageRole::Assistant => {
                let role = match message.role {
                    LlmMessageRole::User => Role::User,
                    LlmMessageRole::Assistant => Role::Model,
                    _ => continue,
                };
                let parts = convert_content_to_parts(&message.content)?;
                contents.push(Content { parts, role });
            }
        }
    }

    let tools = if !request.tools.is_empty() {
        Some(vec![Tool {
            function_declarations: request
                .tools
                .iter()
                .map(|t| FunctionDeclaration {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: serde_json::from_str(&t.input_schema).unwrap_or_default(),
                })
                .collect(),
        }])
    } else {
        None
    };

    let tool_config = request.tool_choice.as_ref().map(|choice| {
        let mode = match choice {
            zed::LlmToolChoice::Auto => FunctionCallingMode::Auto,
            zed::LlmToolChoice::Any => FunctionCallingMode::Any,
            zed::LlmToolChoice::None => FunctionCallingMode::None,
        };
        ToolConfig {
            function_calling_config: FunctionCallingConfig {
                mode,
                allowed_function_names: None,
            },
        }
    });

    let generation_config = Some(GenerationConfig {
        candidate_count: Some(1),
        stop_sequences: if request.stop_sequences.is_empty() {
            None
        } else {
            Some(request.stop_sequences.clone())
        },
        max_output_tokens: request.max_tokens.map(|t| t as usize),
        temperature: request.temperature.map(|t| t as f64),
        top_p: None,
        top_k: None,
        thinking_config: if request.thinking_allowed {
            Some(ThinkingConfig {
                thinking_budget: 8192,
            })
        } else {
            None
        },
    });

    Ok(GenerateContentRequest {
        model: ModelName {
            model_id: model_id.to_string(),
        },
        contents,
        system_instruction,
        generation_config,
        safety_settings: None,
        tools,
        tool_config,
    })
}

fn convert_content_to_parts(content: &[LlmMessageContent]) -> Result<Vec<Part>, String> {
    let mut parts = Vec::new();

    for item in content {
        match item {
            LlmMessageContent::Text(text) => {
                parts.push(Part::TextPart(TextPart { text: text.clone() }));
            }
            LlmMessageContent::Image(image) => {
                parts.push(Part::InlineDataPart(InlineDataPart {
                    inline_data: GenerativeContentBlob {
                        mime_type: "image/png".to_string(),
                        data: image.source.clone(),
                    },
                }));
            }
            LlmMessageContent::ToolUse(tool_use) => {
                parts.push(Part::FunctionCallPart(FunctionCallPart {
                    function_call: FunctionCall {
                        name: tool_use.name.clone(),
                        args: serde_json::from_str(&tool_use.input).unwrap_or_default(),
                    },
                    thought_signature: tool_use.thought_signature.clone(),
                }));
            }
            LlmMessageContent::ToolResult(tool_result) => {
                let response_value = match &tool_result.content {
                    zed::LlmToolResultContent::Text(text) => {
                        serde_json::json!({ "result": text })
                    }
                    zed::LlmToolResultContent::Image(_) => {
                        serde_json::json!({ "error": "Image results not supported" })
                    }
                };
                parts.push(Part::FunctionResponsePart(FunctionResponsePart {
                    function_response: FunctionResponse {
                        name: tool_result.tool_name.clone(),
                        response: response_value,
                    },
                }));
            }
            LlmMessageContent::Thinking(thinking) => {
                if let Some(signature) = &thinking.signature {
                    parts.push(Part::ThoughtPart(ThoughtPart {
                        thought: true,
                        thought_signature: signature.clone(),
                    }));
                }
            }
            LlmMessageContent::RedactedThinking(_) => {}
        }
    }

    Ok(parts)
}

// Data structures for Google AI API

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    #[serde(default, skip_serializing_if = "ModelName::is_empty")]
    pub model: ModelName,
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<SystemInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<GenerateContentCandidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_feedback: Option<PromptFeedback>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentCandidate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_metadata: Option<CitationMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(default)]
    pub parts: Vec<Part>,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInstruction {
    pub parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    User,
    Model,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Part {
    TextPart(TextPart),
    InlineDataPart(InlineDataPart),
    FunctionCallPart(FunctionCallPart),
    FunctionResponsePart(FunctionResponsePart),
    ThoughtPart(ThoughtPart),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPart {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineDataPart {
    pub inline_data: GenerativeContentBlob,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerativeContentBlob {
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallPart {
    pub function_call: FunctionCall,
    /// Thought signature returned by the model for function calls.
    /// Only present on the first function call in parallel call scenarios.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResponsePart {
    pub function_response: FunctionResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThoughtPart {
    pub thought: bool,
    pub thought_signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    pub citation_sources: Vec<CitationSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_reason_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_prompt_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thoughts_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_token_count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {
    pub thinking_budget: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_config: Option<ThinkingConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "HARM_CATEGORY_DEROGATORY")]
    Derogatory,
    #[serde(rename = "HARM_CATEGORY_TOXICITY")]
    Toxicity,
    #[serde(rename = "HARM_CATEGORY_VIOLENCE")]
    Violence,
    #[serde(rename = "HARM_CATEGORY_SEXUAL")]
    Sexual,
    #[serde(rename = "HARM_CATEGORY_MEDICAL")]
    Medical,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS")]
    Dangerous,
    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
    Harassment,
    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
    HateSpeech,
    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    SexuallyExplicit,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    DangerousContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    Unspecified,
    BlockLowAndAbove,
    BlockMediumAndAbove,
    BlockOnlyHigh,
    BlockNone,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    Unspecified,
    Negligible,
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensRequest {
    pub generate_content_request: GenerateContentRequest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensResponse {
    pub total_tokens: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionResponse {
    pub name: String,
    pub response: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub function_declarations: Vec<FunctionDeclaration>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolConfig {
    pub function_calling_config: FunctionCallingConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallingConfig {
    pub mode: FunctionCallingMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_function_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionCallingMode {
    Auto,
    Any,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Default)]
pub struct ModelName {
    pub model_id: String,
}

impl ModelName {
    pub fn is_empty(&self) -> bool {
        self.model_id.is_empty()
    }
}

const MODEL_NAME_PREFIX: &str = "models/";

impl Serialize for ModelName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{MODEL_NAME_PREFIX}{}", &self.model_id))
    }
}

impl<'de> Deserialize<'de> for ModelName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        if let Some(id) = string.strip_prefix(MODEL_NAME_PREFIX) {
            Ok(Self {
                model_id: id.to_string(),
            })
        } else {
            Err(serde::de::Error::custom(format!(
                "Expected model name to begin with {}, got: {}",
                MODEL_NAME_PREFIX, string
            )))
        }
    }
}
