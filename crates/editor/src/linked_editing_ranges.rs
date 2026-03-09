use collections::HashMap;
use gpui::{AppContext, Context, Entity, Window};
use itertools::Itertools;
use language::Buffer;
use multi_buffer::MultiBufferOffset;
use std::{ops::Range, sync::Arc, time::Duration};
use text::{Anchor, AnchorRangeExt, Bias, BufferId, ToOffset, ToPoint};
use util::ResultExt;

use crate::Editor;

#[derive(Clone, Default)]
pub(super) struct LinkedEditingRanges(
    /// Ranges are non-overlapping and sorted by .0 (thus, [x + 1].start > [x].end must hold)
    pub HashMap<BufferId, Vec<(Range<Anchor>, Vec<Range<Anchor>>)>>,
);

impl LinkedEditingRanges {
    pub(super) fn get(
        &self,
        id: BufferId,
        anchor: Range<Anchor>,
        snapshot: &text::BufferSnapshot,
    ) -> Option<&(Range<Anchor>, Vec<Range<Anchor>>)> {
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

    pub(super) fn clear(&mut self) {
        self.0.clear();
    }
}

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

// TODO do not refresh anything at all, if the settings/capabilities do not have it enabled.
pub(super) fn refresh_linked_ranges(
    editor: &mut Editor,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Option<()> {
    if !editor.mode().is_full() || editor.pending_rename.is_some() {
        return None;
    }
    let project = editor.project()?.downgrade();

    editor.linked_editing_range_task = Some(cx.spawn_in(window, async move |editor, cx| {
        cx.background_executor().timer(UPDATE_DEBOUNCE).await;

        let mut applicable_selections = Vec::new();
        editor
            .update(cx, |editor, cx| {
                let display_snapshot = editor.display_snapshot(cx);
                let selections = editor
                    .selections
                    .all::<MultiBufferOffset>(&display_snapshot);
                let snapshot = display_snapshot.buffer_snapshot();
                let buffer = editor.buffer.read(cx);
                for selection in selections {
                    let cursor_position = selection.head();
                    let start_position = snapshot.anchor_before(cursor_position);
                    let end_position = snapshot.anchor_after(selection.tail());
                    if start_position.text_anchor.buffer_id != end_position.text_anchor.buffer_id
                        || end_position.text_anchor.buffer_id.is_none()
                    {
                        // Throw away selections spanning multiple buffers.
                        continue;
                    }
                    if let Some(buffer) = buffer.buffer_for_anchor(end_position, cx) {
                        applicable_selections.push((
                            buffer,
                            start_position.text_anchor,
                            end_position.text_anchor,
                        ));
                    }
                }
            })
            .ok()?;

        if applicable_selections.is_empty() {
            return None;
        }

        let highlights = project
            .update(cx, |project, cx| {
                let mut linked_edits_tasks = vec![];
                for (buffer, start, end) in &applicable_selections {
                    let linked_edits_task = project.linked_edits(buffer, *start, cx);
                    let cx = cx.to_async();
                    let highlights = async move {
                        let edits = linked_edits_task.await.log_err()?;
                        let snapshot = cx.read_entity(&buffer, |buffer, _| buffer.snapshot());
                        let buffer_id = snapshot.remote_id();

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
                        _current_selection_contains_range?;
                        // Now link every range as each-others sibling.
                        let mut siblings: HashMap<Range<Anchor>, Vec<_>> = Default::default();
                        let mut insert_sorted_anchor =
                            |key: &Range<Anchor>, value: &Range<Anchor>| {
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
                    linked_edits_tasks.push(highlights);
                }
                linked_edits_tasks
            })
            .ok()?;

        let highlights = futures::future::join_all(highlights).await;

        editor
            .update(cx, |this, cx| {
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
            .ok()?;

        Some(())
    }));
    None
}

/// Accumulates edits destined for linked editing ranges, for example, matching
/// HTML/JSX tags, across one or more buffers. Edits are stored as anchor ranges
/// so they track buffer changes and are only resolved to concrete points at
/// apply time.
pub struct LinkedEdits(HashMap<Entity<Buffer>, Vec<(Range<Anchor>, Arc<str>)>>);

impl LinkedEdits {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    /// Queries the editor's linked editing ranges for the given anchor range and, if any
    /// are found, records them paired with `text` for later application.
    pub(crate) fn push(
        &mut self,
        editor: &Editor,
        anchor_range: Range<Anchor>,
        text: Arc<str>,
        cx: &gpui::App,
    ) {
        if let Some(editing_ranges) = editor.linked_editing_ranges_for(anchor_range, cx) {
            for (buffer, ranges) in editing_ranges {
                self.0
                    .entry(buffer)
                    .or_default()
                    .extend(ranges.into_iter().map(|range| (range, text.clone())));
            }
        }
    }

    /// Resolves all stored anchor ranges to points using the current buffer snapshot,
    /// sorts them, and applies the edits.
    pub fn apply(self, cx: &mut Context<Editor>) {
        self.apply_inner(false, cx);
    }

    /// Like [`apply`](Self::apply), but empty ranges (where start == end) are
    /// expanded one character to the left before applying. For context, this
    /// was introduced in order to be available to `backspace` so as to delete a
    /// character in each linked range even when the selection was a cursor.
    pub fn apply_with_left_expansion(self, cx: &mut Context<Editor>) {
        self.apply_inner(true, cx);
    }

    fn apply_inner(self, expand_empty_ranges_left: bool, cx: &mut Context<Editor>) {
        for (buffer, ranges_edits) in self.0 {
            buffer.update(cx, |buffer, cx| {
                let snapshot = buffer.snapshot();
                let edits = ranges_edits
                    .into_iter()
                    .map(|(range, text)| {
                        let mut start = range.start.to_point(&snapshot);
                        let end = range.end.to_point(&snapshot);

                        if expand_empty_ranges_left && start == end {
                            let offset = range.start.to_offset(&snapshot).saturating_sub(1);
                            start = snapshot.clip_point(offset.to_point(&snapshot), Bias::Left);
                        }

                        (start..end, text)
                    })
                    .sorted_by_key(|(range, _)| range.start);

                buffer.edit(edits, None, cx);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{editor_tests::init_test, test::editor_test_context::EditorTestContext};
    use gpui::TestAppContext;
    use text::Point;

    #[gpui::test]
    async fn test_linked_edits_push_and_apply(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;

        cx.set_state("<diˇv></div>");
        cx.update_editor(|editor, _window, cx| {
            editor
                .set_linked_edit_ranges_for_testing(
                    vec![(
                        Point::new(0, 1)..Point::new(0, 4),
                        vec![Point::new(0, 7)..Point::new(0, 10)],
                    )],
                    cx,
                )
                .unwrap();
        });

        cx.simulate_keystroke("x");
        cx.assert_editor_state("<dixˇv></dixv>");
    }

    #[gpui::test]
    async fn test_linked_edits_backspace(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;

        cx.set_state("<divˇ></div>");
        cx.update_editor(|editor, _window, cx| {
            editor
                .set_linked_edit_ranges_for_testing(
                    vec![(
                        Point::new(0, 1)..Point::new(0, 4),
                        vec![Point::new(0, 7)..Point::new(0, 10)],
                    )],
                    cx,
                )
                .unwrap();
        });

        cx.update_editor(|editor, window, cx| {
            editor.backspace(&Default::default(), window, cx);
        });
        cx.assert_editor_state("<diˇ></di>");
    }

    #[gpui::test]
    async fn test_linked_edits_delete(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;

        cx.set_state("<ˇdiv></div>");
        cx.update_editor(|editor, _window, cx| {
            editor
                .set_linked_edit_ranges_for_testing(
                    vec![(
                        Point::new(0, 1)..Point::new(0, 4),
                        vec![Point::new(0, 7)..Point::new(0, 10)],
                    )],
                    cx,
                )
                .unwrap();
        });

        cx.update_editor(|editor, window, cx| {
            editor.delete(&Default::default(), window, cx);
        });
        cx.assert_editor_state("<ˇiv></iv>");
    }

    #[gpui::test]
    async fn test_linked_edits_selection(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;

        cx.set_state("<«divˇ»></div>");
        cx.update_editor(|editor, _window, cx| {
            editor
                .set_linked_edit_ranges_for_testing(
                    vec![(
                        Point::new(0, 1)..Point::new(0, 4),
                        vec![Point::new(0, 7)..Point::new(0, 10)],
                    )],
                    cx,
                )
                .unwrap();
        });

        cx.simulate_keystrokes("s p a n");
        cx.assert_editor_state("<spanˇ></span>");
    }
}
