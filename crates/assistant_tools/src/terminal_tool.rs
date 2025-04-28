use crate::schema::json_schema_for;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolCard, ToolResult, ToolUseStatus};
use futures::{
    AsyncBufReadExt, SinkExt, StreamExt,
    channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded},
    io::BufReader,
    stream::SelectAll,
};
use gpui::{
    AnyWindowHandle, App, AppContext, Entity, StyledText, Task, TextLayout, WeakEntity, Window,
    prelude::FluentBuilder,
};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::Path, process::Stdio, sync::Arc};
use ui::{IconName, prelude::*};
use util::{
    command::new_smol_command,
    get_system_shell,
    markdown::{MarkdownInlineCode, MarkdownString},
};
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
    working_dir: Option<Arc<Path>>,
    command: String,
    mut line_sender: UnboundedSender<Result<String>>,
    cx: &mut App,
) -> Result<Task<Result<String>>> {
    let shell = get_system_shell();

    let mut cmd = {
        let mut cmd = new_smol_command(&shell);
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }
        cmd.args(["-c", &command])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to execute terminal command")?
    };

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

        let mut truncated = false;

        while let Some(line) = line_stream.next().await {
            let line = match line {
                Ok(line) => line,
                Err(err) => {
                    let err = format!("Failed to read line: {err}");
                    let _ = line_sender.send(Err(anyhow!(err.clone()))).await;
                    return Err(anyhow!(err));
                }
            };

            truncated |= combined_output.len() + line.len() > COMMAND_OUTPUT_LIMIT;

            let line = if truncated {
                let remaining_capacity = COMMAND_OUTPUT_LIMIT.saturating_sub(combined_output.len());
                &line[..remaining_capacity]
            } else {
                &line
            };

            combined_output.push_str(line);
            combined_output.push('\n');
            let send_result = line_sender.send(Ok(line.to_owned())).await;

            if truncated || send_result.is_err() {
                break;
            }
        }

        let truncated_output = if truncated {
            let last_line_ix = combined_output.rfind('\n');
            // Don't truncate mid-line, clear the remainder of the last line
            let output = &combined_output[..last_line_ix.unwrap_or(combined_output.len())];

            format!(
                "Command output too long. The first {} bytes:\n\n{}",
                output.len(),
                output_block(&output),
            )
        } else {
            output_block(&combined_output)
        };

        let status = match cmd.status().await {
            Ok(status) => status,
            Err(err) => {
                // Error occurred getting status (potential interruption), include partial output
                let partial_output = output_block(&combined_output);
                let error_message = format!(
                    "Command failed or was interrupted.\nPartial output captured:\n\n{}",
                    partial_output,
                );
                return Err(anyhow!(err).context(error_message));
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
                shell,
                truncated_output,
            )
        };

        Ok(output_with_status)
    });

    Ok(fut)
}

fn working_dir(
    cx: &mut App,
    input: &TerminalToolInput,
    project: &Entity<Project>,
    input_path: &Path,
) -> Result<Option<Arc<Path>>, &'static str> {
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
                Ok(Some(worktree.read(cx).abs_path()))
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

        Ok(Some(worktree.read(cx).abs_path()))
    }
}

fn output_block(output: &str) -> String {
    format!(
        "```\n{}{}```",
        output,
        if output.ends_with('\n') { "" } else { "\n" }
    )
}

struct TerminalToolCardElement {
    // card: Entity<TerminalToolCard>,
    // styled_text: StyledText,
}

struct TerminalToolCard {
    failed: bool,
    contents: String,
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
                            Err(_) => {
                                card.failed = true;
                                return; // stop receiving
                            }
                        };

                        card.contents += &line;
                        cx.notify();
                    })
                    .is_err();

                if is_entity_released {
                    return;
                }
            }
        });

        Self {
            failed: false,
            contents: String::new(),
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
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        self.contents.to_owned()
        // TerminalToolCardElement {
        //     // card: cx.entity(),
        //     // styled_text: StyledText::,
        // }
    }
}

// impl IntoElement for TerminalToolCardElement {
//     type Element = Self;

//     fn into_element(self) -> Self::Element {
//         self
//     }
// }

// impl Element for TerminalToolCardElement {
//     type RequestLayoutState = ();
//     type PrepaintState = ();

//     fn id(&self) -> Option<ElementId> {
//         None
//     }

//     fn request_layout(
//         &mut self,
//         id: Option<&gpui::GlobalElementId>,
//         window: &mut Window,
//         cx: &mut App,
//     ) -> (gpui::LayoutId, Self::RequestLayoutState) {
//         todo!()
//     }

//     fn prepaint(
//         &mut self,
//         id: Option<&gpui::GlobalElementId>,
//         bounds: gpui::Bounds<Pixels>,
//         request_layout: &mut Self::RequestLayoutState,
//         window: &mut Window,
//         cx: &mut App,
//     ) -> Self::PrepaintState {
//         todo!()
//     }

//     fn paint(
//         &mut self,
//         id: Option<&gpui::GlobalElementId>,
//         bounds: gpui::Bounds<Pixels>,
//         request_layout: &mut Self::RequestLayoutState,
//         prepaint: &mut Self::PrepaintState,
//         window: &mut Window,
//         cx: &mut App,
//     ) {
//         todo!()
//     }
// }
