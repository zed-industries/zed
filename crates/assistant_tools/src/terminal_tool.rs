use crate::schema::json_schema_for;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use futures::io::BufReader;
use futures::{AsyncBufReadExt, AsyncReadExt, FutureExt};
use gpui::{App, AppContext, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future;
use util::get_system_shell;

use std::path::Path;
use std::sync::Arc;
use ui::IconName;
use util::command::new_smol_command;
use util::markdown::MarkdownString;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalToolInput {
    /// The one-liner command to execute.
    command: String,
    /// Working directory for the command. This must be one of the root directories of the project.
    cd: String,
}

pub struct TerminalTool;

impl Tool for TerminalTool {
    fn name(&self) -> String {
        "terminal".to_string()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./terminal_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Terminal
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<TerminalToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<TerminalToolInput>(input.clone()) {
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
            Err(_) => "Run terminal command".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> ToolResult {
        let input: TerminalToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let project = project.read(cx);
        let input_path = Path::new(&input.cd);
        let working_dir = if input.cd == "." {
            // Accept "." as meaning "the one worktree" if we only have one worktree.
            let mut worktrees = project.worktrees(cx);

            let only_worktree = match worktrees.next() {
                Some(worktree) => worktree,
                None => {
                    return Task::ready(Err(anyhow!("No worktrees found in the project"))).into();
                }
            };

            if worktrees.next().is_some() {
                return Task::ready(Err(anyhow!(
                    "'.' is ambiguous in multi-root workspaces. Please specify a root directory explicitly."
                ))).into();
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
                )))
                .into();
            }

            input_path.into()
        } else {
            let Some(worktree) = project.worktree_for_root_name(&input.cd, cx) else {
                return Task::ready(Err(anyhow!(
                    "`cd` directory {} not found in the project",
                    &input.cd
                )))
                .into();
            };

            worktree.read(cx).abs_path()
        };

        cx.background_spawn(run_command_limited(working_dir, input.command))
            .into()
    }
}

const LIMIT: usize = 16 * 1024;

async fn run_command_limited(working_dir: Arc<Path>, command: String) -> Result<String> {
    let shell = get_system_shell();

    let mut cmd = new_smol_command(&shell)
        .arg("-c")
        .arg(&command)
        .current_dir(working_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .context("Failed to execute terminal command")?;

    let mut combined_buffer = String::with_capacity(LIMIT + 1);

    let mut out_reader = BufReader::new(cmd.stdout.take().context("Failed to get stdout")?);
    let mut out_tmp_buffer = String::with_capacity(512);
    let mut err_reader = BufReader::new(cmd.stderr.take().context("Failed to get stderr")?);
    let mut err_tmp_buffer = String::with_capacity(512);

    let mut out_line = Box::pin(
        out_reader
            .read_line(&mut out_tmp_buffer)
            .left_future()
            .fuse(),
    );
    let mut err_line = Box::pin(
        err_reader
            .read_line(&mut err_tmp_buffer)
            .left_future()
            .fuse(),
    );

    let mut has_stdout = true;
    let mut has_stderr = true;
    while (has_stdout || has_stderr) && combined_buffer.len() < LIMIT + 1 {
        futures::select_biased! {
            read = out_line => {
                drop(out_line);
                combined_buffer.extend(out_tmp_buffer.drain(..));
                if read? == 0 {
                    out_line = Box::pin(future::pending().right_future().fuse());
                    has_stdout = false;
                } else {
                    out_line = Box::pin(out_reader.read_line(&mut out_tmp_buffer).left_future().fuse());
                }
            }
            read = err_line => {
                drop(err_line);
                combined_buffer.extend(err_tmp_buffer.drain(..));
                if read? == 0 {
                    err_line = Box::pin(future::pending().right_future().fuse());
                    has_stderr = false;
                } else {
                    err_line = Box::pin(err_reader.read_line(&mut err_tmp_buffer).left_future().fuse());
                }
            }
        };
    }

    drop((out_line, err_line));

    let truncated = combined_buffer.len() > LIMIT;
    combined_buffer.truncate(LIMIT);

    consume_reader(out_reader, truncated).await?;
    consume_reader(err_reader, truncated).await?;

    let status = cmd.status().await.context("Failed to get command status")?;

    let output_string = if truncated {
        // Valid to find `\n` in UTF-8 since 0-127 ASCII characters are not used in
        // multi-byte characters.
        let last_line_ix = combined_buffer.bytes().rposition(|b| b == b'\n');
        let combined_buffer = &combined_buffer[..last_line_ix.unwrap_or(combined_buffer.len())];

        format!(
            "Command output too long. The first {} bytes:\n\n{}",
            combined_buffer.len(),
            output_block(&combined_buffer),
        )
    } else {
        output_block(&combined_buffer)
    };

    let output_with_status = if status.success() {
        if output_string.is_empty() {
            "Command executed successfully.".to_string()
        } else {
            output_string.to_string()
        }
    } else {
        format!(
            "Command failed with exit code {} (shell: {}).\n\n{}",
            status.code().unwrap_or(-1),
            shell,
            output_string,
        )
    };

    Ok(output_with_status)
}

async fn consume_reader<T: AsyncReadExt + Unpin>(
    mut reader: BufReader<T>,
    truncated: bool,
) -> Result<(), std::io::Error> {
    loop {
        let skipped_bytes = reader.fill_buf().await?;
        if skipped_bytes.is_empty() {
            break;
        }
        let skipped_bytes_len = skipped_bytes.len();
        reader.consume_unpin(skipped_bytes_len);

        // Should only skip if we went over the limit
        debug_assert!(truncated);
    }
    Ok(())
}

fn output_block(output: &str) -> String {
    format!(
        "```\n{}{}```",
        output,
        if output.ends_with('\n') { "" } else { "\n" }
    )
}

#[cfg(test)]
#[cfg(not(windows))]
mod tests {
    use gpui::TestAppContext;

    use super::*;

    #[gpui::test(iterations = 10)]
    async fn test_run_command_simple(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let result =
            run_command_limited(Path::new(".").into(), "echo 'Hello, World!'".to_string()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "```\nHello, World!\n```");
    }

    #[gpui::test(iterations = 10)]
    async fn test_interleaved_stdout_stderr(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let command = "echo 'stdout 1' && sleep 0.01 && echo 'stderr 1' >&2 && sleep 0.01 && echo 'stdout 2' && sleep 0.01 && echo 'stderr 2' >&2";
        let result = run_command_limited(Path::new(".").into(), command.to_string()).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "```\nstdout 1\nstderr 1\nstdout 2\nstderr 2\n```"
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_multiple_output_reads(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        // Command with multiple outputs that might require multiple reads
        let result = run_command_limited(
            Path::new(".").into(),
            "echo '1'; sleep 0.01; echo '2'; sleep 0.01; echo '3'".to_string(),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "```\n1\n2\n3\n```");
    }

    #[gpui::test(iterations = 10)]
    async fn test_output_truncation_single_line(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let cmd = format!("echo '{}'; sleep 0.01;", "X".repeat(LIMIT * 2));

        let result = run_command_limited(Path::new(".").into(), cmd).await;

        assert!(result.is_ok());
        let output = result.unwrap();

        let content_start = output.find("```\n").map(|i| i + 4).unwrap_or(0);
        let content_end = output.rfind("\n```").unwrap_or(output.len());
        let content_length = content_end - content_start;

        // Output should be exactly the limit
        assert_eq!(content_length, LIMIT);
    }

    #[gpui::test(iterations = 10)]
    async fn test_output_truncation_multiline(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let cmd = format!("echo '{}'; ", "X".repeat(120)).repeat(160);
        let result = run_command_limited(Path::new(".").into(), cmd).await;

        assert!(result.is_ok());
        let output = result.unwrap();

        assert!(output.starts_with("Command output too long. The first 16334 bytes:\n\n"));

        let content_start = output.find("```\n").map(|i| i + 4).unwrap_or(0);
        let content_end = output.rfind("\n```").unwrap_or(output.len());
        let content_length = content_end - content_start;

        assert!(content_length <= LIMIT);
    }

    #[gpui::test(iterations = 10)]
    async fn test_command_failure(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let result = run_command_limited(Path::new(".").into(), "exit 42".to_string()).await;

        assert!(result.is_ok());
        let output = result.unwrap();

        // Extract the shell name from path for cleaner test output
        let shell_path = std::env::var("SHELL").unwrap_or("bash".to_string());

        let expected_output = format!(
            "Command failed with exit code 42 (shell: {}).\n\n```\n\n```",
            shell_path
        );
        assert_eq!(output, expected_output);
    }
}
