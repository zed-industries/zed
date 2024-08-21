use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};
use gpui::{AppContext, Task, View, WeakView};
use language::{CodeLabel, LspAdapterDelegate};
use terminal_view::{terminal_panel::TerminalPanel, TerminalView};
use ui::prelude::*;
use workspace::{dock::Panel, Workspace};

use crate::DEFAULT_CONTEXT_LINES;

use super::create_label_for_command;

pub(crate) struct TerminalSlashCommand;

const LINE_COUNT_ARG: &str = "--line-count";

impl SlashCommand for TerminalSlashCommand {
    fn name(&self) -> String {
        "terminal".into()
    }

    fn label(&self, cx: &AppContext) -> CodeLabel {
        create_label_for_command("terminal", &[LINE_COUNT_ARG], cx)
    }

    fn description(&self) -> String {
        "insert terminal output".into()
    }

    fn menu_text(&self) -> String {
        "Insert Terminal Output".into()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn accepts_arguments(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };

        let Some(active_terminal) = resolve_active_terminal(&workspace, cx) else {
            return Task::ready(Err(anyhow::anyhow!("no active terminal")));
        };

        let line_count = arguments
            .get(0)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(DEFAULT_CONTEXT_LINES);

        let lines = active_terminal
            .read(cx)
            .model()
            .read(cx)
            .last_n_non_empty_lines(line_count);

        let mut text = String::new();
        text.push_str("Terminal output:\n");
        text.push_str(&lines.join("\n"));
        let range = 0..text.len();

        Task::ready(Ok(SlashCommandOutput {
            text,
            sections: vec![SlashCommandOutputSection {
                range,
                icon: IconName::Terminal,
                label: "Terminal".into(),
            }],
            run_commands_in_text: false,
        }))
    }
}

fn resolve_active_terminal(
    workspace: &View<Workspace>,
    cx: &WindowContext,
) -> Option<View<TerminalView>> {
    if let Some(terminal_view) = workspace
        .read(cx)
        .active_item(cx)
        .and_then(|item| item.act_as::<TerminalView>(cx))
    {
        return Some(terminal_view);
    }

    let terminal_panel = workspace.read(cx).panel::<TerminalPanel>(cx)?;
    terminal_panel.read(cx).pane().and_then(|pane| {
        pane.read(cx)
            .active_item()
            .and_then(|t| t.downcast::<TerminalView>())
    })
}
