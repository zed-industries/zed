use crate::base::dimension::{Dyn, U1, U2, U3, U4, U5, U6};
use crate::base::matrix_view::{ViewStorage, ViewStorageMut};
use crate::base::{Const, Matrix};
use crate::slice_deprecation_note;

/*
 *
 *
 * Matrix slice aliases.
 *
 *
 */
// NOTE: we can't provide defaults for the strides because it's not supported yet by min_const_generics.
/// A column-major matrix slice with dimensions known at compile-time.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(SMatrixView)]
pub type SMatrixSlice<'a, T, const R: usize, const C: usize> =
    Matrix<T, Const<R>, Const<C>, ViewStorage<'a, T, Const<R>, Const<C>, Const<1>, Const<R>>>;

/// A column-major matrix slice dynamic numbers of rows and columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(DMatrixView)]
pub type DMatrixSlice<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, Dyn, ViewStorage<'a, T, Dyn, Dyn, RStride, CStride>>;

/// A column-major 1x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView1)]
pub type MatrixSlice1<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U1, ViewStorage<'a, T, U1, U1, RStride, CStride>>;
/// A column-major 2x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView2)]
pub type MatrixSlice2<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U2, ViewStorage<'a, T, U2, U2, RStride, CStride>>;
/// A column-major 3x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView3)]
pub type MatrixSlice3<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U3, ViewStorage<'a, T, U3, U3, RStride, CStride>>;
/// A column-major 4x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView4)]
pub type MatrixSlice4<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U4, ViewStorage<'a, T, U4, U4, RStride, CStride>>;
/// A column-major 5x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView5)]
pub type MatrixSlice5<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U5, ViewStorage<'a, T, U5, U5, RStride, CStride>>;
/// A column-major 6x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView6)]
pub type MatrixSlice6<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U6, ViewStorage<'a, T, U6, U6, RStride, CStride>>;

/// A column-major 1x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView1x2)]
pub type MatrixSlice1x2<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U2, ViewStorage<'a, T, U1, U2, RStride, CStride>>;
/// A column-major 1x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView1x3)]
pub type MatrixSlice1x3<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U3, ViewStorage<'a, T, U1, U3, RStride, CStride>>;
/// A column-major 1x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView1x4)]
pub type MatrixSlice1x4<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U4, ViewStorage<'a, T, U1, U4, RStride, CStride>>;
/// A column-major 1x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView1x5)]
pub type MatrixSlice1x5<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U5, ViewStorage<'a, T, U1, U5, RStride, CStride>>;
/// A column-major 1x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView1x6)]
pub type MatrixSlice1x6<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U6, ViewStorage<'a, T, U1, U6, RStride, CStride>>;

/// A column-major 2x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView2x1)]
pub type MatrixSlice2x1<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U1, ViewStorage<'a, T, U2, U1, RStride, CStride>>;
/// A column-major 2x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView2x3)]
pub type MatrixSlice2x3<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U3, ViewStorage<'a, T, U2, U3, RStride, CStride>>;
/// A column-major 2x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView2x4)]
pub type MatrixSlice2x4<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U4, ViewStorage<'a, T, U2, U4, RStride, CStride>>;
/// A column-major 2x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView2x5)]
pub type MatrixSlice2x5<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U5, ViewStorage<'a, T, U2, U5, RStride, CStride>>;
/// A column-major 2x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView2x6)]
pub type MatrixSlice2x6<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U6, ViewStorage<'a, T, U2, U6, RStride, CStride>>;

/// A column-major 3x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView3x1)]
pub type MatrixSlice3x1<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U1, ViewStorage<'a, T, U3, U1, RStride, CStride>>;
/// A column-major 3x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView3x2)]
pub type MatrixSlice3x2<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U2, ViewStorage<'a, T, U3, U2, RStride, CStride>>;
/// A column-major 3x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView3x4)]
pub type MatrixSlice3x4<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U4, ViewStorage<'a, T, U3, U4, RStride, CStride>>;
/// A column-major 3x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView3x5)]
pub type MatrixSlice3x5<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U5, ViewStorage<'a, T, U3, U5, RStride, CStride>>;
/// A column-major 3x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView3x6)]
pub type MatrixSlice3x6<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U6, ViewStorage<'a, T, U3, U6, RStride, CStride>>;

/// A column-major 4x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView4x1)]
pub type MatrixSlice4x1<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U1, ViewStorage<'a, T, U4, U1, RStride, CStride>>;
/// A column-major 4x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView4x2)]
pub type MatrixSlice4x2<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U2, ViewStorage<'a, T, U4, U2, RStride, CStride>>;
/// A column-major 4x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView4x3)]
pub type MatrixSlice4x3<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U3, ViewStorage<'a, T, U4, U3, RStride, CStride>>;
/// A column-major 4x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView4x5)]
pub type MatrixSlice4x5<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U5, ViewStorage<'a, T, U4, U5, RStride, CStride>>;
/// A column-major 4x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView4x6)]
pub type MatrixSlice4x6<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U6, ViewStorage<'a, T, U4, U6, RStride, CStride>>;

/// A column-major 5x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView5x1)]
pub type MatrixSlice5x1<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U1, ViewStorage<'a, T, U5, U1, RStride, CStride>>;
/// A column-major 5x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView5x2)]
pub type MatrixSlice5x2<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U2, ViewStorage<'a, T, U5, U2, RStride, CStride>>;
/// A column-major 5x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView5x3)]
pub type MatrixSlice5x3<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U3, ViewStorage<'a, T, U5, U3, RStride, CStride>>;
/// A column-major 5x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView5x4)]
pub type MatrixSlice5x4<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U4, ViewStorage<'a, T, U5, U4, RStride, CStride>>;
/// A column-major 5x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView5x6)]
pub type MatrixSlice5x6<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U6, ViewStorage<'a, T, U5, U6, RStride, CStride>>;

/// A column-major 6x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView6x1)]
pub type MatrixSlice6x1<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U1, ViewStorage<'a, T, U6, U1, RStride, CStride>>;
/// A column-major 6x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView6x2)]
pub type MatrixSlice6x2<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U2, ViewStorage<'a, T, U6, U2, RStride, CStride>>;
/// A column-major 6x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView6x3)]
pub type MatrixSlice6x3<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U3, ViewStorage<'a, T, U6, U3, RStride, CStride>>;
/// A column-major 6x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView6x4)]
pub type MatrixSlice6x4<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U4, ViewStorage<'a, T, U6, U4, RStride, CStride>>;
/// A column-major 6x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixView6x5)]
pub type MatrixSlice6x5<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U5, ViewStorage<'a, T, U6, U5, RStride, CStride>>;

/// A column-major matrix slice with 1 row and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixView1xX)]
pub type MatrixSlice1xX<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, Dyn, ViewStorage<'a, T, U1, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 2 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixView2xX)]
pub type MatrixSlice2xX<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, Dyn, ViewStorage<'a, T, U2, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 3 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixView3xX)]
pub type MatrixSlice3xX<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, Dyn, ViewStorage<'a, T, U3, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 4 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixView4xX)]
pub type MatrixSlice4xX<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, Dyn, ViewStorage<'a, T, U4, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 5 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixView5xX)]
pub type MatrixSlice5xX<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, Dyn, ViewStorage<'a, T, U5, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 6 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixView6xX)]
pub type MatrixSlice6xX<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, Dyn, ViewStorage<'a, T, U6, Dyn, RStride, CStride>>;

/// A column-major matrix slice with a number of rows chosen at runtime and 1 column.
#[deprecated = slice_deprecation_note!(MatrixViewXx1)]
pub type MatrixSliceXx1<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U1, ViewStorage<'a, T, Dyn, U1, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 2 columns.
#[deprecated = slice_deprecation_note!(MatrixViewXx2)]
pub type MatrixSliceXx2<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U2, ViewStorage<'a, T, Dyn, U2, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 3 columns.
#[deprecated = slice_deprecation_note!(MatrixViewXx3)]
pub type MatrixSliceXx3<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U3, ViewStorage<'a, T, Dyn, U3, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 4 columns.
#[deprecated = slice_deprecation_note!(MatrixViewXx4)]
pub type MatrixSliceXx4<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U4, ViewStorage<'a, T, Dyn, U4, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 5 columns.
#[deprecated = slice_deprecation_note!(MatrixViewXx5)]
pub type MatrixSliceXx5<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U5, ViewStorage<'a, T, Dyn, U5, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 6 columns.
#[deprecated = slice_deprecation_note!(MatrixViewXx6)]
pub type MatrixSliceXx6<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U6, ViewStorage<'a, T, Dyn, U6, RStride, CStride>>;

/// A column vector slice with dimensions known at compile-time.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorView)]
pub type VectorSlice<'a, T, D, RStride = U1, CStride = D> =
    Matrix<T, D, U1, ViewStorage<'a, T, D, U1, RStride, CStride>>;

/// A column vector slice with dimensions known at compile-time.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(SVectorView)]
pub type SVectorSlice<'a, T, const D: usize> =
    Matrix<T, Const<D>, Const<1>, ViewStorage<'a, T, Const<D>, Const<1>, Const<1>, Const<D>>>;

/// A column vector slice dynamic numbers of rows and columns.
#[deprecated = slice_deprecation_note!(DVectorView)]
pub type DVectorSlice<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U1, ViewStorage<'a, T, Dyn, U1, RStride, CStride>>;

/// A 1D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorView1)]
pub type VectorSlice1<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U1, ViewStorage<'a, T, U1, U1, RStride, CStride>>;
/// A 2D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorView2)]
pub type VectorSlice2<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U1, ViewStorage<'a, T, U2, U1, RStride, CStride>>;
/// A 3D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorView3)]
pub type VectorSlice3<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U1, ViewStorage<'a, T, U3, U1, RStride, CStride>>;
/// A 4D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorView4)]
pub type VectorSlice4<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U1, ViewStorage<'a, T, U4, U1, RStride, CStride>>;
/// A 5D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorView5)]
pub type VectorSlice5<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U1, ViewStorage<'a, T, U5, U1, RStride, CStride>>;
/// A 6D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorView6)]
pub type VectorSlice6<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U1, ViewStorage<'a, T, U6, U1, RStride, CStride>>;

/*
 *
 *
 * Same thing, but for mutable slices.
 *
 *
 */
/// A column-major matrix slice with `R` rows and `C` columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = "Use MatrixViewMut instead, which has an identical definition."]
pub type MatrixSliceMutMN<'a, T, R, C, RStride = U1, CStride = R> =
    Matrix<T, R, C, ViewStorageMut<'a, T, R, C, RStride, CStride>>;

/// A column-major matrix slice with `D` rows and columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = "Use MatrixViewMut instead."]
pub type MatrixSliceMutN<'a, T, D, RStride = U1, CStride = D> =
    Matrix<T, D, D, ViewStorageMut<'a, T, D, D, RStride, CStride>>;

/// A column-major matrix slice with dimensions known at compile-time.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(SMatrixViewMut)]
pub type SMatrixSliceMut<'a, T, const R: usize, const C: usize> =
    Matrix<T, Const<R>, Const<C>, ViewStorageMut<'a, T, Const<R>, Const<C>, Const<1>, Const<R>>>;

/// A column-major matrix slice dynamic numbers of rows and columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(DMatrixViewMut)]
pub type DMatrixSliceMut<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, Dyn, ViewStorageMut<'a, T, Dyn, Dyn, RStride, CStride>>;

/// A column-major 1x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut1)]
pub type MatrixSliceMut1<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U1, ViewStorageMut<'a, T, U1, U1, RStride, CStride>>;
/// A column-major 2x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut2)]
pub type MatrixSliceMut2<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U2, ViewStorageMut<'a, T, U2, U2, RStride, CStride>>;
/// A column-major 3x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut3)]
pub type MatrixSliceMut3<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U3, ViewStorageMut<'a, T, U3, U3, RStride, CStride>>;
/// A column-major 4x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut4)]
pub type MatrixSliceMut4<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U4, ViewStorageMut<'a, T, U4, U4, RStride, CStride>>;
/// A column-major 5x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut5)]
pub type MatrixSliceMut5<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U5, ViewStorageMut<'a, T, U5, U5, RStride, CStride>>;
/// A column-major 6x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut6)]
pub type MatrixSliceMut6<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U6, ViewStorageMut<'a, T, U6, U6, RStride, CStride>>;

/// A column-major 1x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut1x2)]
pub type MatrixSliceMut1x2<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U2, ViewStorageMut<'a, T, U1, U2, RStride, CStride>>;
/// A column-major 1x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut1x3)]
pub type MatrixSliceMut1x3<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U3, ViewStorageMut<'a, T, U1, U3, RStride, CStride>>;
/// A column-major 1x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut1x4)]
pub type MatrixSliceMut1x4<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U4, ViewStorageMut<'a, T, U1, U4, RStride, CStride>>;
/// A column-major 1x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut1x5)]
pub type MatrixSliceMut1x5<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U5, ViewStorageMut<'a, T, U1, U5, RStride, CStride>>;
/// A column-major 1x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut1x6)]
pub type MatrixSliceMut1x6<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U6, ViewStorageMut<'a, T, U1, U6, RStride, CStride>>;

/// A column-major 2x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut2x1)]
pub type MatrixSliceMut2x1<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U1, ViewStorageMut<'a, T, U2, U1, RStride, CStride>>;
/// A column-major 2x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut2x3)]
pub type MatrixSliceMut2x3<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U3, ViewStorageMut<'a, T, U2, U3, RStride, CStride>>;
/// A column-major 2x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut2x4)]
pub type MatrixSliceMut2x4<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U4, ViewStorageMut<'a, T, U2, U4, RStride, CStride>>;
/// A column-major 2x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut2x5)]
pub type MatrixSliceMut2x5<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U5, ViewStorageMut<'a, T, U2, U5, RStride, CStride>>;
/// A column-major 2x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut2x6)]
pub type MatrixSliceMut2x6<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U6, ViewStorageMut<'a, T, U2, U6, RStride, CStride>>;

/// A column-major 3x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut3x1)]
pub type MatrixSliceMut3x1<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U1, ViewStorageMut<'a, T, U3, U1, RStride, CStride>>;
/// A column-major 3x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut3x2)]
pub type MatrixSliceMut3x2<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U2, ViewStorageMut<'a, T, U3, U2, RStride, CStride>>;
/// A column-major 3x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut3x4)]
pub type MatrixSliceMut3x4<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U4, ViewStorageMut<'a, T, U3, U4, RStride, CStride>>;
/// A column-major 3x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut3x5)]
pub type MatrixSliceMut3x5<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U5, ViewStorageMut<'a, T, U3, U5, RStride, CStride>>;
/// A column-major 3x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut3x6)]
pub type MatrixSliceMut3x6<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U6, ViewStorageMut<'a, T, U3, U6, RStride, CStride>>;

/// A column-major 4x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut4x1)]
pub type MatrixSliceMut4x1<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U1, ViewStorageMut<'a, T, U4, U1, RStride, CStride>>;
/// A column-major 4x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut4x2)]
pub type MatrixSliceMut4x2<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U2, ViewStorageMut<'a, T, U4, U2, RStride, CStride>>;
/// A column-major 4x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut4x3)]
pub type MatrixSliceMut4x3<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U3, ViewStorageMut<'a, T, U4, U3, RStride, CStride>>;
/// A column-major 4x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut4x5)]
pub type MatrixSliceMut4x5<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U5, ViewStorageMut<'a, T, U4, U5, RStride, CStride>>;
/// A column-major 4x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut4x6)]
pub type MatrixSliceMut4x6<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U6, ViewStorageMut<'a, T, U4, U6, RStride, CStride>>;

/// A column-major 5x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut5x1)]
pub type MatrixSliceMut5x1<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U1, ViewStorageMut<'a, T, U5, U1, RStride, CStride>>;
/// A column-major 5x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut5x2)]
pub type MatrixSliceMut5x2<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U2, ViewStorageMut<'a, T, U5, U2, RStride, CStride>>;
/// A column-major 5x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut5x3)]
pub type MatrixSliceMut5x3<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U3, ViewStorageMut<'a, T, U5, U3, RStride, CStride>>;
/// A column-major 5x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut5x4)]
pub type MatrixSliceMut5x4<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U4, ViewStorageMut<'a, T, U5, U4, RStride, CStride>>;
/// A column-major 5x6 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut5x6)]
pub type MatrixSliceMut5x6<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U6, ViewStorageMut<'a, T, U5, U6, RStride, CStride>>;

/// A column-major 6x1 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut6x1)]
pub type MatrixSliceMut6x1<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U1, ViewStorageMut<'a, T, U6, U1, RStride, CStride>>;
/// A column-major 6x2 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut6x2)]
pub type MatrixSliceMut6x2<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U2, ViewStorageMut<'a, T, U6, U2, RStride, CStride>>;
/// A column-major 6x3 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut6x3)]
pub type MatrixSliceMut6x3<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U3, ViewStorageMut<'a, T, U6, U3, RStride, CStride>>;
/// A column-major 6x4 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut6x4)]
pub type MatrixSliceMut6x4<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U4, ViewStorageMut<'a, T, U6, U4, RStride, CStride>>;
/// A column-major 6x5 matrix slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(MatrixViewMut6x5)]
pub type MatrixSliceMut6x5<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U5, ViewStorageMut<'a, T, U6, U5, RStride, CStride>>;

/// A column-major matrix slice with 1 row and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixViewMut1xX)]
pub type MatrixSliceMut1xX<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, Dyn, ViewStorageMut<'a, T, U1, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 2 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixViewMut2xX)]
pub type MatrixSliceMut2xX<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, Dyn, ViewStorageMut<'a, T, U2, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 3 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixViewMut3xX)]
pub type MatrixSliceMut3xX<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, Dyn, ViewStorageMut<'a, T, U3, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 4 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixViewMut4xX)]
pub type MatrixSliceMut4xX<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, Dyn, ViewStorageMut<'a, T, U4, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 5 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixViewMut5xX)]
pub type MatrixSliceMut5xX<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, Dyn, ViewStorageMut<'a, T, U5, Dyn, RStride, CStride>>;
/// A column-major matrix slice with 6 rows and a number of columns chosen at runtime.
#[deprecated = slice_deprecation_note!(MatrixViewMut6xX)]
pub type MatrixSliceMut6xX<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, Dyn, ViewStorageMut<'a, T, U6, Dyn, RStride, CStride>>;

/// A column-major matrix slice with a number of rows chosen at runtime and 1 column.
#[deprecated = slice_deprecation_note!(MatrixViewMutXx1)]
pub type MatrixSliceMutXx1<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U1, ViewStorageMut<'a, T, Dyn, U1, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 2 columns.
#[deprecated = slice_deprecation_note!(MatrixViewMutXx2)]
pub type MatrixSliceMutXx2<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U2, ViewStorageMut<'a, T, Dyn, U2, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 3 columns.
#[deprecated = slice_deprecation_note!(MatrixViewMutXx3)]
pub type MatrixSliceMutXx3<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U3, ViewStorageMut<'a, T, Dyn, U3, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 4 columns.
#[deprecated = slice_deprecation_note!(MatrixViewMutXx4)]
pub type MatrixSliceMutXx4<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U4, ViewStorageMut<'a, T, Dyn, U4, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 5 columns.
#[deprecated = slice_deprecation_note!(MatrixViewMutXx5)]
pub type MatrixSliceMutXx5<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U5, ViewStorageMut<'a, T, Dyn, U5, RStride, CStride>>;
/// A column-major matrix slice with a number of rows chosen at runtime and 6 columns.
#[deprecated = slice_deprecation_note!(MatrixViewMutXx6)]
pub type MatrixSliceMutXx6<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U6, ViewStorageMut<'a, T, Dyn, U6, RStride, CStride>>;

/// A column vector slice with dimensions known at compile-time.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorViewMut)]
pub type VectorSliceMut<'a, T, D, RStride = U1, CStride = D> =
    Matrix<T, D, U1, ViewStorageMut<'a, T, D, U1, RStride, CStride>>;

/// A column vector slice with dimensions known at compile-time.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(SVectorViewMut)]
pub type SVectorSliceMut<'a, T, const D: usize> =
    Matrix<T, Const<D>, Const<1>, ViewStorageMut<'a, T, Const<D>, Const<1>, Const<1>, Const<D>>>;

/// A column vector slice dynamic numbers of rows and columns.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(DVectorViewMut)]
pub type DVectorSliceMut<'a, T, RStride = U1, CStride = Dyn> =
    Matrix<T, Dyn, U1, ViewStorageMut<'a, T, Dyn, U1, RStride, CStride>>;

/// A 1D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorViewMut1)]
pub type VectorSliceMut1<'a, T, RStride = U1, CStride = U1> =
    Matrix<T, U1, U1, ViewStorageMut<'a, T, U1, U1, RStride, CStride>>;
/// A 2D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorViewMut2)]
pub type VectorSliceMut2<'a, T, RStride = U1, CStride = U2> =
    Matrix<T, U2, U1, ViewStorageMut<'a, T, U2, U1, RStride, CStride>>;
/// A 3D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorViewMut3)]
pub type VectorSliceMut3<'a, T, RStride = U1, CStride = U3> =
    Matrix<T, U3, U1, ViewStorageMut<'a, T, U3, U1, RStride, CStride>>;
/// A 4D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorViewMut4)]
pub type VectorSliceMut4<'a, T, RStride = U1, CStride = U4> =
    Matrix<T, U4, U1, ViewStorageMut<'a, T, U4, U1, RStride, CStride>>;
/// A 5D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorViewMut5)]
pub type VectorSliceMut5<'a, T, RStride = U1, CStride = U5> =
    Matrix<T, U5, U1, ViewStorageMut<'a, T, U5, U1, RStride, CStride>>;
/// A 6D column vector slice.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Matrix`] type too.**
#[deprecated = slice_deprecation_note!(VectorViewMut6)]
pub type VectorSliceMut6<'a, T, RStride = U1, CStride = U6> =
    Matrix<T, U6, U1, ViewStorageMut<'a, T, U6, U1, RStride, CStride>>;
