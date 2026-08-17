//! Contains CellOccupancyMatrix used to track occupied cells during grid placement
use super::TrackCounts;
use crate::compute::grid::OriginZeroLine;
use crate::geometry::AbsoluteAxis;
use crate::geometry::Line;
use crate::util::sys::{new_vec_with_capacity, Vec};
use core::cmp::{max, min};
use core::fmt::Debug;
use core::ops::Range;
use smallvec::SmallVec;

/// The occupancy state of a single grid cell
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub(crate) enum CellOccupancyState {
    #[default]
    /// Indicates that a grid cell is unoccupied
    Unoccupied,
    /// Indicates that a grid cell is occupied by a definitely placed item
    DefinitelyPlaced,
    /// Indicates that a grid cell is occupied by an item that was placed by the auto placement algorithm
    AutoPlaced,
}

/// A run of occupied cells within a single track. The range is in OriginZero line coordinates
/// along the track (so for a row track, the range spans column lines and vice versa).
#[derive(Debug, Clone, PartialEq, Eq)]
struct OccupiedInterval {
    /// The range of cells covered by this interval (start line..end line in OriginZero coordinates)
    range: Range<i16>,
    /// The occupancy state of every cell within this interval
    state: CellOccupancyState,
}

impl OccupiedInterval {
    /// Whether this interval overlaps the given range
    fn overlaps(&self, range: &Range<i16>) -> bool {
        self.range.start < range.end && self.range.end > range.start
    }
}

/// The occupied cells of a single track, stored as a sorted list of disjoint intervals in
/// OriginZero line coordinates. Gaps between intervals are unoccupied. Touching intervals
/// with the same state are merged.
#[derive(Debug, Clone, Default)]
struct TrackIntervals {
    /// The sorted, disjoint list of occupied intervals within the track
    intervals: SmallVec<[OccupiedInterval; 2]>,
}

impl TrackIntervals {
    /// Whether the track contains any occupied cells
    fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// The occupancy state of the cell whose start line is at `coordinate`
    fn state_at(&self, coordinate: i16) -> CellOccupancyState {
        self.intervals
            .iter()
            .find(|interval| interval.range.start <= coordinate && coordinate < interval.range.end)
            .map(|interval| interval.state)
            .unwrap_or(CellOccupancyState::Unoccupied)
    }

    /// Set the cells covered by `range` to `state`, overwriting the state of any already-occupied
    /// cells within the range (matching the overwrite semantics of a dense matrix of cells)
    fn paint(&mut self, range: Range<i16>, state: CellOccupancyState) {
        if range.start >= range.end {
            return;
        }

        // Fast path: the painted range lies entirely after all existing intervals
        // (the common case, as placement queries and inserts mostly advance forwards)
        match self.intervals.last_mut() {
            None => {
                self.intervals.push(OccupiedInterval { range, state });
                return;
            }
            Some(last) if last.range.end <= range.start => {
                if last.state == state && last.range.end == range.start {
                    last.range.end = range.end;
                } else {
                    self.intervals.push(OccupiedInterval { range, state });
                }
                return;
            }
            _ => {}
        }

        let mut result: SmallVec<[OccupiedInterval; 2]> = SmallVec::new();

        // Intervals (or partial intervals) entirely before the painted range
        for interval in &self.intervals {
            if interval.range.end <= range.start {
                result.push(interval.clone());
            } else if interval.range.start < range.start {
                result.push(OccupiedInterval { range: interval.range.start..range.start, state: interval.state });
            }
        }

        // The painted range, merged with the preceding interval if it touches and has the same state
        match result.last_mut() {
            Some(last) if last.state == state && last.range.end == range.start => last.range.end = range.end,
            _ => result.push(OccupiedInterval { range: range.clone(), state }),
        }

        // Intervals (or partial intervals) entirely after the painted range
        for interval in &self.intervals {
            let trimmed = if interval.range.start >= range.end {
                interval.clone()
            } else if interval.range.end > range.end {
                OccupiedInterval { range: range.end..interval.range.end, state: interval.state }
            } else {
                continue;
            };
            match result.last_mut() {
                Some(last) if last.state == trimmed.state && last.range.end == trimmed.range.start => {
                    last.range.end = trimmed.range.end
                }
                _ => result.push(trimmed),
            }
        }

        self.intervals = result;
    }

    /// Find the extent of the occupied interval which an auto-placement search along the track
    /// would collide with last: the end of the last overlapping interval when searching forwards,
    /// or the start of the first overlapping interval when searching backwards (`reversed ==
    /// true`). Returns the extremal occupied cell of that interval (which may lie outside
    /// `range`: every search position before the returned extent also collides with the
    /// interval), or `None` if the range is entirely unoccupied.
    fn collision_extent(&self, range: &Range<i16>, reversed: bool) -> Option<i16> {
        if reversed {
            let interval = self.intervals.iter().find(|interval| interval.overlaps(range))?;
            Some(interval.range.start)
        } else {
            let interval = self.intervals.iter().rev().find(|interval| interval.overlaps(range))?;
            Some(interval.range.end - 1)
        }
    }

    /// The start line of the first (lowest coordinate) cell with the specified state, if any
    fn first_of_state(&self, state: CellOccupancyState) -> Option<i16> {
        self.intervals.iter().find(|interval| interval.state == state).map(|interval| interval.range.start)
    }

    /// The start line of the last (highest coordinate) cell with the specified state, if any
    fn last_of_state(&self, state: CellOccupancyState) -> Option<i16> {
        self.intervals.iter().rev().find(|interval| interval.state == state).map(|interval| interval.range.end - 1)
    }
}

/// A dynamically sized matrix (2d grid) which tracks the occupancy of each grid cell during auto-placement.
/// It also keeps tabs on how many tracks there are and which tracks are implicit and which are explicit.
///
/// Occupancy is stored sparsely as per-track interval lists (in both orientations), so memory usage
/// is proportional to the number of placed items rather than the total number of grid cells.
pub(crate) struct CellOccupancyMatrix {
    /// The counts of implicit and explicit columns
    columns: TrackCounts,
    /// The counts of implicit and explicit rows
    rows: TrackCounts,
    /// For each row track: the occupied intervals within that row (in column coordinates)
    row_intervals: Vec<TrackIntervals>,
    /// For each column track: the occupied intervals within that column (in row coordinates)
    column_intervals: Vec<TrackIntervals>,
}

/// Debug impl that represents the matrix in a compact 2d text format
impl Debug for CellOccupancyMatrix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "Rows: neg_implicit={} explicit={} pos_implicit={}",
            self.rows.negative_implicit, self.rows.explicit, self.rows.positive_implicit
        )?;
        writeln!(
            f,
            "Cols: neg_implicit={} explicit={} pos_implicit={}",
            self.columns.negative_implicit, self.columns.explicit, self.columns.positive_implicit
        )?;
        if self.rows.len() > 100 || self.columns.len() > 100 {
            writeln!(f, "State: (not printed: more than 100 tracks)")?;
            return Ok(());
        }
        writeln!(f, "State:")?;

        for row in &self.row_intervals {
            for column_index in 0..self.columns.len() {
                let coordinate = self.columns.track_to_prev_oz_line(column_index as u16);
                let letter = match row.state_at(coordinate.0) {
                    CellOccupancyState::Unoccupied => '_',
                    CellOccupancyState::DefinitelyPlaced => 'D',
                    CellOccupancyState::AutoPlaced => 'A',
                };
                write!(f, "{letter}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl CellOccupancyMatrix {
    /// Create a CellOccupancyMatrix given a set of provisional track counts. The grid can expand as needed to fit more tracks,
    /// the provisional track counts represent a best effort attempt to avoid the extra allocations this requires.
    pub fn with_track_counts(columns: TrackCounts, rows: TrackCounts) -> Self {
        let mut row_intervals = new_vec_with_capacity(rows.len());
        row_intervals.resize(rows.len(), TrackIntervals::default());
        let mut column_intervals = new_vec_with_capacity(columns.len());
        column_intervals.resize(columns.len(), TrackIntervals::default());
        Self { rows, columns, row_intervals, column_intervals }
    }

    /// The per-track interval lists for tracks in the specified axis. Each row track's intervals
    /// are in column coordinates and vice versa.
    fn track_lists(&self, track_axis: AbsoluteAxis) -> &[TrackIntervals] {
        match track_axis {
            AbsoluteAxis::Horizontal => &self.column_intervals,
            AbsoluteAxis::Vertical => &self.row_intervals,
        }
    }

    /// Expands the grid (potentially in all 4 directions) in order to ensure that the specified
    /// spans (in OriginZero coordinates) fit within the tracked tracks
    fn expand_to_fit_range(&mut self, row_span: Line<OriginZeroLine>, col_span: Line<OriginZeroLine>) {
        // Calculate number of rows and columns missing to accommodate ranges (if any)
        let req_negative_rows = max(-(self.rows.negative_implicit as i16) - row_span.start.0, 0);
        let req_positive_rows = max(row_span.end.0 - self.rows.implicit_end_line().0, 0);
        let req_negative_cols = max(-(self.columns.negative_implicit as i16) - col_span.start.0, 0);
        let req_positive_cols = max(col_span.end.0 - self.columns.implicit_end_line().0, 0);

        // Add empty tracks to the front and/or back of the per-track interval lists.
        // Interval contents are stored in OriginZero coordinates, so they do not need to shift.
        if req_negative_rows > 0 {
            self.row_intervals
                .splice(0..0, core::iter::repeat_with(TrackIntervals::default).take(req_negative_rows as usize));
        }
        if req_positive_rows > 0 {
            let new_len = self.row_intervals.len() + req_positive_rows as usize;
            self.row_intervals.resize(new_len, TrackIntervals::default());
        }
        if req_negative_cols > 0 {
            self.column_intervals
                .splice(0..0, core::iter::repeat_with(TrackIntervals::default).take(req_negative_cols as usize));
        }
        if req_positive_cols > 0 {
            let new_len = self.column_intervals.len() + req_positive_cols as usize;
            self.column_intervals.resize(new_len, TrackIntervals::default());
        }

        self.rows.negative_implicit += req_negative_rows as u16;
        self.rows.positive_implicit += req_positive_rows as u16;
        self.columns.negative_implicit += req_negative_cols as u16;
        self.columns.positive_implicit += req_positive_cols as u16;
    }

    /// Mark an area of the matrix as occupied, expanding the allocated space as necessary to accommodate the passed area.
    pub fn mark_area_as(
        &mut self,
        primary_axis: AbsoluteAxis,
        primary_span: Line<OriginZeroLine>,
        secondary_span: Line<OriginZeroLine>,
        value: CellOccupancyState,
    ) {
        let (row_span, column_span) = match primary_axis {
            AbsoluteAxis::Horizontal => (secondary_span, primary_span),
            AbsoluteAxis::Vertical => (primary_span, secondary_span),
        };

        self.expand_to_fit_range(row_span, column_span);

        let row_range = self.rows.oz_line_range_to_track_range(row_span);
        let col_range = self.columns.oz_line_range_to_track_range(column_span);
        for row_index in row_range {
            self.row_intervals[row_index as usize].paint(column_span.start.0..column_span.end.0, value);
        }
        for column_index in col_range {
            self.column_intervals[column_index as usize].paint(row_span.start.0..row_span.end.0, value);
        }
    }

    /// Determines whether a grid area specified by the bounding grid lines in OriginZero coordinates
    /// is entirely unnocupied. Returns true if all grid cells within the grid area are unnocupied, else false.
    #[cfg(test)]
    pub fn line_area_is_unoccupied(
        &self,
        primary_axis: AbsoluteAxis,
        primary_span: Line<OriginZeroLine>,
        secondary_span: Line<OriginZeroLine>,
    ) -> bool {
        self.line_area_collision_jump(primary_axis, primary_span, secondary_span, false).is_none()
    }

    /// Checks the specified area for occupied cells (`primary_span` and `secondary_span` are
    /// bounding grid lines in OriginZero coordinates). Returns `None` if the area is entirely
    /// unoccupied. Otherwise returns the next search position (in OriginZero coordinates, along
    /// `primary_axis`) that is not guaranteed to collide with the occupied cells found in the
    /// area. This allows the auto-placement search cursor to jump past collisions rather than
    /// advancing one track at a time.
    pub fn line_area_collision_jump(
        &self,
        primary_axis: AbsoluteAxis,
        primary_span: Line<OriginZeroLine>,
        secondary_span: Line<OriginZeroLine>,
        reversed: bool,
    ) -> Option<OriginZeroLine> {
        let track_lists = self.track_lists(primary_axis.other_axis());
        let secondary_counts = self.track_counts(primary_axis.other_axis());
        let secondary_range = secondary_counts.oz_line_range_to_track_range(secondary_span);

        // Out of bounds cells are considered unoccupied, so clamp the secondary range to the
        // tracks which actually exist
        let secondary_start = max(secondary_range.start, 0);
        let secondary_end = min(secondary_range.end, track_lists.len() as i16);

        let primary_range = primary_span.start.0..primary_span.end.0;

        let mut extent: Option<i16> = None;
        for secondary_index in secondary_start..secondary_end {
            let Some(cell) = track_lists[secondary_index as usize].collision_extent(&primary_range, reversed) else {
                continue;
            };
            extent = Some(match extent {
                None => cell,
                Some(best) => {
                    if reversed {
                        min(best, cell)
                    } else {
                        max(best, cell)
                    }
                }
            });
        }

        extent.map(|cell| if reversed { OriginZeroLine(cell - 1) } else { OriginZeroLine(cell + 1) })
    }

    /// Given a span of tracks in `axis` (in OriginZero coordinates), returns the next search
    /// position past all non-empty tracks within the span, or `None` if all tracks within the
    /// span are entirely unoccupied. Used to place items which span every track in the other
    /// axis (such items can only fit in a stripe of entirely unoccupied tracks).
    pub fn occupied_track_jump(
        &self,
        axis: AbsoluteAxis,
        span: Line<OriginZeroLine>,
        reversed: bool,
    ) -> Option<OriginZeroLine> {
        let counts = self.track_counts(axis);
        let track_lists = self.track_lists(axis);
        let range = counts.oz_line_range_to_track_range(span);
        let start = max(range.start, 0);
        let end = min(range.end, track_lists.len() as i16);
        let found = if !reversed {
            (start..end).rev().find(|&index| !track_lists[index as usize].is_empty())
        } else {
            (start..end).find(|&index| !track_lists[index as usize].is_empty())
        };
        found.map(|track_index| {
            let line = counts.track_to_prev_oz_line(track_index as u16);
            if reversed {
                OriginZeroLine(line.0 - 1)
            } else {
                line + 1
            }
        })
    }

    /// Determines whether the specified row contains any items
    pub fn row_is_occupied(&self, row_index: usize) -> bool {
        self.track_lists(AbsoluteAxis::Vertical).get(row_index).is_some_and(|track| !track.is_empty())
    }

    /// Determines whether the specified column contains any items
    pub fn column_is_occupied(&self, column_index: usize) -> bool {
        self.track_lists(AbsoluteAxis::Horizontal).get(column_index).is_some_and(|track| !track.is_empty())
    }

    /// Returns the track counts of this CellOccunpancyMatrix in the relevant axis
    pub fn track_counts(&self, track_type: AbsoluteAxis) -> &TrackCounts {
        match track_type {
            AbsoluteAxis::Horizontal => &self.columns,
            AbsoluteAxis::Vertical => &self.rows,
        }
    }

    /// Given an axis and a track index
    /// Search backwards from the end of the track and find the last grid cell matching the specified state (if any)
    /// Return the index of that cell or None.
    pub fn last_of_type(
        &self,
        track_type: AbsoluteAxis,
        start_at: OriginZeroLine,
        kind: CellOccupancyState,
    ) -> Option<OriginZeroLine> {
        let track_counts = self.track_counts(track_type.other_axis());
        let track_computed_index = track_counts.oz_line_to_next_track(start_at);
        let track_lists = self.track_lists(track_type.other_axis());
        if track_computed_index < 0 || track_computed_index >= track_lists.len() as i16 {
            // Index out of bounds: no tracks to search
            return None;
        }
        track_lists[track_computed_index as usize].last_of_state(kind).map(OriginZeroLine)
    }

    /// Given an axis and a track index
    /// Search forwards from the start of the track and find the first grid cell matching the specified state (if any)
    /// Return the index of that cell or None.
    pub fn first_of_type(
        &self,
        track_type: AbsoluteAxis,
        start_at: OriginZeroLine,
        kind: CellOccupancyState,
    ) -> Option<OriginZeroLine> {
        let track_counts = self.track_counts(track_type.other_axis());
        let track_computed_index = track_counts.oz_line_to_next_track(start_at);
        let track_lists = self.track_lists(track_type.other_axis());
        if track_computed_index < 0 || track_computed_index >= track_lists.len() as i16 {
            // Index out of bounds: no tracks to search
            return None;
        }
        track_lists[track_computed_index as usize].first_of_state(kind).map(OriginZeroLine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interval(range: Range<i16>, state: CellOccupancyState) -> OccupiedInterval {
        OccupiedInterval { range, state }
    }

    mod track_intervals {
        use super::*;
        use CellOccupancyState::{AutoPlaced, DefinitelyPlaced};

        #[test]
        fn paint_merges_touching_same_state_intervals() {
            let mut track = TrackIntervals::default();
            track.paint(0..2, AutoPlaced);
            track.paint(4..6, AutoPlaced);
            track.paint(2..4, AutoPlaced);
            assert_eq!(track.intervals.as_slice(), &[interval(0..6, AutoPlaced)]);
        }

        #[test]
        fn paint_does_not_merge_different_states() {
            let mut track = TrackIntervals::default();
            track.paint(0..2, AutoPlaced);
            track.paint(2..4, DefinitelyPlaced);
            assert_eq!(track.intervals.as_slice(), &[interval(0..2, AutoPlaced), interval(2..4, DefinitelyPlaced)]);
        }

        #[test]
        fn paint_overwrites_overlapped_cells() {
            let mut track = TrackIntervals::default();
            track.paint(0..6, AutoPlaced);
            track.paint(2..4, DefinitelyPlaced);
            assert_eq!(
                track.intervals.as_slice(),
                &[interval(0..2, AutoPlaced), interval(2..4, DefinitelyPlaced), interval(4..6, AutoPlaced)]
            );

            // Painting over everything replaces all intervals
            track.paint(-1..7, AutoPlaced);
            assert_eq!(track.intervals.as_slice(), &[interval(-1..7, AutoPlaced)]);
        }

        #[test]
        fn paint_overwrites_multiple_intervals() {
            let mut track = TrackIntervals::default();
            track.paint(0..2, AutoPlaced);
            track.paint(3..5, DefinitelyPlaced);
            track.paint(6..8, AutoPlaced);
            track.paint(1..7, DefinitelyPlaced);
            assert_eq!(
                track.intervals.as_slice(),
                &[interval(0..1, AutoPlaced), interval(1..7, DefinitelyPlaced), interval(7..8, AutoPlaced)]
            );
        }

        #[test]
        fn collision_extent_finds_extremal_occupied_cell() {
            let mut track = TrackIntervals::default();
            track.paint(2..4, AutoPlaced);
            track.paint(6..8, DefinitelyPlaced);
            // Forward search: the last cell of the last overlapping interval
            assert_eq!(track.collision_extent(&(0..10), false), Some(7));
            assert_eq!(track.collision_extent(&(0..7), false), Some(7));
            assert_eq!(track.collision_extent(&(0..6), false), Some(3));
            assert_eq!(track.collision_extent(&(4..6), false), None);
            // Reverse search: the first cell of the first overlapping interval
            assert_eq!(track.collision_extent(&(0..10), true), Some(2));
            assert_eq!(track.collision_extent(&(3..10), true), Some(2));
            assert_eq!(track.collision_extent(&(4..10), true), Some(6));
        }

        #[test]
        fn first_and_last_of_state_ignore_other_states() {
            let mut track = TrackIntervals::default();
            track.paint(0..2, DefinitelyPlaced);
            track.paint(2..4, AutoPlaced);
            track.paint(6..8, AutoPlaced);
            track.paint(8..9, DefinitelyPlaced);
            assert_eq!(track.first_of_state(AutoPlaced), Some(2));
            assert_eq!(track.last_of_state(AutoPlaced), Some(7));
            assert_eq!(track.first_of_state(DefinitelyPlaced), Some(0));
            assert_eq!(track.last_of_state(DefinitelyPlaced), Some(8));
        }

        #[test]
        fn definitely_placed_overwrite_hides_auto_placed_cells() {
            let mut track = TrackIntervals::default();
            track.paint(0..4, CellOccupancyState::AutoPlaced);
            track.paint(2..4, CellOccupancyState::DefinitelyPlaced);
            assert_eq!(track.last_of_state(CellOccupancyState::AutoPlaced), Some(1));
        }
    }

    mod cell_occupancy_matrix {
        use super::*;
        use crate::geometry::AbsoluteAxis::{Horizontal, Vertical};
        use CellOccupancyState::AutoPlaced;

        fn line(start: i16, end: i16) -> Line<OriginZeroLine> {
            Line { start: OriginZeroLine(start), end: OriginZeroLine(end) }
        }

        #[test]
        fn negative_expansion_preserves_occupancy() {
            let mut matrix =
                CellOccupancyMatrix::with_track_counts(TrackCounts::from_raw(0, 2, 0), TrackCounts::from_raw(0, 2, 0));
            matrix.mark_area_as(Horizontal, line(0, 1), line(0, 1), AutoPlaced);
            // Expand by marking an area in negative tracks
            matrix.mark_area_as(Horizontal, line(-2, -1), line(-1, 0), AutoPlaced);

            assert_eq!(*matrix.track_counts(Horizontal), TrackCounts::from_raw(2, 2, 0));
            assert_eq!(*matrix.track_counts(Vertical), TrackCounts::from_raw(1, 2, 0));

            // Original cell still occupied at the same OriginZero coordinates
            assert!(!matrix.line_area_is_unoccupied(Horizontal, line(0, 1), line(0, 1)));
            assert!(!matrix.line_area_is_unoccupied(Horizontal, line(-2, -1), line(-1, 0)));
            assert!(matrix.line_area_is_unoccupied(Horizontal, line(-1, 0), line(0, 1)));

            // Matrix-index based queries account for the shifted origin
            assert!(matrix.column_is_occupied(0)); // OriginZero column -2
            assert!(!matrix.column_is_occupied(1)); // OriginZero column -1
            assert!(matrix.column_is_occupied(2)); // OriginZero column 0
            assert!(matrix.row_is_occupied(0)); // OriginZero row -1
            assert!(matrix.row_is_occupied(1)); // OriginZero row 0
            assert!(!matrix.row_is_occupied(2)); // OriginZero row 1
        }

        #[test]
        fn collision_jump_returns_next_search_position() {
            let mut matrix =
                CellOccupancyMatrix::with_track_counts(TrackCounts::from_raw(0, 4, 0), TrackCounts::from_raw(0, 4, 0));
            matrix.mark_area_as(Horizontal, line(1, 3), line(0, 1), AutoPlaced);

            // Forwards: jump past the end of the last colliding interval
            assert_eq!(
                matrix.line_area_collision_jump(Horizontal, line(0, 2), line(0, 1), false),
                Some(OriginZeroLine(3))
            );
            assert_eq!(
                matrix.line_area_collision_jump(Horizontal, line(0, 4), line(0, 1), false),
                Some(OriginZeroLine(3))
            );
            // Backwards: jump past the start of the first colliding interval
            assert_eq!(
                matrix.line_area_collision_jump(Horizontal, line(2, 4), line(0, 1), true),
                Some(OriginZeroLine(0))
            );
            assert_eq!(
                matrix.line_area_collision_jump(Horizontal, line(0, 4), line(0, 1), true),
                Some(OriginZeroLine(0))
            );
            // No collision in a different row
            assert_eq!(matrix.line_area_collision_jump(Horizontal, line(0, 4), line(1, 2), false), None);
        }

        #[test]
        fn occupied_track_jump_skips_non_empty_tracks() {
            let mut matrix =
                CellOccupancyMatrix::with_track_counts(TrackCounts::from_raw(0, 4, 0), TrackCounts::from_raw(0, 4, 0));
            matrix.mark_area_as(Horizontal, line(0, 1), line(1, 2), AutoPlaced);

            // Vertical (row) tracks: row 1 is occupied
            assert_eq!(matrix.occupied_track_jump(Vertical, line(0, 4), false), Some(OriginZeroLine(2)));
            assert_eq!(matrix.occupied_track_jump(Vertical, line(0, 4), true), Some(OriginZeroLine(0)));
            assert_eq!(matrix.occupied_track_jump(Vertical, line(2, 4), false), None);
        }
    }
}
