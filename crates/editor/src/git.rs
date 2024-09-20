pub mod blame;

use std::ops::Range;

use git::diff::DiffHunkStatus;
use language::Point;
use multi_buffer::{Anchor, MultiBufferDiffHunk};

use crate::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    hunk_status, AnchorRangeExt, DisplayRow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayDiffHunk {
    Folded {
        display_row: DisplayRow,
    },

    Unfolded {
        diff_base_byte_range: Range<usize>,
        display_row_range: Range<DisplayRow>,
        multi_buffer_range: Range<Anchor>,
        status: DiffHunkStatus,
    },
}

impl DisplayDiffHunk {
    pub fn start_display_row(&self) -> DisplayRow {
        match self {
            &DisplayDiffHunk::Folded { display_row } => display_row,
            DisplayDiffHunk::Unfolded {
                display_row_range, ..
            } => display_row_range.start,
        }
    }

    pub fn contains_display_row(&self, display_row: DisplayRow) -> bool {
        let range = match self {
            &DisplayDiffHunk::Folded { display_row } => display_row..=display_row,

            DisplayDiffHunk::Unfolded {
                display_row_range, ..
            } => display_row_range.start..=display_row_range.end,
        };

        range.contains(&display_row)
    }
}

pub fn diff_hunk_to_display(
    hunk: &MultiBufferDiffHunk,
    snapshot: &DisplaySnapshot,
) -> DisplayDiffHunk {
    let hunk_start_point = Point::new(hunk.row_range.start.0, 0);
    let hunk_start_point_sub = Point::new(hunk.row_range.start.0.saturating_sub(1), 0);
    let hunk_end_point_sub = Point::new(
        hunk.row_range
            .end
            .0
            .saturating_sub(1)
            .max(hunk.row_range.start.0),
        0,
    );

    let status = hunk_status(hunk);
    let is_removal = status == DiffHunkStatus::Removed;

    let folds_start = Point::new(hunk.row_range.start.0.saturating_sub(2), 0);
    let folds_end = Point::new(hunk.row_range.end.0 + 2, 0);
    let folds_range = folds_start..folds_end;

    let containing_fold = snapshot.folds_in_range(folds_range).find(|fold| {
        let fold_point_range = fold.range.to_point(&snapshot.buffer_snapshot);
        let fold_point_range = fold_point_range.start..=fold_point_range.end;

        let folded_start = fold_point_range.contains(&hunk_start_point);
        let folded_end = fold_point_range.contains(&hunk_end_point_sub);
        let folded_start_sub = fold_point_range.contains(&hunk_start_point_sub);

        (folded_start && folded_end) || (is_removal && folded_start_sub)
    });

    if let Some(fold) = containing_fold {
        let row = fold.range.start.to_display_point(snapshot).row();
        DisplayDiffHunk::Folded { display_row: row }
    } else {
        let start = hunk_start_point.to_display_point(snapshot).row();

        let hunk_end_row = hunk.row_range.end.max(hunk.row_range.start);
        let hunk_end_point = Point::new(hunk_end_row.0, 0);

        let multi_buffer_start = snapshot.buffer_snapshot.anchor_after(hunk_start_point);
        let multi_buffer_end = snapshot.buffer_snapshot.anchor_before(hunk_end_point);
        let end = hunk_end_point.to_display_point(snapshot).row();

        DisplayDiffHunk::Unfolded {
            display_row_range: start..end,
            multi_buffer_range: multi_buffer_start..multi_buffer_end,
            status,
            diff_base_byte_range: hunk.diff_base_byte_range.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;
    use crate::{editor_tests::init_test, hunk_status};
    use gpui::{Context, TestAppContext};
    use language::Capability::ReadWrite;
    use multi_buffer::{ExcerptRange, MultiBuffer, MultiBufferRow};
    use project::{FakeFs, Project};
    use unindent::Unindent;
    #[gpui::test]
    async fn test_diff_hunks_in_range(cx: &mut TestAppContext) {
        use git::diff::DiffHunkStatus;
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.background_executor.clone());
        let project = Project::test(fs, [], cx).await;

        // buffer has two modified hunks with two rows each
        let buffer_1 = project.update(cx, |project, cx| {
            project.create_local_buffer(
                "
                        1.zero
                        1.ONE
                        1.TWO
                        1.three
                        1.FOUR
                        1.FIVE
                        1.six
                    "
                .unindent()
                .as_str(),
                None,
                cx,
            )
        });
        buffer_1.update(cx, |buffer, cx| {
            buffer.set_diff_base(
                Some(
                    "
                        1.zero
                        1.one
                        1.two
                        1.three
                        1.four
                        1.five
                        1.six
                    "
                    .unindent(),
                ),
                cx,
            );
        });

        // buffer has a deletion hunk and an insertion hunk
        let buffer_2 = project.update(cx, |project, cx| {
            project.create_local_buffer(
                "
                        2.zero
                        2.one
                        2.two
                        2.three
                        2.four
                        2.five
                        2.six
                    "
                .unindent()
                .as_str(),
                None,
                cx,
            )
        });
        buffer_2.update(cx, |buffer, cx| {
            buffer.set_diff_base(
                Some(
                    "
                        2.zero
                        2.one
                        2.one-and-a-half
                        2.two
                        2.three
                        2.four
                        2.six
                    "
                    .unindent(),
                ),
                cx,
            );
        });

        cx.background_executor.run_until_parked();

        let multibuffer = cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new(ReadWrite);
            multibuffer.push_excerpts(
                buffer_1.clone(),
                [
                    // excerpt ends in the middle of a modified hunk
                    ExcerptRange {
                        context: Point::new(0, 0)..Point::new(1, 5),
                        primary: Default::default(),
                    },
                    // excerpt begins in the middle of a modified hunk
                    ExcerptRange {
                        context: Point::new(5, 0)..Point::new(6, 5),
                        primary: Default::default(),
                    },
                ],
                cx,
            );
            multibuffer.push_excerpts(
                buffer_2.clone(),
                [
                    // excerpt ends at a deletion
                    ExcerptRange {
                        context: Point::new(0, 0)..Point::new(1, 5),
                        primary: Default::default(),
                    },
                    // excerpt starts at a deletion
                    ExcerptRange {
                        context: Point::new(2, 0)..Point::new(2, 5),
                        primary: Default::default(),
                    },
                    // excerpt fully contains a deletion hunk
                    ExcerptRange {
                        context: Point::new(1, 0)..Point::new(2, 5),
                        primary: Default::default(),
                    },
                    // excerpt fully contains an insertion hunk
                    ExcerptRange {
                        context: Point::new(4, 0)..Point::new(6, 5),
                        primary: Default::default(),
                    },
                ],
                cx,
            );
            multibuffer
        });

        let snapshot = multibuffer.read_with(cx, |b, cx| b.snapshot(cx));

        assert_eq!(
            snapshot.text(),
            "
                1.zero
                1.ONE
                1.FIVE
                1.six
                2.zero
                2.one
                2.two
                2.one
                2.two
                2.four
                2.five
                2.six"
                .unindent()
        );

        let expected = [
            (
                DiffHunkStatus::Modified,
                MultiBufferRow(1)..MultiBufferRow(2),
            ),
            (
                DiffHunkStatus::Modified,
                MultiBufferRow(2)..MultiBufferRow(3),
            ),
            //TODO: Define better when and where removed hunks show up at range extremities
            (
                DiffHunkStatus::Removed,
                MultiBufferRow(6)..MultiBufferRow(6),
            ),
            (
                DiffHunkStatus::Removed,
                MultiBufferRow(8)..MultiBufferRow(8),
            ),
            (
                DiffHunkStatus::Added,
                MultiBufferRow(10)..MultiBufferRow(11),
            ),
        ];

        assert_eq!(
            snapshot
                .git_diff_hunks_in_range(MultiBufferRow(0)..MultiBufferRow(12))
                .map(|hunk| (hunk_status(&hunk), hunk.row_range))
                .collect::<Vec<_>>(),
            &expected,
        );

        assert_eq!(
            snapshot
                .git_diff_hunks_in_range_rev(MultiBufferRow(0)..MultiBufferRow(12))
                .map(|hunk| (hunk_status(&hunk), hunk.row_range))
                .collect::<Vec<_>>(),
            expected
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .as_slice(),
        );
    }
}
