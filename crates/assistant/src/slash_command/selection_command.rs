use super::super::assistant_panel::selections_creases;
use anyhow::anyhow;
use assistant_slash_command::{SlashCommand, SlashCommandContent, SlashCommandEvent};
use futures::stream;
use gpui::Task;
use smol::stream::StreamExt;

use ui::IconName;
pub(crate) struct SelectionCommand;

impl SlashCommand for SelectionCommand {
    fn name(&self) -> String {
        "selection".into()
    }

    fn description(&self) -> String {
        "Quote your active selection".into()
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn complete_argument(
        self: std::sync::Arc<Self>,
        _arguments: &[String],
        _cancel: std::sync::Arc<std::sync::atomic::AtomicBool>,
        _workspace: Option<gpui::WeakView<workspace::Workspace>>,
        _cx: &mut ui::WindowContext,
    ) -> gpui::Task<gpui::Result<Vec<assistant_slash_command::ArgumentCompletion>>> {
        gpui::Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn run(
        self: std::sync::Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[assistant_slash_command::SlashCommandOutputSection<language::Anchor>],
        _context_buffer: language::BufferSnapshot,
        workspace: gpui::WeakView<workspace::Workspace>,
        _delegate: Option<std::sync::Arc<dyn language::LspAdapterDelegate>>,
        cx: &mut ui::WindowContext,
    ) -> gpui::Task<assistant_slash_command::SlashCommandResult> {
        let mut events = vec![];

        let Some(creases) = workspace
            .update(cx, selections_creases)
            .unwrap_or_else(|e| {
                events.push(Err(e));
                None
            })
        else {
            return Task::ready(Err(anyhow!("no active selection")));
        };

        for (text, b) in creases.iter() {
            events.push(Ok(SlashCommandEvent::StartSection {
                icon: IconName::TextSnippet,
                label: b.clone().into(),
                metadata: None,
            }));
            events.push(Ok(SlashCommandEvent::Content(SlashCommandContent::Text {
                text: text.into(),
                run_commands_in_text: false,
            })));
            events.push(Ok(SlashCommandEvent::EndSection { metadata: None }));
            events.push(Ok(SlashCommandEvent::Content(SlashCommandContent::Text {
                text: "\n".into(),
                run_commands_in_text: false,
            })));
        }

        let result = stream::iter(events).boxed();

        Task::ready(Ok(result))
    }

    fn label(&self, _cx: &gpui::AppContext) -> language::CodeLabel {
        language::CodeLabel::plain(self.name(), None)
    }

    fn accepts_arguments(&self) -> bool {
        self.requires_argument()
    }
}
