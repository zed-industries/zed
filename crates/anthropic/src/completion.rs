use anyhow::Result;
use collections::HashMap;
use futures::{Stream, StreamExt};
use language_model_core::{
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolResultContent, LanguageModelToolUse, MessageContent,
    Role, StopReason, TokenUsage,
    util::{fix_streamed_json, parse_tool_arguments},
};
use std::pin::Pin;
use std::str::FromStr;

use crate::{
    AnthropicError, AnthropicModelMode, CacheControl, CacheControlType, ContentDelta, Event,
    ImageSource, Message, RequestContent, ResponseContent, StringOrContents, Thinking, Tool,
    ToolChoice, ToolResultContent, ToolResultPart, Usage,
};

fn to_anthropic_content(content: MessageContent) -> Option<RequestContent> {
    match content {
        MessageContent::Text(text) => {
            let text = if text.chars().last().is_some_and(|c| c.is_whitespace()) {
                text.trim_end().to_string()
            } else {
                text
            };
            if !text.is_empty() {
                Some(RequestContent::Text {
                    text,
                    cache_control: None,
                })
            } else {
                None
            }
        }
        MessageContent::Thinking {
            text: thinking,
            signature,
        } => {
            if let Some(signature) = signature
                && !thinking.is_empty()
            {
                Some(RequestContent::Thinking {
                    thinking,
                    signature,
                    cache_control: None,
                })
            } else {
                None
            }
        }
        MessageContent::RedactedThinking(data) => {
            if !data.is_empty() {
                Some(RequestContent::RedactedThinking { data })
            } else {
                None
            }
        }
        MessageContent::Image(image) => Some(RequestContent::Image {
            source: ImageSource {
                source_type: "base64".to_string(),
                media_type: "image/png".to_string(),
                data: image.source.to_string(),
            },
            cache_control: None,
        }),
        MessageContent::ToolUse(tool_use) => Some(RequestContent::ToolUse {
            id: tool_use.id.to_string(),
            name: tool_use.name.to_string(),
            input: tool_use.input,
            cache_control: None,
        }),
        MessageContent::ToolResult(tool_result) => Some(RequestContent::ToolResult {
            tool_use_id: tool_result.tool_use_id.to_string(),
            is_error: tool_result.is_error,
            content: match tool_result.content {
                LanguageModelToolResultContent::Text(text) => {
                    ToolResultContent::Plain(text.to_string())
                }
                LanguageModelToolResultContent::Image(image) => {
                    ToolResultContent::Multipart(vec![ToolResultPart::Image {
                        source: ImageSource {
                            source_type: "base64".to_string(),
                            media_type: "image/png".to_string(),
                            data: image.source.to_string(),
                        },
                    }])
                }
            },
            cache_control: None,
        }),
    }
}

pub fn into_anthropic(
    request: LanguageModelRequest,
    model: String,
    default_temperature: f32,
    max_output_tokens: u64,
    mode: AnthropicModelMode,
) -> crate::Request {
    let mut new_messages: Vec<Message> = Vec::new();
    let mut system_message = String::new();

    for message in request.messages {
        if message.contents_empty() {
            continue;
        }

        match message.role {
            Role::User | Role::Assistant => {
                let mut anthropic_message_content: Vec<RequestContent> = message
                    .content
                    .into_iter()
                    .filter_map(to_anthropic_content)
                    .collect();
                let anthropic_role = match message.role {
                    Role::User => crate::Role::User,
                    Role::Assistant => crate::Role::Assistant,
                    Role::System => unreachable!("System role should never occur here"),
                };
                if anthropic_message_content.is_empty() {
                    continue;
                }

                if let Some(last_message) = new_messages.last_mut()
                    && last_message.role == anthropic_role
                {
                    last_message.content.extend(anthropic_message_content);
                    continue;
                }

                // Mark the last segment of the message as cached
                if message.cache {
                    let cache_control_value = Some(CacheControl {
                        cache_type: CacheControlType::Ephemeral,
                    });
                    for message_content in anthropic_message_content.iter_mut().rev() {
                        match message_content {
                            RequestContent::RedactedThinking { .. } => {
                                // Caching is not possible, fallback to next message
                            }
                            RequestContent::Text { cache_control, .. }
                            | RequestContent::Thinking { cache_control, .. }
                            | RequestContent::Image { cache_control, .. }
                            | RequestContent::ToolUse { cache_control, .. }
                            | RequestContent::ToolResult { cache_control, .. } => {
                                *cache_control = cache_control_value;
                                break;
                            }
                        }
                    }
                }

                new_messages.push(Message {
                    role: anthropic_role,
                    content: anthropic_message_content,
                });
            }
            Role::System => {
                if !system_message.is_empty() {
                    system_message.push_str("\n\n");
                }
                system_message.push_str(&message.string_contents());
            }
        }
    }

    crate::Request {
        model,
        messages: new_messages,
        max_tokens: max_output_tokens,
        system: if system_message.is_empty() {
            None
        } else {
            Some(StringOrContents::String(system_message))
        },
        thinking: if request.thinking_allowed {
            match mode {
                AnthropicModelMode::Thinking { budget_tokens } => {
                    Some(Thinking::Enabled { budget_tokens })
                }
                AnthropicModelMode::AdaptiveThinking => Some(Thinking::Adaptive),
                AnthropicModelMode::Default => None,
            }
        } else {
            None
        },
        tools: request
            .tools
            .into_iter()
            .map(|tool| Tool {
                name: tool.name,
                description: tool.description,
                input_schema: tool.input_schema,
                eager_input_streaming: tool.use_input_streaming,
            })
            .collect(),
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => ToolChoice::Auto,
            LanguageModelToolChoice::Any => ToolChoice::Any,
            LanguageModelToolChoice::None => ToolChoice::None,
        }),
        metadata: None,
        output_config: if request.thinking_allowed
            && matches!(mode, AnthropicModelMode::AdaptiveThinking)
        {
            request.thinking_effort.as_deref().and_then(|effort| {
                let effort = match effort {
                    "low" => Some(crate::Effort::Low),
                    "medium" => Some(crate::Effort::Medium),
                    "high" => Some(crate::Effort::High),
                    "max" => Some(crate::Effort::Max),
                    _ => None,
                };
                effort.map(|effort| crate::OutputConfig {
                    effort: Some(effort),
                })
            })
        } else {
            None
        },
        stop_sequences: Vec::new(),
        speed: request.speed.map(Into::into),
        temperature: request.temperature.or(Some(default_temperature)),
        top_k: None,
        top_p: None,
    }
}

pub struct AnthropicEventMapper {
    tool_uses_by_index: HashMap<usize, RawToolUse>,
    usage: Usage,
    stop_reason: StopReason,
}

impl AnthropicEventMapper {
    pub fn new() -> Self {
        Self {
            tool_uses_by_index: HashMap::default(),
            usage: Usage::default(),
            stop_reason: StopReason::EndTurn,
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<Event, AnthropicError>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(error.into())],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: Event,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        match event {
            Event::ContentBlockStart {
                index,
                content_block,
            } => match content_block {
                ResponseContent::Text { text } => {
                    vec![Ok(LanguageModelCompletionEvent::Text(text))]
                }
                ResponseContent::Thinking { thinking } => {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: thinking,
                        signature: None,
                    })]
                }
                ResponseContent::RedactedThinking { data } => {
                    vec![Ok(LanguageModelCompletionEvent::RedactedThinking { data })]
                }
                ResponseContent::ToolUse { id, name, .. } => {
                    self.tool_uses_by_index.insert(
                        index,
                        RawToolUse {
                            id,
                            name,
                            input_json: String::new(),
                        },
                    );
                    Vec::new()
                }
            },
            Event::ContentBlockDelta { index, delta } => match delta {
                ContentDelta::TextDelta { text } => {
                    vec![Ok(LanguageModelCompletionEvent::Text(text))]
                }
                ContentDelta::ThinkingDelta { thinking } => {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: thinking,
                        signature: None,
                    })]
                }
                ContentDelta::SignatureDelta { signature } => {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: "".to_string(),
                        signature: Some(signature),
                    })]
                }
                ContentDelta::InputJsonDelta { partial_json } => {
                    if let Some(tool_use) = self.tool_uses_by_index.get_mut(&index) {
                        tool_use.input_json.push_str(&partial_json);

                        // Try to convert invalid (incomplete) JSON into
                        // valid JSON that serde can accept, e.g. by closing
                        // unclosed delimiters. This way, we can update the
                        // UI with whatever has been streamed back so far.
                        if let Ok(input) =
                            serde_json::Value::from_str(&fix_streamed_json(&tool_use.input_json))
                        {
                            return vec![Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id: tool_use.id.clone().into(),
                                    name: tool_use.name.clone().into(),
                                    is_input_complete: false,
                                    raw_input: tool_use.input_json.clone(),
                                    input,
                                    thought_signature: None,
                                },
                            ))];
                        }
                    }
                    vec![]
                }
            },
            Event::ContentBlockStop { index } => {
                if let Some(tool_use) = self.tool_uses_by_index.remove(&index) {
                    let input_json = tool_use.input_json.trim();
                    let event_result = match parse_tool_arguments(input_json) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_use.id.into(),
                                name: tool_use.name.into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_use.input_json.clone(),
                                thought_signature: None,
                            },
                        )),
                        Err(json_parse_err) => {
                            Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                id: tool_use.id.into(),
                                tool_name: tool_use.name.into(),
                                raw_input: input_json.into(),
                                json_parse_error: json_parse_err.to_string(),
                            })
                        }
                    };

                    vec![event_result]
                } else {
                    Vec::new()
                }
            }
            Event::MessageStart { message } => {
                update_usage(&mut self.usage, &message.usage);
                vec![
                    Ok(LanguageModelCompletionEvent::UsageUpdate(convert_usage(
                        &self.usage,
                    ))),
                    Ok(LanguageModelCompletionEvent::StartMessage {
                        message_id: message.id,
                    }),
                ]
            }
            Event::MessageDelta { delta, usage } => {
                update_usage(&mut self.usage, &usage);
                if let Some(stop_reason) = delta.stop_reason.as_deref() {
                    self.stop_reason = match stop_reason {
                        "end_turn" => StopReason::EndTurn,
                        "max_tokens" => StopReason::MaxTokens,
                        "tool_use" => StopReason::ToolUse,
                        "refusal" => StopReason::Refusal,
                        _ => {
                            log::error!("Unexpected anthropic stop_reason: {stop_reason}");
                            StopReason::EndTurn
                        }
                    };
                }
                vec![Ok(LanguageModelCompletionEvent::UsageUpdate(
                    convert_usage(&self.usage),
                ))]
            }
            Event::MessageStop => {
                vec![Ok(LanguageModelCompletionEvent::Stop(self.stop_reason))]
            }
            Event::Error { error } => {
                vec![Err(error.into())]
            }
            _ => Vec::new(),
        }
    }
}

struct RawToolUse {
    id: String,
    name: String,
    input_json: String,
}

/// Updates usage data by preferring counts from `new`.
fn update_usage(usage: &mut Usage, new: &Usage) {
    if let Some(input_tokens) = new.input_tokens {
        usage.input_tokens = Some(input_tokens);
    }
    if let Some(output_tokens) = new.output_tokens {
        usage.output_tokens = Some(output_tokens);
    }
    if let Some(cache_creation_input_tokens) = new.cache_creation_input_tokens {
        usage.cache_creation_input_tokens = Some(cache_creation_input_tokens);
    }
    if let Some(cache_read_input_tokens) = new.cache_read_input_tokens {
        usage.cache_read_input_tokens = Some(cache_read_input_tokens);
    }
}

fn convert_usage(usage: &Usage) -> TokenUsage {
    TokenUsage {
        input_tokens: usage.input_tokens.unwrap_or(0),
        output_tokens: usage.output_tokens.unwrap_or(0),
        cache_creation_input_tokens: usage.cache_creation_input_tokens.unwrap_or(0),
        cache_read_input_tokens: usage.cache_read_input_tokens.unwrap_or(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AnthropicModelMode;
    use language_model_core::{LanguageModelImage, LanguageModelRequestMessage, MessageContent};

    #[test]
    fn test_cache_control_only_on_last_segment() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![
                    MessageContent::Text("Some prompt".to_string()),
                    MessageContent::Image(LanguageModelImage::empty()),
                    MessageContent::Image(LanguageModelImage::empty()),
                    MessageContent::Image(LanguageModelImage::empty()),
                    MessageContent::Image(LanguageModelImage::empty()),
                ],
                cache: true,
                reasoning_details: None,
            }],
            thread_id: None,
            prompt_id: None,
            intent: None,
            stop: vec![],
            temperature: None,
            tools: vec![],
            tool_choice: None,
            thinking_allowed: true,
            thinking_effort: None,
            speed: None,
        };

        let anthropic_request = into_anthropic(
            request,
            "claude-3-5-sonnet".to_string(),
            0.7,
            4096,
            AnthropicModelMode::Default,
        );

        assert_eq!(anthropic_request.messages.len(), 1);

        let message = &anthropic_request.messages[0];
        assert_eq!(message.content.len(), 5);

        assert!(matches!(
            message.content[0],
            RequestContent::Text {
                cache_control: None,
                ..
            }
        ));
        for i in 1..3 {
            assert!(matches!(
                message.content[i],
                RequestContent::Image {
                    cache_control: None,
                    ..
                }
            ));
        }

        assert!(matches!(
            message.content[4],
            RequestContent::Image {
                cache_control: Some(CacheControl {
                    cache_type: CacheControlType::Ephemeral,
                }),
                ..
            }
        ));
    }

    fn request_with_assistant_content(assistant_content: Vec<MessageContent>) -> crate::Request {
        let mut request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".to_string())],
                cache: false,
                reasoning_details: None,
            }],
            thinking_effort: None,
            thread_id: None,
            prompt_id: None,
            intent: None,
            stop: vec![],
            temperature: None,
            tools: vec![],
            tool_choice: None,
            thinking_allowed: true,
            speed: None,
        };
        request.messages.push(LanguageModelRequestMessage {
            role: Role::Assistant,
            content: assistant_content,
            cache: false,
            reasoning_details: None,
        });
        into_anthropic(
            request,
            "claude-sonnet-4-5".to_string(),
            1.0,
            16000,
            AnthropicModelMode::Thinking {
                budget_tokens: Some(10000),
            },
        )
    }

    #[test]
    fn test_unsigned_thinking_blocks_stripped() {
        let result = request_with_assistant_content(vec![
            MessageContent::Thinking {
                text: "Cancelled mid-think, no signature".to_string(),
                signature: None,
            },
            MessageContent::Text("Some response text".to_string()),
        ]);

        let assistant_message = result
            .messages
            .iter()
            .find(|m| m.role == crate::Role::Assistant)
            .expect("assistant message should still exist");

        assert_eq!(
            assistant_message.content.len(),
            1,
            "Only the text content should remain; unsigned thinking block should be stripped"
        );
        assert!(matches!(
            &assistant_message.content[0],
            RequestContent::Text { text, .. } if text == "Some response text"
        ));
    }

    #[test]
    fn test_signed_thinking_blocks_preserved() {
        let result = request_with_assistant_content(vec![
            MessageContent::Thinking {
                text: "Completed thinking".to_string(),
                signature: Some("valid-signature".to_string()),
            },
            MessageContent::Text("Response".to_string()),
        ]);

        let assistant_message = result
            .messages
            .iter()
            .find(|m| m.role == crate::Role::Assistant)
            .expect("assistant message should exist");

        assert_eq!(
            assistant_message.content.len(),
            2,
            "Both the signed thinking block and text should be preserved"
        );
        assert!(matches!(
            &assistant_message.content[0],
            RequestContent::Thinking { thinking, signature, .. }
                if thinking == "Completed thinking" && signature == "valid-signature"
        ));
    }

    #[test]
    fn test_only_unsigned_thinking_block_omits_entire_message() {
        let result = request_with_assistant_content(vec![MessageContent::Thinking {
            text: "Cancelled before any text or signature".to_string(),
            signature: None,
        }]);

        let assistant_messages: Vec<_> = result
            .messages
            .iter()
            .filter(|m| m.role == crate::Role::Assistant)
            .collect();

        assert_eq!(
            assistant_messages.len(),
            0,
            "An assistant message whose only content was an unsigned thinking block \
             should be omitted entirely"
        );
    }
}
