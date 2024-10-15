use crate::slash_command::file_command::FileSlashCommand;
use crate::slash_command::FileCommandMetadata;
use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandContentType, SlashCommandEvent,
    SlashCommandOutputSection, SlashCommandResult,
};
use collections::HashSet;
use futures::{
    future,
    stream::{self, StreamExt},
};
use gpui::{Task, WeakView, WindowContext};
use language::{BufferSnapshot, LspAdapterDelegate};
use std::sync::{atomic::AtomicBool, Arc};
use text::OffsetRangeExt;
use workspace::Workspace;

pub(crate) struct DeltaSlashCommand;

impl SlashCommand for DeltaSlashCommand {
    fn name(&self) -> String {
        "delta".into()
    }

    fn description(&self) -> String {
        "Re-insert changed files".into()
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
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        unimplemented!()
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        context_buffer: BufferSnapshot,
        workspace: WeakView<Workspace>,
        delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
        let mut paths = HashSet::default();
        let mut file_command_old_outputs = Vec::new();
        let mut file_command_new_outputs = Vec::new();
        for section in context_slash_command_output_sections.iter().rev() {
            if let Some(metadata) = section
                .metadata
                .as_ref()
                .and_then(|value| serde_json::from_value::<FileCommandMetadata>(value.clone()).ok())
            {
                if paths.insert(metadata.path.clone()) {
                    file_command_old_outputs.push(
                        context_buffer
                            .as_rope()
                            .slice(section.range.to_offset(&context_buffer)),
                    );
                    file_command_new_outputs.push(Arc::new(FileSlashCommand).run(
                        &[metadata.path.clone()],
                        context_slash_command_output_sections,
                        context_buffer.clone(),
                        workspace.clone(),
                        delegate.clone(),
                        cx,
                    ));
                }
            }
        }

        cx.background_executor().spawn(async move {
            let mut events = Vec::new();

            let file_command_new_outputs = future::join_all(file_command_new_outputs).await;
            for (old_text, new_output) in file_command_old_outputs
                .into_iter()
                .zip(file_command_new_outputs)
            {
                if let Ok(new_events) = new_output {
                    let new_content = stream::StreamExt::collect::<Vec<_>>(new_events).await;
                    {
                        if let Some(first_content) = new_content.iter().find_map(|event| {
                            if let SlashCommandEvent::Content(SlashCommandContentType::Text {
                                text,
                                ..
                            }) = event
                            {
                                Some(text)
                            } else {
                                None
                            }
                        }) {
                            if old_text.chars().ne(first_content.chars()) {
                                events.extend(new_content);
                            }
                        }
                    }
                }
            }

            Ok(stream::iter(events).boxed())
        })
    }
}
