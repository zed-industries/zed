use anyhow::{Result, anyhow};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use gpui::{Task, WeakEntity};
use language::{BufferSnapshot, LspAdapterDelegate};
use prompt_store::PromptStore;
use std::{
    fmt::Write,
    sync::{Arc, atomic::AtomicBool},
};
use ui::prelude::*;
use workspace::Workspace;

pub struct DefaultSlashCommand;

impl SlashCommand for DefaultSlashCommand {
    fn name(&self) -> String {
        "default".into()
    }

    fn description(&self) -> String {
        "Insert default prompt".into()
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
        _cancellation_flag: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let store = PromptStore::global(cx);
        cx.spawn(async move |cx| {
            let store = store.await?;
            let prompts = store.read_with(cx, |store, _cx| store.default_prompt_metadata())?;

            let mut text = String::new();
            text.push('\n');
            for prompt in prompts {
                if let Some(title) = prompt.title {
                    writeln!(text, "/prompt {}", title).unwrap();
                }
            }
            text.pop();

            if text.is_empty() {
                text.push('\n');
            }

            if !text.ends_with('\n') {
                text.push('\n');
            }

            Ok(SlashCommandOutput {
                sections: vec![SlashCommandOutputSection {
                    range: 0..text.len(),
                    icon: IconName::Library,
                    label: "Default".into(),
                    metadata: None,
                }],
                text,
                run_commands_in_text: true,
            }
            .into_event_stream())
        })
    }
}
