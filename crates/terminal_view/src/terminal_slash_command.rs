use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::{TerminalView, terminal_panel::TerminalPanel};
use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use gpui::{App, Entity, Task, WeakEntity};
use language::{BufferSnapshot, CodeLabel, LspAdapterDelegate};
use ui::prelude::*;
use workspace::{Workspace, dock::Panel};

use assistant_slash_command::create_label_for_command;

pub struct TerminalSlashCommand;

const LINE_COUNT_ARG: &str = "--line-count";

const DEFAULT_CONTEXT_LINES: usize = 50;

impl SlashCommand for TerminalSlashCommand {
    fn name(&self) -> String {
        "terminal".into()
    }

    fn label(&self, cx: &App) -> CodeLabel {
        create_label_for_command("terminal", &[LINE_COUNT_ARG], cx)
    }

    fn description(&self) -> String {
        "Insert terminal output".into()
    }

    fn icon(&self) -> IconName {
        IconName::Terminal
    }

    fn menu_text(&self) -> String {
        self.description()
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
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
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
            .entity()
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
                metadata: None,
            }],
            run_commands_in_text: false,
        }
        .into_event_stream()))
    }
}

fn resolve_active_terminal(
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Option<Entity<TerminalView>> {
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
