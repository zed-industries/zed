use std::ops::Range;

use collections::HashMap;
use itertools::Itertools;
use text::{AnchorRangeExt, BufferId, ToPoint};
use ui::ViewContext;
use util::ResultExt;

use crate::Editor;

#[derive(Clone, Default)]
pub(super) struct LinkedEditingRanges(
    /// Ranges are non-overlapping and sorted by .0 (thus, [x + 1].start > [x].end must hold)
    pub HashMap<BufferId, Vec<(Range<text::Anchor>, Vec<Range<text::Anchor>>)>>,
);

impl LinkedEditingRanges {
    pub(super) fn get(
        &self,
        id: BufferId,
        anchor: Range<text::Anchor>,
        snapshot: &text::BufferSnapshot,
    ) -> Option<&(Range<text::Anchor>, Vec<Range<text::Anchor>>)> {
        let ranges_for_buffer = self.0.get(&id)?;
        let lower_bound = ranges_for_buffer
            .partition_point(|(range, _)| range.start.cmp(&anchor.start, snapshot).is_le());
        if lower_bound == 0 {
            // None of the linked ranges contains `anchor`.
            return None;
        }
        ranges_for_buffer
            .get(lower_bound - 1)
            .filter(|(range, _)| range.end.cmp(&anchor.end, snapshot).is_ge())
    }
    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
pub(super) fn refresh_linked_ranges(this: &mut Editor, cx: &mut ViewContext<Editor>) -> Option<()> {
    if this.pending_rename.is_some() {
        return None;
    }
    let project = this.project.clone()?;
    let buffer = this.buffer.read(cx);
    let mut applicable_selections = vec![];
    let selections = this.selections.all::<usize>(cx);
    let snapshot = buffer.snapshot(cx);
    for selection in selections {
        let cursor_position = selection.head();
        let start_position = snapshot.anchor_before(cursor_position);
        let end_position = snapshot.anchor_after(selection.tail());
        if start_position.buffer_id != end_position.buffer_id || end_position.buffer_id.is_none() {
            // Throw away selections spanning multiple buffers.
            continue;
        }
        if let Some(buffer) = end_position.buffer_id.and_then(|id| buffer.buffer(id)) {
            applicable_selections.push((
                buffer,
                start_position.text_anchor,
                end_position.text_anchor,
            ));
        }
    }
    if applicable_selections.is_empty() {
        return None;
    }
    this.linked_editing_range_task = Some(cx.spawn(|this, mut cx| async move {
        let highlights = project
            .update(&mut cx, |project, cx| {
                let mut linked_edits_tasks = vec![];

                for (buffer, start, end) in &applicable_selections {
                    let snapshot = buffer.read(cx).snapshot();
                    let buffer_id = buffer.read(cx).remote_id();

                    let linked_edits_task = project.linked_edit(&buffer, *start, cx);
                    let highlights = move || async move {
                        let edits = linked_edits_task.await.log_err()?;
                        // Find the range containing our current selection.
                        // We might not find one, because the selection contains both the start and end of the contained range
                        // (think of selecting <`html>foo`</html> - even though there's a matching closing tag, the selection goes beyond the range of the opening tag)
                        // or the language server may not have returned any ranges.

                        let start_point = start.to_point(&snapshot);
                        let end_point = end.to_point(&snapshot);
                        let _current_selection_contains_range = edits.iter().find(|range| {
                            range.start.to_point(&snapshot) <= start_point
                                && range.end.to_point(&snapshot) >= end_point
                        });
                        if _current_selection_contains_range.is_none() {
                            return None;
                        }
                        // Now link every range as each-others sibling.
                        let mut siblings: HashMap<Range<text::Anchor>, Vec<_>> = Default::default();
                        let mut insert_sorted_anchor =
                            |key: &Range<text::Anchor>, value: &Range<text::Anchor>| {
                                siblings.entry(key.clone()).or_default().push(value.clone());
                            };
                        for items in edits.into_iter().combinations(2) {
                            let Ok([first, second]): Result<[_; 2], _> = items.try_into() else {
                                unreachable!()
                            };

                            insert_sorted_anchor(&first, &second);
                            insert_sorted_anchor(&second, &first);
                        }
                        let mut siblings: Vec<(_, _)> = siblings.into_iter().collect();
                        siblings.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0, &snapshot));
                        Some((buffer_id, siblings))
                    };
                    linked_edits_tasks.push(highlights());
                }
                linked_edits_tasks
            })
            .log_err()?;

        let highlights = futures::future::join_all(highlights).await;

        this.update(&mut cx, |this, cx| {
            this.linked_edit_ranges.0.clear();
            if this.pending_rename.is_some() {
                return;
            }
            for (buffer_id, ranges) in highlights.into_iter().flatten() {
                this.linked_edit_ranges
                    .0
                    .entry(buffer_id)
                    .or_default()
                    .extend(ranges);
            }
            for (buffer_id, values) in this.linked_edit_ranges.0.iter_mut() {
                let Some(snapshot) = this
                    .buffer
                    .read(cx)
                    .buffer(*buffer_id)
                    .map(|buffer| buffer.read(cx).snapshot())
                else {
                    continue;
                };
                values.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0, &snapshot));
            }

            cx.notify();
        })
        .log_err();

        Some(())
    }));
    None
}
