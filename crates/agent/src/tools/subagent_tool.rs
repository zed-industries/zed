use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use collections::HashSet;
use futures::channel::mpsc;
use gpui::{App, AppContext, AsyncApp, Entity, SharedString, Task, WeakEntity};
use project::Project;
use prompt_store::ProjectContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::sync::Arc;
use std::time::Duration;

use crate::{
    AgentTool, ContextServerRegistry, MAX_SUBAGENT_DEPTH, SubagentContext, Templates, Thread,
    ThreadEvent, ToolCallEventStream,
};

/// When a subagent's remaining context window falls below this fraction (25%),
/// the "context running out" prompt is sent to encourage the subagent to wrap up.
const CONTEXT_LOW_THRESHOLD: f32 = 0.25;

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

pub struct SubagentTool {
    parent_thread: WeakEntity<Thread>,
    project: Entity<Project>,
    project_context: Entity<ProjectContext>,
    context_server_registry: Entity<ContextServerRegistry>,
    templates: Arc<Templates>,
    current_depth: u8,
    parent_tool_names: HashSet<SharedString>,
}

impl SubagentTool {
    pub fn new(
        parent_thread: WeakEntity<Thread>,
        project: Entity<Project>,
        project_context: Entity<ProjectContext>,
        context_server_registry: Entity<ContextServerRegistry>,
        templates: Arc<Templates>,
        current_depth: u8,
        parent_tool_names: Vec<SharedString>,
    ) -> Self {
        Self {
            parent_thread,
            project,
            project_context,
            context_server_registry,
            templates,
            current_depth,
            parent_tool_names: parent_tool_names.into_iter().collect(),
        }
    }

    fn validate_allowed_tools(&self, allowed_tools: &Option<Vec<String>>) -> Result<()> {
        if let Some(tools) = allowed_tools {
            for tool in tools {
                if !self.parent_tool_names.contains(tool.as_str()) {
                    return Err(anyhow!(
                        "Tool '{}' is not available to the parent agent. Available tools: {:?}",
                        tool,
                        self.parent_tool_names.iter().collect::<Vec<_>>()
                    ));
                }
            }
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
        if self.current_depth >= MAX_SUBAGENT_DEPTH {
            return Task::ready(Err(anyhow!(
                "Maximum subagent depth ({}) reached",
                MAX_SUBAGENT_DEPTH
            )));
        }

        if let Err(e) = self.validate_allowed_tools(&input.allowed_tools) {
            return Task::ready(Err(e));
        }

        let Some(parent_thread) = self.parent_thread.upgrade() else {
            return Task::ready(Err(anyhow!("Parent thread no longer exists")));
        };

        let parent_thread_id = parent_thread.read(cx).id().clone();
        let parent_model = parent_thread.read(cx).model().cloned();
        let tool_use_id = event_stream.tool_use_id().clone();

        let Some(model) = parent_model else {
            return Task::ready(Err(anyhow!("No model configured")));
        };

        let subagent_context = SubagentContext {
            parent_thread_id,
            tool_use_id,
            depth: self.current_depth + 1,
            summary_prompt: input.summary_prompt.clone(),
            context_low_prompt: input.context_low_prompt.clone(),
        };

        let project = self.project.clone();
        let project_context = self.project_context.clone();
        let context_server_registry = self.context_server_registry.clone();
        let templates = self.templates.clone();
        let task_prompt = input.task_prompt;
        let timeout_ms = input.timeout_ms;
        let allowed_tools: Option<HashSet<SharedString>> = input
            .allowed_tools
            .map(|tools| tools.into_iter().map(SharedString::from).collect());

        let parent_thread = self.parent_thread.clone();

        cx.spawn(async move |cx| {
            let subagent_thread = cx.new(|cx| {
                Thread::new_subagent(
                    project.clone(),
                    project_context.clone(),
                    context_server_registry.clone(),
                    templates.clone(),
                    model,
                    subagent_context,
                    cx,
                )
            })?;

            let subagent_weak = subagent_thread.downgrade();

            if let Some(parent) = parent_thread.upgrade() {
                parent.update(cx, |thread, _cx| {
                    thread.register_running_subagent(subagent_weak.clone());
                })?;
            }

            let result =
                run_subagent(&subagent_thread, allowed_tools, task_prompt, timeout_ms, cx).await;

            if let Some(parent) = parent_thread.upgrade() {
                let _ = parent.update(cx, |thread, _cx| {
                    thread.unregister_running_subagent(&subagent_weak);
                });
            }

            result
        })
    }
}

async fn run_subagent(
    subagent_thread: &Entity<Thread>,
    allowed_tools: Option<HashSet<SharedString>>,
    task_prompt: String,
    timeout_ms: Option<u64>,
    cx: &mut AsyncApp,
) -> Result<String> {
    if let Some(ref allowed) = allowed_tools {
        subagent_thread.update(cx, |thread, _cx| {
            thread.restrict_tools(allowed);
        })?;
    }

    let mut events_rx =
        subagent_thread.update(cx, |thread, cx| thread.submit_user_message(task_prompt, cx))??;

    let timed_out = if let Some(timeout) = timeout_ms {
        wait_for_turn_completion_with_timeout(&mut events_rx, Duration::from_millis(timeout), cx)
            .await
    } else {
        wait_for_turn_completion(&mut events_rx).await;
        false
    };

    let should_interrupt =
        timed_out || check_context_low(subagent_thread, CONTEXT_LOW_THRESHOLD, cx)?;

    if should_interrupt {
        let mut summary_rx =
            subagent_thread.update(cx, |thread, cx| thread.interrupt_for_summary(cx))??;
        wait_for_turn_completion(&mut summary_rx).await;
    } else {
        let mut summary_rx =
            subagent_thread.update(cx, |thread, cx| thread.request_final_summary(cx))??;
        wait_for_turn_completion(&mut summary_rx).await;
    }

    extract_last_message(subagent_thread, cx)
}

async fn wait_for_turn_completion(events_rx: &mut mpsc::UnboundedReceiver<Result<ThreadEvent>>) {
    while let Some(event) = events_rx.next().await {
        match event {
            Ok(ThreadEvent::Stop(_)) => break,
            Err(_) => break,
            _ => continue,
        }
    }
}

async fn wait_for_turn_completion_with_timeout(
    events_rx: &mut mpsc::UnboundedReceiver<Result<ThreadEvent>>,
    timeout: Duration,
    cx: &AsyncApp,
) -> bool {
    use futures::future::{self, Either};

    let deadline = std::time::Instant::now() + timeout;

    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return true;
        }

        let timeout_future = cx.background_executor().timer(remaining);
        let event_future = events_rx.next();

        match future::select(event_future, timeout_future).await {
            Either::Left((event, _)) => match event {
                Some(Ok(ThreadEvent::Stop(_))) => return false,
                Some(Err(_)) => return false,
                None => return false,
                Some(_) => continue,
            },
            Either::Right((_, _)) => return true,
        }
    }
}

fn check_context_low(thread: &Entity<Thread>, threshold: f32, cx: &mut AsyncApp) -> Result<bool> {
    thread.read_with(cx, |thread, _| {
        if let Some(usage) = thread.latest_token_usage() {
            let remaining_ratio = 1.0 - (usage.used_tokens as f32 / usage.max_tokens as f32);
            remaining_ratio <= threshold
        } else {
            false
        }
    })
}

fn extract_last_message(thread: &Entity<Thread>, cx: &mut AsyncApp) -> Result<String> {
    thread.read_with(cx, |thread, _| {
        thread
            .last_message()
            .map(|m| m.to_markdown())
            .unwrap_or_else(|| "No response from subagent".to_string())
    })
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
