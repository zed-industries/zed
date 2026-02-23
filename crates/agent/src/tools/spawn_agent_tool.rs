use acp_thread::SUBAGENT_SESSION_ID_META_KEY;
use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task, WeakEntity};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{rc::Rc, time::Duration};

use crate::{AgentTool, Thread, ThreadEnvironment, ToolCallEventStream};

/// Spawns an agent to perform a delegated task.
///
/// Use this tool when you want to do any of the following:
/// - Run multiple tasks in parallel that would take significantly longer to run sequentially.
/// - Complete a self-contained task where you need to know if it succeeded or failed (and how), but none of its intermediate output.
/// - Perform an investigation where all you need to know is the outcome, not the research that led to that outcome.
///
/// Do NOT use this tool for simple tasks you could accomplish directly with one or two tool calls (e.g. reading a file, running a single command). Each agent has startup overhead.
///
/// You control what the agent does by providing a prompt describing what the agent should do. The agent has access to the same tools you do, but does NOT see your conversation history or any context the user attached. You must include all relevant context (file paths, requirements, constraints) in the prompt.
///
/// You will receive only the agent's final message as output.
///
/// If a response (success or error) includes a session_id, you can send a follow-up message to that session by passing the session_id back. This is useful for multi-turn conversations with an agent, asking clarifying questions about its output, or retrying after timeouts or transient failures.
///
/// Note:
/// - Agents cannot use tools you don't have access to.
/// - If spawning multiple agents that might write to the filesystem, provide guidance on how to avoid conflicts (e.g. assign each to different directories).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpawnAgentToolInput {
    /// Short label displayed in the UI while the agent runs (e.g., "Researching alternatives")
    pub label: String,
    /// Describe the task for the agent to perform. Be specific about what you want accomplished. Include all necessary context (file paths, requirements, constraints) since the agent cannot see your conversation.
    pub message: String,
    /// Optional session ID of an existing agent session to continue a conversation with. When provided, the message is sent as a follow-up to that session instead of creating a new one. Use this to ask clarifying questions, request changes based on previous output, or retry after errors.
    #[serde(default)]
    pub session_id: Option<acp::SessionId>,
    /// Optional maximum runtime in seconds. The purpose of this timeout is to prevent the agent from getting stuck in infinite loops, NOT to estimate task duration. Be generous if setting. If not set, the agent runs until it completes or is cancelled.
    #[serde(default)]
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SpawnAgentToolOutput {
    Success {
        session_id: acp::SessionId,
        output: String,
    },
    Error {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        session_id: Option<acp::SessionId>,
        error: String,
    },
}

impl From<SpawnAgentToolOutput> for LanguageModelToolResultContent {
    fn from(output: SpawnAgentToolOutput) -> Self {
        serde_json::to_string(&output)
            .unwrap_or_else(|e| format!("Failed to serialize spawn_agent output: {e}"))
            .into()
    }
}

/// Tool that spawns an agent thread to work on a task.
pub struct SpawnAgentTool {
    parent_thread: WeakEntity<Thread>,
    environment: Rc<dyn ThreadEnvironment>,
}

impl SpawnAgentTool {
    pub fn new(parent_thread: WeakEntity<Thread>, environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self {
            parent_thread,
            environment,
        }
    }
}

impl AgentTool for SpawnAgentTool {
    type Input = SpawnAgentToolInput;
    type Output = SpawnAgentToolOutput;

    const NAME: &'static str = "spawn_agent";

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
            .unwrap_or_else(|_| "Spawning agent".into())
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let Some(parent_thread_entity) = self.parent_thread.upgrade() else {
            return Task::ready(Err(SpawnAgentToolOutput::Error {
                session_id: None,
                error: "Parent thread no longer exists".to_string(),
            }));
        };

        let subagent = if let Some(session_id) = input.session_id {
            self.environment.resume_subagent(
                parent_thread_entity,
                session_id,
                input.message,
                input.timeout_secs.map(Duration::from_secs),
                cx,
            )
        } else {
            self.environment.create_subagent(
                parent_thread_entity,
                input.label,
                input.message,
                input.timeout_secs.map(Duration::from_secs),
                cx,
            )
        };
        let subagent = match subagent {
            Ok(subagent) => subagent,
            Err(err) => {
                return Task::ready(Err(SpawnAgentToolOutput::Error {
                    session_id: None,
                    error: err.to_string(),
                }));
            }
        };
        let subagent_session_id = subagent.id();

        event_stream.subagent_spawned(subagent_session_id.clone());
        let meta = acp::Meta::from_iter([(
            SUBAGENT_SESSION_ID_META_KEY.into(),
            subagent_session_id.to_string().into(),
        )]);
        event_stream.update_fields_with_meta(acp::ToolCallUpdateFields::new(), Some(meta));

        cx.spawn(async move |cx| {
            let output =
                subagent
                    .wait_for_output(cx)
                    .await
                    .map_err(|e| SpawnAgentToolOutput::Error {
                        session_id: Some(subagent_session_id.clone()),
                        error: e.to_string(),
                    })?;
            Ok(SpawnAgentToolOutput::Success {
                session_id: subagent_session_id,
                output,
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
        let session_id = match &output {
            SpawnAgentToolOutput::Success { session_id, .. } => Some(session_id),
            SpawnAgentToolOutput::Error { session_id, .. } => session_id.as_ref(),
        };

        if let Some(session_id) = session_id {
            event_stream.subagent_spawned(session_id.clone());
            let meta = acp::Meta::from_iter([(
                SUBAGENT_SESSION_ID_META_KEY.into(),
                session_id.to_string().into(),
            )]);
            event_stream.update_fields_with_meta(acp::ToolCallUpdateFields::new(), Some(meta));
        }

        Ok(())
    }
}
