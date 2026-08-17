//! TrueType outline types.

use std::mem::size_of;

use super::super::{
    path::{to_path, ToPathError},
    pen::PathStyle,
    DrawError, Hinting, OutlinePen,
};
use raw::tables::glyf::PointCoord;
use read_fonts::{
    tables::glyf::{Glyph, PointFlags},
    types::{F26Dot6, Fixed, GlyphId, Point},
};

/// Maximum number of points we support in a single outline including
/// composites.
///
/// TrueType uses a 16 bit integer to store contour end points so
/// we must keep the total count within this value.
///
/// The maxp <https://learn.microsoft.com/en-us/typography/opentype/spec/maxp>
/// table encodes `maxCompositePoints` as a `uint16` so the spec enforces
/// this limit.
const MAX_POINTS: usize = u16::MAX as usize;

/// Represents the information necessary to scale a glyph outline.
///
/// Contains a reference to the glyph data itself as well as metrics that
/// can be used to compute the memory requirements for scaling the glyph.
#[derive(Clone, Default)]
pub struct Outline<'a> {
    pub glyph_id: GlyphId,
    /// The associated top-level glyph for the outline.
    pub glyph: Option<Glyph<'a>>,
    /// Sum of the point counts of all simple glyphs in an outline.
    pub points: usize,
    /// Sum of the contour counts of all simple glyphs in an outline.
    pub contours: usize,
    /// Maximum number of points in a single simple glyph.
    pub max_simple_points: usize,
    /// "Other" points are the unscaled or original scaled points.
    ///
    /// The size of these buffer is the same and this value tracks the size
    /// for one (not both) of the buffers. This is the maximum of
    /// `max_simple_points` and the total number of points for all component
    /// glyphs in a single composite glyph.
    pub max_other_points: usize,
    /// Maximum size of the component delta stack.
    ///
    /// For composite glyphs in variable fonts, delta values are computed
    /// for each component. This tracks the maximum stack depth necessary
    /// to store those values during processing.
    pub max_component_delta_stack: usize,
    /// Number of entries in the hinting value stack.
    pub max_stack: usize,
    /// Number of CVT entries for copy-on-write support.
    pub cvt_count: usize,
    /// Number of storage area entries for copy-on-write support.
    pub storage_count: usize,
    /// Maximum number of points in the twilight zone for hinting.
    pub max_twilight_points: usize,
    /// True if any component of a glyph has bytecode instructions.
    pub has_hinting: bool,
    /// True if the glyph requires variation delta processing.
    pub has_variations: bool,
    /// True if the glyph contains any simple or compound overlap flags.
    pub has_overlaps: bool,
}

impl Outline<'_> {
    /// Returns the minimum size in bytes required to scale an outline based
    /// on the computed sizes.
    pub fn required_buffer_size(&self, hinting: Hinting) -> usize {
        let mut size = 0;
        let hinting = self.has_hinting && hinting == Hinting::Embedded;
        // Scaled, unscaled and (for hinting) original scaled points
        size += self.points * size_of::<Point<F26Dot6>>();
        // Unscaled and (if hinted) original scaled points
        size += self.max_other_points * size_of::<Point<i32>>() * if hinting { 2 } else { 1 };
        // Contour end points
        size += self.contours * size_of::<u16>();
        // Point flags
        size += self.points * size_of::<PointFlags>();
        if self.has_variations {
            // Interpolation buffer for delta IUP
            size += self.max_simple_points * size_of::<Point<Fixed>>();
            // Delta buffer for points
            size += self.max_simple_points * size_of::<Point<Fixed>>();
            // Delta buffer for composite components
            size += self.max_component_delta_stack * size_of::<Point<Fixed>>();
        }
        if hinting {
            // Hinting value stack
            size += self.max_stack * size_of::<i32>();
            // CVT and storage area copy-on-write buffers
            size += (self.cvt_count + self.storage_count) * size_of::<i32>();
            // Twilight zone storage. Two point buffers plus one point flags buffer
            size += self.max_twilight_points
                * (size_of::<Point<F26Dot6>>() * 2 + size_of::<PointFlags>());
        }
        if size != 0 {
            // If we're given a buffer that is not aligned, we'll need to
            // adjust, so add our maximum alignment requirement in bytes.
            size += std::mem::align_of::<i32>();
        }
        size
    }

    pub(super) fn ensure_point_count_limit(&self) -> Result<(), DrawError> {
        if self.points > MAX_POINTS {
            Err(DrawError::TooManyPoints(self.glyph_id))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScaledOutline<'a, C>
where
    C: PointCoord,
{
    pub points: &'a mut [Point<C>],
    pub flags: &'a mut [PointFlags],
    pub contours: &'a mut [u16],
    pub phantom_points: [Point<C>; 4],
    pub hdmx_width: Option<u8>,
}

impl<'a, C> ScaledOutline<'a, C>
where
    C: PointCoord,
{
    pub(crate) fn new(
        points: &'a mut [Point<C>],
        phantom_points: [Point<C>; 4],
        flags: &'a mut [PointFlags],
        contours: &'a mut [u16],
        hdmx_width: Option<u8>,
    ) -> Self {
        let x_shift = phantom_points[0].x;
        if x_shift != C::zeroed() {
            for point in points.iter_mut() {
                point.x = point.x - x_shift;
            }
        }
        Self {
            points,
            flags,
            contours,
            phantom_points,
            hdmx_width,
        }
    }

    pub fn adjusted_lsb(&self) -> C {
        self.phantom_points[0].x
    }

    pub fn adjusted_advance_width(&self) -> C {
        // Prefer widths from hdmx, otherwise take difference between first
        // two phantom points
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttgload.c#L1996>
        if let Some(hdmx_width) = self.hdmx_width {
            C::from_i32(hdmx_width as i32)
        } else {
            self.phantom_points[1].x - self.phantom_points[0].x
        }
    }

    pub fn to_path(
        &self,
        path_style: PathStyle,
        pen: &mut impl OutlinePen,
    ) -> Result<(), ToPathError> {
        to_path(self.points, self.flags, self.contours, path_style, pen)
    }
}
