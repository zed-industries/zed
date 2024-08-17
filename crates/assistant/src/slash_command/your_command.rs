use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};
use gpui::{Task, WeakView};
use language::LspAdapterDelegate;
use ui::prelude::*;
use workspace::Workspace;

pub(crate) struct YourSlashCommand;

impl SlashCommand for YourSlashCommand {
    fn name(&self) -> String {
        "your-command".into()
    }

    fn description(&self) -> String {
        "Insert docs about creating a custom command".into()
    }

    fn menu_text(&self) -> String {
        "Insert docs about creating a custom command".into()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn accepts_arguments(&self) -> bool {
        false
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
        _arguments: &[String],
        _workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        cx.open_url("https://zed.dev/docs/extensions/slash-commands");

        Task::ready(Ok(SlashCommandOutput {
            text: "Slash commands docs".to_string(),
            sections: vec![SlashCommandOutputSection {
                range: 0..23,
                icon: IconName::FileDoc,
                label: "https://zed.dev/docs/extensions/slash-commands".into(),
            }],
            run_commands_in_text: false,
        }))
    }
}
