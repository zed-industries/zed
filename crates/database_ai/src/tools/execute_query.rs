use std::sync::Arc;

use agent::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Executes a SQL query against a named database connection and returns the results.
/// Use this tool when you need to read data from a database. The results are returned
/// as a markdown table for readability. Use the `connection` parameter to specify
/// which database connection to use. If unsure which connections are available,
/// use the `database_list_objects` tool first.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteQueryToolInput {
    /// The SQL query to execute. Should be a SELECT or other read-only statement.
    pub sql: String,
    /// The name of the database connection to use.
    pub connection: String,
    /// Maximum number of rows to return. Defaults to 100 if not specified.
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    100
}

pub struct ExecuteQueryTool;

impl AgentTool for ExecuteQueryTool {
    type Input = ExecuteQueryToolInput;
    type Output = String;

    const NAME: &'static str = "database_execute_query";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Execute query on `{}`", input.connection).into()
        } else {
            "Execute database query".into()
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
            database_core::execute_query_core(&input.sql, &input.connection, input.limit)
        })
    }
}
