use acp_thread::SUBAGENT_SESSION_ID_META_KEY;
use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;

use crate::{AgentTool, ThreadEnvironment, ToolCallEventStream, ToolInput};

/// Spawns an agent to perform a delegated task.
///
/// Use this tool when you want to:
/// - Run multiple tasks in parallel.
/// - Delegate a self-contained task where you only need the final outcome.
///
/// You will receive only the agent's final message as output.
///
/// **New session** (no session_id): Creates a new agent that does NOT see your conversation history. Include all relevant context (file paths, requirements, constraints) in the message.
///
/// **Follow-up** (with session_id): Sends a follow-up to an existing agent session. The agent already has full context, so send only a short, direct message — do NOT repeat the original task or context. Examples: "Also update the tests", "Fix the compile error in foo.rs", "Retry".
///
/// - If spawning multiple agents that might write to the filesystem, provide guidance on how to avoid conflicts (e.g. assign each to different directories).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpawnAgentToolInput {
    /// Short label displayed in the UI while the agent runs (e.g., "Researching alternatives")
    pub label: String,
    /// The prompt for the agent. For new sessions, include full context needed for the task. For follow-ups (with session_id), you can rely on the agent already having the previous message.
    pub message: String,
    /// Session ID of an existing agent session to continue instead of creating a new one.
    #[serde(default)]
    pub session_id: Option<acp::SessionId>,
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
    environment: Rc<dyn ThreadEnvironment>,
}

impl SpawnAgentTool {
    pub fn new(environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self { environment }
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
        match input {
            Ok(i) => i.label.into(),
            Err(value) => value
                .get("label")
                .and_then(|v| v.as_str())
                .map(|s| SharedString::from(s.to_owned()))
                .unwrap_or_else(|| "Spawning agent".into()),
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| SpawnAgentToolOutput::Error {
                    session_id: None,
                    error: format!("Failed to receive tool input: {e}"),
                })?;

            let (subagent, subagent_session_id) = cx.update(|cx| {
                let subagent = if let Some(session_id) = input.session_id {
                    self.environment.resume_subagent(session_id, cx)
                } else {
                    self.environment.create_subagent(input.label, cx)
                };
                let subagent = subagent.map_err(|err| SpawnAgentToolOutput::Error {
                    session_id: None,
                    error: err.to_string(),
                })?;
                let subagent_session_id = subagent.id();

                event_stream.subagent_spawned(subagent_session_id.clone());
                let meta = acp::Meta::from_iter([(
                    SUBAGENT_SESSION_ID_META_KEY.into(),
                    subagent_session_id.to_string().into(),
                )]);
                event_stream.update_fields_with_meta(acp::ToolCallUpdateFields::new(), Some(meta));

                Ok((subagent, subagent_session_id))
            })?;

            match subagent.send(input.message, cx).await {
                Ok(output) => {
                    event_stream.update_fields(
                        acp::ToolCallUpdateFields::new().content(vec![output.clone().into()]),
                    );
                    Ok(SpawnAgentToolOutput::Success {
                        session_id: subagent_session_id,
                        output,
                    })
                }
                Err(e) => {
                    let error = e.to_string();
                    // workaround for now because the agent loop will always mark this as ToolCallStatus::Failed
                    let canceled = error == "User canceled";
                    event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![
                        acp::ToolCallContent::Content(acp::Content::new(error.clone()).meta(
                            acp::Meta::from_iter([("cancelled".into(), canceled.into())]),
                        )),
                    ]));
                    Err(SpawnAgentToolOutput::Error {
                        session_id: Some(subagent_session_id),
                        error,
                    })
                }
            }
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

        let content = match &output {
            SpawnAgentToolOutput::Success { output, .. } => output.into(),
            SpawnAgentToolOutput::Error { error, .. } => error.into(),
        };
        event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![content]));

        Ok(())
    }
}
