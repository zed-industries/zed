use anyhow::{anyhow, Context as _, Result};
use assistant_tool::Tool;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::command::new_smol_command;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BashToolInput {
    /// The bash command to execute as a one-liner.
    ///
    /// WARNING: you must not `cd` into the working directory, as that's already
    /// taken care of automatically. Doing so will cause the command to fail!
    command: String,
    /// Working directory for the command. This must be one of the root directories of the project.
    working_directory: String,
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
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input: BashToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let Some(worktree) = project
            .read(cx)
            .worktree_for_root_name(&input.working_directory, cx)
        else {
            return Task::ready(Err(anyhow!("Working directory not found in the project")));
        };
        let working_directory = worktree.read(cx).abs_path();

        cx.spawn(|_| async move {
            // Add 2>&1 to merge stderr into stdout for proper interleaving.
            let command = format!("{} 2>&1", input.command);

            let output = new_smol_command("bash")
                .arg("-c")
                .arg(&command)
                .current_dir(working_directory)
                .output()
                .await
                .context("Failed to execute bash command")?;

            let output_string = String::from_utf8_lossy(&output.stdout).to_string();

            if output.status.success() {
                if output_string.is_empty() {
                    Ok("Command executed successfully.".to_string())
                } else {
                    Ok(output_string)
                }
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
