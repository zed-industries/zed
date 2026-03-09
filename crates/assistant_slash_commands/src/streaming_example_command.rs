use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandContent, SlashCommandEvent,
    SlashCommandOutputSection, SlashCommandResult,
};
use feature_flags::FeatureFlag;
use futures::channel::mpsc;
use gpui::{Task, WeakEntity};
use language::{BufferSnapshot, LspAdapterDelegate};
use smol::stream::StreamExt;
use ui::prelude::*;
use workspace::Workspace;

pub struct StreamingExampleSlashCommandFeatureFlag;

impl FeatureFlag for StreamingExampleSlashCommandFeatureFlag {
    const NAME: &'static str = "streaming-example-slash-command";
}

pub struct StreamingExampleSlashCommand;

impl SlashCommand for StreamingExampleSlashCommand {
    fn name(&self) -> String {
        "streaming-example".into()
    }

    fn description(&self) -> String {
        "An example slash command that showcases streaming.".into()
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn requires_argument(&self) -> bool {
        false
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
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let (events_tx, events_rx) = mpsc::unbounded();
        let executor = cx.background_executor().clone();
        cx.background_spawn(async move {
            events_tx.unbounded_send(Ok(SlashCommandEvent::StartSection {
                icon: IconName::FileRust,
                label: "Section 1".into(),
                metadata: None,
            }))?;
            events_tx.unbounded_send(Ok(SlashCommandEvent::Content(
                SlashCommandContent::Text {
                    text: "Hello".into(),
                    run_commands_in_text: false,
                },
            )))?;
            events_tx.unbounded_send(Ok(SlashCommandEvent::EndSection))?;

            executor.timer(Duration::from_secs(1)).await;

            events_tx.unbounded_send(Ok(SlashCommandEvent::StartSection {
                icon: IconName::FileRust,
                label: "Section 2".into(),
                metadata: None,
            }))?;
            events_tx.unbounded_send(Ok(SlashCommandEvent::Content(
                SlashCommandContent::Text {
                    text: "World".into(),
                    run_commands_in_text: false,
                },
            )))?;
            events_tx.unbounded_send(Ok(SlashCommandEvent::EndSection))?;

            for n in 1..=10 {
                executor.timer(Duration::from_secs(1)).await;

                events_tx.unbounded_send(Ok(SlashCommandEvent::StartSection {
                    icon: IconName::StarFilled,
                    label: format!("Section {n}").into(),
                    metadata: None,
                }))?;
                events_tx.unbounded_send(Ok(SlashCommandEvent::Content(
                    SlashCommandContent::Text {
                        text: "lorem ipsum ".repeat(n).trim().into(),
                        run_commands_in_text: false,
                    },
                )))?;
                events_tx.unbounded_send(Ok(SlashCommandEvent::EndSection))?;
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        Task::ready(Ok(events_rx.boxed()))
    }
}
