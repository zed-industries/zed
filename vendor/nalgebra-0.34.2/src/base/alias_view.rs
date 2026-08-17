use crate::base::dimension::{Dyn, U1, U2, U3, U4, U5, U6};
use crate::base::matrix_view::{ViewStorage, ViewStorageMut};
use crate::base::{Const, Matrix};

/*
 *
 *
 * Matrix view aliases.
 *
 *
 */
// NOTE: we can't provide defaults for the strides because it's not supported yet by min_const_generics.
/// An immutable column-major matrix view with dimensions known at compile-time.
///
/// See [`SMatrixViewMut`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type SMatrixView<'a, T, const R: usize, const C: usize> =
    Matrix<T, Const<R>, Const<C>, ViewStorage<'a, T, Const<R>, Const<C>, Const<1>, Const<R>>>;

/// An immutable column-major matrix view dynamic numbers of rows and columns.
///
/// See [`DMatrixViewMut`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type DMatrixView<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, Dyn, ViewStorage<'a, T, Dyn, Dyn, RStride, CStride>>;

/// An immutable column-major 1x1 matrix view.
///
/// See [`MatrixViewMut1`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView1<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U1, ViewStorage<'a, T, U1, U1, RStride, CStride>>;
/// An immutable column-major 2x2 matrix view.
///
/// See [`MatrixViewMut2`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView2<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U2, ViewStorage<'a, T, U2, U2, RStride, CStride>>;
/// An immutable column-major 3x3 matrix view.
///
/// See [`MatrixViewMut3`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView3<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U3, ViewStorage<'a, T, U3, U3, RStride, CStride>>;
/// An immutable column-major 4x4 matrix view.
///
/// See [`MatrixViewMut4`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView4<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U4, ViewStorage<'a, T, U4, U4, RStride, CStride>>;
/// An immutable column-major 5x5 matrix view.
///
/// See [`MatrixViewMut5`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView5<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U5, ViewStorage<'a, T, U5, U5, RStride, CStride>>;
/// An immutable column-major 6x6 matrix view.
///
/// See [`MatrixViewMut6`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView6<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U6, ViewStorage<'a, T, U6, U6, RStride, CStride>>;

/// An immutable column-major 1x2 matrix view.
///
/// See [`MatrixViewMut1x2`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView1x2<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U2, ViewStorage<'a, T, U1, U2, RStride, CStride>>;
/// An immutable column-major 1x3 matrix view.
///
/// See [`MatrixViewMut1x3`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView1x3<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U3, ViewStorage<'a, T, U1, U3, RStride, CStride>>;
/// An immutable column-major 1x4 matrix view.
///
/// See [`MatrixViewMut1x4`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView1x4<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U4, ViewStorage<'a, T, U1, U4, RStride, CStride>>;
/// An immutable column-major 1x5 matrix view.
///
/// See [`MatrixViewMut1x5`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView1x5<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U5, ViewStorage<'a, T, U1, U5, RStride, CStride>>;
/// An immutable column-major 1x6 matrix view.
///
/// See [`MatrixViewMut1x6`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView1x6<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U6, ViewStorage<'a, T, U1, U6, RStride, CStride>>;

/// An immutable column-major 2x1 matrix view.
///
/// See [`MatrixViewMut2x1`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView2x1<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U1, ViewStorage<'a, T, U2, U1, RStride, CStride>>;
/// An immutable column-major 2x3 matrix view.
///
/// See [`MatrixViewMut2x3`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView2x3<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U3, ViewStorage<'a, T, U2, U3, RStride, CStride>>;
/// An immutable column-major 2x4 matrix view.
///
/// See [`MatrixViewMut2x4`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView2x4<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U4, ViewStorage<'a, T, U2, U4, RStride, CStride>>;
/// An immutable column-major 2x5 matrix view.
///
/// See [`MatrixViewMut2x5`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView2x5<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U5, ViewStorage<'a, T, U2, U5, RStride, CStride>>;
/// An immutable column-major 2x6 matrix view.
///
/// See [`MatrixViewMut2x6`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView2x6<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U6, ViewStorage<'a, T, U2, U6, RStride, CStride>>;

/// An immutable column-major 3x1 matrix view.
///
/// See [`MatrixViewMut3x1`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView3x1<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U1, ViewStorage<'a, T, U3, U1, RStride, CStride>>;
/// An immutable column-major 3x2 matrix view.
///
/// See [`MatrixViewMut3x2`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView3x2<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U2, ViewStorage<'a, T, U3, U2, RStride, CStride>>;
/// An immutable column-major 3x4 matrix view.
///
/// See [`MatrixViewMut3x4`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView3x4<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U4, ViewStorage<'a, T, U3, U4, RStride, CStride>>;
/// An immutable column-major 3x5 matrix view.
///
/// See [`MatrixViewMut3x5`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView3x5<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U5, ViewStorage<'a, T, U3, U5, RStride, CStride>>;
/// An immutable column-major 3x6 matrix view.
///
/// See [`MatrixViewMut3x6`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView3x6<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U6, ViewStorage<'a, T, U3, U6, RStride, CStride>>;

/// An immutable column-major 4x1 matrix view.
///
/// See [`MatrixViewMut4x1`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView4x1<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U1, ViewStorage<'a, T, U4, U1, RStride, CStride>>;
/// An immutable column-major 4x2 matrix view.
///
/// See [`MatrixViewMut4x2`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView4x2<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U2, ViewStorage<'a, T, U4, U2, RStride, CStride>>;
/// An immutable column-major 4x3 matrix view.
///
/// See [`MatrixViewMut4x3`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView4x3<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U3, ViewStorage<'a, T, U4, U3, RStride, CStride>>;
/// An immutable column-major 4x5 matrix view.
///
/// See [`MatrixViewMut4x5`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView4x5<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U5, ViewStorage<'a, T, U4, U5, RStride, CStride>>;
/// An immutable column-major 4x6 matrix view.
///
/// See [`MatrixViewMut4x6`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView4x6<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U6, ViewStorage<'a, T, U4, U6, RStride, CStride>>;

/// An immutable column-major 5x1 matrix view.
///
/// See [`MatrixViewMut5x1`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView5x1<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U1, ViewStorage<'a, T, U5, U1, RStride, CStride>>;
/// An immutable column-major 5x2 matrix view.
///
/// See [`MatrixViewMut5x2`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView5x2<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U2, ViewStorage<'a, T, U5, U2, RStride, CStride>>;
/// An immutable column-major 5x3 matrix view.
///
/// See [`MatrixViewMut5x3`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView5x3<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U3, ViewStorage<'a, T, U5, U3, RStride, CStride>>;
/// An immutable column-major 5x4 matrix view.
///
/// See [`MatrixViewMut5x4`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView5x4<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U4, ViewStorage<'a, T, U5, U4, RStride, CStride>>;
/// An immutable column-major 5x6 matrix view.
///
/// See [`MatrixViewMut5x6`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView5x6<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U6, ViewStorage<'a, T, U5, U6, RStride, CStride>>;

/// An immutable column-major 6x1 matrix view.
///
/// See [`MatrixViewMut6x1`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView6x1<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U1, ViewStorage<'a, T, U6, U1, RStride, CStride>>;
/// An immutable column-major 6x2 matrix view.
///
/// See [`MatrixViewMut6x2`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView6x2<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U2, ViewStorage<'a, T, U6, U2, RStride, CStride>>;
/// An immutable column-major 6x3 matrix view.
///
/// See [`MatrixViewMut6x3`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView6x3<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U3, ViewStorage<'a, T, U6, U3, RStride, CStride>>;
/// An immutable column-major 6x4 matrix view.
///
/// See [`MatrixViewMut6x4`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView6x4<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U4, ViewStorage<'a, T, U6, U4, RStride, CStride>>;
/// An immutable column-major 6x5 matrix view.
///
/// See [`MatrixViewMut6x5`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView6x5<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U5, ViewStorage<'a, T, U6, U5, RStride, CStride>>;

/// An immutable column-major matrix view with 1 row and a number of columns chosen at runtime.
///
/// See [`MatrixViewMut1xX`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView1xX<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, Dyn, ViewStorage<'a, T, U1, Dyn, RStride, CStride>>;
/// An immutable column-major matrix view with 2 rows and a number of columns chosen at runtime.
///
/// See [`MatrixViewMut2xX`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView2xX<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, Dyn, ViewStorage<'a, T, U2, Dyn, RStride, CStride>>;
/// An immutable column-major matrix view with 3 rows and a number of columns chosen at runtime.
///
/// See [`MatrixViewMut3xX`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView3xX<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, Dyn, ViewStorage<'a, T, U3, Dyn, RStride, CStride>>;
/// An immutable column-major matrix view with 4 rows and a number of columns chosen at runtime.
///
/// See [`MatrixViewMut4xX`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView4xX<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, Dyn, ViewStorage<'a, T, U4, Dyn, RStride, CStride>>;
/// An immutable column-major matrix view with 5 rows and a number of columns chosen at runtime.
///
/// See [`MatrixViewMut5xX`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView5xX<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, Dyn, ViewStorage<'a, T, U5, Dyn, RStride, CStride>>;
/// An immutable column-major matrix view with 6 rows and a number of columns chosen at runtime.
///
/// See [`MatrixViewMut6xX`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixView6xX<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, Dyn, ViewStorage<'a, T, U6, Dyn, RStride, CStride>>;

/// An immutable column-major matrix view with a number of rows chosen at runtime and 1 column.
///
/// See [`MatrixViewMutXx1`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewXx1<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U1, ViewStorage<'a, T, Dyn, U1, RStride, CStride>>;
/// An immutable column-major matrix view with a number of rows chosen at runtime and 2 columns.
///
/// See [`MatrixViewMutXx2`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewXx2<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U2, ViewStorage<'a, T, Dyn, U2, RStride, CStride>>;
/// An immutable column-major matrix view with a number of rows chosen at runtime and 3 columns.
///
/// See [`MatrixViewMutXx3`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewXx3<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U3, ViewStorage<'a, T, Dyn, U3, RStride, CStride>>;
/// An immutable column-major matrix view with a number of rows chosen at runtime and 4 columns.
///
/// See [`MatrixViewMutXx4`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewXx4<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U4, ViewStorage<'a, T, Dyn, U4, RStride, CStride>>;
/// An immutable column-major matrix view with a number of rows chosen at runtime and 5 columns.
///
/// See [`MatrixViewMutXx5`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewXx5<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U5, ViewStorage<'a, T, Dyn, U5, RStride, CStride>>;
/// An immutable column-major matrix view with a number of rows chosen at runtime and 6 columns.
///
/// See [`MatrixViewMutXx6`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewXx6<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U6, ViewStorage<'a, T, Dyn, U6, RStride, CStride>>;

/// An immutable column vector view with dimensions known at compile-time.
///
/// See [`VectorViewMut`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorView<'a, T, D, RStride = U1, CStride = D> =
    Matrix<T, D, U1, ViewStorage<'a, T, D, U1, RStride, CStride>>;

/// An immutable column vector view with dimensions known at compile-time.
///
/// See [`SVectorViewMut`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type SVectorView<'a, T, const D: usize> =
    Matrix<T, Const<D>, Const<1>, ViewStorage<'a, T, Const<D>, Const<1>, Const<1>, Const<D>>>;

/// An immutable column vector view dynamic numbers of rows and columns.
///
/// See [`DVectorViewMut`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type DVectorView<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U1, ViewStorage<'a, T, Dyn, U1, RStride, CStride>>;

/// An immutable 1D column vector view.
///
/// See [`VectorViewMut1`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorView1<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U1, ViewStorage<'a, T, U1, U1, RStride, CStride>>;
/// An immutable 2D column vector view.
///
/// See [`VectorViewMut2`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorView2<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U1, ViewStorage<'a, T, U2, U1, RStride, CStride>>;
/// An immutable 3D column vector view.
///
/// See [`VectorViewMut3`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorView3<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U1, ViewStorage<'a, T, U3, U1, RStride, CStride>>;
/// An immutable 4D column vector view.
///
/// See [`VectorViewMut4`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorView4<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U1, ViewStorage<'a, T, U4, U1, RStride, CStride>>;
/// An immutable 5D column vector view.
///
/// See [`VectorViewMut5`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorView5<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U1, ViewStorage<'a, T, U5, U1, RStride, CStride>>;
/// An immutable 6D column vector view.
///
/// See [`VectorViewMut6`] for a mutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorView6<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U1, ViewStorage<'a, T, U6, U1, RStride, CStride>>;

/*
 *
 *
 * Same thing, but for mutable views.
 *
 *
 */

/// A mutable column-major matrix view with dimensions known at compile-time.
///
/// See [`SMatrixView`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type SMatrixViewMut<'a, T, const R: usize, const C: usize> =
    Matrix<T, Const<R>, Const<C>, ViewStorageMut<'a, T, Const<R>, Const<C>, Const<1>, Const<R>>>;

/// A mutable column-major matrix view dynamic numbers of rows and columns.
///
/// See [`DMatrixView`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type DMatrixViewMut<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, Dyn, ViewStorageMut<'a, T, Dyn, Dyn, RStride, CStride>>;

/// A mutable column-major 1x1 matrix view.
///
/// See [`MatrixView1`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut1<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U1, ViewStorageMut<'a, T, U1, U1, RStride, CStride>>;
/// A mutable column-major 2x2 matrix view.
///
/// See [`MatrixView2`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut2<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U2, ViewStorageMut<'a, T, U2, U2, RStride, CStride>>;
/// A mutable column-major 3x3 matrix view.
///
/// See [`MatrixView3`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut3<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U3, ViewStorageMut<'a, T, U3, U3, RStride, CStride>>;
/// A mutable column-major 4x4 matrix view.
///
/// See [`MatrixView4`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut4<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U4, ViewStorageMut<'a, T, U4, U4, RStride, CStride>>;
/// A mutable column-major 5x5 matrix view.
///
/// See [`MatrixView5`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut5<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U5, ViewStorageMut<'a, T, U5, U5, RStride, CStride>>;
/// A mutable column-major 6x6 matrix view.
///
/// See [`MatrixView6`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut6<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U6, ViewStorageMut<'a, T, U6, U6, RStride, CStride>>;

/// A mutable column-major 1x2 matrix view.
///
/// See [`MatrixView1x2`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut1x2<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U2, ViewStorageMut<'a, T, U1, U2, RStride, CStride>>;
/// A mutable column-major 1x3 matrix view.
///
/// See [`MatrixView1x3`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut1x3<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U3, ViewStorageMut<'a, T, U1, U3, RStride, CStride>>;
/// A mutable column-major 1x4 matrix view.
///
/// See [`MatrixView1x4`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut1x4<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U4, ViewStorageMut<'a, T, U1, U4, RStride, CStride>>;
/// A mutable column-major 1x5 matrix view.
///
/// See [`MatrixView1x5`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut1x5<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U5, ViewStorageMut<'a, T, U1, U5, RStride, CStride>>;
/// A mutable column-major 1x6 matrix view.
///
/// See [`MatrixView1x6`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut1x6<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U6, ViewStorageMut<'a, T, U1, U6, RStride, CStride>>;

/// A mutable column-major 2x1 matrix view.
///
/// See [`MatrixView2x1`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut2x1<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U1, ViewStorageMut<'a, T, U2, U1, RStride, CStride>>;
/// A mutable column-major 2x3 matrix view.
///
/// See [`MatrixView2x3`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut2x3<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U3, ViewStorageMut<'a, T, U2, U3, RStride, CStride>>;
/// A mutable column-major 2x4 matrix view.
///
/// See [`MatrixView2x4`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut2x4<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U4, ViewStorageMut<'a, T, U2, U4, RStride, CStride>>;
/// A mutable column-major 2x5 matrix view.
///
/// See [`MatrixView2x5`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut2x5<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U5, ViewStorageMut<'a, T, U2, U5, RStride, CStride>>;
/// A mutable column-major 2x6 matrix view.
///
/// See [`MatrixView2x6`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut2x6<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U6, ViewStorageMut<'a, T, U2, U6, RStride, CStride>>;

/// A mutable column-major 3x1 matrix view.
///
/// See [`MatrixView3x1`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut3x1<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U1, ViewStorageMut<'a, T, U3, U1, RStride, CStride>>;
/// A mutable column-major 3x2 matrix view.
///
/// See [`MatrixView3x2`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut3x2<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U2, ViewStorageMut<'a, T, U3, U2, RStride, CStride>>;
/// A mutable column-major 3x4 matrix view.
///
/// See [`MatrixView3x4`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut3x4<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U4, ViewStorageMut<'a, T, U3, U4, RStride, CStride>>;
/// A mutable column-major 3x5 matrix view.
///
/// See [`MatrixView3x5`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut3x5<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U5, ViewStorageMut<'a, T, U3, U5, RStride, CStride>>;
/// A mutable column-major 3x6 matrix view.
///
/// See [`MatrixView3x6`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut3x6<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U6, ViewStorageMut<'a, T, U3, U6, RStride, CStride>>;

/// A mutable column-major 4x1 matrix view.
///
/// See [`MatrixView4x1`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut4x1<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U1, ViewStorageMut<'a, T, U4, U1, RStride, CStride>>;
/// A mutable column-major 4x2 matrix view.
///
/// See [`MatrixView4x2`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut4x2<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U2, ViewStorageMut<'a, T, U4, U2, RStride, CStride>>;
/// A mutable column-major 4x3 matrix view.
///
/// See [`MatrixView4x3`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut4x3<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U3, ViewStorageMut<'a, T, U4, U3, RStride, CStride>>;
/// A mutable column-major 4x5 matrix view.
///
/// See [`MatrixView4x5`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut4x5<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U5, ViewStorageMut<'a, T, U4, U5, RStride, CStride>>;
/// A mutable column-major 4x6 matrix view.
///
/// See [`MatrixView4x6`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut4x6<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U6, ViewStorageMut<'a, T, U4, U6, RStride, CStride>>;

/// A mutable column-major 5x1 matrix view.
///
/// See [`MatrixView5x1`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut5x1<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U1, ViewStorageMut<'a, T, U5, U1, RStride, CStride>>;
/// A mutable column-major 5x2 matrix view.
///
/// See [`MatrixView5x2`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut5x2<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U2, ViewStorageMut<'a, T, U5, U2, RStride, CStride>>;
/// A mutable column-major 5x3 matrix view.
///
/// See [`MatrixView5x3`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut5x3<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U3, ViewStorageMut<'a, T, U5, U3, RStride, CStride>>;
/// A mutable column-major 5x4 matrix view.
///
/// See [`MatrixView5x4`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut5x4<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U4, ViewStorageMut<'a, T, U5, U4, RStride, CStride>>;
/// A mutable column-major 5x6 matrix view.
///
/// See [`MatrixView5x6`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut5x6<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U6, ViewStorageMut<'a, T, U5, U6, RStride, CStride>>;

/// A mutable column-major 6x1 matrix view.
///
/// See [`MatrixView6x1`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut6x1<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U1, ViewStorageMut<'a, T, U6, U1, RStride, CStride>>;
/// A mutable column-major 6x2 matrix view.
///
/// See [`MatrixView6x2`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut6x2<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U2, ViewStorageMut<'a, T, U6, U2, RStride, CStride>>;
/// A mutable column-major 6x3 matrix view.
///
/// See [`MatrixView6x3`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut6x3<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U3, ViewStorageMut<'a, T, U6, U3, RStride, CStride>>;
/// A mutable column-major 6x4 matrix view.
///
/// See [`MatrixView6x4`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut6x4<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U4, ViewStorageMut<'a, T, U6, U4, RStride, CStride>>;
/// A mutable column-major 6x5 matrix view.
///
/// See [`MatrixView6x5`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut6x5<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U5, ViewStorageMut<'a, T, U6, U5, RStride, CStride>>;

/// A mutable column-major matrix view with 1 row and a number of columns chosen at runtime.
///
/// See [`MatrixView1xX`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut1xX<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, Dyn, ViewStorageMut<'a, T, U1, Dyn, RStride, CStride>>;
/// A mutable column-major matrix view with 2 rows and a number of columns chosen at runtime.
///
/// See [`MatrixView2xX`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut2xX<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, Dyn, ViewStorageMut<'a, T, U2, Dyn, RStride, CStride>>;
/// A mutable column-major matrix view with 3 rows and a number of columns chosen at runtime.
///
/// See [`MatrixView3xX`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut3xX<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, Dyn, ViewStorageMut<'a, T, U3, Dyn, RStride, CStride>>;
/// A mutable column-major matrix view with 4 rows and a number of columns chosen at runtime.
///
/// See [`MatrixView4xX`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut4xX<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, Dyn, ViewStorageMut<'a, T, U4, Dyn, RStride, CStride>>;
/// A mutable column-major matrix view with 5 rows and a number of columns chosen at runtime.
///
/// See [`MatrixView5xX`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut5xX<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, Dyn, ViewStorageMut<'a, T, U5, Dyn, RStride, CStride>>;
/// A mutable column-major matrix view with 6 rows and a number of columns chosen at runtime.
///
/// See [`MatrixView6xX`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMut6xX<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, Dyn, ViewStorageMut<'a, T, U6, Dyn, RStride, CStride>>;

/// A mutable column-major matrix view with a number of rows chosen at runtime and 1 column.
///
/// See [`MatrixViewXx1`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMutXx1<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U1, ViewStorageMut<'a, T, Dyn, U1, RStride, CStride>>;
/// A mutable column-major matrix view with a number of rows chosen at runtime and 2 columns.
///
/// See [`MatrixViewXx2`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMutXx2<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U2, ViewStorageMut<'a, T, Dyn, U2, RStride, CStride>>;
/// A mutable column-major matrix view with a number of rows chosen at runtime and 3 columns.
///
/// See [`MatrixViewXx3`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMutXx3<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U3, ViewStorageMut<'a, T, Dyn, U3, RStride, CStride>>;
/// A mutable column-major matrix view with a number of rows chosen at runtime and 4 columns.
///
/// See [`MatrixViewXx4`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMutXx4<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U4, ViewStorageMut<'a, T, Dyn, U4, RStride, CStride>>;
/// A mutable column-major matrix view with a number of rows chosen at runtime and 5 columns.
///
/// See [`MatrixViewXx5`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMutXx5<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U5, ViewStorageMut<'a, T, Dyn, U5, RStride, CStride>>;
/// A mutable column-major matrix view with a number of rows chosen at runtime and 6 columns.
///
/// See [`MatrixViewXx6`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type MatrixViewMutXx6<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U6, ViewStorageMut<'a, T, Dyn, U6, RStride, CStride>>;

/// A mutable column vector view with dimensions known at compile-time.
///
/// See [`VectorView`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorViewMut<'a, T, D, RStride = U1, CStride = D> =
    Matrix<T, D, U1, ViewStorageMut<'a, T, D, U1, RStride, CStride>>;

/// A mutable column vector view with dimensions known at compile-time.
///
/// See [`SVectorView`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type SVectorViewMut<'a, T, const D: usize> =
    Matrix<T, Const<D>, Const<1>, ViewStorageMut<'a, T, Const<D>, Const<1>, Const<1>, Const<D>>>;

/// A mutable column vector view dynamic numbers of rows and columns.
///
/// See [`DVectorView`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type DVectorViewMut<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U1, ViewStorageMut<'a, T, Dyn, U1, RStride, CStride>>;

/// A mutable 1D column vector view.
///
/// See [`VectorView1`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorViewMut1<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U1, ViewStorageMut<'a, T, U1, U1, RStride, CStride>>;
/// A mutable 2D column vector view.
///
/// See [`VectorView2`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorViewMut2<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U1, ViewStorageMut<'a, T, U2, U1, RStride, CStride>>;
/// A mutable 3D column vector view.
///
/// See [`VectorView3`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorViewMut3<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U1, ViewStorageMut<'a, T, U3, U1, RStride, CStride>>;
/// A mutable 4D column vector view.
///
/// See [`VectorView4`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorViewMut4<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U1, ViewStorageMut<'a, T, U4, U1, RStride, CStride>>;
/// A mutable 5D column vector view.
///
/// See [`VectorView5`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorViewMut5<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U1, ViewStorageMut<'a, T, U5, U1, RStride, CStride>>;
/// A mutable 6D column vector view.
///
/// See [`VectorView6`] for an immutable version of this type.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
pub type VectorViewMut6<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U1, ViewStorageMut<'a, T, U6, U1, RStride, CStride>>;
