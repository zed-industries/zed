use super::*;
use gpui::{App, SharedString, Task};
use std::future;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// A streaming tool that echoes its input, used to test streaming tool
/// lifecycle (e.g. partial delivery and cleanup when the LLM stream ends
/// before `is_input_complete`).
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct StreamingEchoToolInput {
    /// The text to echo.
    pub text: String,
}

pub struct StreamingEchoTool {
    wait_until_complete_rx: Mutex<Option<oneshot::Receiver<()>>>,
}

impl StreamingEchoTool {
    pub fn new() -> Self {
        Self {
            wait_until_complete_rx: Mutex::new(None),
        }
    }

    pub fn with_wait_until_complete(mut self, receiver: oneshot::Receiver<()>) -> Self {
        self.wait_until_complete_rx = Mutex::new(Some(receiver));
        self
    }
}

impl AgentTool for StreamingEchoTool {
    type Input = StreamingEchoToolInput;
    type Output = String;

    const NAME: &'static str = "streaming_echo";

    fn supports_input_streaming() -> bool {
        true
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Streaming Echo".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        let wait_until_complete_rx = self.wait_until_complete_rx.lock().unwrap().take();
        cx.spawn(async move |_cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;
            if let Some(rx) = wait_until_complete_rx {
                rx.await.ok();
            }
            Ok(input.text)
        })
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct StreamingJsonErrorContextToolInput {
    /// The text to echo.
    pub text: String,
}

pub struct StreamingJsonErrorContextTool;

impl AgentTool for StreamingJsonErrorContextTool {
    type Input = StreamingJsonErrorContextToolInput;
    type Output = String;

    const NAME: &'static str = "streaming_json_error_context";

    fn supports_input_streaming() -> bool {
        true
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Streaming JSON Error Context".into()
    }

    fn run(
        self: Arc<Self>,
        mut input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        cx.spawn(async move |_cx| {
            let mut last_partial_text = None;

            loop {
                match input.next().await {
                    Ok(ToolInputPayload::Partial(partial)) => {
                        if let Some(text) = partial.get("text").and_then(|value| value.as_str()) {
                            last_partial_text = Some(text.to_string());
                        }
                    }
                    Ok(ToolInputPayload::Full(input)) => return Ok(input.text),
                    Ok(ToolInputPayload::InvalidJson { error_message }) => {
                        let partial_text = last_partial_text.unwrap_or_default();
                        return Err(format!(
                            "Saw partial text '{partial_text}' before invalid JSON: {error_message}"
                        ));
                    }
                    Err(error) => {
                        return Err(format!("Failed to receive tool input: {error}"));
                    }
                }
            }
        })
    }
}

/// A streaming tool that echoes its input, used to test streaming tool
/// lifecycle (e.g. partial delivery and cleanup when the LLM stream ends
/// before `is_input_complete`).
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct StreamingFailingEchoToolInput {
    /// The text to echo.
    pub text: String,
}

pub struct StreamingFailingEchoTool {
    pub receive_chunks_until_failure: usize,
}

impl AgentTool for StreamingFailingEchoTool {
    type Input = StreamingFailingEchoToolInput;

    type Output = String;

    const NAME: &'static str = "streaming_failing_echo";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn supports_input_streaming() -> bool {
        true
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "echo".into()
    }

    fn run(
        self: Arc<Self>,
        mut input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |_cx| {
            for _ in 0..self.receive_chunks_until_failure {
                let _ = input.next().await;
            }
            Err("failed".into())
        })
    }
}

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

    const NAME: &'static str = "echo";

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
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        cx.spawn(async move |_cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;
            Ok(input.text)
        })
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

    const NAME: &'static str = "delay";

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
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>>
    where
        Self: Sized,
    {
        let executor = cx.background_executor().clone();
        cx.foreground_executor().spawn(async move {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;
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

    const NAME: &'static str = "tool_requiring_permission";

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
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        cx.spawn(async move |cx| {
            let _input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            let authorize = cx.update(|cx| {
                let context = crate::ToolPermissionContext::new(Self::NAME, vec![String::new()]);
                event_stream.authorize("Authorize?", context, cx)
            });
            authorize.await.map_err(|e| e.to_string())?;
            Ok("Allowed".to_string())
        })
    }
}

/// A second tool that also requires permission, used to verify that
/// permission decisions scoped to one tool don't leak into prompts for a
/// different tool.
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ToolRequiringPermission2Input {}

pub struct ToolRequiringPermission2;

impl AgentTool for ToolRequiringPermission2 {
    type Input = ToolRequiringPermission2Input;
    type Output = String;

    const NAME: &'static str = "tool_requiring_permission_2";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "This tool also requires permission".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        cx.spawn(async move |cx| {
            let _input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            let authorize = cx.update(|cx| {
                let context = crate::ToolPermissionContext::new(Self::NAME, vec![String::new()]);
                event_stream.authorize("Authorize?", context, cx)
            });
            authorize.await.map_err(|e| e.to_string())?;
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

    const NAME: &'static str = "infinite";

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
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        cx.foreground_executor().spawn(async move {
            let _input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;
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

    const NAME: &'static str = "cancellation_aware";

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
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        cx.foreground_executor().spawn(async move {
            let _input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;
            // Wait for cancellation - this tool does nothing but wait to be cancelled
            event_stream.cancelled_by_user().await;
            self.was_cancelled.store(true, Ordering::SeqCst);
            Err("Tool cancelled by user".to_string())
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

    const NAME: &'static str = "word_list";

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
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        cx.spawn(async move |_cx| {
            let _input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;
            Ok("ok".to_string())
        })
    }
}
