use std::sync::Arc;

use agent::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Returns detailed schema information about a database object (table or view),
/// including columns, types, primary keys, foreign keys, and indexes.
/// Use this tool when you need to understand the structure of a specific table
/// before writing queries.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DescribeObjectToolInput {
    /// The name of the table or view to describe.
    pub object_name: String,
    /// The name of the database connection to use.
    pub connection: String,
}

pub struct DescribeObjectTool;

impl AgentTool for DescribeObjectTool {
    type Input = DescribeObjectToolInput;
    type Output = String;

    const NAME: &'static str = "database_describe_object";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Describe `{}` on `{}`", input.object_name, input.connection).into()
        } else {
            "Describe database object".into()
        }
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
                .map_err(|error| format!("Failed to receive tool input: {error}"))?;
            database_core::describe_object_core(&input.object_name, &input.connection)
        })
    }
}
