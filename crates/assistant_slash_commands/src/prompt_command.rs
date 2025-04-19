use anyhow::{Context as _, Result, anyhow};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use gpui::{Task, WeakEntity};
use language::{BufferSnapshot, LspAdapterDelegate};
use prompt_store::{PromptMetadata, PromptStore};
use std::sync::{Arc, atomic::AtomicBool};
use ui::prelude::*;
use workspace::Workspace;

pub struct PromptSlashCommand;

impl SlashCommand for PromptSlashCommand {
    fn name(&self) -> String {
        "prompt".into()
    }

    fn description(&self) -> String {
        "Insert prompt from library".into()
    }

    fn icon(&self) -> IconName {
        IconName::Library
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        _cancellation_flag: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let store = PromptStore::global(cx);
        let query = arguments.to_owned().join(" ");
        cx.spawn(async move |cx| {
            let prompts: Vec<PromptMetadata> = store
                .await?
                .read_with(cx, |store, cx| store.search(query, cx))?
                .await;
            Ok(prompts
                .into_iter()
                .filter_map(|prompt| {
                    let prompt_title = prompt.title?.to_string();
                    Some(ArgumentCompletion {
                        label: prompt_title.clone().into(),
                        new_text: prompt_title,
                        after_completion: true.into(),
                        replace_previous_arguments: true,
                    })
                })
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let title = arguments.to_owned().join(" ");
        if title.trim().is_empty() {
            return Task::ready(Err(anyhow!("missing prompt name")));
        };

        let store = PromptStore::global(cx);
        let title = SharedString::from(title.clone());
        let prompt = cx.spawn({
            let title = title.clone();
            async move |cx| {
                let store = store.await?;
                let body = store
                    .read_with(cx, |store, cx| {
                        let prompt_id = store
                            .id_for_title(&title)
                            .with_context(|| format!("no prompt found with title {:?}", title))?;
                        anyhow::Ok(store.load(prompt_id, cx))
                    })??
                    .await?;
                anyhow::Ok(body)
            }
        });
        cx.foreground_executor().spawn(async move {
            let mut prompt = prompt.await?;

            if prompt.starts_with('/') {
                // Prevent an edge case where the inserted prompt starts with a slash command (that leads to funky rendering).
                prompt.insert(0, '\n');
            }
            if prompt.is_empty() {
                prompt.push('\n');
            }
            let range = 0..prompt.len();
            Ok(SlashCommandOutput {
                text: prompt,
                sections: vec![SlashCommandOutputSection {
                    range,
                    icon: IconName::Library,
                    label: title,
                    metadata: None,
                }],
                run_commands_in_text: true,
            }
            .to_event_stream())
        })
    }
}
