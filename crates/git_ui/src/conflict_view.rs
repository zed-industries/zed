use agent_settings::AgentSettings;
use collections::{HashMap, HashSet};
use editor::{
    ConflictsOurs, ConflictsOursMarker, ConflictsOuter, ConflictsTheirs, ConflictsTheirsMarker,
    Editor, EditorEvent, MultiBuffer, RowHighlightOptions, SelectionEffects,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
    scroll::Autoscroll,
};
use gpui::{
    App, ClickEvent, Context, Empty, Entity, InteractiveElement as _, ParentElement as _,
    Subscription, Task, WeakEntity,
};
use language::{Anchor, Buffer, BufferId, ToOffset as _, ToPoint as _};
use project::{
    ConflictRegion, ConflictSet, ConflictSetSnapshot, ConflictSetUpdate, Project, ProjectItem as _,
    ProjectPath,
    git_store::{GitStore, GitStoreEvent, RepositoryEvent},
};
use settings::Settings;
use std::{ops::Range, sync::Arc};
use ui::{ButtonLike, Divider, Tooltip, prelude::*};
use util::{ResultExt as _, debug_panic, maybe};
use workspace::{StatusItemView, Workspace, item::ItemHandle};
use zed_actions::agent::{
    ConflictContent, ResolveConflictedFilesWithAgent, ResolveConflictsWithAgent,
};

pub(crate) struct ConflictAddon {
    buffers: HashMap<BufferId, BufferConflicts>,
}

impl ConflictAddon {
    pub(crate) fn conflict_set(&self, buffer_id: BufferId) -> Option<Entity<ConflictSet>> {
        self.buffers
            .get(&buffer_id)
            .map(|entry| entry.conflict_set.clone())
    }
}

struct BufferConflicts {
    block_ids: Vec<(Range<Anchor>, CustomBlockId)>,
    conflict_set: Entity<ConflictSet>,
    _subscription: Subscription,
}

impl editor::Addon for ConflictAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

pub fn register_editor(editor: &mut Editor, buffer: Entity<MultiBuffer>, cx: &mut Context<Editor>) {
    // Only show conflict UI for singletons and in the project diff.
    if !editor.mode().is_full()
        || (!editor.buffer().read(cx).is_singleton()
            && !editor.buffer().read(cx).all_diff_hunks_expanded())
        || editor.read_only(cx)
    {
        return;
    }

    editor.register_addon(ConflictAddon {
        buffers: Default::default(),
    });

    let buffers = buffer.read(cx).all_buffers();
    for buffer in buffers {
        buffer_ranges_updated(editor, buffer, cx);
    }

    cx.subscribe(&cx.entity(), |editor, _, event, cx| match event {
        EditorEvent::BufferRangesUpdated { buffer, .. } => {
            buffer_ranges_updated(editor, buffer.clone(), cx)
        }
        EditorEvent::BuffersRemoved { removed_buffer_ids } => {
            buffers_removed(editor, removed_buffer_ids, cx)
        }
        _ => {}
    })
    .detach();
}

fn buffer_ranges_updated(editor: &mut Editor, buffer: Entity<Buffer>, cx: &mut Context<Editor>) {
    let Some(project) = editor.project() else {
        return;
    };
    let git_store = project.read(cx).git_store().clone();

    let buffer_conflicts = editor
        .addon_mut::<ConflictAddon>()
        .unwrap()
        .buffers
        .entry(buffer.read(cx).remote_id())
        .or_insert_with(|| {
            let conflict_set = git_store.update(cx, |git_store, cx| {
                git_store.open_conflict_set(buffer.clone(), cx)
            });
            let subscription = cx.subscribe(&conflict_set, conflicts_updated);
            BufferConflicts {
                block_ids: Vec::new(),
                conflict_set,
                _subscription: subscription,
            }
        });

    let conflict_set = buffer_conflicts.conflict_set.clone();
    let conflicts_len = conflict_set.read(cx).snapshot().conflicts.len();
    let addon_conflicts_len = buffer_conflicts.block_ids.len();
    conflicts_updated(
        editor,
        conflict_set,
        &ConflictSetUpdate {
            buffer_range: None,
            old_range: 0..addon_conflicts_len,
            new_range: 0..conflicts_len,
        },
        cx,
    );
}

fn buffers_removed(editor: &mut Editor, removed_buffer_ids: &[BufferId], cx: &mut Context<Editor>) {
    let mut removed_block_ids = HashSet::default();
    editor
        .addon_mut::<ConflictAddon>()
        .unwrap()
        .buffers
        .retain(|buffer_id, buffer| {
            if removed_buffer_ids.contains(buffer_id) {
                removed_block_ids.extend(buffer.block_ids.iter().map(|(_, block_id)| *block_id));
                false
            } else {
                true
            }
        });
    editor.remove_blocks(removed_block_ids, None, cx);
}

#[ztracing::instrument(skip_all)]
fn conflicts_updated(
    editor: &mut Editor,
    conflict_set: Entity<ConflictSet>,
    event: &ConflictSetUpdate,
    cx: &mut Context<Editor>,
) {
    let buffer_id = conflict_set.read(cx).snapshot.buffer_id;
    let conflict_set = conflict_set.read(cx).snapshot();
    let multibuffer = editor.buffer().read(cx);
    let snapshot = multibuffer.snapshot(cx);
    let old_range = maybe!({
        let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
        let buffer_conflicts = conflict_addon.buffers.get(&buffer_id)?;
        match buffer_conflicts.block_ids.get(event.old_range.clone()) {
            Some(_) => Some(event.old_range.clone()),
            None => {
                debug_panic!(
                    "conflicts updated event old range is invalid for buffer conflicts view (block_ids len is {:?}, old_range is {:?})",
                    buffer_conflicts.block_ids.len(),
                    event.old_range,
                );
                if event.old_range.start <= event.old_range.end {
                    Some(
                        event.old_range.start.min(buffer_conflicts.block_ids.len())
                            ..event.old_range.end.min(buffer_conflicts.block_ids.len()),
                    )
                } else {
                    None
                }
            }
        }
    });

    // Remove obsolete highlights and blocks
    let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
    if let Some((buffer_conflicts, old_range)) = conflict_addon
        .buffers
        .get_mut(&buffer_id)
        .zip(old_range.clone())
    {
        let old_conflicts = buffer_conflicts.block_ids[old_range].to_owned();
        let mut removed_highlighted_ranges = Vec::new();
        let mut removed_block_ids = HashSet::default();
        for (conflict_range, block_id) in old_conflicts {
            let Some(range) = snapshot.buffer_anchor_range_to_anchor_range(conflict_range) else {
                continue;
            };
            removed_highlighted_ranges.push(range.clone());
            removed_block_ids.insert(block_id);
        }

        editor.remove_gutter_highlights::<ConflictsOuter>(removed_highlighted_ranges.clone(), cx);

        editor.remove_highlighted_rows::<ConflictsOuter>(removed_highlighted_ranges.clone(), cx);
        editor.remove_highlighted_rows::<ConflictsOurs>(removed_highlighted_ranges.clone(), cx);
        editor
            .remove_highlighted_rows::<ConflictsOursMarker>(removed_highlighted_ranges.clone(), cx);
        editor.remove_highlighted_rows::<ConflictsTheirs>(removed_highlighted_ranges.clone(), cx);
        editor.remove_highlighted_rows::<ConflictsTheirsMarker>(
            removed_highlighted_ranges.clone(),
            cx,
        );
        editor.remove_blocks(removed_block_ids, None, cx);
    }

    // Add new highlights and blocks
    let editor_handle = cx.weak_entity();
    let new_conflicts = &conflict_set.conflicts[event.new_range.clone()];
    let mut blocks = Vec::new();
    for conflict in new_conflicts {
        update_conflict_highlighting(editor, conflict, &snapshot, cx);

        let Some(anchor) = snapshot.anchor_in_excerpt(conflict.range.start) else {
            continue;
        };

        let editor_handle = editor_handle.clone();
        blocks.push(BlockProperties {
            placement: BlockPlacement::Above(anchor),
            height: Some(1),
            style: BlockStyle::Sticky,
            render: Arc::new({
                let conflict = conflict.clone();
                move |cx| render_conflict_buttons(&conflict, editor_handle.clone(), cx)
            }),
            priority: 0,
        })
    }
    let new_block_ids = editor.insert_blocks(blocks, None, cx);

    let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
    if let Some((buffer_conflicts, old_range)) =
        conflict_addon.buffers.get_mut(&buffer_id).zip(old_range)
    {
        buffer_conflicts.block_ids.splice(
            old_range,
            new_conflicts
                .iter()
                .map(|conflict| conflict.range.clone())
                .zip(new_block_ids),
        );
    }
}

#[ztracing::instrument(skip_all)]
fn update_conflict_highlighting(
    editor: &mut Editor,
    conflict: &ConflictRegion,
    buffer: &editor::MultiBufferSnapshot,
    cx: &mut Context<Editor>,
) -> Option<()> {
    log::debug!("update conflict highlighting for {conflict:?}");

    let outer = buffer.buffer_anchor_range_to_anchor_range(conflict.range.clone())?;
    let ours = buffer.buffer_anchor_range_to_anchor_range(conflict.ours.clone())?;
    let theirs = buffer.buffer_anchor_range_to_anchor_range(conflict.theirs.clone())?;

    let ours_background = cx.theme().colors().version_control_conflict_marker_ours;
    let theirs_background = cx.theme().colors().version_control_conflict_marker_theirs;

    let options = RowHighlightOptions {
        include_gutter: true,
        ..Default::default()
    };

    editor.insert_gutter_highlight::<ConflictsOuter>(
        outer.start..theirs.end,
        |cx| cx.theme().colors().editor_background,
        cx,
    );

    // Prevent diff hunk highlighting within the entire conflict region.
    editor.highlight_rows::<ConflictsOuter>(outer.clone(), theirs_background, options, cx);
    editor.highlight_rows::<ConflictsOurs>(ours.clone(), ours_background, options, cx);
    editor.highlight_rows::<ConflictsOursMarker>(
        outer.start..ours.start,
        ours_background,
        options,
        cx,
    );
    editor.highlight_rows::<ConflictsTheirs>(theirs.clone(), theirs_background, options, cx);
    editor.highlight_rows::<ConflictsTheirsMarker>(
        theirs.end..outer.end,
        theirs_background,
        options,
        cx,
    );

    Some(())
}

fn render_conflict_buttons(
    conflict: &ConflictRegion,
    editor: WeakEntity<Editor>,
    cx: &mut BlockContext,
) -> AnyElement {
    let is_ai_enabled = AgentSettings::get_global(cx).enabled(cx);

    h_flex()
        .id(cx.block_id)
        .h(cx.line_height)
        .ml(cx.margins.gutter.width)
        .gap_1()
        .bg(cx.theme().colors().editor_background)
        .child(
            Button::new("head", format!("Use {}", conflict.ours_branch_name))
                .label_size(LabelSize::Small)
                .on_click({
                    let editor = editor.clone();
                    let conflict = conflict.clone();
                    let ours = conflict.ours.clone();
                    move |_, window, cx| {
                        resolve_conflict(
                            editor.clone(),
                            conflict.clone(),
                            vec![ours.clone()],
                            window,
                            cx,
                        )
                        .detach()
                    }
                }),
        )
        .child(
            Button::new("origin", format!("Use {}", conflict.theirs_branch_name))
                .label_size(LabelSize::Small)
                .on_click({
                    let editor = editor.clone();
                    let conflict = conflict.clone();
                    let theirs = conflict.theirs.clone();
                    move |_, window, cx| {
                        resolve_conflict(
                            editor.clone(),
                            conflict.clone(),
                            vec![theirs.clone()],
                            window,
                            cx,
                        )
                        .detach()
                    }
                }),
        )
        .child(
            Button::new("both", "Use Both")
                .label_size(LabelSize::Small)
                .on_click({
                    let editor = editor.clone();
                    let conflict = conflict.clone();
                    let ours = conflict.ours.clone();
                    let theirs = conflict.theirs.clone();
                    move |_, window, cx| {
                        resolve_conflict(
                            editor.clone(),
                            conflict.clone(),
                            vec![ours.clone(), theirs.clone()],
                            window,
                            cx,
                        )
                        .detach()
                    }
                }),
        )
        .when(is_ai_enabled, |this| {
            this.child(Divider::vertical()).child(
                Button::new("resolve-with-agent", "Resolve with Agent")
                    .label_size(LabelSize::Small)
                    .start_icon(
                        Icon::new(IconName::ZedAssistant)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .on_click({
                        let conflict = conflict.clone();
                        move |_, window, cx| {
                            let content = editor
                                .update(cx, |editor, cx| {
                                    let multibuffer = editor.buffer().read(cx);
                                    let buffer_id = conflict.ours.end.buffer_id;
                                    let buffer = multibuffer.buffer(buffer_id)?;
                                    let buffer_read = buffer.read(cx);
                                    let snapshot = buffer_read.snapshot();
                                    let conflict_text = snapshot
                                        .text_for_range(conflict.range.clone())
                                        .collect::<String>();
                                    let file_path = buffer_read
                                        .file()
                                        .and_then(|file| file.as_local())
                                        .map(|f| f.abs_path(cx).to_string_lossy().to_string())
                                        .unwrap_or_default();
                                    Some(ConflictContent {
                                        file_path,
                                        conflict_text,
                                        ours_branch_name: conflict.ours_branch_name.to_string(),
                                        theirs_branch_name: conflict.theirs_branch_name.to_string(),
                                    })
                                })
                                .ok()
                                .flatten();
                            if let Some(content) = content {
                                window.dispatch_action(
                                    Box::new(ResolveConflictsWithAgent {
                                        conflicts: vec![content],
                                    }),
                                    cx,
                                );
                            }
                        }
                    }),
            )
        })
        .into_any()
}

fn collect_conflicted_project_paths(project: &Project, cx: &App) -> Vec<ProjectPath> {
    let git_store = project.git_store().read(cx);
    let mut paths = Vec::new();

    for repo in git_store.repositories().values() {
        let snapshot = repo.read(cx).snapshot();
        for (repo_path, _) in snapshot.merge.merge_heads_by_conflicted_path.iter() {
            let is_currently_conflicted = snapshot
                .status_for_path(repo_path)
                .is_some_and(|entry| entry.status.is_conflicted());
            if !is_currently_conflicted {
                continue;
            }
            if let Some(project_path) = repo.read(cx).repo_path_to_project_path(repo_path, cx) {
                paths.push(project_path);
            }
        }
    }

    paths.sort();
    paths.dedup();
    paths
}

fn collect_conflicted_file_paths(project: &Project, cx: &App) -> Vec<String> {
    collect_conflicted_project_paths(project, cx)
        .into_iter()
        .map(|p| p.path.as_std_path().to_string_lossy().to_string())
        .collect()
}

fn find_next_path(
    paths: &[ProjectPath],
    current: Option<&ProjectPath>,
    next: bool,
) -> Option<ProjectPath> {
    if paths.is_empty() {
        return None;
    }

    let Some(current) = current else {
        return if next {
            paths.first().cloned()
        } else {
            paths.last().cloned()
        };
    };

    if next {
        let index = paths.partition_point(|p| p <= current);
        paths.get(index).or_else(|| paths.first()).cloned()
    } else {
        let index = paths.partition_point(|p| p < current);
        if index > 0 {
            Some(paths[index - 1].clone())
        } else {
            paths.last().cloned()
        }
    }
}

fn find_conflict_in_snapshot(
    conflict_set: &ConflictSetSnapshot,
    snapshot: &text::BufferSnapshot,
    cursor_offset: usize,
    next: bool,
) -> Option<text::Point> {
    if conflict_set.conflicts.is_empty() {
        return None;
    }

    if next {
        conflict_set
            .conflicts
            .iter()
            .find(|c| c.range.start.to_offset(snapshot) > cursor_offset)
            .map(|c| c.range.start.to_point(snapshot))
    } else {
        conflict_set
            .conflicts
            .iter()
            .rev()
            .find(|c| c.range.start.to_offset(snapshot) < cursor_offset)
            .map(|c| c.range.start.to_point(snapshot))
    }
}

pub fn go_to_next_conflict(
    workspace: &mut Workspace,
    _action: &zed_actions::git::GoToNextConflict,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    go_to_conflict_impl(workspace, true, window, cx);
}

pub fn go_to_previous_conflict(
    workspace: &mut Workspace,
    _action: &zed_actions::git::GoToPreviousConflict,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    go_to_conflict_impl(workspace, false, window, cx);
}

fn jump_to_conflict_point(
    editor: &mut Editor,
    destination: text::Point,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    editor.unfold_ranges(&[destination..destination], false, false, cx);
    editor.change_selections(
        SelectionEffects::scroll(Autoscroll::center()),
        window,
        cx,
        |s| s.select_ranges([destination..destination]),
    );
}

fn go_to_conflict_impl(
    workspace: &mut Workspace,
    next: bool,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project().clone();
    let conflicted_paths = collect_conflicted_project_paths(project.read(cx), cx);
    if conflicted_paths.is_empty() {
        return;
    }

    let active_editor = workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx));

    let current_path = if let Some(ref editor_entity) = active_editor {
        let jumped = editor_entity.update(cx, |editor, cx| {
            let multibuffer = editor.buffer().read(cx);
            let buffer = multibuffer.as_singleton()?;
            let path = buffer.read(cx).project_path(cx)?;

            if !conflicted_paths.iter().any(|p| p == &path) {
                return Some((path, false));
            }

            let buffer_id = buffer.read(cx).remote_id();
            let text_snapshot = buffer.read(cx).text_snapshot();
            let conflict_set = editor
                .addon::<ConflictAddon>()
                .and_then(|addon| addon.conflict_set(buffer_id))
                .map(|cs| cs.read(cx).snapshot())
                .unwrap_or_else(|| ConflictSet::parse(&text_snapshot));
            let display_snapshot = editor.display_snapshot(cx);
            let cursor_point = editor
                .selections
                .newest::<text::Point>(&display_snapshot)
                .head();
            let cursor_offset = cursor_point.to_offset(&text_snapshot);

            let target = find_conflict_in_snapshot(
                &conflict_set,
                &text_snapshot,
                cursor_offset,
                next,
            );

            if let Some(destination) = target {
                jump_to_conflict_point(editor, destination, window, cx);
                return Some((path, true));
            }

            Some((path, false))
        });

        match jumped {
            Some((_path, true)) => return,
            Some((path, false)) => Some(path),
            None => None,
        }
    } else {
        None
    };

    let target_path = find_next_path(&conflicted_paths, current_path.as_ref(), next);
    let Some(target_path) = target_path else {
        return;
    };

    let open_task = workspace.open_path(target_path, None, true, window, cx);

    cx.spawn_in(window, async move |workspace, cx| {
        let item = open_task.await?;
        workspace.update_in(cx, |_workspace, window, cx| {
            let Some(editor_entity) = item.act_as::<Editor>(&*cx) else {
                return;
            };
            editor_entity.update(cx, |editor, cx| {
                let multibuffer = editor.buffer().read(cx);
                let Some(buffer) = multibuffer.as_singleton() else {
                    return;
                };
                let text_snapshot = buffer.read(cx).text_snapshot();
                let conflict_set = ConflictSet::parse(&text_snapshot);

                let conflict = if next {
                    conflict_set.conflicts.first()
                } else {
                    conflict_set.conflicts.last()
                };

                if let Some(conflict) = conflict {
                    let destination = conflict.range.start.to_point(&text_snapshot);
                    jump_to_conflict_point(editor, destination, window, cx);
                }
            });
        })?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

pub(crate) fn resolve_conflict(
    editor: WeakEntity<Editor>,
    resolved_conflict: ConflictRegion,
    ranges: Vec<Range<Anchor>>,
    window: &mut Window,
    cx: &mut App,
) -> Task<()> {
    window.spawn(cx, async move |cx| {
        let Some((workspace, project, multibuffer, buffer)) = editor
            .update(cx, |editor, cx| {
                let workspace = editor.workspace()?;
                let project = editor.project()?.clone();
                let multibuffer = editor.buffer().clone();
                let buffer_id = resolved_conflict.ours.end.buffer_id;
                let buffer = multibuffer.read(cx).buffer(buffer_id)?;
                resolved_conflict.resolve(buffer.clone(), &ranges, cx);
                let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
                let snapshot = multibuffer.read(cx).snapshot(cx);
                let buffer_snapshot = buffer.read(cx).snapshot();
                let state = conflict_addon
                    .buffers
                    .get_mut(&buffer_snapshot.remote_id())?;
                let ix = state
                    .block_ids
                    .binary_search_by(|(range, _)| {
                        range
                            .start
                            .cmp(&resolved_conflict.range.start, &buffer_snapshot)
                    })
                    .ok()?;
                let &(_, block_id) = &state.block_ids[ix];
                let range =
                    snapshot.buffer_anchor_range_to_anchor_range(resolved_conflict.range)?;

                editor.remove_gutter_highlights::<ConflictsOuter>(vec![range.clone()], cx);

                editor.remove_highlighted_rows::<ConflictsOuter>(vec![range.clone()], cx);
                editor.remove_highlighted_rows::<ConflictsOurs>(vec![range.clone()], cx);
                editor.remove_highlighted_rows::<ConflictsTheirs>(vec![range.clone()], cx);
                editor.remove_highlighted_rows::<ConflictsOursMarker>(vec![range.clone()], cx);
                editor.remove_highlighted_rows::<ConflictsTheirsMarker>(vec![range], cx);
                editor.remove_blocks(HashSet::from_iter([block_id]), None, cx);
                Some((workspace, project, multibuffer, buffer))
            })
            .ok()
            .flatten()
        else {
            return;
        };
        let save = project.update(cx, |project, cx| {
            if multibuffer.read(cx).all_diff_hunks_expanded() {
                project.save_buffer(buffer.clone(), cx)
            } else {
                Task::ready(Ok(()))
            }
        });
        if save.await.log_err().is_none() {
            let open_path = maybe!({
                let path = buffer.read_with(cx, |buffer, cx| buffer.project_path(cx))?;
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.open_path_preview(path, None, false, false, false, window, cx)
                    })
                    .ok()
            });

            if let Some(open_path) = open_path {
                open_path.await.log_err();
            }
        }
    })
}

pub struct MergeConflictIndicator {
    project: Entity<Project>,
    conflicted_paths: Vec<String>,
    last_shown_paths: HashSet<String>,
    dismissed: bool,
    _subscription: Subscription,
}

impl MergeConflictIndicator {
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();

        let subscription = cx.subscribe(&git_store, Self::on_git_store_event);

        let conflicted_paths = collect_conflicted_file_paths(project.read(cx), cx);
        let last_shown_paths: HashSet<String> = conflicted_paths.iter().cloned().collect();

        Self {
            project,
            conflicted_paths,
            last_shown_paths,
            dismissed: false,
            _subscription: subscription,
        }
    }

    fn on_git_store_event(
        &mut self,
        _git_store: Entity<GitStore>,
        event: &GitStoreEvent,
        cx: &mut Context<Self>,
    ) {
        let conflicts_changed = matches!(
            event,
            GitStoreEvent::ConflictsUpdated
                | GitStoreEvent::RepositoryUpdated(_, RepositoryEvent::StatusesChanged, _)
        );

        let agent_settings = AgentSettings::get_global(cx);
        if !agent_settings.enabled(cx)
            || !agent_settings.show_merge_conflict_indicator
            || !conflicts_changed
        {
            return;
        }

        let project = self.project.read(cx);
        if project.is_via_collab() {
            return;
        }

        let paths = collect_conflicted_file_paths(project, cx);
        let current_paths_set: HashSet<String> = paths.iter().cloned().collect();

        if paths.is_empty() {
            self.conflicted_paths.clear();
            self.last_shown_paths.clear();
            self.dismissed = false;
            cx.notify();
        } else if self.last_shown_paths != current_paths_set {
            self.last_shown_paths = current_paths_set;
            self.conflicted_paths = paths;
            self.dismissed = false;
            cx.notify();
        }
    }

    fn resolve_with_agent(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        window.dispatch_action(
            Box::new(ResolveConflictedFilesWithAgent {
                conflicted_file_paths: self.conflicted_paths.clone(),
            }),
            cx,
        );
        self.dismissed = true;
        cx.notify();
    }

    fn dismiss(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.dismissed = true;
        cx.notify();
    }
}

impl Render for MergeConflictIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let agent_settings = AgentSettings::get_global(cx);
        if !agent_settings.enabled(cx)
            || !agent_settings.show_merge_conflict_indicator
            || self.conflicted_paths.is_empty()
            || self.dismissed
        {
            return Empty.into_any_element();
        }

        let file_count = self.conflicted_paths.len();

        let message: SharedString = format!(
            "Resolve Merge Conflict{} with Agent",
            if file_count == 1 { "" } else { "s" }
        )
        .into();

        let tooltip_label: SharedString = format!(
            "Found {} {} across the codebase",
            file_count,
            if file_count == 1 {
                "conflict"
            } else {
                "conflicts"
            }
        )
        .into();

        let border_color = cx.theme().colors().text_accent.opacity(0.2);

        h_flex()
            .h(rems_from_px(22.))
            .rounded_sm()
            .border_1()
            .border_color(border_color)
            .child(
                ButtonLike::new("update-button")
                    .child(
                        h_flex()
                            .h_full()
                            .gap_1()
                            .child(
                                Icon::new(IconName::GitMergeConflict)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Label::new(message).size(LabelSize::Small)),
                    )
                    .tooltip(move |_, cx| {
                        Tooltip::with_meta(
                            tooltip_label.clone(),
                            None,
                            "Click to Resolve with Agent",
                            cx,
                        )
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.resolve_with_agent(window, cx);
                    })),
            )
            .child(
                div().border_l_1().border_color(border_color).child(
                    IconButton::new("dismiss-merge-conflicts", IconName::Close)
                        .icon_size(IconSize::XSmall)
                        .on_click(cx.listener(Self::dismiss)),
                ),
            )
            .into_any_element()
    }
}

impl StatusItemView for MergeConflictIndicator {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git::status::{UnmergedStatus, UnmergedStatusCode};
    use gpui::{TestAppContext, VisualTestContext};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use text::{Buffer, BufferId, ReplicaId};
    use unindent::Unindent as _;
    use util::{path, rel_path::rel_path};
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            language::language_settings::AllLanguageSettings::register(cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    fn make_project_path(worktree_id: usize, path: &str) -> ProjectPath {
        ProjectPath {
            worktree_id: settings::WorktreeId::from_usize(worktree_id),
            path: Arc::from(rel_path(path)),
        }
    }

    // ── find_next_path tests ──────────────────────────────────────────

    #[test]
    fn test_find_next_path_empty() {
        let paths: Vec<ProjectPath> = vec![];
        assert!(find_next_path(&paths, None, true).is_none());
        assert!(find_next_path(&paths, None, false).is_none());
    }

    #[test]
    fn test_find_next_path_no_current() {
        let paths = vec![
            make_project_path(0, "a.rs"),
            make_project_path(0, "b.rs"),
            make_project_path(0, "c.rs"),
        ];

        assert_eq!(find_next_path(&paths, None, true), Some(paths[0].clone()));
        assert_eq!(find_next_path(&paths, None, false), Some(paths[2].clone()));
    }

    #[test]
    fn test_find_next_path_advances() {
        let paths = vec![
            make_project_path(0, "a.rs"),
            make_project_path(0, "b.rs"),
            make_project_path(0, "c.rs"),
        ];

        assert_eq!(
            find_next_path(&paths, Some(&paths[0]), true),
            Some(paths[1].clone())
        );
        assert_eq!(
            find_next_path(&paths, Some(&paths[1]), true),
            Some(paths[2].clone())
        );
        assert_eq!(
            find_next_path(&paths, Some(&paths[2]), false),
            Some(paths[1].clone())
        );
        assert_eq!(
            find_next_path(&paths, Some(&paths[1]), false),
            Some(paths[0].clone())
        );
    }

    #[test]
    fn test_find_next_path_wraps() {
        let paths = vec![
            make_project_path(0, "a.rs"),
            make_project_path(0, "b.rs"),
        ];

        assert_eq!(
            find_next_path(&paths, Some(&paths[1]), true),
            Some(paths[0].clone()),
        );
        assert_eq!(
            find_next_path(&paths, Some(&paths[0]), false),
            Some(paths[1].clone()),
        );
    }

    #[test]
    fn test_find_next_path_single() {
        let paths = vec![make_project_path(0, "a.rs")];

        assert_eq!(
            find_next_path(&paths, Some(&paths[0]), true),
            Some(paths[0].clone()),
        );
        assert_eq!(
            find_next_path(&paths, Some(&paths[0]), false),
            Some(paths[0].clone()),
        );
    }

    #[test]
    fn test_find_next_path_current_not_in_list() {
        let paths = vec![
            make_project_path(0, "a.rs"),
            make_project_path(0, "c.rs"),
        ];
        let current = make_project_path(0, "b.rs");

        assert_eq!(
            find_next_path(&paths, Some(&current), true),
            Some(paths[1].clone()),
        );
        assert_eq!(
            find_next_path(&paths, Some(&current), false),
            Some(paths[0].clone()),
        );
    }

    // ── find_conflict_in_snapshot tests ───────────────────────────────

    fn make_conflicted_buffer(content: &str) -> (text::Buffer, ConflictSetSnapshot) {
        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(ReplicaId::LOCAL, buffer_id, content.to_string());
        let snapshot = buffer.snapshot();
        let conflict_set = ConflictSet::parse(&snapshot);
        (buffer, conflict_set)
    }

    #[test]
    fn test_find_conflict_in_snapshot_empty() {
        let (buffer, conflict_set) = make_conflicted_buffer("no conflicts here\n");
        let snapshot = buffer.snapshot();
        assert!(find_conflict_in_snapshot(&conflict_set, &snapshot, 0, true).is_none());
        assert!(find_conflict_in_snapshot(&conflict_set, &snapshot, 0, false).is_none());
    }

    #[test]
    fn test_find_conflict_in_snapshot_next() {
        let content = r#"
            line 1
            <<<<<<< HEAD
            ours
            =======
            theirs
            >>>>>>> branch
            middle
            <<<<<<< HEAD
            ours2
            =======
            theirs2
            >>>>>>> branch
        "#
        .unindent();

        let (buffer, conflict_set) = make_conflicted_buffer(&content);
        let snapshot = buffer.snapshot();
        assert_eq!(conflict_set.conflicts.len(), 2);

        let first_start = conflict_set.conflicts[0].range.start.to_offset(&snapshot);
        let second_start = conflict_set.conflicts[1].range.start.to_offset(&snapshot);

        let result = find_conflict_in_snapshot(&conflict_set, &snapshot, 0, true);
        assert_eq!(
            result,
            Some(conflict_set.conflicts[0].range.start.to_point(&snapshot))
        );

        let result = find_conflict_in_snapshot(&conflict_set, &snapshot, first_start, true);
        assert_eq!(
            result,
            Some(conflict_set.conflicts[1].range.start.to_point(&snapshot))
        );

        let result = find_conflict_in_snapshot(&conflict_set, &snapshot, second_start, true);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_conflict_in_snapshot_previous() {
        let content = r#"
            line 1
            <<<<<<< HEAD
            ours
            =======
            theirs
            >>>>>>> branch
            middle
            <<<<<<< HEAD
            ours2
            =======
            theirs2
            >>>>>>> branch
        "#
        .unindent();

        let (buffer, conflict_set) = make_conflicted_buffer(&content);
        let snapshot = buffer.snapshot();
        let second_start = conflict_set.conflicts[1].range.start.to_offset(&snapshot);
        let second_end = conflict_set.conflicts[1].range.end.to_offset(&snapshot);

        let result = find_conflict_in_snapshot(&conflict_set, &snapshot, second_end, false);
        assert_eq!(
            result,
            Some(conflict_set.conflicts[1].range.start.to_point(&snapshot))
        );

        let result = find_conflict_in_snapshot(&conflict_set, &snapshot, second_start, false);
        assert_eq!(
            result,
            Some(conflict_set.conflicts[0].range.start.to_point(&snapshot))
        );

        let result = find_conflict_in_snapshot(&conflict_set, &snapshot, 0, false);
        assert!(result.is_none());
    }

    // ── Integration tests for go_to_next/previous_conflict ───────────

    fn unmerged() -> git::status::FileStatus {
        UnmergedStatus {
            first_head: UnmergedStatusCode::Updated,
            second_head: UnmergedStatusCode::Updated,
        }
        .into()
    }

    #[gpui::test]
    async fn test_go_to_next_conflict_within_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.rs": "line 1\n<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\nmiddle\n<<<<<<< HEAD\nours2\n=======\ntheirs2\n>>>>>>> branch\n",
            }),
        )
        .await;
        fs.set_status_for_repo(
            Path::new(path!("/project/.git")),
            &[("file.rs", unmerged())],
        );

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window.read_with(cx, |mw, _| mw.workspace().clone()).unwrap();
        let cx = &mut VisualTestContext::from_window(*window, cx);

        let worktree_id = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .project()
                .read(cx)
                .worktrees(cx)
                .next()
                .unwrap()
                .read(cx)
                .id()
                .to_usize()
        });

        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    make_project_path(worktree_id, "file.rs"),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            go_to_conflict_impl(workspace, true, window, cx);
        });
        cx.run_until_parked();

        editor.update_in(cx, |editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            let cursor = editor.selections.newest::<text::Point>(&snapshot).head();
            assert_eq!(cursor, text::Point::new(1, 0));
        });

        workspace.update_in(cx, |workspace, window, cx| {
            go_to_conflict_impl(workspace, true, window, cx);
        });
        cx.run_until_parked();

        editor.update_in(cx, |editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            let cursor = editor.selections.newest::<text::Point>(&snapshot).head();
            assert_eq!(cursor, text::Point::new(7, 0));
        });
    }

    #[gpui::test]
    async fn test_go_to_previous_conflict_within_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.rs": "line 1\n<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\nmiddle\n<<<<<<< HEAD\nours2\n=======\ntheirs2\n>>>>>>> branch\n",
            }),
        )
        .await;
        fs.set_status_for_repo(
            Path::new(path!("/project/.git")),
            &[("file.rs", unmerged())],
        );

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window.read_with(cx, |mw, _| mw.workspace().clone()).unwrap();
        let cx = &mut VisualTestContext::from_window(*window, cx);

        let worktree_id = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .project()
                .read(cx)
                .worktrees(cx)
                .next()
                .unwrap()
                .read(cx)
                .id()
                .to_usize()
        });

        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    make_project_path(worktree_id, "file.rs"),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        cx.run_until_parked();

        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(None.into(), window, cx, |s| {
                s.select_ranges([text::Point::new(12, 0)..text::Point::new(12, 0)]);
            });
        });

        workspace.update_in(cx, |workspace, window, cx| {
            go_to_conflict_impl(workspace, false, window, cx);
        });
        cx.run_until_parked();

        editor.update_in(cx, |editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            let cursor = editor.selections.newest::<text::Point>(&snapshot).head();
            assert_eq!(cursor, text::Point::new(7, 0));
        });

        workspace.update_in(cx, |workspace, window, cx| {
            go_to_conflict_impl(workspace, false, window, cx);
        });
        cx.run_until_parked();

        editor.update_in(cx, |editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            let cursor = editor.selections.newest::<text::Point>(&snapshot).head();
            assert_eq!(cursor, text::Point::new(1, 0));
        });
    }

    #[gpui::test]
    async fn test_go_to_conflict_crosses_files(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "aaa.rs": "<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\n",
                "bbb.rs": "<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\n",
            }),
        )
        .await;
        fs.set_status_for_repo(
            Path::new(path!("/project/.git")),
            &[("aaa.rs", unmerged()), ("bbb.rs", unmerged())],
        );

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window.read_with(cx, |mw, _| mw.workspace().clone()).unwrap();
        let cx = &mut VisualTestContext::from_window(*window, cx);

        let worktree_id = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .project()
                .read(cx)
                .worktrees(cx)
                .next()
                .unwrap()
                .read(cx)
                .id()
                .to_usize()
        });

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    make_project_path(worktree_id, "aaa.rs"),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        cx.run_until_parked();

        // Cursor starts at 0,0 which is the start of the only conflict in aaa.rs.
        // "Next conflict" finds no conflict ahead (0 > 0 is false) and jumps
        // to the next conflicted file (bbb.rs) asynchronously.
        workspace.update_in(cx, |workspace, window, cx| {
            go_to_conflict_impl(workspace, true, window, cx);
        });
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, _window, cx| {
            let item = workspace.active_item(cx).unwrap();
            let editor = item.act_as::<Editor>(cx).unwrap();
            let active_path = editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap()
                .read(cx)
                .project_path(cx);
            assert_eq!(
                active_path,
                Some(make_project_path(worktree_id, "bbb.rs"))
            );
        });
    }

    #[gpui::test]
    async fn test_go_to_conflict_no_conflicts(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.rs": "fn main() {}\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window.read_with(cx, |mw, _| mw.workspace().clone()).unwrap();
        let cx = &mut VisualTestContext::from_window(*window, cx);

        let worktree_id = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .project()
                .read(cx)
                .worktrees(cx)
                .next()
                .unwrap()
                .read(cx)
                .id()
                .to_usize()
        });

        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    make_project_path(worktree_id, "file.rs"),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            go_to_conflict_impl(workspace, true, window, cx);
        });
        cx.run_until_parked();

        editor.update_in(cx, |editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            let cursor = editor.selections.newest::<text::Point>(&snapshot).head();
            assert_eq!(cursor, text::Point::new(0, 0));
        });
    }
}
