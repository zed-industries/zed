//! Geometric primitives.

#[allow(unused)]
use crate::F32Ext;

use core::borrow::Borrow;
use core::ops::{Add, Div, Mul, Sub};

/// Represents an angle in degrees or radians.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Angle(f32);

impl Angle {
    /// Angle of zero degrees.
    pub const ZERO: Self = Self(0.);

    /// Creates a new angle from degrees.
    pub fn from_degrees(degrees: f32) -> Self {
        Self(degrees * core::f32::consts::PI / 180.)
    }

    /// Creates a new angle from radians.
    pub fn from_radians(radians: f32) -> Self {
        Self(radians)
    }

    /// Creates a new angle from gradians.
    pub fn from_gradians(gradians: f32) -> Self {
        Self::from_degrees(gradians / 400. * 360.)
    }

    /// Creates a new angle from turns.
    pub fn from_turns(turns: f32) -> Self {
        Self::from_degrees(turns * 360.)
    }

    /// Returns the angle in radians.
    pub fn to_radians(self) -> f32 {
        self.0
    }

    /// Returns the angle in degrees.
    pub fn to_degrees(self) -> f32 {
        self.0 * 180. / core::f32::consts::PI
    }
}

/// Two dimensional vector.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    /// Vector with both components set to zero.
    pub const ZERO: Self = Self { x: 0., y: 0. };

    /// Creates a new vector with the specified coordinates.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns the length of the vector.
    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the squared length of the vector.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the distance between two points.
    #[inline]
    pub fn distance_to(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Computes the dot product of two vectors.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Computes the cross product of two vectors.
    #[inline]
    pub fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    /// Returns a normalized copy of the vector.
    #[inline]
    pub fn normalize(self) -> Self {
        let length = self.length();
        if length == 0. {
            return Self::new(0., 0.);
        }
        let inverse = 1. / length;
        Self::new(self.x * inverse, self.y * inverse)
    }

    /// Returns a new vector containing the smallest integer values greater than
    /// or equal to each component.
    pub fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    /// Returns a new vector containing the largest integer values less than
    /// or equal to each component.
    pub fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }

    /// Returns the angle to the specified vector.
    pub fn angle_to(self, other: Self) -> Angle {
        Angle::from_radians(self.cross(other).atan2(self.dot(other)))
    }

    /// Returns true if this vector is approximately equal to other using a
    /// standard single precision epsilon value.
    #[inline]
    pub fn nearly_eq(self, other: Vector) -> bool {
        self.nearly_eq_by(other, f32::EPSILON)
    }

    /// Returns true if this vector is approximately equal to other using
    /// the specified epsilon value.
    #[inline]
    pub fn nearly_eq_by(self, other: Vector, epsilon: f32) -> bool {
        (self.x - other.x).abs() < epsilon && (self.y - other.y).abs() < epsilon
    }
}

impl Add for Vector {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for Vector {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<f32> for Vector {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div for Vector {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Div<f32> for Vector {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        let s = 1. / rhs;
        Self::new(self.x * s, self.y * s)
    }
}

impl From<[f32; 2]> for Vector {
    fn from(v: [f32; 2]) -> Self {
        Self::new(v[0], v[1])
    }
}

impl From<[i32; 2]> for Vector {
    fn from(v: [i32; 2]) -> Self {
        Self::new(v[0] as f32, v[1] as f32)
    }
}

impl From<(f32, f32)> for Vector {
    fn from(v: (f32, f32)) -> Self {
        Self::new(v.0, v.1)
    }
}

impl From<(i32, i32)> for Vector {
    fn from(v: (i32, i32)) -> Self {
        Self::new(v.0 as f32, v.1 as f32)
    }
}

impl From<(f32, i32)> for Vector {
    fn from(v: (f32, i32)) -> Self {
        Self::new(v.0, v.1 as f32)
    }
}

impl From<(i32, f32)> for Vector {
    fn from(v: (i32, f32)) -> Self {
        Self::new(v.0 as f32, v.1)
    }
}

impl From<f32> for Vector {
    fn from(x: f32) -> Self {
        Self::new(x, x)
    }
}

impl From<i32> for Vector {
    fn from(x: i32) -> Self {
        let x = x as f32;
        Self::new(x, x)
    }
}

impl From<Vector> for [f32; 2] {
    fn from(v: Vector) -> Self {
        [v.x, v.y]
    }
}

impl From<Vector> for (f32, f32) {
    fn from(v: Vector) -> Self {
        (v.x, v.y)
    }
}

/// Alias for vector to distinguish intended use.
pub type Point = Vector;

#[inline(always)]
pub(super) fn normal(start: Vector, end: Vector) -> Vector {
    Vector::new(end.y - start.y, -(end.x - start.x)).normalize()
}

/// Two dimensional transformation matrix.
#[derive(Copy, Clone, Default, Debug)]
pub struct Transform {
    pub xx: f32,
    pub xy: f32,
    pub yx: f32,
    pub yy: f32,
    pub x: f32,
    pub y: f32,
}

impl Transform {
    /// Identity matrix.
    pub const IDENTITY: Self = Self {
        xx: 1.,
        xy: 0.,
        yy: 1.,
        yx: 0.,
        x: 0.,
        y: 0.,
    };

    /// Creates a new transform.
    pub fn new(xx: f32, xy: f32, yx: f32, yy: f32, x: f32, y: f32) -> Self {
        Self {
            xx,
            xy,
            yx,
            yy,
            x,
            y,
        }
    }

    /// Creates a translation transform.
    pub fn translation(x: f32, y: f32) -> Self {
        Self::new(1., 0., 0., 1., x, y)
    }

    /// Creates a rotation transform.
    pub fn rotation(angle: Angle) -> Self {
        let (sin, cos) = angle.0.sin_cos();
        Self {
            xx: cos,
            xy: sin,
            yx: -sin,
            yy: cos,
            x: 0.,
            y: 0.,
        }
    }

    /// Creates a rotation transform around a point.
    pub fn rotation_about(point: impl Into<Point>, angle: Angle) -> Self {
        let p = point.into();
        Self::translation(p.x, p.y)
            .then_rotate(angle)
            .then_translate(-p.x, -p.y)
    }

    /// Creates a scale transform.
    pub fn scale(x: f32, y: f32) -> Self {
        Self::new(x, 0., 0., y, 0., 0.)
    }

    /// Creates a skew transform.
    pub fn skew(x: Angle, y: Angle) -> Self {
        Self {
            xx: 1.,
            xy: y.0.tan(),
            yx: x.0.tan(),
            yy: 1.,
            x: 0.,
            y: 0.,
        }
    }

    fn combine(a: &Transform, b: &Transform) -> Self {
        let xx = a.xx * b.xx + a.yx * b.xy;
        let yx = a.xx * b.yx + a.yx * b.yy;
        let xy = a.xy * b.xx + a.yy * b.xy;
        let yy = a.xy * b.yx + a.yy * b.yy;
        let x = a.x * b.xx + a.y * b.xy + b.x;
        let y = a.x * b.yx + a.y * b.yy + b.y;
        Self {
            xx,
            yx,
            xy,
            yy,
            x,
            y,
        }
    }

    /// Returns a new transform that represents the application of this transform
    /// followed by other.
    pub fn then(&self, other: &Transform) -> Self {
        Self::combine(self, other)
    }

    /// Returns a new transform that represents a translation followed by this
    /// transform.
    pub fn pre_translate(&self, x: f32, y: f32) -> Self {
        Self::combine(&Self::translation(x, y), self)
    }

    /// Returns a new transform that represents this transform followed by a
    /// translation.
    pub fn then_translate(&self, x: f32, y: f32) -> Self {
        let mut t = *self;
        t.x += x;
        t.y += y;
        t
    }

    /// Returns a new transform that represents a rotation followed by this
    /// transform.
    pub fn pre_rotate(&self, angle: Angle) -> Self {
        Self::combine(&Self::rotation(angle), self)
    }

    /// Returns a new transform that represents this transform followed by a
    /// rotation.
    pub fn then_rotate(&self, angle: Angle) -> Self {
        Self::combine(self, &Self::rotation(angle))
    }

    /// Returns a new transform that represents a scale followed by this
    /// transform.
    pub fn pre_scale(&self, x: f32, y: f32) -> Self {
        Self::combine(&Self::scale(x, y), self)
    }

    /// Returns a new transform that represents this transform followed by a
    /// scale.    
    pub fn then_scale(&self, x: f32, y: f32) -> Self {
        Self::combine(self, &Self::scale(x, y))
    }

    /// Returns the determinant of the transform.
    pub fn determinant(&self) -> f32 {
        self.xx * self.yy - self.yx * self.xy
    }

    /// Returns the inverse of the transform, if any.
    pub fn invert(&self) -> Option<Transform> {
        let det = self.determinant();
        if !det.is_finite() || det == 0. {
            return None;
        }
        let s = 1. / det;
        let a = self.xx;
        let b = self.xy;
        let c = self.yx;
        let d = self.yy;
        let x = self.x;
        let y = self.y;
        Some(Transform {
            xx: d * s,
            xy: -b * s,
            yx: -c * s,
            yy: a * s,
            x: (b * y - d * x) * s,
            y: (c * x - a * y) * s,
        })
    }

    /// Returns the result of applying this transform to a point.
    #[inline(always)]
    pub fn transform_point(&self, point: Point) -> Point {
        Vector {
            x: (point.x * self.xx + point.y * self.yx) + self.x,
            y: (point.x * self.xy + point.y * self.yy) + self.y,
        }
    }

    /// Returns the result of applying this transform to a vector.
    #[inline(always)]
    pub fn transform_vector(&self, vector: Vector) -> Vector {
        Vector {
            x: (vector.x * self.xx + vector.y * self.yx),
            y: (vector.x * self.xy + vector.y * self.yy),
        }
    }
}

/// The origin of the coordinate system for rendering.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Origin {
    /// Origin (0, 0) at the top left of the image.
    TopLeft,
    /// Origin (0, 0) at the bottom left of the image.
    BottomLeft,
}

impl Default for Origin {
    fn default() -> Self {
        Self::TopLeft
    }
}

/// Describes the offset and dimensions of a rendered mask.
#[derive(Copy, Clone, Debug, Default)]
pub struct Placement {
    /// Horizontal offset with respect to the origin specified when computing
    /// the placement.
    pub left: i32,
    /// Vertical offset with respect to the origin specified when computing
    /// the placement.
    pub top: i32,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

impl Placement {
    /// Given an origin, offset and bounding box, computes the resulting offset
    /// and placement for a tightly bounded mask.
    pub fn compute(
        origin: Origin,
        offset: impl Into<Vector>,
        bounds: &Bounds,
    ) -> (Vector, Placement) {
        let offset = offset.into();
        let mut bounds = *bounds;
        bounds.min = (bounds.min + offset).floor();
        bounds.max = (bounds.max + offset).ceil();
        let offset = Vector::new(-bounds.min.x, -bounds.min.y);
        let width = bounds.width() as u32;
        let height = bounds.height() as u32;
        let left = -offset.x as i32;
        let top = if origin == Origin::BottomLeft {
            (-offset.y).floor() + height as f32
        } else {
            -offset.y
        } as i32;
        (
            offset,
            Placement {
                left,
                top,
                width,
                height,
            },
        )
    }
}

/// Axis-aligned bounding box.
#[derive(Copy, Clone, Default, Debug)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    /// Creates a new bounding box from minimum and maximum points.
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    /// Creates a new bounding box from a sequence of points.
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Point>,
    {
        let mut b = BoundsBuilder::new();
        for p in points {
            b.add(*p.borrow());
        }
        b.build()
    }

    /// Returns true if the bounding box is empty.
    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y
    }

    /// Returns the width of the bounding box.
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Returns the height of the bounding box.
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Returns true if the box contains the specified point.
    pub fn contains(&self, point: impl Into<Point>) -> bool {
        let p = point.into();
        p.x > self.min.x && p.x < self.max.x && p.y > self.min.y && p.y < self.max.y
    }
}

pub(super) struct BoundsBuilder {
    pub count: usize,
    #[allow(dead_code)]
    pub start: Point,
    pub current: Point,
    pub min: Point,
    pub max: Point,
}

impl BoundsBuilder {
    pub fn new() -> Self {
        Self {
            count: 0,
            start: Point::ZERO,
            current: Point::ZERO,
            min: Point::new(f32::MAX, f32::MAX),
            max: Point::new(f32::MIN, f32::MIN),
        }
    }

    pub fn add(&mut self, p: Point) -> &mut Self {
        let x = p.x;
        let y = p.y;
        if x < self.min.x {
            self.min.x = x;
        }
        if x > self.max.x {
            self.max.x = x;
        }
        if y < self.min.y {
            self.min.y = y;
        }
        if y > self.max.y {
            self.max.y = y;
        }
        self.count += 1;
        self
    }

    pub fn build(&self) -> Bounds {
        if self.count != 0 {
            Bounds {
                min: self.min,
                max: self.max,
            }
        } else {
            Bounds::default()
        }
    }
}
