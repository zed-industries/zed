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
    InlineDataPart, ModelName, Part, SystemInstruction, TextPart, ThinkingConfig, ToolConfig,
    UsageMetadata,
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
                        vec![Part::TextPart(TextPart { text })]
                    } else {
                        vec![]
                    }
                }
                MessageContent::Thinking {
                    text: _,
                    signature: Some(signature),
                } => {
                    if !signature.is_empty() {
                        vec![Part::ThoughtPart(crate::ThoughtPart {
                            thought: true,
                            thought_signature: signature,
                        })]
                    } else {
                        vec![]
                    }
                }
                MessageContent::Thinking { .. } => {
                    vec![]
                }
                MessageContent::RedactedThinking(_) => vec![],
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
                        },
                        thought_signature,
                    })]
                }
                MessageContent::ToolResult(tool_result) => {
                    match tool_result.content {
                        language_model_core::LanguageModelToolResultContent::Text(text) => {
                            vec![Part::FunctionResponsePart(crate::FunctionResponsePart {
                                function_response: crate::FunctionResponse {
                                    name: tool_result.tool_name.to_string(),
                                    // The API expects a valid JSON object
                                    response: serde_json::json!({
                                        "output": text
                                    }),
                                },
                            })]
                        }
                        language_model_core::LanguageModelToolResultContent::Image(image) => {
                            vec![
                                Part::FunctionResponsePart(crate::FunctionResponsePart {
                                    function_response: crate::FunctionResponse {
                                        name: tool_result.tool_name.to_string(),
                                        // The API expects a valid JSON object
                                        response: serde_json::json!({
                                            "output": "Tool responded with an image"
                                        }),
                                    },
                                }),
                                Part::InlineDataPart(InlineDataPart {
                                    inline_data: GenerativeContentBlob {
                                        mime_type: "image/png".to_string(),
                                        data: image.source.to_string(),
                                    },
                                }),
                            ]
                        }
                    }
                }
            })
            .collect()
    }

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
            temperature: request.temperature.map(|t| t as f64).or(Some(1.0)),
            thinking_config: match (request.thinking_allowed, mode) {
                (true, GoogleModelMode::Thinking { budget_tokens }) => {
                    budget_tokens.map(|thinking_budget| ThinkingConfig { thinking_budget })
                }
                _ => None,
            },
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
                            events.push(Ok(LanguageModelCompletionEvent::Text(text_part.text)))
                        }
                        Part::InlineDataPart(_) => {}
                        Part::FunctionCallPart(function_call_part) => {
                            wants_to_use_tool = true;
                            let name: Arc<str> = function_call_part.function_call.name.into();
                            let next_tool_id =
                                TOOL_CALL_COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
                            let id: LanguageModelToolUseId =
                                format!("{}-{}", name, next_tool_id).into();

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
                        Part::ThoughtPart(part) => {
                            events.push(Ok(LanguageModelCompletionEvent::Thinking {
                                text: "(Encrypted thought)".to_string(), // TODO: Can we populate this from thought summaries?
                                signature: Some(part.thought_signature),
                            }));
                        }
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
    use serde_json::json;

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
