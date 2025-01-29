use anyhow::{anyhow, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use gpui::{Task, WeakEntity};
use language::{BufferSnapshot, LspAdapterDelegate};
use prompt_library::PromptStore;
use std::{
    fmt::Write,
    sync::{atomic::AtomicBool, Arc},
};
use ui::prelude::*;
use workspace::Workspace;

pub struct DefaultSlashCommand;

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
        cx.background_executor().spawn(async move {
            let store = store.await?;
            let prompts = store.default_prompt_metadata();

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
            .to_event_stream())
        })
    }
}
