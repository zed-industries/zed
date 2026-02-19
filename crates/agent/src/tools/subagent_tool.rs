use acp_thread::SUBAGENT_SESSION_ID_META_KEY;
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, SharedString, Task, WeakEntity};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{rc::Rc, time::Duration};

use crate::{AgentTool, Thread, ThreadEnvironment, ToolCallEventStream};

// - By default: same tools
// - Params:
//   - label / task name / title
//   - Prompt
//   - (optional) session id to prompt the same thread again
//   - optional timeout
// - Return: last assistant message

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

    /// Optional: Maximum runtime in milliseconds. If exceeded, the subagent is
    /// asked to summarize and return. No timeout by default.
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubagentToolOutput {
    pub subagent_session_id: acp::SessionId,
    pub summary: String,
}

impl From<SubagentToolOutput> for LanguageModelToolResultContent {
    fn from(output: SubagentToolOutput) -> Self {
        output.summary.into()
    }
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
}

impl AgentTool for SubagentTool {
    type Input = SubagentToolInput;
    type Output = SubagentToolOutput;

    const NAME: &'static str = "subagent";

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
    ) -> Task<Result<SubagentToolOutput>> {
        let Some(parent_thread_entity) = self.parent_thread.upgrade() else {
            return Task::ready(Err(anyhow!("Parent thread no longer exists")));
        };

        let subagent = match self.environment.create_subagent(
            parent_thread_entity,
            input.label,
            input.task_prompt,
            input.timeout_ms.map(|ms| Duration::from_millis(ms)),
            cx,
        ) {
            Ok(subagent) => subagent,
            Err(err) => return Task::ready(Err(err)),
        };

        let subagent_session_id = subagent.id();

        event_stream.subagent_spawned(subagent_session_id.clone());
        let meta = acp::Meta::from_iter([(
            SUBAGENT_SESSION_ID_META_KEY.into(),
            subagent_session_id.to_string().into(),
        )]);
        event_stream.update_fields_with_meta(acp::ToolCallUpdateFields::new(), Some(meta));

        cx.spawn(async move |cx| {
            let summary = subagent.wait_for_summary(input.summary_prompt, cx).await?;
            Ok(SubagentToolOutput {
                subagent_session_id,
                summary,
            })
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Result<()> {
        event_stream.subagent_spawned(output.subagent_session_id.clone());
        let meta = acp::Meta::from_iter([(
            SUBAGENT_SESSION_ID_META_KEY.into(),
            output.subagent_session_id.to_string().into(),
        )]);
        event_stream.update_fields_with_meta(acp::ToolCallUpdateFields::new(), Some(meta));
        Ok(())
    }
}
