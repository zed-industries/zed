use crate::assistant_panel::selections_creases;
use anyhow::{anyhow, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandContent, SlashCommandEvent,
    SlashCommandOutputSection, SlashCommandResult,
};
use futures::StreamExt;
use gpui::{AppContext, Task, WeakView};
use language::{BufferSnapshot, CodeLabel, LspAdapterDelegate};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use ui::{IconName, SharedString, WindowContext};
use workspace::Workspace;

pub(crate) struct SelectionCommand;

impl SlashCommand for SelectionCommand {
    fn name(&self) -> String {
        "selection".into()
    }

    fn label(&self, _cx: &AppContext) -> CodeLabel {
        CodeLabel::plain(self.name(), None)
    }

    fn description(&self) -> String {
        "Insert editor selection".into()
    }

    fn icon(&self) -> IconName {
        IconName::Quote
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
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
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

        for (text, title) in creases {
            events.push(Ok(SlashCommandEvent::StartSection {
                icon: IconName::TextSnippet,
                label: SharedString::from(title),
                metadata: None,
            }));
            events.push(Ok(SlashCommandEvent::Content(SlashCommandContent::Text {
                text,
                run_commands_in_text: false,
            })));
            events.push(Ok(SlashCommandEvent::EndSection));
            events.push(Ok(SlashCommandEvent::Content(SlashCommandContent::Text {
                text: "\n".to_string(),
                run_commands_in_text: false,
            })));
        }

        let result = futures::stream::iter(events).boxed();

        Task::ready(Ok(result))
    }
}
