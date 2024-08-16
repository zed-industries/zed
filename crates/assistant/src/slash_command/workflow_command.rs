use crate::prompts::PromptBuilder;
use std::sync::Arc;

use std::sync::atomic::AtomicBool;

use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};
use gpui::{Task, WeakView};
use language::LspAdapterDelegate;
use ui::prelude::*;

use workspace::Workspace;

pub(crate) struct WorkflowSlashCommand {
    prompt_builder: Arc<PromptBuilder>,
}

impl WorkflowSlashCommand {
    pub fn new(prompt_builder: Arc<PromptBuilder>) -> Self {
        Self { prompt_builder }
    }
}

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
        let prompt_builder = self.prompt_builder.clone();
        cx.spawn(|_cx| async move {
            let text = prompt_builder.generate_workflow_prompt()?;
            let range = 0..text.len();

            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    icon: IconName::Route,
                    label: "Workflow".into(),
                }],
                run_commands_in_text: false,
            })
        })
    }
}
