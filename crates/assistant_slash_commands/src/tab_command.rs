use anyhow::{Context as _, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use collections::{HashMap, HashSet};
use editor::Editor;
use futures::future::join_all;
use gpui::{Task, WeakEntity};
use language::{BufferSnapshot, CodeLabel, HighlightId, LspAdapterDelegate};
use std::sync::{Arc, atomic::AtomicBool};
use ui::{ActiveTheme, App, Window, prelude::*};
use util::{ResultExt, paths::PathStyle};
use workspace::Workspace;

use crate::file_command::append_buffer_to_output;

pub struct TabSlashCommand;

const ALL_TABS_COMPLETION_ITEM: &str = "all";

impl SlashCommand for TabSlashCommand {
    fn name(&self) -> String {
        "tab".into()
    }

    fn description(&self) -> String {
        "Insert open tabs (active tab by default)".to_owned()
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
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        cancel: Arc<AtomicBool>,
        workspace: Option<WeakEntity<Workspace>>,
        window: &mut Window,
        cx: &mut App,
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

        let Some(workspace) = workspace.and_then(|workspace| workspace.upgrade()) else {
            return Task::ready(Err(anyhow::anyhow!("no workspace")));
        };

        let active_item_path = workspace.update(cx, |workspace, cx| {
            let snapshot = active_item_buffer(workspace, cx).ok()?;
            snapshot.resolve_file_path(true, cx)
        });
        let path_style = workspace.read(cx).path_style(cx);

        let current_query = arguments.last().cloned().unwrap_or_default();
        let tab_items_search = tab_items_for_queries(
            Some(workspace.downgrade()),
            &[current_query],
            cancel,
            false,
            window,
            cx,
        );

        let comment_id = cx.theme().syntax().highlight_id("comment").map(HighlightId);
        window.spawn(cx, async move |_| {
            let tab_items = tab_items_search.await?;
            let run_command = tab_items.len() == 1;
            let tab_completion_items = tab_items.into_iter().filter_map(|(path, ..)| {
                let path = path?;
                if argument_set.contains(&path) {
                    return None;
                }
                if active_item_path.as_ref() == Some(&path) {
                    return None;
                }
                let label = create_tab_completion_label(&path, path_style, comment_id);
                Some(ArgumentCompletion {
                    label,
                    new_text: path,
                    replace_previous_arguments: false,
                    after_completion: run_command.into(),
                })
            });

            let active_item_completion = active_item_path
                .as_deref()
                .map(|active_item_path| {
                    let path_string = active_item_path.to_string();
                    let label =
                        create_tab_completion_label(active_item_path, path_style, comment_id);
                    ArgumentCompletion {
                        label,
                        new_text: path_string,
                        replace_previous_arguments: false,
                        after_completion: run_command.into(),
                    }
                })
                .filter(|completion| !argument_set.contains(&completion.new_text));

            Ok(active_item_completion
                .into_iter()
                .chain(Some(ArgumentCompletion {
                    label: ALL_TABS_COMPLETION_ITEM.into(),
                    new_text: ALL_TABS_COMPLETION_ITEM.to_owned(),
                    replace_previous_arguments: false,
                    after_completion: true.into(),
                }))
                .chain(tab_completion_items)
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let tab_items_search = tab_items_for_queries(
            Some(workspace),
            arguments,
            Arc::new(AtomicBool::new(false)),
            true,
            window,
            cx,
        );

        cx.background_spawn(async move {
            let mut output = SlashCommandOutput::default();
            for (full_path, buffer, _) in tab_items_search.await? {
                append_buffer_to_output(&buffer, full_path.as_deref(), &mut output).log_err();
            }
            Ok(output.into_event_stream())
        })
    }
}

fn tab_items_for_queries(
    workspace: Option<WeakEntity<Workspace>>,
    queries: &[String],
    cancel: Arc<AtomicBool>,
    strict_match: bool,
    window: &mut Window,
    cx: &mut App,
) -> Task<anyhow::Result<Vec<(Option<String>, BufferSnapshot, usize)>>> {
    let empty_query = queries.is_empty() || queries.iter().all(|query| query.trim().is_empty());
    let queries = queries.to_owned();
    window.spawn(cx, async move |cx| {
        let mut open_buffers =
            workspace
                .context("no workspace")?
                .update(cx, |workspace, cx| {
                    if strict_match && empty_query {
                        let snapshot = active_item_buffer(workspace, cx)?;
                        let full_path = snapshot.resolve_file_path(true, cx);
                        return anyhow::Ok(vec![(full_path, snapshot, 0)]);
                    }

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
                        if let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton()
                            && let Some(timestamp) =
                                timestamps_by_entity_id.get(&editor.entity_id())
                            && visited_buffers.insert(buffer.read(cx).remote_id())
                        {
                            let snapshot = buffer.read(cx).snapshot();
                            let full_path = snapshot.resolve_file_path(true, cx);
                            open_buffers.push((full_path, snapshot, *timestamp));
                        }
                    }

                    Ok(open_buffers)
                })??;

        let background_executor = cx.background_executor().clone();
        cx.background_spawn(async move {
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
                    .filter_map(|(id, (full_path, ..))| Some((id, full_path.clone()?)))
                    .fold(HashMap::default(), |mut candidates, (id, path_string)| {
                        candidates
                            .entry(path_string)
                            .or_insert_with(Vec::new)
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
                        Some(fuzzy::StringMatchCandidate::new(id, full_path.as_ref()?))
                    })
                    .collect::<Vec<_>>();
                let mut processed_matches = HashSet::default();
                let file_queries = queries.iter().map(|query| {
                    fuzzy::match_strings(
                        &match_candidates,
                        query,
                        true,
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
    cx: &mut Context<Workspace>,
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

fn create_tab_completion_label(
    path: &str,
    path_style: PathStyle,
    comment_id: Option<HighlightId>,
) -> CodeLabel {
    let (parent_path, file_name) = path_style.split(path);
    let mut label = CodeLabel::default();
    label.push_str(file_name, None);
    label.push_str(" ", None);
    label.push_str(parent_path.unwrap_or_default(), comment_id);
    label.filter_range = 0..file_name.len();
    label
}
