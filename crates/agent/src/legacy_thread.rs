use crate::ProjectSnapshot;
use agent_settings::AgentProfileId;
use anyhow::Result;
use chrono::{DateTime, Utc};
use gpui::SharedString;
use language_model::{LanguageModelToolResultContent, LanguageModelToolUseId, Role, TokenUsage};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum DetailedSummaryState {
    #[default]
    NotGenerated,
    Generating,
    Generated {
        text: SharedString,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct MessageId(pub usize);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SerializedThread {
    pub version: String,
    pub summary: SharedString,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<SerializedMessage>,
    #[serde(default)]
    pub initial_project_snapshot: Option<Arc<ProjectSnapshot>>,
    #[serde(default)]
    pub cumulative_token_usage: TokenUsage,
    #[serde(default)]
    pub request_token_usage: Vec<TokenUsage>,
    #[serde(default)]
    pub detailed_summary_state: DetailedSummaryState,
    #[serde(default)]
    pub model: Option<SerializedLanguageModel>,
    #[serde(default)]
    pub tool_use_limit_reached: bool,
    #[serde(default)]
    pub profile: Option<AgentProfileId>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SerializedLanguageModel {
    pub provider: String,
    pub model: String,
}

impl SerializedThread {
    pub const VERSION: &'static str = "0.2.0";

    pub fn from_json(json: &[u8]) -> Result<Self> {
        let saved_thread_json = serde_json::from_slice::<serde_json::Value>(json)?;
        match saved_thread_json.get("version") {
            Some(serde_json::Value::String(version)) => match version.as_str() {
                SerializedThreadV0_1_0::VERSION => {
                    let saved_thread =
                        serde_json::from_value::<SerializedThreadV0_1_0>(saved_thread_json)?;
                    Ok(saved_thread.upgrade())
                }
                SerializedThread::VERSION => Ok(serde_json::from_value::<SerializedThread>(
                    saved_thread_json,
                )?),
                _ => anyhow::bail!("unrecognized serialized thread version: {version:?}"),
            },
            None => {
                let saved_thread =
                    serde_json::from_value::<LegacySerializedThread>(saved_thread_json)?;
                Ok(saved_thread.upgrade())
            }
            version => anyhow::bail!("unrecognized serialized thread version: {version:?}"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializedThreadV0_1_0(
    // The structure did not change, so we are reusing the latest SerializedThread.
    // When making the next version, make sure this points to SerializedThreadV0_2_0
    SerializedThread,
);

impl SerializedThreadV0_1_0 {
    pub const VERSION: &'static str = "0.1.0";

    pub fn upgrade(self) -> SerializedThread {
        debug_assert_eq!(SerializedThread::VERSION, "0.2.0");

        let mut messages: Vec<SerializedMessage> = Vec::with_capacity(self.0.messages.len());

        for message in self.0.messages {
            if message.role == Role::User
                && !message.tool_results.is_empty()
                && let Some(last_message) = messages.last_mut()
            {
                debug_assert!(last_message.role == Role::Assistant);

                last_message.tool_results = message.tool_results;
                continue;
            }

            messages.push(message);
        }

        SerializedThread {
            messages,
            version: SerializedThread::VERSION.to_string(),
            ..self.0
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedMessage {
    pub id: MessageId,
    pub role: Role,
    #[serde(default)]
    pub segments: Vec<SerializedMessageSegment>,
    #[serde(default)]
    pub tool_uses: Vec<SerializedToolUse>,
    #[serde(default)]
    pub tool_results: Vec<SerializedToolResult>,
    #[serde(default)]
    pub context: String,
    #[serde(default)]
    pub creases: Vec<SerializedCrease>,
    #[serde(default)]
    pub is_hidden: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum SerializedMessageSegment {
    #[serde(rename = "text")]
    Text {
        text: String,
    },
    #[serde(rename = "thinking")]
    Thinking {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<String>,
    },
    RedactedThinking {
        data: String,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedToolUse {
    pub id: LanguageModelToolUseId,
    pub name: SharedString,
    pub input: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedToolResult {
    pub tool_use_id: LanguageModelToolUseId,
    pub is_error: bool,
    pub content: LanguageModelToolResultContent,
    pub output: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct LegacySerializedThread {
    pub summary: SharedString,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<LegacySerializedMessage>,
    #[serde(default)]
    pub initial_project_snapshot: Option<Arc<ProjectSnapshot>>,
}

impl LegacySerializedThread {
    pub fn upgrade(self) -> SerializedThread {
        SerializedThread {
            version: SerializedThread::VERSION.to_string(),
            summary: self.summary,
            updated_at: self.updated_at,
            messages: self.messages.into_iter().map(|msg| msg.upgrade()).collect(),
            initial_project_snapshot: self.initial_project_snapshot,
            cumulative_token_usage: TokenUsage::default(),
            request_token_usage: Vec::new(),
            detailed_summary_state: DetailedSummaryState::default(),
            model: None,
            tool_use_limit_reached: false,
            profile: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LegacySerializedMessage {
    pub id: MessageId,
    pub role: Role,
    pub text: String,
    #[serde(default)]
    pub tool_uses: Vec<SerializedToolUse>,
    #[serde(default)]
    pub tool_results: Vec<SerializedToolResult>,
}

impl LegacySerializedMessage {
    fn upgrade(self) -> SerializedMessage {
        SerializedMessage {
            id: self.id,
            role: self.role,
            segments: vec![SerializedMessageSegment::Text { text: self.text }],
            tool_uses: self.tool_uses,
            tool_results: self.tool_results,
            context: String::new(),
            creases: Vec::new(),
            is_hidden: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedCrease {
    pub start: usize,
    pub end: usize,
    pub icon_path: SharedString,
    pub label: SharedString,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use language_model::{Role, TokenUsage};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_legacy_serialized_thread_upgrade() {
        let updated_at = Utc::now();
        let legacy_thread = LegacySerializedThread {
            summary: "Test conversation".into(),
            updated_at,
            messages: vec![LegacySerializedMessage {
                id: MessageId(1),
                role: Role::User,
                text: "Hello, world!".to_string(),
                tool_uses: vec![],
                tool_results: vec![],
            }],
            initial_project_snapshot: None,
        };

        let upgraded = legacy_thread.upgrade();

        assert_eq!(
            upgraded,
            SerializedThread {
                summary: "Test conversation".into(),
                updated_at,
                messages: vec![SerializedMessage {
                    id: MessageId(1),
                    role: Role::User,
                    segments: vec![SerializedMessageSegment::Text {
                        text: "Hello, world!".to_string()
                    }],
                    tool_uses: vec![],
                    tool_results: vec![],
                    context: "".to_string(),
                    creases: vec![],
                    is_hidden: false
                }],
                version: SerializedThread::VERSION.to_string(),
                initial_project_snapshot: None,
                cumulative_token_usage: TokenUsage::default(),
                request_token_usage: vec![],
                detailed_summary_state: DetailedSummaryState::default(),
                model: None,
                tool_use_limit_reached: false,
                profile: None
            }
        )
    }

    #[test]
    fn test_serialized_threadv0_1_0_upgrade() {
        let updated_at = Utc::now();
        let thread_v0_1_0 = SerializedThreadV0_1_0(SerializedThread {
            summary: "Test conversation".into(),
            updated_at,
            messages: vec![
                SerializedMessage {
                    id: MessageId(1),
                    role: Role::User,
                    segments: vec![SerializedMessageSegment::Text {
                        text: "Use tool_1".to_string(),
                    }],
                    tool_uses: vec![],
                    tool_results: vec![],
                    context: "".to_string(),
                    creases: vec![],
                    is_hidden: false,
                },
                SerializedMessage {
                    id: MessageId(2),
                    role: Role::Assistant,
                    segments: vec![SerializedMessageSegment::Text {
                        text: "I want to use a tool".to_string(),
                    }],
                    tool_uses: vec![SerializedToolUse {
                        id: "abc".into(),
                        name: "tool_1".into(),
                        input: serde_json::Value::Null,
                    }],
                    tool_results: vec![],
                    context: "".to_string(),
                    creases: vec![],
                    is_hidden: false,
                },
                SerializedMessage {
                    id: MessageId(1),
                    role: Role::User,
                    segments: vec![SerializedMessageSegment::Text {
                        text: "Here is the tool result".to_string(),
                    }],
                    tool_uses: vec![],
                    tool_results: vec![SerializedToolResult {
                        tool_use_id: "abc".into(),
                        is_error: false,
                        content: LanguageModelToolResultContent::Text("abcdef".into()),
                        output: Some(serde_json::Value::Null),
                    }],
                    context: "".to_string(),
                    creases: vec![],
                    is_hidden: false,
                },
            ],
            version: SerializedThreadV0_1_0::VERSION.to_string(),
            initial_project_snapshot: None,
            cumulative_token_usage: TokenUsage::default(),
            request_token_usage: vec![],
            detailed_summary_state: DetailedSummaryState::default(),
            model: None,
            tool_use_limit_reached: false,
            profile: None,
        });
        let upgraded = thread_v0_1_0.upgrade();

        assert_eq!(
            upgraded,
            SerializedThread {
                summary: "Test conversation".into(),
                updated_at,
                messages: vec![
                    SerializedMessage {
                        id: MessageId(1),
                        role: Role::User,
                        segments: vec![SerializedMessageSegment::Text {
                            text: "Use tool_1".to_string()
                        }],
                        tool_uses: vec![],
                        tool_results: vec![],
                        context: "".to_string(),
                        creases: vec![],
                        is_hidden: false
                    },
                    SerializedMessage {
                        id: MessageId(2),
                        role: Role::Assistant,
                        segments: vec![SerializedMessageSegment::Text {
                            text: "I want to use a tool".to_string(),
                        }],
                        tool_uses: vec![SerializedToolUse {
                            id: "abc".into(),
                            name: "tool_1".into(),
                            input: serde_json::Value::Null,
                        }],
                        tool_results: vec![SerializedToolResult {
                            tool_use_id: "abc".into(),
                            is_error: false,
                            content: LanguageModelToolResultContent::Text("abcdef".into()),
                            output: Some(serde_json::Value::Null),
                        }],
                        context: "".to_string(),
                        creases: vec![],
                        is_hidden: false,
                    },
                ],
                version: SerializedThread::VERSION.to_string(),
                initial_project_snapshot: None,
                cumulative_token_usage: TokenUsage::default(),
                request_token_usage: vec![],
                detailed_summary_state: DetailedSummaryState::default(),
                model: None,
                tool_use_limit_reached: false,
                profile: None
            }
        )
    }
}
