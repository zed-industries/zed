use std::sync::Arc;

use agent::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Lists all database objects (tables and views) in a database connection.
/// Use this tool to discover what tables and views are available in a database
/// before querying or describing them. If no connection name is provided,
/// lists all available connections and their objects.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListObjectsToolInput {
    /// The name of the database connection to use.
    pub connection: String,
    /// Filter by object type: "tables", "views", or "all" (default).
    #[serde(default = "default_object_type")]
    pub object_type: Option<String>,
}

fn default_object_type() -> Option<String> {
    Some("all".to_string())
}

pub struct ListObjectsTool;

impl AgentTool for ListObjectsTool {
    type Input = ListObjectsToolInput;
    type Output = String;

    const NAME: &'static str = "database_list_objects";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("List objects in `{}`", input.connection).into()
        } else {
            "List database objects".into()
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
            database_core::list_objects_core(
                &input.connection,
                input.object_type.as_deref(),
            )
        })
    }
}
