use anyhow::Error;
use etagere::euclid::{Point2D, Vector2D};
use lyon::geom::{Angle, CubicBezierSegment};
use lyon::math::{Vector, vector};
use lyon::path::traits::SvgPathBuilder;
use lyon::path::{ArcFlags, Event, Polygon};

pub use lyon::math::Transform;
pub use lyon::tessellation::{FillOptions, FillRule, LineCap, LineJoin, StrokeOptions};

use crate::{Path, Pixels, Point, point, px};

/// Style of the PathBuilder
pub enum PathStyle {
    /// Stroke style. Building expands the stroke into a filled outline
    /// (caps, joins, miter limits, and dashing included), so stroked paths
    /// render through the same fill pipeline as everything else. Honored
    /// [`StrokeOptions`]: `line_width`, `line_join`, `miter_limit`, and
    /// `start_cap` (applied to both ends — `end_cap` is ignored).
    /// `tolerance` and `variable_line_width` are ignored; the expansion
    /// uses a fixed internal tolerance. A zero-width stroke renders
    /// nothing.
    Stroke(StrokeOptions),
    /// Fill style. Only [`FillOptions::fill_rule`] is honored: fills are
    /// rendered analytically from the exact curves, so the tessellation
    /// tolerance, sweep orientation, and intersection handling that
    /// [`FillOptions`] carries for lyon's tessellator have nothing to
    /// configure. (Cubic Béziers are approximated by quadratic chains at a
    /// fixed internal tolerance, independent of these options.)
    Fill(FillOptions),
}

/// A [`Path`] builder.
pub struct PathBuilder {
    raw: lyon::path::builder::WithSvg<lyon::path::BuilderImpl>,
    transform: Option<lyon::math::Transform>,
    /// PathStyle of the PathBuilder
    pub style: PathStyle,
    dash_array: Option<Vec<Pixels>>,
}

impl From<lyon::path::Builder> for PathBuilder {
    fn from(builder: lyon::path::Builder) -> Self {
        Self {
            raw: builder.with_svg(),
            ..Default::default()
        }
    }
}

impl From<lyon::path::builder::WithSvg<lyon::path::BuilderImpl>> for PathBuilder {
    fn from(raw: lyon::path::builder::WithSvg<lyon::path::BuilderImpl>) -> Self {
        Self {
            raw,
            ..Default::default()
        }
    }
}

impl From<lyon::math::Point> for Point<Pixels> {
    fn from(p: lyon::math::Point) -> Self {
        point(px(p.x), px(p.y))
    }
}

impl From<Point<Pixels>> for lyon::math::Point {
    fn from(p: Point<Pixels>) -> Self {
        lyon::math::point(p.x.0, p.y.0)
    }
}

impl From<Point<Pixels>> for Vector {
    fn from(p: Point<Pixels>) -> Self {
        vector(p.x.0, p.y.0)
    }
}

impl From<Point<Pixels>> for Point2D<f32, Pixels> {
    fn from(p: Point<Pixels>) -> Self {
        Point2D::new(p.x.0, p.y.0)
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self {
            raw: lyon::path::Path::builder().with_svg(),
            style: PathStyle::Fill(FillOptions::default()),
            transform: None,
            dash_array: None,
        }
    }
}

impl PathBuilder {
    /// Creates a new [`PathBuilder`] to build a Stroke path.
    pub fn stroke(width: Pixels) -> Self {
        Self {
            style: PathStyle::Stroke(StrokeOptions::default().with_line_width(width.0)),
            ..Self::default()
        }
    }

    /// Creates a new [`PathBuilder`] to build a Fill path.
    pub fn fill() -> Self {
        Self::default()
    }

    /// Sets the style of the [`PathBuilder`].
    pub fn with_style(self, style: PathStyle) -> Self {
        Self { style, ..self }
    }

    /// Sets the dash array of the [`PathBuilder`]. Only stroked paths
    /// dash; fills ignore it.
    ///
    /// [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/stroke-dasharray)
    pub fn dash_array(mut self, dash_array: &[Pixels]) -> Self {
        // If an odd number of values is provided, then the list of values is repeated to yield an even number of values.
        // Thus, 5,3,2 is equivalent to 5,3,2,5,3,2.
        let array = if dash_array.len() % 2 == 1 {
            let mut new_dash_array = dash_array.to_vec();
            new_dash_array.extend_from_slice(dash_array);
            new_dash_array
        } else {
            dash_array.to_vec()
        };

        self.dash_array = Some(array);
        self
    }

    /// Move the current point to the given point.
    #[inline]
    pub fn move_to(&mut self, to: Point<Pixels>) {
        self.raw.move_to(to.into());
    }

    /// Draw a straight line from the current point to the given point.
    #[inline]
    pub fn line_to(&mut self, to: Point<Pixels>) {
        self.raw.line_to(to.into());
    }

    /// Draw a curve from the current point to the given point, using the given control point.
    #[inline]
    pub fn curve_to(&mut self, to: Point<Pixels>, ctrl: Point<Pixels>) {
        self.raw.quadratic_bezier_to(ctrl.into(), to.into());
    }

    /// Adds a cubic Bézier to the [`Path`] given its two control points
    /// and its end point.
    #[inline]
    pub fn cubic_bezier_to(
        &mut self,
        to: Point<Pixels>,
        control_a: Point<Pixels>,
        control_b: Point<Pixels>,
    ) {
        self.raw
            .cubic_bezier_to(control_a.into(), control_b.into(), to.into());
    }

    /// Adds an elliptical arc.
    pub fn arc_to(
        &mut self,
        radii: Point<Pixels>,
        x_rotation: Pixels,
        large_arc: bool,
        sweep: bool,
        to: Point<Pixels>,
    ) {
        self.raw.arc_to(
            radii.into(),
            Angle::degrees(x_rotation.into()),
            ArcFlags { large_arc, sweep },
            to.into(),
        );
    }

    /// Equivalent to `arc_to` in relative coordinates.
    pub fn relative_arc_to(
        &mut self,
        radii: Point<Pixels>,
        x_rotation: Pixels,
        large_arc: bool,
        sweep: bool,
        to: Point<Pixels>,
    ) {
        self.raw.relative_arc_to(
            radii.into(),
            Angle::degrees(x_rotation.into()),
            ArcFlags { large_arc, sweep },
            to.into(),
        );
    }

    /// Adds a polygon.
    pub fn add_polygon(&mut self, points: &[Point<Pixels>], closed: bool) {
        let points = points.iter().copied().map(|p| p.into()).collect::<Vec<_>>();
        self.raw.add_polygon(Polygon {
            points: points.as_ref(),
            closed,
        });
    }

    /// Close the current sub-path.
    #[inline]
    pub fn close(&mut self) {
        self.raw.close();
    }

    /// Applies a transform to the path.
    #[inline]
    pub fn transform(&mut self, transform: Transform) {
        self.transform = Some(transform);
    }

    /// Applies a translation to the path.
    #[inline]
    pub fn translate(&mut self, to: Point<Pixels>) {
        if let Some(transform) = self.transform {
            self.transform = Some(transform.then_translate(Vector2D::new(to.x.0, to.y.0)));
        } else {
            self.transform = Some(Transform::translation(to.x.0, to.y.0))
        }
    }

    /// Applies a scale to the path.
    #[inline]
    pub fn scale(&mut self, scale: f32) {
        if let Some(transform) = self.transform {
            self.transform = Some(transform.then_scale(scale, scale));
        } else {
            self.transform = Some(Transform::scale(scale, scale));
        }
    }

    /// Applies a rotation to the path.
    ///
    /// The `angle` is in degrees value in the range 0.0 to 360.0.
    #[inline]
    pub fn rotate(&mut self, angle: f32) {
        let radians = angle.to_radians();
        if let Some(transform) = self.transform {
            self.transform = Some(transform.then_rotate(Angle::radians(radians)));
        } else {
            self.transform = Some(Transform::rotation(Angle::radians(radians)));
        }
    }

    /// Builds into a [`Path`].
    #[inline]
    pub fn build(self) -> Result<Path, Error> {
        let path = if let Some(transform) = self.transform {
            self.raw.build().transformed(&transform)
        } else {
            self.raw.build()
        };

        match self.style {
            PathStyle::Stroke(options) => Ok(Self::stroke_outline(
                &path,
                &options,
                self.dash_array.as_deref(),
            )),
            PathStyle::Fill(options) => Ok(Self::fill_outline(&path, options.fill_rule)),
        }
    }

    /// Expand a stroke into the closed contours outlining it — tiny-skia's
    /// port of Skia's stroker does the computational geometry (offset
    /// curves, caps, joins, dashing) — and convert the outline into an
    /// ordinary fill path. The stroker orients inner and outer contours so
    /// the nonzero rule fills exactly the stroked region, overlapping
    /// joins and dashes unioning via winding.
    fn stroke_outline(
        path: &lyon::path::Path,
        options: &StrokeOptions,
        dash_array: Option<&[Pixels]>,
    ) -> Path {
        let outline = skia_path(path)
            .map(|skia_path| match dash_array {
                Some(dash_array) => {
                    let array = dash_array.iter().map(|length| length.0).collect();
                    match tiny_skia_path::StrokeDash::new(array, 0.0) {
                        Some(dash) => skia_path
                            .dash(&dash, STROKE_RESOLUTION_SCALE)
                            .unwrap_or(skia_path),
                        // An invalid dash array (nonpositive total) strokes
                        // undashed, matching SVG.
                        None => skia_path,
                    }
                }
                None => skia_path,
            })
            .and_then(|skia_path| {
                skia_path.stroke(
                    &tiny_skia_path::Stroke {
                        width: options.line_width,
                        miter_limit: options.miter_limit,
                        line_cap: match options.start_cap {
                            LineCap::Butt => tiny_skia_path::LineCap::Butt,
                            LineCap::Square => tiny_skia_path::LineCap::Square,
                            LineCap::Round => tiny_skia_path::LineCap::Round,
                        },
                        line_join: match options.line_join {
                            LineJoin::Miter => tiny_skia_path::LineJoin::Miter,
                            LineJoin::MiterClip => tiny_skia_path::LineJoin::MiterClip,
                            LineJoin::Round => tiny_skia_path::LineJoin::Round,
                            LineJoin::Bevel => tiny_skia_path::LineJoin::Bevel,
                        },
                        dash: None,
                    },
                    STROKE_RESOLUTION_SCALE,
                )
            });
        let Some(outline) = outline else {
            // Empty geometry or a zero-width stroke outlines nothing.
            return Path::new(Point::default());
        };

        let mut output: Option<Path> = None;
        // `PathSegment` carries no `from` point, but cubic flattening
        // needs one.
        let mut current = lyon::math::point(0.0, 0.0);
        for segment in outline.segments() {
            match segment {
                tiny_skia_path::PathSegment::MoveTo(p) => {
                    let to = lyon::math::point(p.x, p.y);
                    match &mut output {
                        Some(output) => output.move_to(to.into()),
                        None => output = Some(Path::new(to.into())),
                    }
                    current = to;
                }
                tiny_skia_path::PathSegment::LineTo(p) => {
                    let to = lyon::math::point(p.x, p.y);
                    if let Some(output) = &mut output {
                        output.line_to(to.into());
                    }
                    current = to;
                }
                tiny_skia_path::PathSegment::QuadTo(ctrl, p) => {
                    let to = lyon::math::point(p.x, p.y);
                    if let Some(output) = &mut output {
                        output.curve_to(to.into(), lyon::math::point(ctrl.x, ctrl.y).into());
                    }
                    current = to;
                }
                tiny_skia_path::PathSegment::CubicTo(ctrl1, ctrl2, p) => {
                    let to = lyon::math::point(p.x, p.y);
                    if let Some(output) = &mut output {
                        let cubic = CubicBezierSegment {
                            from: current,
                            ctrl1: lyon::math::point(ctrl1.x, ctrl1.y),
                            ctrl2: lyon::math::point(ctrl2.x, ctrl2.y),
                            to,
                        };
                        cubic.for_each_quadratic_bezier(PATH_FLATTEN_TOLERANCE, &mut |quadratic| {
                            output.curve_to(quadratic.to.into(), quadratic.ctrl.into());
                        });
                    }
                    current = to;
                }
                tiny_skia_path::PathSegment::Close => {
                    if let Some(output) = &mut output {
                        output.close();
                    }
                }
            }
        }
        // The stroker's contour orientations assume the nonzero rule;
        // `Path::new` already defaults to it.
        output.unwrap_or_else(|| Path::new(Point::default()))
    }

    /// Convert a lyon path into contours of quadratic segments; every subpath
    /// is treated as closed, matching fill semantics.
    fn fill_outline(path: &lyon::path::Path, fill_rule: FillRule) -> Path {
        let mut output: Option<Path> = None;
        for event in path.iter() {
            match event {
                Event::Begin { at } => match &mut output {
                    Some(output) => output.move_to(at.into()),
                    None => output = Some(Path::new(at.into())),
                },
                Event::Line { to, .. } => {
                    if let Some(output) = &mut output {
                        output.line_to(to.into());
                    }
                }
                Event::Quadratic { ctrl, to, .. } => {
                    if let Some(output) = &mut output {
                        output.curve_to(to.into(), ctrl.into());
                    }
                }
                Event::Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                } => {
                    if let Some(output) = &mut output {
                        let cubic = CubicBezierSegment {
                            from,
                            ctrl1,
                            ctrl2,
                            to,
                        };
                        // Cubics are approximated by quadratic chains (the
                        // shader evaluates quadratics); this is the only
                        // approximation baked into a built fill path.
                        cubic.for_each_quadratic_bezier(PATH_FLATTEN_TOLERANCE, &mut |quadratic| {
                            output.curve_to(quadratic.to.into(), quadratic.ctrl.into());
                        });
                    }
                }
                Event::End { .. } => {
                    if let Some(output) = &mut output {
                        output.close();
                    }
                }
            }
        }
        let mut path = output.unwrap_or_else(|| Path::new(Point::default()));
        path.fill_rule = fill_rule;
        path
    }
}

/// Maximum deviation (in pixels, before display scaling) allowed of the
/// cubic-to-quadratic conversion in `PathBuilder`, the only approximation
/// baked into a built fill path. See `docs/trapezoid_path_rendering.md` for
/// the measurements behind the value.
const PATH_FLATTEN_TOLERANCE: f32 = 0.25;

/// Resolution scale passed to tiny-skia's stroker and dasher, whose
/// approximation error is about `0.25 / scale` in input units. Strokes are
/// expanded once in logical pixels and repainted at any display scale, so
/// 2.0 keeps the error within [`PATH_FLATTEN_TOLERANCE`] device pixels up
/// to 2x displays.
const STROKE_RESOLUTION_SCALE: f32 = 2.0;

/// Convert a lyon path into a tiny-skia path for stroke expansion.
/// Returns [`None`] for empty geometry.
fn skia_path(path: &lyon::path::Path) -> Option<tiny_skia_path::Path> {
    let mut builder = tiny_skia_path::PathBuilder::new();
    for event in path.iter() {
        match event {
            Event::Begin { at } => builder.move_to(at.x, at.y),
            Event::Line { to, .. } => builder.line_to(to.x, to.y),
            Event::Quadratic { ctrl, to, .. } => builder.quad_to(ctrl.x, ctrl.y, to.x, to.y),
            Event::Cubic {
                ctrl1, ctrl2, to, ..
            } => builder.cubic_to(ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y),
            Event::End { close, .. } => {
                if close {
                    builder.close();
                }
            }
        }
    }
    builder.finish()
}
