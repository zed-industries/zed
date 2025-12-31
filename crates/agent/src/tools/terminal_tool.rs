use agent_client_protocol as acp;
use anyhow::Result;
use futures::FutureExt as _;
use gpui::{App, AppContext, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use util::markdown::MarkdownInlineCode;

use crate::{AgentTool, ThreadEnvironment, ToolCallEventStream};

const COMMAND_OUTPUT_LIMIT: u64 = 16 * 1024;

/// Executes a shell one-liner and returns the combined output.
///
/// This tool spawns a process using the user's shell, reads from stdout and stderr (preserving the order of writes), and returns a string with the combined output result.
///
/// The output results will be shown to the user already, only list it again if necessary, avoid being redundant.
///
/// Make sure you use the `cd` parameter to navigate to one of the root directories of the project. NEVER do it as part of the `command` itself, otherwise it will error.
///
/// Do not use this tool for commands that run indefinitely, such as servers (like `npm run start`, `npm run dev`, `python -m http.server`, etc) or file watchers that don't terminate on their own.
///
/// For potentially long-running commands, prefer specifying `timeout_ms` to bound runtime and prevent indefinite hangs.
///
/// Remember that each invocation of this tool will spawn a new shell process, so you can't rely on any state from previous invocations.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalToolInput {
    /// The one-liner command to execute.
    pub command: String,
    /// Working directory for the command. This must be one of the root directories of the project.
    pub cd: String,
    /// Optional maximum runtime (in milliseconds). If exceeded, the running terminal task is killed.
    pub timeout_ms: Option<u64>,
}

pub struct TerminalTool {
    project: Entity<Project>,
    environment: Rc<dyn ThreadEnvironment>,
}

impl TerminalTool {
    pub fn new(project: Entity<Project>, environment: Rc<dyn ThreadEnvironment>) -> Self {
        Self {
            project,
            environment,
        }
    }
}

impl AgentTool for TerminalTool {
    type Input = TerminalToolInput;
    type Output = String;

    fn name() -> &'static str {
        "terminal"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Execute
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            let mut lines = input.command.lines();
            let first_line = lines.next().unwrap_or_default();
            let remaining_line_count = lines.count();
            match remaining_line_count {
                0 => MarkdownInlineCode(first_line).to_string().into(),
                1 => MarkdownInlineCode(&format!(
                    "{} - {} more line",
                    first_line, remaining_line_count
                ))
                .to_string()
                .into(),
                n => MarkdownInlineCode(&format!("{} - {} more lines", first_line, n))
                    .to_string()
                    .into(),
            }
        } else {
            "".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let working_dir = match working_dir(&input, &self.project, cx) {
            Ok(dir) => dir,
            Err(err) => return Task::ready(Err(err)),
        };

        let authorize = event_stream.authorize(self.initial_title(Ok(input.clone()), cx), cx);
        cx.spawn(async move |cx| {
            authorize.await?;

            let terminal = self
                .environment
                .create_terminal(
                    input.command.clone(),
                    working_dir,
                    Some(COMMAND_OUTPUT_LIMIT),
                    cx,
                )
                .await?;

            let terminal_id = terminal.id(cx)?;
            event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![
                acp::ToolCallContent::Terminal(acp::Terminal::new(terminal_id)),
            ]));

            let timeout = input.timeout_ms.map(Duration::from_millis);

            let mut timed_out = false;
            let wait_for_exit = terminal.wait_for_exit(cx)?;

            match timeout {
                Some(timeout) => {
                    let timeout_task = cx.background_spawn(async move {
                        smol::Timer::after(timeout).await;
                    });

                    futures::select! {
                        _ = wait_for_exit.clone().fuse() => {},
                        _ = timeout_task.fuse() => {
                            timed_out = true;
                            terminal.kill(cx)?;
                            wait_for_exit.await;
                        }
                    }
                }
                None => {
                    wait_for_exit.await;
                }
            };

            // Check if user stopped - we check both:
            // 1. The cancellation signal from RunningTurn::cancel (e.g. user pressed main Stop button)
            // 2. The terminal's user_stopped flag (e.g. user clicked Stop on the terminal card)
            let user_stopped_via_signal = event_stream.was_cancelled_by_user();
            let user_stopped_via_terminal = terminal.was_stopped_by_user(cx).unwrap_or(false);
            let user_stopped = user_stopped_via_signal || user_stopped_via_terminal;

            let output = terminal.current_output(cx)?;

            Ok(process_content(
                output,
                &input.command,
                timed_out,
                user_stopped,
            ))
        })
    }
}

fn process_content(
    output: acp::TerminalOutputResponse,
    command: &str,
    timed_out: bool,
    user_stopped: bool,
) -> String {
    let content = output.output.trim();
    let is_empty = content.is_empty();

    let content = format!("```\n{content}\n```");
    let content = if output.truncated {
        format!(
            "Command output too long. The first {} bytes:\n\n{content}",
            content.len(),
        )
    } else {
        content
    };

    let content = if user_stopped {
        if is_empty {
            "The user stopped this command. No output was captured before stopping.\n\n\
            Since the user intentionally interrupted this command, ask them what they would like to do next \
            rather than automatically retrying or assuming something went wrong.".to_string()
        } else {
            format!(
                "The user stopped this command. Output captured before stopping:\n\n{}\n\n\
                Since the user intentionally interrupted this command, ask them what they would like to do next \
                rather than automatically retrying or assuming something went wrong.",
                content
            )
        }
    } else if timed_out {
        if is_empty {
            format!("Command \"{command}\" timed out. No output was captured.")
        } else {
            format!(
                "Command \"{command}\" timed out. Output captured before timeout:\n\n{}",
                content
            )
        }
    } else {
        let exit_code = output.exit_status.as_ref().and_then(|s| s.exit_code);
        match exit_code {
            Some(0) => {
                if is_empty {
                    "Command executed successfully.".to_string()
                } else {
                    content
                }
            }
            Some(exit_code) => {
                if is_empty {
                    format!("Command \"{command}\" failed with exit code {}.", exit_code)
                } else {
                    format!(
                        "Command \"{command}\" failed with exit code {}.\n\n{content}",
                        exit_code
                    )
                }
            }
            None => {
                if is_empty {
                    "Command terminated unexpectedly. No output was captured.".to_string()
                } else {
                    format!(
                        "Command terminated unexpectedly. Output captured:\n\n{}",
                        content
                    )
                }
            }
        }
    };
    content
}

fn working_dir(
    input: &TerminalToolInput,
    project: &Entity<Project>,
    cx: &mut App,
) -> Result<Option<PathBuf>> {
    let project = project.read(cx);
    let cd = &input.cd;

    if cd == "." || cd.is_empty() {
        // Accept "." or "" as meaning "the one worktree" if we only have one worktree.
        let mut worktrees = project.worktrees(cx);

        match worktrees.next() {
            Some(worktree) => {
                anyhow::ensure!(
                    worktrees.next().is_none(),
                    "'.' is ambiguous in multi-root workspaces. Please specify a root directory explicitly.",
                );
                Ok(Some(worktree.read(cx).abs_path().to_path_buf()))
            }
            None => Ok(None),
        }
    } else {
        let input_path = Path::new(cd);

        if input_path.is_absolute() {
            // Absolute paths are allowed, but only if they're in one of the project's worktrees.
            if project
                .worktrees(cx)
                .any(|worktree| input_path.starts_with(&worktree.read(cx).abs_path()))
            {
                return Ok(Some(input_path.into()));
            }
        } else if let Some(worktree) = project.worktree_for_root_name(cd, cx) {
            return Ok(Some(worktree.read(cx).abs_path().to_path_buf()));
        }

        anyhow::bail!("`cd` directory {cd:?} was not in any of the project's worktrees.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_content_user_stopped() {
        let output = acp::TerminalOutputResponse::new("partial output".to_string(), false);

        let result = process_content(output, "cargo build", false, true);

        assert!(
            result.contains("user stopped"),
            "Expected 'user stopped' message, got: {}",
            result
        );
        assert!(
            result.contains("partial output"),
            "Expected output to be included, got: {}",
            result
        );
        assert!(
            result.contains("ask them what they would like to do"),
            "Should instruct agent to ask user, got: {}",
            result
        );
    }

    #[test]
    fn test_process_content_user_stopped_empty_output() {
        let output = acp::TerminalOutputResponse::new("".to_string(), false);

        let result = process_content(output, "cargo build", false, true);

        assert!(
            result.contains("user stopped"),
            "Expected 'user stopped' message, got: {}",
            result
        );
        assert!(
            result.contains("No output was captured"),
            "Expected 'No output was captured', got: {}",
            result
        );
    }

    #[test]
    fn test_process_content_timed_out() {
        let output = acp::TerminalOutputResponse::new("build output here".to_string(), false);

        let result = process_content(output, "cargo build", true, false);

        assert!(
            result.contains("timed out"),
            "Expected 'timed out' message for timeout, got: {}",
            result
        );
        assert!(
            result.contains("build output here"),
            "Expected output to be included, got: {}",
            result
        );
    }

    #[test]
    fn test_process_content_timed_out_with_empty_output() {
        let output = acp::TerminalOutputResponse::new("".to_string(), false);

        let result = process_content(output, "sleep 1000", true, false);

        assert!(
            result.contains("timed out"),
            "Expected 'timed out' for timeout, got: {}",
            result
        );
        assert!(
            result.contains("No output was captured"),
            "Expected 'No output was captured' for empty output, got: {}",
            result
        );
    }

    #[test]
    fn test_process_content_with_success() {
        let output = acp::TerminalOutputResponse::new("success output".to_string(), false)
            .exit_status(acp::TerminalExitStatus::new().exit_code(0));

        let result = process_content(output, "echo hello", false, false);

        assert!(
            result.contains("success output"),
            "Expected output to be included, got: {}",
            result
        );
        assert!(
            !result.contains("failed"),
            "Success should not say 'failed', got: {}",
            result
        );
    }

    #[test]
    fn test_process_content_with_success_empty_output() {
        let output = acp::TerminalOutputResponse::new("".to_string(), false)
            .exit_status(acp::TerminalExitStatus::new().exit_code(0));

        let result = process_content(output, "true", false, false);

        assert!(
            result.contains("executed successfully"),
            "Expected success message for empty output, got: {}",
            result
        );
    }

    #[test]
    fn test_process_content_with_error_exit() {
        let output = acp::TerminalOutputResponse::new("error output".to_string(), false)
            .exit_status(acp::TerminalExitStatus::new().exit_code(1));

        let result = process_content(output, "false", false, false);

        assert!(
            result.contains("failed with exit code 1"),
            "Expected failure message, got: {}",
            result
        );
        assert!(
            result.contains("error output"),
            "Expected output to be included, got: {}",
            result
        );
    }

    #[test]
    fn test_process_content_with_error_exit_empty_output() {
        let output = acp::TerminalOutputResponse::new("".to_string(), false)
            .exit_status(acp::TerminalExitStatus::new().exit_code(1));

        let result = process_content(output, "false", false, false);

        assert!(
            result.contains("failed with exit code 1"),
            "Expected failure message, got: {}",
            result
        );
    }

    #[test]
    fn test_process_content_unexpected_termination() {
        let output = acp::TerminalOutputResponse::new("some output".to_string(), false);

        let result = process_content(output, "some_command", false, false);

        assert!(
            result.contains("terminated unexpectedly"),
            "Expected 'terminated unexpectedly' message, got: {}",
            result
        );
        assert!(
            result.contains("some output"),
            "Expected output to be included, got: {}",
            result
        );
    }

    #[test]
    fn test_process_content_unexpected_termination_empty_output() {
        let output = acp::TerminalOutputResponse::new("".to_string(), false);

        let result = process_content(output, "some_command", false, false);

        assert!(
            result.contains("terminated unexpectedly"),
            "Expected 'terminated unexpectedly' message, got: {}",
            result
        );
        assert!(
            result.contains("No output was captured"),
            "Expected 'No output was captured' for empty output, got: {}",
            result
        );
    }
}
