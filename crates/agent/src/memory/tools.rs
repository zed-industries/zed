//! Model tools for the memory system.
//!
//! Provides `memory_write` and `memory_search` so the agent can
//! persist and retrieve facts across sessions. These tools form the
//! foundation for self-learning: the agent saves what it learns
//! about the user, the project, and its own workflows.

use std::sync::Arc;

use crate::memory::{MemoryStore, global_store};
use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema::v1 as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// memory_write
// ---------------------------------------------------------------------------

/// Save a fact to persistent memory. The agent can later retrieve it with
/// `memory_search`. Use this for user preferences, project conventions,
/// environment quirks, or any information worth remembering across sessions.
///
/// Facts persist in `~/.zed/memory.jsonl` and survive Zed restarts.
/// If a fact with the same `key` already exists, it is overwritten.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MemoryWriteToolInput {
    /// A unique slug for this fact, e.g. `"user-prefers-tabs"`.
    pub key: String,
    /// The content of the fact. This is what the model will read back.
    pub value: String,
    /// Optional category for grouping, e.g. `"preference"`, `"convention"`,
    /// `"environment"`, `"workflow"`.
    pub category: Option<String>,
    /// Optional tags for search, e.g. `["rust", "linting"]`.
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MemoryWriteToolOutput {
    Success { key: String, updated: bool },
    Error { error: String },
}

impl From<MemoryWriteToolOutput> for LanguageModelToolResultContent {
    fn from(output: MemoryWriteToolOutput) -> Self {
        match output {
            MemoryWriteToolOutput::Success { key, updated } => {
                let verb = if updated { "updated" } else { "saved" };
                format!("Fact {key:?} {verb}").into()
            }
            MemoryWriteToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct MemoryWriteTool;

impl AgentTool for MemoryWriteTool {
    type Input = MemoryWriteToolInput;
    type Output = MemoryWriteToolOutput;

    const NAME: &'static str = "memory_write";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Write
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Saving memory fact…".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        Task::ready(async move {
            let input = input.recv().await.map_err(|e| {
                MemoryWriteToolOutput::Error {
                    error: format!("Failed to receive input: {e}"),
                }
            })?;

            let store = global_store();
            let existed = store.get(&input.key).is_some();

            store.write(input.key, input.value, input.category, input.tags);

            Ok(MemoryWriteToolOutput::Success {
                key: input.key,
                updated: existed,
            })
        })
    }
}

// ---------------------------------------------------------------------------
// memory_search
// ---------------------------------------------------------------------------

/// Search previously saved memory facts. Queries match against the fact's
/// key, value, category, and tags (case-insensitive substring).
///
/// Results are returned sorted from most-recently-updated to least.
/// Use this to recall user preferences, project conventions, and other
/// persistent context the agent has learned.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MemorySearchToolInput {
    /// Search query — matched case-insensitively against key, value,
    /// category, and tags.
    pub query: String,
    /// Maximum number of results to return (default 10, max 50).
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MemorySearchToolOutput {
    Success {
        facts: Vec<serde_json::Value>,
        total: usize,
    },
    Error {
        error: String,
    },
}

impl From<MemorySearchToolOutput> for LanguageModelToolResultContent {
    fn from(output: MemorySearchToolOutput) -> Self {
        match output {
            MemorySearchToolOutput::Success { facts, total } => {
                let body = if facts.is_empty() {
                    "No matching memory facts found.".to_string()
                } else {
                    let mut lines = format!("Found {total} memory fact(s):\n\n");
                    for (i, fact) in facts.iter().enumerate() {
                        let key = fact.get("key").and_then(|v| v.as_str()).unwrap_or("?");
                        let value = fact.get("value").and_then(|v| v.as_str()).unwrap_or("");
                        let category = fact
                            .get("category")
                            .and_then(|v| v.as_str())
                            .filter(|s| !s.is_empty());
                        let tags: Vec<&str> = fact
                            .get("tags")
                            .and_then(|v| v.as_array())
                            .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
                            .unwrap_or_default();

                        lines.push_str(&format!("{}. **{key}**: {value}", i + 1));
                        if let Some(cat) = category {
                            lines.push_str(&format!(" [{}]", cat));
                        }
                        if !tags.is_empty() {
                            lines.push_str(&format!(" ({})", tags.join(", ")));
                        }
                        lines.push('\n');
                    }
                    lines
                };
                body.into()
            }
            MemorySearchToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct MemorySearchTool;

impl AgentTool for MemorySearchTool {
    type Input = MemorySearchToolInput;
    type Output = MemorySearchToolOutput;

    const NAME: &'static str = "memory_search";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Searching memory…".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        Task::ready(async move {
            let input = input.recv().await.map_err(|e| {
                MemorySearchToolOutput::Error {
                    error: format!("Failed to receive input: {e}"),
                }
            })?;

            let limit = input.limit.min(50);
            let store = global_store();
            let results = store.search(&input.query);
            let total = results.len();

            let facts: Vec<serde_json::Value> = results
                .into_iter()
                .take(limit)
                .filter_map(|f| serde_json::to_value(f).ok())
                .collect();

            Ok(MemorySearchToolOutput::Success { facts, total })
        })
    }
}
