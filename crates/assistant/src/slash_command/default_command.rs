use super::{prompt_command::PromptPlaceholder, SlashCommand, SlashCommandOutput};
use crate::prompt_library::PromptStore;
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use gpui::{AppContext, Task, WeakView};
use language::LspAdapterDelegate;
use std::{
    fmt::Write,
    sync::{atomic::AtomicBool, Arc},
};
use ui::prelude::*;
use workspace::Workspace;

pub(crate) struct DefaultSlashCommand;

impl SlashCommand for DefaultSlashCommand {
    fn name(&self) -> String {
        "default".into()
    }

    fn description(&self) -> String {
        "insert default prompt".into()
    }

    fn menu_text(&self) -> String {
        "Insert Default Prompt".into()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancellation_flag: Arc<AtomicBool>,
        _workspace: WeakView<Workspace>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn run(
        self: Arc<Self>,
        _argument: Option<&str>,
        _workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let store = PromptStore::global(cx);
        cx.background_executor().spawn(async move {
            let store = store.await?;
            let prompts = store.default_prompt_metadata();

            let mut text = String::new();
            writeln!(text, "Default Prompt:").unwrap();
            for prompt in prompts {
                if let Some(title) = prompt.title {
                    writeln!(text, "/prompt {}", title).unwrap();
                }
            }
            text.pop();

            Ok(SlashCommandOutput {
                sections: vec![SlashCommandOutputSection {
                    range: 0..text.len(),
                    render_placeholder: Arc::new(move |id, unfold, _cx| {
                        PromptPlaceholder {
                            title: "Default".into(),
                            id,
                            unfold,
                        }
                        .into_any_element()
                    }),
                }],
                text,
                run_commands_in_text: true,
            })
        })
    }
}
