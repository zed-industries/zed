use agent_client_protocol as acp;
use anyhow::Result;
use futures::FutureExt as _;
use gpui::{App, Entity, SharedString, Task};
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

use super::terminal_job_manager::TerminalJobManager;
use crate::{AgentTool, ThreadEnvironment, ToolCallEventStream};

const COMMAND_OUTPUT_LIMIT: u64 = 16 * 1024;

/// Executes a shell one-liner and returns the combined output.
///
/// This tool spawns a process using the user's shell, reads from stdout and stderr (preserving the order of writes), and returns a string with the combined output result.
///
/// The output results will be shown to the user already, only list it again if necessary, avoid being redundant.
///
/// The `cd` parameter is optional and defaults to the project root. If the project has multiple roots, you must specify which one to use. NEVER use `cd` as part of the `command` itself, otherwise it will error.
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
    /// If not specified, defaults to the project root (only works with single-root projects).
    #[serde(default)]
    pub cd: Option<String>,
    /// Optional maximum runtime (in milliseconds). If exceeded, the running terminal task is killed.
    /// Only applicable when async is false. Ignored and should not be set when async is true.
    pub timeout_ms: Option<u64>,
    /// If true, run the command asynchronously in the background without timeout.
    /// The tool returns immediately with a job ID, and the job runs indefinitely until completion.
    /// Use terminal_job_status, terminal_job_list, and terminal_job_cancel to manage async jobs.
    /// Default is false (synchronous execution with optional timeout).
    #[serde(default)]
    pub r#async: bool,
    /// Output limit in bytes. Default is 16384 (16KB).
    #[serde(default = "default_output_limit")]
    pub output_limit: Option<u64>,
}

fn default_output_limit() -> Option<u64> {
    Some(COMMAND_OUTPUT_LIMIT)
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

        // Handle async execution
        if input.r#async {
            // Warn if timeout_ms is set for async jobs (it will be ignored)
            if input.timeout_ms.is_some() {
                log::warn!("timeout_ms is ignored for async terminal jobs and should not be set");
            }

            let input_command = input.command.clone();
            let input_output_limit = input.output_limit;
            let self_env = self.environment.clone();
            let working_dir_clone = working_dir.clone();

            async fn run_async_terminal(
                authorize: Task<Result<()>>,
                input_command: String,
                input_output_limit: Option<u64>,
                self_env: Rc<dyn ThreadEnvironment>,
                working_dir_clone: Option<PathBuf>,
                event_stream: ToolCallEventStream,
                cx: &mut gpui::AsyncApp,
            ) -> Result<String> {
                authorize.await?;

                let output_limit = input_output_limit.or(Some(COMMAND_OUTPUT_LIMIT));
                let terminal = self_env
                    .create_terminal(
                        input_command.clone(),
                        working_dir_clone.clone(),
                        output_limit,
                        cx,
                    )
                    .await?;

                let terminal_id = terminal.id(cx)?;
                let job_manager = cx.update(|cx| TerminalJobManager::global(cx))?;
                let job_id = job_manager.new_job_id();

                // Register the job
                let working_dir_str = working_dir_clone
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| ".".to_string());

                job_manager.register_job(
                    job_id.clone(),
                    input_command.clone(),
                    working_dir_str,
                    terminal_id.clone(),
                    terminal.clone(),
                );

                event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![
                    acp::ToolCallContent::Terminal(acp::Terminal::new(terminal_id.clone())),
                ]));

                // Spawn a background task to wait for completion and update job status
                let terminal_clone = terminal.clone();
                let job_id_clone = job_id.clone();
                let mut cx_clone = cx.clone();
                cx.foreground_executor()
                    .spawn(async move {
                        // Wait for terminal to complete (no timeout for async jobs)
                        let exit_status = match terminal_clone.wait_for_exit(&mut cx_clone) {
                            Ok(wait) => wait.await,
                            Err(_) => return,
                        };

                        // Get final output and update job status
                        if let Ok(output) = terminal_clone.current_output(&mut cx_clone) {
                            let _ = cx_clone.update(|cx| {
                                let job_manager = TerminalJobManager::global(cx);
                                job_manager.complete_job(
                                    &job_id_clone,
                                    exit_status.exit_code.map(|c| c as i32),
                                    output.output.clone(),
                                );
                            });
                        }
                    })
                    .detach();

                // Return immediately with job info
                Ok(format!(
                    "Background job started successfully.\n\nJob ID: {}\nTerminal ID: {}\n\nThe terminal output is streaming above. Use `terminal_job_status` with the job ID to check the final status and output later.",
                    job_id, terminal_id
                ))
            }

            let cx_async = cx.to_async();
            return cx.foreground_executor().spawn(async move {
                run_async_terminal(
                    authorize,
                    input_command,
                    input_output_limit,
                    self_env,
                    working_dir_clone,
                    event_stream,
                    &mut cx_async.clone(),
                )
                .await
            });
        }

        // Synchronous execution (original behavior)
        async fn run_sync_terminal(
            authorize: Task<Result<()>>,
            input: TerminalToolInput,
            working_dir: Option<PathBuf>,
            self_env: Rc<dyn ThreadEnvironment>,
            event_stream: ToolCallEventStream,
            cx: &mut gpui::AsyncApp,
        ) -> Result<String> {
            authorize.await?;

            let output_limit = input.output_limit.or(Some(COMMAND_OUTPUT_LIMIT));
            let terminal = self_env
                .create_terminal(input.command.clone(), working_dir, output_limit, cx)
                .await?;

            let terminal_id = terminal.id(cx)?;
            event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![
                acp::ToolCallContent::Terminal(acp::Terminal::new(terminal_id)),
            ]));

            let exit_status = match input.timeout_ms {
                Some(timeout_ms) => {
                    let timeout = Duration::from_millis(timeout_ms);
                    let wait_for_exit = terminal.wait_for_exit(cx)?;
                    let timeout_task = cx.background_executor().spawn(async move {
                        smol::Timer::after(timeout).await;
                    });

                    futures::select! {
                        status = wait_for_exit.clone().fuse() => status,
                        _ = timeout_task.fuse() => {
                            terminal.kill(cx)?;
                            wait_for_exit.await
                        }
                    }
                }
                None => terminal.wait_for_exit(cx)?.await,
            };

            let output = terminal.current_output(cx)?;

            Ok(process_content(output, &input.command, exit_status))
        }

        let cx_async = cx.to_async();
        cx.foreground_executor().spawn(async move {
            run_sync_terminal(
                authorize,
                input,
                working_dir,
                self.environment.clone(),
                event_stream,
                &mut cx_async.clone(),
            )
            .await
        })
    }
}

fn process_content(
    output: acp::TerminalOutputResponse,
    command: &str,
    exit_status: acp::TerminalExitStatus,
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

    let content = match exit_status.exit_code {
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
            format!(
                "Command failed or was interrupted.\nPartial output captured:\n\n{}",
                content,
            )
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

    let cd = match &input.cd {
        Some(cd) => cd,
        None => {
            // No cd specified, use the single worktree root if there's only one
            let mut worktrees = project.worktrees(cx);
            match worktrees.next() {
                Some(worktree) => {
                    anyhow::ensure!(
                        worktrees.next().is_none(),
                        "cd parameter is required in multi-root workspaces. Please specify a root directory explicitly.",
                    );
                    return Ok(Some(worktree.read(cx).abs_path().to_path_buf()));
                }
                None => return Ok(None),
            }
        }
    };

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
