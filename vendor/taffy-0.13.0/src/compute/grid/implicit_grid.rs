//! This module is not required for spec compliance, but is used as a performance optimisation
//! to reduce the number of allocations required when creating a grid.
use crate::geometry::Line;
use crate::style::{GenericGridPlacement, GridPlacement};
use crate::{CheapCloneStr, Direction, GridItemStyle};
use core::cmp::{max, min};

use super::types::TrackCounts;
use super::{OriginZeroLine, MAX_OZ_LINE, MIN_OZ_LINE};

/// Estimate the number of rows and columns in the grid
/// This is used as a performance optimisation to pre-size vectors and reduce allocations. It also forms a necessary step
/// in the auto-placement
///   - The estimates for the explicit and negative implicit track counts are exact.
///   - However, the estimates for the positive explicit track count is a lower bound as auto-placement can affect this
///     in ways which are impossible to predict until the auto-placement algorithm is run.
///
/// Note that this function internally mixes use of grid track numbers and grid line numbers
pub(crate) fn compute_grid_size_estimate<'a, S: GridItemStyle + 'a>(
    explicit_col_count: u16,
    explicit_row_count: u16,
    direction: Direction,
    child_styles_iter: impl Iterator<Item = S>,
) -> (TrackCounts, TrackCounts) {
    // Iterate over children, producing an estimate of the min and max grid lines (in origin-zero coordinates where)
    // along with the span of each item
    let (col_min, col_max, col_max_span, row_min, row_max, row_max_span) =
        get_known_child_positions(child_styles_iter, explicit_col_count, explicit_row_count, direction);

    // Compute *track* count estimates for each axis from:
    //   - The explicit track counts
    //   - The origin-zero coordinate min and max grid line variables
    let negative_implicit_inline_tracks = col_min.implied_negative_implicit_tracks();
    let explicit_inline_tracks = explicit_col_count;
    let mut positive_implicit_inline_tracks = col_max.implied_positive_implicit_tracks(explicit_col_count);
    let negative_implicit_block_tracks = row_min.implied_negative_implicit_tracks();
    let explicit_block_tracks = explicit_row_count;
    let mut positive_implicit_block_tracks = row_max.implied_positive_implicit_tracks(explicit_row_count);

    // In each axis, adjust positive track estimate if any items have a span that does not fit within
    // the total number of tracks in the estimate
    let tot_inline_tracks = negative_implicit_inline_tracks + explicit_inline_tracks + positive_implicit_inline_tracks;
    if tot_inline_tracks < col_max_span {
        positive_implicit_inline_tracks = col_max_span - explicit_inline_tracks - negative_implicit_inline_tracks;
    }

    let tot_block_tracks = negative_implicit_block_tracks + explicit_block_tracks + positive_implicit_block_tracks;
    if tot_block_tracks < row_max_span {
        positive_implicit_block_tracks = row_max_span - explicit_block_tracks - negative_implicit_block_tracks;
    }

    let column_counts =
        TrackCounts::from_raw(negative_implicit_inline_tracks, explicit_inline_tracks, positive_implicit_inline_tracks);

    let row_counts =
        TrackCounts::from_raw(negative_implicit_block_tracks, explicit_block_tracks, positive_implicit_block_tracks);

    (column_counts, row_counts)
}

/// Iterate over children, producing an estimate of the min and max grid *lines* along with the span of each item
///
/// Min and max grid lines are returned in origin-zero coordinates)
/// The span is measured in tracks spanned
fn get_known_child_positions<'a, S: GridItemStyle + 'a>(
    children_iter: impl Iterator<Item = S>,
    explicit_col_count: u16,
    explicit_row_count: u16,
    direction: Direction,
) -> (OriginZeroLine, OriginZeroLine, u16, OriginZeroLine, OriginZeroLine, u16) {
    let (mut col_min, mut col_max, mut col_max_span) = (OriginZeroLine(0), OriginZeroLine(0), 0);
    let (mut row_min, mut row_max, mut row_max_span) = (OriginZeroLine(0), OriginZeroLine(0), 0);
    children_iter.for_each(|child_style| {
        let col_line = child_style.grid_column();
        let row_line = child_style.grid_row();

        // Note: that the children reference the lines in between (and around) the tracks not tracks themselves,
        // and thus we must subtract 1 to get an accurate estimate of the number of tracks
        let (mut child_col_min, mut child_col_max, child_col_span) =
            child_min_line_max_line_span::<S::CustomIdent>(col_line, explicit_col_count);
        let (child_row_min, child_row_max, child_row_span) =
            child_min_line_max_line_span::<S::CustomIdent>(row_line, explicit_row_count);

        // Placement mirrors horizontal spans in RTL, so mirror known column line bounds here
        // to keep implicit-grid pre-sizing consistent with actual placement.
        if direction.is_rtl() && (child_col_min != OriginZeroLine(0) || child_col_max != OriginZeroLine(0)) {
            let explicit_col_end_line = explicit_col_count as i16;
            let mirrored_min = OriginZeroLine(explicit_col_end_line - child_col_max.0);
            let mirrored_max = OriginZeroLine(explicit_col_end_line - child_col_min.0);
            child_col_min = mirrored_min;
            child_col_max = mirrored_max;
        }

        col_min = min(col_min, child_col_min);
        col_max = max(col_max, child_col_max);
        col_max_span = max(col_max_span, child_col_span);
        row_min = min(row_min, child_row_min);
        row_max = max(row_max, child_row_max);
        row_max_span = max(row_max_span, child_row_span);
    });

    (col_min, col_max, col_max_span, row_min, row_max, row_max_span)
}

/// Helper function for `compute_grid_size_estimate`
/// Produces a conservative estimate of the greatest and smallest grid lines used by a single grid item
///
/// Values are returned in origin-zero coordinates
#[inline]
fn child_min_line_max_line_span<S: CheapCloneStr>(
    line: Line<GridPlacement<S>>,
    explicit_track_count: u16,
) -> (OriginZeroLine, OriginZeroLine, u16) {
    use GenericGridPlacement::*;

    // 8.3.1. Grid Placement Conflict Handling
    // A. If the placement for a grid item contains two lines, and the start line is further end-ward than the end line, swap the two lines.
    // B. If the start line is equal to the end line, remove the end line.
    // C. If the placement contains two spans, remove the one contributed by the end grid-placement property.
    // D. If the placement contains only a span for a named line, replace it with a span of 1.

    // Convert line into origin-zero coordinates before attempting to analyze
    // We ignore named lines here as they are accounted for separately
    let oz_line = line.into_origin_zero_ignoring_named(explicit_track_count);

    let min = match (oz_line.start, oz_line.end) {
        // Both tracks specified
        (Line(track1), Line(track2)) => {
            // See rules A and B above
            if track1 == track2 {
                track1
            } else {
                min(track1, track2)
            }
        }

        // Start track specified
        (Line(track), Auto) => track,
        (Line(track), Span(_)) => track,

        // End track specified
        (Auto, Line(track)) => track,
        (Span(span), Line(track)) => track - span,

        // Only spans or autos
        // We ignore spans here by returning 0 which never effect the estimate as these are accounted for separately
        (Auto | Span(_), Auto | Span(_)) => OriginZeroLine(0),
    };

    let max = match (oz_line.start, oz_line.end) {
        // Both tracks specified
        (Line(track1), Line(track2)) => {
            // See rules A and B above
            if track1 == track2 {
                track1 + 1
            } else {
                max(track1, track2)
            }
        }

        // Start track specified
        (Line(track), Auto) => track + 1,
        (Line(track), Span(span)) => track + span,

        // End track specified
        (Auto, Line(track)) => track,
        (Span(_), Line(track)) => track,

        // Only spans or autos
        // We ignore spans here by returning 0 which never effect the estimate as these are accounted for separately
        (Auto | Span(_), Auto | Span(_)) => OriginZeroLine(0),
    };

    // Calculate span only for indefinitely placed items as we don't need for other items (whose required space will
    // be taken into account by min and max)
    let span = match (oz_line.start, oz_line.end) {
        (Auto | Span(_), Auto | Span(_)) => oz_line.indefinite_span(),
        _ => 1,
    };

    // Clamp the min and max lines into the limited grid so that the estimated implicit track counts
    // stay within the maximum track limit (https://www.w3.org/TR/css-grid-1/#overlarge-grids).
    // This matches the clamping of the actual item placements performed during placement.
    let clamped_min = OriginZeroLine(min.0.max(MIN_OZ_LINE));
    let clamped_max = OriginZeroLine(max.0.min(MAX_OZ_LINE));

    (clamped_min, clamped_max, span)
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    mod test_child_min_max_line {
        type S = String;
        use super::super::child_min_line_max_line_span;
        use super::super::OriginZeroLine;
        use crate::geometry::Line;
        use crate::style_helpers::*;

        #[test]
        fn child_min_max_line_auto() {
            let (min_col, max_col, span) = child_min_line_max_line_span::<S>(Line { start: line(5), end: span(6) }, 6);
            assert_eq!(min_col, OriginZeroLine(4));
            assert_eq!(max_col, OriginZeroLine(10));
            assert_eq!(span, 1);
        }

        #[test]
        fn child_min_max_line_negative_track() {
            let (min_col, max_col, span) = child_min_line_max_line_span::<S>(Line { start: line(-5), end: span(3) }, 6);
            assert_eq!(min_col, OriginZeroLine(2));
            assert_eq!(max_col, OriginZeroLine(5));
            assert_eq!(span, 1);
        }
    }

    mod test_initial_grid_sizing {
        use super::super::compute_grid_size_estimate;
        use crate::compute::grid::util::test_helpers::*;
        use crate::style_helpers::*;
        use crate::Direction;

        #[test]
        fn explicit_grid_sizing_with_children() {
            let explicit_col_count = 6;
            let explicit_row_count = 8;
            let child_styles = [
                (line(1), span(2), line(2), auto()).into_grid_child(),
                (line(-4), auto(), line(-2), auto()).into_grid_child(),
            ];
            let (inline, block) =
                compute_grid_size_estimate(explicit_col_count, explicit_row_count, Direction::Ltr, child_styles.iter());
            assert_eq!(inline.negative_implicit, 0);
            assert_eq!(inline.explicit, explicit_col_count);
            assert_eq!(inline.positive_implicit, 0);
            assert_eq!(block.negative_implicit, 0);
            assert_eq!(block.explicit, explicit_row_count);
            assert_eq!(block.positive_implicit, 0);
        }

        #[test]
        fn negative_implicit_grid_sizing() {
            let explicit_col_count = 4;
            let explicit_row_count = 4;
            let child_styles = [
                (line(-6), span(2), line(-8), auto()).into_grid_child(),
                (line(4), auto(), line(3), auto()).into_grid_child(),
            ];
            let (inline, block) =
                compute_grid_size_estimate(explicit_col_count, explicit_row_count, Direction::Ltr, child_styles.iter());
            assert_eq!(inline.negative_implicit, 1);
            assert_eq!(inline.explicit, explicit_col_count);
            assert_eq!(inline.positive_implicit, 0);
            assert_eq!(block.negative_implicit, 3);
            assert_eq!(block.explicit, explicit_row_count);
            assert_eq!(block.positive_implicit, 0);
        }
    }
}
