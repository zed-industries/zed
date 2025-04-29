use crate::schema::json_schema_for;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolCard, ToolResult, ToolUseStatus};
use gpui::{AnyWindowHandle, App, AppContext, Empty, Entity, Task, WeakEntity, Window};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::{Project, terminals::TerminalKind};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::ExitStatus,
    sync::Arc,
};
use terminal_view::TerminalView;
use ui::{IconName, prelude::*};
use util::{get_system_shell, markdown::MarkdownInlineCode};
use workspace::Workspace;

const COMMAND_OUTPUT_LIMIT: usize = 16 * 1024;

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
                    0 => MarkdownInlineCode(&first_line).to_string(),
                    1 => MarkdownInlineCode(&format!(
                        "{} - {} more line",
                        first_line, remaining_line_count
                    ))
                    .to_string(),
                    n => MarkdownInlineCode(&format!("{} - {} more lines", first_line, n))
                        .to_string(),
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
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let Some(window) = window else {
            return Task::ready(Err(anyhow!("no window options"))).into();
        };

        let input: TerminalToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let input_path = Path::new(&input.cd);
        let working_dir = match working_dir(cx, &input, &project, input_path) {
            Ok(dir) => dir,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };
        let terminal = project.update(cx, |project, cx| {
            project.create_terminal(
                TerminalKind::Task(task::SpawnInTerminal {
                    command: get_system_shell(),
                    args: vec!["-c".into(), input.command.clone()],
                    cwd: working_dir,
                    show_command: true,
                    ..Default::default()
                }),
                window,
                cx,
            )
        });

        let card = cx.new(|_| TerminalToolCard::default());

        let output = cx.spawn({
            let card = card.clone();
            async move |cx| {
                let terminal = terminal.await?;
                let workspace = window
                    .downcast::<Workspace>()
                    .and_then(|handle| handle.entity(cx).ok())
                    .context("no workspace entity in root of window")?;

                let terminal_view = window.update(cx, |_, window, cx| {
                    cx.new(|cx| {
                        TerminalView::new(
                            terminal.clone(),
                            workspace.downgrade(),
                            None,
                            project.downgrade(),
                            window,
                            cx,
                        )
                    })
                })?;
                let _ = card.update(cx, |card, _| {
                    card.terminal = Some(terminal_view.clone());
                });

                let exit_status = terminal
                    .update(cx, |terminal, cx| terminal.wait_for_completed_task(cx))?
                    .await;
                let content = terminal.update(cx, |terminal, _| terminal.get_content())?;

                let original_size = content.len();
                let should_truncate = content.len() > COMMAND_OUTPUT_LIMIT;

                let truncated_output = if should_truncate {
                    let last_line_ix = content.rfind('\n');
                    // Don't truncate mid-line, clear the remainder of the last line
                    let output = &content[..last_line_ix.unwrap_or(content.len())];

                    format!(
                        "Command output too long. The first {} bytes:\n\n{}",
                        output.len(),
                        output_block(&output),
                    )
                } else {
                    output_block(&content)
                };

                let status = match exit_status {
                    Some(status) => status,
                    None => {
                        // Error occurred getting status (potential interruption), include partial output
                        let partial_output = output_block(&content);
                        let error_message = format!(
                            "Command failed or was interrupted.\nPartial output captured:\n\n{}",
                            partial_output,
                        );
                        return Err(anyhow!(error_message));
                    }
                };

                let output_with_status = if status.success() {
                    if truncated_output.is_empty() {
                        "Command executed successfully.".to_string()
                    } else {
                        truncated_output.to_string()
                    }
                } else {
                    format!(
                        "Command failed with exit code {} (shell: {}).\n\n{}",
                        status.code().unwrap_or(-1),
                        input.command,
                        truncated_output,
                    )
                };

                let _ = card.update(cx, |card, _| {
                    card.status = exit_status;
                    card.truncated = should_truncate;
                    card.original_size = original_size;
                    card.truncated_size = content.len();
                });

                Ok(output_with_status)
            }
        });

        ToolResult {
            output,
            card: Some(card.into()),
        }
    }
}

fn working_dir(
    cx: &mut App,
    input: &TerminalToolInput,
    project: &Entity<Project>,
    input_path: &Path,
) -> Result<Option<PathBuf>, &'static str> {
    let project = project.read(cx);

    if input.cd == "." {
        // Accept "." as meaning "the one worktree" if we only have one worktree.
        let mut worktrees = project.worktrees(cx);

        match worktrees.next() {
            Some(worktree) => {
                if worktrees.next().is_some() {
                    return Err(
                        "'.' is ambiguous in multi-root workspaces. Please specify a root directory explicitly.",
                    );
                }
                Ok(Some(worktree.read(cx).abs_path().to_path_buf()))
            }
            None => Ok(None),
        }
    } else if input_path.is_absolute() {
        // Absolute paths are allowed, but only if they're in one of the project's worktrees.
        if !project
            .worktrees(cx)
            .any(|worktree| input_path.starts_with(&worktree.read(cx).abs_path()))
        {
            return Err("The absolute path must be within one of the project's worktrees");
        }

        Ok(Some(input_path.into()))
    } else {
        let Some(worktree) = project.worktree_for_root_name(&input.cd, cx) else {
            return Err("`cd` directory {} not found in the project");
        };

        Ok(Some(worktree.read(cx).abs_path().to_path_buf()))
    }
}

fn output_block(output: &str) -> String {
    format!(
        "```\n{}{}```",
        output,
        if output.ends_with('\n') { "" } else { "\n" }
    )
}

#[derive(Default)]
struct TerminalToolCard {
    status: Option<ExitStatus>,
    terminal: Option<Entity<TerminalView>>,
    truncated: bool,
    original_size: usize,
    truncated_size: usize,
}

impl ToolCard for TerminalToolCard {
    fn render(
        &mut self,
        _status: &ToolUseStatus,
        _window: &mut Window,
        _workspace: WeakEntity<Workspace>,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if let Some(terminal) = self.terminal.as_ref() {
            div()
                .min_h(px(500.0))
                .min_w(px(300.0))
                .child(terminal.clone())
                .into_any_element()
        } else {
            Empty.into_any_element()
        }
    }
}
