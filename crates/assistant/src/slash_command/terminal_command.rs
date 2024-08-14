use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};
use gpui::{AppContext, Task, WeakView};
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

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let completions = if arguments.iter().any(|arg| arg == LINE_COUNT_ARG) {
            Vec::new()
        } else {
            vec![ArgumentCompletion {
                label: LINE_COUNT_ARG.into(),
                new_text: LINE_COUNT_ARG.to_string(),
                run_command: false,
            }]
        };
        Task::ready(Ok(completions))
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
        let Some(terminal_panel) = workspace.read(cx).panel::<TerminalPanel>(cx) else {
            return Task::ready(Err(anyhow::anyhow!("no terminal panel open")));
        };
        let Some(active_terminal) = terminal_panel.read(cx).pane().and_then(|pane| {
            pane.read(cx)
                .active_item()
                .and_then(|t| t.downcast::<TerminalView>())
        }) else {
            return Task::ready(Err(anyhow::anyhow!("no active terminal")));
        };

        let mut line_count = DEFAULT_CONTEXT_LINES;
        if arguments.get(0).map(|s| s.as_str()) == Some(LINE_COUNT_ARG) {
            if let Some(parsed_line_count) = arguments.get(1).and_then(|s| s.parse::<usize>().ok())
            {
                line_count = parsed_line_count;
            }
        }

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
