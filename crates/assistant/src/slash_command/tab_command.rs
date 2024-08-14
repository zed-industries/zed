use super::{
    diagnostics_command::write_single_file_diagnostics,
    file_command::{build_entry_output_section, codeblock_fence_for_path},
    SlashCommand, SlashCommandOutput,
};
use anyhow::{Context, Result};
use assistant_slash_command::ArgumentCompletion;
use collections::{HashMap, HashSet};
use editor::Editor;
use futures::future::join_all;
use gpui::{Entity, Task, WeakView};
use language::{BufferSnapshot, LspAdapterDelegate};
use std::{
    fmt::Write,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};
use ui::WindowContext;
use workspace::Workspace;

pub(crate) struct TabSlashCommand;

const ALL_TABS_COMPLETION_ITEM: &str = "all";

impl SlashCommand for TabSlashCommand {
    fn name(&self) -> String {
        "tab".into()
    }

    fn description(&self) -> String {
        "insert open tabs (active tab by default)".to_owned()
    }

    fn menu_text(&self) -> String {
        "Insert Open Tabs".to_owned()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        cancel: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let mut has_all_tabs_completion_item = false;
        let argument_set = arguments
            .iter()
            .filter(|argument| {
                if has_all_tabs_completion_item || ALL_TABS_COMPLETION_ITEM == argument.as_str() {
                    has_all_tabs_completion_item = true;
                    false
                } else {
                    true
                }
            })
            .cloned()
            .collect::<HashSet<_>>();
        if has_all_tabs_completion_item {
            return Task::ready(Ok(Vec::new()));
        }

        let active_item_path = workspace.as_ref().and_then(|workspace| {
            workspace
                .update(cx, |workspace, cx| {
                    let snapshot = active_item_buffer(workspace, cx).ok()?;
                    snapshot.resolve_file_path(cx, true)
                })
                .ok()
                .flatten()
        });
        let current_query = arguments.last().cloned().unwrap_or_default();
        let tab_items_search =
            tab_items_for_queries(workspace, &[current_query], cancel, false, cx);
        cx.spawn(|_| async move {
            let tab_items = tab_items_search.await?;
            let run_command = tab_items.len() == 1;
            let tab_completion_items = tab_items.into_iter().filter_map(|(path, ..)| {
                let path_string = path.as_deref()?.to_string_lossy().to_string();
                if argument_set.contains(&path_string) {
                    return None;
                }
                if active_item_path.is_some() && active_item_path == path {
                    return None;
                }
                Some(ArgumentCompletion {
                    label: path_string.clone().into(),
                    new_text: path_string,
                    replace_previous_arguments: false,
                    run_command,
                })
            });

            let active_item_completion = active_item_path.as_deref().map(|active_item_path| {
                let path_string = active_item_path.to_string_lossy().to_string();
                ArgumentCompletion {
                    label: path_string.clone().into(),
                    new_text: path_string,
                    replace_previous_arguments: false,
                    run_command,
                }
            });

            Ok(active_item_completion
                .into_iter()
                .chain(Some(ArgumentCompletion {
                    label: ALL_TABS_COMPLETION_ITEM.into(),
                    new_text: ALL_TABS_COMPLETION_ITEM.to_owned(),
                    replace_previous_arguments: false,
                    run_command: true,
                }))
                .chain(tab_completion_items)
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let tab_items_search = tab_items_for_queries(
            Some(workspace),
            arguments,
            Arc::new(AtomicBool::new(false)),
            true,
            cx,
        );

        cx.background_executor().spawn(async move {
            let mut sections = Vec::new();
            let mut text = String::new();
            let mut has_diagnostics = false;
            for (full_path, buffer, _) in tab_items_search.await? {
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
        })
    }
}

fn tab_items_for_queries(
    workspace: Option<WeakView<Workspace>>,
    queries: &[String],
    cancel: Arc<AtomicBool>,
    strict_match: bool,
    cx: &mut WindowContext,
) -> Task<anyhow::Result<Vec<(Option<PathBuf>, BufferSnapshot, usize)>>> {
    let empty_query = queries.is_empty() || queries.iter().all(|query| query.trim().is_empty());
    let queries = queries.to_owned();
    cx.spawn(|mut cx| async move {
        let mut open_buffers =
            workspace
                .context("no workspace")?
                .update(&mut cx, |workspace, cx| {
                    if strict_match && empty_query {
                        let snapshot = active_item_buffer(workspace, cx)?;
                        let full_path = snapshot.resolve_file_path(cx, true);
                        return anyhow::Ok(vec![(full_path, snapshot, 0)]);
                    }

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
                            if let Some(timestamp) =
                                timestamps_by_entity_id.get(&editor.entity_id())
                            {
                                let snapshot = buffer.read(cx).snapshot();
                                let full_path = snapshot.resolve_file_path(cx, true);
                                open_buffers.push((full_path, snapshot, *timestamp));
                            }
                        }
                    }

                    Ok(open_buffers)
                })??;

        let background_executor = cx.background_executor().clone();
        cx.background_executor()
            .spawn(async move {
                open_buffers.sort_by_key(|(_, _, timestamp)| *timestamp);
                if empty_query
                    || queries
                        .iter()
                        .any(|query| query == ALL_TABS_COMPLETION_ITEM)
                {
                    return Ok(open_buffers);
                }

                let matched_items = if strict_match {
                    let match_candidates = open_buffers
                        .iter()
                        .enumerate()
                        .filter_map(|(id, (full_path, ..))| {
                            let path_string = full_path.as_deref()?.to_string_lossy().to_string();
                            Some((id, path_string))
                        })
                        .fold(HashMap::default(), |mut candidates, (id, path_string)| {
                            candidates
                                .entry(path_string)
                                .or_insert_with(|| Vec::new())
                                .push(id);
                            candidates
                        });

                    queries
                        .iter()
                        .filter_map(|query| match_candidates.get(query))
                        .flatten()
                        .copied()
                        .filter_map(|id| open_buffers.get(id))
                        .cloned()
                        .collect()
                } else {
                    let match_candidates = open_buffers
                        .iter()
                        .enumerate()
                        .filter_map(|(id, (full_path, ..))| {
                            let path_string = full_path.as_deref()?.to_string_lossy().to_string();
                            Some(fuzzy::StringMatchCandidate {
                                id,
                                char_bag: path_string.as_str().into(),
                                string: path_string,
                            })
                        })
                        .collect::<Vec<_>>();
                    let mut processed_matches = HashSet::default();
                    let file_queries = queries.iter().map(|query| {
                        fuzzy::match_strings(
                            &match_candidates,
                            query,
                            true,
                            usize::MAX,
                            &cancel,
                            background_executor.clone(),
                        )
                    });

                    join_all(file_queries)
                        .await
                        .into_iter()
                        .flatten()
                        .filter(|string_match| processed_matches.insert(string_match.candidate_id))
                        .filter_map(|string_match| open_buffers.get(string_match.candidate_id))
                        .cloned()
                        .collect()
                };
                Ok(matched_items)
            })
            .await
    })
}

fn active_item_buffer(
    workspace: &mut Workspace,
    cx: &mut ui::ViewContext<Workspace>,
) -> anyhow::Result<BufferSnapshot> {
    let active_editor = workspace
        .active_item(cx)
        .context("no active item")?
        .downcast::<Editor>()
        .context("active item is not an editor")?;
    let snapshot = active_editor
        .read(cx)
        .buffer()
        .read(cx)
        .as_singleton()
        .context("active editor is not a singleton buffer")?
        .read(cx)
        .snapshot();
    Ok(snapshot)
}
