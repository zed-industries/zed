use std::ops::Range;

use git::diff::{DiffHunk, DiffHunkStatus};
use language::Point;

use crate::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    AnchorRangeExt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayDiffHunk {
    Folded {
        display_row: u32,
    },

    Unfolded {
        display_row_range: Range<u32>,
        status: DiffHunkStatus,
    },
}

impl DisplayDiffHunk {
    pub fn start_display_row(&self) -> u32 {
        match self {
            &DisplayDiffHunk::Folded { display_row } => display_row,
            DisplayDiffHunk::Unfolded {
                display_row_range, ..
            } => display_row_range.start,
        }
    }

    pub fn contains_display_row(&self, display_row: u32) -> bool {
        let range = match self {
            &DisplayDiffHunk::Folded { display_row } => display_row..=display_row,

            DisplayDiffHunk::Unfolded {
                display_row_range, ..
            } => display_row_range.start..=display_row_range.end - 1,
        };

        range.contains(&display_row)
    }
}

pub fn diff_hunk_to_display(hunk: DiffHunk<u32>, snapshot: &DisplaySnapshot) -> DisplayDiffHunk {
    let hunk_start_point = Point::new(hunk.buffer_range.start, 0);
    let hunk_start_point_sub = Point::new(hunk.buffer_range.start.saturating_sub(1), 0);
    let hunk_end_point_sub = Point::new(
        hunk.buffer_range
            .end
            .saturating_sub(1)
            .max(hunk.buffer_range.start),
        0,
    );

    let is_removal = hunk.status() == DiffHunkStatus::Removed;

    let folds_start = Point::new(hunk.buffer_range.start.saturating_sub(2), 0);
    let folds_end = Point::new(hunk.buffer_range.end + 2, 0);
    let folds_range = folds_start..folds_end;

    let containing_fold = snapshot.folds_in_range(folds_range).find(|fold_range| {
        let fold_point_range = fold_range.to_point(&snapshot.buffer_snapshot);
        let fold_point_range = fold_point_range.start..=fold_point_range.end;

        let folded_start = fold_point_range.contains(&hunk_start_point);
        let folded_end = fold_point_range.contains(&hunk_end_point_sub);
        let folded_start_sub = fold_point_range.contains(&hunk_start_point_sub);

        (folded_start && folded_end) || (is_removal && folded_start_sub)
    });

    if let Some(fold) = containing_fold {
        let row = fold.start.to_display_point(snapshot).row();
        DisplayDiffHunk::Folded { display_row: row }
    } else {
        let start = hunk_start_point.to_display_point(snapshot).row();

        let hunk_end_row_inclusive = hunk.buffer_range.end.max(hunk.buffer_range.start);
        let hunk_end_point = Point::new(hunk_end_row_inclusive, 0);
        let end = hunk_end_point.to_display_point(snapshot).row();

        DisplayDiffHunk::Unfolded {
            display_row_range: start..end,
            status: hunk.status(),
        }
    }
}
