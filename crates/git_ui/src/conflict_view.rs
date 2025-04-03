use collections::HashSet;
use editor::{
    Editor, RowHighlightOptions,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
};
use gpui::{App, Context, Entity, Hsla, InteractiveElement as _, ParentElement as _, WeakEntity};
use language::{Anchor, Buffer, OffsetRangeExt as _};
use project::{ConflictRegion, ConflictSet, ConflictSetUpdate};
use std::{ops::Range, sync::Arc};
use ui::{ActiveTheme, AnyElement, Button, Clickable as _, Element as _, Styled, h_flex};

struct ConflictAddon {
    conflict_set: Entity<ConflictSet>,
    block_ids: Vec<(Anchor, CustomBlockId)>,
}

impl editor::Addon for ConflictAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

pub fn register_editor(editor: &mut Editor, buffer: Entity<Buffer>, cx: &mut Context<Editor>) {
    let Some(project) = &editor.project else {
        return;
    };
    let git_store = project.read(cx).git_store().clone();
    let conflict_set_task =
        git_store.update(cx, |git_store, cx| git_store.open_conflict_set(buffer, cx));

    cx.spawn(async move |editor, cx| {
        let conflict_set = conflict_set_task.await?;

        let conflict_view = ConflictAddon {
            conflict_set,
            block_ids: Vec::new(),
        };

        editor.update(cx, |editor, cx| {
            cx.subscribe(&conflict_view.conflict_set, |editor, _, event, cx| {
                conflicts_updated(editor, event, cx);
            })
            .detach();
            editor.register_addon(conflict_view);
        })
    })
    .detach();
}

fn conflicts_updated(editor: &mut Editor, event: &ConflictSetUpdate, cx: &mut Context<Editor>) {
    let conflict_set = editor
        .addon::<ConflictAddon>()
        .unwrap()
        .conflict_set
        .clone();

    if !conflict_set.read(cx).has_conflict {
        let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
        conflict_addon.block_ids.clear();
        return;
    }

    let conflict_set = conflict_set.read(cx).snapshot();
    let buffer = editor.buffer().read(cx).snapshot(cx);
    let Some((excerpt_id, _, _)) = buffer.as_singleton() else {
        return;
    };
    let changed_conflicts = &conflict_set.conflicts[event.new_range.clone()];
    let editor_handle = cx.weak_entity();
    let blocks = changed_conflicts.iter().map(|conflict| {
        let editor_handle = editor_handle.clone();
        BlockProperties {
            placement: BlockPlacement::Above(
                buffer
                    .anchor_in_excerpt(*excerpt_id, conflict.range.start)
                    .unwrap(),
            ),
            height: 1,
            style: BlockStyle::Fixed,
            render: Arc::new({
                let conflict = conflict.clone();
                move |cx| render_conflict_buttons(&conflict, editor_handle.clone(), cx)
            }),
            priority: 0,
        }
    });
    let new_block_ids = editor.insert_blocks(blocks, None, cx);

    let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
    let removed_block_ids = conflict_addon
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

    update_conflict_highlighting(
        editor,
        conflict_set.conflicts.iter(),
        &buffer,
        *excerpt_id,
        cx,
    );
}

fn update_conflict_highlighting<'a>(
    editor: &mut Editor,
    conflicts: impl Iterator<Item = &'a ConflictRegion>,
    buffer: &editor::MultiBufferSnapshot,
    excerpt_id: editor::ExcerptId,
    cx: &mut Context<Editor>,
) {
    let theme = cx.theme().clone();
    let colors = theme.colors();
    editor.clear_row_highlights::<ConflictsOuter>();
    editor.clear_row_highlights::<ConflictsOurs>();
    editor.clear_row_highlights::<ConflictsOursMarker>();
    editor.clear_row_highlights::<ConflictsTheirs>();
    editor.clear_row_highlights::<ConflictsTheirsMarker>();
    for conflict in conflicts {
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
            colors.version_control_conflict_ours,
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
            colors.version_control_conflict_theirs,
            RowHighlightOptions {
                include_gutter: false,
                ..Default::default()
            },
            cx,
        );
    }
}

fn render_conflict_buttons(
    conflict: &ConflictRegion,
    editor: WeakEntity<Editor>,
    cx: &mut BlockContext,
) -> AnyElement {
    h_flex()
        .id(cx.block_id)
        .gap_1()
        .child(Button::new("ours", "Accept Ours").on_click({
            let editor = editor.clone();
            let range = conflict.range.clone();
            let ours = conflict.ours.clone();
            move |_, _, cx| resolve_conflict(editor.clone(), range.clone(), &[ours.clone()], cx)
        }))
        .child(Button::new("theirs", "Accept Theirs").on_click({
            let editor = editor.clone();
            let range = conflict.range.clone();
            let theirs = conflict.theirs.clone();
            move |_, _, cx| resolve_conflict(editor.clone(), range.clone(), &[theirs.clone()], cx)
        }))
        .child(Button::new("both", "Accept Both").on_click({
            let editor = editor.clone();
            let range = conflict.range.clone();
            let ours = conflict.ours.clone();
            let theirs = conflict.theirs.clone();
            move |_, _, cx| {
                resolve_conflict(
                    editor.clone(),
                    range.clone(),
                    &[ours.clone(), theirs.clone()],
                    cx,
                )
            }
        }))
        .into_any()
}

fn resolve_conflict(
    editor: WeakEntity<Editor>,
    conflict_range: Range<Anchor>,
    ranges: &[Range<Anchor>],
    cx: &mut App,
) {
    let Some(editor) = editor.upgrade() else {
        return;
    };

    let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() else {
        return;
    };
    let snapshot = buffer.read(cx).snapshot();

    let mut deletions = Vec::new();
    let empty = "";
    let outer_range = conflict_range.to_offset(&snapshot);
    let mut offset = outer_range.start;
    for kept_range in ranges {
        let kept_range = kept_range.to_offset(&snapshot);
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
        let multibuffer = editor.buffer().read(cx).snapshot(cx);
        let conflict_addon = editor.addon_mut::<ConflictAddon>().unwrap();
        let Some((excerpt_id, _, buffer)) = multibuffer.as_singleton() else {
            return;
        };
        let conflict_set = conflict_addon.conflict_set.read(cx).snapshot();
        let conflicts = conflict_set
            .conflicts
            .iter()
            .filter(|conflict| conflict.range != conflict_range);
        let Ok(ix) = conflict_addon
            .block_ids
            .binary_search_by(|(start, _)| start.cmp(&conflict_range.start, &buffer))
        else {
            return;
        };
        let &(_, block_id) = &conflict_addon.block_ids[ix];
        update_conflict_highlighting(editor, conflicts, &multibuffer, *excerpt_id, cx);
        editor.remove_blocks(HashSet::from_iter([block_id]), None, cx);
    })
}

enum ConflictsOuter {}
enum ConflictsOurs {}
enum ConflictsTheirs {}
enum ConflictsOursMarker {}
enum ConflictsTheirsMarker {}
