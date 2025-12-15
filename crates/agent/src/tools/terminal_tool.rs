use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
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
/// Remember that each invocation of this tool will spawn a new shell process, so you can't rely on any state from previous invocations.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalToolInput {
    /// The one-liner command to execute.
    command: String,
    /// Working directory for the command. This must be one of the root directories of the project.
    cd: String,
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

            let exit_status = terminal.wait_for_exit(cx)?.await;
            let output = terminal.current_output(cx)?;

            Ok(process_content(output, &input.command, exit_status))
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
    let cd = &input.cd;

    // Diagnostics: When a user reports that they added a worktree but the tool can't `cd` into it,
    // the most common culprits are (a) the agent is bound to a different Project entity/window, or
    // (b) path normalization/canonicalization differences causing starts_with checks to fail.
    //
    // Enable with `ZED_AGENT_TERMINAL_CD_DIAGNOSTICS=1`.
    let diagnostics_enabled = std::env::var("ZED_AGENT_TERMINAL_CD_DIAGNOSTICS")
        .ok()
        .as_deref()
        == Some("1");

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

        if diagnostics_enabled {
            let mut worktree_debug_lines = Vec::new();
            for worktree in project.worktrees(cx) {
                let worktree = worktree.read(cx);
                let root_name = worktree.root_name();
                let abs_path = worktree.abs_path().to_path_buf();

                let starts_with = input_path.is_absolute() && input_path.starts_with(&abs_path);

                let canonical_input = if input_path.is_absolute() {
                    std::fs::canonicalize(input_path).ok()
                } else {
                    None
                };
                let canonical_worktree = std::fs::canonicalize(&abs_path).ok();

                let canonical_starts_with = match (&canonical_input, &canonical_worktree) {
                    (Some(ci), Some(cw)) => ci.starts_with(cw),
                    _ => false,
                };

                worktree_debug_lines.push(format!(
                    "  - root_name={root_name:?}\n    abs_path={abs_path:?}\n    starts_with(abs_path)={starts_with}\n    canonical_worktree={canonical_worktree:?}\n    canonical_input={canonical_input:?}\n    canonical_starts_with={canonical_starts_with}"
                ));
            }

            log::warn!(
                "[agent][terminal] rejecting cd: cd={cd:?} input_path={input_path:?} input_is_absolute={} worktrees_count={} \nworktrees:\n{}",
                input_path.is_absolute(),
                project.worktrees(cx).count(),
                worktree_debug_lines.join("\n")
            );
        }

        anyhow::bail!("`cd` directory {cd:?} was not in any of the project's worktrees.");
    }
}
