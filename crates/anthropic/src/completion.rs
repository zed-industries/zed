use anyhow::{Result, anyhow};
use collections::HashMap;
use futures::{Stream, StreamExt};
use language_model_core::{
    CompactedContext, CompactionUpdate, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelRequest,
    LanguageModelRequestToolInput, LanguageModelToolChoice, LanguageModelToolResultContent,
    LanguageModelToolUse, LanguageModelToolUseInput, MessageContent, ProviderCompactionState, Role,
    SharedString, StopReason, TokenUsage,
    util::{fix_streamed_json, parse_tool_arguments},
};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;

use crate::{
    AdaptiveThinkingDisplay, AnthropicError, AnthropicModelMode, CacheControl, CacheControlType,
    CacheTtl, CompactionTrigger, ContentDelta, ContextManagement, ContextManagementEdit, Event,
    ImageSource, Message, RequestContent, ResponseContent, StringOrContents, Thinking, Tool,
    ToolChoice, ToolResultContent, ToolResultPart, Usage, completion_error_from_anthropic,
    completion_error_from_anthropic_api,
};

pub const COMPACTION_STATE_FORMAT: &str = "anthropic.messages.encrypted-content.v1";

/// Packages a compaction block's opaque `encrypted_content` into provider
/// state owned by `owner`.
///
/// Anthropic requires the metadata to be round-tripped verbatim, and only the
/// backend whose infrastructure produced it can make sense of it. The owner
/// recorded here is what [`provider_compaction_encrypted_content`] later
/// compares against, so it must identify that backend, not merely the wire
/// protocol.
pub fn provider_compaction_state_from_encrypted_content(
    owner: LanguageModelProviderId,
    encrypted_content: impl Into<Arc<str>>,
) -> ProviderCompactionState {
    ProviderCompactionState::new(
        owner,
        SharedString::new_static(COMPACTION_STATE_FORMAT),
        encrypted_content,
    )
}

/// Recovers the `encrypted_content` to round-trip from `state` if it is owned
/// by `owner`, or `None` when the state belongs to a different backend and the
/// summary should be replayed without it.
pub fn provider_compaction_encrypted_content(
    state: &ProviderCompactionState,
    owner: &LanguageModelProviderId,
) -> Result<Option<Arc<str>>> {
    if state.provider_id() != owner {
        return Ok(None);
    }
    if state.format() != COMPACTION_STATE_FORMAT {
        return Err(anyhow!(
            "unsupported Anthropic compaction state format: {}",
            state.format()
        ));
    }
    Ok(Some(state.payload().into()))
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AnthropicPromptCacheMode {
    Disabled,
    Legacy,
    #[default]
    Automatic,
}

fn set_cache_control(content: &mut RequestContent, cache_control: Option<CacheControl>) -> bool {
    match content {
        RequestContent::RedactedThinking { .. } => false,
        RequestContent::Text {
            cache_control: target,
            ..
        }
        | RequestContent::Thinking {
            cache_control: target,
            ..
        }
        | RequestContent::Image {
            cache_control: target,
            ..
        }
        | RequestContent::ToolUse {
            cache_control: target,
            ..
        }
        | RequestContent::ToolResult {
            cache_control: target,
            ..
        }
        | RequestContent::Compaction {
            cache_control: target,
            ..
        } => {
            *target = cache_control;
            true
        }
    }
}

fn mark_last_cacheable_content(content: &mut [RequestContent], cache_control: CacheControl) {
    for content in content.iter_mut().rev() {
        if set_cache_control(content, Some(cache_control)) {
            break;
        }
    }
}

fn to_anthropic_content(
    content: MessageContent,
    compaction_state_owner: &LanguageModelProviderId,
) -> Result<Option<RequestContent>> {
    match content {
        MessageContent::Text(text) => {
            let text = if text.chars().last().is_some_and(|c| c.is_whitespace()) {
                text.trim_end().to_string()
            } else {
                text
            };
            if !text.is_empty() {
                Ok(Some(RequestContent::Text {
                    text,
                    cache_control: None,
                }))
            } else {
                Ok(None)
            }
        }
        MessageContent::Thinking {
            text: thinking,
            signature,
        } => {
            if let Some(signature) = signature
                && !thinking.is_empty()
            {
                Ok(Some(RequestContent::Thinking {
                    thinking,
                    signature,
                    cache_control: None,
                }))
            } else {
                Ok(None)
            }
        }
        MessageContent::RedactedThinking(data) => {
            if !data.is_empty() {
                Ok(Some(RequestContent::RedactedThinking { data }))
            } else {
                Ok(None)
            }
        }
        MessageContent::Image(image) => Ok(Some(RequestContent::Image {
            source: ImageSource {
                source_type: "base64".to_string(),
                media_type: "image/png".to_string(),
                data: image.source.to_string(),
            },
            cache_control: None,
        })),
        MessageContent::ToolUse(tool_use) => match tool_use.input {
            LanguageModelToolUseInput::Json(input) => Ok(Some(RequestContent::ToolUse {
                id: tool_use.id.to_string(),
                name: tool_use.name.to_string(),
                input,
                cache_control: None,
            })),
            LanguageModelToolUseInput::Text(_) => Err(anyhow::anyhow!(
                "Anthropic does not support custom tool calls"
            )),
        },
        MessageContent::ToolResult(tool_result) => {
            let content = match tool_result.content.as_slice() {
                [LanguageModelToolResultContent::Text(text)] => {
                    ToolResultContent::Plain(text.to_string())
                }
                _ => {
                    let parts = tool_result
                        .content
                        .into_iter()
                        .map(|part| match part {
                            LanguageModelToolResultContent::Text(text) => ToolResultPart::Text {
                                text: text.to_string(),
                            },
                            LanguageModelToolResultContent::Image(image) => ToolResultPart::Image {
                                source: ImageSource {
                                    source_type: "base64".to_string(),
                                    media_type: "image/png".to_string(),
                                    data: image.source.to_string(),
                                },
                            },
                        })
                        .collect();
                    ToolResultContent::Multipart(parts)
                }
            };
            Ok(Some(RequestContent::ToolResult {
                tool_use_id: tool_result.tool_use_id.to_string(),
                is_error: tool_result.is_error,
                content,
                cache_control: None,
            }))
        }
        MessageContent::Compaction(CompactedContext::Summary {
            content,
            provider_state,
        }) => {
            let encrypted_content = match &provider_state {
                Some(state) => {
                    provider_compaction_encrypted_content(state, compaction_state_owner)?
                }
                None => None,
            };
            Ok(Some(RequestContent::Compaction {
                content: Some(content),
                encrypted_content,
                cache_control: None,
            }))
        }
        MessageContent::Compaction(CompactedContext::ProviderState(_)) => Ok(None),
    }
}

pub fn into_anthropic(
    request: LanguageModelRequest,
    model: String,
    default_temperature: f32,
    max_output_tokens: u64,
    mode: AnthropicModelMode,
    cache_mode: AnthropicPromptCacheMode,
    compaction_state_owner: &LanguageModelProviderId,
) -> Result<crate::Request> {
    let mut new_messages: Vec<Message> = Vec::new();
    let mut system_message = String::new();
    let mut any_message_wants_cache = false;

    for message in request.messages {
        if message.contents_empty() {
            continue;
        }

        any_message_wants_cache |= message.cache;

        match message.role {
            Role::User | Role::Assistant => {
                let mut anthropic_message_content = Vec::new();
                for content in message.content {
                    if let Some(content) = to_anthropic_content(content, compaction_state_owner)? {
                        anthropic_message_content.push(content);
                    }
                }
                let anthropic_role = match message.role {
                    Role::User => crate::Role::User,
                    Role::Assistant => crate::Role::Assistant,
                    Role::System => unreachable!("System role should never occur here"),
                };
                if anthropic_message_content.is_empty() {
                    continue;
                }

                if cache_mode == AnthropicPromptCacheMode::Legacy && message.cache {
                    mark_last_cacheable_content(
                        &mut anthropic_message_content,
                        CacheControl {
                            cache_type: CacheControlType::Ephemeral,
                            ttl: None,
                        },
                    );
                }

                if let Some(last_message) = new_messages.last_mut()
                    && last_message.role == anthropic_role
                {
                    last_message.content.extend(anthropic_message_content);
                    continue;
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

    // When caching is enabled, mark the static prefix (tools + system) with an
    // explicit long-TTL breakpoint, and let Anthropic's automatic top-level
    // cache_control handle the short-TTL conversation breakpoint. Anthropic
    // requires that longer TTLs appear earlier in the prefix, and the prefix
    // order is tools → system → messages, so long-TTL tools/system before a
    // short-TTL conversation breakpoint is a valid mix.
    let long_lived_cache = (cache_mode == AnthropicPromptCacheMode::Automatic
        && any_message_wants_cache)
        .then_some(CacheControl {
            cache_type: CacheControlType::Ephemeral,
            ttl: Some(CacheTtl::OneHour),
        });

    let system = if system_message.is_empty() {
        None
    } else if let Some(cache_control) = long_lived_cache {
        Some(StringOrContents::Content(vec![RequestContent::Text {
            text: system_message,
            cache_control: Some(cache_control),
        }]))
    } else {
        Some(StringOrContents::String(system_message))
    };

    let mut tools: Vec<Tool> = request
        .tools
        .into_iter()
        .map(|tool| match tool.input {
            LanguageModelRequestToolInput::Function {
                input_schema,
                use_input_streaming,
            } => Ok(Tool {
                name: tool.name,
                description: tool.description,
                input_schema,
                eager_input_streaming: use_input_streaming,
                cache_control: None,
            }),
            LanguageModelRequestToolInput::Custom { .. } => {
                Err(anyhow::anyhow!("Anthropic does not support custom tools"))
            }
        })
        .collect::<Result<_>>()?;
    if let Some(cache_control) = long_lived_cache
        && let Some(last_tool) = tools.last_mut()
    {
        last_tool.cache_control = Some(cache_control);
    }

    let thinking = if request.thinking_allowed {
        match mode {
            AnthropicModelMode::Thinking { budget_tokens } => {
                Some(Thinking::Enabled { budget_tokens })
            }
            AnthropicModelMode::AdaptiveThinking => Some(Thinking::Adaptive {
                display: Some(AdaptiveThinkingDisplay::Summarized),
            }),
            AnthropicModelMode::Default => None,
        }
    } else if crate::requires_explicit_thinking_opt_out(&model) {
        // On Claude Opus 5, omitting the `thinking` field no longer means
        // "off": the model runs adaptive thinking by default, so features
        // that suppress thinking (e.g. inline assist) must opt out
        // explicitly. `disabled` is only accepted at effort `high` or below;
        // that holds here because `output_config` is never sent when thinking
        // is disallowed, and the server-side default effort is `high`.
        // <https://platform.claude.com/docs/en/about-claude/models/migration-guide#migrating-to-claude-opus-5>
        Some(Thinking::Disabled)
    } else {
        None
    };

    Ok(crate::Request {
        model,
        messages: new_messages,
        max_tokens: max_output_tokens,
        system,
        // Opt into Anthropic's automatic prompt caching for the conversation
        // tail. Omitting `ttl` uses the default (short) TTL, which refreshes
        // for free on every cache hit — ideal for the rapidly-changing
        // conversation suffix.
        cache_control: (cache_mode == AnthropicPromptCacheMode::Automatic
            && any_message_wants_cache)
            .then_some(CacheControl {
                cache_type: CacheControlType::Ephemeral,
                ttl: None,
            }),
        thinking,
        tools,
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
                    "xhigh" => Some(crate::Effort::XHigh),
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
        context_management: request.compact_at_tokens.map(|value| ContextManagement {
            edits: vec![ContextManagementEdit::Compact {
                trigger: Some(CompactionTrigger::InputTokens { value }),
            }],
        }),
    })
}

pub struct AnthropicEventMapper {
    tool_uses_by_index: HashMap<usize, RawToolUse>,
    compactions_by_index: HashMap<usize, RawCompaction>,
    usage: Usage,
    stop_reason: StopReason,
    provider_name: LanguageModelProviderName,
    compaction_state_owner: LanguageModelProviderId,
}

impl AnthropicEventMapper {
    /// `compaction_state_owner` identifies the backend whose infrastructure
    /// produced this stream, so that any `encrypted_content` it emits is only
    /// ever round-tripped back to that same backend.
    pub fn new(
        provider_name: LanguageModelProviderName,
        compaction_state_owner: LanguageModelProviderId,
    ) -> Self {
        Self {
            tool_uses_by_index: HashMap::default(),
            compactions_by_index: HashMap::default(),
            usage: Usage::default(),
            stop_reason: StopReason::EndTurn,
            provider_name,
            compaction_state_owner,
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
                Err(error) => vec![Err(completion_error_from_anthropic(
                    error,
                    self.provider_name.clone(),
                ))],
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
                ResponseContent::Compaction {
                    content,
                    encrypted_content,
                } => {
                    let mut events = vec![Ok(LanguageModelCompletionEvent::Compaction(
                        CompactionUpdate::Started,
                    ))];
                    let compaction = self.compactions_by_index.entry(index).or_default();
                    if let Some(encrypted_content) =
                        encrypted_content.filter(|encrypted| !encrypted.is_empty())
                    {
                        compaction.encrypted_content = Some(encrypted_content);
                    }
                    if let Some(content) = content
                        && !content.is_empty()
                    {
                        compaction.summary.push_str(&content);
                        events.push(Ok(LanguageModelCompletionEvent::Compaction(
                            CompactionUpdate::SummaryDelta(content),
                        )));
                    }
                    events
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
                ContentDelta::CompactionDelta {
                    content,
                    encrypted_content,
                } => {
                    let Some(compaction) = self.compactions_by_index.get_mut(&index) else {
                        return vec![Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                            "Anthropic streamed a compaction delta before starting its content block"
                        )))];
                    };
                    // Unlike summary text, `encrypted_content` arrives whole:
                    // a later delta carries a complete replacement value, not
                    // a chunk to append (Anthropic's own SDKs assign it, the
                    // way they do thinking signatures).
                    if let Some(encrypted_content) =
                        encrypted_content.filter(|encrypted| !encrypted.is_empty())
                    {
                        compaction.encrypted_content = Some(encrypted_content);
                    }
                    let Some(content) = content.filter(|content| !content.is_empty()) else {
                        return Vec::new();
                    };
                    compaction.summary.push_str(&content);
                    vec![Ok(LanguageModelCompletionEvent::Compaction(
                        CompactionUpdate::SummaryDelta(content),
                    ))]
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
                                    input: LanguageModelToolUseInput::Json(input),
                                    thought_signature: None,
                                },
                            ))];
                        }
                    }
                    vec![]
                }
            },
            Event::ContentBlockStop { index } => {
                if let Some(compaction) = self.compactions_by_index.remove(&index) {
                    // A compaction block that closes without content is a
                    // documented failed compaction, which the server treats
                    // as a no-op: there is nothing to persist, and the
                    // conversation continues on the uncompacted transcript.
                    if compaction.summary.is_empty() {
                        return vec![Ok(LanguageModelCompletionEvent::Compaction(
                            CompactionUpdate::Failed,
                        ))];
                    }
                    let provider_state = compaction.encrypted_content.map(|encrypted_content| {
                        provider_compaction_state_from_encrypted_content(
                            self.compaction_state_owner.clone(),
                            encrypted_content,
                        )
                    });
                    vec![Ok(LanguageModelCompletionEvent::Compaction(
                        CompactionUpdate::Finished(CompactedContext::Summary {
                            content: compaction.summary.into(),
                            provider_state,
                        }),
                    ))]
                } else if let Some(tool_use) = self.tool_uses_by_index.remove(&index) {
                    let input_json = tool_use.input_json.trim();
                    let event_result = match parse_tool_arguments(input_json) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_use.id.into(),
                                name: tool_use.name.into(),
                                is_input_complete: true,
                                input: LanguageModelToolUseInput::Json(input),
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
                // Anthropic closes every content block before ending the
                // message, so an unclosed compaction block means the stream
                // was malformed and its finalized summary never arrived.
                // Consumers would otherwise see `Started` with no terminal
                // event and treat the compaction as still in progress.
                if !self.compactions_by_index.is_empty() {
                    self.compactions_by_index.clear();
                    return vec![Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                        "Anthropic ended the stream without finishing its compaction summary"
                    )))];
                }
                vec![Ok(LanguageModelCompletionEvent::Stop(self.stop_reason))]
            }
            Event::Error { error } => {
                vec![Err(completion_error_from_anthropic_api(
                    error,
                    self.provider_name.clone(),
                ))]
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

#[derive(Default)]
struct RawCompaction {
    summary: String,
    encrypted_content: Option<Arc<str>>,
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
    use crate::{AnthropicModelMode, UsageIteration, UsageIterationType};
    use language_model_core::{
        ANTHROPIC_PROVIDER_ID, ANTHROPIC_PROVIDER_NAME, LanguageModelImage,
        LanguageModelRequestMessage, MessageContent,
    };

    #[test]
    fn test_caching_uses_top_level_auto_and_long_lived_prefix() {
        let request = LanguageModelRequest {
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("You are helpful.".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![
                        MessageContent::Text("Some prompt".to_string()),
                        MessageContent::Image(LanguageModelImage::empty()),
                        MessageContent::Image(LanguageModelImage::empty()),
                    ],
                    cache: true,
                    reasoning_details: None,
                },
            ],
            thread_id: None,
            prompt_id: None,
            intent: None,
            stop: vec![],
            temperature: None,
            tools: vec![language_model_core::LanguageModelRequestTool::function(
                "do_thing".into(),
                "Does a thing.".into(),
                serde_json::json!({"type": "object"}),
                false,
            )],
            tool_choice: None,
            thinking_allowed: true,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let anthropic_request = into_anthropic(
            request,
            "claude-3-5-sonnet".to_string(),
            0.7,
            4096,
            AnthropicModelMode::Default,
            AnthropicPromptCacheMode::Automatic,
            &ANTHROPIC_PROVIDER_ID,
        )
        .unwrap();

        // No message content block should carry cache_control anymore; the
        // conversation breakpoint is set via top-level automatic caching.
        assert_eq!(anthropic_request.messages.len(), 1);
        for block in &anthropic_request.messages[0].content {
            let cache_control = match block {
                RequestContent::Text { cache_control, .. }
                | RequestContent::Thinking { cache_control, .. }
                | RequestContent::Image { cache_control, .. }
                | RequestContent::ToolUse { cache_control, .. }
                | RequestContent::ToolResult { cache_control, .. }
                | RequestContent::Compaction { cache_control, .. } => *cache_control,
                RequestContent::RedactedThinking { .. } => None,
            };
            assert!(
                cache_control.is_none(),
                "message content blocks should no longer be individually marked",
            );
        }

        // Top-level cache_control opts into automatic caching with the default
        // 5-minute TTL for the conversation tail.
        assert!(matches!(
            anthropic_request.cache_control,
            Some(CacheControl {
                cache_type: CacheControlType::Ephemeral,
                ttl: None,
            })
        ));

        // System prompt is emitted in array form with a long-TTL breakpoint on
        // the final text block.
        match anthropic_request.system {
            Some(StringOrContents::Content(ref blocks)) => {
                assert_eq!(blocks.len(), 1);
                assert!(matches!(
                    blocks[0],
                    RequestContent::Text {
                        cache_control: Some(CacheControl {
                            cache_type: CacheControlType::Ephemeral,
                            ttl: Some(CacheTtl::OneHour),
                        }),
                        ..
                    }
                ));
            }
            other => panic!("expected system content array, got {other:?}"),
        }

        // The last (and only) tool carries a long-TTL breakpoint.
        assert_eq!(anthropic_request.tools.len(), 1);
        assert!(matches!(
            anthropic_request.tools[0].cache_control,
            Some(CacheControl {
                cache_type: CacheControlType::Ephemeral,
                ttl: Some(CacheTtl::OneHour),
            })
        ));
    }

    #[test]
    fn test_legacy_caching_marks_last_message_content_block() {
        let request = LanguageModelRequest {
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("You are helpful.".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![
                        MessageContent::Text("Some prompt".to_string()),
                        MessageContent::Image(LanguageModelImage::empty()),
                    ],
                    cache: true,
                    reasoning_details: None,
                },
            ],
            thread_id: None,
            prompt_id: None,
            intent: None,
            stop: vec![],
            temperature: None,
            tools: vec![language_model_core::LanguageModelRequestTool::function(
                "do_thing".into(),
                "Does a thing.".into(),
                serde_json::json!({"type": "object"}),
                false,
            )],
            tool_choice: None,
            thinking_allowed: true,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let anthropic_request = into_anthropic(
            request,
            "claude-3-5-sonnet".to_string(),
            0.7,
            4096,
            AnthropicModelMode::Default,
            AnthropicPromptCacheMode::Legacy,
            &ANTHROPIC_PROVIDER_ID,
        )
        .unwrap();

        assert!(anthropic_request.cache_control.is_none());
        assert!(matches!(
            anthropic_request.system,
            Some(StringOrContents::String(_))
        ));
        assert_eq!(anthropic_request.tools.len(), 1);
        assert!(anthropic_request.tools[0].cache_control.is_none());
        assert_eq!(anthropic_request.messages.len(), 1);
        assert!(matches!(
            anthropic_request.messages[0].content[0],
            RequestContent::Text {
                cache_control: None,
                ..
            }
        ));
        assert!(matches!(
            anthropic_request.messages[0].content[1],
            RequestContent::Image {
                cache_control: Some(CacheControl {
                    cache_type: CacheControlType::Ephemeral,
                    ttl: None,
                }),
                ..
            }
        ));
    }

    #[test]
    fn test_xhigh_effort_is_serialized_for_adaptive_thinking() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hi".to_string())],
                cache: false,
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
            thinking_effort: Some("xhigh".into()),
            speed: None,
            compact_at_tokens: None,
        };

        let anthropic_request = into_anthropic(
            request,
            "claude-opus-4-8".to_string(),
            1.0,
            128_000,
            AnthropicModelMode::AdaptiveThinking,
            AnthropicPromptCacheMode::Automatic,
            &ANTHROPIC_PROVIDER_ID,
        )
        .unwrap();

        assert_eq!(
            anthropic_request
                .output_config
                .and_then(|config| config.effort),
            Some(crate::Effort::XHigh)
        );
    }

    #[test]
    fn test_thinking_disallowed_sends_explicit_opt_out_only_where_required() {
        // (model, expects_explicit_opt_out): Claude Opus 5 thinks by default
        // when the `thinking` field is omitted, so suppressing thinking
        // requires sending `{"type": "disabled"}`. Earlier Opus models treat
        // omission as "off", and Fable rejects `disabled` outright, so both
        // must keep omitting the field.
        for (model, expects_explicit_opt_out) in [
            ("claude-opus-5", true),
            ("claude-opus-4-8", false),
            ("claude-fable-5", false),
        ] {
            let request = LanguageModelRequest {
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hi".to_string())],
                    cache: false,
                    reasoning_details: None,
                }],
                thread_id: None,
                prompt_id: None,
                intent: None,
                stop: vec![],
                temperature: None,
                tools: vec![],
                tool_choice: None,
                thinking_allowed: false,
                thinking_effort: None,
                speed: None,
                compact_at_tokens: None,
            };

            let anthropic_request = into_anthropic(
                request,
                model.to_string(),
                1.0,
                128_000,
                AnthropicModelMode::AdaptiveThinking,
                AnthropicPromptCacheMode::Automatic,
                &ANTHROPIC_PROVIDER_ID,
            )
            .unwrap();

            if expects_explicit_opt_out {
                assert!(
                    matches!(anthropic_request.thinking, Some(Thinking::Disabled)),
                    "{model} should send an explicit thinking opt-out"
                );
                // `disabled` combined with effort `xhigh`/`max` is a 400, so
                // no effort may accompany the opt-out.
                assert!(
                    anthropic_request.output_config.is_none(),
                    "{model} must not send output_config with thinking disabled"
                );
            } else {
                assert!(
                    anthropic_request.thinking.is_none(),
                    "{model} should omit the thinking field entirely"
                );
            }
        }
    }

    #[test]
    fn test_no_cache_control_when_caching_disabled() {
        let request = LanguageModelRequest {
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("You are helpful.".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hi".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
            ],
            thread_id: None,
            prompt_id: None,
            intent: None,
            stop: vec![],
            temperature: None,
            tools: vec![language_model_core::LanguageModelRequestTool::function(
                "do_thing".into(),
                "Does a thing.".into(),
                serde_json::json!({"type": "object"}),
                false,
            )],
            tool_choice: None,
            thinking_allowed: true,
            thinking_effort: None,
            speed: None,
            compact_at_tokens: None,
        };

        let anthropic_request = into_anthropic(
            request,
            "claude-3-5-sonnet".to_string(),
            0.7,
            4096,
            AnthropicModelMode::Default,
            AnthropicPromptCacheMode::Automatic,
            &ANTHROPIC_PROVIDER_ID,
        )
        .unwrap();

        assert!(anthropic_request.cache_control.is_none());
        assert!(matches!(
            anthropic_request.system,
            Some(StringOrContents::String(_))
        ));
        assert!(anthropic_request.tools[0].cache_control.is_none());
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
            compact_at_tokens: None,
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
            AnthropicPromptCacheMode::Automatic,
            &ANTHROPIC_PROVIDER_ID,
        )
        .unwrap()
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

    #[test]
    fn test_compact_at_tokens_maps_to_context_management() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".to_string())],
                cache: false,
                reasoning_details: None,
            }],
            compact_at_tokens: Some(100_000),
            ..Default::default()
        };

        let anthropic_request = into_anthropic(
            request,
            "claude-sonnet-4-5".to_string(),
            1.0,
            4096,
            AnthropicModelMode::Default,
            AnthropicPromptCacheMode::Disabled,
            &ANTHROPIC_PROVIDER_ID,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_value(&anthropic_request.context_management).unwrap(),
            serde_json::json!({
                "edits": [{
                    "type": "compact_20260112",
                    "trigger": { "type": "input_tokens", "value": 100_000 }
                }]
            })
        );
    }

    #[test]
    fn test_no_context_management_without_compact_at_tokens() {
        let result =
            request_with_assistant_content(vec![MessageContent::Text("Response".to_string())]);

        assert!(result.context_management.is_none());
    }

    #[test]
    fn test_compaction_content_replayed_as_compaction_block() {
        let result = request_with_assistant_content(vec![
            MessageContent::Compaction(CompactedContext::Summary {
                content: "Summary of the conversation so far.".into(),
                provider_state: None,
            }),
            MessageContent::Text("Response".to_string()),
        ]);

        let assistant_message = result
            .messages
            .iter()
            .find(|m| m.role == crate::Role::Assistant)
            .expect("assistant message should exist");

        assert_eq!(
            serde_json::to_value(&assistant_message.content[0]).unwrap(),
            serde_json::json!({
                "type": "compaction",
                "content": "Summary of the conversation so far."
            })
        );
    }

    #[test]
    fn test_compaction_encrypted_content_replayed_only_for_owning_backend() {
        let summary_owned_by = |owner: LanguageModelProviderId| {
            MessageContent::Compaction(CompactedContext::Summary {
                content: "Summary of the conversation so far.".into(),
                provider_state: Some(provider_compaction_state_from_encrypted_content(
                    owner,
                    "opaque-compaction-payload",
                )),
            })
        };

        let owned = to_anthropic_content(
            summary_owned_by(ANTHROPIC_PROVIDER_ID),
            &ANTHROPIC_PROVIDER_ID,
        )
        .unwrap()
        .expect("compaction block should be produced");
        assert_eq!(
            serde_json::to_value(&owned).unwrap(),
            serde_json::json!({
                "type": "compaction",
                "content": "Summary of the conversation so far.",
                "encrypted_content": "opaque-compaction-payload"
            })
        );

        // State produced by a different Anthropic-protocol backend must not
        // be round-tripped: the summary is still replayed, but without the
        // foreign encrypted payload.
        let foreign = to_anthropic_content(
            summary_owned_by(LanguageModelProviderId::new("other-anthropic-backend")),
            &ANTHROPIC_PROVIDER_ID,
        )
        .unwrap()
        .expect("compaction block should be produced");
        assert_eq!(
            serde_json::to_value(&foreign).unwrap(),
            serde_json::json!({
                "type": "compaction",
                "content": "Summary of the conversation so far."
            })
        );
    }

    #[test]
    fn test_event_mapper_maps_compaction_block_and_deltas() {
        let mut mapper = AnthropicEventMapper::new(ANTHROPIC_PROVIDER_NAME, ANTHROPIC_PROVIDER_ID);

        let start_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_start",
            "index": 0,
            "content_block": { "type": "compaction", "content": "Summary " }
        }))
        .unwrap();
        let delta_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_delta",
            "index": 0,
            "delta": { "type": "compaction_delta", "content": "in " }
        }))
        .unwrap();
        let second_delta_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_delta",
            "index": 0,
            "delta": { "type": "compaction_delta", "content": "chunks" }
        }))
        .unwrap();
        let stop_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_stop",
            "index": 0
        }))
        .unwrap();

        let mut events = Vec::new();
        events.extend(mapper.map_event(start_event));
        events.extend(mapper.map_event(delta_event));
        events.extend(mapper.map_event(second_delta_event));
        events.extend(mapper.map_event(stop_event));
        let events = events
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .expect("all events should map successfully");

        assert_eq!(
            events,
            vec![
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::Started),
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::SummaryDelta(
                    "Summary ".into()
                )),
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::SummaryDelta(
                    "in ".into()
                )),
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::SummaryDelta(
                    "chunks".into()
                )),
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::Finished(
                    CompactedContext::Summary {
                        content: "Summary in chunks".into(),
                        provider_state: None,
                    }
                )),
            ]
        );
    }

    /// Mirrors the stream shape in Anthropic's SDK fixtures: the block starts
    /// with both fields null, then a single delta carries the summary text
    /// alongside the opaque `encrypted_content` that must be round-tripped.
    #[test]
    fn test_event_mapper_captures_encrypted_content_as_provider_state() {
        let mut mapper = AnthropicEventMapper::new(ANTHROPIC_PROVIDER_NAME, ANTHROPIC_PROVIDER_ID);

        let start_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_start",
            "index": 0,
            "content_block": { "type": "compaction", "content": null, "encrypted_content": null }
        }))
        .unwrap();
        let delta_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_delta",
            "index": 0,
            "delta": {
                "type": "compaction_delta",
                "content": "Earlier conversation summarized.",
                "encrypted_content": "opaque-compaction-payload"
            }
        }))
        .unwrap();
        let stop_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_stop",
            "index": 0
        }))
        .unwrap();

        let mut events = Vec::new();
        events.extend(mapper.map_event(start_event));
        events.extend(mapper.map_event(delta_event));
        events.extend(mapper.map_event(stop_event));
        let mut events = events
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .expect("all events should map successfully");

        let Some(LanguageModelCompletionEvent::Compaction(CompactionUpdate::Finished(
            CompactedContext::Summary {
                content,
                provider_state: Some(state),
            },
        ))) = events.pop()
        else {
            panic!("expected a finished summary carrying provider state");
        };
        assert_eq!(content.as_ref(), "Earlier conversation summarized.");
        assert_eq!(
            provider_compaction_encrypted_content(&state, &ANTHROPIC_PROVIDER_ID)
                .unwrap()
                .as_deref(),
            Some("opaque-compaction-payload")
        );
        assert_eq!(
            provider_compaction_encrypted_content(
                &state,
                &LanguageModelProviderId::new("other-anthropic-backend")
            )
            .unwrap(),
            None
        );
    }

    /// A compaction block that closes without any content is Anthropic's
    /// documented representation of a failed compaction, which the server
    /// treats as a no-op. It must surface as `Failed` -- not as an error that
    /// would kill the rest of the response.
    #[test]
    fn test_event_mapper_maps_null_content_compaction_to_failed() {
        let mut mapper = AnthropicEventMapper::new(ANTHROPIC_PROVIDER_NAME, ANTHROPIC_PROVIDER_ID);
        let start_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_start",
            "index": 0,
            "content_block": { "type": "compaction", "content": null, "encrypted_content": null }
        }))
        .unwrap();
        let stop_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_stop",
            "index": 0
        }))
        .unwrap();

        assert_eq!(
            mapper
                .map_event(start_event)
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![LanguageModelCompletionEvent::Compaction(
                CompactionUpdate::Started
            )]
        );
        assert_eq!(
            mapper
                .map_event(stop_event)
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![LanguageModelCompletionEvent::Compaction(
                CompactionUpdate::Failed
            )]
        );
    }

    #[test]
    fn test_event_mapper_rejects_compaction_delta_before_start() {
        let mut mapper = AnthropicEventMapper::new(ANTHROPIC_PROVIDER_NAME, ANTHROPIC_PROVIDER_ID);
        let delta_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_delta",
            "index": 0,
            "delta": { "type": "compaction_delta", "content": "Summary chunk" }
        }))
        .unwrap();

        let error = mapper.map_event(delta_event).pop().unwrap().unwrap_err();

        assert!(
            error
                .to_string()
                .contains("compaction delta before starting")
        );
    }

    #[test]
    fn test_event_mapper_rejects_stream_end_with_unfinished_compaction() {
        let mut mapper = AnthropicEventMapper::new(ANTHROPIC_PROVIDER_NAME, ANTHROPIC_PROVIDER_ID);
        let start_event: Event = serde_json::from_value(serde_json::json!({
            "type": "content_block_start",
            "index": 0,
            "content_block": { "type": "compaction", "content": "Summary " }
        }))
        .unwrap();
        let stop_event: Event = serde_json::from_value(serde_json::json!({
            "type": "message_stop"
        }))
        .unwrap();

        let started = mapper
            .map_event(start_event)
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            started,
            vec![
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::Started),
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::SummaryDelta(
                    "Summary ".into()
                )),
            ]
        );

        let error = mapper.map_event(stop_event).pop().unwrap().unwrap_err();

        assert!(
            error
                .to_string()
                .contains("without finishing its compaction summary")
        );
    }

    #[test]
    fn test_usage_iterations_parsed_from_message_delta() {
        let event: Event = serde_json::from_value(serde_json::json!({
            "type": "message_delta",
            "delta": { "stop_reason": "end_turn", "stop_sequence": null },
            "usage": {
                "input_tokens": 100,
                "output_tokens": 39,
                "iterations": [
                    { "type": "compaction", "input_tokens": 180000, "output_tokens": 1200 },
                    { "type": "message", "input_tokens": 100, "output_tokens": 39 }
                ]
            }
        }))
        .unwrap();

        let Event::MessageDelta { usage, .. } = event else {
            panic!("expected message_delta event");
        };
        let iterations = usage.iterations.as_deref().expect("iterations expected");
        assert!(matches!(
            iterations[0],
            UsageIteration {
                iteration_type: UsageIterationType::Compaction,
                input_tokens: Some(180000),
                ..
            }
        ));
        assert!(matches!(
            iterations[1],
            UsageIteration {
                iteration_type: UsageIterationType::Message,
                input_tokens: Some(100),
                ..
            }
        ));
    }
}
