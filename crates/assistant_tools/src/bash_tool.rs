use crate::schema::json_schema_for;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;
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

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> serde_json::Value {
        json_schema_for::<BashToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<BashToolInput>(input.clone()) {
            Ok(input) => {
                let mut lines = input.command.lines();
                let first_line = lines.next().unwrap_or_default();
                let remaining_line_count = lines.count();
                match remaining_line_count {
                    0 => MarkdownString::inline_code(&first_line).0,
                    1 => {
                        MarkdownString::inline_code(&format!(
                            "{} - {} more line",
                            first_line, remaining_line_count
                        ))
                        .0
                    }
                    n => {
                        MarkdownString::inline_code(&format!("{} - {} more lines", first_line, n)).0
                    }
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

        let project = project.read(cx);
        let input_path = Path::new(&input.cd);
        let working_dir = if input.cd == "." {
            // Accept "." as meaning "the one worktree" if we only have one worktree.
            let mut worktrees = project.worktrees(cx);

            let only_worktree = match worktrees.next() {
                Some(worktree) => worktree,
                None => return Task::ready(Err(anyhow!("No worktrees found in the project"))),
            };

            if worktrees.next().is_some() {
                return Task::ready(Err(anyhow!(
                    "'.' is ambiguous in multi-root workspaces. Please specify a root directory explicitly."
                )));
            }

            only_worktree.read(cx).abs_path()
        } else if input_path.is_absolute() {
            // Absolute paths are allowed, but only if they're in one of the project's worktrees.
            if !project
                .worktrees(cx)
                .any(|worktree| input_path.starts_with(&worktree.read(cx).abs_path()))
            {
                return Task::ready(Err(anyhow!(
                    "The absolute path must be within one of the project's worktrees"
                )));
            }

            input_path.into()
        } else {
            let Some(worktree) = project.worktree_for_root_name(&input.cd, cx) else {
                return Task::ready(Err(anyhow!(
                    "`cd` directory {} not found in the project",
                    &input.cd
                )));
            };

            worktree.read(cx).abs_path()
        };

        cx.spawn(async move |_| {
            // Add 2>&1 to merge stderr into stdout for proper interleaving.
            let command = format!("({}) 2>&1", input.command);

            let output = new_smol_command("bash")
                .arg("-c")
                .arg(&command)
                .current_dir(working_dir)
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
