use agent_client_protocol as acp;
use agent_settings::AgentSettings;
use anyhow::Result;
use futures::FutureExt as _;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use util::markdown::MarkdownInlineCode;

use crate::{
    AgentTool, ThreadEnvironment, ToolCallEventStream, ToolPermissionDecision,
    decide_permission_from_settings,
};

const COMMAND_OUTPUT_LIMIT: u64 = 16 * 1024;

/// Executes a shell command or interacts with a running terminal process.
///
/// This tool can:
/// 1. Run a new command in a terminal (RunCmd)
/// 2. Send input to an already-running process (SendInput)
///
/// When a command times out, the process is NOT killed. Instead, you get
/// the current terminal output and can decide what to do next:
/// - Send input to interact with the process (e.g., "q" to quit less, Ctrl+C to interrupt)
/// - Make a different tool call or respond with text (this will automatically kill the terminal)
///
/// Make sure you use the `cd` parameter to navigate to one of the root directories of the project. NEVER do it as part of the `command` itself, otherwise it will error.
///
/// For potentially long-running commands, prefer specifying `timeout_ms` to bound runtime and prevent indefinite hangs.
///
/// The terminal emulator is an interactive pty, so commands may block waiting for user input.
/// Some commands can be configured not to do this, such as `git --no-pager diff` and similar.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalToolInput {
    /// The action to perform: run a command or send input to a running process.
    pub action: TerminalAction,
    /// Optional timeout in milliseconds. If the process hasn't exited by then, the tool returns
    /// with the current terminal state. The process is NOT killed - you can send more input or wait again.
    pub timeout_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum TerminalAction {
    /// Executes a command in a terminal.
    /// For example, "git status" would run `git status`.
    /// Returns a terminal_id that can be used with SendInput.
    /// If the command doesn't exit within timeout_ms, returns the current output
    /// and the process keeps running - use SendInput to interact with it.
    RunCmd {
        /// The one-liner command to execute.
        command: String,
        /// Working directory for the command. This must be one of the root directories of the project.
        cd: String,
    },
    /// Sends input to an already-running process.
    SendInput {
        /// The ID of the terminal to send input to (from a previous RunCmd).
        terminal_id: String,
        /// The input string to send (e.g., "q\n" to quit less, or "\x03" for Ctrl+C).
        input: String,
    },
}

impl TerminalAction {
    /// Returns the user-facing label for this action type.
    pub fn ui_label(&self) -> &'static str {
        match self {
            TerminalAction::RunCmd { .. } => "Run Command",
            TerminalAction::SendInput { .. } => "Send Input to Process",
        }
    }

    /// Parses the action from raw JSON input (e.g., from a tool call's raw_input field).
    /// Returns None if the JSON doesn't represent a valid TerminalToolInput.
    pub fn parse_from_json(json: &serde_json::Value) -> Option<Self> {
        serde_json::from_value::<TerminalToolInput>(json.clone())
            .ok()
            .map(|input| input.action)
    }
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

    const NAME: &'static str = "terminal";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Execute
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            let text = match &input.action {
                TerminalAction::RunCmd { command, .. } => command.as_str(),
                TerminalAction::SendInput { input, .. } => input.as_str(),
            };
            let mut lines = text.lines();
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
        let timeout = input.timeout_ms.map(Duration::from_millis);

        match &input.action {
            TerminalAction::RunCmd { command, cd } => {
                let working_dir = match working_dir_from_cd(cd, &self.project, cx) {
                    Ok(dir) => dir,
                    Err(err) => return Task::ready(Err(err)),
                };
                let command = command.clone();

                let settings = AgentSettings::get_global(cx);
                let decision = decide_permission_from_settings(
                    Self::NAME,
                    std::slice::from_ref(&command),
                    settings,
                );

                let authorize = match decision {
                    ToolPermissionDecision::Allow => None,
                    ToolPermissionDecision::Deny(reason) => {
                        return Task::ready(Err(anyhow::anyhow!("{}", reason)));
                    }
                    ToolPermissionDecision::Confirm => {
                        let context =
                            crate::ToolPermissionContext::new(Self::NAME, vec![command.clone()]);
                        Some(event_stream.authorize(
                            self.initial_title(Ok(input.clone()), cx),
                            context,
                            cx,
                        ))
                    }
                };

                cx.spawn(async move |cx| {
                    if let Some(authorize) = authorize {
                        authorize.await?;
                    }

                    let terminal = self
                        .environment
                        .create_terminal(
                            command.clone(),
                            working_dir,
                            Some(COMMAND_OUTPUT_LIMIT),
                            cx,
                        )
                        .await?;

                    let terminal_id = terminal.id(cx)?;
                    event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![
                        acp::ToolCallContent::Terminal(acp::Terminal::new(terminal_id.clone())),
                    ]));

                    let mut user_stopped_via_signal = false;
                    let (exited, exit_status) = match timeout {
                        Some(timeout) => {
                            let wait_for_exit = terminal.wait_for_exit(cx)?;
                            let timeout_task = cx.background_executor().timer(timeout);
                            futures::select! {
                                status = wait_for_exit.clone().fuse() => (true, status),
                                _ = timeout_task.fuse() => (false, acp::TerminalExitStatus::new()),
                                _ = event_stream.cancelled_by_user().fuse() => {
                                    user_stopped_via_signal = true;
                                    terminal.kill(cx)?;
                                    terminal.wait_for_exit(cx)?.await;
                                    (true, acp::TerminalExitStatus::new())
                                },
                            }
                        }
                        None => {
                            let wait_for_exit = terminal.wait_for_exit(cx)?;
                            futures::select! {
                                status = wait_for_exit.clone().fuse() => (true, status),
                                _ = event_stream.cancelled_by_user().fuse() => {
                                    user_stopped_via_signal = true;
                                    terminal.kill(cx)?;
                                    wait_for_exit.await;
                                    (true, acp::TerminalExitStatus::new())
                                },
                            }
                        }
                    };

                    let user_stopped = user_stopped_via_signal
                        || event_stream.was_cancelled_by_user()
                        || terminal.was_stopped_by_user(cx).unwrap_or(false);

                    let output = terminal.current_output(cx)?;
                    let terminal_id_str = terminal_id.0.to_string();

                    Ok(process_run_cmd_result(
                        output,
                        &command,
                        &terminal_id_str,
                        exited,
                        exit_status,
                        timeout,
                        user_stopped,
                    ))
                })
            }
            TerminalAction::SendInput { terminal_id, input } => {
                let terminal_id = acp::TerminalId::new(terminal_id.clone());
                let input = input.clone();

                let title: SharedString =
                    MarkdownInlineCode(&format!("Send input '{}' to current process", input))
                        .to_string()
                        .into();

                let settings = AgentSettings::get_global(cx);
                let decision = decide_permission_from_settings(
                    Self::NAME,
                    std::slice::from_ref(&input),
                    settings,
                );

                let authorize = match decision {
                    ToolPermissionDecision::Allow => None,
                    ToolPermissionDecision::Deny(reason) => {
                        return Task::ready(Err(anyhow::anyhow!("{}", reason)));
                    }
                    ToolPermissionDecision::Confirm => {
                        let context =
                            crate::ToolPermissionContext::new(Self::NAME, vec![input.clone()]);
                        Some(event_stream.authorize(title, context, cx))
                    }
                };

                cx.spawn(async move |cx| {
                    if let Some(authorize) = authorize {
                        authorize.await?;
                    }

                    let terminal = self.environment.get_terminal(&terminal_id, cx)?;
                    terminal.send_input(&input, cx)?;

                    let timeout = timeout.unwrap_or(Duration::from_millis(1000));
                    let (exited, exit_status) = {
                        let wait_for_exit = terminal.wait_for_exit(cx)?;
                        let timeout_task = cx.background_executor().timer(timeout);
                        futures::select! {
                            status = wait_for_exit.clone().fuse() => (true, status),
                            _ = timeout_task.fuse() => (false, acp::TerminalExitStatus::new()),
                            _ = event_stream.cancelled_by_user().fuse() => {
                                terminal.kill(cx)?;
                                terminal.wait_for_exit(cx)?.await;
                                let output = terminal.current_output(cx)?;
                                return Ok(format!(
                                    "The user stopped this operation. Input \"{}\" was sent before stopping.\n\nTerminal output:\n\n```\n{}\n```",
                                    input,
                                    output.output.trim()
                                ));
                            },
                        }
                    };

                    let output = terminal.current_output(cx)?;
                    Ok(process_send_input_result(
                        output,
                        &input,
                        exited,
                        exit_status,
                        timeout,
                    ))
                })
            }
        }
    }
}

fn process_run_cmd_result(
    output: acp::TerminalOutputResponse,
    command: &str,
    terminal_id: &str,
    exited: bool,
    exit_status: acp::TerminalExitStatus,
    timeout: Option<Duration>,
    user_stopped: bool,
) -> String {
    let content = output.output.trim();
    let content_block = if content.is_empty() {
        String::new()
    } else if output.truncated {
        format!(
            "Output truncated (limit: {} bytes):\n\n```\n{}\n```",
            COMMAND_OUTPUT_LIMIT, content
        )
    } else {
        format!("```\n{}\n```", content)
    };

    if user_stopped {
        if content_block.is_empty() {
            "The user stopped this command. No output was captured before stopping.\n\n\
            Since the user intentionally interrupted this command, ask them what they would like to do next \
            rather than automatically retrying or assuming something went wrong.".to_string()
        } else {
            format!(
                "The user stopped this command. Output captured before stopping:\n\n{}\n\n\
                Since the user intentionally interrupted this command, ask them what they would like to do next \
                rather than automatically retrying or assuming something went wrong.",
                content_block
            )
        }
    } else if exited {
        match exit_status.exit_code {
            Some(0) => {
                if content_block.is_empty() {
                    "Command executed successfully.".to_string()
                } else {
                    content_block
                }
            }
            Some(code) => {
                if content_block.is_empty() {
                    format!("Command \"{}\" failed with exit code {}.", command, code)
                } else {
                    format!(
                        "Command \"{}\" failed with exit code {}.\n\n{}",
                        command, code, content_block
                    )
                }
            }
            None => {
                if content_block.is_empty() {
                    format!("Command \"{}\" was interrupted.", command)
                } else {
                    format!(
                        "Command \"{}\" was interrupted.\n\n{}",
                        command, content_block
                    )
                }
            }
        }
    } else {
        let timeout_ms = timeout
            .expect("timeout must be Some when process hasn't exited")
            .as_millis();
        let still_running_msg = format!(
            "The command is still running after {} ms. Terminal ID: {}\n\n\
            You can:\n\
            - Use SendInput with terminal_id \"{}\" to send input (e.g., \"q\" to quit, or Ctrl+C as \"\\x03\")\n\
            - Make a different tool call or respond with text (this will automatically clean up the process)",
            timeout_ms, terminal_id, terminal_id
        );
        if content_block.is_empty() {
            still_running_msg
        } else {
            format!(
                "{}\n\nCurrent terminal output:\n\n{}",
                still_running_msg, content_block
            )
        }
    }
}

fn process_send_input_result(
    output: acp::TerminalOutputResponse,
    input: &str,
    exited: bool,
    exit_status: acp::TerminalExitStatus,
    timeout: Duration,
) -> String {
    let content = output.output.trim();
    let content_block = if content.is_empty() {
        String::new()
    } else if output.truncated {
        format!(
            "Output truncated (limit: {} bytes):\n\n```\n{}\n```",
            COMMAND_OUTPUT_LIMIT, content
        )
    } else {
        format!("```\n{}\n```", content)
    };

    if exited {
        match exit_status.exit_code {
            Some(0) => {
                if content_block.is_empty() {
                    format!(
                        "Input \"{}\" was sent. The process exited successfully.",
                        input
                    )
                } else {
                    format!(
                        "Input \"{}\" was sent. The process exited successfully.\n\n{}",
                        input, content_block
                    )
                }
            }
            Some(code) => {
                if content_block.is_empty() {
                    format!(
                        "Input \"{}\" was sent. The process exited with code {}.",
                        input, code
                    )
                } else {
                    format!(
                        "Input \"{}\" was sent. The process exited with code {}.\n\n{}",
                        input, code, content_block
                    )
                }
            }
            None => {
                if content_block.is_empty() {
                    format!("Input \"{}\" was sent. The process was interrupted.", input)
                } else {
                    format!(
                        "Input \"{}\" was sent. The process was interrupted.\n\n{}",
                        input, content_block
                    )
                }
            }
        }
    } else {
        let timeout_ms = timeout.as_millis();
        if content_block.is_empty() {
            format!(
                "Input \"{}\" was sent. The process has not exited after {} ms.",
                input, timeout_ms
            )
        } else {
            format!(
                "Input \"{}\" was sent. The process has not exited after {} ms. Current terminal state:\n\n{}",
                input, timeout_ms, content_block
            )
        }
    }
}

fn working_dir_from_cd(
    cd: &str,
    project: &Entity<Project>,
    cx: &mut App,
) -> Result<Option<PathBuf>> {
    let project = project.read(cx);

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
