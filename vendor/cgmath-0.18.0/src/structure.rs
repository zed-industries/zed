// Copyright 2016 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Generic algebraic structures

use num_traits::{cast, Float};
use std::cmp;
use std::iter;
use std::ops::*;

use approx;

use angle::Rad;
use num::{BaseFloat, BaseNum};

pub use num_traits::{Bounded, One, Zero};

/// An array containing elements of type `Element`
pub trait Array
where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: Index<usize, Output = <Self as Array>::Element>,
    Self: IndexMut<usize, Output = <Self as Array>::Element>,
{
    type Element: Copy;

    /// Get the number of elements in the array type
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Vector3;
    ///
    /// assert_eq!(Vector3::<f32>::len(), 3);
    /// ```
    fn len() -> usize;

    /// Construct a vector from a single value, replicating it.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Vector3;
    ///
    /// assert_eq!(Vector3::from_value(1),
    ///            Vector3::new(1, 1, 1));
    /// ```
    fn from_value(value: Self::Element) -> Self;

    /// Get the pointer to the first element of the array.
    #[inline]
    fn as_ptr(&self) -> *const Self::Element {
        &self[0]
    }

    /// Get a mutable pointer to the first element of the array.
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Element {
        &mut self[0]
    }

    /// Swap the elements at indices `i` and `j` in-place.
    #[inline]
    fn swap_elements(&mut self, i: usize, j: usize) {
        use std::ptr;

        // Yeah, ok borrow checker – I know what I'm doing here
        unsafe { ptr::swap(&mut self[i], &mut self[j]) };
    }

    /// The sum of the elements of the array.
    fn sum(self) -> Self::Element
    where
        Self::Element: Add<Output = <Self as Array>::Element>;

    /// The product of the elements of the array.
    fn product(self) -> Self::Element
    where
        Self::Element: Mul<Output = <Self as Array>::Element>;

    /// Whether all elements of the array are finite
    fn is_finite(&self) -> bool
    where
        Self::Element: Float;
}

/// Element-wise arithmetic operations. These are supplied for pragmatic
/// reasons, but will usually fall outside of traditional algebraic properties.
pub trait ElementWise<Rhs = Self> {
    fn add_element_wise(self, rhs: Rhs) -> Self;
    fn sub_element_wise(self, rhs: Rhs) -> Self;
    fn mul_element_wise(self, rhs: Rhs) -> Self;
    fn div_element_wise(self, rhs: Rhs) -> Self;
    fn rem_element_wise(self, rhs: Rhs) -> Self;

    fn add_assign_element_wise(&mut self, rhs: Rhs);
    fn sub_assign_element_wise(&mut self, rhs: Rhs);
    fn mul_assign_element_wise(&mut self, rhs: Rhs);
    fn div_assign_element_wise(&mut self, rhs: Rhs);
    fn rem_assign_element_wise(&mut self, rhs: Rhs);
}

/// Vectors that can be [added](http://mathworld.wolfram.com/VectorAddition.html)
/// together and [multiplied](https://en.wikipedia.org/wiki/Scalar_multiplication)
/// by scalars.
///
/// Examples include vectors, matrices, and quaternions.
///
/// # Required operators
///
/// ## Vector addition
///
/// Vectors can be added, subtracted, or negated via the following traits:
///
/// - `Add<Output = Self>`
/// - `Sub<Output = Self>`
/// - `Neg<Output = Self>`
///
/// ```rust
/// use cgmath::Vector3;
///
/// let velocity0 = Vector3::new(1, 2, 0);
/// let velocity1 = Vector3::new(1, 1, 0);
///
/// let total_velocity = velocity0 + velocity1;
/// let velocity_diff = velocity1 - velocity0;
/// let reversed_velocity0 = -velocity0;
/// ```
///
/// Vector spaces are also required to implement the additive identity trait,
/// `Zero`. Adding this to another vector should have no effect:
///
/// ```rust
/// use cgmath::prelude::*;
/// use cgmath::Vector2;
///
/// let v = Vector2::new(1, 2);
/// assert_eq!(v + Vector2::zero(), v);
/// ```
///
/// ## Scalar multiplication
///
/// Vectors can be multiplied or divided by their associated scalars via the
/// following traits:
///
/// - `Mul<Self::Scalar, Output = Self>`
/// - `Div<Self::Scalar, Output = Self>`
/// - `Rem<Self::Scalar, Output = Self>`
///
/// ```rust
/// use cgmath::Vector2;
///
/// let translation = Vector2::new(3.0, 4.0);
/// let scale_factor = 2.0;
///
/// let upscaled_translation = translation * scale_factor;
/// let downscaled_translation = translation / scale_factor;
/// ```
pub trait VectorSpace: Copy + Clone
where
    Self: Zero,

    Self: Add<Self, Output = Self>,
    Self: Sub<Self, Output = Self>,
    Self: iter::Sum<Self>,

    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: Mul<<Self as VectorSpace>::Scalar, Output = Self>,
    Self: Div<<Self as VectorSpace>::Scalar, Output = Self>,
    Self: Rem<<Self as VectorSpace>::Scalar, Output = Self>,
{
    /// The associated scalar.
    type Scalar: BaseNum;

    /// Returns the result of linearly interpolating the vector
    /// towards `other` by the specified amount.
    #[inline]
    fn lerp(self, other: Self, amount: Self::Scalar) -> Self {
        self + ((other - self) * amount)
    }
}

/// A type with a distance function between values.
///
/// Examples are vectors, points, and quaternions.
pub trait MetricSpace: Sized {
    /// The metric to be returned by the `distance` function.
    type Metric;

    /// Returns the squared distance.
    ///
    /// This does not perform an expensive square root operation like in
    /// `MetricSpace::distance` method, and so can be used to compare distances
    /// more efficiently.
    fn distance2(self, other: Self) -> Self::Metric;

    /// The distance between two values.
    fn distance(self, other: Self) -> Self::Metric
    where
        Self::Metric: Float,
    {
        Float::sqrt(Self::distance2(self, other))
    }
}

/// Vectors that also have a [dot](https://en.wikipedia.org/wiki/Dot_product)
/// (or [inner](https://en.wikipedia.org/wiki/Inner_product_space)) product.
///
/// The dot product allows for the definition of other useful operations, like
/// finding the magnitude of a vector or normalizing it.
///
/// Examples include vectors and quaternions.
pub trait InnerSpace: VectorSpace
where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: MetricSpace<Metric = <Self as VectorSpace>::Scalar>,
{
    /// Vector dot (or inner) product.
    fn dot(self, other: Self) -> Self::Scalar;

    /// Returns `true` if the vector is perpendicular (at right angles) to the
    /// other vector.
    fn is_perpendicular(self, other: Self) -> bool
    where
        Self::Scalar: approx::UlpsEq,
    {
        ulps_eq!(Self::dot(self, other), &Self::Scalar::zero())
    }

    /// Returns the squared magnitude.
    ///
    /// This does not perform an expensive square root operation like in
    /// `InnerSpace::magnitude` method, and so can be used to compare magnitudes
    /// more efficiently.
    #[inline]
    fn magnitude2(self) -> Self::Scalar {
        Self::dot(self, self)
    }

    /// Returns the angle between two vectors in radians.
    fn angle(self, other: Self) -> Rad<Self::Scalar>
    where
        Self::Scalar: BaseFloat,
    {
        Rad::acos(Self::dot(self, other) / (self.magnitude() * other.magnitude()))
    }

    /// Returns the
    /// [vector projection](https://en.wikipedia.org/wiki/Vector_projection)
    /// of the current inner space projected onto the supplied argument.
    #[inline]
    fn project_on(self, other: Self) -> Self {
        other * (self.dot(other) / other.magnitude2())
    }

    /// The distance from the tail to the tip of the vector.
    #[inline]
    fn magnitude(self) -> Self::Scalar
    where
        Self::Scalar: Float,
    {
        Float::sqrt(self.magnitude2())
    }

    /// Returns a vector with the same direction, but with a magnitude of `1`.
    #[inline]
    fn normalize(self) -> Self
    where
        Self::Scalar: Float,
    {
        self.normalize_to(Self::Scalar::one())
    }

    /// Returns a vector with the same direction and a given magnitude.
    #[inline]
    fn normalize_to(self, magnitude: Self::Scalar) -> Self
    where
        Self::Scalar: Float,
    {
        self * (magnitude / self.magnitude())
    }
}

/// Points in a [Euclidean space](https://en.wikipedia.org/wiki/Euclidean_space)
/// with an associated space of displacement vectors.
///
/// # Point-Vector distinction
///
/// `cgmath` distinguishes between points and vectors in the following way:
///
/// - Points are _locations_ relative to an origin
/// - Vectors are _displacements_ between those points
///
/// For example, to find the midpoint between two points, you can write the
/// following:
///
/// ```rust
/// use cgmath::Point3;
///
/// let p0 = Point3::new(1.0, 2.0, 3.0);
/// let p1 = Point3::new(-3.0, 1.0, 2.0);
/// let midpoint: Point3<f32> = p0 + (p1 - p0) * 0.5;
/// ```
///
/// Breaking the expression up, and adding explicit types makes it clearer
/// to see what is going on:
///
/// ```rust
/// # use cgmath::{Point3, Vector3};
/// #
/// # let p0 = Point3::new(1.0, 2.0, 3.0);
/// # let p1 = Point3::new(-3.0, 1.0, 2.0);
/// #
/// let dv: Vector3<f32> = p1 - p0;
/// let half_dv: Vector3<f32> = dv * 0.5;
/// let midpoint: Point3<f32> = p0 + half_dv;
/// ```
///
/// ## Converting between points and vectors
///
/// Points can be converted to and from displacement vectors using the
/// `EuclideanSpace::{from_vec, to_vec}` methods. Note that under the hood these
/// are implemented as inlined a type conversion, so should not have any
/// performance implications.
///
/// ## References
///
/// - [CGAL 4.7 - 2D and 3D Linear Geometry Kernel: 3.1 Points and Vectors](http://doc.cgal.org/latest/Kernel_23/index.html#Kernel_23PointsandVectors)
/// - [What is the difference between a point and a vector](http://math.stackexchange.com/q/645827)
///
pub trait EuclideanSpace: Copy + Clone
where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: Array<Element = <Self as EuclideanSpace>::Scalar>,

    Self: Add<<Self as EuclideanSpace>::Diff, Output = Self>,
    Self: Sub<<Self as EuclideanSpace>::Diff, Output = Self>,
    Self: Sub<Self, Output = <Self as EuclideanSpace>::Diff>,

    Self: Mul<<Self as EuclideanSpace>::Scalar, Output = Self>,
    Self: Div<<Self as EuclideanSpace>::Scalar, Output = Self>,
    Self: Rem<<Self as EuclideanSpace>::Scalar, Output = Self>,
{
    /// The associated scalar over which the space is defined.
    ///
    /// Due to the equality constraints demanded by `Self::Diff`, this is effectively just an
    /// alias to `Self::Diff::Scalar`.
    type Scalar: BaseNum;

    /// The associated space of displacement vectors.
    type Diff: VectorSpace<Scalar = Self::Scalar>;

    /// The point at the origin of the Euclidean space.
    fn origin() -> Self;

    /// Convert a displacement vector to a point.
    ///
    /// This can be considered equivalent to the addition of the displacement
    /// vector `v` to to `Self::origin()`.
    fn from_vec(v: Self::Diff) -> Self;

    /// Convert a point to a displacement vector.
    ///
    /// This can be seen as equivalent to the displacement vector from
    /// `Self::origin()` to `self`.
    fn to_vec(self) -> Self::Diff;

    /// Returns the middle point between two other points.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Point3;
    ///
    /// let p = Point3::midpoint(
    ///     Point3::new(1.0, 2.0, 3.0),
    ///     Point3::new(3.0, 1.0, 2.0),
    /// );
    /// ```
    #[inline]
    fn midpoint(self, other: Self) -> Self {
        self + (other - self) / cast(2).unwrap()
    }

    /// Returns the average position of all points in the slice.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Point2;
    ///
    /// let triangle = [
    ///     Point2::new(1.0, 1.0),
    ///     Point2::new(2.0, 3.0),
    ///     Point2::new(3.0, 1.0),
    /// ];
    ///
    /// let centroid = Point2::centroid(&triangle);
    /// ```
    #[inline]
    fn centroid(points: &[Self]) -> Self {
        let total_displacement = points
            .iter()
            .fold(Self::Diff::zero(), |acc, p| acc + p.to_vec());

        Self::from_vec(total_displacement / cast(points.len()).unwrap())
    }

    /// This is a weird one, but its useful for plane calculations.
    fn dot(self, v: Self::Diff) -> Self::Scalar;
}

/// A column-major matrix of arbitrary dimensions.
///
/// Because this is constrained to the `VectorSpace` trait, this means that
/// following operators are required to be implemented:
///
/// Matrix addition:
///
/// - `Add<Output = Self>`
/// - `Sub<Output = Self>`
/// - `Neg<Output = Self>`
///
/// Scalar multiplication:
///
/// - `Mul<Self::Scalar, Output = Self>`
/// - `Div<Self::Scalar, Output = Self>`
/// - `Rem<Self::Scalar, Output = Self>`
///
/// Note that matrix multiplication is not required for implementors of this
/// trait. This is due to the complexities of implementing these operators with
/// Rust's current type system. For the multiplication of square matrices,
/// see `SquareMatrix`.
pub trait Matrix: VectorSpace
where
    Self::Scalar: Float,

    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: Index<usize, Output = <Self as Matrix>::Column>,
    Self: IndexMut<usize, Output = <Self as Matrix>::Column>,
{
    /// The row vector of the matrix.
    type Row: VectorSpace<Scalar = Self::Scalar> + Array<Element = Self::Scalar>;

    /// The column vector of the matrix.
    type Column: VectorSpace<Scalar = Self::Scalar> + Array<Element = Self::Scalar>;

    /// The result of transposing the matrix
    type Transpose: Matrix<Scalar = Self::Scalar, Row = Self::Column, Column = Self::Row>;

    /// Get the pointer to the first element of the array.
    #[inline]
    fn as_ptr(&self) -> *const Self::Scalar {
        &self[0][0]
    }

    /// Get a mutable pointer to the first element of the array.
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Scalar {
        &mut self[0][0]
    }

    /// Replace a column in the array.
    #[inline]
    fn replace_col(&mut self, c: usize, src: Self::Column) -> Self::Column {
        use std::mem;

        mem::replace(&mut self[c], src)
    }

    /// Get a row from this matrix by-value.
    fn row(&self, r: usize) -> Self::Row;

    /// Swap two rows of this array.
    fn swap_rows(&mut self, a: usize, b: usize);
    /// Swap two columns of this array.
    fn swap_columns(&mut self, a: usize, b: usize);
    /// Swap the values at index `a` and `b`
    fn swap_elements(&mut self, a: (usize, usize), b: (usize, usize));

    /// Transpose this matrix, returning a new matrix.
    fn transpose(&self) -> Self::Transpose;
}

/// A column-major major matrix where the rows and column vectors are of the same dimensions.
pub trait SquareMatrix
where
    Self::Scalar: Float,

    Self: One,
    Self: iter::Product<Self>,

    Self: Matrix<
        // FIXME: Can be cleaned up once equality constraints in where clauses are implemented
        Column = <Self as SquareMatrix>::ColumnRow,
        Row = <Self as SquareMatrix>::ColumnRow,
        Transpose = Self,
    >,
    Self: Mul<<Self as SquareMatrix>::ColumnRow, Output = <Self as SquareMatrix>::ColumnRow>,
    Self: Mul<Self, Output = Self>,
{
    // FIXME: Will not be needed once equality constraints in where clauses are implemented
    /// The row/column vector of the matrix.
    ///
    /// This is used to constrain the column and rows to be of the same type in lieu of equality
    /// constraints being implemented for `where` clauses. Once those are added, this type will
    /// likely go away.
    type ColumnRow: VectorSpace<Scalar = Self::Scalar> + Array<Element = Self::Scalar>;

    /// Create a new diagonal matrix using the supplied value.
    fn from_value(value: Self::Scalar) -> Self;
    /// Create a matrix from a non-uniform scale
    fn from_diagonal(diagonal: Self::ColumnRow) -> Self;

    /// The [identity matrix]. Multiplying this matrix with another should have
    /// no effect.
    ///
    /// Note that this is exactly the same as `One::one`. The term 'identity
    /// matrix' is more common though, so we provide this method as an
    /// alternative.
    ///
    /// [identity matrix]: https://en.wikipedia.org/wiki/Identity_matrix
    #[inline]
    fn identity() -> Self {
        Self::one()
    }

    /// Transpose this matrix in-place.
    fn transpose_self(&mut self);
    /// Take the determinant of this matrix.
    fn determinant(&self) -> Self::Scalar;

    /// Return a vector containing the diagonal of this matrix.
    fn diagonal(&self) -> Self::ColumnRow;

    /// Return the trace of this matrix. That is, the sum of the diagonal.
    #[inline]
    fn trace(&self) -> Self::Scalar {
        self.diagonal().sum()
    }

    /// Invert this matrix, returning a new matrix. `m.mul_m(m.invert())` is
    /// the identity matrix. Returns `None` if this matrix is not invertible
    /// (has a determinant of zero).
    fn invert(&self) -> Option<Self>;

    /// Test if this matrix is invertible.
    #[inline]
    fn is_invertible(&self) -> bool
    where
        Self::Scalar: approx::UlpsEq,
    {
        ulps_ne!(self.determinant(), &Self::Scalar::zero())
    }

    /// Test if this matrix is the identity matrix. That is, it is diagonal
    /// and every element in the diagonal is one.
    #[inline]
    fn is_identity(&self) -> bool
    where
        Self: approx::UlpsEq,
    {
        ulps_eq!(self, &Self::identity())
    }

    /// Test if this is a diagonal matrix. That is, every element outside of
    /// the diagonal is 0.
    fn is_diagonal(&self) -> bool;

    /// Test if this matrix is symmetric. That is, it is equal to its
    /// transpose.
    fn is_symmetric(&self) -> bool;
}

/// Angles and their associated trigonometric functions.
///
/// Typed angles allow for the writing of self-documenting code that makes it
/// clear when semantic violations have occured - for example, adding degrees to
/// radians, or adding a number to an angle.
///
pub trait Angle
where
    Self: Copy + Clone,
    Self: PartialEq + cmp::PartialOrd,
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: approx::AbsDiffEq<Epsilon = <Self as Angle>::Unitless>,
    Self: approx::RelativeEq<Epsilon = <Self as Angle>::Unitless>,
    Self: approx::UlpsEq<Epsilon = <Self as Angle>::Unitless>,

    Self: Zero,

    Self: Neg<Output = Self>,
    Self: Add<Self, Output = Self>,
    Self: Sub<Self, Output = Self>,
    Self: Rem<Self, Output = Self>,
    Self: Mul<<Self as Angle>::Unitless, Output = Self>,
    Self: Div<Self, Output = <Self as Angle>::Unitless>,
    Self: Div<<Self as Angle>::Unitless, Output = Self>,

    Self: iter::Sum,
{
    type Unitless: BaseFloat;

    /// Return the angle, normalized to the range `[0, full_turn)`.
    #[inline]
    fn normalize(self) -> Self {
        let rem = self % Self::full_turn();
        if rem < Self::zero() {
            rem + Self::full_turn()
        } else {
            rem
        }
    }

    /// Return the angle, normalized to the range `[-turn_div_2, turn_div_2)`.
    #[inline]
    fn normalize_signed(self) -> Self {
        let rem = self.normalize();
        if Self::turn_div_2() < rem {
            rem - Self::full_turn()
        } else {
            rem
        }
    }

    /// Return the angle rotated by half a turn.
    #[inline]
    fn opposite(self) -> Self {
        Self::normalize(self + Self::turn_div_2())
    }

    /// Returns the interior bisector of the two angles.
    #[inline]
    fn bisect(self, other: Self) -> Self {
        let half = cast(0.5f64).unwrap();
        Self::normalize((self - other) * half + self)
    }

    /// A full rotation.
    fn full_turn() -> Self;

    /// Half of a full rotation.
    #[inline]
    fn turn_div_2() -> Self {
        let factor: Self::Unitless = cast(2).unwrap();
        Self::full_turn() / factor
    }

    /// A third of a full rotation.
    #[inline]
    fn turn_div_3() -> Self {
        let factor: Self::Unitless = cast(3).unwrap();
        Self::full_turn() / factor
    }

    /// A quarter of a full rotation.
    #[inline]
    fn turn_div_4() -> Self {
        let factor: Self::Unitless = cast(4).unwrap();
        Self::full_turn() / factor
    }

    /// A sixth of a full rotation.
    #[inline]
    fn turn_div_6() -> Self {
        let factor: Self::Unitless = cast(6).unwrap();
        Self::full_turn() / factor
    }

    /// Compute the sine of the angle, returning a unitless ratio.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad(35.0);
    /// let ratio: f32 = Rad::sin(angle);
    /// ```
    fn sin(self) -> Self::Unitless;

    /// Compute the cosine of the angle, returning a unitless ratio.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad(35.0);
    /// let ratio: f32 = Rad::cos(angle);
    /// ```
    fn cos(self) -> Self::Unitless;

    /// Compute the tangent of the angle, returning a unitless ratio.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad(35.0);
    /// let ratio: f32 = Rad::tan(angle);
    /// ```
    fn tan(self) -> Self::Unitless;

    /// Compute the sine and cosine of the angle, returning the result as a
    /// pair.
    ///
    /// This does not have any performance benefits, but calculating both the
    /// sine and cosine of a single angle is a common operation.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad(35.0);
    /// let (s, c) = Rad::sin_cos(angle);
    /// ```
    fn sin_cos(self) -> (Self::Unitless, Self::Unitless);

    /// Compute the cosecant of the angle.
    ///
    /// This is the same as computing the reciprocal of `Self::sin`.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad(35.0);
    /// let ratio: f32 = Rad::csc(angle);
    /// ```
    #[inline]
    fn csc(self) -> Self::Unitless {
        Self::sin(self).recip()
    }

    /// Compute the cotangent of the angle.
    ///
    /// This is the same as computing the reciprocal of `Self::tan`.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad(35.0);
    /// let ratio: f32 = Rad::cot(angle);
    /// ```
    #[inline]
    fn cot(self) -> Self::Unitless {
        Self::tan(self).recip()
    }

    /// Compute the secant of the angle.
    ///
    /// This is the same as computing the reciprocal of `Self::cos`.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad(35.0);
    /// let ratio: f32 = Rad::sec(angle);
    /// ```
    #[inline]
    fn sec(self) -> Self::Unitless {
        Self::cos(self).recip()
    }

    /// Compute the arcsine of the ratio, returning the resulting angle.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle: Rad<f32> = Rad::asin(0.5);
    /// ```
    fn asin(ratio: Self::Unitless) -> Self;

    /// Compute the arccosine of the ratio, returning the resulting angle.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle: Rad<f32> = Rad::acos(0.5);
    /// ```
    fn acos(ratio: Self::Unitless) -> Self;

    /// Compute the arctangent of the ratio, returning the resulting angle.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle: Rad<f32> = Rad::atan(0.5);
    /// ```
    fn atan(ratio: Self::Unitless) -> Self;

    fn atan2(a: Self::Unitless, b: Self::Unitless) -> Self;
}
