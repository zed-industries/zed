use std::sync::Arc;

use agent::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Runs EXPLAIN or EXPLAIN ANALYZE on a SQL query to show the execution plan.
/// Use this tool to understand how a database will execute a query, identify
/// potential performance issues, and optimize query plans.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExplainQueryToolInput {
    /// The SQL query to explain.
    pub sql: String,
    /// The name of the database connection to use.
    pub connection: String,
    /// If true, runs EXPLAIN ANALYZE which actually executes the query and
    /// shows real execution statistics. If false, shows the estimated plan only.
    #[serde(default)]
    pub analyze: bool,
}

pub struct ExplainQueryTool;

impl AgentTool for ExplainQueryTool {
    type Input = ExplainQueryToolInput;
    type Output = String;

    const NAME: &'static str = "database_explain_query";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            let mode = if input.analyze {
                "EXPLAIN ANALYZE"
            } else {
                "EXPLAIN"
            };
            format!("{mode} on `{}`", input.connection).into()
        } else {
            "Explain query".into()
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
            database_core::explain_query_core(&input.sql, &input.connection, input.analyze)
        })
    }
}
