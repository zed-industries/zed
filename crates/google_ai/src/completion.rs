use anyhow::Result;
use futures::{Stream, StreamExt};
use language_model_core::{
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolUse, LanguageModelToolUseId, MessageContent, Role,
    StopReason, TokenUsage,
};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{self, AtomicU64};

use crate::{
    Content, FunctionCallingConfig, FunctionCallingMode, FunctionDeclaration,
    GenerateContentResponse, GenerationConfig, GenerativeContentBlob, GoogleModelMode,
    InlineDataPart, ModelName, Part, SystemInstruction, TextPart, ThinkingConfig, ThinkingLevel,
    ToolConfig, UsageMetadata,
};

pub fn into_google(
    mut request: LanguageModelRequest,
    model_id: String,
    mode: GoogleModelMode,
) -> crate::GenerateContentRequest {
    fn map_content(content: Vec<MessageContent>) -> Vec<Part> {
        content
            .into_iter()
            .flat_map(|content| match content {
                MessageContent::Text(text) => {
                    if !text.is_empty() {
                        vec![Part::TextPart(TextPart {
                            text,
                            thought: false,
                            thought_signature: None,
                        })]
                    } else {
                        vec![]
                    }
                }
                MessageContent::Thinking {
                    text,
                    signature: Some(signature),
                } => {
                    if !signature.is_empty() {
                        vec![Part::TextPart(TextPart {
                            text,
                            thought: true,
                            thought_signature: Some(signature),
                        })]
                    } else {
                        vec![]
                    }
                }
                MessageContent::Thinking { .. } => {
                    vec![]
                }
                MessageContent::RedactedThinking(_) | MessageContent::Compaction(_) => vec![],
                MessageContent::Image(image) => {
                    vec![Part::InlineDataPart(InlineDataPart {
                        inline_data: GenerativeContentBlob {
                            mime_type: "image/png".to_string(),
                            data: image.source.to_string(),
                        },
                    })]
                }
                MessageContent::ToolUse(tool_use) => {
                    // Normalize empty string signatures to None
                    let thought_signature = tool_use.thought_signature.filter(|s| !s.is_empty());

                    vec![Part::FunctionCallPart(crate::FunctionCallPart {
                        function_call: crate::FunctionCall {
                            name: tool_use.name.to_string(),
                            args: tool_use.input,
                            id: Some(tool_use.id.to_string()),
                        },
                        thought_signature,
                    })]
                }
                MessageContent::ToolResult(tool_result) => {
                    let mut text_output = String::new();
                    let mut images: Vec<InlineDataPart> = Vec::new();
                    for part in tool_result.content {
                        match part {
                            language_model_core::LanguageModelToolResultContent::Text(text) => {
                                text_output.push_str(&text);
                            }
                            language_model_core::LanguageModelToolResultContent::Image(image) => {
                                images.push(InlineDataPart {
                                    inline_data: GenerativeContentBlob {
                                        mime_type: "image/png".to_string(),
                                        data: image.source.to_string(),
                                    },
                                });
                            }
                        }
                    }
                    let output = if text_output.is_empty() && !images.is_empty() {
                        "Tool responded with an image".to_string()
                    } else {
                        text_output
                    };
                    let mut parts = vec![Part::FunctionResponsePart(crate::FunctionResponsePart {
                        function_response: crate::FunctionResponse {
                            name: tool_result.tool_name.to_string(),
                            // The API expects a valid JSON object
                            response: serde_json::json!({
                                "output": output
                            }),
                            id: Some(tool_result.tool_use_id.to_string()),
                        },
                    })];
                    parts.extend(images.into_iter().map(Part::InlineDataPart));
                    parts
                }
            })
            .collect()
    }

    let thinking_config = thinking_config_for_request(&request, &model_id, mode);

    let system_instructions = if request
        .messages
        .first()
        .is_some_and(|msg| matches!(msg.role, Role::System))
    {
        let message = request.messages.remove(0);
        Some(SystemInstruction {
            parts: map_content(message.content),
        })
    } else {
        None
    };

    crate::GenerateContentRequest {
        model: ModelName { model_id },
        system_instruction: system_instructions,
        contents: request
            .messages
            .into_iter()
            .filter_map(|message| {
                let parts = map_content(message.content);
                if parts.is_empty() {
                    None
                } else {
                    Some(Content {
                        parts,
                        role: match message.role {
                            Role::User => crate::Role::User,
                            Role::Assistant => crate::Role::Model,
                            Role::System => crate::Role::User, // Google AI doesn't have a system role
                        },
                    })
                }
            })
            .collect(),
        generation_config: Some(GenerationConfig {
            candidate_count: Some(1),
            stop_sequences: Some(request.stop),
            max_output_tokens: None,
            temperature: request.temperature.map(|t| t as f64),
            thinking_config,
            top_p: None,
            top_k: None,
        }),
        safety_settings: None,
        tools: (!request.tools.is_empty()).then(|| {
            vec![crate::Tool {
                function_declarations: request
                    .tools
                    .into_iter()
                    .map(|tool| FunctionDeclaration {
                        name: tool.name,
                        description: tool.description,
                        parameters: tool.input_schema,
                    })
                    .collect(),
            }]
        }),
        tool_config: request.tool_choice.map(|choice| ToolConfig {
            function_calling_config: FunctionCallingConfig {
                mode: match choice {
                    LanguageModelToolChoice::Auto => FunctionCallingMode::Auto,
                    LanguageModelToolChoice::Any => FunctionCallingMode::Any,
                    LanguageModelToolChoice::None => FunctionCallingMode::None,
                },
                allowed_function_names: None,
            },
        }),
    }
}

fn thinking_config_for_request(
    request: &LanguageModelRequest,
    model_id: &str,
    mode: GoogleModelMode,
) -> Option<ThinkingConfig> {
    let supports_thinking =
        matches!(mode, GoogleModelMode::Thinking { .. }) || is_google_thinking_model(model_id);
    if !supports_thinking {
        return None;
    }

    let mut config = ThinkingConfig::default();

    if request.thinking_allowed {
        config.include_thoughts = Some(true);
        config.thinking_level = request
            .thinking_effort
            .as_deref()
            .and_then(ThinkingLevel::from_effort);

        if config.thinking_level.is_none()
            && let GoogleModelMode::Thinking {
                budget_tokens: Some(budget_tokens),
            } = mode
        {
            config.thinking_budget = Some(budget_tokens);
        }
    } else if let Some(thinking_level) = disabled_thinking_level(model_id) {
        config.thinking_level = Some(thinking_level);
    } else if supports_thinking_budget_disable(model_id) {
        config.thinking_budget = Some(0);
    }

    (!config.is_empty()).then_some(config)
}

impl ThinkingConfig {
    fn is_empty(&self) -> bool {
        self.thinking_budget.is_none()
            && self.thinking_level.is_none()
            && self.include_thoughts.is_none()
    }
}

fn is_google_thinking_model(model_id: &str) -> bool {
    model_id.starts_with("gemini-2.5-") || model_id.starts_with("gemini-3")
}

fn disabled_thinking_level(model_id: &str) -> Option<ThinkingLevel> {
    match model_id {
        model_id if model_id.starts_with("gemini-3") && model_id.contains("-pro") => {
            Some(ThinkingLevel::Low)
        }
        model_id if model_id.starts_with("gemini-3") => Some(ThinkingLevel::Minimal),
        _ => None,
    }
}

fn supports_thinking_budget_disable(model_id: &str) -> bool {
    matches!(
        model_id,
        "gemini-2.5-flash"
            | "gemini-2.5-flash-lite"
            | "gemini-2.5-flash-preview-latest"
            | "gemini-2.5-flash-preview-04-17"
            | "gemini-2.5-flash-preview-05-20"
            | "gemini-2.5-flash-lite-preview-06-17"
    )
}

pub struct GoogleEventMapper {
    usage: UsageMetadata,
    stop_reason: StopReason,
}

impl GoogleEventMapper {
    pub fn new() -> Self {
        Self {
            usage: UsageMetadata::default(),
            stop_reason: StopReason::EndTurn,
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<GenerateContentResponse>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events
            .map(Some)
            .chain(futures::stream::once(async { None }))
            .flat_map(move |event| {
                futures::stream::iter(match event {
                    Some(Ok(event)) => self.map_event(event),
                    Some(Err(error)) => {
                        vec![Err(LanguageModelCompletionError::from(error))]
                    }
                    None => vec![Ok(LanguageModelCompletionEvent::Stop(self.stop_reason))],
                })
            })
    }

    pub fn map_event(
        &mut self,
        event: GenerateContentResponse,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        static TOOL_CALL_COUNTER: AtomicU64 = AtomicU64::new(0);

        let mut events: Vec<_> = Vec::new();
        let mut wants_to_use_tool = false;
        if let Some(usage_metadata) = event.usage_metadata {
            update_usage(&mut self.usage, &usage_metadata);
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(
                convert_usage(&self.usage),
            )))
        }

        if let Some(prompt_feedback) = event.prompt_feedback
            && let Some(block_reason) = prompt_feedback.block_reason.as_deref()
        {
            self.stop_reason = match block_reason {
                "SAFETY" | "OTHER" | "BLOCKLIST" | "PROHIBITED_CONTENT" | "IMAGE_SAFETY" => {
                    StopReason::Refusal
                }
                _ => {
                    log::error!("Unexpected Google block_reason: {block_reason}");
                    StopReason::Refusal
                }
            };
            events.push(Ok(LanguageModelCompletionEvent::Stop(self.stop_reason)));

            return events;
        }

        if let Some(candidates) = event.candidates {
            for candidate in candidates {
                if let Some(finish_reason) = candidate.finish_reason.as_deref() {
                    self.stop_reason = match finish_reason {
                        "STOP" => StopReason::EndTurn,
                        "MAX_TOKENS" => StopReason::MaxTokens,
                        "SAFETY"
                        | "RECITATION"
                        | "LANGUAGE"
                        | "OTHER"
                        | "BLOCKLIST"
                        | "PROHIBITED_CONTENT"
                        | "SPII"
                        | "MALFORMED_FUNCTION_CALL"
                        | "IMAGE_SAFETY"
                        | "IMAGE_PROHIBITED_CONTENT"
                        | "IMAGE_OTHER"
                        | "NO_IMAGE"
                        | "IMAGE_RECITATION"
                        | "UNEXPECTED_TOOL_CALL"
                        | "TOO_MANY_TOOL_CALLS"
                        | "MISSING_THOUGHT_SIGNATURE"
                        | "MALFORMED_RESPONSE" => StopReason::Refusal,
                        _ => {
                            log::error!("Unexpected google finish_reason: {finish_reason}");
                            StopReason::EndTurn
                        }
                    };
                }
                candidate
                    .content
                    .parts
                    .into_iter()
                    .for_each(|part| match part {
                        Part::TextPart(text_part) => {
                            let thought_signature =
                                text_part.thought_signature.filter(|s| !s.is_empty());
                            if text_part.thought {
                                if !text_part.text.is_empty() || thought_signature.is_some() {
                                    events.push(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: text_part.text,
                                        signature: thought_signature,
                                    }))
                                }
                            } else {
                                if let Some(thought_signature) = thought_signature {
                                    events.push(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: String::new(),
                                        signature: Some(thought_signature),
                                    }));
                                }
                                if !text_part.text.is_empty() {
                                    events.push(Ok(LanguageModelCompletionEvent::Text(
                                        text_part.text,
                                    )));
                                }
                            }
                        }
                        Part::InlineDataPart(_) => {}
                        Part::FunctionCallPart(function_call_part) => {
                            wants_to_use_tool = true;
                            let name: Arc<str> = function_call_part.function_call.name.into();
                            let id: LanguageModelToolUseId =
                                if let Some(ref call_id) = function_call_part.function_call.id {
                                    call_id.clone().into()
                                } else {
                                    let next_tool_id =
                                        TOOL_CALL_COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
                                    format!("{}-{}", name, next_tool_id).into()
                                };

                            // Normalize empty string signatures to None
                            let thought_signature = function_call_part
                                .thought_signature
                                .filter(|s| !s.is_empty());

                            events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id,
                                    name,
                                    is_input_complete: true,
                                    raw_input: function_call_part.function_call.args.to_string(),
                                    input: function_call_part.function_call.args,
                                    thought_signature,
                                },
                            )));
                        }
                        Part::FunctionResponsePart(_) => {}
                    });
            }
        }

        // Even when Gemini wants to use a Tool, the API
        // responds with `finish_reason: STOP`
        if wants_to_use_tool {
            self.stop_reason = StopReason::ToolUse;
            events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
        }
        events
    }
}

fn update_usage(usage: &mut UsageMetadata, new: &UsageMetadata) {
    if let Some(prompt_token_count) = new.prompt_token_count {
        usage.prompt_token_count = Some(prompt_token_count);
    }
    if let Some(cached_content_token_count) = new.cached_content_token_count {
        usage.cached_content_token_count = Some(cached_content_token_count);
    }
    if let Some(candidates_token_count) = new.candidates_token_count {
        usage.candidates_token_count = Some(candidates_token_count);
    }
    if let Some(tool_use_prompt_token_count) = new.tool_use_prompt_token_count {
        usage.tool_use_prompt_token_count = Some(tool_use_prompt_token_count);
    }
    if let Some(thoughts_token_count) = new.thoughts_token_count {
        usage.thoughts_token_count = Some(thoughts_token_count);
    }
    if let Some(total_token_count) = new.total_token_count {
        usage.total_token_count = Some(total_token_count);
    }
}

fn convert_usage(usage: &UsageMetadata) -> TokenUsage {
    let prompt_tokens = usage.prompt_token_count.unwrap_or(0);
    let cached_tokens = usage.cached_content_token_count.unwrap_or(0);
    let input_tokens = prompt_tokens - cached_tokens;
    let output_tokens = usage.candidates_token_count.unwrap_or(0);

    TokenUsage {
        input_tokens,
        output_tokens,
        cache_read_input_tokens: cached_tokens,
        cache_creation_input_tokens: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Content, FunctionCall, FunctionCallPart, GenerateContentCandidate, GenerateContentResponse,
        Part, Role as GoogleRole,
    };
    use language_model_core::LanguageModelRequestMessage;
    use serde_json::json;

    fn text_request() -> LanguageModelRequest {
        LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".to_string())],
                cache: false,
                reasoning_details: None,
            }],
            ..Default::default()
        }
    }

    #[test]
    fn into_google_requests_thought_summaries_and_thinking_level() {
        let mut request = text_request();
        request.thinking_allowed = true;
        request.thinking_effort = Some("low".to_string());

        let request = into_google(
            request,
            "gemini-3.5-flash".to_string(),
            GoogleModelMode::Thinking {
                budget_tokens: None,
            },
        );

        let thinking_config = request.generation_config.unwrap().thinking_config.unwrap();
        assert_eq!(thinking_config.include_thoughts, Some(true));
        assert_eq!(thinking_config.thinking_level, Some(ThinkingLevel::Low));

        let serialized = serde_json::to_value(thinking_config).unwrap();
        assert_eq!(serialized["thinkingLevel"], "LOW");
        assert_eq!(serialized["includeThoughts"], true);
    }

    #[test]
    fn into_google_turns_off_budget_thinking_when_supported() {
        let mut request = text_request();
        request.thinking_allowed = false;

        let request = into_google(
            request,
            "gemini-2.5-flash".to_string(),
            GoogleModelMode::Thinking {
                budget_tokens: None,
            },
        );

        let thinking_config = request.generation_config.unwrap().thinking_config.unwrap();
        assert_eq!(thinking_config.thinking_budget, Some(0));
        assert_eq!(thinking_config.include_thoughts, None);
    }

    #[test]
    fn into_google_uses_minimal_level_when_gemini_3_flash_thinking_is_off() {
        let mut request = text_request();
        request.thinking_allowed = false;

        let request = into_google(
            request,
            "gemini-3.5-flash".to_string(),
            GoogleModelMode::Thinking {
                budget_tokens: None,
            },
        );

        let thinking_config = request.generation_config.unwrap().thinking_config.unwrap();
        assert_eq!(thinking_config.thinking_level, Some(ThinkingLevel::Minimal));
        assert_eq!(thinking_config.include_thoughts, None);
    }

    #[test]
    fn into_google_replays_signed_thinking_as_thought_text_part() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::Thinking {
                    text: "summary".to_string(),
                    signature: Some("signature".to_string()),
                }],
                cache: false,
                reasoning_details: None,
            }],
            ..Default::default()
        };

        let request = into_google(
            request,
            "gemini-3.5-flash".to_string(),
            GoogleModelMode::Thinking {
                budget_tokens: None,
            },
        );

        let Part::TextPart(text_part) = &request.contents[0].parts[0] else {
            panic!("expected text part");
        };
        assert_eq!(text_part.text, "summary");
        assert!(text_part.thought);
        assert_eq!(text_part.thought_signature.as_deref(), Some("signature"));
    }

    #[test]
    fn thought_text_part_deserializes_and_maps_to_thinking_event() {
        let part: Part = serde_json::from_value(json!({
            "text": "checking the constraints",
            "thought": true,
            "thoughtSignature": "thought-signature"
        }))
        .unwrap();

        let mut mapper = GoogleEventMapper::new();
        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![part],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            Ok(LanguageModelCompletionEvent::Thinking { text, signature })
                if text == "checking the constraints"
                    && signature.as_deref() == Some("thought-signature")
        ));
    }

    #[test]
    fn signed_non_thought_text_part_preserves_signature() {
        let part: Part = serde_json::from_value(json!({
            "text": "visible text",
            "thoughtSignature": "visible-signature"
        }))
        .unwrap();

        let Part::TextPart(text_part) = part else {
            panic!("expected text part");
        };
        assert_eq!(text_part.text, "visible text");
        assert!(!text_part.thought);
        assert_eq!(
            text_part.thought_signature.as_deref(),
            Some("visible-signature")
        );
    }

    #[test]
    fn signed_non_thought_text_part_maps_signature_carrier() {
        let part: Part = serde_json::from_value(json!({
            "text": "visible text",
            "thoughtSignature": "visible-signature"
        }))
        .unwrap();

        let mut mapper = GoogleEventMapper::new();
        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![part],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);
        assert_eq!(events.len(), 2);
        assert!(matches!(
            &events[0],
            Ok(LanguageModelCompletionEvent::Thinking { text, signature })
                if text.is_empty() && signature.as_deref() == Some("visible-signature")
        ));
        assert!(matches!(
            &events[1],
            Ok(LanguageModelCompletionEvent::Text(text)) if text == "visible text"
        ));
    }

    #[test]
    fn safety_finish_reason_is_refusal() {
        let mut mapper = GoogleEventMapper::new();
        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: Vec::new(),
                    role: GoogleRole::Model,
                },
                finish_reason: Some("SAFETY".to_string()),
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        mapper.map_event(response);
        assert_eq!(mapper.stop_reason, StopReason::Refusal);
    }

    #[test]
    fn test_function_call_with_signature_creates_tool_use_with_signature() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test_function".to_string(),
                            args: json!({"arg": "value"}),
                            id: None,
                        },
                        thought_signature: Some("test_signature_123".to_string()),
                    })],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);
        assert_eq!(events.len(), 2);

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            assert_eq!(tool_use.name.as_ref(), "test_function");
            assert_eq!(
                tool_use.thought_signature.as_deref(),
                Some("test_signature_123")
            );
        } else {
            panic!("Expected ToolUse event");
        }
    }

    #[test]
    fn test_function_call_without_signature_has_none() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test_function".to_string(),
                            args: json!({"arg": "value"}),
                            id: None,
                        },
                        thought_signature: None,
                    })],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);
        assert_eq!(events.len(), 2);

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            assert!(tool_use.thought_signature.is_none());
        } else {
            panic!("Expected ToolUse event");
        }
    }

    #[test]
    fn test_empty_string_signature_normalized_to_none() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test_function".to_string(),
                            args: json!({"arg": "value"}),
                            id: None,
                        },
                        thought_signature: Some("".to_string()),
                    })],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);
        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            assert!(tool_use.thought_signature.is_none());
        } else {
            panic!("Expected ToolUse event");
        }
    }
}
