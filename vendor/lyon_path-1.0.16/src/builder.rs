//! Path building utilities.
//!
//! ## `PathBuilder` or `SvgPathBuilder`
//!
//! Path can be built via either of two abstractions:
//!
//! - [PathBuilder](trait.PathBuilder.html) is a simple and efficient interface which
//!   does not deal with any ambiguous cases.
//! - [SvgPathBuilder](trait.SvgPathBuilder.html) is a higher-level interface that
//!   follows SVG's specification, removing the the burden of dealing with special cases
//!   from the user at a run-time cost.
//!
//! `SvgPathBuilder` may be a better choice when interactive with SVG, or dealing with arbitrary
//! input. `PathBuilder`. `PathBuilder` is probably a more useful trait to implement when creating
//! a new path data structure since all `PathBuilder` implementations automatically get an
//! `SvgPathBuilder` adapter (see the `with_svg` method). It may also make sense to use the
//! `PathBuilder` API when following a specification that behaves like SVG paths or when no
//! performance can be traded for convenience.
//!
//! ## Examples
//!
//! The following example shows how to create a simple path using the
//! [PathBuilder](trait.PathBuilder.html) interface.
//!
//! ```
//! use lyon_path::{Path, geom::point};
//!
//! let mut builder = Path::builder();
//!
//! // All sub-paths *must* have be contained in a begin/end pair.
//! builder.begin(point(0.0, 0.0));
//! builder.line_to(point(1.0, 0.0));
//! builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
//! builder.end(false);
//!
//! builder.begin(point(10.0, 0.0));
//! builder.cubic_bezier_to(point(12.0, 2.0), point(11.0, 2.0), point(5.0, 0.0));
//! builder.close(); // close() is equivalent to end(true).
//!
//! let path = builder.build();
//! ```
//!
//! The same path can be built using the `SvgPathBuilder` API:
//!
//! ```
//! use lyon_path::{Path, geom::{point, vector}, builder::SvgPathBuilder};
//!
//! // Use the SVG adapter.
//! let mut builder = Path::builder().with_svg();
//!
//! // All sub-paths *must* have be contained in a begin/end pair.
//! builder.move_to(point(0.0, 0.0));
//! builder.line_to(point(1.0, 0.0));
//! builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
//! // No need to explicitly end a sub-path.
//!
//! builder.move_to(point(10.0, 0.0));
//! builder.relative_cubic_bezier_to(vector(2.0, 2.0), vector(1.0, 2.0), vector(-5.0, 0.0));
//! builder.close();
//!
//! let path = builder.build();
//! ```
//!
//! Implementors of the `PathBuilder` trait automatically gain access to a few other adapters.
//! For example a builder that approximates curves with a sequence of line segments:
//!
//! ```
//! use lyon_path::{Path, geom::point};
//!
//! let tolerance = 0.05;// maximum distance between a curve and its approximation.
//! let mut builder = Path::builder().flattened(tolerance);
//!
//! builder.begin(point(0.0, 0.0));
//! builder.quadratic_bezier_to(point(1.0, 0.0), point(1.0, 1.0));
//! builder.end(true);
//!
//! // The resulting path contains only Begin, Line and End events.
//! let path = builder.build();
//! ```
//!

use crate::events::{Event, PathEvent};
use crate::geom::{traits::Transformation, Arc, ArcFlags, LineSegment, SvgArc};
use crate::math::*;
use crate::path::Verb;
use crate::polygon::Polygon;
use crate::{Attributes, EndpointId, Winding, NO_ATTRIBUTES};

use core::f32::consts::PI;
use core::marker::Sized;

use alloc::vec;
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use num_traits::Float;

/// The radius of each corner of a rounded rectangle.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct BorderRadii {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl BorderRadii {
    pub fn new(radius: f32) -> Self {
        let r = radius.abs();
        BorderRadii {
            top_left: r,
            top_right: r,
            bottom_left: r,
            bottom_right: r,
        }
    }
}

impl core::fmt::Display for BorderRadii {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // In the order of a well known convention (CSS) clockwise from top left
        write!(
            f,
            "BorderRadii({}, {}, {}, {})",
            self.top_left, self.top_right, self.bottom_left, self.bottom_right
        )
    }
}

/// A convenience wrapper for `PathBuilder` without custom attributes.
///
/// See the [PathBuilder] trait.
///
/// This simply forwards to an underlying `PathBuilder` implementation,
/// using no attributes.
#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct NoAttributes<B: PathBuilder> {
    pub(crate) inner: B,
}

impl<B: PathBuilder> NoAttributes<B> {
    #[inline]
    pub fn wrap(inner: B) -> Self {
        assert_eq!(inner.num_attributes(), 0);
        NoAttributes { inner }
    }

    pub fn new() -> Self
    where
        B: Default,
    {
        NoAttributes::wrap(B::default())
    }

    pub fn with_capacity(endpoints: usize, ctrl_points: usize) -> Self
    where
        B: Default,
    {
        let mut builder = B::default();
        builder.reserve(endpoints, ctrl_points);
        NoAttributes::wrap(builder)
    }

    /// Starts a new sub-path at a given position.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// `at` becomes the current position of the sub-path.
    #[inline]
    pub fn begin(&mut self, at: Point) -> EndpointId {
        self.inner.begin(at, NO_ATTRIBUTES)
    }

    /// Ends the current sub path.
    ///
    /// A sub-path must be in progress when this method is called.
    /// After this method is called, there is no sub-path in progress until
    /// `begin` is called again.
    #[inline]
    pub fn end(&mut self, close: bool) {
        self.inner.end(close);
    }

    /// Closes the current sub path.
    ///
    /// Shorthand for `builder.end(true)`.
    #[inline]
    pub fn close(&mut self) {
        self.inner.close();
    }

    /// Adds a line segment to the current sub-path.
    ///
    /// A sub-path must be in progress when this method is called.
    #[inline]
    pub fn line_to(&mut self, to: Point) -> EndpointId {
        self.inner.line_to(to, NO_ATTRIBUTES)
    }

    /// Adds a quadratic bézier curve to the current sub-path.
    ///
    /// A sub-path must be in progress when this method is called.
    #[inline]
    pub fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point) -> EndpointId {
        self.inner.quadratic_bezier_to(ctrl, to, NO_ATTRIBUTES)
    }

    /// Adds a cubic bézier curve to the current sub-path.
    ///
    /// A sub-path must be in progress when this method is called.
    #[inline]
    pub fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) -> EndpointId {
        self.inner.cubic_bezier_to(ctrl1, ctrl2, to, NO_ATTRIBUTES)
    }

    /// Hints at the builder that a certain number of endpoints and control
    /// points will be added.
    ///
    /// The Builder implementation may use this information to pre-allocate
    /// memory as an optimization.
    #[inline]
    pub fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.inner.reserve(endpoints, ctrl_points);
    }

    /// Applies the provided path event.
    ///
    /// By default this calls one of `begin`, `end`, `line`, `quadratic_bezier_segment`,
    /// or `cubic_bezier_segment` according to the path event.
    ///
    /// The requirements for each method apply to the corresponding event.
    #[inline]
    pub fn path_event(&mut self, event: PathEvent) {
        self.inner.path_event(event, NO_ATTRIBUTES);
    }

    /// Adds a sub-path from a polygon.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    #[inline]
    pub fn add_polygon(&mut self, polygon: Polygon<Point>) {
        self.inner.add_polygon(polygon, NO_ATTRIBUTES);
    }

    /// Adds a sub-path containing a single point.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    #[inline]
    pub fn add_point(&mut self, at: Point) -> EndpointId {
        self.inner.add_point(at, NO_ATTRIBUTES)
    }

    /// Adds a sub-path containing a single line segment.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    #[inline]
    pub fn add_line_segment(&mut self, line: &LineSegment<f32>) -> (EndpointId, EndpointId) {
        self.inner.add_line_segment(line, NO_ATTRIBUTES)
    }

    /// Adds a sub-path containing an ellipse.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    #[inline]
    pub fn add_ellipse(
        &mut self,
        center: Point,
        radii: Vector,
        x_rotation: Angle,
        winding: Winding,
    ) {
        self.inner
            .add_ellipse(center, radii, x_rotation, winding, NO_ATTRIBUTES);
    }

    /// Adds a sub-path containing a circle.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    #[inline]
    pub fn add_circle(&mut self, center: Point, radius: f32, winding: Winding)
    where
        B: Sized,
    {
        self.inner
            .add_circle(center, radius, winding, NO_ATTRIBUTES);
    }

    /// Adds a sub-path containing a rectangle.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    #[inline]
    pub fn add_rectangle(&mut self, rect: &Box2D, winding: Winding) {
        self.inner.add_rectangle(rect, winding, NO_ATTRIBUTES);
    }

    /// Adds a sub-path containing a rectangle.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    #[inline]
    pub fn add_rounded_rectangle(&mut self, rect: &Box2D, radii: &BorderRadii, winding: Winding)
    where
        B: Sized,
    {
        self.inner
            .add_rounded_rectangle(rect, radii, winding, NO_ATTRIBUTES);
    }

    /// Returns a builder that approximates all curves with sequences of line segments.
    #[inline]
    pub fn flattened(self, tolerance: f32) -> NoAttributes<Flattened<B>>
    where
        B: Sized,
    {
        NoAttributes {
            inner: Flattened::new(self.inner, tolerance),
        }
    }

    /// Returns a builder that applies the given transformation to all positions.
    #[inline]
    pub fn transformed<Transform>(
        self,
        transform: Transform,
    ) -> NoAttributes<Transformed<B, Transform>>
    where
        B: Sized,
        Transform: Transformation<f32>,
    {
        NoAttributes {
            inner: Transformed::new(self.inner, transform),
        }
    }

    /// Returns a builder that support SVG commands.
    ///
    /// This must be called before starting to add any sub-path.
    #[inline]
    pub fn with_svg(self) -> WithSvg<B>
    where
        B: Sized,
    {
        WithSvg::new(self.inner)
    }

    /// Builds a path object, consuming the builder.
    #[inline]
    pub fn build<P>(self) -> P
    where
        B: Build<PathType = P>,
    {
        self.inner.build()
    }

    #[inline]
    pub fn inner(&self) -> &B {
        &self.inner
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut B {
        &mut self.inner
    }

    #[inline]
    pub fn into_inner(self) -> B {
        self.inner
    }
}

impl<B: PathBuilder> PathBuilder for NoAttributes<B> {
    #[inline]
    fn num_attributes(&self) -> usize {
        0
    }

    #[inline]
    fn begin(&mut self, at: Point, _attributes: Attributes) -> EndpointId {
        self.inner.begin(at, NO_ATTRIBUTES)
    }

    #[inline]
    fn end(&mut self, close: bool) {
        self.inner.end(close);
    }

    #[inline]
    fn line_to(&mut self, to: Point, _attributes: Attributes) -> EndpointId {
        self.inner.line_to(to, NO_ATTRIBUTES)
    }

    #[inline]
    fn quadratic_bezier_to(
        &mut self,
        ctrl: Point,
        to: Point,
        _attributes: Attributes,
    ) -> EndpointId {
        self.inner.quadratic_bezier_to(ctrl, to, NO_ATTRIBUTES)
    }

    #[inline]
    fn cubic_bezier_to(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        _attributes: Attributes,
    ) -> EndpointId {
        self.inner.cubic_bezier_to(ctrl1, ctrl2, to, NO_ATTRIBUTES)
    }

    #[inline]
    fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.inner.reserve(endpoints, ctrl_points)
    }
}

impl<B: PathBuilder + Build> Build for NoAttributes<B> {
    type PathType = B::PathType;

    fn build(self) -> B::PathType {
        self.inner.build()
    }
}

impl<B: PathBuilder + Default> Default for NoAttributes<B> {
    fn default() -> Self {
        Self::new()
    }
}

/// The base path building interface.
///
/// Unlike `SvgPathBuilder`, this interface strictly requires sub-paths to be manually
/// started and ended (See the `begin` and `end` methods).
/// All positions are provided in absolute coordinates.
///
/// The goal of this interface is to abstract over simple and fast implementations that
/// do not deal with corner cases such as adding segments without starting a sub-path.
///
/// More elaborate interfaces are built on top of the provided primitives. In particular,
/// the `SvgPathBuilder` trait providing more permissive and richer interface is
/// automatically implemented via the `WithSvg` adapter (See the `with_svg` method).
pub trait PathBuilder {
    fn num_attributes(&self) -> usize;
    /// Starts a new sub-path at a given position.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// `at` becomes the current position of the sub-path.
    fn begin(&mut self, at: Point, custom_attributes: Attributes) -> EndpointId;

    /// Ends the current sub path.
    ///
    /// A sub-path must be in progress when this method is called.
    /// After this method is called, there is no sub-path in progress until
    /// `begin` is called again.
    fn end(&mut self, close: bool);

    /// Closes the current sub path.
    ///
    /// Shorthand for `builder.end(true)`.
    fn close(&mut self) {
        self.end(true)
    }

    /// Adds a line segment to the current sub-path.
    ///
    /// A sub-path must be in progress when this method is called.
    fn line_to(&mut self, to: Point, custom_attributes: Attributes) -> EndpointId;

    /// Adds a quadratic bézier curve to the current sub-path.
    ///
    /// A sub-path must be in progress when this method is called.
    fn quadratic_bezier_to(
        &mut self,
        ctrl: Point,
        to: Point,
        custom_attributes: Attributes,
    ) -> EndpointId;

    /// Adds a cubic bézier curve to the current sub-path.
    ///
    /// A sub-path must be in progress when this method is called.
    fn cubic_bezier_to(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        custom_attributes: Attributes,
    ) -> EndpointId;

    /// Hints at the builder that a certain number of endpoints and control
    /// points will be added.
    ///
    /// The Builder implementation may use this information to pre-allocate
    /// memory as an optimization.
    fn reserve(&mut self, _endpoints: usize, _ctrl_points: usize) {}

    /// Applies the provided path event.
    ///
    /// By default this calls one of `begin`, `end`, `line`, `quadratic_bezier_segment`,
    /// or `cubic_bezier_segment` according to the path event.
    ///
    /// The requirements for each method apply to the corresponding event.
    fn path_event(&mut self, event: PathEvent, attributes: Attributes) {
        match event {
            PathEvent::Begin { at } => {
                self.begin(at, attributes);
            }
            PathEvent::Line { to, .. } => {
                self.line_to(to, attributes);
            }
            PathEvent::Quadratic { ctrl, to, .. } => {
                self.quadratic_bezier_to(ctrl, to, attributes);
            }
            PathEvent::Cubic {
                ctrl1, ctrl2, to, ..
            } => {
                self.cubic_bezier_to(ctrl1, ctrl2, to, attributes);
            }
            PathEvent::End { close, .. } => {
                self.end(close);
            }
        }
    }

    fn event(&mut self, event: Event<(Point, Attributes), Point>) {
        match event {
            Event::Begin { at } => {
                self.begin(at.0, at.1);
            }
            Event::Line { to, .. } => {
                self.line_to(to.0, to.1);
            }
            Event::Quadratic { ctrl, to, .. } => {
                self.quadratic_bezier_to(ctrl, to.0, to.1);
            }
            Event::Cubic {
                ctrl1, ctrl2, to, ..
            } => {
                self.cubic_bezier_to(ctrl1, ctrl2, to.0, to.1);
            }
            Event::End { close, .. } => {
                self.end(close);
            }
        }
    }

    /// Adds a sub-path from a polygon.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    fn add_polygon(&mut self, polygon: Polygon<Point>, attributes: Attributes) {
        if polygon.points.is_empty() {
            return;
        }

        self.reserve(polygon.points.len(), 0);

        self.begin(polygon.points[0], attributes);
        for p in &polygon.points[1..] {
            self.line_to(*p, attributes);
        }

        self.end(polygon.closed);
    }

    /// Adds a sub-path containing a single point.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    fn add_point(&mut self, at: Point, attributes: Attributes) -> EndpointId {
        let id = self.begin(at, attributes);
        self.end(false);

        id
    }

    /// Adds a sub-path containing a single line segment.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    fn add_line_segment(
        &mut self,
        line: &LineSegment<f32>,
        attributes: Attributes,
    ) -> (EndpointId, EndpointId) {
        let a = self.begin(line.from, attributes);
        let b = self.line_to(line.to, attributes);
        self.end(false);

        (a, b)
    }

    /// Adds a sub-path containing an ellipse.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    fn add_ellipse(
        &mut self,
        center: Point,
        radii: Vector,
        x_rotation: Angle,
        winding: Winding,
        attributes: Attributes,
    ) {
        let dir = match winding {
            Winding::Positive => 1.0,
            Winding::Negative => -1.0,
        };

        let arc = Arc {
            center,
            radii,
            x_rotation,
            start_angle: Angle::radians(0.0),
            sweep_angle: Angle::radians(2.0 * PI) * dir,
        };

        self.begin(arc.sample(0.0), attributes);
        arc.for_each_quadratic_bezier(&mut |curve| {
            self.quadratic_bezier_to(curve.ctrl, curve.to, attributes);
        });
        self.end(true);
    }

    /// Adds a sub-path containing a circle.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    fn add_circle(&mut self, center: Point, radius: f32, winding: Winding, attributes: Attributes)
    where
        Self: Sized,
    {
        add_circle(self, center, radius, winding, attributes);
    }

    /// Adds a sub-path containing a rectangle.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    fn add_rectangle(&mut self, rect: &Box2D, winding: Winding, attributes: Attributes) {
        match winding {
            Winding::Positive => self.add_polygon(
                Polygon {
                    points: &[
                        rect.min,
                        point(rect.max.x, rect.min.y),
                        rect.max,
                        point(rect.min.x, rect.max.y),
                    ],
                    closed: true,
                },
                attributes,
            ),
            Winding::Negative => self.add_polygon(
                Polygon {
                    points: &[
                        rect.min,
                        point(rect.min.x, rect.max.y),
                        rect.max,
                        point(rect.max.x, rect.min.y),
                    ],
                    closed: true,
                },
                attributes,
            ),
        };
    }

    /// Adds a sub-path containing a rectangle.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    fn add_rounded_rectangle(
        &mut self,
        rect: &Box2D,
        radii: &BorderRadii,
        winding: Winding,
        custom_attributes: Attributes,
    ) where
        Self: Sized,
    {
        add_rounded_rectangle(self, rect, radii, winding, custom_attributes);
    }

    /// Returns a builder that approximates all curves with sequences of line segments.
    fn flattened(self, tolerance: f32) -> Flattened<Self>
    where
        Self: Sized,
    {
        Flattened::new(self, tolerance)
    }

    /// Returns a builder that applies the given transformation to all positions.
    fn transformed<Transform>(self, transform: Transform) -> Transformed<Self, Transform>
    where
        Self: Sized,
        Transform: Transformation<f32>,
    {
        Transformed::new(self, transform)
    }

    /// Returns a builder that support SVG commands.
    ///
    /// This must be called before starting to add any sub-path.
    fn with_svg(self) -> WithSvg<Self>
    where
        Self: Sized,
    {
        WithSvg::new(self)
    }
}

/// A path building interface that tries to stay close to SVG's path specification.
/// <https://svgwg.org/specs/paths/>
///
/// Some of the wording in the documentation of this trait is borrowed from the SVG
/// specification.
///
/// Unlike `PathBuilder`, implementations of this trait are expected to deal with
/// various corners cases such as adding segments without starting a sub-path.
pub trait SvgPathBuilder {
    /// Start a new sub-path at the given position.
    ///
    /// Corresponding SVG command: `M`.
    ///
    /// This command establishes a new initial point and a new current point. The effect
    /// is as if the "pen" were lifted and moved to a new location.
    /// If a sub-path is in progress, it is ended without being closed.
    fn move_to(&mut self, to: Point);

    /// Ends the current sub-path by connecting it back to its initial point.
    ///
    /// Corresponding SVG command: `Z`.
    ///
    /// A straight line is drawn from the current point to the initial point of the
    /// current sub-path.
    /// The current position is set to the initial position of the sub-path that was
    /// closed.
    fn close(&mut self);

    /// Adds a line segment to the current sub-path.
    ///
    /// Corresponding SVG command: `L`.
    ///
    /// The segment starts at the builder's current position.
    /// If this is the very first command of the path (the builder therefore does not
    /// have a current position), the `line_to` command is replaced with a `move_to(to)`.
    fn line_to(&mut self, to: Point);

    /// Adds a quadratic bézier segment to the current sub-path.
    ///
    /// Corresponding SVG command: `Q`.
    ///
    /// The segment starts at the builder's current position.
    /// If this is the very first command of the path (the builder therefore does not
    /// have a current position), the `quadratic_bezier_to` command is replaced with
    /// a `move_to(to)`.
    fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point);

    /// Adds a cubic bézier segment to the current sub-path.
    ///
    /// Corresponding SVG command: `C`.
    ///
    /// The segment starts at the builder's current position.
    /// If this is the very first command of the path (the builder therefore does not
    /// have a current position), the `cubic_bezier_to` command is replaced with
    /// a `move_to(to)`.
    fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point);

    /// Equivalent to `move_to` in relative coordinates.
    ///
    /// Corresponding SVG command: `m`.
    ///
    /// The provided coordinates are offsets relative to the current position of
    /// the builder.
    fn relative_move_to(&mut self, to: Vector);

    /// Equivalent to `line_to` in relative coordinates.
    ///
    /// Corresponding SVG command: `l`.
    ///
    /// The provided coordinates are offsets relative to the current position of
    /// the builder.
    fn relative_line_to(&mut self, to: Vector);

    /// Equivalent to `quadratic_bezier_to` in relative coordinates.
    ///
    /// Corresponding SVG command: `q`.
    ///
    /// the provided coordinates are offsets relative to the current position of
    /// the builder.
    fn relative_quadratic_bezier_to(&mut self, ctrl: Vector, to: Vector);

    /// Equivalent to `cubic_bezier_to` in relative coordinates.
    ///
    /// Corresponding SVG command: `c`.
    ///
    /// The provided coordinates are offsets relative to the current position of
    /// the builder.
    fn relative_cubic_bezier_to(&mut self, ctrl1: Vector, ctrl2: Vector, to: Vector);

    /// Equivalent to `cubic_bezier_to` with implicit first control point.
    ///
    /// Corresponding SVG command: `S`.
    ///
    /// The first control point is assumed to be the reflection of the second
    /// control point on the previous command relative to the current point.
    /// If there is no previous command or if the previous command was not a
    /// cubic bézier segment, the first control point is coincident with
    /// the current position.
    fn smooth_cubic_bezier_to(&mut self, ctrl2: Point, to: Point);

    /// Equivalent to `smooth_cubic_bezier_to` in relative coordinates.
    ///
    /// Corresponding SVG command: `s`.
    ///
    /// The provided coordinates are offsets relative to the current position of
    /// the builder.
    fn smooth_relative_cubic_bezier_to(&mut self, ctrl2: Vector, to: Vector);

    /// Equivalent to `quadratic_bezier_to` with implicit control point.
    ///
    /// Corresponding SVG command: `T`.
    ///
    /// The control point is assumed to be the reflection of the control
    /// point on the previous command relative to the current point.
    /// If there is no previous command or if the previous command was not a
    /// quadratic bézier segment, a line segment is added instead.
    fn smooth_quadratic_bezier_to(&mut self, to: Point);

    /// Equivalent to `smooth_quadratic_bezier_to` in relative coordinates.
    ///
    /// Corresponding SVG command: `t`.
    ///
    /// The provided coordinates are offsets relative to the current position of
    /// the builder.
    fn smooth_relative_quadratic_bezier_to(&mut self, to: Vector);

    /// Adds an horizontal line segment.
    ///
    /// Corresponding SVG command: `H`.
    ///
    /// Equivalent to `line_to`, using the y coordinate of the current position.
    fn horizontal_line_to(&mut self, x: f32);

    /// Adds an horizontal line segment in relative coordinates.
    ///
    /// Corresponding SVG command: `l`.
    ///
    /// Equivalent to `line_to`, using the y coordinate of the current position.
    /// `dx` is the horizontal offset relative to the current position.
    fn relative_horizontal_line_to(&mut self, dx: f32);

    /// Adds a vertical line segment.
    ///
    /// Corresponding SVG command: `V`.
    ///
    /// Equivalent to `line_to`, using the x coordinate of the current position.
    fn vertical_line_to(&mut self, y: f32);

    /// Adds a vertical line segment in relative coordinates.
    ///
    /// Corresponding SVG command: `v`.
    ///
    /// Equivalent to `line_to`, using the y coordinate of the current position.
    /// `dy` is the horizontal offset relative to the current position.
    fn relative_vertical_line_to(&mut self, dy: f32);

    /// Adds an elliptical arc.
    ///
    /// Corresponding SVG command: `A`.
    ///
    /// The arc starts at the current point and ends at `to`.
    /// The size and orientation of the ellipse are defined by `radii` and an `x_rotation`,
    /// which indicates how the ellipse as a whole is rotated relative to the current coordinate
    /// system. The center of the ellipse is calculated automatically to satisfy the constraints
    /// imposed by the other parameters. the arc `flags` contribute to the automatic calculations
    /// and help determine how the arc is built.
    fn arc_to(&mut self, radii: Vector, x_rotation: Angle, flags: ArcFlags, to: Point);

    /// Equivalent to `arc_to` in relative coordinates.
    ///
    /// Corresponding SVG command: `a`.
    ///
    /// The provided `to` coordinates are offsets relative to the current position of
    /// the builder.
    fn relative_arc_to(&mut self, radii: Vector, x_rotation: Angle, flags: ArcFlags, to: Vector);

    /// Hints at the builder that a certain number of endpoints and control
    /// points will be added.
    ///
    /// The Builder implementation may use this information to pre-allocate
    /// memory as an optimization.
    fn reserve(&mut self, _endpoints: usize, _ctrl_points: usize) {}

    /// Adds a sub-path from a polygon.
    ///
    /// There must be no sub-path in progress when this method is called.
    /// No sub-path is in progress after the method is called.
    fn add_polygon(&mut self, polygon: Polygon<Point>) {
        if polygon.points.is_empty() {
            return;
        }

        self.reserve(polygon.points.len(), 0);

        self.move_to(polygon.points[0]);
        for p in &polygon.points[1..] {
            self.line_to(*p);
        }

        if polygon.closed {
            self.close();
        }
    }
}

/// Builds a path.
///
/// This trait is separate from `PathBuilder` and `SvgPathBuilder` to allow them to
/// be used as trait object (which isn't possible when a method returns an associated
/// type).
pub trait Build {
    /// The type of object that is created by this builder.
    type PathType;

    /// Builds a path object, consuming the builder.
    fn build(self) -> Self::PathType;
}

/// A Builder that approximates curves with successions of line segments.
pub struct Flattened<Builder> {
    builder: Builder,
    current_position: Point,
    tolerance: f32,
    prev_attributes: Vec<f32>,
    attribute_buffer: Vec<f32>,
}

impl<Builder: Build> Build for Flattened<Builder> {
    type PathType = Builder::PathType;

    fn build(self) -> Builder::PathType {
        self.builder.build()
    }
}

impl<Builder: PathBuilder> PathBuilder for Flattened<Builder> {
    fn num_attributes(&self) -> usize {
        self.builder.num_attributes()
    }

    fn begin(&mut self, at: Point, attributes: Attributes) -> EndpointId {
        self.current_position = at;
        self.builder.begin(at, attributes)
    }

    fn end(&mut self, close: bool) {
        self.builder.end(close)
    }

    fn line_to(&mut self, to: Point, attributes: Attributes) -> EndpointId {
        let id = self.builder.line_to(to, attributes);
        self.current_position = to;
        self.prev_attributes.copy_from_slice(attributes);
        id
    }

    fn quadratic_bezier_to(
        &mut self,
        ctrl: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        let id = crate::private::flatten_quadratic_bezier(
            self.tolerance,
            self.current_position,
            ctrl,
            to,
            attributes,
            &self.prev_attributes,
            &mut self.builder,
            &mut self.attribute_buffer,
        );
        self.current_position = to;
        self.prev_attributes.copy_from_slice(attributes);

        id
    }

    fn cubic_bezier_to(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        let id = crate::private::flatten_cubic_bezier(
            self.tolerance,
            self.current_position,
            ctrl1,
            ctrl2,
            to,
            attributes,
            &self.prev_attributes,
            &mut self.builder,
            &mut self.attribute_buffer,
        );
        self.current_position = to;
        self.prev_attributes.copy_from_slice(attributes);

        id
    }

    fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.builder.reserve(endpoints + ctrl_points * 4, 0);
    }
}

impl<Builder: PathBuilder> Flattened<Builder> {
    pub fn new(builder: Builder, tolerance: f32) -> Flattened<Builder> {
        let n = builder.num_attributes();
        Flattened {
            builder,
            current_position: point(0.0, 0.0),
            tolerance,
            prev_attributes: vec![0.0; n],
            attribute_buffer: vec![0.0; n],
        }
    }

    pub fn build(self) -> Builder::PathType
    where
        Builder: Build,
    {
        self.builder.build()
    }

    pub fn set_tolerance(&mut self, tolerance: f32) {
        self.tolerance = tolerance
    }
}

/// Builds a path with a transformation applied.
pub struct Transformed<Builder, Transform> {
    builder: Builder,
    transform: Transform,
}

impl<Builder, Transform> Transformed<Builder, Transform> {
    #[inline]
    pub fn new(builder: Builder, transform: Transform) -> Self {
        Transformed { builder, transform }
    }

    #[inline]
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl<Builder: Build, Transform> Build for Transformed<Builder, Transform> {
    type PathType = Builder::PathType;

    #[inline]
    fn build(self) -> Builder::PathType {
        self.builder.build()
    }
}

impl<Builder, Transform> PathBuilder for Transformed<Builder, Transform>
where
    Builder: PathBuilder,
    Transform: Transformation<f32>,
{
    fn num_attributes(&self) -> usize {
        self.builder.num_attributes()
    }

    #[inline]
    fn begin(&mut self, at: Point, attributes: Attributes) -> EndpointId {
        self.builder
            .begin(self.transform.transform_point(at), attributes)
    }

    #[inline]
    fn end(&mut self, close: bool) {
        self.builder.end(close)
    }

    #[inline]
    fn line_to(&mut self, to: Point, attributes: Attributes) -> EndpointId {
        self.builder
            .line_to(self.transform.transform_point(to), attributes)
    }

    #[inline]
    fn quadratic_bezier_to(
        &mut self,
        ctrl: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        self.builder.quadratic_bezier_to(
            self.transform.transform_point(ctrl),
            self.transform.transform_point(to),
            attributes,
        )
    }

    #[inline]
    fn cubic_bezier_to(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        self.builder.cubic_bezier_to(
            self.transform.transform_point(ctrl1),
            self.transform.transform_point(ctrl2),
            self.transform.transform_point(to),
            attributes,
        )
    }

    #[inline]
    fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.builder.reserve(endpoints, ctrl_points);
    }
}

/// Implements an SVG-like building interface on top of a PathBuilder.
pub struct WithSvg<Builder: PathBuilder> {
    builder: Builder,

    first_position: Point,
    current_position: Point,
    last_ctrl: Point,
    last_cmd: Verb,
    need_moveto: bool,
    is_empty: bool,
    attribute_buffer: Vec<f32>,
}

impl<Builder: PathBuilder> WithSvg<Builder> {
    pub fn new(builder: Builder) -> Self {
        let attribute_buffer = vec![0.0; builder.num_attributes()];
        WithSvg {
            builder,
            first_position: point(0.0, 0.0),
            current_position: point(0.0, 0.0),
            last_ctrl: point(0.0, 0.0),
            need_moveto: true,
            is_empty: true,
            last_cmd: Verb::End,
            attribute_buffer,
        }
    }

    pub fn build(mut self) -> Builder::PathType
    where
        Builder: Build,
    {
        self.end_if_needed();
        self.builder.build()
    }

    pub fn flattened(self, tolerance: f32) -> WithSvg<Flattened<Builder>> {
        WithSvg::new(Flattened::new(self.builder, tolerance))
    }

    pub fn transformed<Transform>(
        self,
        transform: Transform,
    ) -> WithSvg<Transformed<Builder, Transform>>
    where
        Transform: Transformation<f32>,
    {
        WithSvg::new(Transformed::new(self.builder, transform))
    }

    pub fn move_to(&mut self, to: Point) -> EndpointId {
        self.end_if_needed();

        let id = self.builder.begin(to, &self.attribute_buffer);

        self.is_empty = false;
        self.need_moveto = false;
        self.first_position = to;
        self.current_position = to;
        self.last_cmd = Verb::Begin;

        id
    }

    pub fn line_to(&mut self, to: Point) -> EndpointId {
        if let Some(id) = self.begin_if_needed(&to) {
            return id;
        }

        self.current_position = to;
        self.last_cmd = Verb::LineTo;

        self.builder.line_to(to, &self.attribute_buffer)
    }

    pub fn close(&mut self) {
        if self.need_moveto {
            return;
        }

        // Relative path ops tend to accumulate small floating point error,
        // which results in the last segment ending almost but not quite at the
        // start of the sub-path, causing a new edge to be inserted which often
        // intersects with the first or last edge. This can affect algorithms that
        // Don't handle self-intersecting paths.
        // Deal with this by snapping the last point if it is very close to the
        // start of the sub path.
        //
        // TODO
        // if let Some(p) = self.builder.points.last_mut() {
        //     let d = (*p - self.first_position).abs();
        //     if d.x + d.y < 0.0001 {
        //         *p = self.first_position;
        //     }
        // }

        self.current_position = self.first_position;
        self.need_moveto = true;
        self.last_cmd = Verb::Close;

        self.builder.close();
    }

    pub fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point) -> EndpointId {
        if let Some(id) = self.begin_if_needed(&to) {
            return id;
        }

        self.current_position = to;
        self.last_cmd = Verb::QuadraticTo;
        self.last_ctrl = ctrl;

        self.builder
            .quadratic_bezier_to(ctrl, to, &self.attribute_buffer)
    }

    pub fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) -> EndpointId {
        if let Some(id) = self.begin_if_needed(&to) {
            return id;
        }

        self.current_position = to;
        self.last_cmd = Verb::CubicTo;
        self.last_ctrl = ctrl2;

        self.builder
            .cubic_bezier_to(ctrl1, ctrl2, to, &self.attribute_buffer)
    }

    pub fn arc(&mut self, center: Point, radii: Vector, sweep_angle: Angle, x_rotation: Angle) {
        nan_check(center);
        nan_check(radii.to_point());
        debug_assert!(!sweep_angle.get().is_nan());
        debug_assert!(!x_rotation.get().is_nan());

        self.last_ctrl = self.current_position;

        // If the center is equal to the current position, the start and end angles aren't
        // defined, so we just skip the arc to avoid generating NaNs that will cause issues
        // later.
        use lyon_geom::euclid::approxeq::ApproxEq;
        if self.current_position.approx_eq(&center) {
            return;
        }

        let start_angle = (self.current_position - center).angle_from_x_axis() - x_rotation;

        let arc = Arc {
            center,
            radii,
            start_angle,
            sweep_angle,
            x_rotation,
        };

        // If the current position is not on the arc, move or line to the beginning of the
        // arc.
        let arc_start = arc.from();
        if self.need_moveto {
            self.move_to(arc_start);
        } else if (arc_start - self.current_position).square_length() < 0.01 {
            self.builder.line_to(arc_start, &self.attribute_buffer);
        }

        arc.for_each_quadratic_bezier(&mut |curve| {
            self.builder
                .quadratic_bezier_to(curve.ctrl, curve.to, &self.attribute_buffer);
            self.current_position = curve.to;
        });
    }

    /// Ensures the current sub-path has a moveto command.
    ///
    /// Returns an ID if the command should be skipped and the ID returned instead.
    #[inline(always)]
    fn begin_if_needed(&mut self, default: &Point) -> Option<EndpointId> {
        if self.need_moveto {
            return self.insert_move_to(default);
        }

        None
    }

    #[inline(never)]
    fn insert_move_to(&mut self, default: &Point) -> Option<EndpointId> {
        if self.is_empty {
            return Some(self.move_to(*default));
        }

        self.move_to(self.first_position);

        None
    }

    fn end_if_needed(&mut self) {
        if (self.last_cmd as u8) <= (Verb::Begin as u8) {
            self.builder.end(false);
        }
    }

    pub fn current_position(&self) -> Point {
        self.current_position
    }

    pub fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.builder.reserve(endpoints, ctrl_points);
    }

    fn get_smooth_cubic_ctrl(&self) -> Point {
        match self.last_cmd {
            Verb::CubicTo => self.current_position + (self.current_position - self.last_ctrl),
            _ => self.current_position,
        }
    }

    fn get_smooth_quadratic_ctrl(&self) -> Point {
        match self.last_cmd {
            Verb::QuadraticTo => self.current_position + (self.current_position - self.last_ctrl),
            _ => self.current_position,
        }
    }

    fn relative_to_absolute(&self, v: Vector) -> Point {
        self.current_position + v
    }
}

impl<Builder, Transform> WithSvg<Transformed<Builder, Transform>>
where
    Builder: PathBuilder,
    Transform: Transformation<f32>,
{
    #[inline]
    pub fn set_transform(&mut self, transform: Transform) {
        self.builder.set_transform(transform);
    }
}

impl<Builder: PathBuilder + Build> Build for WithSvg<Builder> {
    type PathType = Builder::PathType;

    fn build(mut self) -> Builder::PathType {
        self.end_if_needed();
        self.builder.build()
    }
}

impl<Builder: PathBuilder> SvgPathBuilder for WithSvg<Builder> {
    fn move_to(&mut self, to: Point) {
        self.move_to(to);
    }

    fn close(&mut self) {
        self.close();
    }

    fn line_to(&mut self, to: Point) {
        self.line_to(to);
    }

    fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point) {
        self.quadratic_bezier_to(ctrl, to);
    }

    fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) {
        self.cubic_bezier_to(ctrl1, ctrl2, to);
    }

    fn relative_move_to(&mut self, to: Vector) {
        let to = self.relative_to_absolute(to);
        self.move_to(to);
    }

    fn relative_line_to(&mut self, to: Vector) {
        let to = self.relative_to_absolute(to);
        self.line_to(to);
    }

    fn relative_quadratic_bezier_to(&mut self, ctrl: Vector, to: Vector) {
        let ctrl = self.relative_to_absolute(ctrl);
        let to = self.relative_to_absolute(to);
        self.quadratic_bezier_to(ctrl, to);
    }

    fn relative_cubic_bezier_to(&mut self, ctrl1: Vector, ctrl2: Vector, to: Vector) {
        let to = self.relative_to_absolute(to);
        let ctrl1 = self.relative_to_absolute(ctrl1);
        let ctrl2 = self.relative_to_absolute(ctrl2);
        self.cubic_bezier_to(ctrl1, ctrl2, to);
    }

    fn smooth_cubic_bezier_to(&mut self, ctrl2: Point, to: Point) {
        let ctrl1 = self.get_smooth_cubic_ctrl();
        self.cubic_bezier_to(ctrl1, ctrl2, to);
    }

    fn smooth_relative_cubic_bezier_to(&mut self, ctrl2: Vector, to: Vector) {
        let ctrl1 = self.get_smooth_cubic_ctrl();
        let ctrl2 = self.relative_to_absolute(ctrl2);
        let to = self.relative_to_absolute(to);
        self.cubic_bezier_to(ctrl1, ctrl2, to);
    }

    fn smooth_quadratic_bezier_to(&mut self, to: Point) {
        let ctrl = self.get_smooth_quadratic_ctrl();
        self.quadratic_bezier_to(ctrl, to);
    }

    fn smooth_relative_quadratic_bezier_to(&mut self, to: Vector) {
        let ctrl = self.get_smooth_quadratic_ctrl();
        let to = self.relative_to_absolute(to);
        self.quadratic_bezier_to(ctrl, to);
    }

    fn horizontal_line_to(&mut self, x: f32) {
        let y = self.current_position.y;
        self.line_to(point(x, y));
    }

    fn relative_horizontal_line_to(&mut self, dx: f32) {
        let p = self.current_position;
        self.line_to(point(p.x + dx, p.y));
    }

    fn vertical_line_to(&mut self, y: f32) {
        let x = self.current_position.x;
        self.line_to(point(x, y));
    }

    fn relative_vertical_line_to(&mut self, dy: f32) {
        let p = self.current_position;
        self.line_to(point(p.x, p.y + dy));
    }

    fn arc_to(&mut self, radii: Vector, x_rotation: Angle, flags: ArcFlags, to: Point) {
        let svg_arc = SvgArc {
            from: self.current_position,
            to,
            radii,
            x_rotation,
            flags: ArcFlags {
                large_arc: flags.large_arc,
                sweep: flags.sweep,
            },
        };

        if svg_arc.is_straight_line() {
            self.line_to(to);
        } else {
            let arc = svg_arc.to_arc();
            self.arc(arc.center, arc.radii, arc.sweep_angle, arc.x_rotation);
        }
    }

    fn relative_arc_to(&mut self, radii: Vector, x_rotation: Angle, flags: ArcFlags, to: Vector) {
        let to = self.relative_to_absolute(to);
        self.arc_to(radii, x_rotation, flags, to);
    }

    fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.builder.reserve(endpoints, ctrl_points);
    }
}

/// Tessellate the stroke for an axis-aligned rounded rectangle.
fn add_circle<Builder: PathBuilder>(
    builder: &mut Builder,
    center: Point,
    radius: f32,
    winding: Winding,
    attributes: Attributes,
) {
    let radius = radius.abs();
    let dir = match winding {
        Winding::Positive => 1.0,
        Winding::Negative => -1.0,
    };

    // https://spencermortensen.com/articles/bezier-circle/
    const CONSTANT_FACTOR: f32 = 0.55191505;
    let d = radius * CONSTANT_FACTOR;

    builder.begin(center + vector(-radius, 0.0), attributes);

    let ctrl_0 = center + vector(-radius, -d * dir);
    let ctrl_1 = center + vector(-d, -radius * dir);
    let mid = center + vector(0.0, -radius * dir);
    builder.cubic_bezier_to(ctrl_0, ctrl_1, mid, attributes);

    let ctrl_0 = center + vector(d, -radius * dir);
    let ctrl_1 = center + vector(radius, -d * dir);
    let mid = center + vector(radius, 0.0);
    builder.cubic_bezier_to(ctrl_0, ctrl_1, mid, attributes);

    let ctrl_0 = center + vector(radius, d * dir);
    let ctrl_1 = center + vector(d, radius * dir);
    let mid = center + vector(0.0, radius * dir);
    builder.cubic_bezier_to(ctrl_0, ctrl_1, mid, attributes);

    let ctrl_0 = center + vector(-d, radius * dir);
    let ctrl_1 = center + vector(-radius, d * dir);
    let mid = center + vector(-radius, 0.0);
    builder.cubic_bezier_to(ctrl_0, ctrl_1, mid, attributes);

    builder.close();
}

/// Tessellate the stroke for an axis-aligned rounded rectangle.
fn add_rounded_rectangle<Builder: PathBuilder>(
    builder: &mut Builder,
    rect: &Box2D,
    radii: &BorderRadii,
    winding: Winding,
    attributes: Attributes,
) {
    let w = rect.width();
    let h = rect.height();
    let x_min = rect.min.x;
    let y_min = rect.min.y;
    let x_max = rect.max.x;
    let y_max = rect.max.y;
    let min_wh = w.min(h);
    let mut tl = radii.top_left.abs().min(min_wh);
    let mut tr = radii.top_right.abs().min(min_wh);
    let mut bl = radii.bottom_left.abs().min(min_wh);
    let mut br = radii.bottom_right.abs().min(min_wh);

    // clamp border radii if they don't fit in the rectangle.
    if tl + tr > w {
        let x = (tl + tr - w) * 0.5;
        tl -= x;
        tr -= x;
    }
    if bl + br > w {
        let x = (bl + br - w) * 0.5;
        bl -= x;
        br -= x;
    }
    if tr + br > h {
        let x = (tr + br - h) * 0.5;
        tr -= x;
        br -= x;
    }
    if tl + bl > h {
        let x = (tl + bl - h) * 0.5;
        tl -= x;
        bl -= x;
    }

    // https://spencermortensen.com/articles/bezier-circle/
    const CONSTANT_FACTOR: f32 = 0.55191505;

    let tl_d = tl * CONSTANT_FACTOR;
    let tl_corner = point(x_min, y_min);

    let tr_d = tr * CONSTANT_FACTOR;
    let tr_corner = point(x_max, y_min);

    let br_d = br * CONSTANT_FACTOR;
    let br_corner = point(x_max, y_max);

    let bl_d = bl * CONSTANT_FACTOR;
    let bl_corner = point(x_min, y_max);

    let points = [
        point(x_min, y_min + tl),           // begin
        tl_corner + vector(0.0, tl - tl_d), // control
        tl_corner + vector(tl - tl_d, 0.0), // control
        tl_corner + vector(tl, 0.0),        // end
        point(x_max - tr, y_min),
        tr_corner + vector(-tr + tr_d, 0.0),
        tr_corner + vector(0.0, tr - tr_d),
        tr_corner + vector(0.0, tr),
        point(x_max, y_max - br),
        br_corner + vector(0.0, -br + br_d),
        br_corner + vector(-br + br_d, 0.0),
        br_corner + vector(-br, 0.0),
        point(x_min + bl, y_max),
        bl_corner + vector(bl - bl_d, 0.0),
        bl_corner + vector(0.0, -bl + bl_d),
        bl_corner + vector(0.0, -bl),
    ];

    if winding == Winding::Positive {
        builder.begin(points[0], attributes);
        if tl > 0.0 {
            builder.cubic_bezier_to(points[1], points[2], points[3], attributes);
        }
        builder.line_to(points[4], attributes);
        if tr > 0.0 {
            builder.cubic_bezier_to(points[5], points[6], points[7], attributes);
        }
        builder.line_to(points[8], attributes);
        if br > 0.0 {
            builder.cubic_bezier_to(points[9], points[10], points[11], attributes);
        }
        builder.line_to(points[12], attributes);
        if bl > 0.0 {
            builder.cubic_bezier_to(points[13], points[14], points[15], attributes);
        }
    } else {
        builder.begin(points[15], attributes);
        if bl > 0.0 {
            builder.cubic_bezier_to(points[14], points[13], points[12], attributes);
        }
        builder.line_to(points[11], attributes);
        if br > 0.0 {
            builder.cubic_bezier_to(points[10], points[9], points[8], attributes);
        }
        builder.line_to(points[7], attributes);
        if tr > 0.0 {
            builder.cubic_bezier_to(points[6], points[5], points[4], attributes);
        }
        builder.line_to(points[3], attributes);
        if tl > 0.0 {
            builder.cubic_bezier_to(points[2], points[1], points[0], attributes);
        }
    }
    builder.end(true);
}

#[inline]
fn nan_check(p: Point) {
    debug_assert!(p.x.is_finite());
    debug_assert!(p.y.is_finite());
}

#[test]
fn svg_builder_line_to_after_close() {
    use crate::Path;
    use crate::PathEvent;

    let mut p = Path::svg_builder();
    p.line_to(point(1.0, 0.0));
    p.close();
    p.line_to(point(2.0, 0.0));

    let path = p.build();
    let mut it = path.iter();
    assert_eq!(
        it.next(),
        Some(PathEvent::Begin {
            at: point(1.0, 0.0)
        })
    );
    assert_eq!(
        it.next(),
        Some(PathEvent::End {
            last: point(1.0, 0.0),
            first: point(1.0, 0.0),
            close: true
        })
    );
    assert_eq!(
        it.next(),
        Some(PathEvent::Begin {
            at: point(1.0, 0.0)
        })
    );
    assert_eq!(
        it.next(),
        Some(PathEvent::Line {
            from: point(1.0, 0.0),
            to: point(2.0, 0.0)
        })
    );
    assert_eq!(
        it.next(),
        Some(PathEvent::End {
            last: point(2.0, 0.0),
            first: point(1.0, 0.0),
            close: false
        })
    );
    assert_eq!(it.next(), None);
}

#[test]
fn svg_builder_relative_curves() {
    use crate::Path;
    use crate::PathEvent;

    let mut p = Path::svg_builder();
    p.move_to(point(0.0, 0.0));
    p.relative_quadratic_bezier_to(vector(0., 100.), vector(-100., 100.));
    p.relative_line_to(vector(-50., 0.));

    let path = p.build();
    let mut it = path.iter();
    assert_eq!(
        it.next(),
        Some(PathEvent::Begin {
            at: point(0.0, 0.0)
        })
    );
    assert_eq!(
        it.next(),
        Some(PathEvent::Quadratic {
            from: point(0.0, 0.0),
            ctrl: point(0.0, 100.0),
            to: point(-100., 100.),
        })
    );
    assert_eq!(
        it.next(),
        Some(PathEvent::Line {
            from: point(-100.0, 100.0),
            to: point(-150., 100.)
        })
    );
    assert_eq!(
        it.next(),
        Some(PathEvent::End {
            first: point(0.0, 0.0),
            last: point(-150., 100.),
            close: false,
        })
    );
    assert_eq!(it.next(), None);
}

#[test]
fn svg_builder_arc_to_update_position() {
    use crate::Path;

    let mut p = Path::svg_builder();
    p.move_to(point(0.0, 0.0));
    assert_eq!(p.current_position(), point(0.0, 0.0));
    p.arc_to(
        vector(100., 100.),
        Angle::degrees(0.),
        ArcFlags::default(),
        point(0.0, 100.0),
    );
    assert_ne!(p.current_position(), point(0.0, 0.0));
}

#[test]
fn issue_650() {
    let mut builder = crate::path::Path::builder().with_svg();
    builder.arc(
        point(0.0, 0.0),
        vector(50.0, 50.0),
        Angle::radians(PI),
        Angle::radians(0.0),
    );
    builder.build();
}

#[test]
fn straight_line_arc() {
    use crate::Path;

    let mut p = Path::svg_builder();
    p.move_to(point(100.0, 0.0));
    // Don't assert on a "false" arc that's a straight line
    p.arc_to(
        vector(100., 100.),
        Angle::degrees(0.),
        ArcFlags::default(),
        point(100.0, 0.0),
    );
}

#[test]
fn top_right_rounded_rect() {
    use crate::{math::*, Path};
    let mut builder = Path::builder();
    builder.add_rounded_rectangle(
        &Box2D::new(point(0., 0.), point(100., 100.)),
        &BorderRadii {
            top_right: 2.,
            ..Default::default()
        },
        Winding::Positive,
    );
    let path = builder.build();
    let tr = path.iter().skip(2).next().unwrap();
    assert_eq!(tr.from(), point(98., 0.));
    assert_eq!(tr.to(), point(100., 2.));
}
