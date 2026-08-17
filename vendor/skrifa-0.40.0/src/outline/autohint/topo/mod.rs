//! Topology analysis of segments and edges.

mod edges;
mod segments;

use super::{
    metrics::ScaledWidth,
    outline::{Direction, Orientation, Point},
};
use crate::collections::SmallVec;

pub(crate) use edges::{compute_blue_edges, compute_edges};
pub(crate) use segments::{compute_segments, link_segments};

/// Maximum number of segments and edges stored inline.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.h#L306>
const MAX_INLINE_SEGMENTS: usize = 18;
const MAX_INLINE_EDGES: usize = 12;

/// Either horizontal or vertical.
///
/// A type alias because it's used as an index.
pub type Dimension = usize;

/// Segments and edges for one dimension of an outline.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.h#L309>
#[derive(Clone, Default, Debug)]
pub(crate) struct Axis {
    /// Either horizontal or vertical.
    pub dim: Dimension,
    /// Depends on dimension and outline orientation.
    pub major_dir: Direction,
    /// Collection of segments for the axis.
    pub segments: SmallVec<Segment, MAX_INLINE_SEGMENTS>,
    /// Collection of edges for the axis.
    pub edges: SmallVec<Edge, MAX_INLINE_EDGES>,
}

impl Axis {
    /// X coordinates, i.e. vertical segments and edges.
    pub const HORIZONTAL: Dimension = 0;
    /// Y coordinates, i.e. horizontal segments and edges.
    pub const VERTICAL: Dimension = 1;
}

impl Axis {
    #[cfg(test)]
    pub fn new(dim: Dimension, orientation: Option<Orientation>) -> Self {
        let mut axis = Self::default();
        axis.reset(dim, orientation);
        axis
    }

    pub fn reset(&mut self, dim: Dimension, orientation: Option<Orientation>) {
        self.dim = dim;
        self.major_dir = match (dim, orientation) {
            (Self::HORIZONTAL, Some(Orientation::Clockwise)) => Direction::Down,
            (Self::VERTICAL, Some(Orientation::Clockwise)) => Direction::Right,
            (Self::HORIZONTAL, _) => Direction::Up,
            (Self::VERTICAL, _) => Direction::Left,
            _ => Direction::None,
        };
        self.segments.clear();
        self.edges.clear();
    }
}

impl Axis {
    /// Inserts the given edge into the sorted edge list.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.c#L197>
    pub fn insert_edge(&mut self, edge: Edge, top_to_bottom_hinting: bool) {
        self.edges.push(edge);
        let edges = self.edges.as_mut_slice();
        // If this is the first edge, we're done.
        if edges.len() == 1 {
            return;
        }
        // Now move it into place
        let mut ix = edges.len() - 1;
        while ix > 0 {
            let prev_ix = ix - 1;
            let prev_fpos = edges[prev_ix].fpos;
            if (top_to_bottom_hinting && prev_fpos > edge.fpos)
                || (!top_to_bottom_hinting && prev_fpos < edge.fpos)
            {
                break;
            }
            // Edges with the same position and minor direction should appear
            // before those with the major direction
            if prev_fpos == edge.fpos && edge.dir == self.major_dir {
                break;
            }
            let prev_edge = edges[prev_ix];
            edges[ix] = prev_edge;
            ix -= 1;
        }
        edges[ix] = edge;
    }

    /// Links the given segment and edge.
    pub fn append_segment_to_edge(&mut self, segment_ix: usize, edge_ix: usize) {
        let edge = &mut self.edges[edge_ix];
        let first_ix = edge.first_ix;
        let last_ix = edge.last_ix;
        edge.last_ix = segment_ix as u16;
        let segment = &mut self.segments[segment_ix];
        segment.edge_next_ix = Some(first_ix);
        self.segments[last_ix as usize].edge_next_ix = Some(segment_ix as u16);
    }
}

/// Sequence of points with a single dominant direction.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.h#L262>
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub(crate) struct Segment {
    /// Flags describing the properties of the segment.
    pub flags: u8,
    /// Dominant direction of the segment.
    pub dir: Direction,
    /// Position of the segment.
    pub pos: i16,
    /// Deviation from segment position.
    pub delta: i16,
    /// Minimum coordinate of the segment.
    pub min_coord: i16,
    /// Maximum coordinate of the segment.
    pub max_coord: i16,
    /// Hinted segment height.
    pub height: i16,
    /// Used during stem matching.
    pub score: i32,
    /// Used during stem matching.
    pub len: i32,
    /// Index of best candidate for a stem link.
    pub link_ix: Option<u16>,
    /// Index of best candidate for a serif link.
    pub serif_ix: Option<u16>,
    /// Index of first point in the outline.
    pub first_ix: u16,
    /// Index of last point in the outline.
    pub last_ix: u16,
    /// Index of edge that is associated with the segment.
    pub edge_ix: Option<u16>,
    /// Index of next segment in edge's segment list.
    pub edge_next_ix: Option<u16>,
}

/// Segment flags.
///
/// Note: these are the same as edge flags.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.h#L227>
impl Segment {
    pub const NORMAL: u8 = 0;
    pub const ROUND: u8 = 1;
    pub const SERIF: u8 = 2;
    pub const DONE: u8 = 4;
    pub const NEUTRAL: u8 = 8;
}

impl Segment {
    pub fn first(&self) -> usize {
        self.first_ix as usize
    }

    pub fn first_point<'a>(&self, points: &'a [Point]) -> &'a Point {
        &points[self.first()]
    }

    pub fn last(&self) -> usize {
        self.last_ix as usize
    }

    pub fn last_point<'a>(&self, points: &'a [Point]) -> &'a Point {
        &points[self.last()]
    }

    pub fn edge<'a>(&self, edges: &'a [Edge]) -> Option<&'a Edge> {
        edges.get(self.edge_ix.map(|ix| ix as usize)?)
    }

    /// Returns the next segment in this segment's parent edge.
    pub fn next_in_edge<'a>(&self, segments: &'a [Segment]) -> Option<&'a Segment> {
        segments.get(self.edge_next_ix.map(|ix| ix as usize)?)
    }

    pub fn link<'a>(&self, segments: &'a [Segment]) -> Option<&'a Segment> {
        segments.get(self.link_ix.map(|ix| ix as usize)?)
    }
}

/// Sequence of segments used for grid-fitting.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.h#L286>
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub(crate) struct Edge {
    /// Original, unscaled position in font units.
    pub fpos: i16,
    /// Original, scaled position.
    pub opos: i32,
    /// Current position.
    pub pos: i32,
    /// Edge flags.
    pub flags: u8,
    /// Edge direction.
    pub dir: Direction,
    /// Present if this is a blue edge.
    pub blue_edge: Option<ScaledWidth>,
    /// Index of linked edge.
    pub link_ix: Option<u16>,
    /// Index of primary edge for serif.
    pub serif_ix: Option<u16>,
    /// Used to speed up edge interpolation.
    pub scale: i32,
    /// Index of first segment in edge.
    pub first_ix: u16,
    /// Index of last segment in edge.
    pub last_ix: u16,
}

/// Edge flags.
///
/// Note: these are the same as segment flags.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.h#L227>
impl Edge {
    pub const NORMAL: u8 = Segment::NORMAL;
    pub const ROUND: u8 = Segment::ROUND;
    pub const SERIF: u8 = Segment::SERIF;
    pub const DONE: u8 = Segment::DONE;
    pub const NEUTRAL: u8 = Segment::NEUTRAL;
}

impl Edge {
    pub fn link<'a>(&self, edges: &'a [Edge]) -> Option<&'a Edge> {
        edges.get(self.link_ix.map(|ix| ix as usize)?)
    }

    pub fn serif<'a>(&self, edges: &'a [Edge]) -> Option<&'a Edge> {
        edges.get(self.serif_ix.map(|ix| ix as usize)?)
    }
}
