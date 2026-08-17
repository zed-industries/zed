//! Drawing color glyphs.
//!
//! # Examples
//! ## Retrieve the clip box of a COLRv1 glyph if it has one:
//!
//! ```
//! # use core::result::Result;
//! # use skrifa::{instance::{Size, Location}, color::{ColorGlyphFormat, ColorPainter, PaintError}, GlyphId, MetadataProvider};
//! # fn get_colr_bb(font: read_fonts::FontRef, color_painter_impl : &mut impl ColorPainter, glyph_id : GlyphId, size: Size) -> Result<(), PaintError> {
//! match font.color_glyphs()
//!       .get_with_format(glyph_id, ColorGlyphFormat::ColrV1)
//!       .expect("Glyph not found.")
//!       .bounding_box(&Location::default(), size)
//! {
//!   Some(bounding_box) => {
//!       println!("Bounding box is {:?}", bounding_box);
//!   }
//!   None => {
//!       println!("Glyph has no clip box.");
//!   }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Paint a COLRv1 glyph given a font, and a glyph id and a [`ColorPainter`] implementation:
//! ```
//! # use core::result::Result;
//! # use skrifa::{instance::{Size, Location}, color::{ColorGlyphFormat, ColorPainter, PaintError}, GlyphId, MetadataProvider};
//! # fn paint_colr(font: read_fonts::FontRef, color_painter_impl : &mut impl ColorPainter, glyph_id : GlyphId) -> Result<(), PaintError> {
//! let color_glyph = font.color_glyphs()
//!                     .get_with_format(glyph_id, ColorGlyphFormat::ColrV1)
//!                     .expect("Glyph not found");
//! color_glyph.paint(&Location::default(), color_painter_impl)
//! # }
//! ```
//!
mod instance;
mod transform;
mod traversal;

#[cfg(test)]
mod traversal_tests;

use raw::{tables::colr, FontRef};
#[cfg(test)]
use serde::{Deserialize, Serialize};

pub use read_fonts::tables::colr::{CompositeMode, Extend};

use read_fonts::{
    types::{BoundingBox, GlyphId, Point},
    ReadError, TableProvider,
};

use std::{fmt::Debug, ops::Range};

use traversal::{
    get_clipbox_font_units, traverse_v0_range, traverse_with_callbacks, PaintDecycler,
};

pub use transform::Transform;

use crate::prelude::{LocationRef, Size};

use self::instance::{resolve_paint, PaintId};

/// An error during drawing a COLR glyph.
///
/// This covers inconsistencies in the COLRv1 paint graph as well as downstream
/// parse errors from read-fonts.
#[derive(Debug, Clone)]
pub enum PaintError {
    ParseError(ReadError),
    GlyphNotFound(GlyphId),
    PaintCycleDetected,
    DepthLimitExceeded,
}

impl std::fmt::Display for PaintError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PaintError::ParseError(read_error) => {
                write!(f, "Error parsing font data: {read_error}")
            }
            PaintError::GlyphNotFound(glyph_id) => {
                write!(f, "No COLRv1 glyph found for glyph id: {glyph_id}")
            }
            PaintError::PaintCycleDetected => write!(f, "Paint cycle detected in COLRv1 glyph."),
            PaintError::DepthLimitExceeded => write!(f, "Depth limit exceeded in COLRv1 glyph."),
        }
    }
}

impl From<ReadError> for PaintError {
    fn from(value: ReadError) -> Self {
        PaintError::ParseError(value)
    }
}

/// A color stop of a gradient.
///
/// All gradient callbacks of [`ColorPainter`] normalize color stops to be in the range of 0
/// to 1.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
// This repr(C) is required so that C-side FFI's
// are able to cast the ColorStop slice to a C-side array pointer.
#[repr(C)]
pub struct ColorStop {
    pub offset: f32,
    /// Specifies a color from the `CPAL` table.
    pub palette_index: u16,
    /// Additional alpha value, to be multiplied with the color above before use.
    pub alpha: f32,
}

// Design considerations for choosing a slice of ColorStops as `color_stop`
// type: In principle, a local `Vec<ColorStop>` allocation would not required if
// we're willing to walk the `ResolvedColorStop` iterator to find the minimum
// and maximum color stops.  Then we could scale the color stops based on the
// minimum and maximum. But performing the min/max search would require
// re-applying the deltas at least once, after which we would pass the scaled
// stops to client side and have the client sort the collected items once
// again. If we do want to pre-ort them, and still use use an
// `Iterator<Item=ColorStop>` instead as the `color_stops` field, then we would
// need a Fontations-side allocations to sort, and an extra allocation on the
// client side to `.collect()` from the provided iterator before passing it to
// drawing API.
//
/// A fill type of a COLRv1 glyph (solid fill or various gradient types).
///
/// The client receives the information about the fill type in the
/// [`fill`](ColorPainter::fill) callback of the [`ColorPainter`] trait.
#[derive(Debug, PartialEq)]
pub enum Brush<'a> {
    /// A solid fill with the color specified by `palette_index`. The respective
    /// color from the CPAL table then needs to be multiplied with `alpha`.
    Solid { palette_index: u16, alpha: f32 },
    /// A linear gradient, normalized from the P0, P1 and P2 representation in
    /// the COLRv1 table to a linear gradient between two points `p0` and
    /// `p1`. If there is only one color stop, the client should draw a solid
    /// fill with that color. The `color_stops` are normalized to the range from
    /// 0 to 1.
    LinearGradient {
        p0: Point<f32>,
        p1: Point<f32>,
        color_stops: &'a [ColorStop],
        extend: Extend,
    },
    /// A radial gradient, with color stops normalized to the range of 0 to 1.
    /// Caution: This normalization can mean that negative radii occur. It is
    /// the client's responsibility to truncate the color line at the 0
    /// position, interpolating between `r0` and `r1` and compute an
    /// interpolated color at that position.
    RadialGradient {
        c0: Point<f32>,
        r0: f32,
        c1: Point<f32>,
        r1: f32,
        color_stops: &'a [ColorStop],
        extend: Extend,
    },
    /// A sweep gradient, also called conical gradient. The color stops are
    /// normalized to the range from 0 to 1 and the returned angles are to be
    /// interpreted in _clockwise_ direction (swapped from the meaning in the
    /// font file).  The stop normalization may mean that the angles may be
    /// larger or smaller than the range of 0 to 360. Note that only the range
    /// from 0 to 360 degrees is to be drawn, see
    /// <https://learn.microsoft.com/en-us/typography/opentype/spec/colr#sweep-gradients>.
    SweepGradient {
        c0: Point<f32>,
        start_angle: f32,
        end_angle: f32,
        color_stops: &'a [ColorStop],
        extend: Extend,
    },
}

/// Signals success of request to draw a COLRv1 sub glyph from cache.
///
/// Result of [`paint_cached_color_glyph`](ColorPainter::paint_cached_color_glyph)
/// through which the client signals whether a COLRv1 glyph referenced by
/// another COLRv1 glyph was drawn from cache or whether the glyph's subgraph
/// should be traversed by the skria side COLRv1 implementation.
pub enum PaintCachedColorGlyph {
    /// The specified COLRv1 glyph has been successfully painted client side.
    Ok,
    /// The client does not implement drawing COLRv1 glyphs from cache and the
    /// Fontations side COLRv1 implementation is asked to traverse the
    /// respective PaintColorGlyph sub graph.
    Unimplemented,
}

/// A group of required painting callbacks to be provided by the client.
///
/// Each callback is executing a particular drawing or canvas transformation
/// operation. The trait's callback functions are invoked when
/// [`paint`](ColorGlyph::paint) is called with a [`ColorPainter`] trait
/// object. The documentation for each function describes what actions are to be
/// executed using the client side 2D graphics API, usually by performing some
/// kind of canvas operation.
pub trait ColorPainter {
    /// Push the specified transform by concatenating it to the current
    /// transformation matrix.
    fn push_transform(&mut self, transform: Transform);

    /// Restore the transformation matrix to the state before the previous
    /// [`push_transform`](ColorPainter::push_transform) call.
    fn pop_transform(&mut self);

    /// Apply a clip path in the shape of glyph specified by `glyph_id`.
    fn push_clip_glyph(&mut self, glyph_id: GlyphId);

    /// Apply a clip rectangle specified by `clip_rect`.
    fn push_clip_box(&mut self, clip_box: BoundingBox<f32>);

    /// Restore the clip state to the state before a previous
    /// [`push_clip_glyph`](ColorPainter::push_clip_glyph) or
    /// [`push_clip_box`](ColorPainter::push_clip_box) call.
    fn pop_clip(&mut self);

    /// Fill the current clip area with the specified gradient fill.
    fn fill(&mut self, brush: Brush<'_>);

    /// Combined clip and fill operation.
    ///
    /// Apply the clip path determined by the specified `glyph_id`, then fill it
    /// with the specified [`brush`](Brush), applying the `_brush_transform`
    /// transformation matrix to the brush. The default implementation works
    /// based on existing methods in this trait. It is recommended for clients
    /// to override the default implementaition with a custom combined clip and
    /// fill operation. In this way overriding likely results in performance
    /// gains depending on performance characteristics of the 2D graphics stack
    /// that these calls are mapped to.
    fn fill_glyph(
        &mut self,
        glyph_id: GlyphId,
        brush_transform: Option<Transform>,
        brush: Brush<'_>,
    ) {
        self.push_clip_glyph(glyph_id);
        if let Some(wrap_in_transform) = brush_transform {
            self.push_transform(wrap_in_transform);
            self.fill(brush);
            self.pop_transform();
        } else {
            self.fill(brush);
        }
        self.pop_clip();
    }

    /// Optionally implement this method: Draw an unscaled COLRv1 glyph given
    /// the current transformation matrix (as accumulated by
    /// [`push_transform`](ColorPainter::push_transform) calls).
    fn paint_cached_color_glyph(
        &mut self,
        _glyph: GlyphId,
    ) -> Result<PaintCachedColorGlyph, PaintError> {
        Ok(PaintCachedColorGlyph::Unimplemented)
    }

    /// Open a new layer, and merge the layer down using `composite_mode` when
    /// [`pop_layer`](ColorPainter::pop_layer) is called, signalling that this layer is done drawing.
    fn push_layer(&mut self, composite_mode: CompositeMode);

    /// Merge the pushed layer down using `composite_mode` passed to the matching
    /// [`push_layer`](ColorPainter::push_layer).
    fn pop_layer(&mut self) {}

    /// Alternative version of [`push_layer`](ColorPainter::push_layer) where the
    /// `composite_mode` is also passed to the method. This is useful for
    /// graphics libraries that need the compositing mode at layer pop time
    /// and do not want to manually track the mode.
    ///
    /// Only one of [`pop_layer`](ColorPainter::pop_layer) or this method
    /// need to be implemented. By default, this simply calls
    /// [`pop_layer`](ColorPainter::pop_layer).
    fn pop_layer_with_mode(&mut self, _composite_mode: CompositeMode) {
        self.pop_layer();
    }
}

/// Distinguishes available color glyph formats.
#[derive(Clone, Copy)]
pub enum ColorGlyphFormat {
    ColrV0,
    ColrV1,
}

/// A representation of a color glyph that can be painted through a sequence of [`ColorPainter`] callbacks.
#[derive(Clone)]
pub struct ColorGlyph<'a> {
    colr: colr::Colr<'a>,
    root_paint_ref: ColorGlyphRoot<'a>,
}

#[derive(Clone)]
enum ColorGlyphRoot<'a> {
    V0Range(Range<usize>),
    V1Paint(colr::Paint<'a>, PaintId, GlyphId, Result<u16, ReadError>),
}

impl<'a> ColorGlyph<'a> {
    /// Returns the version of the color table from which this outline was
    /// selected.
    pub fn format(&self) -> ColorGlyphFormat {
        match &self.root_paint_ref {
            ColorGlyphRoot::V0Range(_) => ColorGlyphFormat::ColrV0,
            ColorGlyphRoot::V1Paint(..) => ColorGlyphFormat::ColrV1,
        }
    }

    /// Returns the bounding box.
    ///
    /// For COLRv1 glyphs, this is the clip box of the specified COLRv1 glyph,
    /// or `None` if clip boxes are not present or if there is none for the
    /// particular glyph.
    ///
    /// Always returns `None` for COLRv0 glyphs because precomputed clip boxes
    /// are never available.
    ///
    /// The `size` argument can optionally be used to scale the bounding box
    /// to a particular font size and `location` allows specifying a variation
    /// instance.
    pub fn bounding_box(
        &self,
        location: impl Into<LocationRef<'a>>,
        size: Size,
    ) -> Option<BoundingBox<f32>> {
        match &self.root_paint_ref {
            ColorGlyphRoot::V1Paint(_paint, _paint_id, glyph_id, upem) => {
                let instance =
                    instance::ColrInstance::new(self.colr.clone(), location.into().coords());
                let resolved_bounding_box = get_clipbox_font_units(&instance, *glyph_id);
                resolved_bounding_box.map(|bounding_box| {
                    let scale_factor = size.linear_scale((*upem).clone().unwrap_or(0));
                    bounding_box.scale(scale_factor)
                })
            }
            _ => None,
        }
    }

    /// Evaluates the paint graph at the specified location in variation space
    /// and emits the results to the given painter.
    ///
    ///
    /// For a COLRv1 glyph, traverses the COLRv1 paint graph and invokes drawing callbacks on a
    /// specified [`ColorPainter`] trait object.  The traversal operates in font
    /// units and will call `ColorPainter` methods with font unit values. This
    /// means, if you want to draw a COLRv1 glyph at a particular font size, the
    /// canvas needs to have a transformation matrix applied so that it scales down
    /// the drawing operations to `font_size / upem`.
    ///
    /// # Arguments
    ///
    /// * `glyph_id` the `GlyphId` to be drawn.
    /// * `location` coordinates for specifying a variation instance. This can be empty.
    /// * `painter` a client-provided [`ColorPainter`] implementation receiving drawing callbacks.
    ///
    pub fn paint(
        &self,
        location: impl Into<LocationRef<'a>>,
        painter: &mut impl ColorPainter,
    ) -> Result<(), PaintError> {
        let instance =
            instance::ColrInstance::new(self.colr.clone(), location.into().effective_coords());
        let mut resolved_stops = traversal::ColorStopVec::default();
        match &self.root_paint_ref {
            ColorGlyphRoot::V1Paint(paint, paint_id, glyph_id, _) => {
                let clipbox = get_clipbox_font_units(&instance, *glyph_id);

                if let Some(rect) = clipbox {
                    painter.push_clip_box(rect);
                }

                let mut decycler = PaintDecycler::default();
                let mut cycle_guard = decycler.enter(*paint_id)?;
                traverse_with_callbacks(
                    &resolve_paint(&instance, paint)?,
                    &instance,
                    painter,
                    &mut cycle_guard,
                    &mut resolved_stops,
                    0,
                )?;

                if clipbox.is_some() {
                    painter.pop_clip();
                }
                Ok(())
            }
            ColorGlyphRoot::V0Range(range) => {
                traverse_v0_range(range, &instance, painter)?;
                Ok(())
            }
        }
    }
}

/// Collection of color glyphs.
#[derive(Clone)]
pub struct ColorGlyphCollection<'a> {
    colr: Option<colr::Colr<'a>>,
    upem: Result<u16, ReadError>,
}

impl<'a> ColorGlyphCollection<'a> {
    /// Creates a new collection of paintable color glyphs for the given font.
    pub fn new(font: &FontRef<'a>) -> Self {
        let colr = font.colr().ok();
        let upem = font.head().map(|h| h.units_per_em());

        Self { colr, upem }
    }

    /// Returns the color glyph representation for the given glyph identifier,
    /// given a specific format.
    pub fn get_with_format(
        &self,
        glyph_id: GlyphId,
        glyph_format: ColorGlyphFormat,
    ) -> Option<ColorGlyph<'a>> {
        let colr = self.colr.clone()?;

        let root_paint_ref = match glyph_format {
            ColorGlyphFormat::ColrV0 => {
                let layer_range = colr.v0_base_glyph(glyph_id).ok()??;
                ColorGlyphRoot::V0Range(layer_range)
            }
            ColorGlyphFormat::ColrV1 => {
                let (paint, paint_id) = colr.v1_base_glyph(glyph_id).ok()??;
                ColorGlyphRoot::V1Paint(paint, paint_id, glyph_id, self.upem.clone())
            }
        };
        Some(ColorGlyph {
            colr,
            root_paint_ref,
        })
    }

    /// Returns a color glyph representation for the given glyph identifier if
    /// available, preferring a COLRv1 representation over a COLRv0
    /// representation.
    pub fn get(&self, glyph_id: GlyphId) -> Option<ColorGlyph<'a>> {
        self.get_with_format(glyph_id, ColorGlyphFormat::ColrV1)
            .or_else(|| self.get_with_format(glyph_id, ColorGlyphFormat::ColrV0))
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        color::traversal_tests::test_glyph_defs::PAINTCOLRGLYPH_CYCLE,
        prelude::{LocationRef, Size},
        MetadataProvider,
    };

    use read_fonts::{types::BoundingBox, FontRef};

    use super::{Brush, ColorPainter, CompositeMode, GlyphId, Transform};
    use crate::color::traversal_tests::test_glyph_defs::{COLORED_CIRCLES_V0, COLORED_CIRCLES_V1};

    #[test]
    fn has_colrv1_glyph_test() {
        let colr_font = font_test_data::COLRV0V1_VARIABLE;
        let font = FontRef::new(colr_font).unwrap();
        let get_colrv1_glyph = |codepoint: &[char]| {
            font.charmap().map(codepoint[0]).and_then(|glyph_id| {
                font.color_glyphs()
                    .get_with_format(glyph_id, crate::color::ColorGlyphFormat::ColrV1)
            })
        };

        assert!(get_colrv1_glyph(COLORED_CIRCLES_V0).is_none());
        assert!(get_colrv1_glyph(COLORED_CIRCLES_V1).is_some());
    }
    struct DummyColorPainter {}

    impl DummyColorPainter {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Default for DummyColorPainter {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ColorPainter for DummyColorPainter {
        fn push_transform(&mut self, _transform: Transform) {}
        fn pop_transform(&mut self) {}
        fn push_clip_glyph(&mut self, _glyph: GlyphId) {}
        fn push_clip_box(&mut self, _clip_box: BoundingBox<f32>) {}
        fn pop_clip(&mut self) {}
        fn fill(&mut self, _brush: Brush) {}
        fn push_layer(&mut self, _composite_mode: CompositeMode) {}
        fn pop_layer(&mut self) {}
    }

    #[test]
    fn paintcolrglyph_cycle_test() {
        let colr_font = font_test_data::COLRV0V1_VARIABLE;
        let font = FontRef::new(colr_font).unwrap();
        let cycle_glyph_id = font.charmap().map(PAINTCOLRGLYPH_CYCLE[0]).unwrap();
        let colrv1_glyph = font
            .color_glyphs()
            .get_with_format(cycle_glyph_id, crate::color::ColorGlyphFormat::ColrV1);

        assert!(colrv1_glyph.is_some());
        let mut color_painter = DummyColorPainter::new();

        let result = colrv1_glyph
            .unwrap()
            .paint(LocationRef::default(), &mut color_painter);
        // Expected to fail with an error as the glyph contains a paint cycle.
        assert!(result.is_err());
    }

    #[test]
    fn no_cliplist_test() {
        let colr_font = font_test_data::COLRV1_NO_CLIPLIST;
        let font = FontRef::new(colr_font).unwrap();
        let cycle_glyph_id = GlyphId::new(1);
        let colrv1_glyph = font
            .color_glyphs()
            .get_with_format(cycle_glyph_id, crate::color::ColorGlyphFormat::ColrV1);

        assert!(colrv1_glyph.is_some());
        let mut color_painter = DummyColorPainter::new();

        let result = colrv1_glyph
            .unwrap()
            .paint(LocationRef::default(), &mut color_painter);
        assert!(result.is_ok());
    }

    #[test]
    fn colrv0_no_bbox_test() {
        let colr_font = font_test_data::COLRV0V1;
        let font = FontRef::new(colr_font).unwrap();
        let colrv0_glyph_id = GlyphId::new(168);
        let colrv0_glyph = font
            .color_glyphs()
            .get_with_format(colrv0_glyph_id, super::ColorGlyphFormat::ColrV0)
            .unwrap();
        assert!(colrv0_glyph
            .bounding_box(LocationRef::default(), Size::unscaled())
            .is_none());
    }
}
