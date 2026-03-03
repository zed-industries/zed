use std::sync::Arc;

use agent::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Returns the full DDL (CREATE TABLE/VIEW statements) for a database connection's schema.
/// Use this tool to understand the complete database structure including all tables, columns,
/// types, constraints, indexes, and foreign key relationships. You can optionally specify
/// specific table names to get DDL for only those tables. This is the best tool to use
/// when you need full schema context before writing queries or analyzing the database design.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetSchemaToolInput {
    /// The name of the database connection to use.
    pub connection: String,
    /// Optional list of specific table names to get DDL for.
    /// If empty or not provided, returns DDL for all tables.
    #[serde(default)]
    pub tables: Vec<String>,
}

pub struct GetSchemaTool;

impl AgentTool for GetSchemaTool {
    type Input = GetSchemaToolInput;
    type Output = String;

    const NAME: &'static str = "database_get_schema";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            if input.tables.is_empty() {
                format!("Get full schema for `{}`", input.connection).into()
            } else {
                format!(
                    "Get schema for {} in `{}`",
                    input.tables.join(", "),
                    input.connection
                )
                .into()
            }
        } else {
            "Get database schema".into()
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
            database_core::get_schema_core(&input.connection, &input.tables)
        })
    }
}
