use agent_settings::AgentSettings;
use collections::{HashMap, HashSet};
use editor::{
    ConflictsOurs, ConflictsOursMarker, ConflictsOuter, ConflictsTheirs, ConflictsTheirsMarker,
    Editor, EditorEvent, MultiBuffer, RowHighlightOptions,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
};
use gpui::{
    App, ClickEvent, Context, Empty, Entity, InteractiveElement as _, ParentElement as _,
    Subscription, Task, WeakEntity,
};
use language::{Anchor, Buffer, BufferId};
use project::{
    ConflictRegion, ConflictSet, ConflictSetUpdate, Project, ProjectItem as _,
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

fn collect_conflicted_file_paths(project: &Project, cx: &App) -> Vec<String> {
    let git_store = project.git_store().read(cx);
    let mut paths = Vec::new();

    for repo in git_store.repositories().values() {
        let snapshot = repo.read(cx).snapshot();
        for (repo_path, _) in snapshot.merge.merge_heads_by_conflicted_path.iter() {
            if let Some(project_path) = repo.read(cx).repo_path_to_project_path(repo_path, cx) {
                paths.push(
                    project_path
                        .path
                        .as_std_path()
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }
    }

    paths
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
