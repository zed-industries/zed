use anyhow::{Context, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};

use gpui::{Task, WeakView};
use language::{BufferSnapshot, LspAdapterDelegate};
use std::{
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, WindowContext};
use util::ResultExt;
use workspace::Workspace;

use crate::file_command::append_buffer_to_output;
use crate::tab_command::get_open_buffers;

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
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        return Task::ready(Ok(Vec::new()));
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
        let tab_items_search = tab_items(Some(workspace), cx);

        cx.background_executor().spawn(async move {
            let mut output = SlashCommandOutput::default();
            for (full_path, buffer, _) in tab_items_search.await? {
                append_buffer_to_output(&buffer, full_path.as_deref(), &mut output).log_err();
            }
            Ok(output.to_event_stream())
        })
    }
}

fn tab_items(
    workspace: Option<WeakView<Workspace>>,
    cx: &mut WindowContext,
) -> Task<anyhow::Result<Vec<(Option<PathBuf>, BufferSnapshot, usize)>>> {
    cx.spawn(|mut cx| async move {
        let open_buffers = workspace
            .context("no workspace")?
            .update(&mut cx, |workspace, cx| {
                anyhow::Ok(get_open_buffers(workspace, cx))
            })??;
        return Ok(open_buffers);
    })
}
