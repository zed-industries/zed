// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

// Derived from kurbo.
// Copyright 2018 The kurbo Authors.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use core::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A 2D affine transform. Derived from [kurbo](https://github.com/linebender/kurbo).
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Affine([f64; 6]);

impl Affine {
    /// The identity transform.
    pub const IDENTITY: Affine = Affine::scale(1.0);

    /// A transform that is flipped on the y-axis. Useful for converting between
    /// y-up and y-down spaces.
    pub const FLIP_Y: Affine = Affine::new([1.0, 0., 0., -1.0, 0., 0.]);

    /// A transform that is flipped on the x-axis.
    pub const FLIP_X: Affine = Affine::new([-1.0, 0., 0., 1.0, 0., 0.]);

    /// Construct an affine transform from coefficients.
    ///
    /// If the coefficients are `(a, b, c, d, e, f)`, then the resulting
    /// transformation represents this augmented matrix:
    ///
    /// ```text
    /// | a c e |
    /// | b d f |
    /// | 0 0 1 |
    /// ```
    ///
    /// Note that this convention is transposed from PostScript and
    /// Direct2D, but is consistent with the
    /// [Wikipedia](https://en.wikipedia.org/wiki/Affine_transformation)
    /// formulation of affine transformation as augmented matrix. The
    /// idea is that `(A * B) * v == A * (B * v)`, where `*` is the
    /// [`Mul`](core::ops::Mul) trait.
    #[inline]
    pub const fn new(c: [f64; 6]) -> Affine {
        Affine(c)
    }

    /// An affine transform representing uniform scaling.
    #[inline]
    pub const fn scale(s: f64) -> Affine {
        Affine([s, 0.0, 0.0, s, 0.0, 0.0])
    }

    /// An affine transform representing non-uniform scaling
    /// with different scale values for x and y
    #[inline]
    pub const fn scale_non_uniform(s_x: f64, s_y: f64) -> Affine {
        Affine([s_x, 0.0, 0.0, s_y, 0.0, 0.0])
    }

    /// An affine transform representing translation.
    #[inline]
    pub fn translate<V: Into<Vec2>>(p: V) -> Affine {
        let p = p.into();
        Affine([1.0, 0.0, 0.0, 1.0, p.x, p.y])
    }

    /// Creates an affine transformation that takes the unit square to the given rectangle.
    ///
    /// Useful when you want to draw into the unit square but have your output fill any rectangle.
    /// In this case push the `Affine` onto the transform stack.
    pub fn map_unit_square(rect: Rect) -> Affine {
        Affine([rect.width(), 0., 0., rect.height(), rect.x0, rect.y0])
    }

    /// Get the coefficients of the transform.
    #[inline]
    pub fn as_coeffs(self) -> [f64; 6] {
        self.0
    }

    /// Compute the determinant of this transform.
    pub fn determinant(self) -> f64 {
        self.0[0] * self.0[3] - self.0[1] * self.0[2]
    }

    /// Compute the inverse transform.
    ///
    /// Produces NaN values when the determinant is zero.
    pub fn inverse(self) -> Affine {
        let inv_det = self.determinant().recip();
        Affine([
            inv_det * self.0[3],
            -inv_det * self.0[1],
            -inv_det * self.0[2],
            inv_det * self.0[0],
            inv_det * (self.0[2] * self.0[5] - self.0[3] * self.0[4]),
            inv_det * (self.0[1] * self.0[4] - self.0[0] * self.0[5]),
        ])
    }

    /// Compute the bounding box of a transformed rectangle.
    ///
    /// Returns the minimal `Rect` that encloses the given `Rect` after affine transformation.
    /// If the transform is axis-aligned, then this bounding box is "tight", in other words the
    /// returned `Rect` is the transformed rectangle.
    ///
    /// The returned rectangle always has non-negative width and height.
    pub fn transform_rect_bbox(self, rect: Rect) -> Rect {
        let p00 = self * Point::new(rect.x0, rect.y0);
        let p01 = self * Point::new(rect.x0, rect.y1);
        let p10 = self * Point::new(rect.x1, rect.y0);
        let p11 = self * Point::new(rect.x1, rect.y1);
        Rect::from_points(p00, p01).union(Rect::from_points(p10, p11))
    }

    /// Is this map finite?
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0[0].is_finite()
            && self.0[1].is_finite()
            && self.0[2].is_finite()
            && self.0[3].is_finite()
            && self.0[4].is_finite()
            && self.0[5].is_finite()
    }

    /// Is this map NaN?
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0[0].is_nan()
            || self.0[1].is_nan()
            || self.0[2].is_nan()
            || self.0[3].is_nan()
            || self.0[4].is_nan()
            || self.0[5].is_nan()
    }
}

impl Default for Affine {
    #[inline]
    fn default() -> Affine {
        Affine::IDENTITY
    }
}

impl Mul<Point> for Affine {
    type Output = Point;

    #[inline]
    fn mul(self, other: Point) -> Point {
        Point::new(
            self.0[0] * other.x + self.0[2] * other.y + self.0[4],
            self.0[1] * other.x + self.0[3] * other.y + self.0[5],
        )
    }
}

impl Mul for Affine {
    type Output = Affine;

    #[inline]
    fn mul(self, other: Affine) -> Affine {
        Affine([
            self.0[0] * other.0[0] + self.0[2] * other.0[1],
            self.0[1] * other.0[0] + self.0[3] * other.0[1],
            self.0[0] * other.0[2] + self.0[2] * other.0[3],
            self.0[1] * other.0[2] + self.0[3] * other.0[3],
            self.0[0] * other.0[4] + self.0[2] * other.0[5] + self.0[4],
            self.0[1] * other.0[4] + self.0[3] * other.0[5] + self.0[5],
        ])
    }
}

impl MulAssign for Affine {
    #[inline]
    fn mul_assign(&mut self, other: Affine) {
        *self = self.mul(other);
    }
}

impl Mul<Affine> for f64 {
    type Output = Affine;

    #[inline]
    fn mul(self, other: Affine) -> Affine {
        Affine([
            self * other.0[0],
            self * other.0[1],
            self * other.0[2],
            self * other.0[3],
            self * other.0[4],
            self * other.0[5],
        ])
    }
}

/// A 2D point. Derived from [kurbo](https://github.com/linebender/kurbo).
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Point {
    /// The x coordinate.
    pub x: f64,
    /// The y coordinate.
    pub y: f64,
}

impl Point {
    /// The point (0, 0).
    pub const ZERO: Point = Point::new(0., 0.);

    /// The point at the origin; (0, 0).
    pub const ORIGIN: Point = Point::new(0., 0.);

    /// Create a new `Point` with the provided `x` and `y` coordinates.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Convert this point into a `Vec2`.
    #[inline]
    pub const fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl From<(f64, f64)> for Point {
    #[inline]
    fn from(v: (f64, f64)) -> Point {
        Point { x: v.0, y: v.1 }
    }
}

impl From<Point> for (f64, f64) {
    #[inline]
    fn from(v: Point) -> (f64, f64) {
        (v.x, v.y)
    }
}

impl Add<Vec2> for Point {
    type Output = Point;

    #[inline]
    fn add(self, other: Vec2) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign<Vec2> for Point {
    #[inline]
    fn add_assign(&mut self, other: Vec2) {
        *self = Point::new(self.x + other.x, self.y + other.y);
    }
}

impl Sub<Vec2> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, other: Vec2) -> Self {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign<Vec2> for Point {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        *self = Point::new(self.x - other.x, self.y - other.y);
    }
}

impl Add<(f64, f64)> for Point {
    type Output = Point;

    #[inline]
    fn add(self, (x, y): (f64, f64)) -> Self {
        Point::new(self.x + x, self.y + y)
    }
}

impl AddAssign<(f64, f64)> for Point {
    #[inline]
    fn add_assign(&mut self, (x, y): (f64, f64)) {
        *self = Point::new(self.x + x, self.y + y);
    }
}

impl Sub<(f64, f64)> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, (x, y): (f64, f64)) -> Self {
        Point::new(self.x - x, self.y - y)
    }
}

impl SubAssign<(f64, f64)> for Point {
    #[inline]
    fn sub_assign(&mut self, (x, y): (f64, f64)) {
        *self = Point::new(self.x - x, self.y - y);
    }
}

impl Sub<Point> for Point {
    type Output = Vec2;

    #[inline]
    fn sub(self, other: Point) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

/// A rectangle. Derived from [kurbo](https://github.com/linebender/kurbo).
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
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

impl From<(Point, Point)> for Rect {
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

impl Rect {
    /// The empty rectangle at the origin.
    pub const ZERO: Rect = Rect::new(0., 0., 0., 0.);

    /// A new rectangle from minimum and maximum coordinates.
    #[inline]
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

    /// The width of the rectangle.
    ///
    /// Note: nothing forbids negative width.
    #[inline]
    pub fn width(&self) -> f64 {
        self.x1 - self.x0
    }

    /// The height of the rectangle.
    ///
    /// Note: nothing forbids negative height.
    #[inline]
    pub fn height(&self) -> f64 {
        self.y1 - self.y0
    }

    /// Returns the minimum value for the x-coordinate of the rectangle.
    #[inline]
    pub fn min_x(&self) -> f64 {
        self.x0.min(self.x1)
    }

    /// Returns the maximum value for the x-coordinate of the rectangle.
    #[inline]
    pub fn max_x(&self) -> f64 {
        self.x0.max(self.x1)
    }

    /// Returns the minimum value for the y-coordinate of the rectangle.
    #[inline]
    pub fn min_y(&self) -> f64 {
        self.y0.min(self.y1)
    }

    /// Returns the maximum value for the y-coordinate of the rectangle.
    #[inline]
    pub fn max_y(&self) -> f64 {
        self.y0.max(self.y1)
    }

    /// The origin of the rectangle.
    ///
    /// This is the top left corner in a y-down space and with
    /// non-negative width and height.
    #[inline]
    pub fn origin(&self) -> Point {
        Point::new(self.x0, self.y0)
    }

    /// The size of the rectangle.
    #[inline]
    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    /// Take absolute value of width and height.
    ///
    /// The resulting rect has the same extents as the original, but is
    /// guaranteed to have non-negative width and height.
    #[inline]
    pub fn abs(&self) -> Rect {
        let Rect { x0, y0, x1, y1 } = *self;
        Rect::new(x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1))
    }

    /// The area of the rectangle.
    #[inline]
    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }

    /// Whether this rectangle has zero area.
    ///
    /// Note: a rectangle with negative area is not considered empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.area() == 0.0
    }

    /// Returns `true` if `point` lies within `self`.
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x0 && point.x < self.x1 && point.y >= self.y0 && point.y < self.y1
    }

    /// The smallest rectangle enclosing two rectangles.
    ///
    /// Results are valid only if width and height are non-negative.
    #[inline]
    pub fn union(&self, other: Rect) -> Rect {
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
    pub fn union_pt(&self, pt: Point) -> Rect {
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
    #[inline]
    pub fn intersect(&self, other: Rect) -> Rect {
        let x0 = self.x0.max(other.x0);
        let y0 = self.y0.max(other.y0);
        let x1 = self.x1.min(other.x1);
        let y1 = self.y1.min(other.y1);
        Rect::new(x0, y0, x1.max(x0), y1.max(y0))
    }
}

/// A 2D size. Derived from [kurbo](https://github.com/linebender/kurbo).
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Size {
    /// The width.
    pub width: f64,
    /// The height.
    pub height: f64,
}

impl Size {
    /// A size with zero width or height.
    pub const ZERO: Size = Size::new(0., 0.);

    /// Create a new `Size` with the provided `width` and `height`.
    #[inline]
    pub const fn new(width: f64, height: f64) -> Self {
        Size { width, height }
    }

    /// Convert this size into a [`Vec2`], with `width` mapped to `x` and `height`
    /// mapped to `y`.
    #[inline]
    pub const fn to_vec2(self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}W×{:?}H", self.width, self.height)
    }
}

impl MulAssign<f64> for Size {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        *self = Size {
            width: self.width * other,
            height: self.height * other,
        };
    }
}

impl Mul<Size> for f64 {
    type Output = Size;

    #[inline]
    fn mul(self, other: Size) -> Size {
        other * self
    }
}

impl Mul<f64> for Size {
    type Output = Size;

    #[inline]
    fn mul(self, other: f64) -> Size {
        Size {
            width: self.width * other,
            height: self.height * other,
        }
    }
}

impl DivAssign<f64> for Size {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        *self = Size {
            width: self.width / other,
            height: self.height / other,
        };
    }
}

impl Div<f64> for Size {
    type Output = Size;

    #[inline]
    fn div(self, other: f64) -> Size {
        Size {
            width: self.width / other,
            height: self.height / other,
        }
    }
}

impl Add<Size> for Size {
    type Output = Size;
    #[inline]
    fn add(self, other: Size) -> Size {
        Size {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl AddAssign<Size> for Size {
    #[inline]
    fn add_assign(&mut self, other: Size) {
        *self = *self + other;
    }
}

impl Sub<Size> for Size {
    type Output = Size;
    #[inline]
    fn sub(self, other: Size) -> Size {
        Size {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl SubAssign<Size> for Size {
    #[inline]
    fn sub_assign(&mut self, other: Size) {
        *self = *self - other;
    }
}

impl From<(f64, f64)> for Size {
    #[inline]
    fn from(v: (f64, f64)) -> Size {
        Size {
            width: v.0,
            height: v.1,
        }
    }
}

impl From<Size> for (f64, f64) {
    #[inline]
    fn from(v: Size) -> (f64, f64) {
        (v.width, v.height)
    }
}

/// A 2D vector. Derived from [kurbo](https://github.com/linebender/kurbo).
///
/// This is intended primarily for a vector in the mathematical sense,
/// but it can be interpreted as a translation, and converted to and
/// from a point (vector relative to the origin) and size.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Vec2 {
    /// The x-coordinate.
    pub x: f64,
    /// The y-coordinate.
    pub y: f64,
}

impl Vec2 {
    /// The vector (0, 0).
    pub const ZERO: Vec2 = Vec2::new(0., 0.);

    /// Create a new vector.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    /// Convert this vector into a `Point`.
    #[inline]
    pub const fn to_point(self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Convert this vector into a `Size`.
    #[inline]
    pub const fn to_size(self) -> Size {
        Size::new(self.x, self.y)
    }
}

impl From<(f64, f64)> for Vec2 {
    #[inline]
    fn from(v: (f64, f64)) -> Vec2 {
        Vec2 { x: v.0, y: v.1 }
    }
}

impl From<Vec2> for (f64, f64) {
    #[inline]
    fn from(v: Vec2) -> (f64, f64) {
        (v.x, v.y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Vec2) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        *self = Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, other: f64) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<f64> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        *self = Vec2 {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    #[inline]
    fn mul(self, other: Vec2) -> Vec2 {
        other * self
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    /// Note: division by a scalar is implemented by multiplying by the reciprocal.
    ///
    /// This is more efficient but has different roundoff behavior than division.
    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: f64) -> Vec2 {
        self * other.recip()
    }
}

impl DivAssign<f64> for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        self.mul_assign(other.recip());
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
