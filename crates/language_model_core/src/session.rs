//! An append-only conversation log that creates byte-stable requests.
//!
//! Preserved thinking and prompt caching require each request to extend what
//! was already sent. [`SessionLog`] owns the conversation: hosts may only
//! append user input and tool results, the log records assistant output
//! exactly as it streamed (signatures, redacted thinking, reasoning details
//! included), and [`SessionLog::create_request`] is a pure function of it.

use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    CompletionIntent, LanguageModelCompletionEvent, LanguageModelId, LanguageModelImage,
    LanguageModelProviderId, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelRequestTool, LanguageModelToolResult, LanguageModelToolUse,
    LanguageModelToolUseId, LanguageModelToolUseInput, MessageContent, Role, Speed, StopReason,
};

/// The prefix half of a request, frozen when the session opens.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Copied into every request's [`LanguageModelRequest::thread_id`].
    pub thread_id: Option<String>,
    /// Copied into every request's [`LanguageModelRequest::prompt_cache_key`].
    pub prompt_cache_key: Option<String>,
    /// `None` omits the system message.
    pub system_prompt: Option<String>,
    /// Sent in this order.
    pub tools: Vec<LanguageModelRequestTool>,
}

/// Request parameters outside the cached prefix, chosen per round.
///
/// Each field is copied into the [`LanguageModelRequest`] field of the same
/// name. `tool_choice`, `stop`, `compact_at_tokens`, and `max_output_tokens` always
/// take their defaults.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoundParameters {
    pub intent: Option<CompletionIntent>,
    pub prompt_id: Option<String>,
    pub temperature: Option<f32>,
    pub thinking_allowed: bool,
    pub thinking_effort: Option<String>,
    pub speed: Option<Speed>,
}

/// One part of a user message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UserContent {
    Text(String),
    Image(LanguageModelImage),
}

/// Input a host appends to a session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SessionInput {
    UserMessage(Vec<UserContent>),
}

/// Identifies an appended input, for [`SessionLog::truncate`].
///
/// Anchors are never reused within a log, so an anchor removed by truncation
/// stays invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionAnchor(u64);

impl fmt::Display for SessionAnchor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// One entry of a [`SessionLog`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SessionLogEntry {
    Input {
        anchor: SessionAnchor,
        input: SessionInput,
    },
    /// Answers a tool use in the nearest preceding `Output`.
    ToolResult(LanguageModelToolResult),
    /// One model round, folded as it streamed.
    Output(AssistantOutput),
}

/// Everything the model produced in one round.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantOutput {
    /// The provider that produced this output.
    pub provider: LanguageModelProviderId,
    /// The model that produced this output.
    pub model: LanguageModelId,
    /// One per `StartMessage` boundary.
    pub messages: Vec<AssistantMessage>,
    /// `None` while streaming, or when the round was cancelled or failed.
    pub stop_reason: Option<StopReason>,
}

/// One assistant message within a round.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantMessage {
    /// Only `Text`, `Thinking`, `RedactedThinking`, and `ToolUse`.
    pub content: Vec<MessageContent>,
    /// The last non-empty reasoning details streamed for this message.
    pub reasoning_details: Option<Arc<serde_json::Value>>,
}

/// A session operation that the log's current state doesn't allow.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum SessionError {
    #[error("a completion round is already in progress")]
    RoundInProgress,
    #[error("the session has no user input or tool result awaiting a response")]
    NoPendingInput,
    #[error("no input in the session log has anchor {0}")]
    UnknownAnchor(SessionAnchor),
    #[error("no unanswered tool use with id {0} in the latest assistant output")]
    UnmatchedToolResult(LanguageModelToolUseId),
}

/// The conversation a session sends, as an append-only log.
///
/// The log implements `Serialize` and `Deserialize` so hosts can persist it
/// in a format of their choosing; opaque provider data such as signatures and
/// reasoning details is only reachable through it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionLog {
    config: SessionConfig,
    entries: Vec<SessionLogEntry>,
    next_anchor: u64,
}

impl SessionLog {
    /// An empty log whose requests use `config`.
    pub fn new(config: SessionConfig) -> Self {
        Self {
            config,
            entries: Vec::new(),
            next_anchor: 0,
        }
    }

    /// The configuration the log was created with.
    pub fn config(&self) -> &SessionConfig {
        &self.config
    }

    /// Every entry, oldest first.
    pub fn entries(&self) -> &[SessionLogEntry] {
        &self.entries
    }

    /// Appends `input`, returning an anchor that can later truncate it.
    pub fn append(&mut self, input: SessionInput) -> SessionAnchor {
        let anchor = SessionAnchor(self.next_anchor);
        self.next_anchor += 1;
        self.entries.push(SessionLogEntry::Input { anchor, input });
        anchor
    }

    /// Appends the result of an unanswered tool use in the latest output.
    ///
    /// The result's `output` is cleared: no provider reads it, and it would
    /// bloat the log.
    ///
    /// # Errors
    ///
    /// [`SessionError::UnmatchedToolResult`] unless the latest output has an
    /// unanswered tool use with this id and no input has been appended since.
    pub fn submit_tool_result(
        &mut self,
        mut result: LanguageModelToolResult,
    ) -> Result<(), SessionError> {
        let unmatched = || SessionError::UnmatchedToolResult(result.tool_use_id.clone());
        let output_index = self
            .entries
            .iter()
            .rposition(|entry| matches!(entry, SessionLogEntry::Output(_)))
            .ok_or_else(unmatched)?;
        let SessionLogEntry::Output(output) = &self.entries[output_index] else {
            return Err(unmatched());
        };
        let has_tool_use = output
            .tool_uses()
            .any(|tool_use| tool_use.id == result.tool_use_id);
        let mut later_entries = self.entries[output_index + 1..].iter();
        let already_answered_or_interrupted = later_entries.any(|entry| match entry {
            SessionLogEntry::ToolResult(existing) => existing.tool_use_id == result.tool_use_id,
            SessionLogEntry::Input { .. } | SessionLogEntry::Output(_) => true,
        });
        if !has_tool_use || already_answered_or_interrupted {
            return Err(unmatched());
        }

        result.output = None;
        self.entries.push(SessionLogEntry::ToolResult(result));
        Ok(())
    }

    /// Removes the anchored input and everything after it.
    ///
    /// # Errors
    ///
    /// [`SessionError::UnknownAnchor`] if no input in the log has `anchor`.
    pub fn truncate(&mut self, anchor: SessionAnchor) -> Result<(), SessionError> {
        let index = self
            .entries
            .iter()
            .position(|entry| {
                matches!(entry, SessionLogEntry::Input { anchor: existing, .. } if *existing == anchor)
            })
            .ok_or(SessionError::UnknownAnchor(anchor))?;
        self.entries.truncate(index);
        Ok(())
    }

    /// Starts a round by pushing an empty output for `model`.
    ///
    /// # Errors
    ///
    /// [`SessionError::NoPendingInput`] unless the log ends in an input or a
    /// tool result.
    pub fn begin_round(
        &mut self,
        provider: LanguageModelProviderId,
        model: LanguageModelId,
    ) -> Result<(), SessionError> {
        match self.entries.last() {
            Some(SessionLogEntry::Input { .. } | SessionLogEntry::ToolResult(_)) => {}
            Some(SessionLogEntry::Output(_)) | None => return Err(SessionError::NoPendingInput),
        }
        self.entries.push(SessionLogEntry::Output(AssistantOutput {
            provider,
            model,
            messages: Vec::new(),
            stop_reason: None,
        }));
        Ok(())
    }

    /// Folds a streamed event into the latest output.
    pub fn record_output(&mut self, event: &LanguageModelCompletionEvent) {
        let output = self.entries.iter_mut().rev().find_map(|entry| match entry {
            SessionLogEntry::Output(output) => Some(output),
            _ => None,
        });
        if let Some(output) = output {
            output.record(event);
        }
    }

    /// Ends a round, dropping its output if it recorded nothing.
    ///
    /// An output with a stop reason is kept even without content, since a
    /// refusal or an empty end turn is still the model's answer. Without a
    /// stop reason (cancellation or failure), an output whose messages have no
    /// content is dropped, even if it has reasoning details.
    pub fn end_round(&mut self) {
        if let Some(SessionLogEntry::Output(output)) = self.entries.last()
            && output.stop_reason.is_none()
            && output
                .messages
                .iter()
                .all(|message| message.content.is_empty())
        {
            self.entries.pop();
        }
    }

    /// Creates the request for the next round.
    ///
    /// Each assistant message is followed by a user message holding the
    /// results of its tool uses. Tool uses without a result are omitted, since
    /// providers reject them; the rest of their output is kept.
    ///
    /// The system message, the last message, and the previous user message
    /// are marked for caching. Marking more than the last message is
    /// deliberate: the previous user message lets this request read the cache
    /// entry the previous round wrote.
    pub fn create_request(&self, parameters: &RoundParameters) -> LanguageModelRequest {
        let mut messages = Vec::new();
        if let Some(system_prompt) = &self.config.system_prompt {
            messages.push(LanguageModelRequestMessage {
                role: Role::System,
                content: vec![MessageContent::Text(system_prompt.clone())],
                cache: true,
                reasoning_details: None,
            });
        }

        for (index, entry) in self.entries.iter().enumerate() {
            match entry {
                SessionLogEntry::Input {
                    input: SessionInput::UserMessage(content),
                    ..
                } => {
                    if content.is_empty() {
                        continue;
                    }
                    messages.push(LanguageModelRequestMessage {
                        role: Role::User,
                        content: content
                            .iter()
                            .map(|content| match content {
                                UserContent::Text(text) => MessageContent::Text(text.clone()),
                                UserContent::Image(image) => MessageContent::Image(image.clone()),
                            })
                            .collect(),
                        cache: false,
                        reasoning_details: None,
                    });
                }
                SessionLogEntry::Output(output) => {
                    let results: Vec<&LanguageModelToolResult> = self.entries[index + 1..]
                        .iter()
                        .map_while(|entry| match entry {
                            SessionLogEntry::ToolResult(result) => Some(result),
                            _ => None,
                        })
                        .collect();
                    for message in &output.messages {
                        push_assistant_message(message, &results, &mut messages);
                    }
                }
                // Sent after the assistant message whose tool use it answers.
                SessionLogEntry::ToolResult(_) => {}
            }
        }

        if let Some((last, earlier)) = messages.split_last_mut() {
            last.cache = true;
            if let Some(previous_user) = earlier
                .iter_mut()
                .rev()
                .find(|message| message.role == Role::User)
            {
                previous_user.cache = true;
            }
        }

        LanguageModelRequest {
            thread_id: self.config.thread_id.clone(),
            prompt_cache_key: self.config.prompt_cache_key.clone(),
            prompt_id: parameters.prompt_id.clone(),
            intent: parameters.intent,
            messages,
            tools: self.config.tools.clone(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: parameters.temperature,
            thinking_allowed: parameters.thinking_allowed,
            thinking_effort: parameters.thinking_effort.clone(),
            speed: parameters.speed,
            compact_at_tokens: None,
            max_output_tokens: None,
        }
    }
}

/// Pushes `message` without its unanswered tool uses, then a user message
/// with the results that answer its tool uses, in submission order.
fn push_assistant_message(
    message: &AssistantMessage,
    results: &[&LanguageModelToolResult],
    messages: &mut Vec<LanguageModelRequestMessage>,
) {
    let tool_use_ids: Vec<&LanguageModelToolUseId> = message
        .content
        .iter()
        .filter_map(|content| match content {
            MessageContent::ToolUse(tool_use) => Some(&tool_use.id),
            _ => None,
        })
        .collect();
    let own_results: Vec<&LanguageModelToolResult> = results
        .iter()
        .copied()
        .filter(|result| tool_use_ids.contains(&&result.tool_use_id))
        .collect();

    let content: Vec<MessageContent> = message
        .content
        .iter()
        .filter(|content| match content {
            MessageContent::ToolUse(tool_use) => own_results
                .iter()
                .any(|result| result.tool_use_id == tool_use.id),
            _ => true,
        })
        .cloned()
        .collect();
    if content.is_empty() {
        return;
    }
    messages.push(LanguageModelRequestMessage {
        role: Role::Assistant,
        content,
        cache: false,
        reasoning_details: message.reasoning_details.clone(),
    });

    if own_results.is_empty() {
        return;
    }
    messages.push(LanguageModelRequestMessage {
        role: Role::User,
        content: own_results
            .into_iter()
            .map(|result| {
                let mut result = result.clone();
                // Providers treat an empty result as a missing one.
                if result.is_content_empty() {
                    result.content = vec!["<Tool returned an empty string>".into()];
                }
                MessageContent::ToolResult(result)
            })
            .collect(),
        cache: false,
        reasoning_details: None,
    });
}

impl AssistantOutput {
    fn tool_uses(&self) -> impl Iterator<Item = &LanguageModelToolUse> {
        self.messages
            .iter()
            .flat_map(|message| &message.content)
            .filter_map(|content| match content {
                MessageContent::ToolUse(tool_use) => Some(tool_use),
                _ => None,
            })
    }

    fn record(&mut self, event: &LanguageModelCompletionEvent) {
        match event {
            LanguageModelCompletionEvent::Text(new_text) => {
                let content = &mut self.current_message().content;
                if let Some(MessageContent::Text(text)) = content.last_mut() {
                    text.push_str(new_text);
                } else {
                    content.push(MessageContent::Text(new_text.clone()));
                }
            }
            LanguageModelCompletionEvent::Thinking {
                text: new_text,
                signature: new_signature,
            } => {
                let content = &mut self.current_message().content;
                // A signature seals its block, so thinking after it is a new block.
                if let Some(MessageContent::Thinking {
                    text,
                    signature: signature @ None,
                }) = content.last_mut()
                {
                    text.push_str(new_text);
                    *signature = new_signature.clone();
                } else {
                    content.push(MessageContent::Thinking {
                        text: new_text.clone(),
                        signature: new_signature.clone(),
                    });
                }
            }
            LanguageModelCompletionEvent::RedactedThinking { data } => {
                self.current_message()
                    .content
                    .push(MessageContent::RedactedThinking(data.clone()));
            }
            LanguageModelCompletionEvent::ToolUse(tool_use) => {
                self.record_tool_use(tool_use.clone());
            }
            LanguageModelCompletionEvent::ToolUseJsonParseError {
                id,
                tool_name,
                raw_input,
                ..
            } => {
                self.record_tool_use(LanguageModelToolUse {
                    id: id.clone(),
                    name: tool_name.clone(),
                    raw_input: raw_input.to_string(),
                    input: LanguageModelToolUseInput::Json(serde_json::json!({})),
                    is_input_complete: true,
                    thought_signature: None,
                });
            }
            LanguageModelCompletionEvent::ReasoningDetails(details) => {
                // Early chunks can be empty arrays; the later encrypted details
                // are the ones the provider needs back.
                if !matches!(details, serde_json::Value::Array(array) if array.is_empty()) {
                    self.current_message().reasoning_details = Some(Arc::new(details.clone()));
                }
            }
            LanguageModelCompletionEvent::StartMessage { .. } => {
                if self
                    .messages
                    .last()
                    .is_some_and(|message| *message != AssistantMessage::default())
                {
                    self.messages.push(AssistantMessage::default());
                }
            }
            LanguageModelCompletionEvent::Stop(stop_reason) => {
                self.stop_reason = Some(*stop_reason);
            }
            LanguageModelCompletionEvent::Queued { .. }
            | LanguageModelCompletionEvent::Started
            | LanguageModelCompletionEvent::UsageUpdate(_)
            | LanguageModelCompletionEvent::Compaction(_) => {}
        }
    }

    /// Replaces the tool use with the same id, since partial tool uses are
    /// cumulative, keeping a signature that only an earlier partial carried.
    fn record_tool_use(&mut self, mut tool_use: LanguageModelToolUse) {
        let content = &mut self.current_message().content;
        let existing = content.iter_mut().rev().find_map(|content| match content {
            MessageContent::ToolUse(existing) if existing.id == tool_use.id => Some(existing),
            _ => None,
        });
        match existing {
            Some(existing) => {
                if tool_use.thought_signature.is_none() {
                    tool_use.thought_signature = existing.thought_signature.take();
                }
                *existing = tool_use;
            }
            None => content.push(MessageContent::ToolUse(tool_use)),
        }
    }

    fn current_message(&mut self) -> &mut AssistantMessage {
        if self.messages.is_empty() {
            self.messages.push(AssistantMessage::default());
        }
        let last_index = self.messages.len() - 1;
        &mut self.messages[last_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompactionUpdate, LanguageModelToolResultContent, TokenUsage};
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn create_request_matches_expected_request() {
        let log = full_log();
        let parameters = RoundParameters {
            intent: Some(CompletionIntent::ToolResults),
            prompt_id: Some("prompt".into()),
            temperature: Some(0.5),
            thinking_allowed: true,
            thinking_effort: Some("high".into()),
            speed: Some(Speed::Fast),
        };

        assert_eq!(
            log.create_request(&parameters),
            LanguageModelRequest {
                thread_id: Some("thread".into()),
                prompt_cache_key: Some("cache-key".into()),
                prompt_id: Some("prompt".into()),
                intent: Some(CompletionIntent::ToolResults),
                messages: vec![
                    LanguageModelRequestMessage {
                        role: Role::System,
                        content: vec![MessageContent::Text("You are helpful.".into())],
                        cache: true,
                        reasoning_details: None,
                    },
                    LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec![
                            MessageContent::Text("Look at this".into()),
                            MessageContent::Image(image()),
                        ],
                        cache: true,
                        reasoning_details: None,
                    },
                    LanguageModelRequestMessage {
                        role: Role::Assistant,
                        content: vec![
                            MessageContent::Thinking {
                                text: "Let me look".into(),
                                signature: Some("signature".into()),
                            },
                            MessageContent::RedactedThinking("opaque".into()),
                            MessageContent::Text("Reading the file.".into()),
                            MessageContent::ToolUse(tool_use("tool-1", true)),
                        ],
                        cache: false,
                        reasoning_details: Some(Arc::new(json!([{"type": "encrypted"}]))),
                    },
                    LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec![MessageContent::ToolResult(LanguageModelToolResult {
                            output: None,
                            ..tool_result("tool-1", "file contents")
                        })],
                        cache: true,
                        reasoning_details: None,
                    },
                ],
                tools: vec![read_tool()],
                tool_choice: None,
                stop: Vec::new(),
                temperature: Some(0.5),
                thinking_allowed: true,
                thinking_effort: Some("high".into()),
                speed: Some(Speed::Fast),
                compact_at_tokens: None,
                max_output_tokens: None,
            }
        );
    }

    #[test]
    fn unpaired_tool_use_is_omitted_but_text_is_kept() {
        let mut log = SessionLog::new(config_without_system_prompt());
        log.append(user_text("Read the file"));
        run_round(
            &mut log,
            &[
                LanguageModelCompletionEvent::Text("I'll read it.".into()),
                LanguageModelCompletionEvent::ToolUse(tool_use("tool-1", true)),
            ],
        );
        log.append(user_text("Never mind"));

        assert_eq!(
            log.create_request(&RoundParameters::default()).messages,
            vec![
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Read the file".into())],
                    cache: true,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::Text("I'll read it.".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Never mind".into())],
                    cache: true,
                    reasoning_details: None,
                },
            ]
        );
    }

    #[test]
    fn tool_results_follow_the_message_that_owns_them() {
        let mut log = SessionLog::new(config_without_system_prompt());
        log.append(user_text("Read both files"));
        run_round(
            &mut log,
            &[
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "message-1".into(),
                },
                thinking("Start with a", Some("signature-1")),
                LanguageModelCompletionEvent::Text("Reading a.".into()),
                LanguageModelCompletionEvent::ToolUse(tool_use("tool-a", true)),
                LanguageModelCompletionEvent::ReasoningDetails(json!([{"id": "reasoning-1"}])),
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "message-2".into(),
                },
                thinking("Now b", Some("signature-2")),
                LanguageModelCompletionEvent::ToolUse(tool_use("tool-b", true)),
                LanguageModelCompletionEvent::Stop(StopReason::ToolUse),
            ],
        );
        // Submitted out of order: each result still follows its own message.
        log.submit_tool_result(tool_result("tool-b", "contents of b"))
            .unwrap();
        log.submit_tool_result(tool_result("tool-a", "contents of a"))
            .unwrap();

        assert_eq!(
            log.create_request(&RoundParameters::default()).messages,
            vec![
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Read both files".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![
                        MessageContent::Thinking {
                            text: "Start with a".into(),
                            signature: Some("signature-1".into()),
                        },
                        MessageContent::Text("Reading a.".into()),
                        MessageContent::ToolUse(tool_use("tool-a", true)),
                    ],
                    cache: false,
                    reasoning_details: Some(Arc::new(json!([{"id": "reasoning-1"}]))),
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::ToolResult(LanguageModelToolResult {
                        output: None,
                        ..tool_result("tool-a", "contents of a")
                    })],
                    // The previous user message.
                    cache: true,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![
                        MessageContent::Thinking {
                            text: "Now b".into(),
                            signature: Some("signature-2".into()),
                        },
                        MessageContent::ToolUse(tool_use("tool-b", true)),
                    ],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::ToolResult(LanguageModelToolResult {
                        output: None,
                        ..tool_result("tool-b", "contents of b")
                    })],
                    cache: true,
                    reasoning_details: None,
                },
            ]
        );
    }

    #[test]
    fn empty_tool_result_sends_placeholder() {
        let mut log = SessionLog::new(config_without_system_prompt());
        log.append(user_text("Run it"));
        run_round(
            &mut log,
            &[LanguageModelCompletionEvent::ToolUse(tool_use(
                "tool-1", true,
            ))],
        );
        log.submit_tool_result(LanguageModelToolResult {
            content: Vec::new(),
            ..tool_result("tool-1", "")
        })
        .unwrap();

        let request = log.create_request(&RoundParameters::default());
        let Some(MessageContent::ToolResult(result)) = request
            .messages
            .last()
            .and_then(|message| message.content.first())
        else {
            panic!(
                "expected a trailing tool result, got {:?}",
                request.messages
            );
        };
        assert_eq!(
            result.content,
            vec![LanguageModelToolResultContent::Text(
                "<Tool returned an empty string>".into()
            )]
        );
    }

    #[test]
    fn text_chunks_merge() {
        assert_eq!(
            fold(&[
                LanguageModelCompletionEvent::Text("Hel".into()),
                LanguageModelCompletionEvent::Text("lo".into()),
            ])
            .messages,
            vec![message(vec![MessageContent::Text("Hello".into())])]
        );
    }

    #[test]
    fn thinking_chunks_merge_and_signature_attaches() {
        assert_eq!(
            fold(&[
                thinking("Let me", None),
                thinking(" think", Some("signature")),
            ])
            .messages,
            vec![message(vec![MessageContent::Thinking {
                text: "Let me think".into(),
                signature: Some("signature".into()),
            }])]
        );
    }

    #[test]
    fn thinking_after_signed_thinking_starts_a_new_block() {
        assert_eq!(
            fold(&[
                thinking("first", Some("signature-1")),
                thinking("second", None),
            ])
            .messages,
            vec![message(vec![
                MessageContent::Thinking {
                    text: "first".into(),
                    signature: Some("signature-1".into()),
                },
                MessageContent::Thinking {
                    text: "second".into(),
                    signature: None,
                },
            ])]
        );
    }

    #[test]
    fn partial_tool_uses_replace_by_id_and_keep_earlier_thought_signature() {
        let partial = LanguageModelToolUse {
            raw_input: r#"{"path": "sr"#.into(),
            input: LanguageModelToolUseInput::Json(json!({"path": "sr"})),
            is_input_complete: false,
            thought_signature: Some("thought".into()),
            ..tool_use("tool-1", false)
        };
        let complete = LanguageModelToolUse {
            thought_signature: None,
            ..tool_use("tool-1", true)
        };

        assert_eq!(
            fold(&[
                LanguageModelCompletionEvent::ToolUse(partial),
                LanguageModelCompletionEvent::ToolUse(complete.clone()),
            ])
            .messages,
            vec![message(vec![MessageContent::ToolUse(
                LanguageModelToolUse {
                    thought_signature: Some("thought".into()),
                    ..complete
                }
            )])]
        );
    }

    #[test]
    fn json_parse_error_becomes_empty_object_tool_use() {
        assert_eq!(
            fold(&[LanguageModelCompletionEvent::ToolUseJsonParseError {
                id: "tool-1".into(),
                tool_name: "read".into(),
                raw_input: "{not json".into(),
                json_parse_error: "expected value".into(),
            }])
            .messages,
            vec![message(vec![MessageContent::ToolUse(
                LanguageModelToolUse {
                    id: "tool-1".into(),
                    name: "read".into(),
                    raw_input: "{not json".into(),
                    input: LanguageModelToolUseInput::Json(json!({})),
                    is_input_complete: true,
                    thought_signature: None,
                }
            )])]
        );
    }

    #[test]
    fn last_non_empty_reasoning_details_win() {
        let output = fold(&[
            LanguageModelCompletionEvent::ReasoningDetails(json!([{"text": "draft"}])),
            LanguageModelCompletionEvent::ReasoningDetails(json!([{"encrypted": "final"}])),
            LanguageModelCompletionEvent::ReasoningDetails(json!([])),
        ]);

        assert_eq!(
            output.messages[0].reasoning_details,
            Some(Arc::new(json!([{"encrypted": "final"}])))
        );
    }

    #[test]
    fn start_message_splits_messages_with_their_own_reasoning_details() {
        let output = fold(&[
            LanguageModelCompletionEvent::StartMessage {
                message_id: "one".into(),
            },
            LanguageModelCompletionEvent::Text("first".into()),
            LanguageModelCompletionEvent::ReasoningDetails(json!(["one"])),
            LanguageModelCompletionEvent::StartMessage {
                message_id: "two".into(),
            },
            LanguageModelCompletionEvent::Text("second".into()),
            LanguageModelCompletionEvent::ReasoningDetails(json!(["two"])),
        ]);

        assert_eq!(
            output.messages,
            vec![
                AssistantMessage {
                    content: vec![MessageContent::Text("first".into())],
                    reasoning_details: Some(Arc::new(json!(["one"]))),
                },
                AssistantMessage {
                    content: vec![MessageContent::Text("second".into())],
                    reasoning_details: Some(Arc::new(json!(["two"]))),
                },
            ]
        );
    }

    #[test]
    fn stop_sets_stop_reason() {
        let output = fold(&[
            LanguageModelCompletionEvent::Text("done".into()),
            LanguageModelCompletionEvent::Stop(StopReason::EndTurn),
        ]);

        assert_eq!(output.stop_reason, Some(StopReason::EndTurn));
    }

    #[test]
    fn ignored_events_leave_the_output_unchanged() {
        let text = LanguageModelCompletionEvent::Text("hello".into());

        assert_eq!(
            fold(&[
                LanguageModelCompletionEvent::Queued { position: 1 },
                LanguageModelCompletionEvent::Started,
                text.clone(),
                LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 10,
                    ..TokenUsage::default()
                }),
                LanguageModelCompletionEvent::Compaction(CompactionUpdate::Started),
            ]),
            fold(&[text])
        );
    }

    #[test]
    fn begin_round_requires_user_role_tail() {
        let mut log = SessionLog::new(config_without_system_prompt());
        assert_eq!(
            log.begin_round(provider(), model()),
            Err(SessionError::NoPendingInput)
        );

        log.append(user_text("Read the file"));
        run_round(
            &mut log,
            &[LanguageModelCompletionEvent::ToolUse(tool_use(
                "tool-1", true,
            ))],
        );
        assert_eq!(
            log.begin_round(provider(), model()),
            Err(SessionError::NoPendingInput)
        );

        log.submit_tool_result(tool_result("tool-1", "contents"))
            .unwrap();
        assert_eq!(log.begin_round(provider(), model()), Ok(()));
    }

    #[test]
    fn completed_round_without_content_keeps_its_output() {
        for stop_reason in [
            StopReason::EndTurn,
            StopReason::Refusal,
            StopReason::MaxTokens,
        ] {
            let mut log = SessionLog::new(config_without_system_prompt());
            log.append(user_text("Hello"));
            let before = log.clone();

            run_round(
                &mut log,
                &[
                    LanguageModelCompletionEvent::Started,
                    LanguageModelCompletionEvent::Stop(stop_reason),
                ],
            );

            assert_eq!(
                log.entries()[1..],
                [SessionLogEntry::Output(AssistantOutput {
                    provider: provider(),
                    model: model(),
                    messages: Vec::new(),
                    stop_reason: Some(stop_reason),
                })]
            );
            assert_eq!(
                log.create_request(&RoundParameters::default()),
                before.create_request(&RoundParameters::default())
            );
            assert_eq!(
                log.begin_round(provider(), model()),
                Err(SessionError::NoPendingInput)
            );
        }
    }

    #[test]
    fn interrupted_round_without_content_leaves_no_output_entry() {
        let mut log = SessionLog::new(config_without_system_prompt());
        log.append(user_text("Hello"));
        let before = log.clone();

        // Reasoning details alone don't count as content, and there is no
        // stop reason because the stream errored or was cancelled.
        run_round(
            &mut log,
            &[
                LanguageModelCompletionEvent::Started,
                LanguageModelCompletionEvent::ReasoningDetails(json!([{"type": "encrypted"}])),
            ],
        );

        assert_eq!(log, before);
        assert_eq!(log.begin_round(provider(), model()), Ok(()));
    }

    #[test]
    fn tool_result_during_round_folds_later_events_into_the_round_output() {
        let mut log = SessionLog::new(config_without_system_prompt());
        log.append(user_text("Read the file"));
        log.begin_round(provider(), model()).unwrap();
        log.record_output(&LanguageModelCompletionEvent::ToolUse(tool_use(
            "tool-1", true,
        )));
        log.submit_tool_result(tool_result("tool-1", "contents"))
            .unwrap();
        log.record_output(&LanguageModelCompletionEvent::Text("Done.".into()));
        log.end_round();

        assert_eq!(
            log.entries()[1..],
            [
                SessionLogEntry::Output(AssistantOutput {
                    provider: provider(),
                    model: model(),
                    messages: vec![message(vec![
                        MessageContent::ToolUse(tool_use("tool-1", true)),
                        MessageContent::Text("Done.".into()),
                    ])],
                    stop_reason: None,
                }),
                SessionLogEntry::ToolResult(LanguageModelToolResult {
                    output: None,
                    ..tool_result("tool-1", "contents")
                }),
            ]
        );
    }

    #[test]
    fn submit_tool_result_rejects_unknown_and_duplicate_ids() {
        let mut log = SessionLog::new(config_without_system_prompt());
        log.append(user_text("Read the file"));
        assert_eq!(
            log.submit_tool_result(tool_result("tool-1", "early")),
            Err(SessionError::UnmatchedToolResult("tool-1".into()))
        );

        run_round(
            &mut log,
            &[LanguageModelCompletionEvent::ToolUse(tool_use(
                "tool-1", true,
            ))],
        );
        assert_eq!(
            log.submit_tool_result(tool_result("tool-2", "unknown")),
            Err(SessionError::UnmatchedToolResult("tool-2".into()))
        );
        assert_eq!(
            log.submit_tool_result(tool_result("tool-1", "contents")),
            Ok(())
        );
        assert_eq!(
            log.submit_tool_result(tool_result("tool-1", "again")),
            Err(SessionError::UnmatchedToolResult("tool-1".into()))
        );
    }

    #[test]
    fn truncate_removes_anchor_and_suffix() {
        let mut log = SessionLog::new(config_without_system_prompt());
        log.append(user_text("First"));
        run_round(
            &mut log,
            &[LanguageModelCompletionEvent::Text("One".into())],
        );
        let kept = log.clone();
        let second = log.append(user_text("Second"));
        run_round(
            &mut log,
            &[LanguageModelCompletionEvent::Text("Two".into())],
        );

        log.truncate(second).unwrap();

        assert_eq!(log.entries(), kept.entries());
    }

    #[test]
    fn truncate_rejects_stale_anchor_after_reappend() {
        let mut log = SessionLog::new(config_without_system_prompt());
        let stale = log.append(user_text("Original"));
        log.truncate(stale).unwrap();
        let replacement = log.append(user_text("Edited"));

        assert_ne!(stale, replacement);
        assert_eq!(log.truncate(stale), Err(SessionError::UnknownAnchor(stale)));
        assert_eq!(log.entries().len(), 1);
    }

    /// A system prompt, a tool, an image, every assistant block kind, and an
    /// answered tool use.
    fn full_log() -> SessionLog {
        let mut log = SessionLog::new(SessionConfig {
            thread_id: Some("thread".into()),
            prompt_cache_key: Some("cache-key".into()),
            system_prompt: Some("You are helpful.".into()),
            tools: vec![read_tool()],
        });
        log.append(SessionInput::UserMessage(vec![
            UserContent::Text("Look at this".into()),
            UserContent::Image(image()),
        ]));
        run_round(
            &mut log,
            &[
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "message-1".into(),
                },
                thinking("Let me", None),
                thinking(" look", Some("signature")),
                LanguageModelCompletionEvent::RedactedThinking {
                    data: "opaque".into(),
                },
                LanguageModelCompletionEvent::Text("Reading the file.".into()),
                LanguageModelCompletionEvent::ToolUse(tool_use("tool-1", true)),
                LanguageModelCompletionEvent::ReasoningDetails(json!([{"type": "encrypted"}])),
                LanguageModelCompletionEvent::Stop(StopReason::ToolUse),
            ],
        );
        log.submit_tool_result(tool_result("tool-1", "file contents"))
            .unwrap();
        log
    }

    fn fold(events: &[LanguageModelCompletionEvent]) -> AssistantOutput {
        let mut log = SessionLog::new(config_without_system_prompt());
        log.append(user_text("Hello"));
        log.begin_round(provider(), model()).unwrap();
        for event in events {
            log.record_output(event);
        }
        match log.entries().last() {
            Some(SessionLogEntry::Output(output)) => output.clone(),
            entry => panic!("expected an output, got {entry:?}"),
        }
    }

    fn run_round(log: &mut SessionLog, events: &[LanguageModelCompletionEvent]) {
        log.begin_round(provider(), model()).unwrap();
        for event in events {
            log.record_output(event);
        }
        log.end_round();
    }

    fn config_without_system_prompt() -> SessionConfig {
        SessionConfig {
            thread_id: None,
            prompt_cache_key: None,
            system_prompt: None,
            tools: Vec::new(),
        }
    }

    fn provider() -> LanguageModelProviderId {
        LanguageModelProviderId::from("anthropic".to_string())
    }

    fn model() -> LanguageModelId {
        LanguageModelId::from("claude".to_string())
    }

    fn user_text(text: &str) -> SessionInput {
        SessionInput::UserMessage(vec![UserContent::Text(text.into())])
    }

    fn message(content: Vec<MessageContent>) -> AssistantMessage {
        AssistantMessage {
            content,
            reasoning_details: None,
        }
    }

    fn thinking(text: &str, signature: Option<&str>) -> LanguageModelCompletionEvent {
        LanguageModelCompletionEvent::Thinking {
            text: text.into(),
            signature: signature.map(Into::into),
        }
    }

    fn tool_use(id: &str, is_input_complete: bool) -> LanguageModelToolUse {
        LanguageModelToolUse {
            id: id.into(),
            name: "read".into(),
            raw_input: r#"{"path": "src/main.rs"}"#.into(),
            input: LanguageModelToolUseInput::Json(json!({"path": "src/main.rs"})),
            is_input_complete,
            thought_signature: None,
        }
    }

    fn tool_result(id: &str, text: &str) -> LanguageModelToolResult {
        LanguageModelToolResult {
            tool_use_id: id.into(),
            tool_name: "read".into(),
            is_error: false,
            content: vec![LanguageModelToolResultContent::Text(text.into())],
            output: Some(json!({"debug": "state"})),
        }
    }

    fn read_tool() -> LanguageModelRequestTool {
        LanguageModelRequestTool::function(
            "read".into(),
            "Reads a file".into(),
            json!({"type": "object"}),
            false,
        )
    }

    fn image() -> LanguageModelImage {
        LanguageModelImage {
            source: "aW1hZ2U=".into(),
        }
    }
}
