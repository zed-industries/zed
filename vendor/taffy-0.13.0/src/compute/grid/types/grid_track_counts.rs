//! Contains TrackCounts used to keep track of the number of tracks in the explicit and implicit grids.
//! Also contains coordinate conversion functions which depend on those counts
//!
//! Taffy uses two coordinate systems to refer to grid lines (the gaps/gutters between rows/columns):
//!
//!   "CSS Grid Line" coordinates are those used in grid-row/grid-column in the CSS grid spec:
//!     - 0 is not a valid index
//!     - The line at left hand (or top) edge of the explicit grid is line 1
//!       (and counts up from there)
//!     - The line at the right hand (or bottom) edge of the explicit grid in -1
//!       (and counts down from there)
//!
//!   "OriginZero" coordinates are a normalized form:
//!     - The line at left hand (or top) edge of the explicit grid is line 0
//!     - The next line to the right (or down) is 1, and so on
//!     - The next line to the left (or up) is -1, and so on
//!
//! Taffy also uses two coordinate systems to refer to grid tracks (rows/columns):
//!
//!   Both of these systems represent the entire implicit grid, not just the explicit grid.
//!
//!   "CellOccupancyMatrix track indices":
//!       - These are indexes into the CellOccupancyMatrix
//!       - The CellOccupancyMatrix stores only tracks
//!       - 0 is the leftmost track of the implicit grid, and indexes count up there
//!
//!   "GridTrackVec track indices":
//!       - The GridTrackVecs store both lines and tracks, so:
//!           - even indices (0, 2, 4, etc) represent lines
//!           - odd indices (1, 3, 5, etc) represent tracks
//!           - These is always an odd number of
//!       - Index 1 is the leftmost track of the implicit grid. Index 3 is the second leftmost track, etc.
//!       - Index 0 is the leftmost grid line. Index 2 is the second leftmost line, etc.
//!
use crate::{compute::grid::OriginZeroLine, geometry::Line};
use core::ops::Range;

/// Stores the number of tracks in a given dimension.
/// Stores separately the number of tracks in the implicit and explicit grids
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub(crate) struct TrackCounts {
    /// The number of track in the implicit grid before the explicit grid
    pub negative_implicit: u16,
    /// The number of tracks in the explicit grid
    pub explicit: u16,
    /// The number of tracks in the implicit grid after the explicit grid
    pub positive_implicit: u16,
}

impl TrackCounts {
    /// Create a TrackCounts instance from raw track count numbers
    pub const fn from_raw(negative_implicit: u16, explicit: u16, positive_implicit: u16) -> Self {
        Self { negative_implicit, explicit, positive_implicit }
    }

    /// Count the total number of tracks in the axis
    pub const fn len(&self) -> usize {
        (self.negative_implicit + self.explicit + self.positive_implicit) as usize
    }

    /// The OriginZeroLine representing the start of the implicit grid
    pub const fn implicit_start_line(&self) -> OriginZeroLine {
        OriginZeroLine(-(self.negative_implicit as i16))
    }

    /// The OriginZeroLine representing the end of the implicit grid
    pub const fn implicit_end_line(&self) -> OriginZeroLine {
        OriginZeroLine((self.explicit + self.positive_implicit) as i16)
    }
}

/// Conversion functions between OriginZero coordinates and CellOccupancyMatrix track indexes
#[allow(dead_code)]
impl TrackCounts {
    /// Converts a grid line in OriginZero coordinates into the track immediately
    /// following that grid line as an index into the CellOccupancyMatrix.
    pub const fn oz_line_to_next_track(&self, index: OriginZeroLine) -> i16 {
        index.0 + (self.negative_implicit as i16)
    }

    /// Converts start and end grid lines in OriginZero coordinates into a range of tracks
    /// as indexes into the CellOccupancyMatrix
    pub const fn oz_line_range_to_track_range(&self, input: Line<OriginZeroLine>) -> Range<i16> {
        let start = self.oz_line_to_next_track(input.start);
        let end = self.oz_line_to_next_track(input.end); // Don't subtract 1 as output range is exclusive
        start..end
    }

    /// Converts a track as an index into the CellOccupancyMatrix into the grid line immediately
    /// preceding that track in OriginZero coordinates.
    pub const fn track_to_prev_oz_line(&self, index: u16) -> OriginZeroLine {
        OriginZeroLine((index as i16) - (self.negative_implicit as i16))
    }

    /// Converts a range of tracks as indexes into the CellOccupancyMatrix into
    /// start and end grid lines in OriginZero coordinates
    pub const fn track_range_to_oz_line_range(&self, input: Range<i16>) -> Line<OriginZeroLine> {
        let start = self.track_to_prev_oz_line(input.start as u16);
        let end = self.track_to_prev_oz_line(input.end as u16); // Don't add 1 as input range is exclusive
        Line { start, end }
    }
}
