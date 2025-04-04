use collections::{HashMap, HashSet};
use editor::{
    Editor, EditorEvent, ExcerptId, MultiBuffer, RowHighlightOptions,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
};
use gpui::{
    App, Context, Entity, Hsla, InteractiveElement as _, ParentElement as _, Subscription,
    WeakEntity,
};
use language::{Anchor, Buffer, BufferId, OffsetRangeExt as _};
use project::{ConflictRegion, ConflictSet, ConflictSetUpdate};
use std::{collections::hash_map, ops::Range, sync::Arc};
use ui::{
    ActiveTheme, AnyElement, Element as _, StatefulInteractiveElement, Styled,
    StyledTypography as _, div, h_flex, rems,
};

struct ConflictAddon {
    buffers: HashMap<BufferId, BufferConflicts>,
}

struct BufferConflicts {
    block_ids: Vec<(Anchor, CustomBlockId)>,
    _conflict_set: Entity<ConflictSet>,
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
    editor.register_addon(ConflictAddon {
        buffers: Default::default(),
    });

    let buffers = buffer.read(cx).all_buffers().clone();
    for buffer in buffers {
        buffer_added(editor, buffer, cx);
    }

    cx.subscribe(&cx.entity(), |editor, _, event, cx| match event {
        EditorEvent::ExcerptsAdded { buffer, .. } => buffer_added(editor, buffer.clone(), cx),
        EditorEvent::ExcerptsRemoved {
            removed_buffer_ids, ..
        } => buffers_removed(editor, removed_buffer_ids, cx),
        _ => {}
    })
    .detach();
}

fn buffer_added(editor: &mut Editor, buffer: Entity<Buffer>, cx: &mut Context<Editor>) {
    let Some(project) = &editor.project else {
        return;
    };
    let git_store = project.read(cx).git_store().clone();

    let hash_map::Entry::Vacant(entry) = editor
        .addon_mut::<ConflictAddon>()
        .unwrap()
        .buffers
        .entry(buffer.read(cx).remote_id())
    else {
        return;
    };

    let conflict_set = git_store.update(cx, |git_store, cx| {
        git_store.open_conflict_set(buffer.clone(), cx)
    });
    let conflict_count = conflict_set.read(cx).snapshot().conflicts.len();
    let subscription = cx.subscribe(&conflict_set, conflicts_updated);
    entry.insert(BufferConflicts {
        block_ids: Vec::new(),
        _conflict_set: conflict_set.clone(),
        _subscription: subscription,
    });
    conflicts_updated(
        editor,
        conflict_set,
        &ConflictSetUpdate {
            old_range: 0..0,
            new_range: 0..conflict_count,
            buffer_range: None,
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
            if removed_buffer_ids.contains(&buffer_id) {
                removed_block_ids.extend(buffer.block_ids.iter().map(|(_, block_id)| *block_id));
                false
            } else {
                true
            }
        });
    editor.remove_blocks(removed_block_ids, None, cx);
}

fn conflicts_updated(
    editor: &mut Editor,
    conflict_set: Entity<ConflictSet>,
    event: &ConflictSetUpdate,
    cx: &mut Context<Editor>,
) {
    let buffer_id = conflict_set.read(cx).snapshot.buffer_id;
    if !conflict_set.read(cx).has_conflict {
        let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
        conflict_addon.buffers.remove(&buffer_id);
        return;
    }

    let conflict_set = conflict_set.read(cx).snapshot();
    let multibuffer = editor.buffer().read(cx);
    let snapshot = multibuffer.snapshot(cx);
    let excerpts = multibuffer.excerpts_for_buffer(buffer_id, cx);
    let Some(buffer_snapshot) = excerpts
        .first()
        .and_then(|(excerpt_id, _)| snapshot.buffer_for_excerpt(*excerpt_id))
    else {
        return;
    };
    let changed_conflicts = &conflict_set.conflicts[event.new_range.clone()];
    let editor_handle = cx.weak_entity();

    editor.clear_row_highlights::<ConflictsOuter>();
    editor.clear_row_highlights::<ConflictsOurs>();
    editor.clear_row_highlights::<ConflictsOursMarker>();
    editor.clear_row_highlights::<ConflictsTheirs>();
    editor.clear_row_highlights::<ConflictsTheirsMarker>();

    let mut blocks = Vec::new();
    for conflict in changed_conflicts {
        let Some((excerpt_id, _)) = excerpts.iter().find(|(_, range)| {
            let precedes_start = range
                .context
                .start
                .cmp(&conflict.range.start, &buffer_snapshot)
                .is_le();
            let follows_end = range
                .context
                .end
                .cmp(&conflict.range.start, &buffer_snapshot)
                .is_ge();
            precedes_start && follows_end
        }) else {
            continue;
        };
        let excerpt_id = *excerpt_id;

        update_conflict_highlighting(editor, conflict, &snapshot, excerpt_id, cx);

        let Some(anchor) = snapshot.anchor_in_excerpt(excerpt_id, conflict.range.start) else {
            continue;
        };

        let editor_handle = editor_handle.clone();
        blocks.push(BlockProperties {
            placement: BlockPlacement::Above(anchor),
            height: 1,
            style: BlockStyle::Fixed,
            render: Arc::new({
                let conflict = conflict.clone();
                move |cx| render_conflict_buttons(&conflict, excerpt_id, editor_handle.clone(), cx)
            }),
            priority: 0,
        })
    }
    let new_block_ids = editor.insert_blocks(blocks, None, cx);

    let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
    if let Some(buffer_conflicts) = conflict_addon.buffers.get_mut(&buffer_id) {
        let removed_block_ids = buffer_conflicts
            .block_ids
            .splice(
                event.old_range.clone(),
                changed_conflicts
                    .iter()
                    .map(|conflict| conflict.range.start)
                    .zip(new_block_ids),
            )
            .map(|(_, id)| id)
            .collect();
        editor.remove_blocks(removed_block_ids, None, cx);
    }
}

fn update_conflict_highlighting(
    editor: &mut Editor,
    conflict: &ConflictRegion,
    buffer: &editor::MultiBufferSnapshot,
    excerpt_id: editor::ExcerptId,
    cx: &mut Context<Editor>,
) {
    let theme = cx.theme().clone();
    let colors = theme.colors();
    let outer_start = buffer
        .anchor_in_excerpt(excerpt_id, conflict.range.start)
        .unwrap();
    let outer_end = buffer
        .anchor_in_excerpt(excerpt_id, conflict.range.end)
        .unwrap();
    let our_start = buffer
        .anchor_in_excerpt(excerpt_id, conflict.ours.start)
        .unwrap();
    let our_end = buffer
        .anchor_in_excerpt(excerpt_id, conflict.ours.end)
        .unwrap();
    let their_start = buffer
        .anchor_in_excerpt(excerpt_id, conflict.theirs.start)
        .unwrap();
    let their_end = buffer
        .anchor_in_excerpt(excerpt_id, conflict.theirs.end)
        .unwrap();

    let mut ours_background = colors.version_control_conflict_ours;
    let mut ours_marker = colors.version_control_conflict_ours;
    let mut theirs_marker = colors.version_control_conflict_theirs;
    let mut theirs_background = colors.version_control_conflict_theirs;
    ours_marker.fade_out(0.4);
    theirs_marker.fade_out(0.4);
    ours_background.fade_out(0.7);
    theirs_background.fade_out(0.7);

    // Prevent diff hunk highlighting within the entire conflict region.
    editor.highlight_rows::<ConflictsOuter>(
        outer_start..outer_end,
        Hsla::default(),
        RowHighlightOptions {
            include_gutter: false,
            ..Default::default()
        },
        cx,
    );
    editor.highlight_rows::<ConflictsOurs>(
        our_start..our_end,
        ours_background,
        RowHighlightOptions {
            include_gutter: false,
            ..Default::default()
        },
        cx,
    );
    editor.highlight_rows::<ConflictsOursMarker>(
        outer_start..our_start,
        ours_marker,
        RowHighlightOptions {
            include_gutter: false,
            ..Default::default()
        },
        cx,
    );
    editor.highlight_rows::<ConflictsTheirs>(
        their_start..their_end,
        theirs_background,
        RowHighlightOptions {
            include_gutter: false,
            ..Default::default()
        },
        cx,
    );
    editor.highlight_rows::<ConflictsTheirsMarker>(
        their_end..outer_end,
        theirs_marker,
        RowHighlightOptions {
            include_gutter: false,
            ..Default::default()
        },
        cx,
    );
}

fn render_conflict_buttons(
    conflict: &ConflictRegion,
    excerpt_id: ExcerptId,
    editor: WeakEntity<Editor>,
    cx: &mut BlockContext,
) -> AnyElement {
    h_flex()
        .h(cx.line_height)
        .items_end()
        .ml(cx.gutter_dimensions.width)
        .id(cx.block_id)
        .gap_0p5()
        .child(
            div()
                .id("ours")
                .px_1()
                .child("Accept Ours")
                .rounded_t(rems(0.2))
                .text_ui_sm(cx)
                .hover(|this| this.bg(cx.theme().colors().element_background))
                .cursor_pointer()
                .on_click({
                    let editor = editor.clone();
                    let conflict = conflict.clone();
                    let ours = conflict.ours.clone();
                    move |_, _, cx| {
                        resolve_conflict(editor.clone(), excerpt_id, &conflict, &[ours.clone()], cx)
                    }
                }),
        )
        .child(
            div()
                .id("theirs")
                .px_1()
                .child("Accept Theirs")
                .rounded_t(rems(0.2))
                .text_ui_sm(cx)
                .hover(|this| this.bg(cx.theme().colors().element_background))
                .cursor_pointer()
                .on_click({
                    let editor = editor.clone();
                    let conflict = conflict.clone();
                    let theirs = conflict.theirs.clone();
                    move |_, _, cx| {
                        resolve_conflict(
                            editor.clone(),
                            excerpt_id,
                            &conflict,
                            &[theirs.clone()],
                            cx,
                        )
                    }
                }),
        )
        .child(
            div()
                .id("both")
                .px_1()
                .child("Accept Both")
                .rounded_t(rems(0.2))
                .text_ui_sm(cx)
                .hover(|this| this.bg(cx.theme().colors().element_background))
                .cursor_pointer()
                .on_click({
                    let editor = editor.clone();
                    let conflict = conflict.clone();
                    let ours = conflict.ours.clone();
                    let theirs = conflict.theirs.clone();
                    move |_, _, cx| {
                        resolve_conflict(
                            editor.clone(),
                            excerpt_id,
                            &conflict,
                            &[ours.clone(), theirs.clone()],
                            cx,
                        )
                    }
                }),
        )
        .into_any()
}

fn resolve_conflict(
    editor: WeakEntity<Editor>,
    excerpt_id: ExcerptId,
    resolved_conflict: &ConflictRegion,
    ranges: &[Range<Anchor>],
    cx: &mut App,
) {
    let Some(editor) = editor.upgrade() else {
        return;
    };

    let multibuffer = editor.read(cx).buffer().read(cx);
    let snapshot = multibuffer.snapshot(cx);
    let Some(buffer) = resolved_conflict
        .ours
        .end
        .buffer_id
        .and_then(|buffer_id| multibuffer.buffer(buffer_id))
    else {
        return;
    };
    let buffer_snapshot = buffer.read(cx).snapshot();

    let mut deletions = Vec::new();
    let empty = "";
    let outer_range = resolved_conflict.range.to_offset(&buffer_snapshot);
    let mut offset = outer_range.start;
    for kept_range in ranges {
        let kept_range = kept_range.to_offset(&buffer_snapshot);
        if kept_range.start > offset {
            deletions.push((offset..kept_range.start, empty));
        }
        offset = kept_range.end;
    }
    if outer_range.end > offset {
        deletions.push((offset..outer_range.end, empty));
    }

    buffer.update(cx, |buffer, cx| {
        buffer.edit(deletions, None, cx);
    });

    editor.update(cx, |editor, cx| {
        let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
        let Some(state) = conflict_addon.buffers.get_mut(&buffer_snapshot.remote_id()) else {
            return;
        };
        let Ok(ix) = state.block_ids.binary_search_by(|(start, _)| {
            start.cmp(&resolved_conflict.range.start, &buffer_snapshot)
        }) else {
            return;
        };
        let &(_, block_id) = &state.block_ids[ix];
        let start = snapshot
            .anchor_in_excerpt(excerpt_id, resolved_conflict.range.start)
            .unwrap();
        let end = snapshot
            .anchor_in_excerpt(excerpt_id, resolved_conflict.range.end)
            .unwrap();
        editor.remove_highlighted_rows::<ConflictsOuter>(vec![start..end], cx);
        editor.remove_highlighted_rows::<ConflictsOurs>(vec![start..end], cx);
        editor.remove_highlighted_rows::<ConflictsTheirs>(vec![start..end], cx);
        editor.remove_highlighted_rows::<ConflictsOursMarker>(vec![start..end], cx);
        editor.remove_highlighted_rows::<ConflictsTheirsMarker>(vec![start..end], cx);
        editor.remove_blocks(HashSet::from_iter([block_id]), None, cx);
    })
}

enum ConflictsOuter {}
enum ConflictsOurs {}
enum ConflictsTheirs {}
enum ConflictsOursMarker {}
enum ConflictsTheirsMarker {}
