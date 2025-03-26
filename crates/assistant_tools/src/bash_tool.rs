use anyhow::{anyhow, Context as _, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;
use util::command::new_smol_command;
use util::markdown::MarkdownString;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BashToolInput {
    /// The bash command to execute as a one-liner.
    command: String,
    /// Working directory for the command. This must be one of the root directories of the project.
    cd: String,
}

pub struct BashTool;

impl Tool for BashTool {
    fn name(&self) -> String {
        "bash".to_string()
    }

    fn needs_confirmation(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./bash_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Terminal
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(BashToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<BashToolInput>(input.clone()) {
            Ok(input) => {
                if input.command.contains('\n') {
                    MarkdownString::code_block("bash", &input.command).0
                } else {
                    MarkdownString::inline_code(&input.command).0
                }
            }
            Err(_) => "Run bash command".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input: BashToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let Some(worktree) = project.read(cx).worktree_for_root_name(&input.cd, cx) else {
            return Task::ready(Err(anyhow!("Working directory not found in the project")));
        };
        let working_directory = worktree.read(cx).abs_path();

        cx.spawn(async move |_| {
            // Add 2>&1 to merge stderr into stdout for proper interleaving.
            let command = format!("({}) 2>&1", input.command);

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
