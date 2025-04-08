use crate::schema::json_schema_for;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool};
use futures::io::BufReader;
use futures::{AsyncBufReadExt, AsyncReadExt};
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

            let mut cmd = new_smol_command("bash")
                .arg("-c")
                .arg(&command)
                .current_dir(working_dir)
                .stdout(std::process::Stdio::piped())
                .spawn()
                .context("Failed to execute bash command")?;

            // Capture stdout with a limit
            let stdout = cmd.stdout.take().unwrap();
            let mut reader = BufReader::new(stdout);

            const MESSAGE_1: &str = "Command output too long. The first ";
            const MESSAGE_2: &str = " bytes:\n\n";
            const ERR_MESSAGE_1: &str = "Command failed with exit code ";
            const ERR_MESSAGE_2: &str = "\n\n";

            const STDOUT_LIMIT: usize = 8192;

            const LIMIT: usize = STDOUT_LIMIT
                - (MESSAGE_1.len()
                    + (STDOUT_LIMIT.ilog10() as usize + 1) // byte count
                    + MESSAGE_2.len()
                    + ERR_MESSAGE_1.len()
                    + 3 // status code
                    + ERR_MESSAGE_2.len());

            // Read one more byte to determine whether the output was truncated
            let mut buffer = vec![0; LIMIT + 1];
            let bytes_read = reader.read(&mut buffer).await?;

            // Repeatedly fill the output reader's buffer without copying it.
            loop {
                let skipped_bytes = reader.fill_buf().await?;
                if skipped_bytes.is_empty() {
                    break;
                }
                let skipped_bytes_len = skipped_bytes.len();
                reader.consume_unpin(skipped_bytes_len);
            }

            let output_bytes = &buffer[..bytes_read];

            // Let the process continue running
            let status = cmd.status().await.context("Failed to get command status")?;

            let output_string = if bytes_read > LIMIT {
                // Valid to find `\n` in UTF-8 since 0-127 ASCII characters are not used in
                // multi-byte characters.
                let last_line_ix = output_bytes.iter().rposition(|b| *b == b'\n');
                let output_string = String::from_utf8_lossy(
                    &output_bytes[..last_line_ix.unwrap_or(output_bytes.len())],
                );

                format!(
                    "{}{}{}{}",
                    MESSAGE_1,
                    output_string.len(),
                    MESSAGE_2,
                    output_string
                )
            } else {
                String::from_utf8_lossy(&output_bytes).into()
            };

            let output_with_status = if status.success() {
                if output_string.is_empty() {
                    "Command executed successfully.".to_string()
                } else {
                    output_string.to_string()
                }
            } else {
                format!(
                    "{}{}{}{}",
                    ERR_MESSAGE_1,
                    status.code().unwrap_or(-1),
                    ERR_MESSAGE_2,
                    output_string,
                )
            };

            debug_assert!(output_with_status.len() <= STDOUT_LIMIT);

            Ok(output_with_status)
        })
    }
}
