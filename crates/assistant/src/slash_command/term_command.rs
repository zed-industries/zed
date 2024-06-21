use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::Result;
use assistant_slash_command::{SlashCommand, SlashCommandOutput, SlashCommandOutputSection};
use gpui::{AppContext, Task, WeakView};
use language::{CodeLabel, LspAdapterDelegate};
use terminal_view::{terminal_panel::TerminalPanel, TerminalView};
use ui::prelude::*;
use workspace::Workspace;

use super::create_label_for_command;

pub(crate) struct TermSlashCommand;

const LINE_COUNT_ARG: &str = "--line-count";

impl SlashCommand for TermSlashCommand {
    fn name(&self) -> String {
        "term".into()
    }

    fn label(&self, cx: &AppContext) -> CodeLabel {
        create_label_for_command("term", &[LINE_COUNT_ARG], cx)
    }

    fn description(&self) -> String {
        "insert terminal output".into()
    }

    fn menu_text(&self) -> String {
        "Insert terminal output".into()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn complete_argument(
        self: Arc<Self>,
        _query: String,
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Ok(vec![LINE_COUNT_ARG.to_string()]))
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };
        let Some(terminal_panel) = workspace.read(cx).panel::<TerminalPanel>(cx) else {
            return Task::ready(Err(anyhow::anyhow!("no terminal panel open")));
        };
        let Some(active_terminal) = terminal_panel
            .read(cx)
            .pane()
            .read(cx)
            .active_item()
            .and_then(|t| t.downcast::<TerminalView>())
        else {
            return Task::ready(Err(anyhow::anyhow!("no active terminal")));
        };

        let line_count = argument.and_then(|a| parse_argument(a)).unwrap_or(20);

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

fn parse_argument(argument: &str) -> Option<usize> {
    let mut args = argument.split(' ');
    if args.next() == Some(LINE_COUNT_ARG) {
        if let Some(line_count) = args.next().and_then(|s| s.parse::<usize>().ok()) {
            return Some(line_count);
        }
    }
    None
}
