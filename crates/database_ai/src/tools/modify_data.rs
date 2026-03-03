use std::sync::Arc;

use agent::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Executes a data modification SQL statement (INSERT, UPDATE, DELETE, CREATE, ALTER, DROP)
/// against a named database connection. Returns the number of affected rows.
/// This tool requires user confirmation before execution because it modifies data.
/// For read-only queries, use the `database_execute_query` tool instead.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ModifyDataToolInput {
    /// The SQL statement to execute. Should be an INSERT, UPDATE, DELETE,
    /// CREATE, ALTER, DROP, or other data modification statement.
    pub sql: String,
    /// The name of the database connection to use.
    pub connection: String,
}

pub struct ModifyDataTool;

impl AgentTool for ModifyDataTool {
    type Input = ModifyDataToolInput;
    type Output = String;

    const NAME: &'static str = "database_modify_data";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Edit
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Modify data on `{}`", input.connection).into()
        } else {
            "Modify database data".into()
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
            database_core::modify_data_core(&input.sql, &input.connection)
        })
    }
}
