use crate::schema::json_schema_for;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolCard, ToolResult, ToolUseStatus};
use component::Component;
use futures::{
    AsyncBufReadExt, SinkExt, StreamExt,
    channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded},
    io::BufReader,
    stream::SelectAll,
};
use gpui::{AnyElement, AnyWindowHandle, App, AppContext, Entity, Task, WeakEntity, Window};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::Path, process::Stdio, sync::Arc, time::Duration};
use ui::{ComponentScope, IconName, RegisterComponent, prelude::*};
use util::{command::new_smol_command, get_system_shell, markdown::MarkdownString};
use workspace::Workspace;

const COMMAND_OUTPUT_LIMIT: usize = 16 * 1024;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalToolInput {
    /// The one-liner command to execute.
    command: String,
    /// Working directory for the command. This must be one of the root directories of the project.
    cd: String,
}

#[derive(RegisterComponent)]
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
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input: TerminalToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let input_path = Path::new(&input.cd);
        let working_dir = match working_dir(cx, &input, &project, input_path) {
            Ok(dir) => dir,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let (line_sender, line_receiver) = unbounded();

        let output = spawn_command_and_stream(working_dir, input.command, line_sender, cx);
        let output = match output {
            Ok(ok) => ok,
            Err(err) => return Task::ready(Err(err)).into(),
        };

        let card = cx.new(|cx| TerminalToolCard::new(line_receiver, cx));

        ToolResult {
            output,
            card: Some(card.into()),
        }
    }
}

/// Run a command until completion and return the output.
///
/// Also stream each line through a channel that can be accessed via the returned
/// receiver, the channel will only receive updates if the future is awaited.
fn spawn_command_and_stream(
    working_dir: Arc<Path>,
    command: String,
    mut line_sender: UnboundedSender<Result<String>>,
    cx: &mut App,
) -> Result<Task<Result<String>>> {
    let shell = get_system_shell();

    let mut cmd = new_smol_command(&shell)
        .args(["-c", &command])
        .current_dir(working_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to execute terminal command")?;

    let mut line_stream = SelectAll::new();
    line_stream.push(
        BufReader::new(cmd.stdout.take().context("Failed to get stdout")?)
            .lines()
            .boxed(),
    );
    line_stream.push(
        BufReader::new(cmd.stderr.take().context("Failed to get stderr")?)
            .lines()
            .boxed(),
    );

    let fut = cx.background_spawn(async move {
        let mut combined_output = String::with_capacity(COMMAND_OUTPUT_LIMIT + 1);

        while let Some(line) = line_stream.next().await {
            let line = match line {
                Ok(line) => line,
                Err(err) => {
                    let err = format!("Failed to read line: {err}");
                    // TODO: unwrap
                    line_sender.send(Err(anyhow!(err.clone()))).await.unwrap();
                    return Err(anyhow!(err));
                }
            };
            let truncated = combined_output.len() + line.len() > COMMAND_OUTPUT_LIMIT;

            let line = if truncated {
                let remaining_capacity = COMMAND_OUTPUT_LIMIT.saturating_sub(combined_output.len());
                &line[..remaining_capacity]
            } else {
                &line
            };

            combined_output.push_str(line);
            combined_output.push('\n');
            // TODO: unwrap
            line_sender
                .send(Ok(line.to_owned()))
                .await
                .context("Failed to send terminal output text")
                .unwrap();

            if truncated {
                // TODO
                break;
            }
        }

        Ok(output_block(&combined_output))
    });

    Ok(fut)

    // drop((out_line, err_line));

    // let truncated = combined_buffer.len() > COMMAND_OUTPUT_LIMIT;
    // combined_buffer.truncate(COMMAND_OUTPUT_LIMIT);

    // consume_reader(out_reader, truncated).await?;
    // consume_reader(err_reader, truncated).await?;

    // let status = cmd.status().await.context("Failed to get command status")?;

    // let output_string = if truncated {
    //     // Valid to find `\n` in UTF-8 since 0-127 ASCII characters are not used in
    //     // multi-byte characters.
    //     let last_line_ix = combined_buffer.bytes().rposition(|b| b == b'\n');
    //     let combined_buffer = &combined_buffer[..last_line_ix.unwrap_or(combined_buffer.len())];

    //     format!(
    //         "Command output too long. The first {} bytes:\n\n{}",
    //         combined_buffer.len(),
    //         output_block(&combined_buffer),
    //     )
    // } else {
    //     output_block(&combined_buffer)
    // };

    // let output_with_status = if status.success() {
    //     if output_string.is_empty() {
    //         "Command executed successfully.".to_string()
    //     } else {
    //         output_string.to_string()
    //     }
    // } else {
    //     format!(
    //         "Command failed with exit code {} (shell: {}).\n\n{}",
    //         status.code().unwrap_or(-1),
    //         shell,
    //         output_string,
    //     )
    // };

    // Ok(output_with_status)
}

fn working_dir(
    cx: &mut App,
    input: &TerminalToolInput,
    project: &Entity<Project>,
    input_path: &Path,
) -> Result<Arc<Path>, &'static str> {
    let project = project.read(cx);
    if input.cd == "." {
        // Accept "." as meaning "the one worktree" if we only have one worktree.
        let mut worktrees = project.worktrees(cx);

        let only_worktree = match worktrees.next() {
            Some(worktree) => worktree,
            None => return Err("No worktrees found in the project"),
        };

        if worktrees.next().is_some() {
            return Err(
                "'.' is ambiguous in multi-root workspaces. Please specify a root directory explicitly.",
            );
        }

        Ok(only_worktree.read(cx).abs_path())
    } else if input_path.is_absolute() {
        // Absolute paths are allowed, but only if they're in one of the project's worktrees.
        if !project
            .worktrees(cx)
            .any(|worktree| input_path.starts_with(&worktree.read(cx).abs_path()))
        {
            return Err("The absolute path must be within one of the project's worktrees");
        }

        Ok(input_path.into())
    } else {
        let Some(worktree) = project.worktree_for_root_name(&input.cd, cx) else {
            return Err("`cd` directory {} not found in the project");
        };

        Ok(worktree.read(cx).abs_path())
    }
}

fn output_block(output: &str) -> String {
    format!(
        "```\n{}{}```",
        output,
        if output.ends_with('\n') { "" } else { "\n" }
    )
}

struct TerminalToolCard {
    read_failed: bool,
    combined_contents: String,
    _task: Task<()>,
}

impl TerminalToolCard {
    fn new(mut line_receiver: UnboundedReceiver<Result<String>>, cx: &mut Context<Self>) -> Self {
        let _task = cx.spawn(async move |this, cx| {
            while let Some(line) = line_receiver.next().await {
                let is_entity_released = this
                    .update(cx, |card, cx| {
                        let line = match line {
                            Ok(line) => line,
                            // TODO: don't we need to log these??
                            Err(_) => {
                                card.read_failed = true;
                                return; // stop receiving
                            }
                        };

                        card.combined_contents += &line;
                        cx.notify();
                    })
                    .is_err();

                if is_entity_released {
                    return;
                }
            }
        });

        Self {
            read_failed: false,
            combined_contents: String::new(),
            _task,
        }
    }
}

impl ToolCard for TerminalToolCard {
    fn render(
        &mut self,
        _status: &ToolUseStatus,
        _window: &mut Window,
        _workspace: WeakEntity<Workspace>,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        format!("text: {}", self.combined_contents)
    }
}

impl Component for TerminalTool {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn initial_state(_cx: &mut App) -> Self::InitialState {
        ()
    }

    fn preview(_state: &mut (), window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        enum TerminalToolCardPreviewOperation {
            Sleep(u64),
            SendLine(&'static str),
        }

        use TerminalToolCardPreviewOperation::*;

        const OPERATIONS: &[TerminalToolCardPreviewOperation] = &[
            SendLine("$ ./imaginary-script.sh"),
            Sleep(100),
            SendLine(""),
            Sleep(200),
            SendLine("  This"),
            Sleep(16),
            SendLine("  takes"),
            Sleep(1000),
            SendLine("  LONG"),
            Sleep(100),
            SendLine("  to"),
            Sleep(300),
            SendLine("  finish."),
        ];

        let (mut tx, rx) = unbounded();
        let executor = cx.background_executor().clone();
        let ccccard = cx.new(|cx| TerminalToolCard::new(rx, cx));

        cx.background_spawn(async move {
            for operation in OPERATIONS {
                match operation {
                    &Sleep(millis) => executor.timer(Duration::from_millis(millis)).await,
                    &SendLine(line) => {
                        let _ = tx.send(Ok(line.to_owned())).await;
                    }
                }
            }
        })
        .detach();

        // TODO: add one where it receives a read failure.
        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group(vec![single_example(
                    "No failures (todo naming)",
                    div()
                        .size_full()
                        .child(ccccard.update(cx, |tool, cx| {
                            tool.render(
                                &ToolUseStatus::Pending,
                                window,
                                WeakEntity::new_invalid(),
                                cx,
                            )
                            .into_any_element()
                        }))
                        .into_any_element(),
                )])])
                .into_any_element(),
        )
    }
}

#[cfg(test)]
#[cfg(not(windows))]
mod tests {

    // #[gpui::test(iterations = 10)]
    // async fn test_run_command_simple(cx: &mut TestAppContext) {
    //     cx.executor().allow_parking();

    //     let result =
    //         spawn_command_and_stream(Path::new(".").into(), "echo 'Hello, World!'".to_string())
    //             .await;

    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), "```\nHello, World!\n```");
    // }

    // #[gpui::test(iterations = 10)]
    // async fn test_interleaved_stdout_stderr(cx: &mut TestAppContext) {
    //     cx.executor().allow_parking();

    //     let command = "echo 'stdout 1' && sleep 0.01 && echo 'stderr 1' >&2 && sleep 0.01 && echo 'stdout 2' && sleep 0.01 && echo 'stderr 2' >&2";
    //     let result = spawn_command_and_stream(Path::new(".").into(), command.to_string()).await;

    //     assert!(result.is_ok());
    //     assert_eq!(
    //         result.unwrap(),
    //         "```\nstdout 1\nstderr 1\nstdout 2\nstderr 2\n```"
    //     );
    // }

    // #[gpui::test(iterations = 10)]
    // async fn test_multiple_output_reads(cx: &mut TestAppContext) {
    //     cx.executor().allow_parking();

    //     // Command with multiple outputs that might require multiple reads
    //     let result = spawn_command_and_stream(
    //         Path::new(".").into(),
    //         "echo '1'; sleep 0.01; echo '2'; sleep 0.01; echo '3'".to_string(),
    //     )
    //     .await;

    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), "```\n1\n2\n3\n```");
    // }

    // #[gpui::test(iterations = 10)]
    // async fn test_output_truncation_single_line(cx: &mut TestAppContext) {
    //     cx.executor().allow_parking();

    //     let cmd = format!(
    //         "echo '{}'; sleep 0.01;",
    //         "X".repeat(COMMAND_OUTPUT_LIMIT * 2)
    //     );

    //     let result = spawn_command_and_stream(Path::new(".").into(), cmd).await;

    //     assert!(result.is_ok());
    //     let output = result.unwrap();

    //     let content_start = output.find("```\n").map(|i| i + 4).unwrap_or(0);
    //     let content_end = output.rfind("\n```").unwrap_or(output.len());
    //     let content_length = content_end - content_start;

    //     // Output should be exactly the limit
    //     assert_eq!(content_length, COMMAND_OUTPUT_LIMIT);
    // }

    // #[gpui::test(iterations = 10)]
    // async fn test_output_truncation_multiline(cx: &mut TestAppContext) {
    //     cx.executor().allow_parking();

    //     let cmd = format!("echo '{}'; ", "X".repeat(120)).repeat(160);
    //     let result = spawn_command_and_stream(Path::new(".").into(), cmd).await;

    //     assert!(result.is_ok());
    //     let output = result.unwrap();

    //     assert!(output.starts_with("Command output too long. The first 16334 bytes:\n\n"));

    //     let content_start = output.find("```\n").map(|i| i + 4).unwrap_or(0);
    //     let content_end = output.rfind("\n```").unwrap_or(output.len());
    //     let content_length = content_end - content_start;

    //     assert!(content_length <= COMMAND_OUTPUT_LIMIT);
    // }

    // #[gpui::test(iterations = 10)]
    // async fn test_command_failure(cx: &mut TestAppContext) {
    //     cx.executor().allow_parking();

    //     let result = spawn_command_and_stream(Path::new(".").into(), "exit 42".to_string()).await;

    //     assert!(result.is_ok());
    //     let output = result.unwrap();

    //     // Extract the shell name from path for cleaner test output
    //     let shell_path = std::env::var("SHELL").unwrap_or("bash".to_string());

    //     let expected_output = format!(
    //         "Command failed with exit code 42 (shell: {}).\n\n```\n\n```",
    //         shell_path
    //     );
    //     assert_eq!(output, expected_output);
    // }
}
