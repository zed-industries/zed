use crate::schema::json_schema_for;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolCard, ToolResult, ToolUseStatus};
use gpui::{
    Animation, AnimationExt, AnyWindowHandle, App, AppContext, Empty, Entity, EntityId, Task,
    Transformation, WeakEntity, Window, percentage,
};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::{Project, terminals::TerminalKind};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    env,
    path::{Path, PathBuf},
    process::ExitStatus,
    sync::Arc,
    time::{Duration, Instant},
};
use terminal_view::TerminalView;
use ui::{Disclosure, IconName, Tooltip, prelude::*};
use util::{
    get_system_shell, markdown::MarkdownInlineCode, size::format_file_size,
    time::duration_alt_display,
};
use workspace::Workspace;

const COMMAND_OUTPUT_LIMIT: usize = 16 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
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
                    cwd: working_dir.clone(),
                    ..Default::default()
                }),
                window,
                cx,
            )
        });

        let card = cx.new(|cx| {
            TerminalToolCard::new(input.command.clone(), working_dir.clone(), cx.entity_id())
        });

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
                    card.start_instant = Instant::now();
                });

                let exit_status = terminal
                    .update(cx, |terminal, cx| terminal.wait_for_completed_task(cx))?
                    .await;
                let (content, content_line_count) = terminal.update(cx, |terminal, _| {
                    (terminal.get_content(), terminal.total_lines())
                })?;

                let previous_len = content.len();
                let (processed_content, finished_with_empty_output) =
                    process_content(content, &input.command, exit_status);

                let _ = card.update(cx, |card, _| {
                    card.command_finished = true;
                    card.exit_status = exit_status;
                    card.was_content_truncated = processed_content.len() < previous_len;
                    card.original_content_len = previous_len;
                    card.content_line_count = content_line_count;
                    card.finished_with_empty_output = finished_with_empty_output;
                    card.elapsed_time = Some(card.start_instant.elapsed());
                });

                Ok(processed_content)
            }
        });

        ToolResult {
            output,
            card: Some(card.into()),
        }
    }
}

fn process_content(
    content: String,
    command: &str,
    exit_status: Option<ExitStatus>,
) -> (String, bool) {
    let should_truncate = content.len() > COMMAND_OUTPUT_LIMIT;

    let content = if should_truncate {
        let mut end_ix = COMMAND_OUTPUT_LIMIT.min(content.len());
        while !content.is_char_boundary(end_ix) {
            end_ix -= 1;
        }
        // Don't truncate mid-line, clear the remainder of the last line
        end_ix = content[..end_ix].rfind('\n').unwrap_or(end_ix);
        &content[..end_ix]
    } else {
        content.as_str()
    };
    let is_empty = content.trim().is_empty();

    let content = format!(
        "```\n{}{}```",
        content,
        if content.ends_with('\n') { "" } else { "\n" }
    );

    let content = if should_truncate {
        format!(
            "Command output too long. The first {} bytes:\n\n{}",
            content.len(),
            content,
        )
    } else {
        content
    };

    let content = match exit_status {
        Some(exit_status) if exit_status.success() => {
            if is_empty {
                "Command executed successfully.".to_string()
            } else {
                content.to_string()
            }
        }
        Some(exit_status) => {
            let code = exit_status.code().unwrap_or(-1);
            if is_empty {
                format!("Command \"{command}\" failed with exit code {code}.")
            } else {
                format!("Command \"{command}\" failed with exit code {code}.\n\n{content}")
            }
        }
        None => {
            format!(
                "Command failed or was interrupted.\nPartial output captured:\n\n{}",
                content,
            )
        }
    };
    (content, is_empty)
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

struct TerminalToolCard {
    input_command: String,
    working_dir: Option<PathBuf>,
    entity_id: EntityId,
    exit_status: Option<ExitStatus>,
    terminal: Option<Entity<TerminalView>>,
    command_finished: bool,
    was_content_truncated: bool,
    finished_with_empty_output: bool,
    content_line_count: usize,
    original_content_len: usize,
    preview_expanded: bool,
    start_instant: Instant,
    elapsed_time: Option<Duration>,
}

impl TerminalToolCard {
    pub fn new(input_command: String, working_dir: Option<PathBuf>, entity_id: EntityId) -> Self {
        Self {
            input_command,
            working_dir,
            entity_id,
            exit_status: None,
            terminal: None,
            command_finished: false,
            was_content_truncated: false,
            finished_with_empty_output: false,
            original_content_len: 0,
            content_line_count: 0,
            preview_expanded: true,
            start_instant: Instant::now(),
            elapsed_time: None,
        }
    }
}

impl ToolCard for TerminalToolCard {
    fn render(
        &mut self,
        status: &ToolUseStatus,
        _window: &mut Window,
        _workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let Some(terminal) = self.terminal.as_ref() else {
            return Empty.into_any();
        };

        let tool_failed = matches!(status, ToolUseStatus::Error(_));
        let command_failed =
            self.command_finished && self.exit_status.is_none_or(|code| !code.success());
        if (tool_failed || command_failed) && self.elapsed_time.is_none() {
            self.elapsed_time = Some(self.start_instant.elapsed());
        }
        let time_elapsed = self
            .elapsed_time
            .unwrap_or_else(|| self.start_instant.elapsed());
        let should_hide_terminal =
            tool_failed || self.finished_with_empty_output || !self.preview_expanded;

        let border_color = cx.theme().colors().border.opacity(0.6);
        let header_bg = cx
            .theme()
            .colors()
            .element_background
            .blend(cx.theme().colors().editor_foreground.opacity(0.025));

        let header_label = h_flex()
            .w_full()
            .max_w_full()
            .px_1()
            .gap_0p5()
            .opacity(0.8)
            .child(
                h_flex()
                    .child(
                        Icon::new(IconName::Terminal)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        div()
                            .id(("terminal-tool-header-input-command", self.entity_id))
                            .text_size(rems(0.8125))
                            .font_buffer(cx)
                            .child(self.input_command.clone())
                            .ml_1p5()
                            .mr_0p5()
                            .tooltip({
                                let path = self
                                    .working_dir
                                    .as_ref()
                                    .cloned()
                                    .or_else(|| env::current_dir().ok())
                                    .map(|path| format!("\"{}\"", path.display()))
                                    .unwrap_or_else(|| "current directory".to_string());
                                Tooltip::text(if self.command_finished {
                                    format!("Ran in {path}")
                                } else {
                                    format!("Running in {path}")
                                })
                            }),
                    ),
            )
            .into_any_element();

        let header = h_flex()
            .flex_none()
            .p_1()
            .gap_1()
            .justify_between()
            .rounded_t_md()
            .bg(header_bg)
            .child(header_label)
            .map(|header| {
                let header = header
                    .when(self.was_content_truncated, |header| {
                        let tooltip =
                            if self.content_line_count + 10 > terminal::MAX_SCROLL_HISTORY_LINES {
                                "Output exceeded terminal max lines and was \
                                truncated, the model received the first 16 KB."
                                    .to_string()
                            } else {
                                format!(
                                    "Output is {} long, to avoid unexpected token usage, \
                                    only 16 KB was sent back to the model.",
                                    format_file_size(self.original_content_len as u64, true),
                                )
                            };
                        header.child(
                            div()
                                .id(("terminal-tool-truncated-label", self.entity_id))
                                .tooltip(Tooltip::text(tooltip))
                                .child(
                                    Label::new("(truncated)")
                                        .color(Color::Disabled)
                                        .size(LabelSize::Small),
                                ),
                        )
                    })
                    .when(time_elapsed > Duration::from_secs(10), |header| {
                        header.child(
                            Label::new(format!("({})", duration_alt_display(time_elapsed)))
                                .buffer_font(cx)
                                .color(Color::Disabled)
                                .size(LabelSize::Small),
                        )
                    });

                if tool_failed || command_failed {
                    header.child(
                        div()
                            .id(("terminal-tool-error-code-indicator", self.entity_id))
                            .child(
                                Icon::new(IconName::Close)
                                    .size(IconSize::Small)
                                    .color(Color::Error),
                            )
                            .when(command_failed && self.exit_status.is_some(), |this| {
                                this.tooltip(Tooltip::text(format!(
                                    "Exited with code {}",
                                    self.exit_status
                                        .and_then(|status| status.code())
                                        .unwrap_or(-1),
                                )))
                            })
                            .when(
                                !command_failed && tool_failed && status.error().is_some(),
                                |this| {
                                    this.tooltip(Tooltip::text(format!(
                                        "Error: {}",
                                        status.error().unwrap(),
                                    )))
                                },
                            ),
                    )
                } else if self.command_finished {
                    header.child(
                        Icon::new(IconName::Check)
                            .size(IconSize::Small)
                            .color(Color::Success),
                    )
                } else {
                    header.child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::Small)
                            .color(Color::Info)
                            .with_animation(
                                "arrow-circle",
                                Animation::new(Duration::from_secs(2)).repeat(),
                                |icon, delta| {
                                    icon.transform(Transformation::rotate(percentage(delta)))
                                },
                            ),
                    )
                }
            })
            .when(!tool_failed && !self.finished_with_empty_output, |header| {
                header.child(
                    Disclosure::new(
                        ("terminal-tool-disclosure", self.entity_id),
                        self.preview_expanded,
                    )
                    .opened_icon(IconName::ChevronUp)
                    .closed_icon(IconName::ChevronDown)
                    .on_click(cx.listener(
                        move |this, _event, _window, _cx| {
                            this.preview_expanded = !this.preview_expanded;
                        },
                    )),
                )
            });

        v_flex()
            .mb_2()
            .border_1()
            .when(tool_failed || command_failed, |card| card.border_dashed())
            .border_color(border_color)
            .rounded_lg()
            .overflow_hidden()
            .child(header)
            .when(!should_hide_terminal, |this| {
                this.child(div().child(terminal.clone()).min_h(px(250.0)))
            })
            .into_any()
    }
}
