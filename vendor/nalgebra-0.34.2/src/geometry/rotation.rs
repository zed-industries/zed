// Needed otherwise the rkyv macros generate code incompatible with rust-2024
#![cfg_attr(feature = "rkyv-serialize", allow(unsafe_op_in_unsafe_fn))]

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num::{One, Zero};
use std::fmt;
use std::hash;

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde-serialize-no-std")]
use crate::base::storage::Owned;

use simba::scalar::RealField;
use simba::simd::SimdRealField;

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, OMatrix, SMatrix, SVector, Scalar, Unit};
use crate::geometry::Point;

#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;

/// A rotation matrix.
///
/// This is also known as an element of a Special Orthogonal (SO) group.
/// The `Rotation` type can either represent a 2D or 3D rotation, represented as a matrix.
/// For a rotation based on quaternions, see [`UnitQuaternion`](crate::UnitQuaternion) instead.
///
/// Note that instead of using the [`Rotation`] type in your code directly, you should use one
/// of its aliases: [`Rotation2`](crate::Rotation2), or [`Rotation3`](crate::Rotation3). Though
/// keep in mind that all the documentation of all the methods of these aliases will also appears on
/// this page.
///
/// # Construction
/// * [Identity <span style="float:right;">`identity`</span>](#identity)
/// * [From a 2D rotation angle <span style="float:right;">`new`…</span>](#construction-from-a-2d-rotation-angle)
/// * [From an existing 2D matrix or rotations <span style="float:right;">`from_matrix`, `rotation_between`, `powf`…</span>](#construction-from-an-existing-2d-matrix-or-rotations)
/// * [From a 3D axis and/or angles <span style="float:right;">`new`, `from_euler_angles`, `from_axis_angle`…</span>](#construction-from-a-3d-axis-andor-angles)
/// * [From a 3D eye position and target point <span style="float:right;">`look_at`, `look_at_lh`, `rotation_between`…</span>](#construction-from-a-3d-eye-position-and-target-point)
/// * [From an existing 3D matrix or rotations <span style="float:right;">`from_matrix`, `rotation_between`, `powf`…</span>](#construction-from-an-existing-3d-matrix-or-rotations)
///
/// # Transformation and composition
/// Note that transforming vectors and points can be done by multiplication, e.g., `rotation * point`.
/// Composing an rotation with another transformation can also be done by multiplication or division.
/// * [3D axis and angle extraction <span style="float:right;">`angle`, `euler_angles`, `scaled_axis`, `angle_to`…</span>](#3d-axis-and-angle-extraction)
/// * [2D angle extraction <span style="float:right;">`angle`, `angle_to`…</span>](#2d-angle-extraction)
/// * [Transformation of a vector or a point <span style="float:right;">`transform_vector`, `inverse_transform_point`…</span>](#transformation-of-a-vector-or-a-point)
/// * [Transposition and inversion <span style="float:right;">`transpose`, `inverse`…</span>](#transposition-and-inversion)
/// * [Interpolation <span style="float:right;">`slerp`…</span>](#interpolation)
///
/// # Conversion
/// * [Conversion to a matrix <span style="float:right;">`matrix`, `to_homogeneous`…</span>](#conversion-to-a-matrix)
///
#[repr(C)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(
        as = "Rotation<T::Archived, D>",
        bound(archive = "
        T: rkyv::Archive,
        SMatrix<T, D, D>: rkyv::Archive<Archived = SMatrix<T::Archived, D, D>>
    ")
    )
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone)]
pub struct Rotation<T, const D: usize> {
    matrix: SMatrix<T, D, D>,
}

impl<T: fmt::Debug, const D: usize> fmt::Debug for Rotation<T, D> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.matrix.fmt(formatter)
    }
}

impl<T: Scalar + hash::Hash, const D: usize> hash::Hash for Rotation<T, D>
where
    <DefaultAllocator as Allocator<Const<D>, Const<D>>>::Buffer<T>: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.matrix.hash(state)
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, const D: usize> bytemuck::Zeroable for Rotation<T, D>
where
    T: Scalar + bytemuck::Zeroable,
    SMatrix<T, D, D>: bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, const D: usize> bytemuck::Pod for Rotation<T, D>
where
    T: Scalar + bytemuck::Pod,
    SMatrix<T, D, D>: bytemuck::Pod,
{
}

#[cfg(feature = "serde-serialize-no-std")]
impl<T: Scalar, const D: usize> Serialize for Rotation<T, D>
where
    Owned<T, Const<D>, Const<D>>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.matrix.serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'a, T: Scalar, const D: usize> Deserialize<'a> for Rotation<T, D>
where
    Owned<T, Const<D>, Const<D>>: Deserialize<'a>,
{
    fn deserialize<Des>(deserializer: Des) -> Result<Self, Des::Error>
    where
        Des: Deserializer<'a>,
    {
        let matrix = SMatrix::<T, D, D>::deserialize(deserializer)?;

        Ok(Self::from_matrix_unchecked(matrix))
    }
}

impl<T, const D: usize> Rotation<T, D> {
    /// Creates a new rotation from the given square matrix.
    ///
    /// The matrix orthonormality is not checked.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Rotation2, Rotation3, Matrix2, Matrix3};
    /// # use std::f32;
    /// let mat = Matrix3::new(0.8660254, -0.5,      0.0,
    ///                        0.5,       0.8660254, 0.0,
    ///                        0.0,       0.0,       1.0);
    /// let rot = Rotation3::from_matrix_unchecked(mat);
    ///
    /// assert_eq!(*rot.matrix(), mat);
    ///
    ///
    /// let mat = Matrix2::new(0.8660254, -0.5,
    ///                        0.5,       0.8660254);
    /// let rot = Rotation2::from_matrix_unchecked(mat);
    ///
    /// assert_eq!(*rot.matrix(), mat);
    /// ```
    #[inline]
    pub const fn from_matrix_unchecked(matrix: SMatrix<T, D, D>) -> Self {
        Self { matrix }
    }
}

/// # Conversion to a matrix
impl<T: Scalar, const D: usize> Rotation<T, D> {
    /// A reference to the underlying matrix representation of this rotation.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Rotation2, Rotation3, Vector3, Matrix2, Matrix3};
    /// # use std::f32;
    /// let rot = Rotation3::from_axis_angle(&Vector3::z_axis(), f32::consts::FRAC_PI_6);
    /// let expected = Matrix3::new(0.8660254, -0.5,      0.0,
    ///                             0.5,       0.8660254, 0.0,
    ///                             0.0,       0.0,       1.0);
    /// assert_eq!(*rot.matrix(), expected);
    ///
    ///
    /// let rot = Rotation2::new(f32::consts::FRAC_PI_6);
    /// let expected = Matrix2::new(0.8660254, -0.5,
    ///                             0.5,       0.8660254);
    /// assert_eq!(*rot.matrix(), expected);
    /// ```
    #[inline]
    #[must_use]
    pub const fn matrix(&self) -> &SMatrix<T, D, D> {
        &self.matrix
    }

    /// A mutable reference to the underlying matrix representation of this rotation.
    ///
    /// # Safety
    ///
    /// Invariants of the rotation matrix should not be violated.
    #[inline]
    #[deprecated(note = "Use `.matrix_mut_unchecked()` instead.")]
    pub const unsafe fn matrix_mut(&mut self) -> &mut SMatrix<T, D, D> {
        &mut self.matrix
    }

    /// A mutable reference to the underlying matrix representation of this rotation.
    ///
    /// This is suffixed by "_unchecked" because this allows the user to replace the
    /// matrix by another one that is non-inversible or non-orthonormal. If one of
    /// those properties is broken, subsequent method calls may return bogus results.
    #[inline]
    pub const fn matrix_mut_unchecked(&mut self) -> &mut SMatrix<T, D, D> {
        &mut self.matrix
    }

    /// Unwraps the underlying matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Rotation2, Rotation3, Vector3, Matrix2, Matrix3};
    /// # use std::f32;
    /// let rot = Rotation3::from_axis_angle(&Vector3::z_axis(), f32::consts::FRAC_PI_6);
    /// let mat = rot.into_inner();
    /// let expected = Matrix3::new(0.8660254, -0.5,      0.0,
    ///                             0.5,       0.8660254, 0.0,
    ///                             0.0,       0.0,       1.0);
    /// assert_eq!(mat, expected);
    ///
    ///
    /// let rot = Rotation2::new(f32::consts::FRAC_PI_6);
    /// let mat = rot.into_inner();
    /// let expected = Matrix2::new(0.8660254, -0.5,
    ///                             0.5,       0.8660254);
    /// assert_eq!(mat, expected);
    /// ```
    #[inline]
    pub fn into_inner(self) -> SMatrix<T, D, D> {
        self.matrix
    }

    /// Unwraps the underlying matrix.
    /// Deprecated: Use [`Rotation::into_inner`] instead.
    #[deprecated(note = "use `.into_inner()` instead")]
    #[inline]
    pub fn unwrap(self) -> SMatrix<T, D, D> {
        self.matrix
    }

    /// Converts this rotation into its equivalent homogeneous transformation matrix.
    ///
    /// This is the same as `self.into()`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Rotation2, Rotation3, Vector3, Matrix3, Matrix4};
    /// # use std::f32;
    /// let rot = Rotation3::from_axis_angle(&Vector3::z_axis(), f32::consts::FRAC_PI_6);
    /// let expected = Matrix4::new(0.8660254, -0.5,      0.0, 0.0,
    ///                             0.5,       0.8660254, 0.0, 0.0,
    ///                             0.0,       0.0,       1.0, 0.0,
    ///                             0.0,       0.0,       0.0, 1.0);
    /// assert_eq!(rot.to_homogeneous(), expected);
    ///
    ///
    /// let rot = Rotation2::new(f32::consts::FRAC_PI_6);
    /// let expected = Matrix3::new(0.8660254, -0.5,      0.0,
    ///                             0.5,       0.8660254, 0.0,
    ///                             0.0,       0.0,       1.0);
    /// assert_eq!(rot.to_homogeneous(), expected);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
    where
        T: Zero + One,
        Const<D>: DimNameAdd<U1>,
        DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    {
        // We could use `SMatrix::to_homogeneous()` here, but that would imply
        // adding the additional traits `DimAdd` and `IsNotStaticOne`. Maybe
        // these things will get nicer once specialization lands in Rust.
        let mut res = OMatrix::<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>::identity();
        res.fixed_view_mut::<D, D>(0, 0).copy_from(&self.matrix);

        res
    }
}

/// # Transposition and inversion
impl<T: Scalar, const D: usize> Rotation<T, D> {
    /// Transposes `self`.
    ///
    /// Same as `.inverse()` because the inverse of a rotation matrix is its transpose.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Rotation2, Rotation3, Vector3};
    /// let rot = Rotation3::new(Vector3::new(1.0, 2.0, 3.0));
    /// let tr_rot = rot.transpose();
    /// assert_relative_eq!(rot * tr_rot, Rotation3::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(tr_rot * rot, Rotation3::identity(), epsilon = 1.0e-6);
    ///
    /// let rot = Rotation2::new(1.2);
    /// let tr_rot = rot.transpose();
    /// assert_relative_eq!(rot * tr_rot, Rotation2::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(tr_rot * rot, Rotation2::identity(), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use = "Did you mean to use transpose_mut()?"]
    pub fn transpose(&self) -> Self {
        Self::from_matrix_unchecked(self.matrix.transpose())
    }

    /// Inverts `self`.
    ///
    /// Same as `.transpose()` because the inverse of a rotation matrix is its transpose.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Rotation2, Rotation3, Vector3};
    /// let rot = Rotation3::new(Vector3::new(1.0, 2.0, 3.0));
    /// let inv = rot.inverse();
    /// assert_relative_eq!(rot * inv, Rotation3::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(inv * rot, Rotation3::identity(), epsilon = 1.0e-6);
    ///
    /// let rot = Rotation2::new(1.2);
    /// let inv = rot.inverse();
    /// assert_relative_eq!(rot * inv, Rotation2::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(inv * rot, Rotation2::identity(), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use = "Did you mean to use inverse_mut()?"]
    pub fn inverse(&self) -> Self {
        self.transpose()
    }

    /// Transposes `self` in-place.
    ///
    /// Same as `.inverse_mut()` because the inverse of a rotation matrix is its transpose.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Rotation2, Rotation3, Vector3};
    /// let rot = Rotation3::new(Vector3::new(1.0, 2.0, 3.0));
    /// let mut tr_rot = Rotation3::new(Vector3::new(1.0, 2.0, 3.0));
    /// tr_rot.transpose_mut();
    ///
    /// assert_relative_eq!(rot * tr_rot, Rotation3::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(tr_rot * rot, Rotation3::identity(), epsilon = 1.0e-6);
    ///
    /// let rot = Rotation2::new(1.2);
    /// let mut tr_rot = Rotation2::new(1.2);
    /// tr_rot.transpose_mut();
    ///
    /// assert_relative_eq!(rot * tr_rot, Rotation2::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(tr_rot * rot, Rotation2::identity(), epsilon = 1.0e-6);
    /// ```
    #[inline]
    pub fn transpose_mut(&mut self) {
        self.matrix.transpose_mut()
    }

    /// Inverts `self` in-place.
    ///
    /// Same as `.transpose_mut()` because the inverse of a rotation matrix is its transpose.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Rotation2, Rotation3, Vector3};
    /// let rot = Rotation3::new(Vector3::new(1.0, 2.0, 3.0));
    /// let mut inv = Rotation3::new(Vector3::new(1.0, 2.0, 3.0));
    /// inv.inverse_mut();
    ///
    /// assert_relative_eq!(rot * inv, Rotation3::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(inv * rot, Rotation3::identity(), epsilon = 1.0e-6);
    ///
    /// let rot = Rotation2::new(1.2);
    /// let mut inv = Rotation2::new(1.2);
    /// inv.inverse_mut();
    ///
    /// assert_relative_eq!(rot * inv, Rotation2::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(inv * rot, Rotation2::identity(), epsilon = 1.0e-6);
    /// ```
    #[inline]
    pub fn inverse_mut(&mut self) {
        self.transpose_mut()
    }
}

/// # Transformation of a vector or a point
impl<T: SimdRealField, const D: usize> Rotation<T, D>
where
    T::Element: SimdRealField,
{
    /// Rotate the given point.
    ///
    /// This is the same as the multiplication `self * pt`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Point3, Rotation2, Rotation3, UnitQuaternion, Vector3};
    /// let rot = Rotation3::new(Vector3::y() * f32::consts::FRAC_PI_2);
    /// let transformed_point = rot.transform_point(&Point3::new(1.0, 2.0, 3.0));
    ///
    /// assert_relative_eq!(transformed_point, Point3::new(3.0, 2.0, -1.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self * pt
    }

    /// Rotate the given vector.
    ///
    /// This is the same as the multiplication `self * v`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Rotation2, Rotation3, UnitQuaternion, Vector3};
    /// let rot = Rotation3::new(Vector3::y() * f32::consts::FRAC_PI_2);
    /// let transformed_vector = rot.transform_vector(&Vector3::new(1.0, 2.0, 3.0));
    ///
    /// assert_relative_eq!(transformed_vector, Vector3::new(3.0, 2.0, -1.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self * v
    }

    /// Rotate the given point by the inverse of this rotation. This may be
    /// cheaper than inverting the rotation and then transforming the given
    /// point.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Point3, Rotation2, Rotation3, UnitQuaternion, Vector3};
    /// let rot = Rotation3::new(Vector3::y() * f32::consts::FRAC_PI_2);
    /// let transformed_point = rot.inverse_transform_point(&Point3::new(1.0, 2.0, 3.0));
    ///
    /// assert_relative_eq!(transformed_point, Point3::new(-3.0, 2.0, 1.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        Point::from(self.inverse_transform_vector(&pt.coords))
    }

    /// Rotate the given vector by the inverse of this rotation. This may be
    /// cheaper than inverting the rotation and then transforming the given
    /// vector.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Rotation2, Rotation3, UnitQuaternion, Vector3};
    /// let rot = Rotation3::new(Vector3::y() * f32::consts::FRAC_PI_2);
    /// let transformed_vector = rot.inverse_transform_vector(&Vector3::new(1.0, 2.0, 3.0));
    ///
    /// assert_relative_eq!(transformed_vector, Vector3::new(-3.0, 2.0, 1.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.matrix().tr_mul(v)
    }

    /// Rotate the given vector by the inverse of this rotation. This may be
    /// cheaper than inverting the rotation and then transforming the given
    /// vector.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Rotation2, Rotation3, UnitQuaternion, Vector3};
    /// let rot = Rotation3::new(Vector3::z() * f32::consts::FRAC_PI_2);
    /// let transformed_vector = rot.inverse_transform_unit_vector(&Vector3::x_axis());
    ///
    /// assert_relative_eq!(transformed_vector, -Vector3::y_axis(), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_unit_vector(&self, v: &Unit<SVector<T, D>>) -> Unit<SVector<T, D>> {
        Unit::new_unchecked(self.inverse_transform_vector(&**v))
    }
}

impl<T: Scalar + Eq, const D: usize> Eq for Rotation<T, D> {}

impl<T: Scalar + PartialEq, const D: usize> PartialEq for Rotation<T, D> {
    #[inline]
    fn eq(&self, right: &Self) -> bool {
        self.matrix == right.matrix
    }
}

impl<T, const D: usize> AbsDiffEq for Rotation<T, D>
where
    T: Scalar + AbsDiffEq,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.matrix.abs_diff_eq(&other.matrix, epsilon)
    }
}

impl<T, const D: usize> RelativeEq for Rotation<T, D>
where
    T: Scalar + RelativeEq,
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.matrix
            .relative_eq(&other.matrix, epsilon, max_relative)
    }
}

impl<T, const D: usize> UlpsEq for Rotation<T, D>
where
    T: Scalar + UlpsEq,
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.matrix.ulps_eq(&other.matrix, epsilon, max_ulps)
    }
}

/*
 *
 * Display
 *
 */
impl<T, const D: usize> fmt::Display for Rotation<T, D>
where
    T: RealField + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision().unwrap_or(3);

        writeln!(f, "Rotation matrix {{")?;
        write!(f, "{:.*}", precision, self.matrix)?;
        writeln!(f, "}}")
    }
}

//          //         /*
//          //          *
//          //          * Absolute
//          //          *
//          //          */
//          //         impl<T: Absolute> Absolute for $t<T> {
//          //             type AbsoluteValue = $submatrix<T::AbsoluteValue>;
//          //
//          //             #[inline]
//          //             fn abs(m: &$t<T>) -> $submatrix<T::AbsoluteValue> {
//          //                 Absolute::abs(&m.submatrix)
//          //             }
//          //         }
