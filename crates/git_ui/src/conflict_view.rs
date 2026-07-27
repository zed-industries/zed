use agent_settings::AgentSettings;
use collections::{HashMap, HashSet};
use editor::{
    ConflictsOurs, ConflictsOursMarker, ConflictsOuter, ConflictsTheirs, ConflictsTheirsMarker,
    Direction, Editor, MultiBuffer, RowHighlightOptions, SelectionEffects, ToPoint as _,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
    scroll::Autoscroll,
};
use gpui::{
    Action, AnyView, App, ClickEvent, Context, Empty, Entity, FocusHandle, Focusable as _,
    InteractiveElement as _, KeyContext, ParentElement as _, Subscription, Task, WeakEntity,
};
use language::{Anchor, Buffer, BufferId, Point};
use project::{
    ConflictRegion, ConflictSet, ConflictSetUpdate, Project, ProjectPath,
    git_store::{GitStore, GitStoreEvent, RepositoryEvent},
};
use settings::Settings;
use std::{ops::Range, sync::Arc};
use ui::{ButtonLike, Divider, Tooltip, prelude::*};
use util::debug_panic;
use workspace::{HideStatusItem, StatusItemView, Workspace, item::ItemHandle};
use zed_actions::agent::{
    ConflictContent, ResolveConflictedFilesWithAgent, ResolveConflictsWithAgent,
};

pub(crate) struct ConflictAddon {
    buffers: HashMap<BufferId, BufferConflicts>,
    _action_subscriptions: Vec<Subscription>,
}

impl ConflictAddon {
    fn conflicts_for_buffer(&self, buffer_id: BufferId, cx: &App) -> Arc<[ConflictRegion]> {
        self.buffers
            .get(&buffer_id)
            .map(|buffer_conflicts| {
                buffer_conflicts
                    .conflict_set
                    .read(cx)
                    .snapshot
                    .conflicts
                    .clone()
            })
            .unwrap_or_default()
    }

    fn all_conflicts<'a>(&'a self, cx: &'a App) -> impl Iterator<Item = &'a ConflictRegion> {
        self.buffers.values().flat_map(|buffer_conflicts| {
            buffer_conflicts
                .conflict_set
                .read(cx)
                .snapshot
                .conflicts
                .iter()
        })
    }
}

struct BufferConflicts {
    block_ids: Vec<(Range<Anchor>, CustomBlockId)>,
    conflict_set: Entity<ConflictSet>,
    _subscription: Subscription,
}

impl editor::Addon for ConflictAddon {
    fn extend_key_context(&self, key_context: &mut KeyContext, cx: &App) {
        if self.all_conflicts(cx).next().is_some() {
            key_context.add("has_conflicts");
        }
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

pub fn register_editor(editor: &mut Editor, buffer: Entity<MultiBuffer>, cx: &mut Context<Editor>) {
    let is_singleton = editor.buffer().read(cx).is_singleton();
    if !editor.mode().is_full()
        || (!is_singleton && !editor.buffer().read(cx).all_diff_hunks_expanded())
        || editor.read_only(cx)
    {
        return;
    }

    let action_subscriptions = vec![
        register_conflict_action::<git::GoToNextConflict>(editor, cx, |editor, window, cx| {
            go_to_conflict(editor, Direction::Next, window, cx)
        }),
        register_conflict_action::<git::GoToPreviousConflict>(editor, cx, |editor, window, cx| {
            go_to_conflict(editor, Direction::Prev, window, cx)
        }),
        register_conflict_action::<git::AcceptCurrentChange>(editor, cx, |editor, window, cx| {
            accept_conflict_at_cursor(editor, ConflictSide::Ours, window, cx)
        }),
        register_conflict_action::<git::AcceptIncomingChange>(editor, cx, |editor, window, cx| {
            accept_conflict_at_cursor(editor, ConflictSide::Theirs, window, cx)
        }),
        register_conflict_action::<git::AcceptBothChanges>(editor, cx, |editor, window, cx| {
            accept_conflict_at_cursor(editor, ConflictSide::Both, window, cx)
        }),
    ];

    editor.register_addon(ConflictAddon {
        buffers: Default::default(),
        _action_subscriptions: action_subscriptions,
    });

    if is_singleton {
        let buffers = buffer.read(cx).all_buffers();
        for buffer in buffers {
            open_conflict_set_for_buffer(editor, buffer, cx);
        }
    }
}

fn open_conflict_set_for_buffer(
    _editor: &mut Editor,
    buffer: Entity<Buffer>,
    cx: &mut Context<Editor>,
) {
    let buffer = buffer.downgrade();

    cx.spawn(async move |editor, cx| {
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id())?;
        if let Some(conflict_set) = editor.read_with(cx, |editor, _| {
            editor
                .addon::<ConflictAddon>()
                .and_then(|addon| addon.buffers.get(&buffer_id))
                .map(|buffer_conflicts| buffer_conflicts.conflict_set.clone())
        })? {
            editor.update(cx, |editor, cx| {
                buffer_ranges_updated(editor, conflict_set, cx);
            })?;
            return anyhow::Ok(());
        }

        let Some(project) = editor.read_with(cx, |editor, _| editor.project().cloned())? else {
            return anyhow::Ok(());
        };
        let git_store = project.read_with(cx, |project, _| project.git_store().clone());
        let Some(buffer) = buffer.upgrade() else {
            return Ok(());
        };
        let conflict_set = git_store
            .update(cx, |git_store, cx| {
                git_store.open_conflict_set(buffer.clone(), cx)
            })
            .await;
        editor.update(cx, |editor, cx| {
            buffer_ranges_updated(editor, conflict_set, cx);
        })?;
        Ok(())
    })
    .detach();
}

pub(crate) fn buffer_ranges_updated(
    editor: &mut Editor,
    conflict_set: Entity<ConflictSet>,
    cx: &mut Context<Editor>,
) {
    let buffer_id = conflict_set.read(cx).snapshot.buffer_id;
    if editor.buffer().read(cx).buffer(buffer_id).is_none() {
        return;
    }

    let Some(conflict_addon) = editor.addon_mut::<ConflictAddon>() else {
        return;
    };
    let buffer_conflicts = conflict_addon.buffers.entry(buffer_id).or_insert_with(|| {
        let subscription = cx.subscribe(&conflict_set, conflicts_updated);
        BufferConflicts {
            block_ids: Vec::new(),
            conflict_set: conflict_set.clone(),
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

pub(crate) fn buffers_removed(
    editor: &mut Editor,
    removed_buffer_ids: &[BufferId],
    cx: &mut Context<Editor>,
) {
    let mut removed_block_ids = HashSet::default();
    let Some(conflict_addon) = editor.addon_mut::<ConflictAddon>() else {
        return;
    };
    conflict_addon.buffers.retain(|buffer_id, buffer| {
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
    let old_range = {
        let Some(conflict_addon) = editor.addon_mut::<ConflictAddon>() else {
            return;
        };
        let Some(buffer_conflicts) = conflict_addon.buffers.get(&buffer_id) else {
            return;
        };
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
    };

    // Remove obsolete highlights and blocks
    let Some(conflict_addon) = editor.addon_mut::<ConflictAddon>() else {
        return;
    };
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
    editor.refresh_scrollbar_markers(cx);

    let Some(conflict_addon) = editor.addon_mut::<ConflictAddon>() else {
        return;
    };
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

    let ours_background = |cx: &App| cx.theme().colors().version_control_conflict_marker_ours;
    let theirs_background = |cx: &App| cx.theme().colors().version_control_conflict_marker_theirs;

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

/// Position of `conflict` among the conflicts still unresolved in its buffer,
/// as a 1-based index and a total. Recomputed on every render because resolving
/// one conflict renumbers the ones after it without recreating their blocks.
fn conflict_position(
    conflict: &ConflictRegion,
    editor: &WeakEntity<Editor>,
    cx: &App,
) -> Option<(usize, usize)> {
    let buffer_id = conflict.ours.end.buffer_id;
    let editor = editor.upgrade()?;
    let conflicts = editor
        .read(cx)
        .addon::<ConflictAddon>()?
        .conflicts_for_buffer(buffer_id, cx);
    let index = conflicts
        .iter()
        .position(|other| other.range.start == conflict.range.start)?;
    Some((index + 1, conflicts.len()))
}

fn conflict_tooltip(
    label: &'static str,
    action: Box<dyn Action>,
    focus_handle: Option<FocusHandle>,
) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
    move |_, cx| match focus_handle.as_ref() {
        Some(focus_handle) => Tooltip::for_action_in(label, action.as_ref(), focus_handle, cx),
        None => Tooltip::for_action(label, action.as_ref(), cx),
    }
}

fn render_conflict_buttons(
    conflict: &ConflictRegion,
    editor: WeakEntity<Editor>,
    cx: &mut BlockContext,
) -> AnyElement {
    let is_ai_enabled = AgentSettings::get_global(cx).enabled(cx);
    let position = conflict_position(conflict, &editor, cx);
    let focus_handle = editor.upgrade().map(|editor| editor.focus_handle(cx));

    h_flex()
        .id(cx.block_id)
        .h(cx.line_height)
        .ml(cx.margins.gutter.width)
        .gap_1()
        .bg(cx.theme().colors().editor_background)
        .when_some(position, |this, (index, total)| {
            this.child(
                Label::new(format!("Conflict {index} of {total}"))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(Divider::vertical())
        })
        .child(
            Button::new("head", format!("Use {}", conflict.ours_branch_name))
                .label_size(LabelSize::Small)
                .tooltip(conflict_tooltip(
                    "Keep the changes from the current branch",
                    git::AcceptCurrentChange.boxed_clone(),
                    focus_handle.clone(),
                ))
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
                .tooltip(conflict_tooltip(
                    "Keep the changes from the incoming branch",
                    git::AcceptIncomingChange.boxed_clone(),
                    focus_handle.clone(),
                ))
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
                .tooltip(conflict_tooltip(
                    "Keep both sides of the conflict",
                    git::AcceptBothChanges.boxed_clone(),
                    focus_handle,
                ))
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

    // Repositories are stored in a hash map, so the paths have to be sorted for
    // navigation between them to be stable across calls.
    paths.sort();
    paths
}

fn collect_conflicted_file_paths(project: &Project, cx: &App) -> Vec<String> {
    collect_conflicted_project_paths(project, cx)
        .into_iter()
        .map(|project_path| {
            project_path
                .path
                .as_std_path()
                .to_string_lossy()
                .to_string()
        })
        .collect()
}

pub(crate) fn go_to_conflicted_file(
    workspace: &mut Workspace,
    direction: Direction,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project().clone();
    let paths = collect_conflicted_project_paths(project.read(cx), cx);
    let Some(last_index) = paths.len().checked_sub(1) else {
        return;
    };

    let current = workspace
        .active_item(cx)
        .and_then(|item| item.project_path(cx));
    let current_index = current
        .as_ref()
        .and_then(|current| paths.iter().position(|path| path == current));
    let destination = match (current_index, direction) {
        (Some(index), Direction::Next) => &paths[(index + 1) % paths.len()],
        (Some(index), Direction::Prev) => &paths[(index + last_index) % paths.len()],
        (None, Direction::Next) => &paths[0],
        (None, Direction::Prev) => &paths[last_index],
    };

    let open = workspace.open_path(destination.clone(), None, true, window, cx);
    cx.spawn_in(window, async move |_, cx| {
        let Some(editor) = open.await?.downcast::<Editor>() else {
            return anyhow::Ok(());
        };
        // Conflicts are parsed asynchronously once the buffer opens, so there is
        // nothing to select until the conflict set has been loaded.
        let buffer = editor.read_with(cx, |editor, cx| editor.buffer().read(cx).as_singleton());
        if let Some(buffer) = buffer {
            let git_store = project.read_with(cx, |project, _| project.git_store().clone());
            git_store
                .update(cx, |git_store, cx| git_store.open_conflict_set(buffer, cx))
                .await;
        }
        editor.update_in(cx, select_first_conflict)?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

#[derive(Clone, Copy)]
enum ConflictSide {
    Ours,
    Theirs,
    Both,
}

fn register_conflict_action<A: Action>(
    editor: &mut Editor,
    cx: &mut Context<Editor>,
    handler: fn(&mut Editor, &mut Window, &mut Context<Editor>),
) -> Subscription {
    let editor_handle = cx.weak_entity();
    editor.register_action::<A>(move |_, window, cx| {
        editor_handle
            .update(cx, |editor, cx| handler(editor, window, cx))
            .ok();
    })
}

/// Returns the start of every conflict currently known to the editor, in
/// multibuffer coordinates. Conflicts from different excerpts are interleaved,
/// so callers that need document order have to compare the returned points.
fn conflict_starts(editor: &Editor, cx: &App) -> Vec<Point> {
    let Some(addon) = editor.addon::<ConflictAddon>() else {
        return Vec::new();
    };
    let snapshot = editor.buffer().read(cx).snapshot(cx);
    addon
        .all_conflicts(cx)
        .filter_map(|conflict| {
            let range = snapshot.buffer_anchor_range_to_anchor_range(conflict.range.clone())?;
            Some(range.start.to_point(&snapshot))
        })
        .collect()
}

fn go_to_conflict(
    editor: &mut Editor,
    direction: Direction,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let cursor = editor
        .selections
        .newest::<Point>(&editor.display_snapshot(cx))
        .head();
    let starts = conflict_starts(editor, cx);

    let after = starts.iter().filter(|start| **start > cursor).min();
    let before = starts.iter().filter(|start| **start < cursor).max();
    let destination = match direction {
        Direction::Next => after.or_else(|| starts.iter().min()),
        Direction::Prev => before.or_else(|| starts.iter().max()),
    };
    let Some(destination) = destination.copied() else {
        return;
    };
    select_conflict_at(editor, destination, window, cx);
}

pub(crate) fn select_first_conflict(
    editor: &mut Editor,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    if let Some(destination) = conflict_starts(editor, cx).into_iter().min() {
        select_conflict_at(editor, destination, window, cx);
    }
}

fn select_conflict_at(
    editor: &mut Editor,
    destination: Point,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    editor.unfold_ranges(&[destination..destination], false, false, cx);
    editor.change_selections(
        SelectionEffects::scroll(Autoscroll::center()),
        window,
        cx,
        |selections| selections.select_ranges([destination..destination]),
    );
}

fn accept_conflict_at_cursor(
    editor: &mut Editor,
    side: ConflictSide,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let cursor = editor
        .selections
        .newest::<Point>(&editor.display_snapshot(cx))
        .head();
    let conflict = {
        let Some(addon) = editor.addon::<ConflictAddon>() else {
            return;
        };
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        addon
            .all_conflicts(cx)
            .find(|conflict| {
                snapshot
                    .buffer_anchor_range_to_anchor_range(conflict.range.clone())
                    .is_some_and(|range| {
                        (range.start.to_point(&snapshot)..range.end.to_point(&snapshot))
                            .contains(&cursor)
                    })
            })
            .cloned()
    };
    let Some(conflict) = conflict else {
        return;
    };

    let ranges = match side {
        ConflictSide::Ours => vec![conflict.ours.clone()],
        ConflictSide::Theirs => vec![conflict.theirs.clone()],
        ConflictSide::Both => vec![conflict.ours.clone(), conflict.theirs.clone()],
    };
    resolve_conflict(cx.weak_entity(), conflict, ranges, window, cx).detach();
}

pub(crate) fn resolve_conflict(
    editor: WeakEntity<Editor>,
    resolved_conflict: ConflictRegion,
    ranges: Vec<Range<Anchor>>,
    window: &mut Window,
    cx: &mut App,
) -> Task<()> {
    window.spawn(cx, async move |cx| {
        editor
            .update(cx, |editor, cx| {
                let multibuffer = editor.buffer().clone();
                let buffer_id = resolved_conflict.ours.end.buffer_id;
                let buffer = multibuffer.read(cx).buffer(buffer_id)?;
                resolved_conflict.resolve(buffer.clone(), &ranges, cx);
                let conflict_addon = editor.addon_mut::<ConflictAddon>()?;
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
                editor.refresh_scrollbar_markers(cx);
                Some(())
            })
            .ok();
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
                    .tab_index(0isize)
                    .aria_label(message.clone())
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

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .agent
                .get_or_insert_default()
                .show_merge_conflict_indicator = Some(false);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git::{
        repository::repo_path,
        status::{UnmergedStatus, UnmergedStatusCode},
    };
    use gpui::{TestAppContext, VisualTestContext};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use unindent::Unindent as _;
    use util::path;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        zlog::init_test();
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    fn cursor_row(editor: &Editor, cx: &mut App) -> u32 {
        editor
            .selections
            .newest::<Point>(&editor.display_snapshot(cx))
            .head()
            .row
    }

    fn mark_unmerged(fs: &FakeFs, dot_git: &str, paths: &[&str]) {
        fs.with_git_state(dot_git.as_ref(), true, |state| {
            for path in paths {
                state.unmerged_paths.insert(
                    repo_path(path),
                    UnmergedStatus {
                        first_head: UnmergedStatusCode::Updated,
                        second_head: UnmergedStatusCode::Updated,
                    },
                );
            }
            state.refs.insert("MERGE_HEAD".into(), "123".into());
        })
        .unwrap();
    }

    #[gpui::test]
    async fn test_navigation_in_a_heavily_conflicted_file(cx: &mut TestAppContext) {
        init_test(cx);

        // Every conflict gets a block and five row highlights, and navigation
        // scans them all, so a file with this many is the shape worth checking.
        const CONFLICTS: u32 = 1000;
        const LINES_PER_CONFLICT: u32 = 6;

        let mut text = String::new();
        for index in 0..CONFLICTS {
            text.push_str(&format!(
                "line-{index}\n<<<<<<< HEAD\nours-{index}\n=======\ntheirs-{index}\n>>>>>>> feature\n"
            ));
        }

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "a.txt": text,
            }),
        )
        .await;
        mark_unmerged(&fs, path!("/project/.git"), &["a.txt"]);

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/a.txt"), cx)
            })
            .await
            .unwrap();

        let (editor, cx) = cx.add_window_view(|window, cx| {
            Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx)
        });
        cx.run_until_parked();

        let conflict_count = |editor: &Editor, cx: &App| {
            editor
                .addon::<ConflictAddon>()
                .expect("conflict addon should be registered")
                .all_conflicts(cx)
                .count()
        };

        editor.update(cx, |editor, cx| {
            assert_eq!(conflict_count(editor, cx), CONFLICTS as usize);
        });

        editor.update_in(cx, |editor, window, cx| {
            go_to_conflict(editor, Direction::Next, window, cx);
            assert_eq!(cursor_row(editor, cx), 1);

            go_to_conflict(editor, Direction::Prev, window, cx);
            assert_eq!(
                cursor_row(editor, cx),
                (CONFLICTS - 1) * LINES_PER_CONFLICT + 1,
                "wrapping backwards reaches the last conflict"
            );

            accept_conflict_at_cursor(editor, ConflictSide::Ours, window, cx);
        });
        cx.run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(conflict_count(editor, cx), CONFLICTS as usize - 1);
        });
    }

    /// The project diff builds its own multibuffer and wires the conflict set up
    /// from `diff_multibuffer`, bypassing the singleton path in `register_editor`.
    #[gpui::test]
    async fn test_conflict_resolution_in_project_diff(cx: &mut TestAppContext) {
        init_test(cx);

        // Two conflicts, so that each one becomes its own excerpt and the
        // buffer-anchor-to-multibuffer-anchor conversion has to pick the right one.
        let text = "
            before
            <<<<<<< HEAD
            ours-1
            =======
            theirs-1
            >>>>>>> feature
            middle
            <<<<<<< HEAD
            ours-2
            =======
            theirs-2
            >>>>>>> feature
            after
        "
        .unindent();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "a.txt": text,
            }),
        )
        .await;
        mark_unmerged(&fs, path!("/project/.git"), &["a.txt"]);

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) = cx
            .add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |multi_workspace, _| {
            multi_workspace.workspace().clone()
        });
        cx.run_until_parked();

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(crate::project_diff::Diff.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let project_diff = workspace.update(cx, |workspace, cx| {
            workspace
                .active_item_as::<crate::project_diff::ProjectDiff>(cx)
                .expect("the project diff is the active item")
        });
        cx.focus(&project_diff);
        let editor = project_diff
            .read_with(cx, |project_diff, cx| {
                project_diff.editor(cx).read(cx).rhs_editor().clone()
            });
        cx.run_until_parked();

        editor.update(cx, |editor, cx| {
            let conflicts = editor
                .addon::<ConflictAddon>()
                .expect("the conflict addon is registered on the project diff editor")
                .all_conflicts(cx)
                .count();
            assert_eq!(conflicts, 2);
        });

        // The second conflict is the one that would break if every conflict in a
        // buffer mapped back to that buffer's first excerpt.
        editor.update_in(cx, |editor, window, cx| {
            go_to_conflict(editor, Direction::Prev, window, cx);
            accept_conflict_at_cursor(editor, ConflictSide::Theirs, window, cx);
        });
        cx.run_until_parked();

        editor.update_in(cx, |editor, window, cx| {
            go_to_conflict(editor, Direction::Next, window, cx);
            accept_conflict_at_cursor(editor, ConflictSide::Ours, window, cx);
        });
        cx.run_until_parked();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/a.txt"), cx)
            })
            .await
            .unwrap();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "before\nours-1\nmiddle\ntheirs-2\nafter\n",
            "each conflict resolves through its own excerpt's anchors"
        );
    }

    #[gpui::test]
    async fn test_go_to_conflicted_file(cx: &mut TestAppContext) {
        init_test(cx);

        let text = "
            before
            <<<<<<< HEAD
            ours
            =======
            theirs
            >>>>>>> feature
        "
        .unindent();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "a.txt": text,
                "b.txt": text,
                "c.txt": "no conflicts here\n",
            }),
        )
        .await;
        mark_unmerged(&fs, path!("/project/.git"), &["a.txt", "b.txt"]);

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) = cx
            .add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |multi_workspace, _| {
            multi_workspace.workspace().clone()
        });
        cx.run_until_parked();

        let go_to = |direction, cx: &mut VisualTestContext| {
            workspace.update_in(cx, |workspace, window, cx| {
                go_to_conflicted_file(workspace, direction, window, cx);
            });
            cx.run_until_parked();
            workspace.update(cx, |workspace, cx| {
                let item = workspace.active_item(cx).expect("an item is active");
                let path = item.project_path(cx).expect("the active item has a path");
                let editor = item
                    .downcast::<Editor>()
                    .expect("a conflicted file opens in an editor");
                let row = editor.update(cx, |editor, cx| cursor_row(editor, cx));
                (path.path.as_std_path().to_string_lossy().to_string(), row)
            })
        };

        assert_eq!(
            go_to(Direction::Next, cx),
            ("a.txt".to_string(), 1),
            "opens the first conflicted file with the cursor on its conflict"
        );
        assert_eq!(go_to(Direction::Next, cx), ("b.txt".to_string(), 1));
        assert_eq!(
            go_to(Direction::Next, cx),
            ("a.txt".to_string(), 1),
            "wraps around to the first conflicted file"
        );
        assert_eq!(
            go_to(Direction::Prev, cx),
            ("b.txt".to_string(), 1),
            "previous wraps backwards, skipping the file without conflicts"
        );
    }

    #[gpui::test]
    async fn test_conflict_navigation_and_resolution(cx: &mut TestAppContext) {
        init_test(cx);

        let text = "
            one
            <<<<<<< HEAD
            ours-1
            =======
            theirs-1
            >>>>>>> feature
            three
            <<<<<<< HEAD
            ours-2
            =======
            theirs-2
            >>>>>>> feature
            five
        "
        .unindent();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "a.txt": text,
            }),
        )
        .await;
        fs.with_git_state(path!("/project/.git").as_ref(), true, |state| {
            state.unmerged_paths.insert(
                repo_path("a.txt"),
                UnmergedStatus {
                    first_head: UnmergedStatusCode::Updated,
                    second_head: UnmergedStatusCode::Updated,
                },
            );
            state.refs.insert("MERGE_HEAD".into(), "123".into());
        })
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/a.txt"), cx)
            })
            .await
            .unwrap();

        let (editor, cx) = cx.add_window_view(|window, cx| {
            Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx)
        });
        cx.run_until_parked();

        editor.update(cx, |editor, cx| {
            let conflicts = editor
                .addon::<ConflictAddon>()
                .expect("conflict addon should be registered")
                .all_conflicts(cx)
                .count();
            assert_eq!(conflicts, 2);
        });

        editor.update_in(cx, |editor, window, cx| {
            go_to_conflict(editor, Direction::Next, window, cx);
            assert_eq!(cursor_row(editor, cx), 1);
            go_to_conflict(editor, Direction::Next, window, cx);
            assert_eq!(cursor_row(editor, cx), 7);
            go_to_conflict(editor, Direction::Next, window, cx);
            assert_eq!(cursor_row(editor, cx), 1, "next wraps to the first conflict");
            go_to_conflict(editor, Direction::Prev, window, cx);
            assert_eq!(
                cursor_row(editor, cx),
                7,
                "previous wraps to the last conflict"
            );
        });

        editor.update_in(cx, |editor, window, cx| {
            accept_conflict_at_cursor(editor, ConflictSide::Ours, window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "
                one
                <<<<<<< HEAD
                ours-1
                =======
                theirs-1
                >>>>>>> feature
                three
                ours-2
                five
            "
            .unindent(),
        );

        editor.update_in(cx, |editor, window, cx| {
            go_to_conflict(editor, Direction::Next, window, cx);
            accept_conflict_at_cursor(editor, ConflictSide::Both, window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "
                one
                ours-1
                theirs-1
                three
                ours-2
                five
            "
            .unindent(),
        );

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor
                    .addon::<ConflictAddon>()
                    .expect("conflict addon should be registered")
                    .all_conflicts(cx)
                    .count(),
                0
            );
        });
    }
}
