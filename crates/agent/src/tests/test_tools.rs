use super::*;
use agent_settings::AgentSettings;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use std::future;
use std::sync::atomic::{AtomicBool, Ordering};

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

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
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

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
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
        let executor = cx.background_executor().clone();
        cx.foreground_executor().spawn(async move {
            executor.timer(Duration::from_millis(input.ms)).await;
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

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "This tool requires permission".into()
    }

    fn run(
        self: Arc<Self>,
        _input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let settings = AgentSettings::get_global(cx);
        let decision = decide_permission_from_settings(Self::name(), "", settings);

        let authorize = match decision {
            ToolPermissionDecision::Allow => None,
            ToolPermissionDecision::Deny(reason) => {
                return Task::ready(Err(anyhow::anyhow!("{}", reason)));
            }
            ToolPermissionDecision::Confirm => {
                let context = crate::ToolPermissionContext {
                    tool_name: "tool_requiring_permission".to_string(),
                    input_value: String::new(),
                };
                Some(event_stream.authorize("Authorize?", context, cx))
            }
        };

        cx.foreground_executor().spawn(async move {
            if let Some(authorize) = authorize {
                authorize.await?;
            }
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

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
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

/// A tool that loops forever but properly handles cancellation via `select!`,
/// similar to how edit_file_tool handles cancellation.
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct CancellationAwareToolInput {}

pub struct CancellationAwareTool {
    pub was_cancelled: Arc<AtomicBool>,
}

impl CancellationAwareTool {
    pub fn new() -> (Self, Arc<AtomicBool>) {
        let was_cancelled = Arc::new(AtomicBool::new(false));
        (
            Self {
                was_cancelled: was_cancelled.clone(),
            },
            was_cancelled,
        )
    }
}

impl AgentTool for CancellationAwareTool {
    type Input = CancellationAwareToolInput;
    type Output = String;

    fn name() -> &'static str {
        "cancellation_aware"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Cancellation Aware Tool".into()
    }

    fn run(
        self: Arc<Self>,
        _input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>> {
        cx.foreground_executor().spawn(async move {
            // Wait for cancellation - this tool does nothing but wait to be cancelled
            event_stream.cancelled_by_user().await;
            self.was_cancelled.store(true, Ordering::SeqCst);
            anyhow::bail!("Tool cancelled by user");
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

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
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
