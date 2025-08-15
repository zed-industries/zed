use crate::{AgentMessage, AgentMessageContent, UserMessage, UserMessageContent};
use agent::thread_store;
use agent_settings::{AgentProfileId, CompletionMode};
use anyhow::Result;
use chrono::{DateTime, Utc};
use collections::{HashMap, IndexMap};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::SharedString;

pub type DbMessage = crate::Message;
pub type DbSummary = agent::thread::DetailedSummaryState;
pub type DbLanguageModel = thread_store::SerializedLanguageModel;
pub type DbThreadMetadata = thread_store::SerializedThreadMetadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbThread {
    pub title: SharedString,
    pub messages: Vec<DbMessage>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub summary: DbSummary,
    #[serde(default)]
    pub initial_project_snapshot: Option<Arc<agent::thread::ProjectSnapshot>>,
    #[serde(default)]
    pub cumulative_token_usage: language_model::TokenUsage,
    #[serde(default)]
    pub request_token_usage: Vec<language_model::TokenUsage>,
    #[serde(default)]
    pub model: Option<DbLanguageModel>,
    #[serde(default)]
    pub completion_mode: Option<CompletionMode>,
    #[serde(default)]
    pub profile: Option<AgentProfileId>,
}

impl DbThread {
    pub const VERSION: &'static str = "0.3.0";

    pub fn from_json(json: &[u8]) -> Result<Self> {
        let saved_thread_json = serde_json::from_slice::<serde_json::Value>(json)?;
        match saved_thread_json.get("version") {
            Some(serde_json::Value::String(version)) => match version.as_str() {
                Self::VERSION => Ok(serde_json::from_value(saved_thread_json)?),
                _ => Self::upgrade_from_agent_1(agent::SerializedThread::from_json(json)?),
            },
            _ => Self::upgrade_from_agent_1(agent::SerializedThread::from_json(json)?),
        }
    }

    fn upgrade_from_agent_1(thread: agent::SerializedThread) -> Result<Self> {
        let mut messages = Vec::new();
        for msg in thread.messages {
            let message = match msg.role {
                language_model::Role::User => {
                    let mut content = Vec::new();

                    // Convert segments to content
                    for segment in msg.segments {
                        match segment {
                            thread_store::SerializedMessageSegment::Text { text } => {
                                content.push(UserMessageContent::Text(text));
                            }
                            thread_store::SerializedMessageSegment::Thinking { text, .. } => {
                                // User messages don't have thinking segments, but handle gracefully
                                content.push(UserMessageContent::Text(text));
                            }
                            thread_store::SerializedMessageSegment::RedactedThinking { .. } => {
                                // User messages don't have redacted thinking, skip.
                            }
                        }
                    }

                    // If no content was added, add context as text if available
                    if content.is_empty() && !msg.context.is_empty() {
                        content.push(UserMessageContent::Text(msg.context));
                    }

                    crate::Message::User(UserMessage {
                        // MessageId from old format can't be meaningfully converted, so generate a new one
                        id: acp_thread::UserMessageId::new(),
                        content,
                    })
                }
                language_model::Role::Assistant => {
                    let mut content = Vec::new();

                    // Convert segments to content
                    for segment in msg.segments {
                        match segment {
                            thread_store::SerializedMessageSegment::Text { text } => {
                                content.push(AgentMessageContent::Text(text));
                            }
                            thread_store::SerializedMessageSegment::Thinking {
                                text,
                                signature,
                            } => {
                                content.push(AgentMessageContent::Thinking { text, signature });
                            }
                            thread_store::SerializedMessageSegment::RedactedThinking { data } => {
                                content.push(AgentMessageContent::RedactedThinking(data));
                            }
                        }
                    }

                    // Convert tool uses
                    let mut tool_names_by_id = HashMap::default();
                    for tool_use in msg.tool_uses {
                        tool_names_by_id.insert(tool_use.id.clone(), tool_use.name.clone());
                        content.push(AgentMessageContent::ToolUse(
                            language_model::LanguageModelToolUse {
                                id: tool_use.id,
                                name: tool_use.name.into(),
                                raw_input: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                                input: tool_use.input,
                                is_input_complete: true,
                            },
                        ));
                    }

                    // Convert tool results
                    let mut tool_results = IndexMap::default();
                    for tool_result in msg.tool_results {
                        let name = tool_names_by_id
                            .remove(&tool_result.tool_use_id)
                            .unwrap_or_else(|| SharedString::from("unknown"));
                        tool_results.insert(
                            tool_result.tool_use_id.clone(),
                            language_model::LanguageModelToolResult {
                                tool_use_id: tool_result.tool_use_id,
                                tool_name: name.into(),
                                is_error: tool_result.is_error,
                                content: tool_result.content,
                                output: tool_result.output,
                            },
                        );
                    }

                    crate::Message::Agent(AgentMessage {
                        content,
                        tool_results,
                    })
                }
                language_model::Role::System => {
                    // Skip system messages as they're not supported in the new format
                    continue;
                }
            };

            messages.push(message);
        }

        Ok(Self {
            title: thread.summary,
            messages,
            updated_at: thread.updated_at,
            summary: thread.detailed_summary_state,
            initial_project_snapshot: thread.initial_project_snapshot,
            cumulative_token_usage: thread.cumulative_token_usage,
            request_token_usage: thread.request_token_usage,
            model: thread.model,
            completion_mode: thread.completion_mode,
            profile: thread.profile,
        })
    }
}
