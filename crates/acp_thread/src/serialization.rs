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
            // ToolKind is non-exhaustive, so we need a wildcard for future variants
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
/// Note: WaitingForConfirmation cannot be fully reconstructed since it contains
/// a oneshot channel, so when deserializing it becomes InProgress to indicate
/// the tool was still running when serialized.
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
            // WaitingForConfirmation cannot be fully reconstructed (missing channel),
            // so we show it as InProgress to indicate it was still running
            SerializedToolCallStatus::WaitingForConfirmation => Self::InProgress,
            SerializedToolCallStatus::InProgress => Self::InProgress,
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
                    kind: SerializedToolKind::Read,
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

        // Verify UserMessage content
        match (&thread.entries[0], &deserialized.entries[0]) {
            (
                SerializedAgentThreadEntry::UserMessage {
                    id: id1,
                    content: content1,
                    indented: indented1,
                },
                SerializedAgentThreadEntry::UserMessage {
                    id: id2,
                    content: content2,
                    indented: indented2,
                },
            ) => {
                assert_eq!(id1, id2);
                assert_eq!(content1, content2);
                assert_eq!(indented1, indented2);
            }
            _ => panic!("Expected UserMessage entries"),
        }

        // Verify AssistantMessage content
        match (&thread.entries[1], &deserialized.entries[1]) {
            (
                SerializedAgentThreadEntry::AssistantMessage {
                    chunks: chunks1,
                    indented: indented1,
                },
                SerializedAgentThreadEntry::AssistantMessage {
                    chunks: chunks2,
                    indented: indented2,
                },
            ) => {
                assert_eq!(chunks1.len(), chunks2.len());
                assert_eq!(indented1, indented2);
            }
            _ => panic!("Expected AssistantMessage entries"),
        }

        // Verify ToolCall content
        match (&thread.entries[2], &deserialized.entries[2]) {
            (
                SerializedAgentThreadEntry::ToolCall {
                    id: id1,
                    tool_name: name1,
                    ..
                },
                SerializedAgentThreadEntry::ToolCall {
                    id: id2,
                    tool_name: name2,
                    ..
                },
            ) => {
                assert_eq!(id1, id2);
                assert_eq!(name1, name2);
            }
            _ => panic!("Expected ToolCall entries"),
        }
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
    fn test_diff_content_serialization() {
        let thread = SerializedAcpThread {
            title: "Diff Test".to_string(),
            session_id: "diff-session".to_string(),
            entries: vec![SerializedAgentThreadEntry::ToolCall {
                id: "edit-1".to_string(),
                tool_name: Some("edit_file".to_string()),
                label: "Editing src/main.rs".to_string(),
                kind: SerializedToolKind::Edit,
                status: SerializedToolCallStatus::Completed,
                content: vec![SerializedToolCallContent::Diff {
                    path: "src/main.rs".to_string(),
                    old_text: Some("fn main() {\n    println!(\"Hello\");\n}".to_string()),
                    new_text: "fn main() {\n    println!(\"Hello, World!\");\n}".to_string(),
                }],
                raw_input: Some(serde_json::json!({"path": "src/main.rs"})),
                raw_output: None,
            }],
        };

        let json = serde_json::to_string(&thread).expect("serialization should succeed");
        let deserialized: SerializedAcpThread =
            serde_json::from_str(&json).expect("deserialization should succeed");

        if let SerializedAgentThreadEntry::ToolCall { content, .. } = &deserialized.entries[0] {
            if let SerializedToolCallContent::Diff {
                path,
                old_text,
                new_text,
            } = &content[0]
            {
                assert_eq!(path, "src/main.rs");
                assert!(old_text.as_ref().unwrap().contains("Hello"));
                assert!(new_text.contains("Hello, World!"));
            } else {
                panic!("Expected Diff content");
            }
        } else {
            panic!("Expected ToolCall entry");
        }
    }

    #[test]
    fn test_terminal_content_serialization() {
        let thread = SerializedAcpThread {
            title: "Terminal Test".to_string(),
            session_id: "terminal-session".to_string(),
            entries: vec![SerializedAgentThreadEntry::ToolCall {
                id: "terminal-1".to_string(),
                tool_name: Some("terminal".to_string()),
                label: "Running ls -la".to_string(),
                kind: SerializedToolKind::Execute,
                status: SerializedToolCallStatus::Completed,
                content: vec![SerializedToolCallContent::Terminal {
                    id: "term-abc123".to_string(),
                    command: "ls -la".to_string(),
                    output: "total 42\ndrwxr-xr-x  5 user  staff  160 Jan  1 12:00 .\n".to_string(),
                }],
                raw_input: Some(serde_json::json!({"command": "ls -la"})),
                raw_output: None,
            }],
        };

        let json = serde_json::to_string(&thread).expect("serialization should succeed");
        let deserialized: SerializedAcpThread =
            serde_json::from_str(&json).expect("deserialization should succeed");

        if let SerializedAgentThreadEntry::ToolCall { content, .. } = &deserialized.entries[0] {
            if let SerializedToolCallContent::Terminal {
                id,
                command,
                output,
            } = &content[0]
            {
                assert_eq!(id, "term-abc123");
                assert_eq!(command, "ls -la");
                assert!(output.contains("total 42"));
            } else {
                panic!("Expected Terminal content");
            }
        } else {
            panic!("Expected ToolCall entry");
        }
    }

    #[test]
    fn test_thought_chunk_serialization() {
        let thread = SerializedAcpThread {
            title: "Thought Test".to_string(),
            session_id: "thought-session".to_string(),
            entries: vec![SerializedAgentThreadEntry::AssistantMessage {
                chunks: vec![
                    SerializedAssistantChunk::Thought {
                        markdown: "Let me think about this...".to_string(),
                    },
                    SerializedAssistantChunk::Message {
                        markdown: "Here's my answer.".to_string(),
                    },
                ],
                indented: false,
            }],
        };

        let json = serde_json::to_string(&thread).expect("serialization should succeed");
        let deserialized: SerializedAcpThread =
            serde_json::from_str(&json).expect("deserialization should succeed");

        if let SerializedAgentThreadEntry::AssistantMessage { chunks, .. } =
            &deserialized.entries[0]
        {
            assert_eq!(chunks.len(), 2);
            match &chunks[0] {
                SerializedAssistantChunk::Thought { markdown } => {
                    assert!(markdown.contains("think about this"));
                }
                _ => panic!("Expected Thought chunk"),
            }
            match &chunks[1] {
                SerializedAssistantChunk::Message { markdown } => {
                    assert!(markdown.contains("my answer"));
                }
                _ => panic!("Expected Message chunk"),
            }
        } else {
            panic!("Expected AssistantMessage entry");
        }
    }

    #[test]
    fn test_status_preservation() {
        // Test that all status variants serialize and deserialize correctly
        let statuses = vec![
            (SerializedToolCallStatus::Pending, "Pending"),
            (
                SerializedToolCallStatus::WaitingForConfirmation,
                "WaitingForConfirmation",
            ),
            (SerializedToolCallStatus::InProgress, "InProgress"),
            (SerializedToolCallStatus::Completed, "Completed"),
            (SerializedToolCallStatus::Failed, "Failed"),
            (SerializedToolCallStatus::Rejected, "Rejected"),
            (SerializedToolCallStatus::Canceled, "Canceled"),
        ];

        for (status, name) in statuses {
            let thread = SerializedAcpThread {
                title: format!("Status Test - {}", name),
                session_id: "status-session".to_string(),
                entries: vec![SerializedAgentThreadEntry::ToolCall {
                    id: "tool-1".to_string(),
                    tool_name: Some("test".to_string()),
                    label: "Test tool".to_string(),
                    kind: SerializedToolKind::Other,
                    status: status.clone(),
                    content: vec![],
                    raw_input: None,
                    raw_output: None,
                }],
            };

            let json = serde_json::to_string(&thread).expect("serialization should succeed");
            let deserialized: SerializedAcpThread =
                serde_json::from_str(&json).expect("deserialization should succeed");

            if let SerializedAgentThreadEntry::ToolCall {
                status: deser_status,
                ..
            } = &deserialized.entries[0]
            {
                // Verify the serialized status matches
                assert_eq!(
                    std::mem::discriminant(&status),
                    std::mem::discriminant(deser_status),
                    "Status {} should roundtrip correctly",
                    name
                );
            } else {
                panic!("Expected ToolCall entry");
            }
        }
    }

    #[test]
    fn test_user_message_id_preservation() {
        let thread = SerializedAcpThread {
            title: "ID Test".to_string(),
            session_id: "id-session".to_string(),
            entries: vec![
                SerializedAgentThreadEntry::UserMessage {
                    id: Some("user-msg-12345".to_string()),
                    content: "Message with ID".to_string(),
                    indented: false,
                },
                SerializedAgentThreadEntry::UserMessage {
                    id: None,
                    content: "Message without ID".to_string(),
                    indented: true,
                },
            ],
        };

        let json = serde_json::to_string(&thread).expect("serialization should succeed");
        let deserialized: SerializedAcpThread =
            serde_json::from_str(&json).expect("deserialization should succeed");

        match &deserialized.entries[0] {
            SerializedAgentThreadEntry::UserMessage { id, indented, .. } => {
                assert_eq!(id.as_deref(), Some("user-msg-12345"));
                assert!(!indented);
            }
            _ => panic!("Expected UserMessage"),
        }

        match &deserialized.entries[1] {
            SerializedAgentThreadEntry::UserMessage { id, indented, .. } => {
                assert!(id.is_none());
                assert!(*indented);
            }
            _ => panic!("Expected UserMessage"),
        }
    }
}
