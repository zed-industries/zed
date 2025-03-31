use crate::file_command::{FileCommandMetadata, FileSlashCommand};
use anyhow::{Result, anyhow};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use collections::HashSet;
use futures::future;
use gpui::{App, Task, WeakEntity, Window};
use language::{BufferSnapshot, LspAdapterDelegate};
use std::sync::{Arc, atomic::AtomicBool};
use text::OffsetRangeExt;
use ui::prelude::*;
use workspace::Workspace;

pub struct DeltaSlashCommand;

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

    fn icon(&self) -> IconName {
        IconName::Diff
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
        context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        context_buffer: BufferSnapshot,
        workspace: WeakEntity<Workspace>,
        delegate: Option<Arc<dyn LspAdapterDelegate>>,
        window: &mut Window,
        cx: &mut App,
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
                        window,
                        cx,
                    ));
                }
            }
        }

        cx.background_spawn(async move {
            let mut output = SlashCommandOutput::default();
            let mut changes_detected = false;

            let file_command_new_outputs = future::join_all(file_command_new_outputs).await;
            for (old_text, new_output) in file_command_old_outputs
                .into_iter()
                .zip(file_command_new_outputs)
            {
                if let Ok(new_output) = new_output {
                    if let Ok(new_output) = SlashCommandOutput::from_event_stream(new_output).await
                    {
                        if let Some(file_command_range) = new_output.sections.first() {
                            let new_text = &new_output.text[file_command_range.range.clone()];
                            if old_text.chars().ne(new_text.chars()) {
                                changes_detected = true;
                                output.sections.extend(new_output.sections.into_iter().map(
                                    |section| SlashCommandOutputSection {
                                        range: output.text.len() + section.range.start
                                            ..output.text.len() + section.range.end,
                                        icon: section.icon,
                                        label: section.label,
                                        metadata: section.metadata,
                                    },
                                ));
                                output.text.push_str(&new_output.text);
                            }
                        }
                    }
                }
            }

            if !changes_detected {
                return Err(anyhow!("no new changes detected"));
            }

            Ok(output.to_event_stream())
        })
    }
}
