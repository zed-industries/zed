use super::{
    diagnostics_command::write_single_file_diagnostics,
    file_command::{build_entry_output_section, codeblock_fence_for_path},
    SlashCommand, SlashCommandOutput,
};
use anyhow::Result;
use assistant_slash_command::ArgumentCompletion;
use collections::HashMap;
use editor::Editor;
use gpui::{AppContext, Entity, Task, WeakView};
use language::LspAdapterDelegate;
use std::{fmt::Write, sync::Arc};
use ui::WindowContext;
use workspace::Workspace;

pub(crate) struct TabsSlashCommand;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum TabsArgument {
    #[default]
    Active,
    All,
}

impl TabsArgument {
    fn for_query(mut query: String) -> Vec<Self> {
        query.make_ascii_lowercase();
        let query = query.trim();

        let mut matches = Vec::new();
        if Self::Active.name().contains(&query) {
            matches.push(Self::Active);
        }
        if Self::All.name().contains(&query) {
            matches.push(Self::All);
        }
        matches
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::All => "all",
        }
    }

    fn from_name(name: &str) -> Option<Self> {
        match name {
            "active" => Some(Self::Active),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

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
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        query: String,
        _cancel: Arc<std::sync::atomic::AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let arguments = TabsArgument::for_query(query);
        Task::ready(Ok(arguments
            .into_iter()
            .map(|arg| ArgumentCompletion {
                label: arg.name().to_owned(),
                new_text: arg.name().to_owned(),
                run_command: true,
            })
            .collect()))
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let argument = argument
            .and_then(TabsArgument::from_name)
            .unwrap_or_default();
        let open_buffers = workspace.update(cx, |workspace, cx| match argument {
            TabsArgument::Active => {
                let Some(active_item) = workspace.active_item(cx) else {
                    anyhow::bail!("no active item")
                };
                let Some(buffer) = active_item
                    .downcast::<Editor>()
                    .and_then(|editor| editor.read(cx).buffer().read(cx).as_singleton())
                else {
                    anyhow::bail!("active item is not an editor")
                };
                let snapshot = buffer.read(cx).snapshot();
                let full_path = snapshot.resolve_file_path(cx, true);
                anyhow::Ok(vec![(full_path, snapshot, 0)])
            }
            TabsArgument::All => {
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

                Ok(open_buffers)
            }
        });

        match open_buffers {
            Ok(Ok(mut open_buffers)) => cx.background_executor().spawn(async move {
                open_buffers.sort_by_key(|(_, _, timestamp)| *timestamp);

                let mut sections = Vec::new();
                let mut text = String::new();
                let mut has_diagnostics = false;
                for (full_path, buffer, _) in open_buffers {
                    let section_start_ix = text.len();
                    text.push_str(&codeblock_fence_for_path(full_path.as_deref(), None));
                    for chunk in buffer.as_rope().chunks() {
                        text.push_str(chunk);
                    }
                    if !text.ends_with('\n') {
                        text.push('\n');
                    }
                    writeln!(text, "```").unwrap();
                    if write_single_file_diagnostics(&mut text, full_path.as_deref(), &buffer) {
                        has_diagnostics = true;
                    }
                    if !text.ends_with('\n') {
                        text.push('\n');
                    }

                    let section_end_ix = text.len() - 1;
                    sections.push(build_entry_output_section(
                        section_start_ix..section_end_ix,
                        full_path.as_deref(),
                        false,
                        None,
                    ));
                }

                Ok(SlashCommandOutput {
                    text,
                    sections,
                    run_commands_in_text: has_diagnostics,
                })
            }),
            Ok(Err(error)) | Err(error) => Task::ready(Err(error)),
        }
    }
}
