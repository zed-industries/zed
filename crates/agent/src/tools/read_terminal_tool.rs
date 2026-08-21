use agent_client_protocol::schema::v1 as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;

use crate::{AgentTool, ThreadEnvironment, ToolCallEventStream, ToolInput};

/// Reads the current contents of a terminal pane by its id, so the agent can
/// act on command output or errors the user is looking at without re-running
/// anything.
///
/// This is read-only: it does not execute commands or modify the terminal.
///
/// To discover which terminals are available, use the `list_terminals` tool,
/// which returns each terminal's id and title.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ReadTerminalToolInput {
    /// The id of the terminal to read. Obtain it from `list_terminals`.
    pub terminal_id: String,
    /// Return only the first N lines of the terminal buffer. Avoid requesting
    /// too many lines, or the response may waste tokens or exceed the context window.
    #[serde(default)]
    pub head_lines: Option<u32>,
    /// Return only the last N lines of the terminal buffer. Avoid requesting
    /// too many lines, or the response may waste tokens or exceed the context window.
    #[serde(default)]
    pub tail_lines: Option<u32>,
}

/// Reads the current contents of a terminal pane.
pub struct ReadTerminalTool {
    environment: Rc<dyn ThreadEnvironment>,
}

impl ReadTerminalTool {
    pub fn new(environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self { environment }
    }
}

impl AgentTool for ReadTerminalTool {
    type Input = ReadTerminalToolInput;
    type Output = String;

    const NAME: &'static str = "read_terminal";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn allow_in_restricted_mode() -> bool {
        true
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Read Terminal".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|e| e.to_string())?;
            let content = self
                .environment
                .read_terminal(&input.terminal_id, input.head_lines, input.tail_lines, cx)
                .await
                .map_err(|e| e.to_string())?;
            Ok(content)
        })
    }
}

/// Returns the terminals currently open in the workspace, so the agent can
/// reference them with `read_terminal`.
///
/// Each terminal is identified by a stable id and shown to the user by its
/// title; pass the id to `read_terminal` to read its contents.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ListTerminalsToolInput {}

/// Lists the terminals currently open in the workspace.
pub struct ListTerminalsTool {
    environment: Rc<dyn ThreadEnvironment>,
}

impl ListTerminalsTool {
    pub fn new(environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self { environment }
    }
}

impl AgentTool for ListTerminalsTool {
    type Input = ListTerminalsToolInput;
    type Output = String;

    const NAME: &'static str = "list_terminals";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn allow_in_restricted_mode() -> bool {
        true
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "List Terminals".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx| {
            let _ = input.recv().await.map_err(|e| e.to_string())?;
            let terminals = self
                .environment
                .list_terminals(cx)
                .await
                .map_err(|e| e.to_string())?;
            let serialized = serde_json::to_string_pretty(&terminals)
                .map_err(|e| e.to_string())?;
            Ok(serialized)
        })
    }
}
