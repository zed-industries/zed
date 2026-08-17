// Copyright 2019 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A rectangle.

use core::fmt;
use core::ops::{Add, Sub};

use crate::{
    Axis, Ellipse, Insets, PathEl, Point, RoundedRect, RoundedRectRadii, Shape, Size, Vec2,
};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// A rectangle.
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    /// The minimum x coordinate (left edge).
    pub x0: f64,
    /// The minimum y coordinate (top edge in y-down spaces).
    pub y0: f64,
    /// The maximum x coordinate (right edge).
    pub x1: f64,
    /// The maximum y coordinate (bottom edge in y-down spaces).
    pub y1: f64,
}

impl Rect {
    /// The empty rectangle at the origin.
    pub const ZERO: Rect = Rect::new(0., 0., 0., 0.);

    /// A new rectangle from minimum and maximum coordinates.
    #[inline(always)]
    pub const fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Rect {
        Rect { x0, y0, x1, y1 }
    }

    /// A new rectangle from two points.
    ///
    /// The result will have non-negative width and height.
    #[inline]
    pub fn from_points(p0: impl Into<Point>, p1: impl Into<Point>) -> Rect {
        let p0 = p0.into();
        let p1 = p1.into();
        Rect::new(p0.x, p0.y, p1.x, p1.y).abs()
    }

    /// A new rectangle from origin and size.
    ///
    /// The result will have non-negative width and height.
    #[inline]
    pub fn from_origin_size(origin: impl Into<Point>, size: impl Into<Size>) -> Rect {
        let origin = origin.into();
        Rect::from_points(origin, origin + size.into().to_vec2())
    }

    /// A new rectangle from center and size.
    #[inline]
    pub fn from_center_size(center: impl Into<Point>, size: impl Into<Size>) -> Rect {
        let center = center.into();
        let size = 0.5 * size.into();
        Rect::new(
            center.x - size.width,
            center.y - size.height,
            center.x + size.width,
            center.y + size.height,
        )
    }

    /// Create a new `Rect` with the same size as `self` and a new origin.
    #[inline]
    pub fn with_origin(self, origin: impl Into<Point>) -> Rect {
        Rect::from_origin_size(origin, self.size())
    }

    /// Create a new `Rect` with the same origin as `self` and a new size.
    #[inline]
    pub fn with_size(self, size: impl Into<Size>) -> Rect {
        Rect::from_origin_size(self.origin(), size)
    }

    /// Create a new `Rect` by applying the [`Insets`].
    ///
    /// This will not preserve negative width and height.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Rect;
    /// let inset_rect = Rect::new(0., 0., 10., 10.,).inset(2.);
    /// assert_eq!(inset_rect.width(), 14.0);
    /// assert_eq!(inset_rect.x0, -2.0);
    /// assert_eq!(inset_rect.x1, 12.0);
    /// ```
    #[inline]
    pub fn inset(self, insets: impl Into<Insets>) -> Rect {
        self + insets.into()
    }

    /// The width of the rectangle.
    ///
    /// Note: nothing forbids negative width.
    #[inline]
    pub const fn width(&self) -> f64 {
        self.x1 - self.x0
    }

    /// The height of the rectangle.
    ///
    /// Note: nothing forbids negative height.
    #[inline]
    pub const fn height(&self) -> f64 {
        self.y1 - self.y0
    }

    /// Returns the minimum value for the x-coordinate of the rectangle.
    #[inline]
    pub const fn min_x(&self) -> f64 {
        self.x0.min(self.x1)
    }

    /// Returns the maximum value for the x-coordinate of the rectangle.
    #[inline]
    pub const fn max_x(&self) -> f64 {
        self.x0.max(self.x1)
    }

    /// Returns the minimum value for the y-coordinate of the rectangle.
    #[inline]
    pub const fn min_y(&self) -> f64 {
        self.y0.min(self.y1)
    }

    /// Returns the maximum value for the y-coordinate of the rectangle.
    #[inline]
    pub const fn max_y(&self) -> f64 {
        self.y0.max(self.y1)
    }

    /// The origin of the rectangle.
    ///
    /// This is the top left corner in a y-down space and with
    /// non-negative width and height.
    #[inline(always)]
    pub const fn origin(&self) -> Point {
        Point::new(self.x0, self.y0)
    }

    /// The size of the rectangle.
    #[inline]
    pub const fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    /// The area of the rectangle.
    #[inline]
    pub const fn area(&self) -> f64 {
        self.width() * self.height()
    }

    /// Whether this rectangle has zero area.
    #[doc(alias = "is_empty")]
    #[inline]
    pub const fn is_zero_area(&self) -> bool {
        self.area() == 0.0
    }

    /// The center point of the rectangle.
    #[inline]
    pub const fn center(&self) -> Point {
        Point::new(0.5 * (self.x0 + self.x1), 0.5 * (self.y0 + self.y1))
    }

    /// Returns `true` if `point` lies within `self`.
    #[inline]
    pub fn contains(&self, point: impl Into<Point>) -> bool {
        let point = point.into();
        point.x >= self.x0 && point.x < self.x1 && point.y >= self.y0 && point.y < self.y1
    }

    /// Take absolute value of width and height.
    ///
    /// The resulting rect has the same extents as the original, but is
    /// guaranteed to have non-negative width and height.
    #[inline]
    pub const fn abs(&self) -> Rect {
        let Rect { x0, y0, x1, y1 } = *self;
        Rect::new(x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1))
    }

    /// The smallest rectangle enclosing two rectangles.
    ///
    /// Results are valid only if width and height are non-negative.
    #[inline]
    pub const fn union(&self, other: Rect) -> Rect {
        Rect::new(
            self.x0.min(other.x0),
            self.y0.min(other.y0),
            self.x1.max(other.x1),
            self.y1.max(other.y1),
        )
    }

    /// Compute the union with one point.
    ///
    /// This method includes the perimeter of zero-area rectangles.
    /// Thus, a succession of `union_pt` operations on a series of
    /// points yields their enclosing rectangle.
    ///
    /// Results are valid only if width and height are non-negative.
    pub fn union_pt(&self, pt: impl Into<Point>) -> Rect {
        let pt = pt.into();
        Rect::new(
            self.x0.min(pt.x),
            self.y0.min(pt.y),
            self.x1.max(pt.x),
            self.y1.max(pt.y),
        )
    }

    /// The intersection of two rectangles.
    ///
    /// The result is zero-area if either input has negative width or
    /// height. The result always has non-negative width and height.
    ///
    /// If you want to determine whether two rectangles intersect, use the
    /// [`overlaps`] method instead.
    ///
    /// [`overlaps`]: Rect::overlaps
    #[inline]
    pub const fn intersect(&self, other: Rect) -> Rect {
        let x0 = self.x0.max(other.x0);
        let y0 = self.y0.max(other.y0);
        let x1 = self.x1.min(other.x1);
        let y1 = self.y1.min(other.y1);
        Rect::new(x0, y0, x1.max(x0), y1.max(y0))
    }

    /// Determines whether this rectangle overlaps with another in any way.
    ///
    /// Note that the edge of the rectangle is considered to be part of itself, meaning
    /// that two rectangles that share an edge are considered to overlap.
    ///
    /// Returns `true` if the rectangles overlap, `false` otherwise.
    ///
    /// If you want to compute the *intersection* of two rectangles, use the
    /// [`intersect`] method instead.
    ///
    /// [`intersect`]: Rect::intersect
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Rect;
    ///
    /// let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
    /// let rect2 = Rect::new(5.0, 5.0, 15.0, 15.0);
    /// assert!(rect1.overlaps(rect2));
    ///
    /// let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
    /// let rect2 = Rect::new(10.0, 0.0, 20.0, 10.0);
    /// assert!(rect1.overlaps(rect2));
    /// ```
    #[inline]
    pub const fn overlaps(&self, other: Rect) -> bool {
        self.x0 <= other.x1 && self.x1 >= other.x0 && self.y0 <= other.y1 && self.y1 >= other.y0
    }

    /// Returns whether this rectangle contains another rectangle.
    ///
    /// A rectangle is considered to contain another rectangle if the other
    /// rectangle is fully enclosed within the bounds of this rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Rect;
    ///
    /// let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
    /// let rect2 = Rect::new(2.0, 2.0, 4.0, 4.0);
    /// assert!(rect1.contains_rect(rect2));
    /// ```
    ///
    /// Two equal rectangles are considered to contain each other.
    ///
    /// ```
    /// use kurbo::Rect;
    ///
    /// let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    /// assert!(rect.contains_rect(rect));
    /// ```
    #[inline]
    pub const fn contains_rect(&self, other: Rect) -> bool {
        self.x0 <= other.x0 && self.y0 <= other.y0 && self.x1 >= other.x1 && self.y1 >= other.y1
    }

    /// Expand a rectangle by a constant amount in both directions.
    ///
    /// The logic simply applies the amount in each direction. If rectangle
    /// area or added dimensions are negative, this could give odd results.
    pub const fn inflate(&self, width: f64, height: f64) -> Rect {
        Rect::new(
            self.x0 - width,
            self.y0 - height,
            self.x1 + width,
            self.y1 + height,
        )
    }

    /// Returns a new `Rect`,
    /// with each coordinate value [rounded] to the nearest integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Rect;
    /// let rect = Rect::new(3.3, 3.6, 3.0, -3.1).round();
    /// assert_eq!(rect.x0, 3.0);
    /// assert_eq!(rect.y0, 4.0);
    /// assert_eq!(rect.x1, 3.0);
    /// assert_eq!(rect.y1, -3.0);
    /// ```
    ///
    /// [rounded]: f64::round
    #[inline]
    pub fn round(self) -> Rect {
        Rect::new(
            self.x0.round(),
            self.y0.round(),
            self.x1.round(),
            self.y1.round(),
        )
    }

    /// Returns a new `Rect`,
    /// with each coordinate value [rounded up] to the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Rect;
    /// let rect = Rect::new(3.3, 3.6, 3.0, -3.1).ceil();
    /// assert_eq!(rect.x0, 4.0);
    /// assert_eq!(rect.y0, 4.0);
    /// assert_eq!(rect.x1, 3.0);
    /// assert_eq!(rect.y1, -3.0);
    /// ```
    ///
    /// [rounded up]: f64::ceil
    #[inline]
    pub fn ceil(self) -> Rect {
        Rect::new(
            self.x0.ceil(),
            self.y0.ceil(),
            self.x1.ceil(),
            self.y1.ceil(),
        )
    }

    /// Returns a new `Rect`,
    /// with each coordinate value [rounded down] to the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Rect;
    /// let rect = Rect::new(3.3, 3.6, 3.0, -3.1).floor();
    /// assert_eq!(rect.x0, 3.0);
    /// assert_eq!(rect.y0, 3.0);
    /// assert_eq!(rect.x1, 3.0);
    /// assert_eq!(rect.y1, -4.0);
    /// ```
    ///
    /// [rounded down]: f64::floor
    #[inline]
    pub fn floor(self) -> Rect {
        Rect::new(
            self.x0.floor(),
            self.y0.floor(),
            self.x1.floor(),
            self.y1.floor(),
        )
    }

    /// Returns a new `Rect`,
    /// with each coordinate value rounded away from the center of the `Rect`
    /// to the nearest integer, unless they are already an integer.
    /// That is to say this function will return the smallest possible `Rect`
    /// with integer coordinates that is a superset of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Rect;
    ///
    /// // In positive space
    /// let rect = Rect::new(3.3, 3.6, 5.6, 4.1).expand();
    /// assert_eq!(rect.x0, 3.0);
    /// assert_eq!(rect.y0, 3.0);
    /// assert_eq!(rect.x1, 6.0);
    /// assert_eq!(rect.y1, 5.0);
    ///
    /// // In both positive and negative space
    /// let rect = Rect::new(-3.3, -3.6, 5.6, 4.1).expand();
    /// assert_eq!(rect.x0, -4.0);
    /// assert_eq!(rect.y0, -4.0);
    /// assert_eq!(rect.x1, 6.0);
    /// assert_eq!(rect.y1, 5.0);
    ///
    /// // In negative space
    /// let rect = Rect::new(-5.6, -4.1, -3.3, -3.6).expand();
    /// assert_eq!(rect.x0, -6.0);
    /// assert_eq!(rect.y0, -5.0);
    /// assert_eq!(rect.x1, -3.0);
    /// assert_eq!(rect.y1, -3.0);
    ///
    /// // Inverse orientation
    /// let rect = Rect::new(5.6, -3.6, 3.3, -4.1).expand();
    /// assert_eq!(rect.x0, 6.0);
    /// assert_eq!(rect.y0, -3.0);
    /// assert_eq!(rect.x1, 3.0);
    /// assert_eq!(rect.y1, -5.0);
    /// ```
    #[inline]
    pub fn expand(self) -> Rect {
        // The compiler optimizer will remove the if branching.
        let (x0, x1) = if self.x0 < self.x1 {
            (self.x0.floor(), self.x1.ceil())
        } else {
            (self.x0.ceil(), self.x1.floor())
        };
        let (y0, y1) = if self.y0 < self.y1 {
            (self.y0.floor(), self.y1.ceil())
        } else {
            (self.y0.ceil(), self.y1.floor())
        };
        Rect::new(x0, y0, x1, y1)
    }

    /// Returns a new `Rect`,
    /// with each coordinate value rounded towards the center of the `Rect`
    /// to the nearest integer, unless they are already an integer.
    /// That is to say this function will return the biggest possible `Rect`
    /// with integer coordinates that is a subset of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Rect;
    ///
    /// // In positive space
    /// let rect = Rect::new(3.3, 3.6, 5.6, 4.1).trunc();
    /// assert_eq!(rect.x0, 4.0);
    /// assert_eq!(rect.y0, 4.0);
    /// assert_eq!(rect.x1, 5.0);
    /// assert_eq!(rect.y1, 4.0);
    ///
    /// // In both positive and negative space
    /// let rect = Rect::new(-3.3, -3.6, 5.6, 4.1).trunc();
    /// assert_eq!(rect.x0, -3.0);
    /// assert_eq!(rect.y0, -3.0);
    /// assert_eq!(rect.x1, 5.0);
    /// assert_eq!(rect.y1, 4.0);
    ///
    /// // In negative space
    /// let rect = Rect::new(-5.6, -4.1, -3.3, -3.6).trunc();
    /// assert_eq!(rect.x0, -5.0);
    /// assert_eq!(rect.y0, -4.0);
    /// assert_eq!(rect.x1, -4.0);
    /// assert_eq!(rect.y1, -4.0);
    ///
    /// // Inverse orientation
    /// let rect = Rect::new(5.6, -3.6, 3.3, -4.1).trunc();
    /// assert_eq!(rect.x0, 5.0);
    /// assert_eq!(rect.y0, -4.0);
    /// assert_eq!(rect.x1, 4.0);
    /// assert_eq!(rect.y1, -4.0);
    /// ```
    #[inline]
    pub fn trunc(self) -> Rect {
        // The compiler optimizer will remove the if branching.
        let (x0, x1) = if self.x0 < self.x1 {
            (self.x0.ceil(), self.x1.floor())
        } else {
            (self.x0.floor(), self.x1.ceil())
        };
        let (y0, y1) = if self.y0 < self.y1 {
            (self.y0.ceil(), self.y1.floor())
        } else {
            (self.y0.floor(), self.y1.ceil())
        };
        Rect::new(x0, y0, x1, y1)
    }

    /// Scales the `Rect` by `factor` with respect to the origin (the point `(0, 0)`).
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Rect;
    ///
    /// let rect = Rect::new(2., 2., 4., 6.).scale_from_origin(2.);
    /// assert_eq!(rect.x0, 4.);
    /// assert_eq!(rect.x1, 8.);
    /// ```
    #[inline]
    pub const fn scale_from_origin(self, factor: f64) -> Rect {
        Rect {
            x0: self.x0 * factor,
            y0: self.y0 * factor,
            x1: self.x1 * factor,
            y1: self.y1 * factor,
        }
    }

    /// Creates a new [`RoundedRect`] from this `Rect` and the provided
    /// corner [radius](RoundedRectRadii).
    #[inline]
    pub fn to_rounded_rect(self, radii: impl Into<RoundedRectRadii>) -> RoundedRect {
        RoundedRect::from_rect(self, radii)
    }

    /// Returns the [`Ellipse`] that is bounded by this `Rect`.
    #[inline]
    pub fn to_ellipse(self) -> Ellipse {
        Ellipse::from_rect(self)
    }

    /// The aspect ratio of this `Rect`.
    ///
    /// This is defined as the width divided by the height. It measures the
    /// "squareness" of the rectangle (a value of `1` is square).
    ///
    /// If the height is `0`, the output will be `sign(self.width) * infinity`.
    /// If the width and height are both `0`, then the output will be `NaN`.
    #[inline]
    pub const fn aspect_ratio_width(self) -> f64 {
        self.size().aspect_ratio_width()
    }

    /// **The inverse** of the aspect ratio of this `Rect`.
    ///
    /// Aspect ratios are usually defined as the ratio of the width to the height, but
    /// this method incorrectly returns the ratio of height to width.
    /// You should generally prefer [`aspect_ratio_width`](Self::aspect_ratio_width).
    ///
    /// If the width is `0` the output will be `sign(y1 - y0) * infinity`.
    ///
    /// If the width and height are both `0`, the result will be `NaN`.
    #[deprecated(
        note = "You should use `aspect_ratio_width` instead, as this method returns a potentially unexpected value.",
        since = "0.12.0"
    )]
    #[inline]
    pub fn aspect_ratio(&self) -> f64 {
        #[expect(deprecated, reason = "This method is deprecated for the same reason.")]
        self.size().aspect_ratio()
    }

    /// Returns the largest possible `Rect` with the given `aspect_ratio`
    /// that is fully contained in `self`.
    ///
    /// The aspect ratio is specified fractionally, as `width / height`.
    ///
    /// The resulting rectangle will be centered if it is smaller than this rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// # use kurbo::Rect;
    /// let outer = Rect::new(0.0, 0.0, 10.0, 20.0);
    /// let inner = outer.inscribed_rect_with_aspect_ratio(1.0);
    /// // The new `Rect` is a square centered at the center of `outer`.
    /// assert_eq!(inner, Rect::new(0.0, 5.0, 10.0, 15.0));
    /// ```
    pub const fn inscribed_rect_with_aspect_ratio(&self, aspect_ratio: f64) -> Rect {
        let self_size @ Size { width, height } = self.size();
        let self_aspect = self_size.aspect_ratio_width();

        // TODO the parameter `1e-9` was chosen quickly and may not be optimal.
        // if self_aspect is `NaN`, that means we're the 0x0 rectangle (or have a `NaN`).
        // We don't want NaNs in the output for the 0x0 rectangle
        if self_aspect.is_nan() || (self_aspect - aspect_ratio).abs() < 1e-9 {
            // short circuit
            *self
        } else if self_aspect.abs() < aspect_ratio.abs() {
            // Our width/height is less than the requested width/height
            // We use a smaller height to increase the ratio.
            // shrink y to fit
            let new_height = width * aspect_ratio.recip();
            let gap = (height - new_height) * 0.5;
            let y0 = self.y0 + gap;
            let y1 = self.y1 - gap;
            Rect::new(self.x0, y0, self.x1, y1)
        } else {
            // shrink x to fit
            let new_width = height * aspect_ratio;
            let gap = (width - new_width) * 0.5;
            let x0 = self.x0 + gap;
            let x1 = self.x1 - gap;
            Rect::new(x0, self.y0, x1, self.y1)
        }
    }

    /// Returns the largest possible `Rect` with the given <code>**inverse**_aspect_ratio</code>
    /// that is fully contained in `self`.
    ///
    /// Aspect ratios are usually defined as the ratio of the width to the height, but
    /// this method accepts an aspect ratio specified fractionally as `height / width`.
    /// You should generally prefer
    /// [`inscribed_rect_with_aspect_ratio`](Self::inscribed_rect_with_aspect_ratio), which
    /// takes a "normal" aspect ratio.
    ///
    /// The resulting rectangle will be centered if it is smaller than this rectangle.
    #[deprecated(
        note = "You should use `inscribed_rect_with_aspect_ratio` instead, as this method expects an unusually defined parameter.",
        since = "0.12.0"
    )]
    pub fn contained_rect_with_aspect_ratio(&self, inverse_aspect_ratio: f64) -> Rect {
        self.inscribed_rect_with_aspect_ratio(1. / inverse_aspect_ratio)
    }

    /// Is this rectangle [finite]?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    pub const fn is_finite(&self) -> bool {
        self.x0.is_finite() && self.x1.is_finite() && self.y0.is_finite() && self.y1.is_finite()
    }

    /// Is this rectangle [NaN]?
    ///
    /// [NaN]: f64::is_nan
    #[inline]
    pub const fn is_nan(&self) -> bool {
        self.x0.is_nan() || self.y0.is_nan() || self.x1.is_nan() || self.y1.is_nan()
    }

    /// Get the members matching the given axis.
    #[inline]
    pub const fn get_coords(self, axis: Axis) -> (f64, f64) {
        match axis {
            Axis::Horizontal => (self.x0, self.x1),
            Axis::Vertical => (self.y0, self.y1),
        }
    }

    /// Get a mutable reference to the members matching the given axis.
    #[inline]
    pub const fn get_coords_mut(&mut self, axis: Axis) -> (&mut f64, &mut f64) {
        match axis {
            Axis::Horizontal => (&mut self.x0, &mut self.x1),
            Axis::Vertical => (&mut self.y0, &mut self.y1),
        }
    }

    /// Set the members matching the given axis to the given values.
    #[inline]
    pub const fn set_coords(&mut self, axis: Axis, v0: f64, v1: f64) {
        match axis {
            Axis::Horizontal => (self.x0, self.x1) = (v0, v1),
            Axis::Vertical => (self.y0, self.y1) = (v0, v1),
        }
    }
}

impl From<(Point, Point)> for Rect {
    #[inline(always)]
    fn from(points: (Point, Point)) -> Rect {
        Rect::from_points(points.0, points.1)
    }
}

impl From<(Point, Size)> for Rect {
    fn from(params: (Point, Size)) -> Rect {
        Rect::from_origin_size(params.0, params.1)
    }
}

impl Add<Vec2> for Rect {
    type Output = Rect;

    #[inline]
    fn add(self, v: Vec2) -> Rect {
        Rect::new(self.x0 + v.x, self.y0 + v.y, self.x1 + v.x, self.y1 + v.y)
    }
}

impl Sub<Vec2> for Rect {
    type Output = Rect;

    #[inline]
    fn sub(self, v: Vec2) -> Rect {
        Rect::new(self.x0 - v.x, self.y0 - v.y, self.x1 - v.x, self.y1 - v.y)
    }
}

impl Sub for Rect {
    type Output = Insets;

    #[inline]
    fn sub(self, other: Rect) -> Insets {
        let x0 = other.x0 - self.x0;
        let y0 = other.y0 - self.y0;
        let x1 = self.x1 - other.x1;
        let y1 = self.y1 - other.y1;
        Insets { x0, y0, x1, y1 }
    }
}

#[doc(hidden)]
pub struct RectPathIter {
    rect: Rect,
    ix: usize,
}

impl Shape for Rect {
    type PathElementsIter<'iter> = RectPathIter;

    fn path_elements(&self, _tolerance: f64) -> RectPathIter {
        RectPathIter { rect: *self, ix: 0 }
    }

    // It's a bit of duplication having both this and the impl method, but
    // removing that would require using the trait. We'll leave it for now.
    #[inline]
    fn area(&self) -> f64 {
        Rect::area(self)
    }

    #[inline]
    fn perimeter(&self, _accuracy: f64) -> f64 {
        2.0 * (self.width().abs() + self.height().abs())
    }

    /// Note: this function is carefully designed so that if the plane is
    /// tiled with rectangles, the winding number will be nonzero for exactly
    /// one of them.
    #[inline]
    fn winding(&self, pt: Point) -> i32 {
        let xmin = self.x0.min(self.x1);
        let xmax = self.x0.max(self.x1);
        let ymin = self.y0.min(self.y1);
        let ymax = self.y0.max(self.y1);
        if pt.x >= xmin && pt.x < xmax && pt.y >= ymin && pt.y < ymax {
            if (self.x1 > self.x0) ^ (self.y1 > self.y0) {
                -1
            } else {
                1
            }
        } else {
            0
        }
    }

    #[inline]
    fn bounding_box(&self) -> Rect {
        self.abs()
    }

    #[inline(always)]
    fn as_rect(&self) -> Option<Rect> {
        Some(*self)
    }

    #[inline]
    fn contains(&self, pt: Point) -> bool {
        self.contains(pt)
    }
}

// This is clockwise in a y-down coordinate system for positive area.
impl Iterator for RectPathIter {
    type Item = PathEl;

    fn next(&mut self) -> Option<PathEl> {
        self.ix += 1;
        match self.ix {
            1 => Some(PathEl::MoveTo(Point::new(self.rect.x0, self.rect.y0))),
            2 => Some(PathEl::LineTo(Point::new(self.rect.x1, self.rect.y0))),
            3 => Some(PathEl::LineTo(Point::new(self.rect.x1, self.rect.y1))),
            4 => Some(PathEl::LineTo(Point::new(self.rect.x0, self.rect.y1))),
            5 => Some(PathEl::ClosePath),
            _ => None,
        }
    }
}

impl fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "Rect {{ origin: {:?}, size: {:?} }}",
                self.origin(),
                self.size()
            )
        } else {
            write!(
                f,
                "Rect {{ x0: {:?}, y0: {:?}, x1: {:?}, y1: {:?} }}",
                self.x0, self.y0, self.x1, self.y1
            )
        }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect {{ ")?;
        fmt::Display::fmt(&self.origin(), f)?;
        write!(f, " ")?;
        fmt::Display::fmt(&self.size(), f)?;
        write!(f, " }}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, Rect, Shape};

    fn assert_approx_eq(x: f64, y: f64) {
        assert!((x - y).abs() < 1e-7);
    }

    #[test]
    fn area_sign() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        let center = r.center();
        assert_approx_eq(r.area(), 100.0);

        assert_eq!(r.winding(center), 1);

        let p = r.to_path(1e-9);
        assert_approx_eq(r.area(), p.area());
        assert_eq!(r.winding(center), p.winding(center));

        let r_flip = Rect::new(0.0, 10.0, 10.0, 0.0);
        assert_approx_eq(r_flip.area(), -100.0);

        assert_eq!(r_flip.winding(Point::new(5.0, 5.0)), -1);
        let p_flip = r_flip.to_path(1e-9);
        assert_approx_eq(r_flip.area(), p_flip.area());
        assert_eq!(r_flip.winding(center), p_flip.winding(center));
    }

    #[test]
    fn display() {
        let r = Rect::from_origin_size((10., 12.23214), (22.222222222, 23.1));
        assert_eq!(
            format!("{r}"),
            "Rect { (10, 12.23214) (22.222222222×23.1) }"
        );
        assert_eq!(format!("{r:.2}"), "Rect { (10.00, 12.23) (22.22×23.10) }");
    }

    /* TODO uncomment when a (possibly approximate) equality test has been decided on
    #[test]
    fn rect_from_center_size() {
        assert_eq!(
            Rect::from_center_size(Point::new(3.0, 2.0), Size::new(2.0, 4.0)),
            Rect::new(2.0, 0.0, 4.0, 4.0)
        );
    }
    */

    #[test]
    #[expect(
        deprecated,
        reason = "Testing behaviour of deprecated function is still as expected."
    )]
    fn contained_rect_with_aspect_ratio() {
        #[track_caller]
        fn case(outer: [f64; 4], aspect_ratio: f64, expected: [f64; 4]) {
            let outer = Rect::new(outer[0], outer[1], outer[2], outer[3]);
            let expected = Rect::new(expected[0], expected[1], expected[2], expected[3]);
            assert_eq!(
                outer.contained_rect_with_aspect_ratio(aspect_ratio),
                expected
            );
            assert!(
                expected.size().width.abs() <= outer.size().width.abs()
                    && expected.size().width.abs() <= outer.size().height.abs(),
                "Sanity check {expected} should be smaller than {outer}."
            );
        }
        // squares (different point orderings)
        case([0.0, 0.0, 10.0, 20.0], 1.0, [0.0, 5.0, 10.0, 15.0]);
        case([0.0, 20.0, 10.0, 0.0], 1.0, [0.0, 5.0, 10.0, 15.0]);
        case([10.0, 0.0, 0.0, 20.0], 1.0, [10.0, 15.0, 0.0, 5.0]);
        case([10.0, 20.0, 0.0, 0.0], 1.0, [10.0, 15.0, 0.0, 5.0]);
        // non-square
        case([0.0, 0.0, 10.0, 20.0], 0.5, [0.0, 7.5, 10.0, 12.5]);
        // same aspect ratio
        case([0.0, 0.0, 10.0, 20.0], 2.0, [0.0, 0.0, 10.0, 20.0]);
        // negative aspect ratio
        case([0.0, 0.0, 10.0, 20.0], -1.0, [0.0, 15.0, 10.0, 5.0]);
        // infinite aspect ratio
        case([0.0, 0.0, 10.0, 20.0], f64::INFINITY, [5.0, 0.0, 5.0, 20.0]);
        // zero aspect ratio
        case([0.0, 0.0, 10.0, 20.0], 0.0, [0.0, 10.0, 10.0, 10.0]);
        // zero width rect
        case([0.0, 0.0, 0.0, 20.0], 1.0, [0.0, 10.0, 0.0, 10.0]);
        // many zeros
        case([0.0, 0.0, 0.0, 20.0], 0.0, [0.0, 10.0, 0.0, 10.0]);
        // everything zero
        case([0.0, 0.0, 0.0, 0.0], 0.0, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn inscribed_rect_with_aspect_ratio() {
        #[track_caller]
        fn case(outer: [f64; 4], aspect_ratio: f64, expected: [f64; 4]) {
            let outer = Rect::new(outer[0], outer[1], outer[2], outer[3]);
            let expected = Rect::new(expected[0], expected[1], expected[2], expected[3]);
            assert_eq!(
                outer.inscribed_rect_with_aspect_ratio(aspect_ratio),
                expected
            );
            assert!(
                expected.size().width.abs() <= outer.size().width.abs()
                    && expected.size().width.abs() <= outer.size().height.abs(),
                "Sanity check {expected} should be smaller than {outer}."
            );
        }
        // squares (different point orderings)
        case([0.0, 0.0, 10.0, 20.0], 1.0, [0.0, 5.0, 10.0, 15.0]);
        case([0.0, 20.0, 10.0, 0.0], 1.0, [0.0, 5.0, 10.0, 15.0]);
        case([10.0, 0.0, 0.0, 20.0], 1.0, [10.0, 15.0, 0.0, 5.0]);
        case([10.0, 20.0, 0.0, 0.0], 1.0, [10.0, 15.0, 0.0, 5.0]);
        // same aspect ratio
        case([0.0, 0.0, 10.0, 20.0], 0.5, [0.0, 0.0, 10.0, 20.0]);
        // non-square
        case([0.0, 0.0, 10.0, 20.0], 2.0, [0.0, 7.5, 10.0, 12.5]);
        // negative aspect ratio
        case([0.0, 0.0, 10.0, 20.0], -1.0, [0.0, 15.0, 10.0, 5.0]);
        // infinite aspect ratio, horizontal line through the center
        case(
            [0.0, 0.0, 10.0, 20.0],
            f64::INFINITY,
            [0.0, 10.0, 10.0, 10.0],
        );
        // zero aspect ratio, vertical line through the center
        case([0.0, 0.0, 10.0, 20.0], 0.0, [5.0, 0.0, 5.0, 20.0]);
        // zero width rect
        case([0.0, 0.0, 0.0, 20.0], 1.0, [0.0, 10.0, 0.0, 10.0]);
        // many zeros, vertical line
        case([0.0, 0.0, 0.0, 20.0], 0.0, [0.0, 0.0, 0.0, 20.0]);
        // many zeros
        case([0.0, 0.0, 20.0, 0.0], 0.0, [10.0, 0.0, 10.0, 0.0]);
        // everything zero
        case([0.0, 0.0, 0.0, 0.0], 0.0, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    #[expect(deprecated, reason = "Testing deprecated function.")]
    fn aspect_ratio() {
        let test = Rect::new(0.0, 0.0, 1.0, 1.0);
        assert!((test.aspect_ratio() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn aspect_ratio_width() {
        let test = Rect::new(0.0, 0.0, 1.0, 1.0);
        assert!((test.aspect_ratio_width() - 1.0).abs() < 1e-6);
        // 16:10
        let test = Rect::new(0.0, 0.0, 16.0, 10.0);
        assert!((test.aspect_ratio_width() - 1.6).abs() < 1e-6);
        // 16:9 screen resolutions
        let test = Rect::new(0.0, 0.0, 1920.0, 1080.0);
        assert!((test.aspect_ratio_width() - (16. / 9.)).abs() < 1e-6);
    }

    #[test]
    fn contained_rect_overlaps() {
        let outer = Rect::new(0.0, 0.0, 10.0, 10.0);
        let inner = Rect::new(2.0, 2.0, 4.0, 4.0);
        assert!(outer.overlaps(inner));
    }

    #[test]
    fn overlapping_rect_overlaps() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(5.0, 5.0, 15.0, 15.0);
        assert!(a.overlaps(b));
    }

    #[test]
    fn disjoint_rect_overlaps() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(11.0, 11.0, 15.0, 15.0);
        assert!(!a.overlaps(b));
    }

    #[test]
    fn sharing_edge_overlaps() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(10.0, 0.0, 20.0, 10.0);
        assert!(a.overlaps(b));
    }

    // Test the two other directions in case there is a bug that only appears in one direction.
    #[test]
    fn disjoint_rect_overlaps_negative() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(-10.0, -10.0, -5.0, -5.0);
        assert!(!a.overlaps(b));
    }

    #[test]
    fn contained_rectangle_contains() {
        let outer = Rect::new(0.0, 0.0, 10.0, 10.0);
        let inner = Rect::new(2.0, 2.0, 4.0, 4.0);
        assert!(outer.contains_rect(inner));
    }

    #[test]
    fn overlapping_rectangle_contains() {
        let outer = Rect::new(0.0, 0.0, 10.0, 10.0);
        let inner = Rect::new(5.0, 5.0, 15.0, 15.0);
        assert!(!outer.contains_rect(inner));
    }

    #[test]
    fn disjoint_rectangle_contains() {
        let outer = Rect::new(0.0, 0.0, 10.0, 10.0);
        let inner = Rect::new(11.0, 11.0, 15.0, 15.0);
        assert!(!outer.contains_rect(inner));
    }

    #[test]
    fn rect_intersect_zero() {
        // These rectangles don't overlap vertically.
        let a = Rect::new(25., 101., 200., 130.);
        let b = Rect::new(0., 0., 100., 100.);

        for intersection in [a.intersect(b), b.intersect(a)] {
            assert_eq!(intersection.x0, 25.);
            assert_eq!(intersection.x1, 100.);
            assert_eq!(intersection.area(), 0.);
            assert_eq!(intersection.y0, intersection.y1);

            // We don't guarantee this in the documentation, but the intersection's `y` values should
            // be a sensible value taken from the input.
            assert!([0., 100., 101., 130.].contains(&intersection.y0));
        }
    }
}
