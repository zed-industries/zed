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
use language::{Anchor, Buffer, BufferId, BufferSnapshot};
use project::{
    ConflictRegion, ConflictSet, ConflictSetSnapshot, ConflictSetUpdate, Project, ProjectItem as _,
    git_store::{
        AutoResolvePattern, AutoResolveTakeSide, GitStore, GitStoreEvent, LanguageMergeContext,
        RepositoryEvent,
    },
};
use settings::{AutoResolveTake, Settings};
use std::{ops::Range, sync::Arc};

use crate::git_panel_settings::GitPanelSettings;
use ui::{ButtonLike, Divider, Tooltip, prelude::*};
use util::{ResultExt as _, debug_panic, maybe};
use workspace::{
    HideStatusItem, StatusItemView, Workspace, item::ItemHandle, notifications::NotificationId,
};
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
    banner_block_id: Option<CustomBlockId>,
    conflict_set: Entity<ConflictSet>,
    _subscription: Subscription,
}

#[derive(Debug, Clone, Copy)]
enum BannerState {
    Resolvable {
        fully_resolvable: usize,
        partially_resolvable: usize,
        total: usize,
    },
    NoneResolvable,
    NoBase,
}

struct AutoResolveToast;

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
                banner_block_id: None,
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

    update_auto_resolve_banner(editor, buffer_id, &conflict_set, &snapshot, cx);
}

fn update_auto_resolve_banner(
    editor: &mut Editor,
    buffer_id: BufferId,
    conflict_snapshot: &ConflictSetSnapshot,
    multibuffer_snapshot: &editor::MultiBufferSnapshot,
    cx: &mut Context<Editor>,
) {
    let existing_banner = editor
        .addon_mut::<ConflictAddon>()
        .unwrap()
        .buffers
        .get_mut(&buffer_id)
        .and_then(|bc| bc.banner_block_id.take());
    if let Some(block_id) = existing_banner {
        editor.remove_blocks(HashSet::from_iter([block_id]), None, cx);
    }

    if conflict_snapshot.conflicts.is_empty() {
        return;
    }

    let Some(buffer_entity) = editor.buffer().read(cx).buffer(buffer_id) else {
        return;
    };
    let buffer_snapshot = buffer_entity.read(cx).snapshot();
    let patterns = compile_auto_resolve_patterns(cx);
    let language = buffer_entity.read(cx).language().cloned();
    let structural = language.and_then(|language| {
        LanguageMergeContext::build(&buffer_snapshot, language, &conflict_snapshot.conflicts)
    });
    let Some(state) = banner_state_for(
        conflict_snapshot,
        &buffer_snapshot,
        &patterns,
        structural.as_ref(),
    ) else {
        return;
    };

    // Anchor at the buffer's first position so the banner is the first thing
    // the user sees when opening a conflicted file, decoupled from any single
    // conflict region. In multibuffer views (e.g. project diff) the buffer's
    // first position may sit before the first excerpt, in which case we fall
    // back to the first conflict's start so the banner is at least visible
    // somewhere within the multibuffer.
    let buffer_start = buffer_snapshot.anchor_before(0);
    let anchor = multibuffer_snapshot
        .anchor_in_excerpt(buffer_start)
        .or_else(|| {
            conflict_snapshot
                .conflicts
                .first()
                .and_then(|conflict| multibuffer_snapshot.anchor_in_excerpt(conflict.range.start))
        });
    let Some(anchor) = anchor else {
        return;
    };

    let editor_handle = cx.weak_entity();
    let buffer_handle = buffer_entity.downgrade();

    let block = BlockProperties {
        placement: BlockPlacement::Above(anchor),
        height: Some(1),
        style: BlockStyle::Sticky,
        render: Arc::new(move |cx| {
            render_auto_resolve_banner(state, buffer_handle.clone(), editor_handle.clone(), cx)
        }),
        priority: 0,
    };

    let new_block_ids = editor.insert_blocks(vec![block], None, cx);
    if let Some(banner_id) = new_block_ids.into_iter().next()
        && let Some(buffer_conflicts) = editor
            .addon_mut::<ConflictAddon>()
            .unwrap()
            .buffers
            .get_mut(&buffer_id)
    {
        buffer_conflicts.banner_block_id = Some(banner_id);
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

fn compile_auto_resolve_patterns(cx: &App) -> Vec<AutoResolvePattern> {
    GitPanelSettings::get_global(cx)
        .auto_resolve_patterns
        .iter()
        .filter_map(|raw| {
            let regex = regex::Regex::new(&raw.pattern).log_err()?;
            let take = match raw.take {
                AutoResolveTake::Ours => AutoResolveTakeSide::Ours,
                AutoResolveTake::Theirs => AutoResolveTakeSide::Theirs,
            };
            Some(AutoResolvePattern { regex, take })
        })
        .collect()
}

fn banner_state_for(
    conflict_snapshot: &ConflictSetSnapshot,
    buffer_snapshot: &BufferSnapshot,
    patterns: &[AutoResolvePattern],
    structural: Option<&LanguageMergeContext>,
) -> Option<BannerState> {
    let total = conflict_snapshot.conflicts.len();
    if total == 0 {
        return None;
    }
    let any_with_base = conflict_snapshot
        .conflicts
        .iter()
        .any(|conflict| conflict.base.is_some());
    if !any_with_base {
        return Some(BannerState::NoBase);
    }
    let (mut fully_resolvable, mut partially_resolvable) = (0, 0);
    for (_conflict, summary) in
        conflict_snapshot.decomposition_summary(buffer_snapshot, patterns, structural)
    {
        if summary.fully_resolved {
            fully_resolvable += 1;
        } else {
            partially_resolvable += 1;
        }
    }
    if fully_resolvable == 0 && partially_resolvable == 0 {
        return Some(BannerState::NoneResolvable);
    }
    Some(BannerState::Resolvable {
        fully_resolvable,
        partially_resolvable,
        total,
    })
}

fn render_auto_resolve_banner(
    state: BannerState,
    buffer: WeakEntity<Buffer>,
    editor: WeakEntity<Editor>,
    cx: &mut BlockContext,
) -> AnyElement {
    let label: SharedString = match state {
        BannerState::Resolvable {
            fully_resolvable,
            partially_resolvable,
            total,
        } => {
            let remaining = total.saturating_sub(fully_resolvable);
            if partially_resolvable == 0 {
                format!(
                    "Auto-Resolve — {} non-conflicting change{} • {} will remain",
                    fully_resolvable,
                    if fully_resolvable == 1 { "" } else { "s" },
                    remaining,
                )
                .into()
            } else {
                format!(
                    "Auto-Resolve — {} fully + {} partially • {} will remain",
                    fully_resolvable, partially_resolvable, remaining,
                )
                .into()
            }
        }
        BannerState::NoneResolvable => {
            "Auto-Resolve — no non-conflicting changes detected".into()
        }
        BannerState::NoBase => {
            "Auto-Resolve — requires diff3 conflict markers (run `git config merge.conflictStyle zdiff3`)"
                .into()
        }
    };

    let tooltip_text: Option<SharedString> = match state {
        BannerState::Resolvable { .. } => None,
        BannerState::NoneResolvable => {
            Some("All conflicts have changes on both sides; manual resolution required.".into())
        }
        BannerState::NoBase => Some(
            "Run `git config merge.conflictStyle zdiff3` in your terminal to enable diff3 markers for future merges."
                .into(),
        ),
    };

    let enabled = matches!(state, BannerState::Resolvable { .. });
    let mut button = Button::new("auto-resolve", label)
        .label_size(LabelSize::Small)
        .disabled(!enabled);

    if enabled {
        button = button.on_click(move |_, window, cx| {
            auto_resolve_buffer(buffer.clone(), editor.clone(), window, cx);
        });
    }

    if let Some(tooltip_text) = tooltip_text {
        button = button.tooltip(move |_, cx| Tooltip::simple(tooltip_text.clone(), cx));
    }

    h_flex()
        .id(cx.block_id)
        .h(cx.line_height)
        .ml(cx.margins.gutter.width)
        .gap_1()
        .bg(cx.theme().colors().editor_background)
        .child(button)
        .into_any()
}

pub(crate) fn auto_resolve_buffer(
    buffer: WeakEntity<Buffer>,
    editor: WeakEntity<Editor>,
    _window: &mut Window,
    cx: &mut App,
) {
    let Some(buffer) = buffer.upgrade() else {
        return;
    };
    let Some(editor) = editor.upgrade() else {
        return;
    };

    let buffer_id = buffer.read(cx).remote_id();
    let conflict_set = editor
        .read(cx)
        .addon::<ConflictAddon>()
        .and_then(|addon| addon.conflict_set(buffer_id));
    let Some(conflict_set) = conflict_set else {
        return;
    };

    let conflict_snapshot = conflict_set.read(cx).snapshot();
    let total = conflict_snapshot.conflicts.len();

    let patterns = compile_auto_resolve_patterns(cx);
    let language = buffer.read(cx).language().cloned();
    let (edits, breakdown) = {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let structural = language.clone().and_then(|language| {
            LanguageMergeContext::build(&buffer_snapshot, language, &conflict_snapshot.conflicts)
        });
        let edits = conflict_snapshot.auto_resolution_edits(
            &buffer_snapshot,
            &patterns,
            structural.as_ref(),
        );
        let breakdown =
            classify_outcomes(&conflict_snapshot, &buffer_snapshot, &patterns, structural.as_ref());
        (edits, breakdown)
    };
    if edits.is_empty() {
        return;
    }

    buffer.update(cx, |buffer, cx| {
        buffer.edit(edits, None, cx);
    });

    if let Some(workspace) = editor.read(cx).workspace() {
        let message = breakdown.toast_message(total);
        workspace.update(cx, |workspace, cx| {
            workspace.show_toast(
                workspace::Toast::new(NotificationId::unique::<AutoResolveToast>(), message),
                cx,
            );
        });
    }
}

#[derive(Default)]
struct OutcomeBreakdown {
    fully_structural: usize,
    fully_line: usize,
    simplified: usize,
    deferred_with_reason: Vec<project::git_store::DeferReason>,
}

impl OutcomeBreakdown {
    fn toast_message(&self, total: usize) -> String {
        let resolved = self.fully_structural + self.fully_line;
        let remaining = total.saturating_sub(resolved);
        let mut message = String::new();
        message.push_str(&format!(
            "Auto-resolved {} of {} conflict{}",
            resolved,
            total,
            if total == 1 { "" } else { "s" },
        ));
        if self.fully_structural > 0 && self.fully_line > 0 {
            message.push_str(&format!(
                " ({} structural, {} line)",
                self.fully_structural, self.fully_line,
            ));
        } else if self.fully_structural > 0 {
            message.push_str(&format!(" ({} structural)", self.fully_structural));
        }
        if self.simplified > 0 {
            message.push_str(&format!(
                "; simplified {} more",
                self.simplified,
            ));
        }
        message.push_str(&format!("; {} remain", remaining));
        if let Some(reason) = self.deferred_with_reason.first() {
            if let Some(snippet) = describe_defer_reason(reason) {
                message.push_str(&format!(" \u{2014} {}", snippet));
            }
        }
        message
    }
}

fn classify_outcomes(
    snapshot: &ConflictSetSnapshot,
    buffer: &language::BufferSnapshot,
    patterns: &[AutoResolvePattern],
    structural: Option<&LanguageMergeContext>,
) -> OutcomeBreakdown {
    use project::git_store::StructuralMergeOutcome;
    let mut out = OutcomeBreakdown::default();
    for conflict in snapshot.conflicts.iter() {
        if let Some(structural) = structural {
            match structural.try_merge_region(conflict) {
                StructuralMergeOutcome::Resolved { .. } => {
                    out.fully_structural += 1;
                    continue;
                }
                StructuralMergeOutcome::Deferred(reason) => {
                    out.deferred_with_reason.push(reason);
                }
            }
        }
        let Some(segments) = conflict.decompose(buffer, patterns) else {
            continue;
        };
        let summary = project::RegionSummary::from_segments(&segments);
        if !summary.is_improvement {
            continue;
        }
        if summary.fully_resolved {
            out.fully_line += 1;
        } else {
            out.simplified += 1;
        }
    }
    out
}

fn describe_defer_reason(reason: &project::git_store::DeferReason) -> Option<String> {
    use project::git_store::DeferReason;
    Some(match reason {
        DeferReason::BothModifiedDifferently { key } => {
            format!("both branches modified `{}`", key)
        }
        DeferReason::DeleteVsModify { key, .. } => {
            format!("`{}` deleted on one side, modified on the other", key)
        }
        DeferReason::BothAddedDifferently { key } => {
            format!("both branches added `{}` differently", key)
        }
        DeferReason::CrossRegionKeyCollision { key, .. } => {
            format!("`{}` was added in multiple regions", key)
        }
        DeferReason::OrderedHunksOverlap => "ordered-list edits overlap".into(),
        _ => return None,
    })
}

pub(crate) fn auto_resolve_in_editor(
    editor: Entity<Editor>,
    window: &mut Window,
    cx: &mut App,
) {
    let buffer_ids: Vec<BufferId> = editor
        .read(cx)
        .addon::<ConflictAddon>()
        .map(|addon| addon.buffers.keys().copied().collect())
        .unwrap_or_default();

    let multibuffer = editor.read(cx).buffer().clone();
    for buffer_id in buffer_ids {
        let Some(buffer) = multibuffer.read(cx).buffer(buffer_id) else {
            continue;
        };
        auto_resolve_buffer(buffer.downgrade(), editor.downgrade(), window, cx);
    }
}

pub(crate) fn auto_resolve_in_project(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let editors: Vec<Entity<Editor>> = workspace.items_of_type::<Editor>(cx).collect();
    for editor in editors {
        auto_resolve_in_editor(editor, window, cx);
    }
}

fn collect_conflicted_file_paths(project: &Project, cx: &App) -> Vec<String> {
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

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .agent
                .get_or_insert_default()
                .show_merge_conflict_indicator = Some(false);
        }))
    }
}
