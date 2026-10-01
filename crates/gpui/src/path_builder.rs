use anyhow::Error;
use etagere::euclid::{Point2D, Vector2D};
use lyon_path::geom::Angle;
use lyon_path::math::{Vector, vector};
use lyon_path::traits::SvgPathBuilder;
use lyon_path::{ArcFlags, Event, Polygon};

pub use lyon_path::math::Transform;
pub use lyon_path::{FillRule, LineCap, LineJoin};

use crate::{Outline, Path, PathCurve, Pixels, Point, point, px};

/// Style of the PathBuilder
pub enum PathStyle {
    /// Stroke style
    Stroke(StrokeOptions),
    /// Fill style
    Fill(FillOptions),
}

/// How to stroke a path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrokeOptions {
    /// Width of the stroke.
    pub line_width: f32,
    /// Shape of the ends of open subpaths and dashes.
    pub line_cap: LineCap,
    /// Shape of corners.
    pub line_join: LineJoin,
    /// How long a miter join may get, as a multiple of the line width,
    /// before it is beveled instead.
    pub miter_limit: f32,
}

impl Default for StrokeOptions {
    fn default() -> Self {
        Self {
            line_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 4.0,
        }
    }
}

/// How to fill a path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FillOptions {
    /// How overlapping contours combine into the filled region.
    pub fill_rule: FillRule,
}

impl Default for FillOptions {
    fn default() -> Self {
        Self {
            fill_rule: FillRule::EvenOdd,
        }
    }
}

/// A [`Path`] builder.
pub struct PathBuilder {
    raw: lyon_path::builder::WithSvg<lyon_path::BuilderImpl>,
    transform: Option<lyon_path::math::Transform>,
    /// PathStyle of the PathBuilder
    pub style: PathStyle,
    dash_array: Option<Vec<Pixels>>,
}

impl From<lyon_path::Builder> for PathBuilder {
    fn from(builder: lyon_path::Builder) -> Self {
        Self {
            raw: builder.with_svg(),
            ..Default::default()
        }
    }
}

impl From<lyon_path::builder::WithSvg<lyon_path::BuilderImpl>> for PathBuilder {
    fn from(raw: lyon_path::builder::WithSvg<lyon_path::BuilderImpl>) -> Self {
        Self {
            raw,
            ..Default::default()
        }
    }
}

impl From<lyon_path::math::Point> for Point<Pixels> {
    fn from(p: lyon_path::math::Point) -> Self {
        point(px(p.x), px(p.y))
    }
}

impl From<Point<Pixels>> for lyon_path::math::Point {
    fn from(p: Point<Pixels>) -> Self {
        lyon_path::math::point(p.x.0, p.y.0)
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
            raw: lyon_path::Path::builder().with_svg(),
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
            style: PathStyle::Stroke(StrokeOptions {
                line_width: width.0,
                ..Default::default()
            }),
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

    /// Sets the dash array of the [`PathBuilder`]. Fills ignore it.
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
    pub fn build(self) -> Result<Path, Error> {
        let (curves, fill_rule) = self.outline();
        Ok(Path::from_curves(curves, fill_rule))
    }

    /// The closed contours to fill, and the rule to fill them by.
    pub(crate) fn outline(self) -> (Vec<PathCurve>, FillRule) {
        let path = if let Some(transform) = self.transform {
            self.raw.build().transformed(&transform)
        } else {
            self.raw.build()
        };

        match self.style {
            PathStyle::Stroke(options) => (
                Self::stroke_outline(&path, &options, self.dash_array.as_deref()),
                FillRule::NonZero,
            ),
            PathStyle::Fill(options) => (Self::fill_outline(&path), options.fill_rule),
        }
    }

    fn stroke_outline(
        path: &lyon_path::Path,
        options: &StrokeOptions,
        dash_array: Option<&[Pixels]>,
    ) -> Vec<PathCurve> {
        let outline = skia_path(path)
            .map(|skia_path| match dash_array {
                Some(dash_array) => {
                    let array = dash_array.iter().map(|length| length.0).collect();
                    match tiny_skia_path::StrokeDash::new(array, 0.0) {
                        Some(dash) => skia_path
                            .dash(&dash, STROKE_RESOLUTION_SCALE)
                            .unwrap_or(skia_path),
                        // An invalid dash array strokes undashed, as in SVG.
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
                        line_cap: match options.line_cap {
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
            return Vec::new();
        };

        let mut output = Outline::default();
        let at = |p: tiny_skia_path::Point| point(p.x, p.y);
        for segment in outline.segments() {
            match segment {
                tiny_skia_path::PathSegment::MoveTo(p) => output.move_to(at(p)),
                tiny_skia_path::PathSegment::LineTo(p) => output.line_to(at(p)),
                tiny_skia_path::PathSegment::QuadTo(ctrl, p) => output.curve_to(at(p), at(ctrl)),
                tiny_skia_path::PathSegment::CubicTo(ctrl1, ctrl2, p) => {
                    output.cubic_to(at(p), at(ctrl1), at(ctrl2))
                }
                tiny_skia_path::PathSegment::Close => output.close(),
            }
        }
        output.into_curves()
    }

    /// Convert a lyon path into a fill, closing every subpath.
    fn fill_outline(path: &lyon_path::Path) -> Vec<PathCurve> {
        let mut output = Outline::default();
        let at = |p: lyon_path::math::Point| point(p.x, p.y);
        for event in path.iter() {
            match event {
                Event::Begin { at: p } => output.move_to(at(p)),
                Event::Line { to, .. } => output.line_to(at(to)),
                Event::Quadratic { ctrl, to, .. } => output.curve_to(at(to), at(ctrl)),
                Event::Cubic {
                    ctrl1, ctrl2, to, ..
                } => output.cubic_to(at(to), at(ctrl1), at(ctrl2)),
                Event::End { .. } => output.close(),
            }
        }
        output.into_curves()
    }
}

/// tiny-skia's stroker errs by about `0.25 / scale`. Strokes are expanded in
/// logical pixels, so 2.0 keeps that within a quarter device pixel on
/// displays up to 2x.
const STROKE_RESOLUTION_SCALE: f32 = 2.0;

/// Returns [`None`] for empty geometry.
fn skia_path(path: &lyon_path::Path) -> Option<tiny_skia_path::Path> {
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
