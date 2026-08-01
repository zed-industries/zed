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
    /// Stroke style. Currently renders nothing: strokes are pending their
    /// own instance design (see [`PathBuilder::build`]).
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

    /// Sets the dash array of the [`PathBuilder`].
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
    pub fn build(self) -> Result<Path<Pixels>, Error> {
        let path = if let Some(transform) = self.transform {
            self.raw.build().transformed(&transform)
        } else {
            self.raw.build()
        };

        match self.style {
            // Strokes are pending their own instance design (SDF capsules);
            // routing them through the fill pipeline would be thrown away, so
            // they render nothing for now.
            PathStyle::Stroke(_) => Ok(Path::new(Point::default())),
            PathStyle::Fill(options) => {
                let mut path = Self::fill_outline(&path, options.fill_rule);
                // Do the expensive, scale-independent geometry work once at
                // build time; painting the built path only bins it.
                path.ensure_decomposition();
                Ok(path)
            }
        }
    }

    /// Convert a lyon path into contours of quadratic segments; every subpath
    /// is treated as closed, matching fill semantics.
    fn fill_outline(path: &lyon::path::Path, fill_rule: FillRule) -> Path<Pixels> {
        let mut output: Option<Path<Pixels>> = None;
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
                        cubic.for_each_quadratic_bezier(
                            crate::scene::PATH_FLATTEN_TOLERANCE,
                            &mut |quadratic| {
                                output.curve_to(quadratic.to.into(), quadratic.ctrl.into());
                            },
                        );
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
