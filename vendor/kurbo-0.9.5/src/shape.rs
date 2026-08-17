// Copyright 2019 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A generic trait for shapes.

use crate::{segments, BezPath, Circle, Line, PathEl, Point, Rect, RoundedRect, Segments};

/// A generic trait for open and closed shapes.
///
/// This trait provides conversion from shapes to [`BezPath`]s, as well as
/// general geometry functionality like computing [`area`], [`bounding_box`]es,
/// and [`winding`] number.
///
/// [`area`]: Shape::area
/// [`bounding_box`]: Shape::bounding_box
/// [`winding`]: Shape::winding
pub trait Shape: Sized {
    /// The iterator returned by the [`path_elements`] method.
    ///
    /// [`path_elements`]: Shape::path_elements
    type PathElementsIter<'iter>: Iterator<Item = PathEl> + 'iter
    where
        Self: 'iter;

    /// Returns an iterator over this shape expressed as [`PathEl`]s;
    /// that is, as Bézier path _elements_.
    ///
    /// All shapes can be represented as Béziers, but in many situations
    /// (such as when interfacing with a platform drawing API) there are more
    /// efficient native types for specific concrete shapes. In this case,
    /// the user should exhaust the `as_` methods ([`as_rect`], [`as_line`], etc)
    /// before converting to a [`BezPath`], as those are likely to be more
    /// efficient.
    ///
    /// In many cases, shapes are able to iterate their elements without
    /// allocating; however creating a [`BezPath`] object always allocates.
    /// If you need an owned [`BezPath`] you can use [`to_path`] instead.
    ///
    /// # Tolerance
    ///
    /// The `tolerance` parameter controls the accuracy of
    /// conversion of geometric primitives to Bézier curves, as
    /// curves such as circles cannot be represented exactly but
    /// only approximated. For drawing as in UI elements, a value
    /// of 0.1 is appropriate, as it is unlikely to be visible to
    /// the eye. For scientific applications, a smaller value
    /// might be appropriate. Note that in general the number of
    /// cubic Bézier segments scales as `tolerance ^ (-1/6)`.
    ///
    /// [`as_rect`]: Shape::as_rect
    /// [`as_line`]: Shape::as_line
    /// [`to_path`]: Shape::to_path
    fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_>;

    /// Convert to a Bézier path.
    ///
    /// This always allocates. It is appropriate when both the source
    /// shape and the resulting path are to be retained.
    ///
    /// If you only need to iterate the elements (such as to convert them to
    /// drawing commands for a given 2D graphics API) you should prefer
    /// [`path_elements`], which can avoid allocating where possible.
    ///
    /// The `tolerance` parameter is the same as for [`path_elements`].
    ///
    /// [`path_elements`]: Shape::path_elements
    fn to_path(&self, tolerance: f64) -> BezPath {
        self.path_elements(tolerance).collect()
    }

    #[deprecated(since = "0.7.0", note = "Use path_elements instead")]
    #[doc(hidden)]
    fn to_bez_path(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
        self.path_elements(tolerance)
    }
    /// Convert into a Bézier path.
    ///
    /// This allocates in the general case, but is zero-cost if the
    /// shape is already a [`BezPath`].
    ///
    /// The `tolerance` parameter is the same as for [`path_elements()`].
    ///
    /// [`path_elements()`]: Shape::path_elements
    fn into_path(self, tolerance: f64) -> BezPath {
        self.to_path(tolerance)
    }

    #[deprecated(since = "0.7.0", note = "Use into_path instead")]
    #[doc(hidden)]
    fn into_bez_path(self, tolerance: f64) -> BezPath {
        self.into_path(tolerance)
    }

    /// Returns an iterator over this shape expressed as Bézier path
    /// _segments_ ([`PathSeg`]s).
    ///
    /// The allocation behaviour and `tolerance` parameter are the
    /// same as for [`path_elements()`]
    ///
    /// [`PathSeg`]: crate::PathSeg
    /// [`path_elements()`]: Shape::path_elements
    fn path_segments(&self, tolerance: f64) -> Segments<Self::PathElementsIter<'_>> {
        segments(self.path_elements(tolerance))
    }

    /// Signed area.
    ///
    /// This method only produces meaningful results with closed shapes.
    ///
    /// The convention for positive area is that y increases when x is
    /// positive. Thus, it is clockwise when down is increasing y (the
    /// usual convention for graphics), and anticlockwise when
    /// up is increasing y (the usual convention for math).
    fn area(&self) -> f64;

    /// Total length of perimeter.
    //FIXME: document the accuracy param
    fn perimeter(&self, accuracy: f64) -> f64;

    /// The [winding number] of a point.
    ///
    /// This method only produces meaningful results with closed shapes.
    ///
    /// The sign of the winding number is consistent with that of [`area`],
    /// meaning it is +1 when the point is inside a positive area shape
    /// and -1 when it is inside a negative area shape. Of course, greater
    /// magnitude values are also possible when the shape is more complex.
    ///
    /// [`area`]: Shape::area
    /// [winding number]: https://mathworld.wolfram.com/ContourWindingNumber.html
    fn winding(&self, pt: Point) -> i32;

    /// Returns `true` if the [`Point`] is inside this shape.
    ///
    /// This is only meaningful for closed shapes.
    fn contains(&self, pt: Point) -> bool {
        self.winding(pt) != 0
    }

    /// The smallest rectangle that encloses the shape.
    fn bounding_box(&self) -> Rect;

    /// If the shape is a line, make it available.
    fn as_line(&self) -> Option<Line> {
        None
    }

    /// If the shape is a rectangle, make it available.
    fn as_rect(&self) -> Option<Rect> {
        None
    }

    /// If the shape is a rounded rectangle, make it available.
    fn as_rounded_rect(&self) -> Option<RoundedRect> {
        None
    }

    /// If the shape is a circle, make it available.
    fn as_circle(&self) -> Option<Circle> {
        None
    }

    /// If the shape is stored as a slice of path elements, make
    /// that available.
    ///
    /// Note: when GAT's land, a method like `path_elements` would be
    /// able to iterate through the slice with no extra allocation,
    /// without making any assumption that storage is contiguous.
    fn as_path_slice(&self) -> Option<&[PathEl]> {
        None
    }
}

/// Blanket implementation so `impl Shape` will accept owned or reference.
impl<'a, T: Shape> Shape for &'a T {
    type PathElementsIter<'iter>

    = T::PathElementsIter<'iter> where T: 'iter, 'a: 'iter;

    fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
        (*self).path_elements(tolerance)
    }

    fn to_path(&self, tolerance: f64) -> BezPath {
        (*self).to_path(tolerance)
    }

    fn path_segments(&self, tolerance: f64) -> Segments<Self::PathElementsIter<'_>> {
        (*self).path_segments(tolerance)
    }

    fn area(&self) -> f64 {
        (*self).area()
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        (*self).perimeter(accuracy)
    }

    fn winding(&self, pt: Point) -> i32 {
        (*self).winding(pt)
    }

    fn bounding_box(&self) -> Rect {
        (*self).bounding_box()
    }

    fn as_line(&self) -> Option<Line> {
        (*self).as_line()
    }

    fn as_rect(&self) -> Option<Rect> {
        (*self).as_rect()
    }

    fn as_rounded_rect(&self) -> Option<RoundedRect> {
        (*self).as_rounded_rect()
    }

    fn as_circle(&self) -> Option<Circle> {
        (*self).as_circle()
    }

    fn as_path_slice(&self) -> Option<&[PathEl]> {
        (*self).as_path_slice()
    }
}
