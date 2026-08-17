#[cfg(any(feature = "alloc", feature = "std"))]
use crate::base::dimension::Dyn;
use crate::base::dimension::{U1, U2, U3, U4, U5, U6};
use crate::base::storage::Owned;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::base::vec_storage::VecStorage;
use crate::base::{ArrayStorage, Const, Matrix, Unit};
use crate::storage::OwnedUninit;
use std::mem::MaybeUninit;

/*
 *
 *
 * Column-major matrices.
 *
 *
 */

/// An owned matrix column-major matrix with `R` rows and `C` columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type OMatrix<T, R, C> = Matrix<T, R, C, Owned<T, R, C>>;

/// An owned matrix with uninitialized data.
pub type UninitMatrix<T, R, C> = Matrix<MaybeUninit<T>, R, C, OwnedUninit<T, R, C>>;

/// An owned matrix column-major matrix with `R` rows and `C` columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated(
    note = "use SMatrix for a statically-sized matrix using integer dimensions, or OMatrix for an owned matrix using types as dimensions."
)]
pub type MatrixMN<T, R, C> = Matrix<T, R, C, Owned<T, R, C>>;

/// An owned matrix column-major matrix with `D` columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated(note = "use OMatrix<T, D, D> or SMatrix<T, D, D> instead.")]
pub type MatrixN<T, D> = Matrix<T, D, D, Owned<T, D, D>>;

/// A statically sized column-major matrix with `R` rows and `C` columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type SMatrix<T, const R: usize, const C: usize> =
    Matrix<T, Const<R>, Const<C>, ArrayStorage<T, R, C>>;

/// A dynamically sized column-major matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type DMatrix<T> = Matrix<T, Dyn, Dyn, VecStorage<T, Dyn, Dyn>>;

/// A heap-allocated, column-major, matrix with a dynamic number of rows and 1 columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type MatrixXx1<T> = Matrix<T, Dyn, U1, VecStorage<T, Dyn, U1>>;
/// A heap-allocated, column-major, matrix with a dynamic number of rows and 2 columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type MatrixXx2<T> = Matrix<T, Dyn, U2, VecStorage<T, Dyn, U2>>;
/// A heap-allocated, column-major, matrix with a dynamic number of rows and 3 columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type MatrixXx3<T> = Matrix<T, Dyn, U3, VecStorage<T, Dyn, U3>>;
/// A heap-allocated, column-major, matrix with a dynamic number of rows and 4 columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type MatrixXx4<T> = Matrix<T, Dyn, U4, VecStorage<T, Dyn, U4>>;
/// A heap-allocated, column-major, matrix with a dynamic number of rows and 5 columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type MatrixXx5<T> = Matrix<T, Dyn, U5, VecStorage<T, Dyn, U5>>;
/// A heap-allocated, column-major, matrix with a dynamic number of rows and 6 columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type MatrixXx6<T> = Matrix<T, Dyn, U6, VecStorage<T, Dyn, U6>>;

/// A heap-allocated, column-major, matrix with 1 rows and a dynamic number of columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type Matrix1xX<T> = Matrix<T, U1, Dyn, VecStorage<T, U1, Dyn>>;
/// A heap-allocated, column-major, matrix with 2 rows and a dynamic number of columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type Matrix2xX<T> = Matrix<T, U2, Dyn, VecStorage<T, U2, Dyn>>;
/// A heap-allocated, column-major, matrix with 3 rows and a dynamic number of columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type Matrix3xX<T> = Matrix<T, U3, Dyn, VecStorage<T, U3, Dyn>>;
/// A heap-allocated, column-major, matrix with 4 rows and a dynamic number of columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type Matrix4xX<T> = Matrix<T, U4, Dyn, VecStorage<T, U4, Dyn>>;
/// A heap-allocated, column-major, matrix with 5 rows and a dynamic number of columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type Matrix5xX<T> = Matrix<T, U5, Dyn, VecStorage<T, U5, Dyn>>;
/// A heap-allocated, column-major, matrix with 6 rows and a dynamic number of columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[cfg(any(feature = "std", feature = "alloc"))]
pub type Matrix6xX<T> = Matrix<T, U6, Dyn, VecStorage<T, U6, Dyn>>;

/// A stack-allocated, column-major, 1x1 square matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix1<T> = Matrix<T, U1, U1, ArrayStorage<T, 1, 1>>;
/// A stack-allocated, column-major, 2x2 square matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix2<T> = Matrix<T, U2, U2, ArrayStorage<T, 2, 2>>;
/// A stack-allocated, column-major, 3x3 square matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix3<T> = Matrix<T, U3, U3, ArrayStorage<T, 3, 3>>;
/// A stack-allocated, column-major, 4x4 square matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix4<T> = Matrix<T, U4, U4, ArrayStorage<T, 4, 4>>;
/// A stack-allocated, column-major, 5x5 square matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix5<T> = Matrix<T, U5, U5, ArrayStorage<T, 5, 5>>;
/// A stack-allocated, column-major, 6x6 square matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix6<T> = Matrix<T, U6, U6, ArrayStorage<T, 6, 6>>;

/// A stack-allocated, column-major, 1x2 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix1x2<T> = Matrix<T, U1, U2, ArrayStorage<T, 1, 2>>;
/// A stack-allocated, column-major, 1x3 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix1x3<T> = Matrix<T, U1, U3, ArrayStorage<T, 1, 3>>;
/// A stack-allocated, column-major, 1x4 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix1x4<T> = Matrix<T, U1, U4, ArrayStorage<T, 1, 4>>;
/// A stack-allocated, column-major, 1x5 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix1x5<T> = Matrix<T, U1, U5, ArrayStorage<T, 1, 5>>;
/// A stack-allocated, column-major, 1x6 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix1x6<T> = Matrix<T, U1, U6, ArrayStorage<T, 1, 6>>;

/// A stack-allocated, column-major, 2x3 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix2x3<T> = Matrix<T, U2, U3, ArrayStorage<T, 2, 3>>;
/// A stack-allocated, column-major, 2x4 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix2x4<T> = Matrix<T, U2, U4, ArrayStorage<T, 2, 4>>;
/// A stack-allocated, column-major, 2x5 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix2x5<T> = Matrix<T, U2, U5, ArrayStorage<T, 2, 5>>;
/// A stack-allocated, column-major, 2x6 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix2x6<T> = Matrix<T, U2, U6, ArrayStorage<T, 2, 6>>;

/// A stack-allocated, column-major, 3x4 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix3x4<T> = Matrix<T, U3, U4, ArrayStorage<T, 3, 4>>;
/// A stack-allocated, column-major, 3x5 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix3x5<T> = Matrix<T, U3, U5, ArrayStorage<T, 3, 5>>;
/// A stack-allocated, column-major, 3x6 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix3x6<T> = Matrix<T, U3, U6, ArrayStorage<T, 3, 6>>;

/// A stack-allocated, column-major, 4x5 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix4x5<T> = Matrix<T, U4, U5, ArrayStorage<T, 4, 5>>;
/// A stack-allocated, column-major, 4x6 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix4x6<T> = Matrix<T, U4, U6, ArrayStorage<T, 4, 6>>;

/// A stack-allocated, column-major, 5x6 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix5x6<T> = Matrix<T, U5, U6, ArrayStorage<T, 5, 6>>;

/// A stack-allocated, column-major, 2x1 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix2x1<T> = Matrix<T, U2, U1, ArrayStorage<T, 2, 1>>;
/// A stack-allocated, column-major, 3x1 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix3x1<T> = Matrix<T, U3, U1, ArrayStorage<T, 3, 1>>;
/// A stack-allocated, column-major, 4x1 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix4x1<T> = Matrix<T, U4, U1, ArrayStorage<T, 4, 1>>;
/// A stack-allocated, column-major, 5x1 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix5x1<T> = Matrix<T, U5, U1, ArrayStorage<T, 5, 1>>;
/// A stack-allocated, column-major, 6x1 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix6x1<T> = Matrix<T, U6, U1, ArrayStorage<T, 6, 1>>;

/// A stack-allocated, column-major, 3x2 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix3x2<T> = Matrix<T, U3, U2, ArrayStorage<T, 3, 2>>;
/// A stack-allocated, column-major, 4x2 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix4x2<T> = Matrix<T, U4, U2, ArrayStorage<T, 4, 2>>;
/// A stack-allocated, column-major, 5x2 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix5x2<T> = Matrix<T, U5, U2, ArrayStorage<T, 5, 2>>;
/// A stack-allocated, column-major, 6x2 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix6x2<T> = Matrix<T, U6, U2, ArrayStorage<T, 6, 2>>;

/// A stack-allocated, column-major, 4x3 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix4x3<T> = Matrix<T, U4, U3, ArrayStorage<T, 4, 3>>;
/// A stack-allocated, column-major, 5x3 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix5x3<T> = Matrix<T, U5, U3, ArrayStorage<T, 5, 3>>;
/// A stack-allocated, column-major, 6x3 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix6x3<T> = Matrix<T, U6, U3, ArrayStorage<T, 6, 3>>;

/// A stack-allocated, column-major, 5x4 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix5x4<T> = Matrix<T, U5, U4, ArrayStorage<T, 5, 4>>;
/// A stack-allocated, column-major, 6x4 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix6x4<T> = Matrix<T, U6, U4, ArrayStorage<T, 6, 4>>;

/// A stack-allocated, column-major, 6x5 matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type Matrix6x5<T> = Matrix<T, U6, U5, ArrayStorage<T, 6, 5>>;

/*
 *
 *
 * Column vectors.
 *
 *
 */
/// A dynamically sized column vector.
#[cfg(any(feature = "std", feature = "alloc"))]
pub type DVector<T> = Matrix<T, Dyn, U1, VecStorage<T, Dyn, U1>>;

/// An owned D-dimensional column vector.
pub type OVector<T, D> = Matrix<T, D, U1, Owned<T, D, U1>>;
/// A statically sized D-dimensional column vector.
pub type SVector<T, const D: usize> = Matrix<T, Const<D>, U1, ArrayStorage<T, D, 1>>; // Owned<T, Const<D>, U1>>;

/// An owned matrix with uninitialized data.
pub type UninitVector<T, D> = Matrix<MaybeUninit<T>, D, U1, OwnedUninit<T, D, U1>>;

/// An owned matrix column-major matrix with `R` rows and `C` columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated(
    note = "use SVector for a statically-sized matrix using integer dimensions, or OVector for an owned matrix using types as dimensions."
)]
pub type VectorN<T, D> = Matrix<T, D, U1, Owned<T, D, U1>>;

/// A stack-allocated, 1-dimensional column vector.
pub type Vector1<T> = Matrix<T, U1, U1, ArrayStorage<T, 1, 1>>;
/// A stack-allocated, 2-dimensional column vector.
pub type Vector2<T> = Matrix<T, U2, U1, ArrayStorage<T, 2, 1>>;
/// A stack-allocated, 3-dimensional column vector.
pub type Vector3<T> = Matrix<T, U3, U1, ArrayStorage<T, 3, 1>>;
/// A stack-allocated, 4-dimensional column vector.
pub type Vector4<T> = Matrix<T, U4, U1, ArrayStorage<T, 4, 1>>;
/// A stack-allocated, 5-dimensional column vector.
pub type Vector5<T> = Matrix<T, U5, U1, ArrayStorage<T, 5, 1>>;
/// A stack-allocated, 6-dimensional column vector.
pub type Vector6<T> = Matrix<T, U6, U1, ArrayStorage<T, 6, 1>>;

/*
 *
 *
 * Row vectors.
 *
 *
 */
/// A dynamically sized row vector.
#[cfg(any(feature = "std", feature = "alloc"))]
pub type RowDVector<T> = Matrix<T, U1, Dyn, VecStorage<T, U1, Dyn>>;

/// An owned D-dimensional row vector.
pub type RowOVector<T, D> = Matrix<T, U1, D, Owned<T, U1, D>>;

/// A statically sized D-dimensional row vector.
pub type RowSVector<T, const D: usize> = Matrix<T, U1, Const<D>, ArrayStorage<T, 1, D>>;

/// A stack-allocated, 1-dimensional row vector.
pub type RowVector1<T> = Matrix<T, U1, U1, ArrayStorage<T, 1, 1>>;
/// A stack-allocated, 2-dimensional row vector.
pub type RowVector2<T> = Matrix<T, U1, U2, ArrayStorage<T, 1, 2>>;
/// A stack-allocated, 3-dimensional row vector.
pub type RowVector3<T> = Matrix<T, U1, U3, ArrayStorage<T, 1, 3>>;
/// A stack-allocated, 4-dimensional row vector.
pub type RowVector4<T> = Matrix<T, U1, U4, ArrayStorage<T, 1, 4>>;
/// A stack-allocated, 5-dimensional row vector.
pub type RowVector5<T> = Matrix<T, U1, U5, ArrayStorage<T, 1, 5>>;
/// A stack-allocated, 6-dimensional row vector.
pub type RowVector6<T> = Matrix<T, U1, U6, ArrayStorage<T, 1, 6>>;

/*
 *
 *
 * Unit Vector.
 *
 *
 */
/// A stack-allocated, 1-dimensional unit vector.
pub type UnitVector1<T> = Unit<Matrix<T, U1, U1, ArrayStorage<T, 1, 1>>>;
/// A stack-allocated, 2-dimensional unit vector.
pub type UnitVector2<T> = Unit<Matrix<T, U2, U1, ArrayStorage<T, 2, 1>>>;
/// A stack-allocated, 3-dimensional unit vector.
pub type UnitVector3<T> = Unit<Matrix<T, U3, U1, ArrayStorage<T, 3, 1>>>;
/// A stack-allocated, 4-dimensional unit vector.
pub type UnitVector4<T> = Unit<Matrix<T, U4, U1, ArrayStorage<T, 4, 1>>>;
/// A stack-allocated, 5-dimensional unit vector.
pub type UnitVector5<T> = Unit<Matrix<T, U5, U1, ArrayStorage<T, 5, 1>>>;
/// A stack-allocated, 6-dimensional unit vector.
pub type UnitVector6<T> = Unit<Matrix<T, U6, U1, ArrayStorage<T, 6, 1>>>;
