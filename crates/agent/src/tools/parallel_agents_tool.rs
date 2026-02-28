use agent_client_protocol as acp;
use anyhow::Result;
use futures::future::join_all;
use gpui::{App, AsyncApp, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;

use crate::{AgentTool, SubagentHandle, ThreadEnvironment, ToolCallEventStream, ToolInput};

/// Spawns multiple independent agents in parallel and collects all their results.
///
/// Use this when a task can be split into self-contained subtasks with no ordering
/// dependencies. Each agent starts fresh with no conversation history — include all
/// context it needs inside `message`.
///
/// If agents write to the filesystem, assign each to a separate directory to avoid
/// conflicts. For tasks that depend on each other, use sequential `spawn_agent` calls.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParallelAgentsToolInput {
    /// The independent subtasks to run simultaneously.
    pub tasks: Vec<ParallelAgentTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParallelAgentTask {
    /// Short label shown in the UI (e.g. "Write tests for auth module").
    pub label: String,
    /// Full prompt for this agent. Include every file path, requirement, and constraint.
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelAgentsToolOutput {
    pub results: Vec<ParallelAgentResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelAgentResult {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl From<ParallelAgentsToolOutput> for LanguageModelToolResultContent {
    fn from(output: ParallelAgentsToolOutput) -> Self {
        serde_json::to_string(&output)
            .unwrap_or_else(|e| format!("Failed to serialize parallel_agents output: {e}"))
            .into()
    }
}

pub struct ParallelAgentsTool {
    environment: Rc<dyn ThreadEnvironment>,
}

impl ParallelAgentsTool {
    pub fn new(environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self { environment }
    }
}

impl AgentTool for ParallelAgentsTool {
    type Input = ParallelAgentsToolInput;
    type Output = ParallelAgentsToolOutput;

    const NAME: &'static str = "parallel_agents";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        let n = match &input {
            Ok(i) => i.tasks.len(),
            Err(v) => v
                .get("tasks")
                .and_then(|t| t.as_array())
                .map(|a| a.len())
                .unwrap_or(0),
        };
        match n {
            1 => "Running 1 agent".into(),
            n => format!("Running {n} agents in parallel").into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |mut cx: AsyncApp| {
            let input = input.recv().await.map_err(|e| ParallelAgentsToolOutput {
                results: vec![ParallelAgentResult {
                    label: "setup".into(),
                    output: None,
                    error: Some(format!("Failed to receive tool input: {e}")),
                }],
            })?;

            if input.tasks.is_empty() {
                return Ok(ParallelAgentsToolOutput { results: vec![] });
            }

            // Create all subagents synchronously on the main thread before going async.
            let mut spawned: Vec<(String, String, Result<Rc<dyn SubagentHandle>>)> =
                Vec::with_capacity(input.tasks.len());

            cx.update(|cx| {
                for task in input.tasks {
                    let result = self.environment.create_subagent(task.label.clone(), cx);
                    spawned.push((task.label, task.message, result));
                }
            })
            .map_err(|e| ParallelAgentsToolOutput {
                results: vec![ParallelAgentResult {
                    label: "setup".into(),
                    output: None,
                    error: Some(format!("App context dropped: {e}")),
                }],
            })?;

            // Drive all send() calls concurrently.
            let results = join_all(spawned.into_iter().map(|(label, message, creation)| {
                let cx = cx.clone();
                async move {
                    match creation {
                        Err(e) => ParallelAgentResult {
                            label,
                            output: None,
                            error: Some(format!("Failed to create agent: {e}")),
                        },
                        Ok(subagent) => match subagent.send(message, &cx).await {
                            Ok(output) => ParallelAgentResult {
                                label,
                                output: Some(output),
                                error: None,
                            },
                            Err(e) => ParallelAgentResult {
                                label,
                                output: None,
                                error: Some(e.to_string()),
                            },
                        },
                    }
                }
            }))
            .await;

            Ok(ParallelAgentsToolOutput { results })
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Result<()> {
        let content = serde_json::to_string(&output)
            .unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
            .into();
        event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![content]));
        Ok(())
    }
}
