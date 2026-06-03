use std::sync::Arc;

use agent_client_protocol::schema as acp;
use futures::FutureExt as _;
use gpui::{App, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::SharedString;

use crate::{AgentTool, ToolCallEventStream, ToolInput, ToolInputPayload};

/// Displays a message directly to the user, bypassing summarization.
/// Use this for deliverables, progress updates, or any content the user must see verbatim.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SendToUserToolInput {
    /// The message to display to the user.
    pub message: String,
}

/// Partial form of [`SendToUserToolInput`], used to surface the message as it
/// streams in so the UI can render it incrementally.
#[derive(Debug, Deserialize)]
struct SendToUserToolPartialInput {
    message: Option<String>,
}

pub struct SendToUserTool;

impl SendToUserTool {
    fn message_content(message: &str) -> acp::ToolCallUpdateFields {
        acp::ToolCallUpdateFields::new().content(vec![acp::ToolCallContent::Content(
            acp::Content::new(acp::ContentBlock::Text(acp::TextContent::new(
                message.to_string(),
            ))),
        )])
    }
}

impl AgentTool for SendToUserTool {
    type Input = SendToUserToolInput;
    type Output = String;

    const NAME: &'static str = "send_to_user";

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
        "Message to user".into()
    }

    fn run(
        self: Arc<Self>,
        mut input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |_cx| {
            // This tool only surfaces content the model already produced, so it must
            // never be discarded on interruption. Whatever has streamed in is already
            // on screen, so we report success on cancellation (or if the input stream
            // closes mid-turn) rather than erroring out and dropping the message.
            loop {
                futures::select! {
                    payload = input.next().fuse() => {
                        match payload {
                            // Stream the message as it arrives so the UI renders it incrementally.
                            Ok(ToolInputPayload::Partial(partial)) => {
                                if let Ok(parsed) =
                                    serde_json::from_value::<SendToUserToolPartialInput>(partial)
                                    && let Some(message) = parsed.message
                                {
                                    event_stream.update_fields(Self::message_content(&message));
                                }
                            }
                            Ok(ToolInputPayload::Full(input)) => {
                                event_stream.update_fields(Self::message_content(&input.message));
                                return Ok("ok".to_string());
                            }
                            Ok(ToolInputPayload::InvalidJson { error_message }) => {
                                return Err(error_message);
                            }
                            // Input stream closed (e.g. the turn was canceled mid-stream):
                            // keep what already streamed and succeed.
                            Err(_) => return Ok("ok".to_string()),
                        }
                    }
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Ok("ok".to_string());
                    }
                }
            }
        })
    }

    fn replay(
        &self,
        input: Self::Input,
        _output: Self::Output,
        event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> anyhow::Result<()> {
        event_stream.update_fields(Self::message_content(&input.message));
        Ok(())
    }
}
