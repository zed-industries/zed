use super::*;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use std::future;

/// A tool that echoes its input
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct EchoToolInput {
    /// The text to echo.
    pub text: String,
}

pub struct EchoTool;

impl AgentTool for EchoTool {
    type Input = EchoToolInput;
    type Output = String;

    fn name() -> &'static str {
        "echo"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(&self, _input: Result<Self::Input, serde_json::Value>) -> SharedString {
        "Echo".into()
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<String>> {
        Task::ready(Ok(input.text))
    }
}

/// A tool that waits for a specified delay
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct DelayToolInput {
    /// The delay in milliseconds.
    ms: u64,
}

pub struct DelayTool;

impl AgentTool for DelayTool {
    type Input = DelayToolInput;
    type Output = String;

    fn name() -> &'static str {
        "delay"
    }

    fn initial_title(&self, input: Result<Self::Input, serde_json::Value>) -> SharedString {
        if let Ok(input) = input {
            format!("Delay {}ms", input.ms).into()
        } else {
            "Delay".into()
        }
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>>
    where
        Self: Sized,
    {
        cx.foreground_executor().spawn(async move {
            smol::Timer::after(Duration::from_millis(input.ms)).await;
            Ok("Ding".to_string())
        })
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ToolRequiringPermissionInput {}

pub struct ToolRequiringPermission;

impl AgentTool for ToolRequiringPermission {
    type Input = ToolRequiringPermissionInput;
    type Output = String;

    fn name() -> &'static str {
        "tool_requiring_permission"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(&self, _input: Result<Self::Input, serde_json::Value>) -> SharedString {
        "This tool requires permission".into()
    }

    fn run(
        self: Arc<Self>,
        _input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let authorize = event_stream.authorize("Authorize?", cx);
        cx.foreground_executor().spawn(async move {
            authorize.await?;
            Ok("Allowed".to_string())
        })
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct InfiniteToolInput {}

pub struct InfiniteTool;

impl AgentTool for InfiniteTool {
    type Input = InfiniteToolInput;
    type Output = String;

    fn name() -> &'static str {
        "infinite"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(&self, _input: Result<Self::Input, serde_json::Value>) -> SharedString {
        "Infinite Tool".into()
    }

    fn run(
        self: Arc<Self>,
        _input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>> {
        cx.foreground_executor().spawn(async move {
            future::pending::<()>().await;
            unreachable!()
        })
    }
}

/// A tool that takes an object with map from letters to random words starting with that letter.
/// All fiealds are required! Pass a word for every letter!
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct WordListInput {
    /// Provide a random word that starts with A.
    a: Option<String>,
    /// Provide a random word that starts with B.
    b: Option<String>,
    /// Provide a random word that starts with C.
    c: Option<String>,
    /// Provide a random word that starts with D.
    d: Option<String>,
    /// Provide a random word that starts with E.
    e: Option<String>,
    /// Provide a random word that starts with F.
    f: Option<String>,
    /// Provide a random word that starts with G.
    g: Option<String>,
}

pub struct WordListTool;

impl AgentTool for WordListTool {
    type Input = WordListInput;
    type Output = String;

    fn name() -> &'static str {
        "word_list"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(&self, _input: Result<Self::Input, serde_json::Value>) -> SharedString {
        "List of random words".into()
    }

    fn run(
        self: Arc<Self>,
        _input: Self::Input,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<String>> {
        Task::ready(Ok("ok".to_string()))
    }
}
