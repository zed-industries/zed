use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool, ToolWorkingSet};
use futures::future::join_all;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
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
    /// The tool call groups to invoke sequentially. Within each group, the tools will be invoked concurrently.
    ///
    /// <example>
    /// To batch together calls to get_weather and get_time, where those both run concurrently:
    ///
    /// ```
    /// [
    ///     {
    ///       "name": "get_weather",
    ///       "arguments": "{\"location\": \"San Francisco, CA\"}"
    ///     },
    ///     {
    ///       "name": "get_time",
    ///       "arguments": "{\"location\": \"San Francisco, CA\"}"
    ///     }
    /// ]
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
        "batch-tool".into()
    }

    fn needs_confirmation(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./batch_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Cog
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(BatchToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<BatchToolInput>(input.clone()) {
            Ok(input) => {
                let count = input.invocations.len();
                let concurrently = if input.run_tools_concurrently {
                    "concurrently"
                } else {
                    "sequentially"
                };

                let first_tool_name = input.invocations.first().map(|inv| inv.name.clone()).unwrap_or_default();

                let all_same = input.invoications.iter().all(|invocation| invocation.name == first_tool_name);

                let tool_name = if !tool_name.is_empty() {
                    format!("Run {} '{}' tools {}", count, tool_name, concurrently)
                } else {
                    format!("Run {count} tools {concurrently}")
                }
            }
            Err(_) => "Batch: Run tools".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<BatchToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        if input.invocations.is_empty() {
            return Task::ready(Err(anyhow!("No tool invocations provided")));
        }

        let working_set = ToolWorkingSet::default();
        let invocations = input.invocations;
        let messages = messages.to_vec();

        cx.spawn(async move |cx| {
            let mut tasks = Vec::new();
            let mut tool_names = Vec::new();

            // First collect all tools from the working set
            for invocation in invocations {
                let tool_name = invocation.name.clone();
                tool_names.push(tool_name.clone());

                // Look up the tool in the registry
                let tool = cx
                    .update(|cx| working_set.tool(&tool_name, cx))
                    .map_err(|err| anyhow!("Failed to look up tool '{}': {}", tool_name, err))?;

                let Some(tool) = tool else {
                    return Err(anyhow!("Tool '{}' not found", tool_name));
                };

                let project = project.clone();
                let action_log = action_log.clone();
                let messages = messages.clone();

                // Create tasks for each tool invocation
                let task = cx
                    .update(|cx| tool.run(invocation.input, &messages, project, action_log, cx))
                    .map_err(|err| anyhow!("Failed to start tool '{}': {}", tool_name, err))?;

                tasks.push(task);
            }

            let mut results = Vec::with_capacity(tasks.len());

            if input.run_tools_concurrently {
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
    }
}
