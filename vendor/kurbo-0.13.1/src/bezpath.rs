// Copyright 2018 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Bézier paths (up to cubic).

#![allow(clippy::many_single_char_names)]

use core::iter::{Extend, FromIterator};
use core::mem;
use core::ops::{Mul, Range};

use alloc::vec::Vec;

use arrayvec::ArrayVec;

use crate::MAX_EXTREMA;
use crate::common::{solve_cubic, solve_quadratic};
use crate::{
    Affine, CubicBez, Line, Nearest, ParamCurve, ParamCurveArclen, ParamCurveArea,
    ParamCurveExtrema, ParamCurveNearest, Point, QuadBez, Rect, Shape, TranslateScale, Vec2,
};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// A Bézier path.
///
/// These docs assume basic familiarity with Bézier curves; for an introduction,
/// see Pomax's wonderful [A Primer on Bézier Curves].
///
/// This path can contain lines, quadratics ([`QuadBez`]) and cubics
/// ([`CubicBez`]), and may contain multiple subpaths.
///
/// # Elements and Segments
///
/// A Bézier path can be represented in terms of either 'elements' ([`PathEl`])
/// or 'segments' ([`PathSeg`]). Elements map closely to how Béziers are
/// generally used in PostScript-style drawing APIs; they can be thought of as
/// instructions for drawing the path. Segments more directly describe the
/// path itself, with each segment being an independent line or curve.
///
/// These different representations are useful in different contexts.
/// For tasks like drawing, elements are a natural fit, but when doing
/// hit-testing or subdividing, we need to have access to the segments.
///
/// Conceptually, a `BezPath` contains zero or more subpaths. Each subpath
/// *always* begins with a `MoveTo`, then has zero or more `LineTo`, `QuadTo`,
/// and `CurveTo` elements, and optionally ends with a `ClosePath`.
///
/// Internally, a `BezPath` is a list of [`PathEl`]s; as such it implements
/// [`FromIterator<PathEl>`] and [`Extend<PathEl>`]:
///
/// ```
/// use kurbo::{BezPath, Rect, Shape, Vec2};
/// let accuracy = 0.1;
/// let rect = Rect::from_origin_size((0., 0.,), (10., 10.));
/// // these are equivalent
/// let path1 = rect.to_path(accuracy);
/// let path2: BezPath = rect.path_elements(accuracy).collect();
///
/// // extend a path with another path:
/// let mut path = rect.to_path(accuracy);
/// let shifted_rect = rect + Vec2::new(5.0, 10.0);
/// path.extend(shifted_rect.to_path(accuracy));
/// ```
///
/// You can iterate the elements of a `BezPath` with the [`iter`] method,
/// and the segments with the [`segments`] method:
///
/// ```
/// use kurbo::{BezPath, Line, PathEl, PathSeg, Point, Rect, Shape};
/// let accuracy = 0.1;
/// let rect = Rect::from_origin_size((0., 0.,), (10., 10.));
/// // these are equivalent
/// let path = rect.to_path(accuracy);
/// let first_el = PathEl::MoveTo(Point::ZERO);
/// let first_seg = PathSeg::Line(Line::new((0., 0.), (10., 0.)));
/// assert_eq!(path.iter().next(), Some(first_el));
/// assert_eq!(path.segments().next(), Some(first_seg));
/// ```
/// In addition, if you have some other type that implements
/// `Iterator<Item=PathEl>`, you can adapt that to an iterator of segments with
/// the [`segments` free function].
///
/// # Advanced functionality
///
/// In addition to the basic API, there are several useful pieces of advanced
/// functionality available on `BezPath`:
///
/// - [`flatten`] does Bézier flattening, converting a curve to a series of
///   line segments
/// - [`intersect_line`] computes intersections of a path with a line, useful
///   for things like subdividing
///
/// [A Primer on Bézier Curves]: https://pomax.github.io/bezierinfo/
/// [`iter`]: BezPath::iter
/// [`segments`]: BezPath::segments
/// [`flatten`]: flatten
/// [`intersect_line`]: PathSeg::intersect_line
/// [`segments` free function]: segments
/// [`FromIterator<PathEl>`]: std::iter::FromIterator
/// [`Extend<PathEl>`]: std::iter::Extend
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BezPath(Vec<PathEl>);

/// The element of a Bézier path.
///
/// A valid path has `MoveTo` at the beginning of each subpath.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathEl {
    /// Move directly to the point without drawing anything, starting a new
    /// subpath.
    MoveTo(Point),
    /// Draw a line from the current location to the point.
    LineTo(Point),
    /// Draw a quadratic Bézier using the current location and the two points.
    ///
    /// The first point is the Bézier's control point, the second is its end point.
    QuadTo(Point, Point),
    /// Draw a cubic Bézier using the current location and the three points.
    ///
    /// The first two points are the Bézier's control points, the last point is its end point.
    CurveTo(Point, Point, Point),
    /// Close off the path.
    ClosePath,
}

/// A segment of a Bézier path.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathSeg {
    /// A line segment.
    Line(Line),
    /// A quadratic Bézier segment.
    Quad(QuadBez),
    /// A cubic Bézier segment.
    Cubic(CubicBez),
}

/// An intersection of a [`Line`] and a [`PathSeg`].
///
/// This can be generated with the [`PathSeg::intersect_line`] method.
#[derive(Debug, Clone, Copy)]
pub struct LineIntersection {
    /// The 'time' that the intersection occurs, on the line.
    ///
    /// This value is in the range 0..1.
    pub line_t: f64,

    /// The 'time' that the intersection occurs, on the path segment.
    ///
    /// This value is nominally in the range 0..1, although it may slightly exceed
    /// that range at the boundaries of segments.
    pub segment_t: f64,
}

/// The minimum distance between two Bézier curves.
pub struct MinDistance {
    /// The shortest distance between any two points on the two curves.
    pub distance: f64,
    /// The position of the nearest point on the first curve, as a parameter.
    ///
    /// To resolve this to a [`Point`], use [`ParamCurve::eval`].
    ///
    /// [`ParamCurve::eval`]: crate::ParamCurve::eval
    pub t1: f64,
    /// The position of the nearest point on the second curve, as a parameter.
    ///
    /// To resolve this to a [`Point`], use [`ParamCurve::eval`].
    ///
    /// [`ParamCurve::eval`]: crate::ParamCurve::eval
    pub t2: f64,
}

impl BezPath {
    /// Create a new path.
    #[inline(always)]
    pub fn new() -> BezPath {
        BezPath::default()
    }

    /// Create a new path with the specified capacity.
    ///
    /// This can be useful if you already know how many path elements the path
    /// will consist of, to prevent reallocations.
    pub fn with_capacity(capacity: usize) -> BezPath {
        BezPath(Vec::with_capacity(capacity))
    }

    /// Create a path from a vector of path elements.
    ///
    /// `BezPath` also implements `FromIterator<PathEl>`, so it works with `collect`:
    ///
    /// ```
    /// // a very contrived example:
    /// use kurbo::{BezPath, PathEl};
    ///
    /// let path = BezPath::new();
    /// let as_vec: Vec<PathEl> = path.into_iter().collect();
    /// let back_to_path: BezPath = as_vec.into_iter().collect();
    /// ```
    pub fn from_vec(v: Vec<PathEl>) -> BezPath {
        debug_assert!(
            v.is_empty() || matches!(v.first(), Some(PathEl::MoveTo(_))),
            "BezPath must begin with MoveTo"
        );
        BezPath(v)
    }

    /// Removes the last [`PathEl`] from the path and returns it, or `None` if the path is empty.
    pub fn pop(&mut self) -> Option<PathEl> {
        self.0.pop()
    }

    /// Push a generic path element onto the path.
    pub fn push(&mut self, el: PathEl) {
        self.0.push(el);
        debug_assert!(
            matches!(self.0.first(), Some(PathEl::MoveTo(_))),
            "BezPath must begin with MoveTo"
        );
    }

    /// Push a "move to" element onto the path.
    pub fn move_to<P: Into<Point>>(&mut self, p: P) {
        self.push(PathEl::MoveTo(p.into()));
    }

    /// Push a "line to" element onto the path.
    ///
    /// Will panic with a debug assert when the path is empty and there is no
    /// "move to" element on the path.
    ///
    /// If `line_to` is called immediately after `close_path` then the current
    /// subpath starts at the initial point of the previous subpath.
    pub fn line_to<P: Into<Point>>(&mut self, p: P) {
        debug_assert!(!self.0.is_empty(), "uninitialized subpath (missing MoveTo)");
        self.push(PathEl::LineTo(p.into()));
    }

    /// Push a "quad to" element onto the path.
    ///
    /// Will panic with a debug assert when the path is empty and there is no
    /// "move to" element on the path.
    ///
    /// If `quad_to` is called immediately after `close_path` then the current
    /// subpath starts at the initial point of the previous subpath.
    pub fn quad_to<P: Into<Point>>(&mut self, p1: P, p2: P) {
        debug_assert!(!self.0.is_empty(), "uninitialized subpath (missing MoveTo)");
        self.push(PathEl::QuadTo(p1.into(), p2.into()));
    }

    /// Push a "curve to" element onto the path.
    ///
    /// Will panic with a debug assert when the path is empty and there is no
    /// "move to" element on the path.
    ///
    /// If `curve_to` is called immediately after `close_path` then the current
    /// subpath starts at the initial point of the previous subpath.
    pub fn curve_to<P: Into<Point>>(&mut self, p1: P, p2: P, p3: P) {
        debug_assert!(!self.0.is_empty(), "uninitialized subpath (missing MoveTo)");
        self.push(PathEl::CurveTo(p1.into(), p2.into(), p3.into()));
    }

    /// Push a "close path" element onto the path.
    ///
    /// Will panic with a debug assert when the path is empty and there is no
    /// "move to" element on the path.
    pub fn close_path(&mut self) {
        debug_assert!(!self.0.is_empty(), "uninitialized subpath (missing MoveTo)");
        self.push(PathEl::ClosePath);
    }

    /// Consumes the `BezPath` and returns a vector of [`PathEl`]s.
    #[inline(always)]
    pub fn into_elements(self) -> Vec<PathEl> {
        self.0
    }

    /// Get the path elements.
    ///
    /// For owned elements, see [`into_elements`].
    ///
    /// [`into_elements`]: Self::into_elements
    #[inline(always)]
    pub fn elements(&self) -> &[PathEl] {
        &self.0
    }

    /// Get the path elements (mut version).
    #[inline(always)]
    pub fn elements_mut(&mut self) -> &mut [PathEl] {
        &mut self.0
    }

    /// Returns an iterator over the path's elements.
    pub fn iter(&self) -> impl Iterator<Item = PathEl> + Clone + '_ {
        self.0.iter().copied()
    }

    /// Iterate over the path segments.
    pub fn segments(&self) -> impl Iterator<Item = PathSeg> + Clone + '_ {
        segments(self.iter())
    }

    /// Shorten the path, keeping the first `len` elements.
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }

    /// Get the segment at the given element index.
    ///
    /// If you need to access all segments, [`segments`] provides a better
    /// API. This is intended for random access of specific elements, for clients
    /// that require this specifically.
    ///
    /// **note**: This returns the segment that ends at the provided element
    /// index. In effect this means it is *1-indexed*: since no segment ends at
    /// the first element (which is presumed to be a `MoveTo`) `get_seg(0)` will
    /// always return `None`.
    pub fn get_seg(&self, ix: usize) -> Option<PathSeg> {
        if ix == 0 || ix >= self.0.len() {
            return None;
        }
        let last = match self.0[ix - 1] {
            PathEl::MoveTo(p) => p,
            PathEl::LineTo(p) => p,
            PathEl::QuadTo(_, p2) => p2,
            PathEl::CurveTo(_, _, p3) => p3,
            PathEl::ClosePath => return None,
        };
        match self.0[ix] {
            PathEl::LineTo(p) => Some(PathSeg::Line(Line::new(last, p))),
            PathEl::QuadTo(p1, p2) => Some(PathSeg::Quad(QuadBez::new(last, p1, p2))),
            PathEl::CurveTo(p1, p2, p3) => Some(PathSeg::Cubic(CubicBez::new(last, p1, p2, p3))),
            PathEl::ClosePath => self.0[..ix].iter().rev().find_map(|el| match *el {
                PathEl::MoveTo(start) if start != last => {
                    Some(PathSeg::Line(Line::new(last, start)))
                }
                _ => None,
            }),
            PathEl::MoveTo(_) => None,
        }
    }

    /// Returns `true` if the path contains no segments.
    pub fn is_empty(&self) -> bool {
        self.0
            .iter()
            .all(|el| matches!(el, PathEl::MoveTo(..) | PathEl::ClosePath))
    }

    /// Apply an affine transform to the path.
    pub fn apply_affine(&mut self, affine: Affine) {
        for el in self.0.iter_mut() {
            *el = affine * (*el);
        }
    }

    /// Is this path finite?
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.iter().all(|v| v.is_finite())
    }

    /// Is this path NaN?
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.iter().any(|v| v.is_nan())
    }

    /// Returns a rectangle that conservatively encloses the path.
    ///
    /// Unlike the `bounding_box` method, this uses control points directly
    /// rather than computing tight bounds for curve elements.
    pub fn control_box(&self) -> Rect {
        let mut cbox: Option<Rect> = None;
        let mut add_pts = |pts: &[Point]| {
            for pt in pts {
                cbox = match cbox {
                    Some(cbox) => Some(cbox.union_pt(*pt)),
                    _ => Some(Rect::from_points(*pt, *pt)),
                };
            }
        };
        for &el in self.elements() {
            match el {
                PathEl::MoveTo(p0) | PathEl::LineTo(p0) => add_pts(&[p0]),
                PathEl::QuadTo(p0, p1) => add_pts(&[p0, p1]),
                PathEl::CurveTo(p0, p1, p2) => add_pts(&[p0, p1, p2]),
                PathEl::ClosePath => {}
            }
        }
        cbox.unwrap_or_default()
    }

    /// Returns current position in the path, if path is not empty.
    ///
    /// This is different from calling [`PathEl::end_point`] on the last entry of [`BezPath::elements`]:
    /// this method handles [`PathEl::ClosePath`],
    /// by finding the first point of our last subpath, hence the time complexity is O(n).
    pub fn current_position(&self) -> Option<Point> {
        match self.0.last()? {
            PathEl::MoveTo(p) => Some(*p),
            PathEl::LineTo(p1) => Some(*p1),
            PathEl::QuadTo(_, p2) => Some(*p2),
            PathEl::CurveTo(_, _, p3) => Some(*p3),
            PathEl::ClosePath => self
                .elements()
                .iter()
                .rev()
                .skip(1)
                .take_while(|el| !matches!(el, PathEl::ClosePath))
                .last()
                .and_then(|el| el.end_point()),
        }
    }

    /// Returns an iterator over the subpaths of this path. See [Elements and Segments](#elements-and-segments) for more information.
    pub fn subpaths(&self) -> impl Iterator<Item = &[PathEl]> {
        let elements = self.elements();
        let mut i = 0;

        core::iter::from_fn(move || {
            if i >= elements.len() {
                return None;
            }
            let start = i;
            i += 1;
            while i < elements.len() && !matches!(elements[i], PathEl::MoveTo(_)) {
                i += 1;
            }
            Some(&elements[start..i])
        })
    }

    /// Returns a new path with the winding direction of all subpaths reversed.
    pub fn reverse_subpaths(&self) -> BezPath {
        let elements = self.elements();
        let mut start_ix = 1;
        let mut start_pt = Point::default();
        let mut reversed = BezPath(Vec::with_capacity(elements.len()));
        // Pending move is used to capture degenerate subpaths that should
        // remain in the reversed output.
        let mut pending_move = false;
        for (ix, el) in elements.iter().enumerate() {
            match el {
                PathEl::MoveTo(pt) => {
                    if pending_move {
                        reversed.push(PathEl::MoveTo(start_pt));
                    }
                    if start_ix < ix {
                        reverse_subpath(start_pt, &elements[start_ix..ix], &mut reversed);
                    }
                    pending_move = true;
                    start_pt = *pt;
                    start_ix = ix + 1;
                }
                PathEl::ClosePath => {
                    if start_ix <= ix {
                        reverse_subpath(start_pt, &elements[start_ix..ix], &mut reversed);
                    }
                    reversed.push(PathEl::ClosePath);
                    start_ix = ix + 1;
                    pending_move = false;
                }
                _ => {
                    pending_move = false;
                }
            }
        }
        if start_ix < elements.len() {
            reverse_subpath(start_pt, &elements[start_ix..], &mut reversed);
        } else if pending_move {
            reversed.push(PathEl::MoveTo(start_pt));
        }
        reversed
    }
}

/// Helper for reversing a subpath.
///
/// The `els` parameter must not contain any `MoveTo` or `ClosePath` elements.
fn reverse_subpath(start_pt: Point, els: &[PathEl], reversed: &mut BezPath) {
    let end_pt = els.last().and_then(|el| el.end_point()).unwrap_or(start_pt);
    reversed.push(PathEl::MoveTo(end_pt));
    for (ix, el) in els.iter().enumerate().rev() {
        let end_pt = if ix > 0 {
            els[ix - 1].end_point().unwrap()
        } else {
            start_pt
        };
        match el {
            PathEl::LineTo(_) => reversed.push(PathEl::LineTo(end_pt)),
            PathEl::QuadTo(c0, _) => reversed.push(PathEl::QuadTo(*c0, end_pt)),
            PathEl::CurveTo(c0, c1, _) => reversed.push(PathEl::CurveTo(*c1, *c0, end_pt)),
            _ => panic!("reverse_subpath expects MoveTo and ClosePath to be removed"),
        }
    }
}

impl FromIterator<PathEl> for BezPath {
    fn from_iter<T: IntoIterator<Item = PathEl>>(iter: T) -> Self {
        let el_vec: Vec<_> = iter.into_iter().collect();
        BezPath::from_vec(el_vec)
    }
}

/// Allow iteration over references to `BezPath`.
///
/// Note: the semantics are slightly different from simply iterating over the
/// slice, as it returns `PathEl` items, rather than references.
impl<'a> IntoIterator for &'a BezPath {
    type Item = PathEl;
    type IntoIter = core::iter::Cloned<core::slice::Iter<'a, PathEl>>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements().iter().cloned()
    }
}

impl IntoIterator for BezPath {
    type Item = PathEl;
    type IntoIter = alloc::vec::IntoIter<PathEl>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<PathEl> for BezPath {
    /// Add the items from the iterator to this path.
    ///
    /// <div class="warning">
    ///
    /// Note that if you're attempting to make a continuous path, you will generally
    /// want to ensure that the iterator does not contain any [`MoveTo`](PathEl::MoveTo)
    /// or [`ClosePath`](PathEl::ClosePath) elements.
    /// Note especially that many (open) [shapes](Shape) will start with a `MoveTo` if
    /// you use their [`path_elements`](Shape::path_elements) function.
    /// Some shapes have alternatives for this use case, such as [`Arc::append_iter`](crate::Arc::append_iter).
    ///
    /// </div>
    fn extend<I: IntoIterator<Item = PathEl>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

/// Proportion of tolerance budget that goes to cubic to quadratic conversion.
const TO_QUAD_TOL: f64 = 0.1;

/// Flatten the path, invoking the callback repeatedly.
///
/// Flattening is the action of approximating a curve with a succession of line segments.
///
/// <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 30" height="30mm" width="120mm">
///   <path d="M26.7 24.94l.82-11.15M44.46 5.1L33.8 7.34" fill="none" stroke="#55d400" stroke-width=".5"/>
///   <path d="M26.7 24.94c.97-11.13 7.17-17.6 17.76-19.84M75.27 24.94l1.13-5.5 2.67-5.48 4-4.42L88 6.7l5.02-1.6" fill="none" stroke="#000"/>
///   <path d="M77.57 19.37a1.1 1.1 0 0 1-1.08 1.08 1.1 1.1 0 0 1-1.1-1.08 1.1 1.1 0 0 1 1.08-1.1 1.1 1.1 0 0 1 1.1 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M77.57 19.37a1.1 1.1 0 0 1-1.08 1.08 1.1 1.1 0 0 1-1.1-1.08 1.1 1.1 0 0 1 1.08-1.1 1.1 1.1 0 0 1 1.1 1.1" color="#000" fill="#fff"/>
///   <path d="M80.22 13.93a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.08 1.1 1.1 0 0 1 1.08 1.08" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M80.22 13.93a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.08 1.1 1.1 0 0 1 1.08 1.08" color="#000" fill="#fff"/>
///   <path d="M84.08 9.55a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.1-1.1 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M84.08 9.55a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.1-1.1 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="#fff"/>
///   <path d="M89.1 6.66a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.08-1.08 1.1 1.1 0 0 1 1.1 1.08" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M89.1 6.66a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.08-1.08 1.1 1.1 0 0 1 1.1 1.08" color="#000" fill="#fff"/>
///   <path d="M94.4 5a1.1 1.1 0 0 1-1.1 1.1A1.1 1.1 0 0 1 92.23 5a1.1 1.1 0 0 1 1.08-1.08A1.1 1.1 0 0 1 94.4 5" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M94.4 5a1.1 1.1 0 0 1-1.1 1.1A1.1 1.1 0 0 1 92.23 5a1.1 1.1 0 0 1 1.08-1.08A1.1 1.1 0 0 1 94.4 5" color="#000" fill="#fff"/>
///   <path d="M76.44 25.13a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M76.44 25.13a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="#fff"/>
///   <path d="M27.78 24.9a1.1 1.1 0 0 1-1.08 1.08 1.1 1.1 0 0 1-1.1-1.08 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M27.78 24.9a1.1 1.1 0 0 1-1.08 1.08 1.1 1.1 0 0 1-1.1-1.08 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="#fff"/>
///   <path d="M45.4 5.14a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.1-1.1 1.1 1.1 0 0 1 1.1-1.08 1.1 1.1 0 0 1 1.1 1.08" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M45.4 5.14a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.1-1.1 1.1 1.1 0 0 1 1.1-1.08 1.1 1.1 0 0 1 1.1 1.08" color="#000" fill="#fff"/>
///   <path d="M28.67 13.8a1.1 1.1 0 0 1-1.1 1.08 1.1 1.1 0 0 1-1.08-1.08 1.1 1.1 0 0 1 1.08-1.1 1.1 1.1 0 0 1 1.1 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M28.67 13.8a1.1 1.1 0 0 1-1.1 1.08 1.1 1.1 0 0 1-1.08-1.08 1.1 1.1 0 0 1 1.08-1.1 1.1 1.1 0 0 1 1.1 1.1" color="#000" fill="#fff"/>
///   <path d="M35 7.32a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.1A1.1 1.1 0 0 1 35 7.33" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M35 7.32a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.1A1.1 1.1 0 0 1 35 7.33" color="#000" fill="#fff"/>
///   <text style="line-height:6.61458302px" x="35.74" y="284.49" font-size="5.29" font-family="Sans" letter-spacing="0" word-spacing="0" fill="#b3b3b3" stroke-width=".26" transform="translate(19.595 -267)">
///     <tspan x="35.74" y="284.49" font-size="10.58">→</tspan>
///   </text>
/// </svg>
///
/// The tolerance value controls the maximum distance between the curved input
/// segments and their polyline approximations. (In technical terms, this is the
/// Hausdorff distance). The algorithm attempts to bound this distance between
/// by `tolerance` but this is not absolutely guaranteed. The appropriate value
/// depends on the use, but for antialiased rendering, a value of 0.25 has been
/// determined to give good results. The number of segments tends to scale as the
/// inverse square root of tolerance.
///
/// <svg viewBox="0 0 47.5 13.2" height="100" width="350" xmlns="http://www.w3.org/2000/svg">
///   <path d="M-2.44 9.53c16.27-8.5 39.68-7.93 52.13 1.9" fill="none" stroke="#dde9af" stroke-width="4.6"/>
///   <path d="M-1.97 9.3C14.28 1.03 37.36 1.7 49.7 11.4" fill="none" stroke="#00d400" stroke-width=".57" stroke-linecap="round" stroke-dasharray="4.6, 2.291434"/>
///   <path d="M-1.94 10.46L6.2 6.08l28.32-1.4 15.17 6.74" fill="none" stroke="#000" stroke-width=".6"/>
///   <path d="M6.83 6.57a.9.9 0 0 1-1.25.15.9.9 0 0 1-.15-1.25.9.9 0 0 1 1.25-.15.9.9 0 0 1 .15 1.25" color="#000" stroke="#000" stroke-width=".57" stroke-linecap="round" stroke-opacity=".5"/>
///   <path d="M35.35 5.3a.9.9 0 0 1-1.25.15.9.9 0 0 1-.15-1.25.9.9 0 0 1 1.25-.15.9.9 0 0 1 .15 1.24" color="#000" stroke="#000" stroke-width=".6" stroke-opacity=".5"/>
///   <g fill="none" stroke="#ff7f2a" stroke-width=".26">
///     <path d="M20.4 3.8l.1 1.83M19.9 4.28l.48-.56.57.52M21.02 5.18l-.5.56-.6-.53" stroke-width=".2978872"/>
///   </g>
/// </svg>
///
/// The callback will be called in order with each element of the generated
/// path. Because the result is made of polylines, these will be straight-line
/// path elements only, no curves.
///
/// This algorithm is based on the blog post [Flattening quadratic Béziers]
/// but with some refinements. For one, there is a more careful approximation
/// at cusps. For two, the algorithm is extended to work with cubic Béziers
/// as well, by first subdividing into quadratics and then computing the
/// subdivision of each quadratic. However, as a clever trick, these quadratics
/// are subdivided fractionally, and their endpoints are not included.
///
/// TODO: write a paper explaining this in more detail.
///
/// [Flattening quadratic Béziers]: https://raphlinus.github.io/graphics/curves/2019/12/23/flatten-quadbez.html
pub fn flatten(
    path: impl IntoIterator<Item = PathEl>,
    tolerance: f64,
    mut callback: impl FnMut(PathEl),
) {
    let sqrt_tol = tolerance.sqrt();
    let mut last_pt = None;
    let mut quad_buf = Vec::new();
    for el in path {
        match el {
            PathEl::MoveTo(p) => {
                last_pt = Some(p);
                callback(PathEl::MoveTo(p));
            }
            PathEl::LineTo(p) => {
                last_pt = Some(p);
                callback(PathEl::LineTo(p));
            }
            PathEl::QuadTo(p1, p2) => {
                if let Some(p0) = last_pt {
                    let q = QuadBez::new(p0, p1, p2);
                    let params = q.estimate_subdiv(sqrt_tol);
                    let n = ((0.5 * params.val / sqrt_tol).ceil() as usize).max(1);
                    let step = 1.0 / (n as f64);
                    for i in 1..n {
                        let u = (i as f64) * step;
                        let t = q.determine_subdiv_t(&params, u);
                        let p = q.eval(t);
                        callback(PathEl::LineTo(p));
                    }
                    callback(PathEl::LineTo(p2));
                }
                last_pt = Some(p2);
            }
            PathEl::CurveTo(p1, p2, p3) => {
                if let Some(p0) = last_pt {
                    let c = CubicBez::new(p0, p1, p2, p3);

                    // Subdivide into quadratics, and estimate the number of
                    // subdivisions required for each, summing to arrive at an
                    // estimate for the number of subdivisions for the cubic.
                    // Also retain these parameters for later.
                    let iter = c.to_quads(tolerance * TO_QUAD_TOL);
                    quad_buf.clear();
                    quad_buf.reserve(iter.size_hint().0);
                    let sqrt_remain_tol = sqrt_tol * (1.0 - TO_QUAD_TOL).sqrt();
                    let mut sum = 0.0;
                    for (_, _, q) in iter {
                        let params = q.estimate_subdiv(sqrt_remain_tol);
                        sum += params.val;
                        quad_buf.push((q, params));
                    }
                    let n = ((0.5 * sum / sqrt_remain_tol).ceil() as usize).max(1);

                    // Iterate through the quadratics, outputting the points of
                    // subdivisions that fall within that quadratic.
                    let step = sum / (n as f64);
                    let mut i = 1;
                    let mut val_sum = 0.0;
                    for (q, params) in &quad_buf {
                        let mut target = (i as f64) * step;
                        let recip_val = params.val.recip();
                        while target < val_sum + params.val {
                            let u = (target - val_sum) * recip_val;
                            let t = q.determine_subdiv_t(params, u);
                            let p = q.eval(t);
                            callback(PathEl::LineTo(p));
                            i += 1;
                            if i == n + 1 {
                                break;
                            }
                            target = (i as f64) * step;
                        }
                        val_sum += params.val;
                    }
                    callback(PathEl::LineTo(p3));
                }
                last_pt = Some(p3);
            }
            PathEl::ClosePath => {
                last_pt = None;
                callback(PathEl::ClosePath);
            }
        }
    }
}

impl Mul<PathEl> for Affine {
    type Output = PathEl;

    // TODO: Inlining this leads to a huge performance benefit, perhaps the same should be done for
    // other methods.
    #[inline(always)]
    fn mul(self, other: PathEl) -> PathEl {
        match other {
            PathEl::MoveTo(p) => PathEl::MoveTo(self * p),
            PathEl::LineTo(p) => PathEl::LineTo(self * p),
            PathEl::QuadTo(p1, p2) => PathEl::QuadTo(self * p1, self * p2),
            PathEl::CurveTo(p1, p2, p3) => PathEl::CurveTo(self * p1, self * p2, self * p3),
            PathEl::ClosePath => PathEl::ClosePath,
        }
    }
}

impl Mul<PathSeg> for Affine {
    type Output = PathSeg;

    fn mul(self, other: PathSeg) -> PathSeg {
        match other {
            PathSeg::Line(line) => PathSeg::Line(self * line),
            PathSeg::Quad(quad) => PathSeg::Quad(self * quad),
            PathSeg::Cubic(cubic) => PathSeg::Cubic(self * cubic),
        }
    }
}

impl Mul<BezPath> for Affine {
    type Output = BezPath;

    fn mul(self, other: BezPath) -> BezPath {
        BezPath(other.0.iter().map(|&el| self * el).collect())
    }
}

impl Mul<&BezPath> for Affine {
    type Output = BezPath;

    fn mul(self, other: &BezPath) -> BezPath {
        BezPath(other.0.iter().map(|&el| self * el).collect())
    }
}

impl Mul<PathEl> for TranslateScale {
    type Output = PathEl;

    fn mul(self, other: PathEl) -> PathEl {
        match other {
            PathEl::MoveTo(p) => PathEl::MoveTo(self * p),
            PathEl::LineTo(p) => PathEl::LineTo(self * p),
            PathEl::QuadTo(p1, p2) => PathEl::QuadTo(self * p1, self * p2),
            PathEl::CurveTo(p1, p2, p3) => PathEl::CurveTo(self * p1, self * p2, self * p3),
            PathEl::ClosePath => PathEl::ClosePath,
        }
    }
}

impl Mul<PathSeg> for TranslateScale {
    type Output = PathSeg;

    fn mul(self, other: PathSeg) -> PathSeg {
        match other {
            PathSeg::Line(line) => PathSeg::Line(self * line),
            PathSeg::Quad(quad) => PathSeg::Quad(self * quad),
            PathSeg::Cubic(cubic) => PathSeg::Cubic(self * cubic),
        }
    }
}

impl Mul<BezPath> for TranslateScale {
    type Output = BezPath;

    fn mul(self, other: BezPath) -> BezPath {
        BezPath(other.0.iter().map(|&el| self * el).collect())
    }
}

impl Mul<&BezPath> for TranslateScale {
    type Output = BezPath;

    fn mul(self, other: &BezPath) -> BezPath {
        BezPath(other.0.iter().map(|&el| self * el).collect())
    }
}

/// Close all open subpaths in a path, expressed as iterator transform.
pub(crate) fn close_subpaths<I>(elements: I) -> CloseSubpaths<I::IntoIter>
where
    I: IntoIterator<Item = PathEl>,
{
    CloseSubpaths {
        elements: elements.into_iter(),
        state: CloseSubpathState::Start,
    }
}

/// An iterator that closes all open subpaths.
///
/// This struct is created by the [`close_subpaths`] function.
#[derive(Clone)]
pub(crate) struct CloseSubpaths<I: Iterator<Item = PathEl>> {
    elements: I,
    state: CloseSubpathState,
}

#[derive(Clone)]
enum CloseSubpathState {
    Start,
    InSubpath,
    ClosedLast,
    PendingMoveTo(Point),
}

impl<I: Iterator<Item = PathEl>> Iterator for CloseSubpaths<I> {
    type Item = PathEl;

    fn next(&mut self) -> Option<PathEl> {
        match self.state {
            CloseSubpathState::Start => {
                let el = self.elements.next();
                if !matches!(el, Some(PathEl::ClosePath) | None) {
                    self.state = CloseSubpathState::InSubpath;
                }
                el
            }
            CloseSubpathState::InSubpath => {
                let el = self.elements.next();
                match el {
                    None => {
                        self.state = CloseSubpathState::ClosedLast;
                        Some(PathEl::ClosePath)
                    }
                    Some(PathEl::MoveTo(point)) => {
                        self.state = CloseSubpathState::PendingMoveTo(point);
                        Some(PathEl::ClosePath)
                    }
                    Some(PathEl::ClosePath) => {
                        self.state = CloseSubpathState::Start;
                        el
                    }
                    _ => el,
                }
            }
            CloseSubpathState::ClosedLast => None,
            CloseSubpathState::PendingMoveTo(point) => {
                self.state = CloseSubpathState::Start;
                Some(PathEl::MoveTo(point))
            }
        }
    }
}

/// Transform an iterator over path elements into one over path
/// segments.
///
/// See also [`BezPath::segments`].
/// This signature is a bit more general, allowing `&[PathEl]` slices
/// and other iterators yielding `PathEl`.
pub fn segments<I>(elements: I) -> Segments<I::IntoIter>
where
    I: IntoIterator<Item = PathEl>,
{
    Segments {
        elements: elements.into_iter(),
        start_last: None,
    }
}

/// An iterator that transforms path elements to path segments.
///
/// This struct is created by the [`segments`] function.
#[derive(Clone)]
pub struct Segments<I: Iterator<Item = PathEl>> {
    elements: I,
    start_last: Option<(Point, Point)>,
}

impl<I: Iterator<Item = PathEl>> Iterator for Segments<I> {
    type Item = PathSeg;

    #[inline]
    fn next(&mut self) -> Option<PathSeg> {
        for el in &mut self.elements {
            // We first need to check whether this is the first
            // path element we see to fill in the start position.
            let (start, last) = self.start_last.get_or_insert_with(|| {
                let point = match el {
                    PathEl::MoveTo(p) => p,
                    PathEl::LineTo(p) => p,
                    PathEl::QuadTo(_, p2) => p2,
                    PathEl::CurveTo(_, _, p3) => p3,
                    PathEl::ClosePath => panic!("Can't start a segment on a ClosePath"),
                };
                (point, point)
            });

            return Some(match el {
                PathEl::MoveTo(p) => {
                    *start = p;
                    *last = p;
                    continue;
                }
                PathEl::LineTo(p) => PathSeg::Line(Line::new(mem::replace(last, p), p)),
                PathEl::QuadTo(p1, p2) => {
                    PathSeg::Quad(QuadBez::new(mem::replace(last, p2), p1, p2))
                }
                PathEl::CurveTo(p1, p2, p3) => {
                    PathSeg::Cubic(CubicBez::new(mem::replace(last, p3), p1, p2, p3))
                }
                PathEl::ClosePath => {
                    if *last != *start {
                        PathSeg::Line(Line::new(mem::replace(last, *start), *start))
                    } else {
                        continue;
                    }
                }
            });
        }

        None
    }
}

impl<I: Iterator<Item = PathEl>> Segments<I> {
    /// Here, `accuracy` specifies the accuracy for each Bézier segment. At worst,
    /// the total error is `accuracy` times the number of Bézier segments.
    // TODO: pub? Or is this subsumed by method of &[PathEl]?
    pub(crate) fn perimeter(self, accuracy: f64) -> f64 {
        self.map(|seg| seg.arclen(accuracy)).sum()
    }

    // Same
    pub(crate) fn area(self) -> f64 {
        self.map(|seg| seg.signed_area()).sum()
    }

    // Same
    pub(crate) fn winding(self, p: Point) -> i32 {
        self.map(|seg| seg.winding(p)).sum()
    }

    // Same
    pub(crate) fn bounding_box(self) -> Rect {
        let mut bbox: Option<Rect> = None;
        for seg in self {
            let seg_bb = ParamCurveExtrema::bounding_box(&seg);
            if let Some(bb) = bbox {
                bbox = Some(bb.union(seg_bb));
            } else {
                bbox = Some(seg_bb);
            }
        }
        bbox.unwrap_or_default()
    }
}

impl ParamCurve for PathSeg {
    fn eval(&self, t: f64) -> Point {
        match *self {
            PathSeg::Line(line) => line.eval(t),
            PathSeg::Quad(quad) => quad.eval(t),
            PathSeg::Cubic(cubic) => cubic.eval(t),
        }
    }

    fn subsegment(&self, range: Range<f64>) -> PathSeg {
        match *self {
            PathSeg::Line(line) => PathSeg::Line(line.subsegment(range)),
            PathSeg::Quad(quad) => PathSeg::Quad(quad.subsegment(range)),
            PathSeg::Cubic(cubic) => PathSeg::Cubic(cubic.subsegment(range)),
        }
    }

    fn start(&self) -> Point {
        match *self {
            PathSeg::Line(line) => line.start(),
            PathSeg::Quad(quad) => quad.start(),
            PathSeg::Cubic(cubic) => cubic.start(),
        }
    }

    fn end(&self) -> Point {
        match *self {
            PathSeg::Line(line) => line.end(),
            PathSeg::Quad(quad) => quad.end(),
            PathSeg::Cubic(cubic) => cubic.end(),
        }
    }
}

impl ParamCurveArclen for PathSeg {
    fn arclen(&self, accuracy: f64) -> f64 {
        match *self {
            PathSeg::Line(line) => line.arclen(accuracy),
            PathSeg::Quad(quad) => quad.arclen(accuracy),
            PathSeg::Cubic(cubic) => cubic.arclen(accuracy),
        }
    }

    fn inv_arclen(&self, arclen: f64, accuracy: f64) -> f64 {
        match *self {
            PathSeg::Line(line) => line.inv_arclen(arclen, accuracy),
            PathSeg::Quad(quad) => quad.inv_arclen(arclen, accuracy),
            PathSeg::Cubic(cubic) => cubic.inv_arclen(arclen, accuracy),
        }
    }
}

impl ParamCurveArea for PathSeg {
    fn signed_area(&self) -> f64 {
        match *self {
            PathSeg::Line(line) => line.signed_area(),
            PathSeg::Quad(quad) => quad.signed_area(),
            PathSeg::Cubic(cubic) => cubic.signed_area(),
        }
    }
}

impl ParamCurveNearest for PathSeg {
    fn nearest(&self, p: Point, accuracy: f64) -> Nearest {
        match *self {
            PathSeg::Line(line) => line.nearest(p, accuracy),
            PathSeg::Quad(quad) => quad.nearest(p, accuracy),
            PathSeg::Cubic(cubic) => cubic.nearest(p, accuracy),
        }
    }
}

impl ParamCurveExtrema for PathSeg {
    fn extrema(&self) -> ArrayVec<f64, MAX_EXTREMA> {
        match *self {
            PathSeg::Line(line) => line.extrema(),
            PathSeg::Quad(quad) => quad.extrema(),
            PathSeg::Cubic(cubic) => cubic.extrema(),
        }
    }
}

impl PathSeg {
    /// Get the [`PathEl`] that is equivalent to discarding the segment start point.
    pub fn as_path_el(&self) -> PathEl {
        match self {
            PathSeg::Line(line) => PathEl::LineTo(line.p1),
            PathSeg::Quad(q) => PathEl::QuadTo(q.p1, q.p2),
            PathSeg::Cubic(c) => PathEl::CurveTo(c.p1, c.p2, c.p3),
        }
    }

    /// Returns a new `PathSeg` describing the same path as `self`, but with
    /// the points reversed.
    pub fn reverse(&self) -> PathSeg {
        match self {
            PathSeg::Line(Line { p0, p1 }) => PathSeg::Line(Line::new(*p1, *p0)),
            PathSeg::Quad(q) => PathSeg::Quad(QuadBez::new(q.p2, q.p1, q.p0)),
            PathSeg::Cubic(c) => PathSeg::Cubic(CubicBez::new(c.p3, c.p2, c.p1, c.p0)),
        }
    }

    /// Convert this segment to a cubic Bézier.
    pub fn to_cubic(&self) -> CubicBez {
        match *self {
            PathSeg::Line(Line { p0, p1 }) => CubicBez::new(p0, p0, p1, p1),
            PathSeg::Cubic(c) => c,
            PathSeg::Quad(q) => q.raise(),
        }
    }

    /// A single-segment "winding" number.
    ///
    /// Assume that `self` is monotonic in `y`, and take a ray pointing
    /// left from `p`. If `self` crosses that ray, returns 1 (if we're
    /// decreasing in y) or -1 (if we're increasing in y). Otherwise, returns 0.
    ///
    /// Handling of endpoints is a little subtle, just to make sure that
    /// we correctly handle consecutive segments: we consider `self` to
    /// contain the endpoint with smaller y, and not contain the endpoint
    /// with larger y.
    fn winding_inner(&self, p: Point) -> i32 {
        let start = self.start();
        let end = self.end();
        let sign = if end.y > start.y {
            if p.y < start.y || p.y >= end.y {
                return 0;
            }
            -1
        } else if end.y < start.y {
            if p.y < end.y || p.y >= start.y {
                return 0;
            }
            1
        } else {
            return 0;
        };
        match *self {
            PathSeg::Line(_line) => {
                if p.x < start.x.min(end.x) {
                    return 0;
                }
                if p.x >= start.x.max(end.x) {
                    return sign;
                }
                // line equation ax + by = c
                let a = end.y - start.y;
                let b = start.x - end.x;
                let c = a * start.x + b * start.y;
                if (a * p.x + b * p.y - c) * (sign as f64) <= 0.0 {
                    sign
                } else {
                    0
                }
            }
            PathSeg::Quad(quad) => {
                let p1 = quad.p1;
                if p.x < start.x.min(end.x).min(p1.x) {
                    return 0;
                }
                if p.x >= start.x.max(end.x).max(p1.x) {
                    return sign;
                }
                let t = quad.solve_monotonic_for_y(p.y);
                let x = quad.eval(t).x;
                if p.x >= x { sign } else { 0 }
            }
            PathSeg::Cubic(cubic) => {
                let p1 = cubic.p1;
                let p2 = cubic.p2;
                if p.x < start.x.min(end.x).min(p1.x).min(p2.x) {
                    return 0;
                }
                if p.x >= start.x.max(end.x).max(p1.x).max(p2.x) {
                    return sign;
                }
                let t = cubic.solve_monotonic_for_y(p.y);
                let x = cubic.eval(t).x;
                if p.x >= x { sign } else { 0 }
            }
        }
    }

    /// Compute the winding number contribution of a single segment.
    ///
    /// Cast a ray to the left and count intersections.
    fn winding(&self, p: Point) -> i32 {
        self.extrema_ranges()
            .into_iter()
            .map(|range| self.subsegment(range).winding_inner(p))
            .sum()
    }

    /// Compute intersections against a line.
    ///
    /// Returns a vector of the intersections. For each intersection,
    /// the `t` value of the segment and line are given.
    ///
    /// Note: This test is designed to be inclusive of points near the endpoints
    /// of the segment. This is so that testing a line against multiple
    /// contiguous segments of a path will be guaranteed to catch at least one
    /// of them. In such cases, use higher level logic to coalesce the hits
    /// (the `t` value may be slightly outside the range of 0..1).
    ///
    /// # Examples
    ///
    /// ```
    /// # use kurbo::*;
    /// let seg = PathSeg::Line(Line::new((0.0, 0.0), (2.0, 0.0)));
    /// let line = Line::new((1.0, 2.0), (1.0, -2.0));
    /// let intersection = seg.intersect_line(line);
    /// assert_eq!(intersection.len(), 1);
    /// let intersection = intersection[0];
    /// assert_eq!(intersection.segment_t, 0.5);
    /// assert_eq!(intersection.line_t, 0.5);
    ///
    /// let point = seg.eval(intersection.segment_t);
    /// assert_eq!(point, Point::new(1.0, 0.0));
    /// ```
    pub fn intersect_line(&self, line: Line) -> ArrayVec<LineIntersection, 3> {
        const EPSILON: f64 = 1e-9;
        let p0 = line.p0;
        let p1 = line.p1;
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let mut result = ArrayVec::new();
        match self {
            PathSeg::Line(l) => {
                let det = dx * (l.p1.y - l.p0.y) - dy * (l.p1.x - l.p0.x);
                if det.abs() < EPSILON {
                    // Lines are coincident (or nearly so).
                    return result;
                }
                let t = dx * (p0.y - l.p0.y) - dy * (p0.x - l.p0.x);
                // t = position on self
                let t = t / det;
                if (-EPSILON..=(1.0 + EPSILON)).contains(&t) {
                    // u = position on probe line
                    let u =
                        (l.p0.x - p0.x) * (l.p1.y - l.p0.y) - (l.p0.y - p0.y) * (l.p1.x - l.p0.x);
                    let u = u / det;
                    if (0.0..=1.0).contains(&u) {
                        result.push(LineIntersection::new(u, t));
                    }
                }
            }
            PathSeg::Quad(q) => {
                // The basic technique here is to determine x and y as a quadratic polynomial
                // as a function of t. Then plug those values into the line equation for the
                // probe line (giving a sort of signed distance from the probe line) and solve
                // that for t.
                let (px0, px1, px2) = quadratic_bez_coefs(q.p0.x, q.p1.x, q.p2.x);
                let (py0, py1, py2) = quadratic_bez_coefs(q.p0.y, q.p1.y, q.p2.y);
                let c0 = dy * (px0 - p0.x) - dx * (py0 - p0.y);
                let c1 = dy * px1 - dx * py1;
                let c2 = dy * px2 - dx * py2;
                let invlen2 = (dx * dx + dy * dy).recip();
                for t in solve_quadratic(c0, c1, c2) {
                    if (-EPSILON..=(1.0 + EPSILON)).contains(&t) {
                        let x = px0 + t * px1 + t * t * px2;
                        let y = py0 + t * py1 + t * t * py2;
                        let u = ((x - p0.x) * dx + (y - p0.y) * dy) * invlen2;
                        if (0.0..=1.0).contains(&u) {
                            result.push(LineIntersection::new(u, t));
                        }
                    }
                }
            }
            PathSeg::Cubic(c) => {
                // Same technique as above, but cubic polynomial.
                let (px0, px1, px2, px3) = cubic_bez_coefs(c.p0.x, c.p1.x, c.p2.x, c.p3.x);
                let (py0, py1, py2, py3) = cubic_bez_coefs(c.p0.y, c.p1.y, c.p2.y, c.p3.y);
                let c0 = dy * (px0 - p0.x) - dx * (py0 - p0.y);
                let c1 = dy * px1 - dx * py1;
                let c2 = dy * px2 - dx * py2;
                let c3 = dy * px3 - dx * py3;
                let invlen2 = (dx * dx + dy * dy).recip();
                for t in solve_cubic(c0, c1, c2, c3) {
                    if (-EPSILON..=(1.0 + EPSILON)).contains(&t) {
                        let x = px0 + t * px1 + t * t * px2 + t * t * t * px3;
                        let y = py0 + t * py1 + t * t * py2 + t * t * t * py3;
                        let u = ((x - p0.x) * dx + (y - p0.y) * dy) * invlen2;
                        if (0.0..=1.0).contains(&u) {
                            result.push(LineIntersection::new(u, t));
                        }
                    }
                }
            }
        }
        result
    }

    /// Is this Bézier path finite?
    #[inline]
    pub fn is_finite(&self) -> bool {
        match self {
            PathSeg::Line(line) => line.is_finite(),
            PathSeg::Quad(quad_bez) => quad_bez.is_finite(),
            PathSeg::Cubic(cubic_bez) => cubic_bez.is_finite(),
        }
    }

    /// Is this Bézier path NaN?
    #[inline]
    pub fn is_nan(&self) -> bool {
        match self {
            PathSeg::Line(line) => line.is_nan(),
            PathSeg::Quad(quad_bez) => quad_bez.is_nan(),
            PathSeg::Cubic(cubic_bez) => cubic_bez.is_nan(),
        }
    }

    #[inline]
    fn as_vec2_vec(&self) -> ArrayVec<Vec2, 4> {
        let mut a = ArrayVec::new();
        match self {
            PathSeg::Line(l) => {
                a.push(l.p0.to_vec2());
                a.push(l.p1.to_vec2());
            }
            PathSeg::Quad(q) => {
                a.push(q.p0.to_vec2());
                a.push(q.p1.to_vec2());
                a.push(q.p2.to_vec2());
            }
            PathSeg::Cubic(c) => {
                a.push(c.p0.to_vec2());
                a.push(c.p1.to_vec2());
                a.push(c.p2.to_vec2());
                a.push(c.p3.to_vec2());
            }
        }
        a
    }

    /// Minimum distance between two [`PathSeg`]s.
    ///
    /// Returns a tuple of the distance, the path time `t1` of the closest point
    /// on the first `PathSeg`, and the path time `t2` of the closest point on the
    /// second `PathSeg`.
    pub fn min_dist(&self, other: PathSeg, accuracy: f64) -> MinDistance {
        let (distance, t1, t2) = crate::mindist::min_dist_param(
            &self.as_vec2_vec(),
            &other.as_vec2_vec(),
            (0.0, 1.0),
            (0.0, 1.0),
            accuracy,
            None,
        );
        MinDistance {
            distance: distance.sqrt(),
            t1,
            t2,
        }
    }

    /// Compute endpoint tangents of a path segment.
    ///
    /// This version is robust to the path segment not being a regular curve.
    pub(crate) fn tangents(&self) -> (Vec2, Vec2) {
        const EPS: f64 = 1e-12;
        match self {
            PathSeg::Line(l) => {
                let d = l.p1 - l.p0;
                (d, d)
            }
            PathSeg::Quad(q) => {
                let d01 = q.p1 - q.p0;
                let d0 = if d01.hypot2() > EPS { d01 } else { q.p2 - q.p0 };
                let d12 = q.p2 - q.p1;
                let d1 = if d12.hypot2() > EPS { d12 } else { q.p2 - q.p0 };
                (d0, d1)
            }
            PathSeg::Cubic(c) => {
                let d01 = c.p1 - c.p0;
                let d0 = if d01.hypot2() > EPS {
                    d01
                } else {
                    let d02 = c.p2 - c.p0;
                    if d02.hypot2() > EPS { d02 } else { c.p3 - c.p0 }
                };
                let d23 = c.p3 - c.p2;
                let d1 = if d23.hypot2() > EPS {
                    d23
                } else {
                    let d13 = c.p3 - c.p1;
                    if d13.hypot2() > EPS { d13 } else { c.p3 - c.p0 }
                };
                (d0, d1)
            }
        }
    }
}

impl LineIntersection {
    #[inline(always)]
    fn new(line_t: f64, segment_t: f64) -> Self {
        LineIntersection { line_t, segment_t }
    }

    /// Is this line intersection finite?
    #[inline]
    pub fn is_finite(self) -> bool {
        self.line_t.is_finite() && self.segment_t.is_finite()
    }

    /// Is this line intersection NaN?
    #[inline]
    pub fn is_nan(self) -> bool {
        self.line_t.is_nan() || self.segment_t.is_nan()
    }
}

// Return polynomial coefficients given cubic Bézier coordinates.
fn quadratic_bez_coefs(x0: f64, x1: f64, x2: f64) -> (f64, f64, f64) {
    let p0 = x0;
    let p1 = 2.0 * x1 - 2.0 * x0;
    let p2 = x2 - 2.0 * x1 + x0;
    (p0, p1, p2)
}

// Return polynomial coefficients given cubic Bézier coordinates.
fn cubic_bez_coefs(x0: f64, x1: f64, x2: f64, x3: f64) -> (f64, f64, f64, f64) {
    let p0 = x0;
    let p1 = 3.0 * x1 - 3.0 * x0;
    let p2 = 3.0 * x2 - 6.0 * x1 + 3.0 * x0;
    let p3 = x3 - 3.0 * x2 + 3.0 * x1 - x0;
    (p0, p1, p2, p3)
}

impl From<CubicBez> for PathSeg {
    #[inline(always)]
    fn from(cubic_bez: CubicBez) -> PathSeg {
        PathSeg::Cubic(cubic_bez)
    }
}

impl From<Line> for PathSeg {
    #[inline(always)]
    fn from(line: Line) -> PathSeg {
        PathSeg::Line(line)
    }
}

impl From<QuadBez> for PathSeg {
    #[inline(always)]
    fn from(quad_bez: QuadBez) -> PathSeg {
        PathSeg::Quad(quad_bez)
    }
}

impl Shape for BezPath {
    type PathElementsIter<'iter> = core::iter::Copied<core::slice::Iter<'iter, PathEl>>;

    fn path_elements(&self, _tolerance: f64) -> Self::PathElementsIter<'_> {
        self.0.iter().copied()
    }

    fn to_path(&self, _tolerance: f64) -> BezPath {
        self.clone()
    }

    #[inline(always)]
    fn into_path(self, _tolerance: f64) -> BezPath {
        self
    }

    /// Signed area.
    fn area(&self) -> f64 {
        self.elements().area()
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        self.elements().perimeter(accuracy)
    }

    /// Winding number of point.
    fn winding(&self, pt: Point) -> i32 {
        self.elements().winding(pt)
    }

    fn bounding_box(&self) -> Rect {
        self.elements().bounding_box()
    }

    #[inline(always)]
    fn as_path_slice(&self) -> Option<&[PathEl]> {
        Some(&self.0)
    }
}

impl PathEl {
    /// Is this path element finite?
    #[inline]
    pub fn is_finite(&self) -> bool {
        match self {
            PathEl::MoveTo(p) => p.is_finite(),
            PathEl::LineTo(p) => p.is_finite(),
            PathEl::QuadTo(p, p2) => p.is_finite() && p2.is_finite(),
            PathEl::CurveTo(p, p2, p3) => p.is_finite() && p2.is_finite() && p3.is_finite(),
            PathEl::ClosePath => true,
        }
    }

    /// Is this path element NaN?
    #[inline]
    pub fn is_nan(&self) -> bool {
        match self {
            PathEl::MoveTo(p) => p.is_nan(),
            PathEl::LineTo(p) => p.is_nan(),
            PathEl::QuadTo(p, p2) => p.is_nan() || p2.is_nan(),
            PathEl::CurveTo(p, p2, p3) => p.is_nan() || p2.is_nan() || p3.is_nan(),
            PathEl::ClosePath => false,
        }
    }

    /// Get the end point of the path element, if it exists.
    pub fn end_point(&self) -> Option<Point> {
        match self {
            PathEl::MoveTo(p) => Some(*p),
            PathEl::LineTo(p1) => Some(*p1),
            PathEl::QuadTo(_, p2) => Some(*p2),
            PathEl::CurveTo(_, _, p3) => Some(*p3),
            PathEl::ClosePath => None,
        }
    }
}

/// Implements [`Shape`] for a slice of [`PathEl`], provided that the first element of the slice is
/// not a `PathEl::ClosePath`. If it is, several of these functions will panic.
///
/// If the slice starts with `LineTo`, `QuadTo`, or `CurveTo`, it will be treated as a `MoveTo`.
impl<'a> Shape for &'a [PathEl] {
    type PathElementsIter<'iter>
        = core::iter::Copied<core::slice::Iter<'a, PathEl>>
    where
        'a: 'iter;

    #[inline]
    fn path_elements(&self, _tolerance: f64) -> Self::PathElementsIter<'_> {
        self.iter().copied()
    }

    fn to_path(&self, _tolerance: f64) -> BezPath {
        BezPath::from_vec(self.to_vec())
    }

    /// Signed area.
    fn area(&self) -> f64 {
        segments(self.iter().copied()).area()
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        segments(self.iter().copied()).perimeter(accuracy)
    }

    /// Winding number of point.
    fn winding(&self, pt: Point) -> i32 {
        segments(self.iter().copied()).winding(pt)
    }

    fn bounding_box(&self) -> Rect {
        segments(self.iter().copied()).bounding_box()
    }

    #[inline(always)]
    fn as_path_slice(&self) -> Option<&[PathEl]> {
        Some(self)
    }
}

/// Implements [`Shape`] for an array of [`PathEl`], provided that the first element of the array is
/// not a `PathEl::ClosePath`. If it is, several of these functions will panic.
///
/// If the array starts with `LineTo`, `QuadTo`, or `CurveTo`, it will be treated as a `MoveTo`.
impl<const N: usize> Shape for [PathEl; N] {
    type PathElementsIter<'iter> = core::iter::Copied<core::slice::Iter<'iter, PathEl>>;

    #[inline]
    fn path_elements(&self, _tolerance: f64) -> Self::PathElementsIter<'_> {
        self.iter().copied()
    }

    fn to_path(&self, _tolerance: f64) -> BezPath {
        BezPath::from_vec(self.to_vec())
    }

    /// Signed area.
    fn area(&self) -> f64 {
        segments(self.iter().copied()).area()
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        segments(self.iter().copied()).perimeter(accuracy)
    }

    /// Winding number of point.
    fn winding(&self, pt: Point) -> i32 {
        segments(self.iter().copied()).winding(pt)
    }

    fn bounding_box(&self) -> Rect {
        segments(self.iter().copied()).bounding_box()
    }

    #[inline(always)]
    fn as_path_slice(&self) -> Option<&[PathEl]> {
        Some(self)
    }
}

/// An iterator for path segments.
pub struct PathSegIter {
    seg: PathSeg,
    ix: usize,
}

impl Shape for PathSeg {
    type PathElementsIter<'iter> = PathSegIter;

    #[inline(always)]
    fn path_elements(&self, _tolerance: f64) -> PathSegIter {
        PathSegIter { seg: *self, ix: 0 }
    }

    /// The area under the curve.
    ///
    /// We could just return `0`, but this seems more useful.
    fn area(&self) -> f64 {
        self.signed_area()
    }

    #[inline]
    fn perimeter(&self, accuracy: f64) -> f64 {
        self.arclen(accuracy)
    }

    #[inline(always)]
    fn winding(&self, _pt: Point) -> i32 {
        0
    }

    #[inline]
    fn bounding_box(&self) -> Rect {
        ParamCurveExtrema::bounding_box(self)
    }

    fn as_line(&self) -> Option<Line> {
        if let PathSeg::Line(line) = self {
            Some(*line)
        } else {
            None
        }
    }
}

impl Iterator for PathSegIter {
    type Item = PathEl;

    fn next(&mut self) -> Option<PathEl> {
        self.ix += 1;
        match (self.ix, self.seg) {
            // yes I could do some fancy bindings thing here but... :shrug:
            (1, PathSeg::Line(seg)) => Some(PathEl::MoveTo(seg.p0)),
            (1, PathSeg::Quad(seg)) => Some(PathEl::MoveTo(seg.p0)),
            (1, PathSeg::Cubic(seg)) => Some(PathEl::MoveTo(seg.p0)),
            (2, PathSeg::Line(seg)) => Some(PathEl::LineTo(seg.p1)),
            (2, PathSeg::Quad(seg)) => Some(PathEl::QuadTo(seg.p1, seg.p2)),
            (2, PathSeg::Cubic(seg)) => Some(PathEl::CurveTo(seg.p1, seg.p2, seg.p3)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Circle, DEFAULT_ACCURACY};

    use super::*;

    fn assert_approx_eq(x: f64, y: f64) {
        assert!((x - y).abs() < 1e-8, "{x} != {y}");
    }

    #[test]
    #[should_panic(expected = "uninitialized subpath")]
    fn test_elements_to_segments_starts_on_closepath() {
        let mut path = BezPath::new();
        path.close_path();
        path.segments().next();
    }

    #[test]
    fn test_elements_to_segments_closepath_refers_to_last_moveto() {
        let mut path = BezPath::new();
        path.move_to((5.0, 5.0));
        path.line_to((15.0, 15.0));
        path.move_to((10.0, 10.0));
        path.line_to((15.0, 15.0));
        path.close_path();
        assert_eq!(
            path.segments().collect::<Vec<_>>().last(),
            Some(&Line::new((15.0, 15.0), (10.0, 10.0)).into()),
        );
    }

    #[test]
    #[should_panic(expected = "uninitialized subpath")]
    fn test_must_not_start_on_quad() {
        let mut path = BezPath::new();
        path.quad_to((5.0, 5.0), (10.0, 10.0));
        path.line_to((15.0, 15.0));
        path.close_path();
    }

    #[test]
    fn test_intersect_line() {
        let h_line = Line::new((0.0, 0.0), (100.0, 0.0));
        let v_line = Line::new((10.0, -10.0), (10.0, 10.0));
        let intersection = PathSeg::Line(h_line).intersect_line(v_line)[0];
        assert_approx_eq(intersection.segment_t, 0.1);
        assert_approx_eq(intersection.line_t, 0.5);

        let v_line = Line::new((-10.0, -10.0), (-10.0, 10.0));
        assert!(PathSeg::Line(h_line).intersect_line(v_line).is_empty());

        let v_line = Line::new((10.0, 10.0), (10.0, 20.0));
        assert!(PathSeg::Line(h_line).intersect_line(v_line).is_empty());
    }

    #[test]
    fn test_intersect_qad() {
        let q = QuadBez::new((0.0, -10.0), (10.0, 20.0), (20.0, -10.0));
        let v_line = Line::new((10.0, -10.0), (10.0, 10.0));
        assert_eq!(PathSeg::Quad(q).intersect_line(v_line).len(), 1);
        let intersection = PathSeg::Quad(q).intersect_line(v_line)[0];
        assert_approx_eq(intersection.segment_t, 0.5);
        assert_approx_eq(intersection.line_t, 0.75);

        let h_line = Line::new((0.0, 0.0), (100.0, 0.0));
        assert_eq!(PathSeg::Quad(q).intersect_line(h_line).len(), 2);
    }

    #[test]
    fn test_intersect_cubic() {
        let c = CubicBez::new((0.0, -10.0), (10.0, 20.0), (20.0, -20.0), (30.0, 10.0));
        let v_line = Line::new((10.0, -10.0), (10.0, 10.0));
        assert_eq!(PathSeg::Cubic(c).intersect_line(v_line).len(), 1);
        let intersection = PathSeg::Cubic(c).intersect_line(v_line)[0];
        assert_approx_eq(intersection.segment_t, 0.333333333);
        assert_approx_eq(intersection.line_t, 0.592592592);

        let h_line = Line::new((0.0, 0.0), (100.0, 0.0));
        assert_eq!(PathSeg::Cubic(c).intersect_line(h_line).len(), 3);
    }

    #[test]
    fn test_contains() {
        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.line_to((1.0, 1.0));
        path.line_to((2.0, 0.0));
        path.close_path();
        assert_eq!(path.winding(Point::new(1.0, 0.5)), -1);
        assert!(path.contains(Point::new(1.0, 0.5)));
    }

    // get_seg(i) should produce the same results as path_segments().nth(i - 1).
    #[test]
    fn test_get_seg() {
        let circle = Circle::new((10.0, 10.0), 2.0).to_path(DEFAULT_ACCURACY);
        let segments = circle.path_segments(DEFAULT_ACCURACY).collect::<Vec<_>>();
        let get_segs = (1..usize::MAX)
            .map_while(|i| circle.get_seg(i))
            .collect::<Vec<_>>();
        assert_eq!(segments, get_segs);
    }

    #[test]
    fn test_control_box() {
        // a sort of map ping looking thing drawn with a single cubic
        // cbox is wildly different than tight box
        let path = BezPath::from_svg("M200,300 C50,50 350,50 200,300").unwrap();
        assert_eq!(Rect::new(50.0, 50.0, 350.0, 300.0), path.control_box());
        assert!(path.control_box().area() > path.bounding_box().area());
    }

    #[test]
    fn test_subpaths() {
        let path = BezPath::from_svg("M10,10 L0,10 L0,0 L10,0 Z M100,100 M30,0 Q35,10,40,0 L30,0")
            .unwrap();
        assert_eq!(
            vec![
                BezPath::from_svg("M10,10 L0,10 L0,0 L10,0 Z").unwrap(),
                BezPath::from_svg("M100,100").unwrap(),
                BezPath::from_svg("M30,0 Q35,10,40,0 L30,0").unwrap(),
            ],
            path.subpaths()
                .map(|sp| BezPath::from_vec(sp.to_vec()))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_reverse_unclosed() {
        let path = BezPath::from_svg("M10,10 Q40,40 60,10 L100,10 C125,10 150,50 125,60").unwrap();
        let reversed = path.reverse_subpaths();
        assert_eq!(
            "M125,60 C150,50 125,10 100,10 L60,10 Q40,40 10,10",
            reversed.to_svg()
        );
    }

    #[test]
    fn test_reverse_closed_triangle() {
        let path = BezPath::from_svg("M100,100 L150,200 L50,200 Z").unwrap();
        let reversed = path.reverse_subpaths();
        assert_eq!("M50,200 L150,200 L100,100 Z", reversed.to_svg());
    }

    #[test]
    fn test_reverse_closed_shape() {
        let path = BezPath::from_svg(
            "M125,100 Q200,150 175,300 C150,150 50,150 25,300 Q0,150 75,100 L100,50 Z",
        )
        .unwrap();
        let reversed = path.reverse_subpaths();
        assert_eq!(
            "M100,50 L75,100 Q0,150 25,300 C50,150 150,150 175,300 Q200,150 125,100 Z",
            reversed.to_svg()
        );
    }

    #[test]
    fn test_reverse_multiple_subpaths() {
        let svg = "M10,10 Q40,40 60,10 L100,10 C125,10 150,50 125,60 M100,100 L150,200 L50,200 Z M125,100 Q200,150 175,300 C150,150 50,150 25,300 Q0,150 75,100 L100,50 Z";
        let expected_svg = "M125,60 C150,50 125,10 100,10 L60,10 Q40,40 10,10 M50,200 L150,200 L100,100 Z M100,50 L75,100 Q0,150 25,300 C50,150 150,150 175,300 Q200,150 125,100 Z";
        let path = BezPath::from_svg(svg).unwrap();
        let reversed = path.reverse_subpaths();
        assert_eq!(expected_svg, reversed.to_svg());
    }

    // https://github.com/fonttools/fonttools/blob/bf265ce49e0cae6f032420a4c80c31d8e16285b8/Tests/pens/reverseContourPen_test.py#L7
    #[test]
    fn test_reverse_lines() {
        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.line_to((1.0, 1.0));
        path.line_to((2.0, 2.0));
        path.line_to((3.0, 3.0));
        path.close_path();
        let rev = path.reverse_subpaths();
        assert_eq!("M3,3 L2,2 L1,1 L0,0 Z", rev.to_svg());
    }

    #[test]
    fn test_reverse_multiple_moves() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((2.0, 2.0).into()),
                PathEl::MoveTo((3.0, 3.0).into()),
                PathEl::ClosePath,
                PathEl::MoveTo((4.0, 4.0).into()),
            ],
            vec![
                PathEl::MoveTo((2.0, 2.0).into()),
                PathEl::MoveTo((3.0, 3.0).into()),
                PathEl::ClosePath,
                PathEl::MoveTo((4.0, 4.0).into()),
            ],
        );
    }

    // The following are direct port of fonttools'
    // reverseContourPen_test.py::test_reverse_pen, adapted to rust, excluding
    // test cases that don't apply because we don't implement
    // outputImpliedClosingLine=False.
    // https://github.com/fonttools/fonttools/blob/85c80be/Tests/pens/reverseContourPen_test.py#L6-L467

    #[test]
    fn test_reverse_closed_last_line_not_on_move() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((1.0, 1.0).into()),
                PathEl::LineTo((2.0, 2.0).into()),
                PathEl::LineTo((3.0, 3.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((3.0, 3.0).into()),
                PathEl::LineTo((2.0, 2.0).into()),
                PathEl::LineTo((1.0, 1.0).into()),
                PathEl::LineTo((0.0, 0.0).into()), // closing line NOT implied
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_closed_last_line_overlaps_move() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((1.0, 1.0).into()),
                PathEl::LineTo((2.0, 2.0).into()),
                PathEl::LineTo((0.0, 0.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((2.0, 2.0).into()),
                PathEl::LineTo((1.0, 1.0).into()),
                PathEl::LineTo((0.0, 0.0).into()), // closing line NOT implied
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_closed_duplicate_line_following_move() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((0.0, 0.0).into()),
                PathEl::LineTo((1.0, 1.0).into()),
                PathEl::LineTo((2.0, 2.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((2.0, 2.0).into()),
                PathEl::LineTo((1.0, 1.0).into()),
                PathEl::LineTo((0.0, 0.0).into()), // duplicate line retained
                PathEl::LineTo((0.0, 0.0).into()),
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_closed_two_lines() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((1.0, 1.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((1.0, 1.0).into()),
                PathEl::LineTo((0.0, 0.0).into()), // closing line NOT implied
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_closed_last_curve_overlaps_move() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::CurveTo((1.0, 1.0).into(), (2.0, 2.0).into(), (3.0, 3.0).into()),
                PathEl::CurveTo((4.0, 4.0).into(), (5.0, 5.0).into(), (0.0, 0.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((0.0, 0.0).into()), // no extra lineTo added here
                PathEl::CurveTo((5.0, 5.0).into(), (4.0, 4.0).into(), (3.0, 3.0).into()),
                PathEl::CurveTo((2.0, 2.0).into(), (1.0, 1.0).into(), (0.0, 0.0).into()),
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_closed_last_curve_not_on_move() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::CurveTo((1.0, 1.0).into(), (2.0, 2.0).into(), (3.0, 3.0).into()),
                PathEl::CurveTo((4.0, 4.0).into(), (5.0, 5.0).into(), (6.0, 6.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((6.0, 6.0).into()), // the previously implied line
                PathEl::CurveTo((5.0, 5.0).into(), (4.0, 4.0).into(), (3.0, 3.0).into()),
                PathEl::CurveTo((2.0, 2.0).into(), (1.0, 1.0).into(), (0.0, 0.0).into()),
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_closed_line_curve_line() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((1.0, 1.0).into()), // this line...
                PathEl::CurveTo((2.0, 2.0).into(), (3.0, 3.0).into(), (4.0, 4.0).into()),
                PathEl::CurveTo((5.0, 5.0).into(), (6.0, 6.0).into(), (7.0, 7.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((7.0, 7.0).into()),
                PathEl::CurveTo((6.0, 6.0).into(), (5.0, 5.0).into(), (4.0, 4.0).into()),
                PathEl::CurveTo((3.0, 3.0).into(), (2.0, 2.0).into(), (1.0, 1.0).into()),
                PathEl::LineTo((0.0, 0.0).into()), // ... does NOT become implied
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_closed_last_quad_overlaps_move() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::QuadTo((1.0, 1.0).into(), (2.0, 2.0).into()),
                PathEl::QuadTo((3.0, 3.0).into(), (0.0, 0.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((0.0, 0.0).into()), // no extra lineTo added here
                PathEl::QuadTo((3.0, 3.0).into(), (2.0, 2.0).into()),
                PathEl::QuadTo((1.0, 1.0).into(), (0.0, 0.0).into()),
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_closed_last_quad_not_on_move() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::QuadTo((1.0, 1.0).into(), (2.0, 2.0).into()),
                PathEl::QuadTo((3.0, 3.0).into(), (4.0, 4.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((4.0, 4.0).into()), // the previously implied line
                PathEl::QuadTo((3.0, 3.0).into(), (2.0, 2.0).into()),
                PathEl::QuadTo((1.0, 1.0).into(), (0.0, 0.0).into()),
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_closed_line_quad_line() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((1.0, 1.0).into()), // this line...
                PathEl::QuadTo((2.0, 2.0).into(), (3.0, 3.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((3.0, 3.0).into()),
                PathEl::QuadTo((2.0, 2.0).into(), (1.0, 1.0).into()),
                PathEl::LineTo((0.0, 0.0).into()), // ... does NOT become implied
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_empty() {
        reverse_test_helper(vec![], vec![]);
    }

    #[test]
    fn test_reverse_single_point() {
        reverse_test_helper(
            vec![PathEl::MoveTo((0.0, 0.0).into())],
            vec![PathEl::MoveTo((0.0, 0.0).into())],
        );
    }

    #[test]
    fn test_reverse_single_point_closed() {
        reverse_test_helper(
            vec![PathEl::MoveTo((0.0, 0.0).into()), PathEl::ClosePath],
            vec![PathEl::MoveTo((0.0, 0.0).into()), PathEl::ClosePath],
        );
    }

    #[test]
    fn test_reverse_single_line_open() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((1.0, 1.0).into()),
            ],
            vec![
                PathEl::MoveTo((1.0, 1.0).into()),
                PathEl::LineTo((0.0, 0.0).into()),
            ],
        );
    }

    #[test]
    fn test_reverse_single_curve_open() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::CurveTo((1.0, 1.0).into(), (2.0, 2.0).into(), (3.0, 3.0).into()),
            ],
            vec![
                PathEl::MoveTo((3.0, 3.0).into()),
                PathEl::CurveTo((2.0, 2.0).into(), (1.0, 1.0).into(), (0.0, 0.0).into()),
            ],
        );
    }

    #[test]
    fn test_reverse_curve_line_open() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::CurveTo((1.0, 1.0).into(), (2.0, 2.0).into(), (3.0, 3.0).into()),
                PathEl::LineTo((4.0, 4.0).into()),
            ],
            vec![
                PathEl::MoveTo((4.0, 4.0).into()),
                PathEl::LineTo((3.0, 3.0).into()),
                PathEl::CurveTo((2.0, 2.0).into(), (1.0, 1.0).into(), (0.0, 0.0).into()),
            ],
        );
    }

    #[test]
    fn test_reverse_line_curve_open() {
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((1.0, 1.0).into()),
                PathEl::CurveTo((2.0, 2.0).into(), (3.0, 3.0).into(), (4.0, 4.0).into()),
            ],
            vec![
                PathEl::MoveTo((4.0, 4.0).into()),
                PathEl::CurveTo((3.0, 3.0).into(), (2.0, 2.0).into(), (1.0, 1.0).into()),
                PathEl::LineTo((0.0, 0.0).into()),
            ],
        );
    }

    #[test]
    fn test_reverse_duplicate_point_after_move() {
        // Test case from: https://github.com/googlei18n/cu2qu/issues/51#issue-179370514
        // Simplified to only use atomic PathEl::QuadTo (no QuadSplines).
        reverse_test_helper(
            vec![
                PathEl::MoveTo((848.0, 348.0).into()),
                PathEl::LineTo((848.0, 348.0).into()),
                PathEl::QuadTo((848.0, 526.0).into(), (449.0, 704.0).into()),
                PathEl::QuadTo((848.0, 171.0).into(), (848.0, 348.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((848.0, 348.0).into()),
                PathEl::QuadTo((848.0, 171.0).into(), (449.0, 704.0).into()),
                PathEl::QuadTo((848.0, 526.0).into(), (848.0, 348.0).into()),
                PathEl::LineTo((848.0, 348.0).into()),
                PathEl::ClosePath,
            ],
        );
    }

    #[test]
    fn test_reverse_duplicate_point_at_end() {
        // Test case from: https://github.com/googlefonts/fontmake/issues/572
        reverse_test_helper(
            vec![
                PathEl::MoveTo((0.0, 651.0).into()),
                PathEl::LineTo((0.0, 101.0).into()),
                PathEl::LineTo((0.0, 101.0).into()),
                PathEl::LineTo((0.0, 651.0).into()),
                PathEl::LineTo((0.0, 651.0).into()),
                PathEl::ClosePath,
            ],
            vec![
                PathEl::MoveTo((0.0, 651.0).into()),
                PathEl::LineTo((0.0, 651.0).into()),
                PathEl::LineTo((0.0, 101.0).into()),
                PathEl::LineTo((0.0, 101.0).into()),
                PathEl::LineTo((0.0, 651.0).into()),
                PathEl::ClosePath,
            ],
        );
    }

    fn reverse_test_helper(contour: Vec<PathEl>, expected: Vec<PathEl>) {
        assert_eq!(BezPath(contour).reverse_subpaths().0, expected);
    }

    #[test]
    fn test_rect_segments() {
        // Ensure that `from_path_segments` does not insert spurious MoveTos in
        // the middle of a path.
        let x0 = 25.189500810000002;
        let x1 = 568.18950081;
        let y0 = -105.0;
        let y1 = 176.0;
        let r = Rect::from_points((x0, y0), (x1, y1));

        let path0 = r.into_path(0.0);
        assert!(
            path0
                .elements()
                .iter()
                .skip(1)
                .all(|el| !matches!(el, PathEl::MoveTo(_)))
        );

        let path1 = BezPath::from_path_segments(path0.segments());
        assert!(
            path1
                .elements()
                .iter()
                .skip(1)
                .all(|el| !matches!(el, PathEl::MoveTo(_)))
        );
    }

    #[test]
    fn test_current_position() {
        let mut path = BezPath::new();
        assert_eq!(path.current_position(), None);
        path.move_to((0., 0.));
        assert_eq!(path.current_position(), Some(Point::new(0., 0.)));
        path.line_to((10., 10.));
        assert_eq!(path.current_position(), Some(Point::new(10., 10.)));
        path.line_to((10., 0.));
        assert_eq!(path.current_position(), Some(Point::new(10., 0.)));
        path.close_path();
        assert_eq!(path.current_position(), Some(Point::new(0., 0.)));

        path.close_path();
        assert_eq!(path.current_position(), None);

        path.move_to((0., 10.));
        assert_eq!(path.current_position(), Some(Point::new(0., 10.)));
        path.close_path();
        assert_eq!(path.current_position(), Some(Point::new(0., 10.)));
        path.close_path();
        assert_eq!(path.current_position(), None);
    }

    // Regression test for #531
    //
    // When testing the winding number of a point whose y coordinate is very
    // close (or equal to) the y coordinate of a segment's start or end, we need
    // to be consistent about how we treat it. In particular, in #531 we noticed
    // that the point was within the y range of a monotonic segment, but because
    // of rounding errors the cubic solver disagreed.
    #[test]
    fn winding_endpoints() {
        let bez = BezPath::from_vec(vec![
            PathEl::MoveTo((200.0, 410.0).into()),
            PathEl::CurveTo(
                (139.0, 410.0).into(),
                (90.0, 360.8772277832031).into(),
                (90.0, 300.0).into(),
            ),
            PathEl::CurveTo(
                (90.0, 239.0).into(),
                (139.0, 190.0).into(),
                (200.0, 190.0).into(),
            ),
            PathEl::CurveTo(
                (150.0, 210.0).into(),
                (110.0, 250.0).into(),
                (110.0, 300.0).into(),
            ),
            PathEl::CurveTo(
                (110.0, 349.0).into(),
                (150.0, 390.0).into(),
                (200.0, 390.0).into(),
            ),
            PathEl::ClosePath,
        ]);

        assert!(bez.contains((100.0, 300.1).into()));
        assert!(bez.contains((100.0, 299.9).into()));
        assert!(bez.contains((100.0, 300.0).into()));
    }

    #[test]
    fn check_close_subpaths() {
        let mut bez = BezPath::new();
        bez.move_to((10.0, 10.0));
        bez.line_to((100.0, 20.0));
        bez.line_to((60.0, 100.0));
        let elements = close_subpaths(&bez).collect::<Vec<_>>();
        bez.close_path();
        assert_eq!(&elements, bez.elements());

        // Test implicit closepath in middle of path
        let mut bez2 = BezPath::new();
        bez2.move_to((10.0, 10.0));
        bez2.line_to((100.0, 20.0));
        bez2.line_to((60.0, 100.0));
        bez2.move_to((110.0, 10.0));
        bez2.line_to((200.0, 20.0));
        bez2.line_to((160.0, 100.0));
        let elements2 = close_subpaths(&bez2).collect::<Vec<_>>();
        let mut bez3 = BezPath::new();
        bez3.move_to((10.0, 10.0));
        bez3.line_to((100.0, 20.0));
        bez3.line_to((60.0, 100.0));
        bez3.close_path();
        bez3.move_to((110.0, 10.0));
        bez3.line_to((200.0, 20.0));
        bez3.line_to((160.0, 100.0));
        bez3.close_path();
        assert_eq!(&elements2, bez3.elements());
    }

    #[test]
    fn close_subpaths_does_not_duplicate_existing_closepath() {
        let path = BezPath::from_vec(vec![
            PathEl::MoveTo((0.0, 0.0).into()),
            PathEl::LineTo((10.0, 0.0).into()),
            PathEl::LineTo((10.0, 10.0).into()),
            PathEl::ClosePath,
            PathEl::MoveTo((20.0, 0.0).into()),
            PathEl::LineTo((30.0, 0.0).into()),
        ]);

        let closed = close_subpaths(path.iter()).collect::<Vec<_>>();

        assert_eq!(
            closed,
            vec![
                PathEl::MoveTo((0.0, 0.0).into()),
                PathEl::LineTo((10.0, 0.0).into()),
                PathEl::LineTo((10.0, 10.0).into()),
                PathEl::ClosePath,
                PathEl::MoveTo((20.0, 0.0).into()),
                PathEl::LineTo((30.0, 0.0).into()),
                PathEl::ClosePath,
            ]
        );
    }
}
