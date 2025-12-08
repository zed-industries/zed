use anyhow::Result;
use futures::{FutureExt, Stream, StreamExt, future::BoxFuture};
use google_ai::{
    FunctionDeclaration, GenerateContentResponse, GoogleModelMode, Part, SystemInstruction,
    ThinkingConfig, UsageMetadata,
};
use gpui::{App, AppContext as _};
use language_model::{
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolUse, LanguageModelToolUseId, MessageContent, Role,
    StopReason,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use settings::GoogleAvailableModel as AvailableModel;
use std::{
    pin::Pin,
    sync::atomic::{self, AtomicU64},
};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct GoogleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        /// The maximum number of tokens to use for reasoning. Must be lower than the model's `max_output_tokens`.
        budget_tokens: Option<u32>,
    },
}

pub fn into_google(
    mut request: LanguageModelRequest,
    model_id: String,
    mode: GoogleModelMode,
) -> google_ai::GenerateContentRequest {
    fn map_content(content: Vec<MessageContent>) -> Vec<Part> {
        content
            .into_iter()
            .flat_map(|content| match content {
                language_model::MessageContent::Text(text) => {
                    if !text.is_empty() {
                        vec![Part::TextPart(google_ai::TextPart { text })]
                    } else {
                        vec![]
                    }
                }
                language_model::MessageContent::Thinking {
                    text: _,
                    signature: Some(signature),
                } => {
                    if !signature.is_empty() {
                        vec![Part::ThoughtPart(google_ai::ThoughtPart {
                            thought: true,
                            thought_signature: signature,
                        })]
                    } else {
                        vec![]
                    }
                }
                language_model::MessageContent::Thinking { .. } => {
                    vec![]
                }
                language_model::MessageContent::RedactedThinking(_) => vec![],
                language_model::MessageContent::Image(image) => {
                    vec![Part::InlineDataPart(google_ai::InlineDataPart {
                        inline_data: google_ai::GenerativeContentBlob {
                            mime_type: "image/png".to_string(),
                            data: image.source.to_string(),
                        },
                    })]
                }
                language_model::MessageContent::ToolUse(tool_use) => {
                    let thought_signature = tool_use.thought_signature.filter(|s| !s.is_empty());

                    vec![Part::FunctionCallPart(google_ai::FunctionCallPart {
                        function_call: google_ai::FunctionCall {
                            name: tool_use.name.to_string(),
                            args: tool_use.input,
                        },
                        thought_signature,
                    })]
                }
                language_model::MessageContent::ToolResult(tool_result) => {
                    match tool_result.content {
                        language_model::LanguageModelToolResultContent::Text(text) => {
                            vec![Part::FunctionResponsePart(
                                google_ai::FunctionResponsePart {
                                    function_response: google_ai::FunctionResponse {
                                        name: tool_result.tool_name.to_string(),
                                        response: serde_json::json!({
                                            "output": text
                                        }),
                                    },
                                },
                            )]
                        }
                        language_model::LanguageModelToolResultContent::Image(image) => {
                            vec![
                                Part::FunctionResponsePart(google_ai::FunctionResponsePart {
                                    function_response: google_ai::FunctionResponse {
                                        name: tool_result.tool_name.to_string(),
                                        response: serde_json::json!({
                                            "output": "Tool responded with an image"
                                        }),
                                    },
                                }),
                                Part::InlineDataPart(google_ai::InlineDataPart {
                                    inline_data: google_ai::GenerativeContentBlob {
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

    google_ai::GenerateContentRequest {
        model: google_ai::ModelName { model_id },
        system_instruction: system_instructions,
        contents: request
            .messages
            .into_iter()
            .filter_map(|message| {
                let parts = map_content(message.content);
                if parts.is_empty() {
                    None
                } else {
                    Some(google_ai::Content {
                        parts,
                        role: match message.role {
                            Role::User => google_ai::Role::User,
                            Role::Assistant => google_ai::Role::Model,
                            Role::System => google_ai::Role::User,
                        },
                    })
                }
            })
            .collect(),
        generation_config: Some(google_ai::GenerationConfig {
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
            vec![google_ai::Tool {
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
        tool_config: request.tool_choice.map(|choice| google_ai::ToolConfig {
            function_calling_config: google_ai::FunctionCallingConfig {
                mode: match choice {
                    LanguageModelToolChoice::Auto => google_ai::FunctionCallingMode::Auto,
                    LanguageModelToolChoice::Any => google_ai::FunctionCallingMode::Any,
                    LanguageModelToolChoice::None => google_ai::FunctionCallingMode::None,
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
                            let name: std::sync::Arc<str> =
                                function_call_part.function_call.name.into();
                            let next_tool_id =
                                TOOL_CALL_COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
                            let id: LanguageModelToolUseId =
                                format!("{}-{}", name, next_tool_id).into();

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
                                text: "(Encrypted thought)".to_string(),
                                signature: Some(part.thought_signature),
                            }));
                        }
                    });
            }
        }

        if wants_to_use_tool {
            self.stop_reason = StopReason::ToolUse;
            events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
        }
        events
    }
}

pub fn count_google_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
    cx.background_spawn(async move {
        let messages = request
            .messages
            .into_iter()
            .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
                role: match message.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                    Role::System => "system".into(),
                },
                content: Some(message.string_contents()),
                name: None,
                function_call: None,
            })
            .collect::<Vec<_>>();

        tiktoken_rs::num_tokens_from_messages("gpt-4", &messages).map(|tokens| tokens as u64)
    })
    .boxed()
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

fn convert_usage(usage: &UsageMetadata) -> language_model::TokenUsage {
    let prompt_tokens = usage.prompt_token_count.unwrap_or(0);
    let cached_tokens = usage.cached_content_token_count.unwrap_or(0);
    let input_tokens = prompt_tokens - cached_tokens;
    let output_tokens = usage.candidates_token_count.unwrap_or(0);

    language_model::TokenUsage {
        input_tokens,
        output_tokens,
        cache_read_input_tokens: cached_tokens,
        cache_creation_input_tokens: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use google_ai::{
        Content, FunctionCall, FunctionCallPart, GenerateContentCandidate, GenerateContentResponse,
        Part, Role as GoogleRole, TextPart,
    };
    use language_model::{LanguageModelToolUseId, MessageContent, Role};
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

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            assert_eq!(tool_use.thought_signature, None);
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
            assert_eq!(tool_use.thought_signature, None);
        } else {
            panic!("Expected ToolUse event");
        }
    }

    #[test]
    fn test_parallel_function_calls_preserve_signatures() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![
                        Part::FunctionCallPart(FunctionCallPart {
                            function_call: FunctionCall {
                                name: "function_a".to_string(),
                                args: json!({}),
                            },
                            thought_signature: Some("sig_a".to_string()),
                        }),
                        Part::FunctionCallPart(FunctionCallPart {
                            function_call: FunctionCall {
                                name: "function_b".to_string(),
                                args: json!({}),
                            },
                            thought_signature: None,
                        }),
                        Part::FunctionCallPart(FunctionCallPart {
                            function_call: FunctionCall {
                                name: "function_c".to_string(),
                                args: json!({}),
                            },
                            thought_signature: Some("sig_c".to_string()),
                        }),
                    ],
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

        let tool_uses: Vec<_> = events
            .iter()
            .filter_map(|e| {
                if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = e {
                    Some(tool_use)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(tool_uses.len(), 3);
        assert_eq!(tool_uses[0].thought_signature.as_deref(), Some("sig_a"));
        assert_eq!(tool_uses[1].thought_signature, None);
        assert_eq!(tool_uses[2].thought_signature.as_deref(), Some("sig_c"));
    }

    #[test]
    fn test_tool_use_with_signature_converts_to_function_call_part() {
        let tool_use = language_model::LanguageModelToolUse {
            id: LanguageModelToolUseId::from("test-id"),
            name: "test_tool".into(),
            input: json!({"key": "value"}),
            raw_input: r#"{"key": "value"}"#.to_string(),
            is_input_complete: true,
            thought_signature: Some("test_sig".to_string()),
        };

        let request = into_google(
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolUse(tool_use)],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
            "gemini-2.5-flash".to_string(),
            GoogleModelMode::Default,
        );

        let parts = &request.contents[0].parts;
        assert_eq!(parts.len(), 1);

        if let Part::FunctionCallPart(fcp) = &parts[0] {
            assert_eq!(fcp.thought_signature.as_deref(), Some("test_sig"));
        } else {
            panic!("Expected FunctionCallPart");
        }
    }

    #[test]
    fn test_tool_use_without_signature_omits_field() {
        let tool_use = language_model::LanguageModelToolUse {
            id: LanguageModelToolUseId::from("test-id"),
            name: "test_tool".into(),
            input: json!({"key": "value"}),
            raw_input: r#"{"key": "value"}"#.to_string(),
            is_input_complete: true,
            thought_signature: None,
        };

        let request = into_google(
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolUse(tool_use)],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
            "gemini-2.5-flash".to_string(),
            GoogleModelMode::Default,
        );

        let parts = &request.contents[0].parts;

        if let Part::FunctionCallPart(fcp) = &parts[0] {
            assert_eq!(fcp.thought_signature, None);
        } else {
            panic!("Expected FunctionCallPart");
        }
    }

    #[test]
    fn test_empty_signature_in_tool_use_normalized_to_none() {
        let tool_use = language_model::LanguageModelToolUse {
            id: LanguageModelToolUseId::from("test-id"),
            name: "test_tool".into(),
            input: json!({}),
            raw_input: "{}".to_string(),
            is_input_complete: true,
            thought_signature: Some("".to_string()),
        };

        let request = into_google(
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolUse(tool_use)],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
            "gemini-2.5-flash".to_string(),
            GoogleModelMode::Default,
        );

        let parts = &request.contents[0].parts;

        if let Part::FunctionCallPart(fcp) = &parts[0] {
            assert_eq!(fcp.thought_signature, None);
        } else {
            panic!("Expected FunctionCallPart");
        }
    }

    #[test]
    fn test_round_trip_preserves_signature() {
        let original_signature = "original_thought_signature_abc123";

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test_function".to_string(),
                            args: json!({"arg": "value"}),
                        },
                        thought_signature: Some(original_signature.to_string()),
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

        let mut mapper = GoogleEventMapper::new();
        let events = mapper.map_event(response);

        let tool_use = if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            tool_use.clone()
        } else {
            panic!("Expected ToolUse event");
        };

        let request = into_google(
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolUse(tool_use)],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
            "gemini-2.5-flash".to_string(),
            GoogleModelMode::Default,
        );

        let parts = &request.contents[0].parts;
        if let Part::FunctionCallPart(fcp) = &parts[0] {
            assert_eq!(fcp.thought_signature.as_deref(), Some(original_signature));
        } else {
            panic!("Expected FunctionCallPart");
        }
    }

    #[test]
    fn test_mixed_text_and_function_call_with_signature() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![
                        Part::TextPart(TextPart {
                            text: "Let me help you with that.".to_string(),
                        }),
                        Part::FunctionCallPart(FunctionCallPart {
                            function_call: FunctionCall {
                                name: "search".to_string(),
                                args: json!({"query": "test"}),
                            },
                            thought_signature: Some("thinking_sig".to_string()),
                        }),
                    ],
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

        let mut found_text = false;
        let mut found_tool_with_sig = false;

        for event in events {
            match event {
                Ok(LanguageModelCompletionEvent::Text(text)) => {
                    assert_eq!(text, "Let me help you with that.");
                    found_text = true;
                }
                Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) => {
                    assert_eq!(tool_use.thought_signature.as_deref(), Some("thinking_sig"));
                    found_tool_with_sig = true;
                }
                _ => {}
            }
        }

        assert!(found_text, "Should have found text event");
        assert!(
            found_tool_with_sig,
            "Should have found tool use with signature"
        );
    }

    #[test]
    fn test_special_characters_in_signature_preserved() {
        let special_signature = "sig/with+special=chars&more%stuff";

        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test".to_string(),
                            args: json!({}),
                        },
                        thought_signature: Some(special_signature.to_string()),
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
            assert_eq!(
                tool_use.thought_signature.as_deref(),
                Some(special_signature)
            );
        } else {
            panic!("Expected ToolUse event");
        }
    }
}
