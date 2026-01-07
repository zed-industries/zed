use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{AgentTool, ToolCallEventStream};

/// Spawns a subagent with its own context window to perform a delegated task.
///
/// Use this tool when you need to:
/// - Perform research that would consume too many tokens in the main context
/// - Execute a complex subtask independently
/// - Run multiple parallel investigations
///
/// You control what the subagent does by providing:
/// 1. A task prompt describing what the subagent should do
/// 2. A summary prompt that tells the subagent how to summarize its work when done
/// 3. A "context running out" prompt for when the subagent is low on tokens
///
/// The subagent has access to the same tools you do. You can optionally restrict
/// which tools the subagent can use.
///
/// IMPORTANT:
/// - Maximum 8 subagents can be spawned per turn
/// - Subagents cannot use tools you don't have access to
/// - If spawning multiple subagents that might write to the filesystem, provide
///   guidance on how to avoid conflicts (e.g., assign each to different directories)
/// - Instruct subagents to be concise in their summaries to conserve your context
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SubagentToolInput {
    /// Short label displayed in the UI while the subagent runs (e.g., "Researching alternatives")
    pub label: String,

    /// The initial prompt that tells the subagent what task to perform.
    /// Be specific about what you want the subagent to accomplish.
    pub task_prompt: String,

    /// The prompt sent to the subagent when it completes its task, asking it
    /// to summarize what it did and return results. This summary becomes the
    /// tool result you receive.
    ///
    /// Example: "Summarize what you found, listing the top 3 alternatives with pros/cons."
    pub summary_prompt: String,

    /// The prompt sent if the subagent is running low on context (25% remaining).
    /// Should instruct it to stop and summarize progress so far, plus what's left undone.
    ///
    /// Example: "Context is running low. Stop and summarize your progress so far,
    /// and list what remains to be investigated."
    pub context_low_prompt: String,

    /// Optional: Maximum runtime in milliseconds. If exceeded, the subagent is
    /// asked to summarize and return. No timeout by default.
    #[serde(default)]
    pub timeout_ms: Option<u64>,

    /// Optional: List of tool names the subagent is allowed to use.
    /// If not provided, the subagent can use all tools available to the parent.
    /// Tools listed here must be a subset of the parent's available tools.
    #[serde(default)]
    pub allowed_tools: Option<Vec<String>>,
}

pub struct SubagentTool;

impl SubagentTool {
    pub fn new() -> Self {
        Self
    }
}

impl AgentTool for SubagentTool {
    type Input = SubagentToolInput;
    type Output = String;

    fn name() -> &'static str {
        "subagent"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) => format!("Subagent: {}", input.label).into(),
            Err(_) => "Subagent".into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<String>> {
        event_stream.update_fields(
            acp::ToolCallUpdateFields::new()
                .content(vec![format!("Starting subagent: {}", input.label).into()]),
        );
        Task::ready(Ok("Subagent tool not yet implemented.".to_string()))
    }
}
