use anyhow::{Context as _, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use collections::{HashMap, HashSet};
use editor::Editor;
use gpui::{AsyncWindowContext, Task, WeakEntity};
use language::{BufferSnapshot, LspAdapterDelegate};
use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};
use ui::{App, Window, prelude::*};
use util::ResultExt;
use workspace::Workspace;

use crate::file_command::append_buffer_to_output;

pub struct TabsSlashCommand;

impl SlashCommand for TabsSlashCommand {
    fn name(&self) -> String {
        "tabs".into()
    }

    fn description(&self) -> String {
        "Insert all open tabs".to_owned()
    }

    fn icon(&self) -> IconName {
        IconName::FileTree
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn accepts_arguments(&self) -> bool {
        false
    }

    fn complete_argument(
        self: Arc<Self>,
        _: &[String],
        _: Arc<AtomicBool>,
        _: Option<WeakEntity<Workspace>>,
        _: &mut Window,
        _: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        _: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let tab_items_search = tab_items(Some(workspace), window, cx);

        cx.background_spawn(async move {
            let mut output = SlashCommandOutput::default();
            for (full_path, buffer, _) in tab_items_search.await? {
                append_buffer_to_output(&buffer, full_path.as_deref(), &mut output).log_err();
            }
            Ok(output.to_event_stream())
        })
    }
}

fn tab_items(
    workspace: Option<WeakEntity<Workspace>>,
    window: &mut Window,
    cx: &mut App,
) -> Task<anyhow::Result<Vec<(Option<PathBuf>, BufferSnapshot, usize)>>> {
    window.spawn(cx, async move |cx| {
        let workspace = workspace.context("no workspace")?;
        let mut open_buffers = collect_open_buffers(workspace, cx).await?;

        cx.background_spawn(async move {
            open_buffers.sort_by_key(|(_, _, timestamp)| *timestamp);
            Ok(open_buffers)
        })
        .await
    })
}

async fn collect_open_buffers(
    workspace: WeakEntity<Workspace>,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<Vec<(Option<PathBuf>, BufferSnapshot, usize)>> {
    workspace.update(cx, |workspace, cx| {
        let mut timestamps_by_entity_id = HashMap::default();
        let mut visited_buffers = HashSet::default();
        let mut open_buffers = Vec::new();

        for pane in workspace.panes() {
            let pane = pane.read(cx);
            for entry in pane.activation_history() {
                timestamps_by_entity_id.insert(entry.entity_id, entry.timestamp);
            }
        }

        for editor in workspace.items_of_type::<Editor>(cx) {
            if let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() {
                if let Some(timestamp) = timestamps_by_entity_id.get(&editor.entity_id()) {
                    if visited_buffers.insert(buffer.read(cx).remote_id()) {
                        let snapshot = buffer.read(cx).snapshot();
                        let full_path = snapshot.resolve_file_path(cx, true);
                        open_buffers.push((full_path, snapshot, *timestamp));
                    }
                }
            }
        }

        open_buffers
    })
}
