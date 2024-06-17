use super::{
    file_command::{codeblock_fence_for_path, EntryPlaceholder},
    SlashCommand, SlashCommandOutput,
};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use collections::HashMap;
use editor::Editor;
use gpui::{AppContext, Entity, Task, WeakView};
use language::LspAdapterDelegate;
use std::{fmt::Write, sync::Arc};
use ui::{IntoElement, WindowContext};
use workspace::Workspace;

pub(crate) struct TabsSlashCommand;

impl SlashCommand for TabsSlashCommand {
    fn name(&self) -> String {
        "tabs".into()
    }

    fn description(&self) -> String {
        "insert open tabs".into()
    }

    fn menu_text(&self) -> String {
        "Insert Open Tabs".into()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancel: Arc<std::sync::atomic::AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn run(
        self: Arc<Self>,
        _argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let open_buffers = workspace.update(cx, |workspace, cx| {
            let mut timestamps_by_entity_id = HashMap::default();
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
                        let snapshot = buffer.read(cx).snapshot();
                        let full_path = snapshot.resolve_file_path(cx, true);
                        open_buffers.push((full_path, snapshot, *timestamp));
                    }
                }
            }

            open_buffers
        });

        match open_buffers {
            Ok(mut open_buffers) => cx.background_executor().spawn(async move {
                open_buffers.sort_by_key(|(_, _, timestamp)| *timestamp);

                let mut sections = Vec::new();
                let mut text = String::new();
                for (full_path, buffer, _) in open_buffers {
                    let section_start_ix = text.len();
                    text.push_str(&codeblock_fence_for_path(full_path.as_deref(), None));
                    for chunk in buffer.as_rope().chunks() {
                        text.push_str(chunk);
                    }
                    if !text.ends_with('\n') {
                        text.push('\n');
                    }
                    writeln!(text, "```\n").unwrap();
                    let section_end_ix = text.len() - 1;

                    sections.push(SlashCommandOutputSection {
                        range: section_start_ix..section_end_ix,
                        render_placeholder: Arc::new(move |id, unfold, _| {
                            EntryPlaceholder {
                                id,
                                path: full_path.clone(),
                                is_directory: false,
                                line_range: None,
                                unfold,
                            }
                            .into_any_element()
                        }),
                    });
                }

                Ok(SlashCommandOutput {
                    text,
                    sections,
                    run_commands_in_text: false,
                })
            }),
            Err(error) => Task::ready(Err(error)),
        }
    }
}
