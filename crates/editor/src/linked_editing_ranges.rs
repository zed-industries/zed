use std::ops::Range;

use collections::HashMap;
use indexmap::set::IndexSet;
use itertools::Itertools;
use text::{AnchorRangeExt, BufferId};
use ui::ViewContext;
use util::ResultExt;

use crate::Editor;

#[derive(Clone, Default)]
pub(super) struct LinkedEditingRanges(
    HashMap<BufferId, Vec<(Range<text::Anchor>, Vec<Range<text::Anchor>>)>>,
);

impl LinkedEditingRanges {
    pub(super) fn get(
        &self,
        id: BufferId,
        anchor: Range<text::Anchor>,
        snapshot: &text::BufferSnapshot,
    ) -> Option<&[Range<text::Anchor>]> {
        let ranges_for_buffer = self.0.get(&id)?;
        let lower_bound = ranges_for_buffer
            .partition_point(|(range, _)| range.start.cmp(&anchor.start, snapshot).is_lt());
        let entry = ranges_for_buffer.get(lower_bound)?;
        if anchor.start.cmp(&entry.0.start, snapshot).is_ge()
            && anchor.end.cmp(&entry.0.end, snapshot).is_le()
        {
            Some(&entry.1)
        } else {
            None
        }
    }
}
pub(super) fn refresh_linked_ranges(this: &mut Editor, cx: &mut ViewContext<Editor>) -> Option<()> {
    if this.pending_rename.is_some() {
        return None;
    }
    this.linked_edit_ranges.0.clear();
    let project = this.project.clone()?;
    let buffer = this.buffer.read(cx);
    let mut applicable_selections = vec![];
    for selection in this.selections.all::<usize>(cx) {
        let cursor_position = selection.head();
        let (cursor_buffer, start_position) =
            buffer.text_anchor_for_position(cursor_position, cx)?;
        let (tail_buffer, end_position) = buffer.text_anchor_for_position(selection.tail(), cx)?;
        if cursor_buffer != tail_buffer {
            // Throw away selections spanning multiple buffers.
            continue;
        }
        applicable_selections.push((cursor_buffer, start_position, end_position));
    }

    this.linked_editing_range_task = Some(cx.spawn(|this, mut cx| async move {
        if applicable_selections.is_empty() {
            return None;
        }
        let highlights = project
            .update(&mut cx, |project, cx| {
                let mut linked_edits_tasks = vec![];
                for (buffer, start, end) in &applicable_selections {
                    let snapshot = buffer.read(cx).snapshot();
                    let buffer_id = buffer.read(cx).remote_id();
                    let linked_edits_task = project.linked_edit(&buffer, start.clone(), cx);
                    let mut highlights = move || async move {
                        let edits = linked_edits_task.await.log_err()?;

                        // Find the range containing our current selection.
                        // We might not find one, because the selection contains both the start and end of the contained range
                        // (think of selecting <`html>foo`</html> - even though there's a matching closing tag, the selection goes beyond the range of the opening tag)
                        // or the language server may not have returned any ranges.

                        let _current_selection_contains_range = edits.iter().find(|range| {
                            range.start.cmp(start, &snapshot).is_le()
                                && range.end.cmp(end, &snapshot).is_ge()
                        })?;

                        // Now link every range as each-others sibling.
                        let mut siblings: Vec<(Range<text::Anchor>, Vec<_>)> = vec![];
                        let mut insert_sorted_anchor =
                            |key: &Range<text::Anchor>, value: &Range<text::Anchor>| {
                                let lower_bound = siblings.partition_point(|entry| {
                                    entry.0.start.cmp(&key.start, &snapshot).is_lt()
                                });
                                dbg!(&lower_bound);
                                if siblings
                                    .get(lower_bound)
                                    .filter(|entry| entry.0.end.cmp(&key.end, &snapshot).is_ge())
                                    .is_some()
                                {
                                    siblings[lower_bound].1.push(value.clone());
                                } else {
                                    siblings.push((key.clone(), vec![value.clone()]));
                                }
                            };
                        for items in edits.into_iter().combinations(2) {
                            let Ok([first, second]): Result<[_; 2], _> = items.try_into() else {
                                unreachable!()
                            };

                            insert_sorted_anchor(&first, &second);
                            insert_sorted_anchor(&second, &first);
                        }
                        Some((buffer_id, siblings))
                    };
                    linked_edits_tasks.push(highlights());
                }
                linked_edits_tasks
            })
            .log_err()?;

        let highlights = futures::future::join_all(highlights).await;

        this.update(&mut cx, |this, cx| {
            if this.pending_rename.is_some() {
                return;
            }
            for (buffer_id, ranges) in highlights.into_iter().filter_map(|x| x) {
                this.linked_edit_ranges.0.insert(buffer_id, ranges);
            }

            cx.notify();
        })
        .log_err();

        Some(())
    }));
    None
}
