// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use alloc::vec::Vec;

use crate::path_builder::PathBuilder;
use crate::transform::Transform;
use crate::{Point, Rect};

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use crate::NoStdFloat;

/// A path verb.
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum PathVerb {
    Move,
    Line,
    Quad,
    Cubic,
    Close,
}

/// A Bezier path.
///
/// Can be created via [`PathBuilder`].
/// Where [`PathBuilder`] can be created from the [`Path`] using [`clear`] to reuse the allocation.
///
/// Path is immutable and uses compact storage, where segment types and numbers are stored
/// separately. Use can access path segments via [`Path::verbs`] and [`Path::points`],
/// or via [`Path::segments`]
///
/// # Guarantees
///
/// - Has a valid, precomputed bounds.
/// - All points are finite.
/// - Has at least two segments.
/// - Each contour starts with a MoveTo.
/// - No duplicated Move.
/// - No duplicated Close.
/// - Zero-length contours are allowed.
///
/// [`PathBuilder`]: struct.PathBuilder.html
/// [`clear`]: struct.Path.html#method.clear
#[derive(Clone, PartialEq)]
pub struct Path {
    pub(crate) verbs: Vec<PathVerb>,
    pub(crate) points: Vec<Point>,
    pub(crate) bounds: Rect,
}

impl Path {
    /// Returns the number of segments in the path.
    pub fn len(&self) -> usize {
        self.verbs.len()
    }

    /// Return if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.verbs.is_empty()
    }

    /// Returns the bounds of the path's points.
    ///
    /// The value is already calculated.
    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    /// Calculates path's tight bounds.
    ///
    /// This operation can be expensive.
    pub fn compute_tight_bounds(&self) -> Option<Rect> {
        // big enough to hold worst-case curve type (cubic) extremas + 1
        let mut extremas = [Point::zero(); 5];

        let mut min = self.points[0];
        let mut max = self.points[0];
        let mut iter = self.segments();
        let mut last_point = Point::zero();
        while let Some(segment) = iter.next() {
            let mut count = 0;
            match segment {
                PathSegment::MoveTo(p) => {
                    extremas[0] = p;
                    count = 1;
                }
                PathSegment::LineTo(p) => {
                    extremas[0] = p;
                    count = 1;
                }
                PathSegment::QuadTo(p0, p1) => {
                    count = compute_quad_extremas(last_point, p0, p1, &mut extremas);
                }
                PathSegment::CubicTo(p0, p1, p2) => {
                    count = compute_cubic_extremas(last_point, p0, p1, p2, &mut extremas);
                }
                PathSegment::Close => {}
            }

            last_point = iter.last_point;
            for tmp in &extremas[0..count] {
                min.x = min.x.min(tmp.x);
                min.y = min.y.min(tmp.y);
                max.x = max.x.max(tmp.x);
                max.y = max.y.max(tmp.y);
            }
        }

        Rect::from_ltrb(min.x, min.y, max.x, max.y)
    }

    /// Returns an internal vector of verbs.
    pub fn verbs(&self) -> &[PathVerb] {
        &self.verbs
    }

    /// Returns an internal vector of points.
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// Returns a transformed in-place path.
    ///
    /// Some points may become NaN/inf therefore this method can fail.
    pub fn transform(mut self, ts: Transform) -> Option<Self> {
        if ts.is_identity() {
            return Some(self);
        }

        ts.map_points(&mut self.points);

        // Update bounds.
        self.bounds = Rect::from_points(&self.points)?;

        Some(self)
    }

    /// Returns an iterator over path's segments.
    pub fn segments(&self) -> PathSegmentsIter {
        PathSegmentsIter {
            path: self,
            verb_index: 0,
            points_index: 0,
            is_auto_close: false,
            last_move_to: Point::zero(),
            last_point: Point::zero(),
        }
    }

    /// Clears the path and returns a `PathBuilder` that will reuse an allocated memory.
    pub fn clear(mut self) -> PathBuilder {
        self.verbs.clear();
        self.points.clear();

        PathBuilder {
            verbs: self.verbs,
            points: self.points,
            last_move_to_index: 0,
            move_to_required: true,
        }
    }
}

impl core::fmt::Debug for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;

        let mut s = alloc::string::String::new();
        for segment in self.segments() {
            match segment {
                PathSegment::MoveTo(p) => s.write_fmt(format_args!("M {} {} ", p.x, p.y))?,
                PathSegment::LineTo(p) => s.write_fmt(format_args!("L {} {} ", p.x, p.y))?,
                PathSegment::QuadTo(p0, p1) => {
                    s.write_fmt(format_args!("Q {} {} {} {} ", p0.x, p0.y, p1.x, p1.y))?
                }
                PathSegment::CubicTo(p0, p1, p2) => s.write_fmt(format_args!(
                    "C {} {} {} {} {} {} ",
                    p0.x, p0.y, p1.x, p1.y, p2.x, p2.y
                ))?,
                PathSegment::Close => s.write_fmt(format_args!("Z "))?,
            }
        }

        s.pop(); // ' '

        f.debug_struct("Path")
            .field("segments", &s)
            .field("bounds", &self.bounds)
            .finish()
    }
}

fn compute_quad_extremas(p0: Point, p1: Point, p2: Point, extremas: &mut [Point; 5]) -> usize {
    use crate::path_geometry;

    let src = [p0, p1, p2];
    let mut extrema_idx = 0;
    if let Some(t) = path_geometry::find_quad_extrema(p0.x, p1.x, p2.x) {
        extremas[extrema_idx] = path_geometry::eval_quad_at(&src, t.to_normalized());
        extrema_idx += 1;
    }
    if let Some(t) = path_geometry::find_quad_extrema(p0.y, p1.y, p2.y) {
        extremas[extrema_idx] = path_geometry::eval_quad_at(&src, t.to_normalized());
        extrema_idx += 1;
    }
    extremas[extrema_idx] = p2;
    extrema_idx + 1
}

fn compute_cubic_extremas(
    p0: Point,
    p1: Point,
    p2: Point,
    p3: Point,
    extremas: &mut [Point; 5],
) -> usize {
    use crate::path_geometry;

    let mut ts0 = path_geometry::new_t_values();
    let mut ts1 = path_geometry::new_t_values();
    let n0 = path_geometry::find_cubic_extrema(p0.x, p1.x, p2.x, p3.x, &mut ts0);
    let n1 = path_geometry::find_cubic_extrema(p0.y, p1.y, p2.y, p3.y, &mut ts1);
    let total_len = n0 + n1;
    debug_assert!(total_len <= 4);

    let src = [p0, p1, p2, p3];
    let mut extrema_idx = 0;
    for t in &ts0[0..n0] {
        extremas[extrema_idx] = path_geometry::eval_cubic_pos_at(&src, t.to_normalized());
        extrema_idx += 1;
    }
    for t in &ts1[0..n1] {
        extremas[extrema_idx] = path_geometry::eval_cubic_pos_at(&src, t.to_normalized());
        extrema_idx += 1;
    }
    extremas[total_len] = p3;
    total_len + 1
}

/// A path segment.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PathSegment {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CubicTo(Point, Point, Point),
    Close,
}

/// A path segments iterator.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct PathSegmentsIter<'a> {
    path: &'a Path,
    verb_index: usize,
    points_index: usize,

    is_auto_close: bool,
    last_move_to: Point,
    last_point: Point,
}

impl<'a> PathSegmentsIter<'a> {
    /// Sets the auto closing mode. Off by default.
    ///
    /// When enabled, emits an additional `PathSegment::Line` from the current position
    /// to the previous `PathSegment::Move`. And only then emits `PathSegment::Close`.
    pub fn set_auto_close(&mut self, flag: bool) {
        self.is_auto_close = flag;
    }

    pub(crate) fn auto_close(&mut self) -> PathSegment {
        if self.is_auto_close && self.last_point != self.last_move_to {
            self.verb_index -= 1;
            PathSegment::LineTo(self.last_move_to)
        } else {
            PathSegment::Close
        }
    }

    pub(crate) fn has_valid_tangent(&self) -> bool {
        let mut iter = self.clone();
        while let Some(segment) = iter.next() {
            match segment {
                PathSegment::MoveTo(_) => {
                    return false;
                }
                PathSegment::LineTo(p) => {
                    if iter.last_point == p {
                        continue;
                    }

                    return true;
                }
                PathSegment::QuadTo(p1, p2) => {
                    if iter.last_point == p1 && iter.last_point == p2 {
                        continue;
                    }

                    return true;
                }
                PathSegment::CubicTo(p1, p2, p3) => {
                    if iter.last_point == p1 && iter.last_point == p2 && iter.last_point == p3 {
                        continue;
                    }

                    return true;
                }
                PathSegment::Close => {
                    return false;
                }
            }
        }

        false
    }

    /// Returns the current verb.
    pub fn curr_verb(&self) -> PathVerb {
        self.path.verbs[self.verb_index - 1]
    }

    /// Returns the next verb.
    pub fn next_verb(&self) -> Option<PathVerb> {
        self.path.verbs.get(self.verb_index).cloned()
    }
}

impl<'a> Iterator for PathSegmentsIter<'a> {
    type Item = PathSegment;

    fn next(&mut self) -> Option<Self::Item> {
        if self.verb_index < self.path.verbs.len() {
            let verb = self.path.verbs[self.verb_index];
            self.verb_index += 1;

            match verb {
                PathVerb::Move => {
                    self.points_index += 1;
                    self.last_move_to = self.path.points[self.points_index - 1];
                    self.last_point = self.last_move_to;
                    Some(PathSegment::MoveTo(self.last_move_to))
                }
                PathVerb::Line => {
                    self.points_index += 1;
                    self.last_point = self.path.points[self.points_index - 1];
                    Some(PathSegment::LineTo(self.last_point))
                }
                PathVerb::Quad => {
                    self.points_index += 2;
                    self.last_point = self.path.points[self.points_index - 1];
                    Some(PathSegment::QuadTo(
                        self.path.points[self.points_index - 2],
                        self.last_point,
                    ))
                }
                PathVerb::Cubic => {
                    self.points_index += 3;
                    self.last_point = self.path.points[self.points_index - 1];
                    Some(PathSegment::CubicTo(
                        self.path.points[self.points_index - 3],
                        self.path.points[self.points_index - 2],
                        self.last_point,
                    ))
                }
                PathVerb::Close => {
                    let seg = self.auto_close();
                    self.last_point = self.last_move_to;
                    Some(seg)
                }
            }
        } else {
            None
        }
    }
}
