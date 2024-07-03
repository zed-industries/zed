use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::Result;
use assistant_slash_command::{SlashCommand, SlashCommandOutput, SlashCommandOutputSection};
use chrono::Local;
use gpui::{AppContext, Task, WeakView};
use language::LspAdapterDelegate;
use ui::prelude::*;
use workspace::Workspace;

pub(crate) struct NowSlashCommand;

impl SlashCommand for NowSlashCommand {
    fn name(&self) -> String {
        "now".into()
    }

    fn description(&self) -> String {
        "insert the current date and time".into()
    }

    fn menu_text(&self) -> String {
        "Insert current date and time".into()
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
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        _argument: Option<&str>,
        _workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        _cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let now = Local::now();
        let text = format!("Today is {now}.", now = now.to_rfc2822());
        let range = 0..text.len();

        Task::ready(Ok(SlashCommandOutput {
            text,
            sections: vec![SlashCommandOutputSection {
                range,
                icon: IconName::CountdownTimer,
                label: now.to_rfc2822().into(),
            }],
            run_commands_in_text: false,
        }))
    }
}
