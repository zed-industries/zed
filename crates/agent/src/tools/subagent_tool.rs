use acp_thread::{AcpThread, AgentConnection, UserMessageId};
use action_log::ActionLog;
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use collections::{BTreeMap, HashSet};
use futures::{FutureExt, channel::mpsc};
use gpui::{App, AppContext, AsyncApp, Entity, SharedString, Task, WeakEntity};
use language_model::LanguageModelToolUseId;
use project::Project;
use prompt_store::ProjectContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::any::Any;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use util::ResultExt;
use watch;

use crate::{
    AgentTool, AnyAgentTool, ContextServerRegistry, MAX_PARALLEL_SUBAGENTS, MAX_SUBAGENT_DEPTH,
    SubagentContext, Templates, Thread, ThreadEvent, ToolCallAuthorization, ToolCallEventStream,
};

/// When a subagent's remaining context window falls below this fraction (25%),
/// the "context running out" prompt is sent to encourage the subagent to wrap up.
const CONTEXT_LOW_THRESHOLD: f32 = 0.25;

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
    project: Entity<Project>,
    project_context: Entity<ProjectContext>,
    context_server_registry: Entity<ContextServerRegistry>,
    templates: Arc<Templates>,
    current_depth: u8,
    /// The tools available to the parent thread, captured before SubagentTool was added.
    /// Subagents inherit from this set (or a subset via `allowed_tools` in the config).
    /// This is captured early so subagents don't get the subagent tool themselves.
    parent_tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
}

impl SubagentTool {
    pub fn new(
        parent_thread: WeakEntity<Thread>,
        project: Entity<Project>,
        project_context: Entity<ProjectContext>,
        context_server_registry: Entity<ContextServerRegistry>,
        templates: Arc<Templates>,
        current_depth: u8,
        parent_tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
    ) -> Self {
        Self {
            parent_thread,
            project,
            project_context,
            context_server_registry,
            templates,
            current_depth,
            parent_tools,
        }
    }

    pub fn validate_allowed_tools(&self, allowed_tools: &Option<Vec<String>>) -> Result<()> {
        let Some(tools) = allowed_tools else {
            return Ok(());
        };

        let invalid_tools: Vec<&str> = tools
            .iter()
            .filter(|tool| !self.parent_tools.contains_key(tool.as_str()))
            .map(|s| s.as_str())
            .collect();

        if !invalid_tools.is_empty() {
            return Err(anyhow!(
                "The following tools do not exist: {}",
                invalid_tools
                    .iter()
                    .map(|t| format!("'{}'", t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        Ok(())
    }
}

impl AgentTool for SubagentTool {
    type Input = SubagentToolInput;
    type Output = String;

    fn name() -> &'static str {
        acp_thread::SUBAGENT_TOOL_NAME
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
            return Task::ready(Err(anyhow!(
                "Parent thread no longer exists (subagent depth={})",
                self.current_depth + 1
            )));
        };

        let running_count = parent_thread.read(cx).running_subagent_count();
        if running_count >= MAX_PARALLEL_SUBAGENTS {
            return Task::ready(Err(anyhow!(
                "Maximum parallel subagents ({}) reached. Wait for existing subagents to complete.",
                MAX_PARALLEL_SUBAGENTS
            )));
        }

        let parent_model = parent_thread.read(cx).model().cloned();
        let Some(model) = parent_model else {
            return Task::ready(Err(anyhow!("No model configured")));
        };

        let parent_thread_id = parent_thread.read(cx).id().clone();
        let project = self.project.clone();
        let project_context = self.project_context.clone();
        let context_server_registry = self.context_server_registry.clone();
        let templates = self.templates.clone();
        let parent_tools = self.parent_tools.clone();
        let current_depth = self.current_depth;
        let parent_thread_weak = self.parent_thread.clone();

        cx.spawn(async move |cx| {
            let subagent_context = SubagentContext {
                parent_thread_id: parent_thread_id.clone(),
                tool_use_id: LanguageModelToolUseId::from(uuid::Uuid::new_v4().to_string()),
                depth: current_depth + 1,
                summary_prompt: input.summary_prompt.clone(),
                context_low_prompt: input.context_low_prompt.clone(),
            };

            // Determine which tools this subagent gets
            let subagent_tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>> =
                if let Some(ref allowed) = input.allowed_tools {
                    let allowed_set: HashSet<&str> = allowed.iter().map(|s| s.as_str()).collect();
                    parent_tools
                        .iter()
                        .filter(|(name, _)| allowed_set.contains(name.as_ref()))
                        .map(|(name, tool)| (name.clone(), tool.clone()))
                        .collect()
                } else {
                    parent_tools.clone()
                };

            let subagent_thread: Entity<Thread> = cx.new(|cx| {
                Thread::new_subagent(
                    project.clone(),
                    project_context.clone(),
                    context_server_registry.clone(),
                    templates.clone(),
                    model.clone(),
                    subagent_context,
                    subagent_tools,
                    cx,
                )
            });

            let subagent_weak = subagent_thread.downgrade();

            let acp_thread: Entity<AcpThread> = cx.new(|cx| {
                let session_id = subagent_thread.read(cx).id().clone();
                let action_log: Entity<ActionLog> = cx.new(|_| ActionLog::new(project.clone()));
                let connection: Rc<dyn AgentConnection> = Rc::new(SubagentDisplayConnection);
                AcpThread::new(
                    &input.label,
                    connection,
                    project.clone(),
                    action_log,
                    session_id,
                    watch::Receiver::constant(acp::PromptCapabilities::new()),
                    cx,
                )
            });

            event_stream.update_subagent_thread(acp_thread.clone());

            let mut user_stop_rx: watch::Receiver<bool> =
                acp_thread.update(cx, |thread, _| thread.user_stop_receiver());

            if let Some(parent) = parent_thread_weak.upgrade() {
                parent.update(cx, |thread, _cx| {
                    thread.register_running_subagent(subagent_weak.clone());
                });
            }

            // Helper to wait for user stop signal on the subagent card
            let wait_for_user_stop = async {
                loop {
                    if *user_stop_rx.borrow() {
                        return;
                    }
                    if user_stop_rx.changed().await.is_err() {
                        std::future::pending::<()>().await;
                    }
                }
            };

            // Run the subagent, handling cancellation from both:
            // 1. Parent turn cancellation (event_stream.cancelled_by_user)
            // 2. Direct user stop on subagent card (user_stop_rx)
            let result = futures::select! {
                result = run_subagent(
                    &subagent_thread,
                    &acp_thread,
                    input.task_prompt,
                    input.timeout_ms,
                    cx,
                ).fuse() => result,
                _ = event_stream.cancelled_by_user().fuse() => {
                    let _ = subagent_thread.update(cx, |thread, cx| {
                        thread.cancel(cx).detach();
                    });
                    Err(anyhow!("Subagent cancelled by user"))
                }
                _ = wait_for_user_stop.fuse() => {
                    let _ = subagent_thread.update(cx, |thread, cx| {
                        thread.cancel(cx).detach();
                    });
                    Err(anyhow!("Subagent stopped by user"))
                }
            };

            if let Some(parent) = parent_thread_weak.upgrade() {
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
    acp_thread: &Entity<AcpThread>,
    task_prompt: String,
    timeout_ms: Option<u64>,
    cx: &mut AsyncApp,
) -> Result<String> {
    let mut events_rx =
        subagent_thread.update(cx, |thread, cx| thread.submit_user_message(task_prompt, cx))?;

    let acp_thread_weak = acp_thread.downgrade();

    let timed_out = if let Some(timeout) = timeout_ms {
        forward_events_with_timeout(
            &mut events_rx,
            &acp_thread_weak,
            Duration::from_millis(timeout),
            cx,
        )
        .await
    } else {
        forward_events_until_stop(&mut events_rx, &acp_thread_weak, cx).await;
        false
    };

    let should_interrupt =
        timed_out || check_context_low(subagent_thread, CONTEXT_LOW_THRESHOLD, cx);

    if should_interrupt {
        let mut summary_rx =
            subagent_thread.update(cx, |thread, cx| thread.interrupt_for_summary(cx))?;
        forward_events_until_stop(&mut summary_rx, &acp_thread_weak, cx).await;
    } else {
        let mut summary_rx =
            subagent_thread.update(cx, |thread, cx| thread.request_final_summary(cx))?;
        forward_events_until_stop(&mut summary_rx, &acp_thread_weak, cx).await;
    }

    Ok(extract_last_message(subagent_thread, cx))
}

async fn forward_events_until_stop(
    events_rx: &mut mpsc::UnboundedReceiver<Result<ThreadEvent>>,
    acp_thread: &WeakEntity<AcpThread>,
    cx: &mut AsyncApp,
) {
    while let Some(event) = events_rx.next().await {
        match event {
            Ok(ThreadEvent::Stop(_)) => break,
            Ok(event) => {
                forward_event_to_acp_thread(event, acp_thread, cx);
            }
            Err(_) => break,
        }
    }
}

async fn forward_events_with_timeout(
    events_rx: &mut mpsc::UnboundedReceiver<Result<ThreadEvent>>,
    acp_thread: &WeakEntity<AcpThread>,
    timeout: Duration,
    cx: &mut AsyncApp,
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
                Some(Ok(event)) => {
                    forward_event_to_acp_thread(event, acp_thread, cx);
                }
                Some(Err(_)) => return false,
                None => return false,
            },
            Either::Right((_, _)) => return true,
        }
    }
}

fn forward_event_to_acp_thread(
    event: ThreadEvent,
    acp_thread: &WeakEntity<AcpThread>,
    cx: &mut AsyncApp,
) {
    match event {
        ThreadEvent::UserMessage(message) => {
            acp_thread
                .update(cx, |thread, cx| {
                    for content in message.content {
                        thread.push_user_content_block(
                            Some(message.id.clone()),
                            content.into(),
                            cx,
                        );
                    }
                })
                .log_err();
        }
        ThreadEvent::AgentText(text) => {
            acp_thread
                .update(cx, |thread, cx| {
                    thread.push_assistant_content_block(text.into(), false, cx)
                })
                .log_err();
        }
        ThreadEvent::AgentThinking(text) => {
            acp_thread
                .update(cx, |thread, cx| {
                    thread.push_assistant_content_block(text.into(), true, cx)
                })
                .log_err();
        }
        ThreadEvent::ToolCallAuthorization(ToolCallAuthorization {
            tool_call,
            options,
            response,
            ..
        }) => {
            let outcome_task = acp_thread.update(cx, |thread, cx| {
                thread.request_tool_call_authorization(tool_call, options, true, cx)
            });
            if let Ok(Ok(task)) = outcome_task {
                cx.background_spawn(async move {
                    if let acp::RequestPermissionOutcome::Selected(
                        acp::SelectedPermissionOutcome { option_id, .. },
                    ) = task.await
                    {
                        response.send(option_id).ok();
                    }
                })
                .detach();
            }
        }
        ThreadEvent::ToolCall(tool_call) => {
            acp_thread
                .update(cx, |thread, cx| thread.upsert_tool_call(tool_call, cx))
                .log_err();
        }
        ThreadEvent::ToolCallUpdate(update) => {
            acp_thread
                .update(cx, |thread, cx| thread.update_tool_call(update, cx))
                .log_err();
        }
        ThreadEvent::Retry(status) => {
            acp_thread
                .update(cx, |thread, cx| thread.update_retry_status(status, cx))
                .log_err();
        }
        ThreadEvent::Stop(_) => {}
    }
}

fn check_context_low(thread: &Entity<Thread>, threshold: f32, cx: &mut AsyncApp) -> bool {
    thread.read_with(cx, |thread, _| {
        if let Some(usage) = thread.latest_token_usage() {
            let remaining_ratio = 1.0 - (usage.used_tokens as f32 / usage.max_tokens as f32);
            remaining_ratio <= threshold
        } else {
            false
        }
    })
}

fn extract_last_message(thread: &Entity<Thread>, cx: &mut AsyncApp) -> String {
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

struct SubagentDisplayConnection;

impl AgentConnection for SubagentDisplayConnection {
    fn telemetry_id(&self) -> SharedString {
        "subagent".into()
    }

    fn auth_methods(&self) -> &[acp::AuthMethod] {
        &[]
    }

    fn new_thread(
        self: Rc<Self>,
        _project: Entity<Project>,
        _cwd: &Path,
        _cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        unimplemented!("SubagentDisplayConnection does not support new_thread")
    }

    fn authenticate(&self, _method_id: acp::AuthMethodId, _cx: &mut App) -> Task<Result<()>> {
        unimplemented!("SubagentDisplayConnection does not support authenticate")
    }

    fn prompt(
        &self,
        _id: Option<UserMessageId>,
        _params: acp::PromptRequest,
        _cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>> {
        unimplemented!("SubagentDisplayConnection does not support prompt")
    }

    fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {}

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
