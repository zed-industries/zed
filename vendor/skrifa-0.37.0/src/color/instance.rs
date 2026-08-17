//! COLR table instance.

use read_fonts::{
    tables::{
        colr::*,
        variations::{
            DeltaSetIndex, DeltaSetIndexMap, FloatItemDelta, FloatItemDeltaTarget,
            ItemVariationStore,
        },
    },
    types::{BoundingBox, F2Dot14, GlyphId16, Point},
    ReadError,
};

use core::ops::{Deref, Range};

/// Unique paint identifier used for detecting cycles in the paint graph.
pub type PaintId = usize;

/// Combination of a `COLR` table and a location in variation space for
/// resolving paints.
///
/// See [`resolve_paint`], [`ColorStops::resolve`] and [`resolve_clip_box`].
#[derive(Clone)]
pub struct ColrInstance<'a> {
    colr: Colr<'a>,
    index_map: Option<DeltaSetIndexMap<'a>>,
    var_store: Option<ItemVariationStore<'a>>,
    coords: &'a [F2Dot14],
}

impl<'a> ColrInstance<'a> {
    /// Creates a new instance for the given `COLR` table and normalized variation
    /// coordinates.
    pub fn new(colr: Colr<'a>, coords: &'a [F2Dot14]) -> Self {
        let index_map = colr.var_index_map().and_then(|res| res.ok());
        let var_store = colr.item_variation_store().and_then(|res| res.ok());
        Self {
            colr,
            coords,
            index_map,
            var_store,
        }
    }

    /// Computes a sequence of N variation deltas starting at the given
    /// `var_base` index.
    fn var_deltas<const N: usize>(&self, var_index_base: u32) -> [FloatItemDelta; N] {
        // Magic value that indicates deltas should not be applied.
        const NO_VARIATION_DELTAS: u32 = 0xFFFFFFFF;
        // Note: FreeType never returns an error for these lookups, so
        // we do the same and just `unwrap_or_default` on var store
        // errors.
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/fc01e7dd/src/sfnt/ttcolr.c#L574>
        let mut deltas = [FloatItemDelta::ZERO; N];
        if self.coords.is_empty()
            || self.var_store.is_none()
            || var_index_base == NO_VARIATION_DELTAS
        {
            return deltas;
        }
        let var_store = self.var_store.as_ref().unwrap();
        if let Some(index_map) = self.index_map.as_ref() {
            for (i, delta) in deltas.iter_mut().enumerate() {
                let var_index = var_index_base + i as u32;
                if let Ok(delta_ix) = index_map.get(var_index) {
                    *delta = var_store
                        .compute_float_delta(delta_ix, self.coords)
                        .unwrap_or_default();
                }
            }
        } else {
            for (i, delta) in deltas.iter_mut().enumerate() {
                let var_index = var_index_base + i as u32;
                // If we don't have a var index map, use our index as the inner
                // component and set the outer to 0.
                let delta_ix = DeltaSetIndex {
                    outer: 0,
                    inner: var_index as u16,
                };
                *delta = var_store
                    .compute_float_delta(delta_ix, self.coords)
                    .unwrap_or_default();
            }
        }
        deltas
    }
}

impl<'a> Deref for ColrInstance<'a> {
    type Target = Colr<'a>;

    fn deref(&self) -> &Self::Target {
        &self.colr
    }
}

/// Resolves a clip box, applying variation deltas using the given
/// instance.
pub fn resolve_clip_box(instance: &ColrInstance, clip_box: &ClipBox) -> BoundingBox<f32> {
    match clip_box {
        ClipBox::Format1(cbox) => BoundingBox {
            x_min: cbox.x_min().to_i16() as f32,
            y_min: cbox.y_min().to_i16() as f32,
            x_max: cbox.x_max().to_i16() as f32,
            y_max: cbox.y_max().to_i16() as f32,
        },
        ClipBox::Format2(cbox) => {
            let deltas = instance.var_deltas::<4>(cbox.var_index_base());
            BoundingBox {
                x_min: cbox.x_min().apply_float_delta(deltas[0]),
                y_min: cbox.y_min().apply_float_delta(deltas[1]),
                x_max: cbox.x_max().apply_float_delta(deltas[2]),
                y_max: cbox.y_max().apply_float_delta(deltas[3]),
            }
        }
    }
}

/// Simplified version of a [`ColorStop`] or [`VarColorStop`] with applied
/// variation deltas.
#[derive(Clone, Debug)]
pub struct ResolvedColorStop {
    pub offset: f32,
    pub palette_index: u16,
    pub alpha: f32,
}

/// Collection of [`ColorStop`] or [`VarColorStop`].
// Note: only one of these fields is used at any given time, but this structure
// was chosen over the obvious enum approach for simplicity in generating a
// single concrete type for the `impl Iterator` return type of the `resolve`
// method.
#[derive(Clone)]
pub struct ColorStops<'a> {
    stops: &'a [ColorStop],
    var_stops: &'a [VarColorStop],
}

impl ColorStops<'_> {
    pub fn len(&self) -> usize {
        self.stops.len() + self.var_stops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stops.is_empty() && self.var_stops.is_empty()
    }
}

impl<'a> From<ColorLine<'a>> for ColorStops<'a> {
    fn from(value: ColorLine<'a>) -> Self {
        Self {
            stops: value.color_stops(),
            var_stops: &[],
        }
    }
}

impl<'a> From<VarColorLine<'a>> for ColorStops<'a> {
    fn from(value: VarColorLine<'a>) -> Self {
        Self {
            stops: &[],
            var_stops: value.color_stops(),
        }
    }
}

impl<'a> ColorStops<'a> {
    /// Returns an iterator yielding resolved color stops with variation deltas
    /// applied.
    pub fn resolve(
        &self,
        instance: &'a ColrInstance<'a>,
    ) -> impl Iterator<Item = ResolvedColorStop> + 'a {
        self.stops
            .iter()
            .map(|stop| ResolvedColorStop {
                offset: stop.stop_offset().to_f32(),
                palette_index: stop.palette_index(),
                alpha: stop.alpha().to_f32(),
            })
            .chain(self.var_stops.iter().map(|stop| {
                let deltas = instance.var_deltas::<2>(stop.var_index_base());
                ResolvedColorStop {
                    offset: stop.stop_offset().apply_float_delta(deltas[0]),
                    palette_index: stop.palette_index(),
                    alpha: stop.alpha().apply_float_delta(deltas[1]),
                }
            }))
    }
}

/// Simplified version of `Paint` with applied variation deltas.
///
/// These are constructed with the [`resolve_paint`] function.
///
/// This is roughly equivalent to FreeType's
/// [`FT_COLR_Paint`](https://freetype.org/freetype2/docs/reference/ft2-layer_management.html#ft_colr_paint)
/// type.
pub enum ResolvedPaint<'a> {
    ColrLayers {
        range: Range<usize>,
    },
    Solid {
        palette_index: u16,
        alpha: f32,
    },
    LinearGradient {
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color_stops: ColorStops<'a>,
        extend: Extend,
    },
    RadialGradient {
        x0: f32,
        y0: f32,
        radius0: f32,
        x1: f32,
        y1: f32,
        radius1: f32,
        color_stops: ColorStops<'a>,
        extend: Extend,
    },
    SweepGradient {
        center_x: f32,
        center_y: f32,
        start_angle: f32,
        end_angle: f32,
        color_stops: ColorStops<'a>,
        extend: Extend,
    },
    Glyph {
        glyph_id: GlyphId16,
        paint: Paint<'a>,
    },
    ColrGlyph {
        glyph_id: GlyphId16,
    },
    Transform {
        xx: f32,
        yx: f32,
        xy: f32,
        yy: f32,
        dx: f32,
        dy: f32,
        paint: Paint<'a>,
    },
    Translate {
        dx: f32,
        dy: f32,
        paint: Paint<'a>,
    },
    Scale {
        scale_x: f32,
        scale_y: f32,
        around_center: Option<Point<f32>>,
        paint: Paint<'a>,
    },
    Rotate {
        angle: f32,
        around_center: Option<Point<f32>>,
        paint: Paint<'a>,
    },
    Skew {
        x_skew_angle: f32,
        y_skew_angle: f32,
        around_center: Option<Point<f32>>,
        paint: Paint<'a>,
    },
    Composite {
        source_paint: Paint<'a>,
        mode: CompositeMode,
        backdrop_paint: Paint<'a>,
    },
}

/// Resolves this paint with the given instance.
///
/// Resolving means that all numeric values are converted to 32-bit floating
/// point, variation deltas are applied (also computed fully in floating
/// point), and the various transform paints are collapsed into a single value
/// for their category (transform, translate, scale, rotate and skew).
///
/// This provides a simpler type for consumers that are more interested
/// in extracting the semantics of the graph rather than working with the
/// raw encoded structures.
pub fn resolve_paint<'a>(
    instance: &ColrInstance<'a>,
    paint: &Paint<'a>,
) -> Result<ResolvedPaint<'a>, ReadError> {
    Ok(match paint {
        Paint::ColrLayers(layers) => {
            let start = layers.first_layer_index() as usize;
            ResolvedPaint::ColrLayers {
                range: start..start + layers.num_layers() as usize,
            }
        }
        Paint::Solid(solid) => ResolvedPaint::Solid {
            palette_index: solid.palette_index(),
            alpha: solid.alpha().to_f32(),
        },
        Paint::VarSolid(solid) => {
            let deltas = instance.var_deltas::<1>(solid.var_index_base());
            ResolvedPaint::Solid {
                palette_index: solid.palette_index(),
                alpha: solid.alpha().apply_float_delta(deltas[0]),
            }
        }
        Paint::LinearGradient(gradient) => {
            let color_line = gradient.color_line()?;
            let extend = color_line.extend();
            ResolvedPaint::LinearGradient {
                x0: gradient.x0().to_i16() as f32,
                y0: gradient.y0().to_i16() as f32,
                x1: gradient.x1().to_i16() as f32,
                y1: gradient.y1().to_i16() as f32,
                x2: gradient.x2().to_i16() as f32,
                y2: gradient.y2().to_i16() as f32,
                color_stops: color_line.into(),
                extend,
            }
        }
        Paint::VarLinearGradient(gradient) => {
            let color_line = gradient.color_line()?;
            let extend = color_line.extend();
            let deltas = instance.var_deltas::<6>(gradient.var_index_base());
            ResolvedPaint::LinearGradient {
                x0: gradient.x0().apply_float_delta(deltas[0]),
                y0: gradient.y0().apply_float_delta(deltas[1]),
                x1: gradient.x1().apply_float_delta(deltas[2]),
                y1: gradient.y1().apply_float_delta(deltas[3]),
                x2: gradient.x2().apply_float_delta(deltas[4]),
                y2: gradient.y2().apply_float_delta(deltas[5]),
                color_stops: color_line.into(),
                extend,
            }
        }
        Paint::RadialGradient(gradient) => {
            let color_line = gradient.color_line()?;
            let extend = color_line.extend();
            ResolvedPaint::RadialGradient {
                x0: gradient.x0().to_i16() as f32,
                y0: gradient.y0().to_i16() as f32,
                radius0: gradient.radius0().to_u16() as f32,
                x1: gradient.x1().to_i16() as f32,
                y1: gradient.y1().to_i16() as f32,
                radius1: gradient.radius1().to_u16() as f32,
                color_stops: color_line.into(),
                extend,
            }
        }
        Paint::VarRadialGradient(gradient) => {
            let color_line = gradient.color_line()?;
            let extend = color_line.extend();
            let deltas = instance.var_deltas::<6>(gradient.var_index_base());
            ResolvedPaint::RadialGradient {
                x0: gradient.x0().apply_float_delta(deltas[0]),
                y0: gradient.y0().apply_float_delta(deltas[1]),
                radius0: gradient.radius0().apply_float_delta(deltas[2]),
                x1: gradient.x1().apply_float_delta(deltas[3]),
                y1: gradient.y1().apply_float_delta(deltas[4]),
                radius1: gradient.radius1().apply_float_delta(deltas[5]),
                color_stops: color_line.into(),
                extend,
            }
        }
        Paint::SweepGradient(gradient) => {
            let color_line = gradient.color_line()?;
            let extend = color_line.extend();
            ResolvedPaint::SweepGradient {
                center_x: gradient.center_x().to_i16() as f32,
                center_y: gradient.center_y().to_i16() as f32,
                start_angle: gradient.start_angle().to_f32(),
                end_angle: gradient.end_angle().to_f32(),
                color_stops: color_line.into(),
                extend,
            }
        }
        Paint::VarSweepGradient(gradient) => {
            let color_line = gradient.color_line()?;
            let extend = color_line.extend();
            let deltas = instance.var_deltas::<4>(gradient.var_index_base());
            ResolvedPaint::SweepGradient {
                center_x: gradient.center_x().apply_float_delta(deltas[0]),
                center_y: gradient.center_y().apply_float_delta(deltas[1]),
                start_angle: gradient.start_angle().apply_float_delta(deltas[2]),
                end_angle: gradient.end_angle().apply_float_delta(deltas[3]),
                color_stops: color_line.into(),
                extend,
            }
        }
        Paint::Glyph(glyph) => ResolvedPaint::Glyph {
            glyph_id: glyph.glyph_id(),
            paint: glyph.paint()?,
        },
        Paint::ColrGlyph(glyph) => ResolvedPaint::ColrGlyph {
            glyph_id: glyph.glyph_id(),
        },
        Paint::Transform(transform) => {
            let affine = transform.transform()?;
            let paint = transform.paint()?;
            ResolvedPaint::Transform {
                xx: affine.xx().to_f32(),
                yx: affine.yx().to_f32(),
                xy: affine.xy().to_f32(),
                yy: affine.yy().to_f32(),
                dx: affine.dx().to_f32(),
                dy: affine.dy().to_f32(),
                paint,
            }
        }
        Paint::VarTransform(transform) => {
            let affine = transform.transform()?;
            let paint = transform.paint()?;
            let deltas = instance.var_deltas::<6>(affine.var_index_base());
            ResolvedPaint::Transform {
                xx: affine.xx().apply_float_delta(deltas[0]),
                yx: affine.yx().apply_float_delta(deltas[1]),
                xy: affine.xy().apply_float_delta(deltas[2]),
                yy: affine.yy().apply_float_delta(deltas[3]),
                dx: affine.dx().apply_float_delta(deltas[4]),
                dy: affine.dy().apply_float_delta(deltas[5]),
                paint,
            }
        }
        Paint::Translate(transform) => ResolvedPaint::Translate {
            dx: transform.dx().to_i16() as f32,
            dy: transform.dy().to_i16() as f32,
            paint: transform.paint()?,
        },
        Paint::VarTranslate(transform) => {
            let deltas = instance.var_deltas::<2>(transform.var_index_base());
            ResolvedPaint::Translate {
                dx: transform.dx().apply_float_delta(deltas[0]),
                dy: transform.dy().apply_float_delta(deltas[1]),
                paint: transform.paint()?,
            }
        }
        Paint::Scale(transform) => ResolvedPaint::Scale {
            scale_x: transform.scale_x().to_f32(),
            scale_y: transform.scale_y().to_f32(),
            around_center: None,
            paint: transform.paint()?,
        },
        Paint::VarScale(transform) => {
            let deltas = instance.var_deltas::<2>(transform.var_index_base());
            ResolvedPaint::Scale {
                scale_x: transform.scale_x().apply_float_delta(deltas[0]),
                scale_y: transform.scale_y().apply_float_delta(deltas[1]),
                around_center: None,
                paint: transform.paint()?,
            }
        }
        Paint::ScaleAroundCenter(transform) => ResolvedPaint::Scale {
            scale_x: transform.scale_x().to_f32(),
            scale_y: transform.scale_y().to_f32(),
            around_center: Some(Point::new(
                transform.center_x().to_i16() as f32,
                transform.center_y().to_i16() as f32,
            )),
            paint: transform.paint()?,
        },
        Paint::VarScaleAroundCenter(transform) => {
            let deltas = instance.var_deltas::<4>(transform.var_index_base());
            ResolvedPaint::Scale {
                scale_x: transform.scale_x().apply_float_delta(deltas[0]),
                scale_y: transform.scale_y().apply_float_delta(deltas[1]),
                around_center: Some(Point::new(
                    transform.center_x().apply_float_delta(deltas[2]),
                    transform.center_y().apply_float_delta(deltas[3]),
                )),
                paint: transform.paint()?,
            }
        }
        Paint::ScaleUniform(transform) => {
            let scale = transform.scale().to_f32();
            ResolvedPaint::Scale {
                scale_x: scale,
                scale_y: scale,
                around_center: None,
                paint: transform.paint()?,
            }
        }
        Paint::VarScaleUniform(transform) => {
            let deltas = instance.var_deltas::<1>(transform.var_index_base());
            let scale = transform.scale().apply_float_delta(deltas[0]);
            ResolvedPaint::Scale {
                scale_x: scale,
                scale_y: scale,
                around_center: None,
                paint: transform.paint()?,
            }
        }
        Paint::ScaleUniformAroundCenter(transform) => {
            let scale = transform.scale().to_f32();
            ResolvedPaint::Scale {
                scale_x: scale,
                scale_y: scale,
                around_center: Some(Point::new(
                    transform.center_x().to_i16() as f32,
                    transform.center_y().to_i16() as f32,
                )),
                paint: transform.paint()?,
            }
        }
        Paint::VarScaleUniformAroundCenter(transform) => {
            let deltas = instance.var_deltas::<3>(transform.var_index_base());
            let scale = transform.scale().apply_float_delta(deltas[0]);
            ResolvedPaint::Scale {
                scale_x: scale,
                scale_y: scale,
                around_center: Some(Point::new(
                    transform.center_x().apply_float_delta(deltas[1]),
                    transform.center_y().apply_float_delta(deltas[2]),
                )),
                paint: transform.paint()?,
            }
        }
        Paint::Rotate(transform) => ResolvedPaint::Rotate {
            angle: transform.angle().to_f32(),
            around_center: None,
            paint: transform.paint()?,
        },
        Paint::VarRotate(transform) => {
            let deltas = instance.var_deltas::<1>(transform.var_index_base());
            ResolvedPaint::Rotate {
                angle: transform.angle().apply_float_delta(deltas[0]),
                around_center: None,
                paint: transform.paint()?,
            }
        }
        Paint::RotateAroundCenter(transform) => ResolvedPaint::Rotate {
            angle: transform.angle().to_f32(),
            around_center: Some(Point::new(
                transform.center_x().to_i16() as f32,
                transform.center_y().to_i16() as f32,
            )),
            paint: transform.paint()?,
        },
        Paint::VarRotateAroundCenter(transform) => {
            let deltas = instance.var_deltas::<3>(transform.var_index_base());
            ResolvedPaint::Rotate {
                angle: transform.angle().apply_float_delta(deltas[0]),
                around_center: Some(Point::new(
                    transform.center_x().apply_float_delta(deltas[1]),
                    transform.center_y().apply_float_delta(deltas[2]),
                )),
                paint: transform.paint()?,
            }
        }
        Paint::Skew(transform) => ResolvedPaint::Skew {
            x_skew_angle: transform.x_skew_angle().to_f32(),
            y_skew_angle: transform.y_skew_angle().to_f32(),
            around_center: None,
            paint: transform.paint()?,
        },
        Paint::VarSkew(transform) => {
            let deltas = instance.var_deltas::<2>(transform.var_index_base());
            ResolvedPaint::Skew {
                x_skew_angle: transform.x_skew_angle().apply_float_delta(deltas[0]),
                y_skew_angle: transform.y_skew_angle().apply_float_delta(deltas[1]),
                around_center: None,
                paint: transform.paint()?,
            }
        }
        Paint::SkewAroundCenter(transform) => ResolvedPaint::Skew {
            x_skew_angle: transform.x_skew_angle().to_f32(),
            y_skew_angle: transform.y_skew_angle().to_f32(),
            around_center: Some(Point::new(
                transform.center_x().to_i16() as f32,
                transform.center_y().to_i16() as f32,
            )),
            paint: transform.paint()?,
        },
        Paint::VarSkewAroundCenter(transform) => {
            let deltas = instance.var_deltas::<4>(transform.var_index_base());
            ResolvedPaint::Skew {
                x_skew_angle: transform.x_skew_angle().apply_float_delta(deltas[0]),
                y_skew_angle: transform.y_skew_angle().apply_float_delta(deltas[1]),
                around_center: Some(Point::new(
                    transform.center_x().apply_float_delta(deltas[2]),
                    transform.center_y().apply_float_delta(deltas[3]),
                )),
                paint: transform.paint()?,
            }
        }
        Paint::Composite(composite) => ResolvedPaint::Composite {
            source_paint: composite.source_paint()?,
            mode: composite.composite_mode(),
            backdrop_paint: composite.backdrop_paint()?,
        },
    })
}
