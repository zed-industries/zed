use acp_thread::SUBAGENT_SESSION_ID_META_KEY;
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use futures::FutureExt as _;
use gpui::{App, SharedString, Task, WeakEntity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;

use crate::{AgentTool, Thread, ThreadEnvironment, ToolCallEventStream};

/// Spawns a subagent with its own context window to perform a delegated task.
///
/// Use this tool when you want to do any of the following:
/// - Perform an investigation where all you need to know is the outcome, not the research that led to that outcome.
/// - Complete a self-contained task where you need to know if it succeeded or failed (and how), but none of its intermediate output.
/// - Run multiple tasks in parallel that would take significantly longer to run sequentially.
///
/// You control what the subagent does by providing:
/// 1. A task prompt describing what the subagent should do
/// 2. A summary prompt that tells the subagent how to summarize its work when done
/// 3. A "context running out" prompt for when the subagent is low on tokens
///
/// Each subagent has access to the same tools you do. You can optionally restrict
/// which tools each subagent can use.
///
/// Note:
/// - Maximum 8 subagents can run in parallel
/// - Subagents cannot use tools you don't have access to
/// - If spawning multiple subagents that might write to the filesystem, provide
///   guidance on how to avoid conflicts (e.g. assign each to different directories)
/// - Instruct subagents to be concise in their summaries to conserve your context
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

/// Tool that spawns a subagent thread to work on a task.
pub struct SubagentTool {
    parent_thread: WeakEntity<Thread>,
    environment: Rc<dyn ThreadEnvironment>,
}

impl SubagentTool {
    pub fn new(parent_thread: WeakEntity<Thread>, environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self {
            parent_thread,
            environment,
        }
    }

    pub fn validate_allowed_tools(
        &self,
        allowed_tools: &Option<Vec<String>>,
        cx: &App,
    ) -> Result<()> {
        let Some(allowed_tools) = allowed_tools else {
            return Ok(());
        };

        let invalid_tools: Vec<_> = self.parent_thread.read_with(cx, |thread, _cx| {
            allowed_tools
                .iter()
                .filter(|tool| !thread.tools.contains_key(tool.as_str()))
                .map(|s| format!("'{s}'"))
                .collect()
        })?;

        if !invalid_tools.is_empty() {
            return Err(anyhow!(
                "The following tools do not exist: {}",
                invalid_tools.join(", ")
            ));
        }

        Ok(())
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
        input
            .map(|i| i.label.into())
            .unwrap_or_else(|_| "Subagent".into())
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>> {
        if let Err(e) = self.validate_allowed_tools(&input.allowed_tools, cx) {
            return Task::ready(Err(e));
        }

        let Some(parent_thread_entity) = self.parent_thread.upgrade() else {
            return Task::ready(Err(anyhow!("Parent thread no longer exists")));
        };

        let subagent = match self.environment.create_subagent(
            parent_thread_entity,
            input.label,
            input.task_prompt,
            input.timeout_ms,
            input.allowed_tools,
            cx,
        ) {
            Ok(subagent) => subagent,
            Err(err) => return Task::ready(Err(err)),
        };

        let subagent_session = subagent.id();
        let mut meta = acp::Meta::new();
        meta.insert(
            SUBAGENT_SESSION_ID_META_KEY.into(),
            subagent_session.0.to_string().into(),
        );

        event_stream.update_fields_with_meta(acp::ToolCallUpdateFields::new(), Some(meta));

        cx.spawn(async move |cx| {
            let summary_task =
                subagent.wait_for_summary(input.summary_prompt, input.context_low_prompt, cx);

            futures::select_biased! {
                summary = summary_task.fuse() => summary,
                _ = event_stream.cancelled_by_user().fuse() => {
                    Err(anyhow!("Subagent was cancelled by user"))
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use language_model::LanguageModelToolSchemaFormat;

    #[test]
    fn test_subagent_tool_input_json_schema_is_valid() {
        let schema = SubagentTool::input_schema(LanguageModelToolSchemaFormat::JsonSchema);
        let schema_json = serde_json::to_value(&schema).expect("schema should serialize to JSON");

        assert!(
            schema_json.get("properties").is_some(),
            "schema should have properties"
        );
        let properties = schema_json.get("properties").unwrap();

        assert!(properties.get("label").is_some(), "should have label field");
        assert!(
            properties.get("task_prompt").is_some(),
            "should have task_prompt field"
        );
        assert!(
            properties.get("summary_prompt").is_some(),
            "should have summary_prompt field"
        );
        assert!(
            properties.get("context_low_prompt").is_some(),
            "should have context_low_prompt field"
        );
        assert!(
            properties.get("timeout_ms").is_some(),
            "should have timeout_ms field"
        );
        assert!(
            properties.get("allowed_tools").is_some(),
            "should have allowed_tools field"
        );
    }

    #[test]
    fn test_subagent_tool_name() {
        assert_eq!(SubagentTool::name(), "subagent");
    }

    #[test]
    fn test_subagent_tool_kind() {
        assert_eq!(SubagentTool::kind(), acp::ToolKind::Other);
    }
}
