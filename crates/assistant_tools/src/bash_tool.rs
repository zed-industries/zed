use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BashToolInput {
    /// The bash command to execute as a one-liner.
    command: String,
}

pub struct BashTool;

impl Tool for BashTool {
    fn name(&self) -> String {
        "bash".to_string()
    }

    fn description(&self) -> String {
        include_str!("./bash_tool/description.md").to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(BashToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input: BashToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        cx.spawn(|_| async move {
            // Add 2>&1 to merge stderr into stdout for proper interleaving
            let command = format!("{} 2>&1", input.command);

            // Spawn a blocking task to execute the command
            let output = futures::executor::block_on(async {
                std::process::Command::new("bash")
                    .arg("-c")
                    .arg(&command)
                    .output()
                    .map_err(|err| anyhow!("Failed to execute bash command: {}", err))
            })?;

            let output_string = String::from_utf8_lossy(&output.stdout).to_string();

            if output.status.success() {
                Ok(output_string)
            } else {
                Ok(format!(
                    "Command failed with exit code {}\n{}",
                    output.status.code().unwrap_or(-1),
                    &output_string
                ))
            }
        })
    }
}
