use std::sync::Arc;

use agent_client_protocol::schema as acp;
use chrono::{Local, Utc};
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::deserialize_maybe_stringified;
use crate::{AgentTool, ToolCallEventStream, ToolInput};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum Timezone {
    #[serde(alias = "UTC", alias = "Utc")]
    Utc,
    #[serde(alias = "LOCAL", alias = "Local")]
    Local,
}

/// Returns the current datetime in RFC 3339 format.
/// Only use this tool when the user specifically asks for it or the current task would benefit from knowing the current datetime.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NowToolInput {
    /// The timezone to use for the datetime. Use `utc` for UTC, or `local` for the system's local time.
    #[serde(deserialize_with = "deserialize_maybe_stringified")]
    timezone: Timezone,
}

pub struct NowTool;

impl AgentTool for NowTool {
    type Input = NowToolInput;
    type Output = String;

    const NAME: &'static str = "now";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Get current time".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        cx.spawn(async move |_cx| {
            let input = input.recv().await.map_err(|e| e.to_string())?;
            let now = match input.timezone {
                Timezone::Utc => Utc::now().to_rfc3339(),
                Timezone::Local => Local::now().to_rfc3339(),
            };
            Ok(format!("The current datetime is {now}."))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use serde_json::json;

    #[gpui::test]
    async fn test_stringified_timezone_input_succeeds(cx: &mut TestAppContext) {
        let tool = Arc::new(NowTool);
        let (mut sender, input) = ToolInput::<NowToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        sender.send_full(json!({
            "timezone": "\"utc\""
        }));

        let result = task.await.unwrap();
        assert!(
            result.starts_with("The current datetime is "),
            "unexpected output: {result}"
        );
    }
}
