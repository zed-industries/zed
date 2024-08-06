use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use assets::Assets;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};
use gpui::{AppContext, AssetSource, Task, WeakView};
use language::LspAdapterDelegate;
use text::LineEnding;
use ui::prelude::*;
use workspace::Workspace;

pub(crate) struct WorkflowSlashCommand;

impl SlashCommand for WorkflowSlashCommand {
    fn name(&self) -> String {
        "workflow".into()
    }

    fn description(&self) -> String {
        "insert a prompt that opts into the edit workflow".into()
    }

    fn menu_text(&self) -> String {
        "Insert Workflow Prompt".into()
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
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        _argument: Option<&str>,
        _workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let mut text = match Assets
            .load("prompts/edit_workflow.md")
            .and_then(|prompt| prompt.context("prompts/edit_workflow.md not found"))
        {
            Ok(prompt) => String::from_utf8_lossy(&prompt).into_owned(),
            Err(error) => return Task::ready(Err(error)),
        };
        LineEnding::normalize(&mut text);
        let range = 0..text.len();

        Task::ready(Ok(SlashCommandOutput {
            text,
            sections: vec![SlashCommandOutputSection {
                range,
                icon: IconName::Route,
                label: "Workflow".into(),
            }],
            run_commands_in_text: false,
        }))
    }
}
