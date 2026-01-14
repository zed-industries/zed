use agent_client_protocol as acp;
use serde::{Deserialize, Serialize};

/// Serializable representation of an AcpThread for persistence.
/// This captures the display state of a thread so it can be restored when reloading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedAcpThread {
    pub title: String,
    pub session_id: String,
    pub entries: Vec<SerializedAgentThreadEntry>,
}

/// Serializable representation of an entry in the thread.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedAgentThreadEntry {
    UserMessage {
        id: Option<String>,
        /// Markdown representation of the message content
        content: String,
        indented: bool,
    },
    AssistantMessage {
        chunks: Vec<SerializedAssistantChunk>,
        indented: bool,
    },
    ToolCall {
        id: String,
        tool_name: Option<String>,
        label: String,
        kind: SerializedToolKind,
        status: SerializedToolCallStatus,
        content: Vec<SerializedToolCallContent>,
        raw_input: Option<serde_json::Value>,
        raw_output: Option<serde_json::Value>,
    },
}

/// Serializable representation of assistant message chunks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedAssistantChunk {
    Message { markdown: String },
    Thought { markdown: String },
}

/// Serializable representation of tool kinds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedToolKind {
    Read,
    Edit,
    Delete,
    Move,
    Search,
    Execute,
    Think,
    Fetch,
    SwitchMode,
    Other,
}

impl From<acp::ToolKind> for SerializedToolKind {
    fn from(kind: acp::ToolKind) -> Self {
        match kind {
            acp::ToolKind::Read => Self::Read,
            acp::ToolKind::Edit => Self::Edit,
            acp::ToolKind::Delete => Self::Delete,
            acp::ToolKind::Move => Self::Move,
            acp::ToolKind::Search => Self::Search,
            acp::ToolKind::Execute => Self::Execute,
            acp::ToolKind::Think => Self::Think,
            acp::ToolKind::Fetch => Self::Fetch,
            acp::ToolKind::SwitchMode => Self::SwitchMode,
            acp::ToolKind::Other | _ => Self::Other,
        }
    }
}

impl From<SerializedToolKind> for acp::ToolKind {
    fn from(kind: SerializedToolKind) -> Self {
        match kind {
            SerializedToolKind::Read => Self::Read,
            SerializedToolKind::Edit => Self::Edit,
            SerializedToolKind::Delete => Self::Delete,
            SerializedToolKind::Move => Self::Move,
            SerializedToolKind::Search => Self::Search,
            SerializedToolKind::Execute => Self::Execute,
            SerializedToolKind::Think => Self::Think,
            SerializedToolKind::Fetch => Self::Fetch,
            SerializedToolKind::SwitchMode => Self::SwitchMode,
            SerializedToolKind::Other => Self::Other,
        }
    }
}

/// Serializable representation of tool call status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedToolCallStatus {
    Pending,
    WaitingForConfirmation,
    InProgress,
    Completed,
    Failed,
    Rejected,
    Canceled,
}

impl From<&crate::ToolCallStatus> for SerializedToolCallStatus {
    fn from(status: &crate::ToolCallStatus) -> Self {
        match status {
            crate::ToolCallStatus::Pending => Self::Pending,
            crate::ToolCallStatus::WaitingForConfirmation { .. } => Self::WaitingForConfirmation,
            crate::ToolCallStatus::InProgress => Self::InProgress,
            crate::ToolCallStatus::Completed => Self::Completed,
            crate::ToolCallStatus::Failed => Self::Failed,
            crate::ToolCallStatus::Rejected => Self::Rejected,
            crate::ToolCallStatus::Canceled => Self::Canceled,
        }
    }
}

impl From<SerializedToolCallStatus> for crate::ToolCallStatus {
    fn from(status: SerializedToolCallStatus) -> Self {
        match status {
            SerializedToolCallStatus::Pending => Self::Pending,
            SerializedToolCallStatus::WaitingForConfirmation => Self::Completed,
            SerializedToolCallStatus::InProgress => Self::Completed,
            SerializedToolCallStatus::Completed => Self::Completed,
            SerializedToolCallStatus::Failed => Self::Failed,
            SerializedToolCallStatus::Rejected => Self::Rejected,
            SerializedToolCallStatus::Canceled => Self::Canceled,
        }
    }
}

/// Serializable representation of tool call content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedToolCallContent {
    /// Plain markdown content
    Markdown(String),
    /// A file diff
    Diff {
        path: String,
        old_text: Option<String>,
        new_text: String,
    },
    /// Terminal output
    Terminal {
        id: String,
        command: String,
        output: String,
    },
    /// Nested subagent thread (recursive)
    SubagentThread(Box<SerializedAcpThread>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialized_acp_thread_roundtrip() {
        let thread = SerializedAcpThread {
            title: "Test Thread".to_string(),
            session_id: "session-123".to_string(),
            entries: vec![
                SerializedAgentThreadEntry::UserMessage {
                    id: Some("msg-1".to_string()),
                    content: "Hello, world!".to_string(),
                    indented: false,
                },
                SerializedAgentThreadEntry::AssistantMessage {
                    chunks: vec![SerializedAssistantChunk::Message {
                        markdown: "Hi there!".to_string(),
                    }],
                    indented: false,
                },
                SerializedAgentThreadEntry::ToolCall {
                    id: "tool-1".to_string(),
                    tool_name: Some("read_file".to_string()),
                    label: "Reading file".to_string(),
                    kind: SerializedToolKind::Other,
                    status: SerializedToolCallStatus::Completed,
                    content: vec![SerializedToolCallContent::Markdown(
                        "File contents here".to_string(),
                    )],
                    raw_input: Some(serde_json::json!({"path": "test.txt"})),
                    raw_output: Some(serde_json::json!("file contents")),
                },
            ],
        };

        let json = serde_json::to_string(&thread).expect("serialization should succeed");
        let deserialized: SerializedAcpThread =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(thread.title, deserialized.title);
        assert_eq!(thread.session_id, deserialized.session_id);
        assert_eq!(thread.entries.len(), deserialized.entries.len());
    }

    #[test]
    fn test_nested_subagent_serialization() {
        let nested_thread = SerializedAcpThread {
            title: "Nested Subagent".to_string(),
            session_id: "nested-session".to_string(),
            entries: vec![SerializedAgentThreadEntry::AssistantMessage {
                chunks: vec![SerializedAssistantChunk::Message {
                    markdown: "I'm a nested subagent".to_string(),
                }],
                indented: false,
            }],
        };

        let parent_thread = SerializedAcpThread {
            title: "Parent Thread".to_string(),
            session_id: "parent-session".to_string(),
            entries: vec![SerializedAgentThreadEntry::ToolCall {
                id: "subagent-tool".to_string(),
                tool_name: Some("subagent".to_string()),
                label: "Running subagent".to_string(),
                kind: SerializedToolKind::Other,
                status: SerializedToolCallStatus::Completed,
                content: vec![SerializedToolCallContent::SubagentThread(Box::new(
                    nested_thread,
                ))],
                raw_input: None,
                raw_output: None,
            }],
        };

        let json = serde_json::to_string(&parent_thread).expect("serialization should succeed");
        let deserialized: SerializedAcpThread =
            serde_json::from_str(&json).expect("deserialization should succeed");

        if let SerializedAgentThreadEntry::ToolCall { content, .. } = &deserialized.entries[0] {
            if let SerializedToolCallContent::SubagentThread(nested) = &content[0] {
                assert_eq!(nested.title, "Nested Subagent");
            } else {
                panic!("Expected SubagentThread content");
            }
        } else {
            panic!("Expected ToolCall entry");
        }
    }

    #[test]
    fn test_tool_kind_conversion() {
        assert!(matches!(
            SerializedToolKind::from(acp::ToolKind::Read),
            SerializedToolKind::Read
        ));
        assert!(matches!(
            SerializedToolKind::from(acp::ToolKind::Edit),
            SerializedToolKind::Edit
        ));
        assert!(matches!(
            SerializedToolKind::from(acp::ToolKind::Execute),
            SerializedToolKind::Execute
        ));
        assert!(matches!(
            SerializedToolKind::from(acp::ToolKind::Other),
            SerializedToolKind::Other
        ));

        assert!(matches!(
            acp::ToolKind::from(SerializedToolKind::Read),
            acp::ToolKind::Read
        ));
        assert!(matches!(
            acp::ToolKind::from(SerializedToolKind::Edit),
            acp::ToolKind::Edit
        ));
        assert!(matches!(
            acp::ToolKind::from(SerializedToolKind::Execute),
            acp::ToolKind::Execute
        ));
        assert!(matches!(
            acp::ToolKind::from(SerializedToolKind::Other),
            acp::ToolKind::Other
        ));
    }
}
