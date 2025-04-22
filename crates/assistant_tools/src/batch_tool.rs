use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult, ToolWorkingSet};
use futures::future::join_all;
use gpui::{AnyWindowHandle, App, AppContext, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ToolInvocation {
    /// The name of the tool to invoke
    pub name: String,

    /// The input to the tool in JSON format
    pub input: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BatchToolInput {
    /// The tool invocations to run as a batch. These tools will be run either sequentially
    /// or concurrently depending on the `run_tools_concurrently` flag.
    ///
    /// <example>
    /// Basic file operations (concurrent)
    ///
    /// ```json
    /// {
    ///   "invocations": [
    ///     {
    ///       "name": "read_file",
    ///       "input": {
    ///         "path": "src/main.rs"
    ///       }
    ///     },
    ///     {
    ///       "name": "list_directory",
    ///       "input": {
    ///         "path": "src/lib"
    ///       }
    ///     },
    ///     {
    ///       "name": "grep",
    ///       "input": {
    ///         "regex": "fn run\\("
    ///       }
    ///     }
    ///   ],
    ///   "run_tools_concurrently": true
    /// }
    /// ```
    /// </example>
    ///
    /// <example>
    /// Multiple find-replace operations on the same file (sequential)
    ///
    /// ```json
    /// {
    ///   "invocations": [
    ///     {
    ///       "name": "find_replace_file",
    ///       "input": {
    ///         "path": "src/config.rs",
    ///         "display_description": "Update default timeout value",
    ///         "find": "pub const DEFAULT_TIMEOUT: u64 = 30;\n\npub const MAX_RETRIES: u32 = 3;\n\npub const SERVER_URL: &str = \"https://api.example.com\";",
    ///         "replace": "pub const DEFAULT_TIMEOUT: u64 = 60;\n\npub const MAX_RETRIES: u32 = 3;\n\npub const SERVER_URL: &str = \"https://api.example.com\";"
    ///       }
    ///     },
    ///     {
    ///       "name": "find_replace_file",
    ///       "input": {
    ///         "path": "src/config.rs",
    ///         "display_description": "Update API endpoint URL",
    ///         "find": "pub const MAX_RETRIES: u32 = 3;\n\npub const SERVER_URL: &str = \"https://api.example.com\";\n\npub const API_VERSION: &str = \"v1\";",
    ///         "replace": "pub const MAX_RETRIES: u32 = 3;\n\npub const SERVER_URL: &str = \"https://api.newdomain.com\";\n\npub const API_VERSION: &str = \"v1\";"
    ///       }
    ///     }
    ///   ],
    ///   "run_tools_concurrently": false
    /// }
    /// ```
    /// </example>
    ///
    /// <example>
    /// Searching and analyzing code (concurrent)
    ///
    /// ```json
    /// {
    ///   "invocations": [
    ///     {
    ///       "name": "grep",
    ///       "input": {
    ///         "regex": "impl Database"
    ///       }
    ///     },
    ///     {
    ///       "name": "path_search",
    ///       "input": {
    ///         "glob": "**/*test*.rs"
    ///       }
    ///     }
    ///   ],
    ///   "run_tools_concurrently": true
    /// }
    /// ```
    /// </example>
    ///
    /// <example>
    /// Multi-file refactoring (concurrent)
    ///
    /// ```json
    /// {
    ///   "invocations": [
    ///     {
    ///       "name": "find_replace_file",
    ///       "input": {
    ///         "path": "src/models/user.rs",
    ///         "display_description": "Add email field to User struct",
    ///         "find": "pub struct User {\n    pub id: u64,\n    pub username: String,\n    pub created_at: DateTime<Utc>,\n}",
    ///         "replace": "pub struct User {\n    pub id: u64,\n    pub username: String,\n    pub email: String,\n    pub created_at: DateTime<Utc>,\n}"
    ///       }
    ///     },
    ///     {
    ///       "name": "find_replace_file",
    ///       "input": {
    ///         "path": "src/db/queries.rs",
    ///         "display_description": "Update user insertion query",
    ///         "find": "pub async fn insert_user(conn: &mut Connection, user: &User) -> Result<(), DbError> {\n    conn.execute(\n        \"INSERT INTO users (id, username, created_at) VALUES ($1, $2, $3)\",\n        &[&user.id, &user.username, &user.created_at],\n    ).await?;\n    \n    Ok(())\n}",
    ///         "replace": "pub async fn insert_user(conn: &mut Connection, user: &User) -> Result<(), DbError> {\n    conn.execute(\n        \"INSERT INTO users (id, username, email, created_at) VALUES ($1, $2, $3, $4)\",\n        &[&user.id, &user.username, &user.email, &user.created_at],\n    ).await?;\n    \n    Ok(())\n}"
    ///       }
    ///     }
    ///   ],
    ///   "run_tools_concurrently": true
    /// }
    /// ```
    /// </example>
    pub invocations: Vec<ToolInvocation>,

    /// Whether to run the tools in this batch concurrently. If this is false (the default), the tools will run sequentially.
    #[serde(default)]
    pub run_tools_concurrently: bool,
}

pub struct BatchTool;

impl Tool for BatchTool {
    fn name(&self) -> String {
        "batch_tool".into()
    }

    fn needs_confirmation(&self, input: &serde_json::Value, cx: &App) -> bool {
        serde_json::from_value::<BatchToolInput>(input.clone())
            .map(|input| {
                let working_set = ToolWorkingSet::default();
                input.invocations.iter().any(|invocation| {
                    working_set
                        .tool(&invocation.name, cx)
                        .map_or(false, |tool| tool.needs_confirmation(&invocation.input, cx))
                })
            })
            .unwrap_or(false)
    }

    fn description(&self) -> String {
        include_str!("./batch_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Cog
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<BatchToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<BatchToolInput>(input.clone()) {
            Ok(input) => {
                let count = input.invocations.len();
                let mode = if input.run_tools_concurrently {
                    "concurrently"
                } else {
                    "sequentially"
                };

                let first_tool_name = input
                    .invocations
                    .first()
                    .map(|inv| inv.name.clone())
                    .unwrap_or_default();

                let all_same = input
                    .invocations
                    .iter()
                    .all(|invocation| invocation.name == first_tool_name);

                if all_same {
                    format!(
                        "Run `{}` {} times {}",
                        first_tool_name,
                        input.invocations.len(),
                        mode
                    )
                } else {
                    format!("Run {} tools {}", count, mode)
                }
            }
            Err(_) => "Batch tools".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<BatchToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        if input.invocations.is_empty() {
            return Task::ready(Err(anyhow!("No tool invocations provided"))).into();
        }

        let run_tools_concurrently = input.run_tools_concurrently;

        let foreground_task = {
            let working_set = ToolWorkingSet::default();
            let invocations = input.invocations;
            let messages = messages.to_vec();

            cx.spawn(async move |cx| {
                let mut tasks = Vec::new();
                let mut tool_names = Vec::new();

                for invocation in invocations {
                    let tool_name = invocation.name.clone();
                    tool_names.push(tool_name.clone());

                    let tool = cx
                        .update(|cx| working_set.tool(&tool_name, cx))
                        .map_err(|err| {
                            anyhow!("Failed to look up tool '{}': {}", tool_name, err)
                        })?;

                    let Some(tool) = tool else {
                        return Err(anyhow!("Tool '{}' not found", tool_name));
                    };

                    let project = project.clone();
                    let action_log = action_log.clone();
                    let messages = messages.clone();
                    let tool_result = cx
                        .update(|cx| {
                            tool.run(
                                invocation.input,
                                &messages,
                                project,
                                action_log,
                                window.clone(),
                                cx,
                            )
                        })
                        .map_err(|err| anyhow!("Failed to start tool '{}': {}", tool_name, err))?;

                    tasks.push(tool_result.output);
                }

                Ok((tasks, tool_names))
            })
        };

        cx.background_spawn(async move {
            let (tasks, tool_names) = foreground_task.await?;
            let mut results = Vec::with_capacity(tasks.len());

            if run_tools_concurrently {
                results.extend(join_all(tasks).await)
            } else {
                for task in tasks {
                    results.push(task.await);
                }
            };

            let mut formatted_results = String::new();
            let mut error_occurred = false;

            for (i, result) in results.into_iter().enumerate() {
                let tool_name = &tool_names[i];

                match result {
                    Ok(output) => {
                        formatted_results
                            .push_str(&format!("Tool '{}' result:\n{}\n\n", tool_name, output));
                    }
                    Err(err) => {
                        error_occurred = true;
                        formatted_results
                            .push_str(&format!("Tool '{}' error: {}\n\n", tool_name, err));
                    }
                }
            }

            if error_occurred {
                formatted_results
                    .push_str("Note: Some tool invocations failed. See individual results above.");
            }

            Ok(formatted_results.trim().to_string())
        })
        .into()
    }
}
