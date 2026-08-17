// Needed otherwise the rkyv macros generate code incompatible with rust-2024
#![cfg_attr(feature = "rkyv-serialize", allow(unsafe_op_in_unsafe_fn))]

use num::{One, Zero};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::any::TypeId;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "rkyv-serialize-no-std")]
use super::rkyv_wrappers::CustomPhantom;
#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;
#[cfg(feature = "rkyv-serialize-no-std")]
use rkyv::{Archive, Archived, with::With};

use simba::scalar::{ClosedAddAssign, ClosedMulAssign, ClosedSubAssign, Field, SupersetOf};
use simba::simd::SimdPartialOrd;

use crate::base::allocator::{Allocator, SameShapeAllocator, SameShapeC, SameShapeR};
use crate::base::constraint::{DimEq, SameNumberOfColumns, SameNumberOfRows, ShapeConstraint};
use crate::base::dimension::{Dim, DimAdd, DimSum, IsNotStaticOne, U1, U2, U3};
use crate::base::iter::{
    ColumnIter, ColumnIterMut, MatrixIter, MatrixIterMut, RowIter, RowIterMut,
};
use crate::base::storage::{Owned, RawStorage, RawStorageMut, SameShapeStorage};
use crate::base::{Const, DefaultAllocator, OMatrix, OVector, Scalar, Unit};
use crate::{ArrayStorage, SMatrix, SimdComplexField, Storage, UninitMatrix};

use crate::storage::IsContiguous;
use crate::uninit::{Init, InitStatus, Uninit};
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::{DMatrix, DVector, Dyn, RowDVector, VecStorage};
use std::mem::MaybeUninit;

/// A square matrix.
pub type SquareMatrix<T, D, S> = Matrix<T, D, D, S>;

/// A matrix with one column and `D` rows.
pub type Vector<T, D, S> = Matrix<T, D, U1, S>;

/// A matrix with one row and `D` columns .
pub type RowVector<T, D, S> = Matrix<T, U1, D, S>;

/// The type of the result of a matrix sum.
pub type MatrixSum<T, R1, C1, R2, C2> =
    Matrix<T, SameShapeR<R1, R2>, SameShapeC<C1, C2>, SameShapeStorage<T, R1, C1, R2, C2>>;

/// The type of the result of a matrix sum.
pub type VectorSum<T, R1, R2> =
    Matrix<T, SameShapeR<R1, R2>, U1, SameShapeStorage<T, R1, U1, R2, U1>>;

/// The type of the result of a matrix cross product.
pub type MatrixCross<T, R1, C1, R2, C2> =
    Matrix<T, SameShapeR<R1, R2>, SameShapeC<C1, C2>, SameShapeStorage<T, R1, C1, R2, C2>>;

/// The most generic column-major matrix (and vector) type.
///
/// # Methods summary
/// Because `Matrix` is the most generic types used as a common representation of all matrices and
/// vectors of **nalgebra** this documentation page contains every single matrix/vector-related
/// method. In order to make browsing this page simpler, the next subsections contain direct links
/// to groups of methods related to a specific topic.
///
/// #### Vector and matrix construction
/// - [Constructors of statically-sized vectors or statically-sized matrices](#constructors-of-statically-sized-vectors-or-statically-sized-matrices)
///   (`Vector3`, `Matrix3x6`…)
/// - [Constructors of fully dynamic matrices](#constructors-of-fully-dynamic-matrices) (`DMatrix`)
/// - [Constructors of dynamic vectors and matrices with a dynamic number of rows](#constructors-of-dynamic-vectors-and-matrices-with-a-dynamic-number-of-rows)
///   (`DVector`, `MatrixXx3`…)
/// - [Constructors of matrices with a dynamic number of columns](#constructors-of-matrices-with-a-dynamic-number-of-columns)
///   (`Matrix2xX`…)
/// - [Generic constructors](#generic-constructors)
///   (For code generic wrt. the vectors or matrices dimensions.)
///
/// #### Computer graphics utilities for transformations
/// - [2D transformations as a Matrix3 <span style="float:right;">`new_rotation`…</span>](#2d-transformations-as-a-matrix3)
/// - [3D transformations as a Matrix4 <span style="float:right;">`new_rotation`, `new_perspective`, `look_at_rh`…</span>](#3d-transformations-as-a-matrix4)
/// - [Translation and scaling in any dimension <span style="float:right;">`new_scaling`, `new_translation`…</span>](#translation-and-scaling-in-any-dimension)
/// - [Append/prepend translation and scaling <span style="float:right;">`append_scaling`, `prepend_translation_mut`…</span>](#appendprepend-translation-and-scaling)
/// - [Transformation of vectors and points <span style="float:right;">`transform_vector`, `transform_point`…</span>](#transformation-of-vectors-and-points)
///
/// #### Common math operations
/// - [Componentwise operations <span style="float:right;">`component_mul`, `component_div`, `inf`…</span>](#componentwise-operations)
/// - [Special multiplications <span style="float:right;">`tr_mul`, `ad_mul`, `kronecker`…</span>](#special-multiplications)
/// - [Dot/scalar product <span style="float:right;">`dot`, `dotc`, `tr_dot`…</span>](#dotscalar-product)
/// - [Cross product <span style="float:right;">`cross`, `perp`…</span>](#cross-product)
/// - [Magnitude and norms <span style="float:right;">`norm`, `normalize`, `metric_distance`…</span>](#magnitude-and-norms)
/// - [In-place normalization <span style="float:right;">`normalize_mut`, `try_normalize_mut`…</span>](#in-place-normalization)
/// - [Interpolation <span style="float:right;">`lerp`, `slerp`…</span>](#interpolation)
/// - [BLAS functions <span style="float:right;">`gemv`, `gemm`, `syger`…</span>](#blas-functions)
/// - [Swizzling <span style="float:right;">`xx`, `yxz`…</span>](#swizzling)
/// - [Triangular matrix extraction <span style="float:right;">`upper_triangle`, `lower_triangle`</span>](#triangular-matrix-extraction)
///
/// #### Statistics
/// - [Common operations <span style="float:right;">`row_sum`, `column_mean`, `variance`…</span>](#common-statistics-operations)
/// - [Find the min and max components <span style="float:right;">`min`, `max`, `amin`, `amax`, `camin`, `cmax`…</span>](#find-the-min-and-max-components)
/// - [Find the min and max components (vector-specific methods) <span style="float:right;">`argmin`, `argmax`, `icamin`, `icamax`…</span>](#find-the-min-and-max-components-vector-specific-methods)
///
/// #### Iteration, map, and fold
/// - [Iteration on components, rows, and columns <span style="float:right;">`iter`, `column_iter`…</span>](#iteration-on-components-rows-and-columns)
/// - [Parallel iterators using rayon <span style="float:right;">`par_column_iter`, `par_column_iter_mut`…</span>](#parallel-iterators-using-rayon)
/// - [Elementwise mapping and folding <span style="float:right;">`map`, `fold`, `zip_map`…</span>](#elementwise-mapping-and-folding)
/// - [Folding or columns and rows <span style="float:right;">`compress_rows`, `compress_columns`…</span>](#folding-on-columns-and-rows)
///
/// #### Vector and matrix views
/// - [Creating matrix views from `&[T]` <span style="float:right;">`from_slice`, `from_slice_with_strides`…</span>](#creating-matrix-views-from-t)
/// - [Creating mutable matrix views from `&mut [T]` <span style="float:right;">`from_slice_mut`, `from_slice_with_strides_mut`…</span>](#creating-mutable-matrix-views-from-mut-t)
/// - [Views based on index and length <span style="float:right;">`row`, `columns`, `view`…</span>](#views-based-on-index-and-length)
/// - [Mutable views based on index and length <span style="float:right;">`row_mut`, `columns_mut`, `view_mut`…</span>](#mutable-views-based-on-index-and-length)
/// - [Views based on ranges <span style="float:right;">`rows_range`, `columns_range`…</span>](#views-based-on-ranges)
/// - [Mutable views based on ranges <span style="float:right;">`rows_range_mut`, `columns_range_mut`…</span>](#mutable-views-based-on-ranges)
///
/// #### In-place modification of a single matrix or vector
/// - [In-place filling <span style="float:right;">`fill`, `fill_diagonal`, `fill_with_identity`…</span>](#in-place-filling)
/// - [In-place swapping <span style="float:right;">`swap`, `swap_columns`…</span>](#in-place-swapping)
/// - [Set rows, columns, and diagonal <span style="float:right;">`set_column`, `set_diagonal`…</span>](#set-rows-columns-and-diagonal)
///
/// #### Vector and matrix size modification
/// - [Rows and columns insertion <span style="float:right;">`insert_row`, `insert_column`…</span>](#rows-and-columns-insertion)
/// - [Rows and columns removal <span style="float:right;">`remove_row`, `remove column`…</span>](#rows-and-columns-removal)
/// - [Rows and columns extraction <span style="float:right;">`select_rows`, `select_columns`…</span>](#rows-and-columns-extraction)
/// - [Resizing and reshaping <span style="float:right;">`resize`, `reshape_generic`…</span>](#resizing-and-reshaping)
/// - [In-place resizing <span style="float:right;">`resize_mut`, `resize_vertically_mut`…</span>](#in-place-resizing)
///
/// #### Matrix decomposition
/// - [Rectangular matrix decomposition <span style="float:right;">`qr`, `lu`, `svd`…</span>](#rectangular-matrix-decomposition)
/// - [Square matrix decomposition <span style="float:right;">`cholesky`, `symmetric_eigen`…</span>](#square-matrix-decomposition)
///
/// #### Vector basis computation
/// - [Basis and orthogonalization <span style="float:right;">`orthonormal_subspace_basis`, `orthonormalize`…</span>](#basis-and-orthogonalization)
///
/// # Type parameters
/// The generic `Matrix` type has four type parameters:
/// - `T`: for the matrix components scalar type.
/// - `R`: for the matrix number of rows.
/// - `C`: for the matrix number of columns.
/// - `S`: for the matrix data storage, i.e., the buffer that actually contains the matrix
///   components.
///
/// The matrix dimensions parameters `R` and `C` can either be:
/// - type-level unsigned integer constants (e.g. `U1`, `U124`) from the `nalgebra::` root module.
///   All numbers from 0 to 127 are defined that way.
/// - type-level unsigned integer constants (e.g. `U1024`, `U10000`) from the `typenum::` crate.
///   Using those, you will not get error messages as nice as for numbers smaller than 128 defined on
///   the `nalgebra::` module.
/// - the special value `Dyn` from the `nalgebra::` root module. This indicates that the
///   specified dimension is not known at compile-time. Note that this will generally imply that the
///   matrix data storage `S` performs a dynamic allocation and contains extra metadata for the
///   matrix shape.
///
/// Note that mixing `Dyn` with type-level unsigned integers is allowed. Actually, a
/// dynamically-sized column vector should be represented as a `Matrix<T, Dyn, U1, S>` (given
/// some concrete types for `T` and a compatible data storage type `S`).
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(
        as = "Matrix<T::Archived, R, C, S::Archived>",
        bound(archive = "
        T: Archive,
        S: Archive,
        With<PhantomData<(T, R, C)>, CustomPhantom<(Archived<T>, R, C)>>: Archive<Archived = PhantomData<(Archived<T>, R, C)>>
    ")
    )
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Matrix<T, R, C, S> {
    /// The data storage that contains all the matrix components. Disappointed?
    ///
    /// Well, if you came here to see how you can access the matrix components,
    /// you may be in luck: you can access the individual components of all vectors with compile-time
    /// dimensions <= 6 using field notation like this:
    /// `vec.x`, `vec.y`, `vec.z`, `vec.w`, `vec.a`, `vec.b`. Reference and assignation work too:
    /// ```
    /// # use nalgebra::Vector3;
    /// let mut vec = Vector3::new(1.0, 2.0, 3.0);
    /// vec.x = 10.0;
    /// vec.y += 30.0;
    /// assert_eq!(vec.x, 10.0);
    /// assert_eq!(vec.y + 100.0, 132.0);
    /// ```
    /// Similarly, for matrices with compile-time dimensions <= 6, you can use field notation
    /// like this: `mat.m11`, `mat.m42`, etc. The first digit identifies the row to address
    /// and the second digit identifies the column to address. So `mat.m13` identifies the component
    /// at the first row and third column (note that the count of rows and columns start at 1 instead
    /// of 0 here. This is so we match the mathematical notation).
    ///
    /// For all matrices and vectors, independently from their size, individual components can
    /// be accessed and modified using indexing: `vec[20]`, `mat[(20, 19)]`. Here the indexing
    /// starts at 0 as you would expect.
    pub data: S,

    // NOTE: the fact that this field is private is important because
    //       this prevents the user from constructing a matrix with
    //       dimensions R, C that don't match the dimension of the
    //       storage S. Instead they have to use the unsafe function
    //       from_data_statically_unchecked.
    //       Note that it would probably make sense to just have
    //       the type `Matrix<S>`, and have `T, R, C` be associated-types
    //       of the `RawStorage` trait. However, because we don't have
    //       specialization, this is not possible because these `T, R, C`
    //       allows us to desambiguate a lot of configurations.
    #[cfg_attr(feature = "rkyv-serialize-no-std", with(CustomPhantom<(T::Archived, R, C)>))]
    _phantoms: PhantomData<(T, R, C)>,
}

impl<T, R: Dim, C: Dim, S: fmt::Debug> fmt::Debug for Matrix<T, R, C, S> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.data.fmt(formatter)
    }
}

impl<T, R, C, S> Default for Matrix<T, R, C, S>
where
    T: Scalar,
    R: Dim,
    C: Dim,
    S: Default,
{
    fn default() -> Self {
        Matrix {
            data: Default::default(),
            _phantoms: PhantomData,
        }
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<T, R, C, S> Serialize for Matrix<T, R, C, S>
where
    T: Scalar,
    R: Dim,
    C: Dim,
    S: Serialize,
{
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        self.data.serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'de, T, R, C, S> Deserialize<'de> for Matrix<T, R, C, S>
where
    T: Scalar,
    R: Dim,
    C: Dim,
    S: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        S::deserialize(deserializer).map(|x| Matrix {
            data: x,
            _phantoms: PhantomData,
        })
    }
}

#[cfg(feature = "compare")]
impl<T: Scalar, R: Dim, C: Dim, S: RawStorage<T, R, C>> matrixcompare_core::Matrix<T>
    for Matrix<T, R, C, S>
{
    fn rows(&self) -> usize {
        self.nrows()
    }

    fn cols(&self) -> usize {
        self.ncols()
    }

    fn access(&self) -> matrixcompare_core::Access<'_, T> {
        matrixcompare_core::Access::Dense(self)
    }
}

#[cfg(feature = "compare")]
impl<T: Scalar, R: Dim, C: Dim, S: RawStorage<T, R, C>> matrixcompare_core::DenseAccess<T>
    for Matrix<T, R, C, S>
{
    fn fetch_single(&self, row: usize, col: usize) -> T {
        self.index((row, col)).clone()
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Scalar, R: Dim, C: Dim, S: RawStorage<T, R, C>> bytemuck::Zeroable
    for Matrix<T, R, C, S>
where
    S: bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Scalar, R: Dim, C: Dim, S: RawStorage<T, R, C>> bytemuck::Pod for Matrix<T, R, C, S>
where
    S: bytemuck::Pod,
    Self: Copy,
{
}

impl<T, R, C, S> Matrix<T, R, C, S> {
    /// Creates a new matrix with the given data without statically checking that the matrix
    /// dimension matches the storage dimension.
    ///
    /// # Safety
    ///
    /// The storage dimension must match the given dimensions.
    #[inline(always)]
    pub const unsafe fn from_data_statically_unchecked(data: S) -> Matrix<T, R, C, S> {
        Matrix {
            data,
            _phantoms: PhantomData,
        }
    }
}

impl<T, const R: usize, const C: usize> SMatrix<T, R, C> {
    /// Creates a new statically-allocated matrix from the given [`ArrayStorage`].
    ///
    /// This method exists primarily as a workaround for the fact that `from_data` can not
    /// work in `const fn` contexts.
    #[inline(always)]
    pub const fn from_array_storage(storage: ArrayStorage<T, R, C>) -> Self {
        // This is sound because the row and column types are exactly the same as that of the
        // storage, so there can be no mismatch
        unsafe { Self::from_data_statically_unchecked(storage) }
    }
}

// TODO: Consider removing/deprecating `from_vec_storage` once we are able to make
// `from_data` const fn compatible
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> DMatrix<T> {
    /// Creates a new heap-allocated matrix from the given [`VecStorage`].
    ///
    /// This method exists primarily as a workaround for the fact that `from_data` can not
    /// work in `const fn` contexts.
    pub const fn from_vec_storage(storage: VecStorage<T, Dyn, Dyn>) -> Self {
        // This is sound because the dimensions of the matrix and the storage are guaranteed
        // to be the same
        unsafe { Self::from_data_statically_unchecked(storage) }
    }
}

// TODO: Consider removing/deprecating `from_vec_storage` once we are able to make
// `from_data` const fn compatible
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> DVector<T> {
    /// Creates a new heap-allocated matrix from the given [`VecStorage`].
    ///
    /// This method exists primarily as a workaround for the fact that `from_data` can not
    /// work in `const fn` contexts.
    pub const fn from_vec_storage(storage: VecStorage<T, Dyn, U1>) -> Self {
        // This is sound because the dimensions of the matrix and the storage are guaranteed
        // to be the same
        unsafe { Self::from_data_statically_unchecked(storage) }
    }
}

// TODO: Consider removing/deprecating `from_vec_storage` once we are able to make
// `from_data` const fn compatible
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> RowDVector<T> {
    /// Creates a new heap-allocated matrix from the given [`VecStorage`].
    ///
    /// This method exists primarily as a workaround for the fact that `from_data` can not
    /// work in `const fn` contexts.
    pub const fn from_vec_storage(storage: VecStorage<T, U1, Dyn>) -> Self {
        // This is sound because the dimensions of the matrix and the storage are guaranteed
        // to be the same
        unsafe { Self::from_data_statically_unchecked(storage) }
    }
}

impl<T: Scalar, R: Dim, C: Dim> UninitMatrix<T, R, C>
where
    DefaultAllocator: Allocator<R, C>,
{
    /// Assumes a matrix's entries to be initialized. This operation should be near zero-cost.
    ///
    /// # Safety
    /// The user must make sure that every single entry of the buffer has been initialized,
    /// or Undefined Behavior will immediately occur.
    #[inline(always)]
    pub unsafe fn assume_init(self) -> OMatrix<T, R, C> {
        unsafe {
            OMatrix::from_data(<DefaultAllocator as Allocator<R, C>>::assume_init(
                self.data,
            ))
        }
    }
}

impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S> {
    /// Creates a new matrix with the given data.
    #[inline(always)]
    pub const fn from_data(data: S) -> Self {
        unsafe { Self::from_data_statically_unchecked(data) }
    }

    /// The shape of this matrix returned as the tuple (number of rows, number of columns).
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix3x4;
    /// let mat = Matrix3x4::<f32>::zeros();
    /// assert_eq!(mat.shape(), (3, 4));
    /// ```
    #[inline]
    #[must_use]
    pub fn shape(&self) -> (usize, usize) {
        let (nrows, ncols) = self.shape_generic();
        (nrows.value(), ncols.value())
    }

    /// The shape of this matrix wrapped into their representative types (`Const` or `Dyn`).
    #[inline]
    #[must_use]
    pub fn shape_generic(&self) -> (R, C) {
        self.data.shape()
    }

    /// The number of rows of this matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix3x4;
    /// let mat = Matrix3x4::<f32>::zeros();
    /// assert_eq!(mat.nrows(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn nrows(&self) -> usize {
        self.shape().0
    }

    /// The number of columns of this matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix3x4;
    /// let mat = Matrix3x4::<f32>::zeros();
    /// assert_eq!(mat.ncols(), 4);
    /// ```
    #[inline]
    #[must_use]
    pub fn ncols(&self) -> usize {
        self.shape().1
    }

    /// The strides (row stride, column stride) of this matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::DMatrix;
    /// let mat = DMatrix::<f32>::zeros(10, 10);
    /// let view = mat.view_with_steps((0, 0), (5, 3), (1, 2));
    /// // The column strides is the number of steps (here 2) multiplied by the corresponding dimension.
    /// assert_eq!(mat.strides(), (1, 10));
    /// ```
    #[inline]
    #[must_use]
    pub fn strides(&self) -> (usize, usize) {
        let (srows, scols) = self.data.strides();
        (srows.value(), scols.value())
    }

    /// Computes the row and column coordinates of the i-th element of this matrix seen as a
    /// vector.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix2;
    /// let m = Matrix2::new(1, 2,
    ///                      3, 4);
    /// let i = m.vector_to_matrix_index(3);
    /// assert_eq!(i, (1, 1));
    /// assert_eq!(m[i], m[3]);
    /// ```
    #[inline]
    #[must_use]
    pub fn vector_to_matrix_index(&self, i: usize) -> (usize, usize) {
        let (nrows, ncols) = self.shape();

        // Two most common uses that should be optimized by the compiler for statically-sized
        // matrices.
        if nrows == 1 {
            (0, i)
        } else if ncols == 1 {
            (i, 0)
        } else {
            (i % nrows, i / nrows)
        }
    }

    /// Returns a pointer to the start of the matrix.
    ///
    /// If the matrix is not empty, this pointer is guaranteed to be aligned
    /// and non-null.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix2;
    /// let m = Matrix2::new(1, 2,
    ///                      3, 4);
    /// let ptr = m.as_ptr();
    /// assert_eq!(unsafe { *ptr }, m[0]);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        self.data.ptr()
    }

    /// Tests whether `self` and `rhs` are equal up to a given epsilon.
    ///
    /// See `relative_eq` from the `RelativeEq` trait for more details.
    #[inline]
    #[must_use]
    pub fn relative_eq<R2, C2, SB>(
        &self,
        other: &Matrix<T, R2, C2, SB>,
        eps: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool
    where
        T: RelativeEq + Scalar,
        R2: Dim,
        C2: Dim,
        SB: Storage<T, R2, C2>,
        T::Epsilon: Clone,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        assert!(self.shape() == other.shape());
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.relative_eq(b, eps.clone(), max_relative.clone()))
    }

    /// Tests whether `self` and `rhs` are exactly equal.
    #[inline]
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn eq<R2, C2, SB>(&self, other: &Matrix<T, R2, C2, SB>) -> bool
    where
        T: PartialEq,
        R2: Dim,
        C2: Dim,
        SB: RawStorage<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        assert!(self.shape() == other.shape());
        self.iter().zip(other.iter()).all(|(a, b)| *a == *b)
    }

    /// Moves this matrix into one that owns its data.
    #[inline]
    pub fn into_owned(self) -> OMatrix<T, R, C>
    where
        T: Scalar,
        S: Storage<T, R, C>,
        DefaultAllocator: Allocator<R, C>,
    {
        Matrix::from_data(self.data.into_owned())
    }

    // TODO: this could probably benefit from specialization.
    // XXX: bad name.
    /// Moves this matrix into one that owns its data. The actual type of the result depends on
    /// matrix storage combination rules for addition.
    #[inline]
    pub fn into_owned_sum<R2, C2>(self) -> MatrixSum<T, R, C, R2, C2>
    where
        T: Scalar,
        S: Storage<T, R, C>,
        R2: Dim,
        C2: Dim,
        DefaultAllocator: SameShapeAllocator<R, C, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        if TypeId::of::<SameShapeStorage<T, R, C, R2, C2>>() == TypeId::of::<Owned<T, R, C>>() {
            // We can just return `self.into_owned()`.

            unsafe {
                // TODO: check that those copies are optimized away by the compiler.
                let owned = self.into_owned();
                let res = mem::transmute_copy(&owned);
                mem::forget(owned);
                res
            }
        } else {
            self.clone_owned_sum()
        }
    }

    /// Clones this matrix to one that owns its data.
    #[inline]
    #[must_use]
    pub fn clone_owned(&self) -> OMatrix<T, R, C>
    where
        T: Scalar,
        S: Storage<T, R, C>,
        DefaultAllocator: Allocator<R, C>,
    {
        Matrix::from_data(self.data.clone_owned())
    }

    /// Clones this matrix into one that owns its data. The actual type of the result depends on
    /// matrix storage combination rules for addition.
    #[inline]
    #[must_use]
    pub fn clone_owned_sum<R2, C2>(&self) -> MatrixSum<T, R, C, R2, C2>
    where
        T: Scalar,
        S: Storage<T, R, C>,
        R2: Dim,
        C2: Dim,
        DefaultAllocator: SameShapeAllocator<R, C, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        let (nrows, ncols) = self.shape();
        let nrows: SameShapeR<R, R2> = Dim::from_usize(nrows);
        let ncols: SameShapeC<C, C2> = Dim::from_usize(ncols);

        let mut res = Matrix::uninit(nrows, ncols);

        unsafe {
            // TODO: use copy_from?
            for j in 0..res.ncols() {
                for i in 0..res.nrows() {
                    *res.get_unchecked_mut((i, j)) =
                        MaybeUninit::new(self.get_unchecked((i, j)).clone());
                }
            }

            // SAFETY: the output has been initialized above.
            res.assume_init()
        }
    }

    /// Transposes `self` and store the result into `out`.
    #[inline]
    fn transpose_to_uninit<Status, R2, C2, SB>(
        &self,
        _status: Status,
        out: &mut Matrix<Status::Value, R2, C2, SB>,
    ) where
        Status: InitStatus<T>,
        T: Scalar,
        R2: Dim,
        C2: Dim,
        SB: RawStorageMut<Status::Value, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, C2> + SameNumberOfColumns<C, R2>,
    {
        let (nrows, ncols) = self.shape();
        assert!(
            (ncols, nrows) == out.shape(),
            "Incompatible shape for transposition."
        );

        // TODO: optimize that.
        for i in 0..nrows {
            for j in 0..ncols {
                // Safety: the indices are in range.
                unsafe {
                    Status::init(
                        out.get_unchecked_mut((j, i)),
                        self.get_unchecked((i, j)).clone(),
                    );
                }
            }
        }
    }

    /// Transposes `self` and store the result into `out`.
    #[inline]
    pub fn transpose_to<R2, C2, SB>(&self, out: &mut Matrix<T, R2, C2, SB>)
    where
        T: Scalar,
        R2: Dim,
        C2: Dim,
        SB: RawStorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, C2> + SameNumberOfColumns<C, R2>,
    {
        self.transpose_to_uninit(Init, out)
    }

    /// Transposes `self`.
    #[inline]
    #[must_use = "Did you mean to use transpose_mut()?"]
    pub fn transpose(&self) -> OMatrix<T, C, R>
    where
        T: Scalar,
        DefaultAllocator: Allocator<C, R>,
    {
        let (nrows, ncols) = self.shape_generic();

        let mut res = Matrix::uninit(ncols, nrows);
        self.transpose_to_uninit(Uninit, &mut res);
        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }
}

/// # Elementwise mapping and folding
impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S> {
    /// Returns a matrix containing the result of `f` applied to each of its entries.
    #[inline]
    #[must_use]
    pub fn map<T2: Scalar, F: FnMut(T) -> T2>(&self, mut f: F) -> OMatrix<T2, R, C>
    where
        T: Scalar,
        DefaultAllocator: Allocator<R, C>,
    {
        let (nrows, ncols) = self.shape_generic();
        let mut res = Matrix::uninit(nrows, ncols);

        for j in 0..ncols.value() {
            for i in 0..nrows.value() {
                // Safety: all indices are in range.
                unsafe {
                    let a = self.data.get_unchecked(i, j).clone();
                    *res.data.get_unchecked_mut(i, j) = MaybeUninit::new(f(a));
                }
            }
        }

        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// let q = Vector3::new(1.0f64, 2.0, 3.0);
    /// let q2 = q.cast::<f32>();
    /// assert_eq!(q2, Vector3::new(1.0f32, 2.0, 3.0));
    /// ```
    pub fn cast<T2: Scalar>(self) -> OMatrix<T2, R, C>
    where
        T: Scalar,
        OMatrix<T2, R, C>: SupersetOf<Self>,
        DefaultAllocator: Allocator<R, C>,
    {
        crate::convert(self)
    }

    /// Attempts to cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// let q = Vector3::new(1.0f64, 2.0, 3.0);
    /// let q2 = q.try_cast::<i32>();
    /// assert_eq!(q2, Some(Vector3::new(1, 2, 3)));
    /// ```
    pub fn try_cast<T2: Scalar>(self) -> Option<OMatrix<T2, R, C>>
    where
        T: Scalar,
        Self: SupersetOf<OMatrix<T2, R, C>>,
        DefaultAllocator: Allocator<R, C>,
    {
        crate::try_convert(self)
    }

    /// Similar to `self.iter().fold(init, f)` except that `init` is replaced by a closure.
    ///
    /// The initialization closure is given the first component of this matrix:
    /// - If the matrix has no component (0 rows or 0 columns) then `init_f` is called with `None`
    ///   and its return value is the value returned by this method.
    /// - If the matrix has has least one component, then `init_f` is called with the first component
    ///   to compute the initial value. Folding then continues on all the remaining components of the matrix.
    #[inline]
    #[must_use]
    pub fn fold_with<T2>(
        &self,
        init_f: impl FnOnce(Option<&T>) -> T2,
        f: impl FnMut(T2, &T) -> T2,
    ) -> T2
    where
        T: Scalar,
    {
        let mut it = self.iter();
        let init = init_f(it.next());
        it.fold(init, f)
    }

    /// Returns a matrix containing the result of `f` applied to each of its entries. Unlike `map`,
    /// `f` also gets passed the row and column index, i.e. `f(row, col, value)`.
    #[inline]
    #[must_use]
    pub fn map_with_location<T2: Scalar, F: FnMut(usize, usize, T) -> T2>(
        &self,
        mut f: F,
    ) -> OMatrix<T2, R, C>
    where
        T: Scalar,
        DefaultAllocator: Allocator<R, C>,
    {
        let (nrows, ncols) = self.shape_generic();
        let mut res = Matrix::uninit(nrows, ncols);

        for j in 0..ncols.value() {
            for i in 0..nrows.value() {
                // Safety: all indices are in range.
                unsafe {
                    let a = self.data.get_unchecked(i, j).clone();
                    *res.data.get_unchecked_mut(i, j) = MaybeUninit::new(f(i, j, a));
                }
            }
        }

        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }

    /// Returns a matrix containing the result of `f` applied to each entries of `self` and
    /// `rhs`.
    #[inline]
    #[must_use]
    pub fn zip_map<T2, N3, S2, F>(&self, rhs: &Matrix<T2, R, C, S2>, mut f: F) -> OMatrix<N3, R, C>
    where
        T: Scalar,
        T2: Scalar,
        N3: Scalar,
        S2: RawStorage<T2, R, C>,
        F: FnMut(T, T2) -> N3,
        DefaultAllocator: Allocator<R, C>,
    {
        let (nrows, ncols) = self.shape_generic();
        let mut res = Matrix::uninit(nrows, ncols);

        assert_eq!(
            (nrows.value(), ncols.value()),
            rhs.shape(),
            "Matrix simultaneous traversal error: dimension mismatch."
        );

        for j in 0..ncols.value() {
            for i in 0..nrows.value() {
                // Safety: all indices are in range.
                unsafe {
                    let a = self.data.get_unchecked(i, j).clone();
                    let b = rhs.data.get_unchecked(i, j).clone();
                    *res.data.get_unchecked_mut(i, j) = MaybeUninit::new(f(a, b))
                }
            }
        }

        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }

    /// Returns a matrix containing the result of `f` applied to each entries of `self` and
    /// `b`, and `c`.
    #[inline]
    #[must_use]
    pub fn zip_zip_map<T2, N3, N4, S2, S3, F>(
        &self,
        b: &Matrix<T2, R, C, S2>,
        c: &Matrix<N3, R, C, S3>,
        mut f: F,
    ) -> OMatrix<N4, R, C>
    where
        T: Scalar,
        T2: Scalar,
        N3: Scalar,
        N4: Scalar,
        S2: RawStorage<T2, R, C>,
        S3: RawStorage<N3, R, C>,
        F: FnMut(T, T2, N3) -> N4,
        DefaultAllocator: Allocator<R, C>,
    {
        let (nrows, ncols) = self.shape_generic();
        let mut res = Matrix::uninit(nrows, ncols);

        assert_eq!(
            (nrows.value(), ncols.value()),
            b.shape(),
            "Matrix simultaneous traversal error: dimension mismatch."
        );
        assert_eq!(
            (nrows.value(), ncols.value()),
            c.shape(),
            "Matrix simultaneous traversal error: dimension mismatch."
        );

        for j in 0..ncols.value() {
            for i in 0..nrows.value() {
                // Safety: all indices are in range.
                unsafe {
                    let a = self.data.get_unchecked(i, j).clone();
                    let b = b.data.get_unchecked(i, j).clone();
                    let c = c.data.get_unchecked(i, j).clone();
                    *res.data.get_unchecked_mut(i, j) = MaybeUninit::new(f(a, b, c))
                }
            }
        }

        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }

    /// Folds a function `f` on each entry of `self`.
    #[inline]
    #[must_use]
    pub fn fold<Acc>(&self, init: Acc, mut f: impl FnMut(Acc, T) -> Acc) -> Acc
    where
        T: Scalar,
    {
        let (nrows, ncols) = self.shape_generic();

        let mut res = init;

        for j in 0..ncols.value() {
            for i in 0..nrows.value() {
                // Safety: all indices are in range.
                unsafe {
                    let a = self.data.get_unchecked(i, j).clone();
                    res = f(res, a)
                }
            }
        }

        res
    }

    /// Folds a function `f` on each pairs of entries from `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn zip_fold<T2, R2, C2, S2, Acc>(
        &self,
        rhs: &Matrix<T2, R2, C2, S2>,
        init: Acc,
        mut f: impl FnMut(Acc, T, T2) -> Acc,
    ) -> Acc
    where
        T: Scalar,
        T2: Scalar,
        R2: Dim,
        C2: Dim,
        S2: RawStorage<T2, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        let (nrows, ncols) = self.shape_generic();

        let mut res = init;

        assert_eq!(
            (nrows.value(), ncols.value()),
            rhs.shape(),
            "Matrix simultaneous traversal error: dimension mismatch."
        );

        for j in 0..ncols.value() {
            for i in 0..nrows.value() {
                unsafe {
                    let a = self.data.get_unchecked(i, j).clone();
                    let b = rhs.data.get_unchecked(i, j).clone();
                    res = f(res, a, b)
                }
            }
        }

        res
    }

    /// Applies a closure `f` to modify each component of `self`.
    #[inline]
    pub fn apply<F: FnMut(&mut T)>(&mut self, mut f: F)
    where
        S: RawStorageMut<T, R, C>,
    {
        let (nrows, ncols) = self.shape();

        for j in 0..ncols {
            for i in 0..nrows {
                unsafe {
                    let e = self.data.get_unchecked_mut(i, j);
                    f(e)
                }
            }
        }
    }

    /// Replaces each component of `self` by the result of a closure `f` applied on its components
    /// joined with the components from `rhs`.
    #[inline]
    pub fn zip_apply<T2, R2, C2, S2>(
        &mut self,
        rhs: &Matrix<T2, R2, C2, S2>,
        mut f: impl FnMut(&mut T, T2),
    ) where
        S: RawStorageMut<T, R, C>,
        T2: Scalar,
        R2: Dim,
        C2: Dim,
        S2: RawStorage<T2, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        let (nrows, ncols) = self.shape();

        assert_eq!(
            (nrows, ncols),
            rhs.shape(),
            "Matrix simultaneous traversal error: dimension mismatch."
        );

        for j in 0..ncols {
            for i in 0..nrows {
                unsafe {
                    let e = self.data.get_unchecked_mut(i, j);
                    let rhs = rhs.get_unchecked((i, j)).clone();
                    f(e, rhs)
                }
            }
        }
    }

    /// Replaces each component of `self` by the result of a closure `f` applied on its components
    /// joined with the components from `b` and `c`.
    #[inline]
    pub fn zip_zip_apply<T2, R2, C2, S2, N3, R3, C3, S3>(
        &mut self,
        b: &Matrix<T2, R2, C2, S2>,
        c: &Matrix<N3, R3, C3, S3>,
        mut f: impl FnMut(&mut T, T2, N3),
    ) where
        S: RawStorageMut<T, R, C>,
        T2: Scalar,
        R2: Dim,
        C2: Dim,
        S2: RawStorage<T2, R2, C2>,
        N3: Scalar,
        R3: Dim,
        C3: Dim,
        S3: RawStorage<N3, R3, C3>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        let (nrows, ncols) = self.shape();

        assert_eq!(
            (nrows, ncols),
            b.shape(),
            "Matrix simultaneous traversal error: dimension mismatch."
        );
        assert_eq!(
            (nrows, ncols),
            c.shape(),
            "Matrix simultaneous traversal error: dimension mismatch."
        );

        for j in 0..ncols {
            for i in 0..nrows {
                unsafe {
                    let e = self.data.get_unchecked_mut(i, j);
                    let b = b.get_unchecked((i, j)).clone();
                    let c = c.get_unchecked((i, j)).clone();
                    f(e, b, c)
                }
            }
        }
    }
}

/// # Iteration on components, rows, and columns
impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S> {
    /// Iterates through this matrix coordinates in column-major order.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix2x3;
    /// let mat = Matrix2x3::new(11, 12, 13,
    ///                          21, 22, 23);
    /// let mut it = mat.iter();
    /// assert_eq!(*it.next().unwrap(), 11);
    /// assert_eq!(*it.next().unwrap(), 21);
    /// assert_eq!(*it.next().unwrap(), 12);
    /// assert_eq!(*it.next().unwrap(), 22);
    /// assert_eq!(*it.next().unwrap(), 13);
    /// assert_eq!(*it.next().unwrap(), 23);
    /// assert!(it.next().is_none());
    /// ```
    #[inline]
    pub fn iter(&self) -> MatrixIter<'_, T, R, C, S> {
        MatrixIter::new(&self.data)
    }

    /// Iterate through the rows of this matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix2x3;
    /// let mut a = Matrix2x3::new(1, 2, 3,
    ///                            4, 5, 6);
    /// for (i, row) in a.row_iter().enumerate() {
    ///     assert_eq!(row, a.row(i))
    /// }
    /// ```
    #[inline]
    pub const fn row_iter(&self) -> RowIter<'_, T, R, C, S> {
        RowIter::new(self)
    }

    /// Iterate through the columns of this matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix2x3;
    /// let mut a = Matrix2x3::new(1, 2, 3,
    ///                            4, 5, 6);
    /// for (i, column) in a.column_iter().enumerate() {
    ///     assert_eq!(column, a.column(i))
    /// }
    /// ```
    #[inline]
    pub fn column_iter(&self) -> ColumnIter<'_, T, R, C, S> {
        ColumnIter::new(self)
    }

    /// Mutably iterates through this matrix coordinates.
    #[inline]
    pub fn iter_mut(&mut self) -> MatrixIterMut<'_, T, R, C, S>
    where
        S: RawStorageMut<T, R, C>,
    {
        MatrixIterMut::new(&mut self.data)
    }

    /// Mutably iterates through this matrix rows.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix2x3;
    /// let mut a = Matrix2x3::new(1, 2, 3,
    ///                            4, 5, 6);
    /// for (i, mut row) in a.row_iter_mut().enumerate() {
    ///     row *= (i + 1) * 10;
    /// }
    ///
    /// let expected = Matrix2x3::new(10, 20, 30,
    ///                               80, 100, 120);
    /// assert_eq!(a, expected);
    /// ```
    #[inline]
    pub const fn row_iter_mut(&mut self) -> RowIterMut<'_, T, R, C, S>
    where
        S: RawStorageMut<T, R, C>,
    {
        RowIterMut::new(self)
    }

    /// Mutably iterates through this matrix columns.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Matrix2x3;
    /// let mut a = Matrix2x3::new(1, 2, 3,
    ///                            4, 5, 6);
    /// for (i, mut col) in a.column_iter_mut().enumerate() {
    ///     col *= (i + 1) * 10;
    /// }
    ///
    /// let expected = Matrix2x3::new(10, 40, 90,
    ///                               40, 100, 180);
    /// assert_eq!(a, expected);
    /// ```
    #[inline]
    pub fn column_iter_mut(&mut self) -> ColumnIterMut<'_, T, R, C, S>
    where
        S: RawStorageMut<T, R, C>,
    {
        ColumnIterMut::new(self)
    }
}

impl<T, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> Matrix<T, R, C, S> {
    /// Returns a mutable pointer to the start of the matrix.
    ///
    /// If the matrix is not empty, this pointer is guaranteed to be aligned
    /// and non-null.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.ptr_mut()
    }

    /// Swaps two entries without bound-checking.
    ///
    /// # Safety
    ///
    /// Both `(r, c)` must have `r < nrows(), c < ncols()`.
    #[inline]
    pub unsafe fn swap_unchecked(&mut self, row_cols1: (usize, usize), row_cols2: (usize, usize)) {
        unsafe {
            debug_assert!(row_cols1.0 < self.nrows() && row_cols1.1 < self.ncols());
            debug_assert!(row_cols2.0 < self.nrows() && row_cols2.1 < self.ncols());
            self.data.swap_unchecked(row_cols1, row_cols2)
        }
    }

    /// Swaps two entries.
    #[inline]
    pub fn swap(&mut self, row_cols1: (usize, usize), row_cols2: (usize, usize)) {
        let (nrows, ncols) = self.shape();
        assert!(
            row_cols1.0 < nrows && row_cols1.1 < ncols,
            "Matrix elements swap index out of bounds."
        );
        assert!(
            row_cols2.0 < nrows && row_cols2.1 < ncols,
            "Matrix elements swap index out of bounds."
        );
        unsafe { self.swap_unchecked(row_cols1, row_cols2) }
    }

    /// Fills this matrix with the content of a slice. Both must hold the same number of elements.
    ///
    /// The components of the slice are assumed to be ordered in column-major order.
    #[inline]
    pub fn copy_from_slice(&mut self, slice: &[T])
    where
        T: Scalar,
    {
        let (nrows, ncols) = self.shape();

        assert!(
            nrows * ncols == slice.len(),
            "The slice must contain the same number of elements as the matrix."
        );

        for j in 0..ncols {
            for i in 0..nrows {
                unsafe {
                    *self.get_unchecked_mut((i, j)) = slice.get_unchecked(i + j * nrows).clone();
                }
            }
        }
    }

    /// Fills this matrix with the content of another one. Both must have the same shape.
    #[inline]
    pub fn copy_from<R2, C2, SB>(&mut self, other: &Matrix<T, R2, C2, SB>)
    where
        T: Scalar,
        R2: Dim,
        C2: Dim,
        SB: RawStorage<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        assert!(
            self.shape() == other.shape(),
            "Unable to copy from a matrix with a different shape."
        );

        for j in 0..self.ncols() {
            for i in 0..self.nrows() {
                unsafe {
                    *self.get_unchecked_mut((i, j)) = other.get_unchecked((i, j)).clone();
                }
            }
        }
    }

    /// Fills this matrix with the content of the transpose another one.
    #[inline]
    pub fn tr_copy_from<R2, C2, SB>(&mut self, other: &Matrix<T, R2, C2, SB>)
    where
        T: Scalar,
        R2: Dim,
        C2: Dim,
        SB: RawStorage<T, R2, C2>,
        ShapeConstraint: DimEq<R, C2> + SameNumberOfColumns<C, R2>,
    {
        let (nrows, ncols) = self.shape();
        assert!(
            (ncols, nrows) == other.shape(),
            "Unable to copy from a matrix with incompatible shape."
        );

        for j in 0..ncols {
            for i in 0..nrows {
                unsafe {
                    *self.get_unchecked_mut((i, j)) = other.get_unchecked((j, i)).clone();
                }
            }
        }
    }

    // TODO: rename `apply` to `apply_mut` and `apply_into` to `apply`?
    /// Returns `self` with each of its components replaced by the result of a closure `f` applied on it.
    #[inline]
    pub fn apply_into<F: FnMut(&mut T)>(mut self, f: F) -> Self {
        self.apply(f);
        self
    }
}

impl<T, D: Dim, S: RawStorage<T, D>> Vector<T, D, S> {
    /// Gets a reference to the i-th element of this column vector without bound checking.
    /// # Safety
    /// `i` must be less than `D`.
    #[inline]
    #[must_use]
    pub unsafe fn vget_unchecked(&self, i: usize) -> &T {
        unsafe {
            debug_assert!(i < self.nrows(), "Vector index out of bounds.");
            let i = i * self.strides().0;
            self.data.get_unchecked_linear(i)
        }
    }
}

impl<T, D: Dim, S: RawStorageMut<T, D>> Vector<T, D, S> {
    /// Gets a mutable reference to the i-th element of this column vector without bound checking.
    /// # Safety
    /// `i` must be less than `D`.
    #[inline]
    #[must_use]
    pub unsafe fn vget_unchecked_mut(&mut self, i: usize) -> &mut T {
        unsafe {
            debug_assert!(i < self.nrows(), "Vector index out of bounds.");
            let i = i * self.strides().0;
            self.data.get_unchecked_linear_mut(i)
        }
    }
}

impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C> + IsContiguous> Matrix<T, R, C, S> {
    /// Extracts a slice containing the entire matrix entries ordered column-by-columns.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        // Safety: this is OK thanks to the IsContiguous trait.
        unsafe { self.data.as_slice_unchecked() }
    }
}

impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C> + IsContiguous> AsRef<[T]> for Matrix<T, R, C, S> {
    /// Extracts a slice containing the entire matrix entries ordered column-by-columns.
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, R: Dim, C: Dim, S: RawStorageMut<T, R, C> + IsContiguous> Matrix<T, R, C, S> {
    /// Extracts a mutable slice containing the entire matrix entries ordered column-by-columns.
    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // Safety: this is OK thanks to the IsContiguous trait.
        unsafe { self.data.as_mut_slice_unchecked() }
    }
}

impl<T: Scalar, D: Dim, S: RawStorageMut<T, D, D>> Matrix<T, D, D, S> {
    /// Transposes the square matrix `self` in-place.
    pub fn transpose_mut(&mut self) {
        assert!(
            self.is_square(),
            "Unable to transpose a non-square matrix in-place."
        );

        let dim = self.shape().0;

        for i in 1..dim {
            for j in 0..i {
                unsafe { self.swap_unchecked((i, j), (j, i)) }
            }
        }
    }
}

impl<T: SimdComplexField, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S> {
    /// Takes the adjoint (aka. conjugate-transpose) of `self` and store the result into `out`.
    #[inline]
    fn adjoint_to_uninit<Status, R2, C2, SB>(
        &self,
        _status: Status,
        out: &mut Matrix<Status::Value, R2, C2, SB>,
    ) where
        Status: InitStatus<T>,
        R2: Dim,
        C2: Dim,
        SB: RawStorageMut<Status::Value, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, C2> + SameNumberOfColumns<C, R2>,
    {
        let (nrows, ncols) = self.shape();
        assert!(
            (ncols, nrows) == out.shape(),
            "Incompatible shape for transpose-copy."
        );

        // TODO: optimize that.
        for i in 0..nrows {
            for j in 0..ncols {
                // Safety: all indices are in range.
                unsafe {
                    Status::init(
                        out.get_unchecked_mut((j, i)),
                        self.get_unchecked((i, j)).clone().simd_conjugate(),
                    );
                }
            }
        }
    }

    /// Takes the adjoint (aka. conjugate-transpose) of `self` and store the result into `out`.
    #[inline]
    pub fn adjoint_to<R2, C2, SB>(&self, out: &mut Matrix<T, R2, C2, SB>)
    where
        R2: Dim,
        C2: Dim,
        SB: RawStorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, C2> + SameNumberOfColumns<C, R2>,
    {
        self.adjoint_to_uninit(Init, out)
    }

    /// The adjoint (aka. conjugate-transpose) of `self`.
    #[inline]
    #[must_use = "Did you mean to use adjoint_mut()?"]
    pub fn adjoint(&self) -> OMatrix<T, C, R>
    where
        DefaultAllocator: Allocator<C, R>,
    {
        let (nrows, ncols) = self.shape_generic();

        let mut res = Matrix::uninit(ncols, nrows);
        self.adjoint_to_uninit(Uninit, &mut res);

        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }

    /// Takes the conjugate and transposes `self` and store the result into `out`.
    #[deprecated(note = "Renamed `self.adjoint_to(out)`.")]
    #[inline]
    pub fn conjugate_transpose_to<R2, C2, SB>(&self, out: &mut Matrix<T, R2, C2, SB>)
    where
        R2: Dim,
        C2: Dim,
        SB: RawStorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, C2> + SameNumberOfColumns<C, R2>,
    {
        self.adjoint_to(out)
    }

    /// The conjugate transposition of `self`.
    #[deprecated(note = "Renamed `self.adjoint()`.")]
    #[inline]
    pub fn conjugate_transpose(&self) -> OMatrix<T, C, R>
    where
        DefaultAllocator: Allocator<C, R>,
    {
        self.adjoint()
    }

    /// The conjugate of `self`.
    #[inline]
    #[must_use = "Did you mean to use conjugate_mut()?"]
    pub fn conjugate(&self) -> OMatrix<T, R, C>
    where
        DefaultAllocator: Allocator<R, C>,
    {
        self.map(|e| e.simd_conjugate())
    }

    /// Divides each component of the complex matrix `self` by the given real.
    #[inline]
    #[must_use = "Did you mean to use unscale_mut()?"]
    pub fn unscale(&self, real: T::SimdRealField) -> OMatrix<T, R, C>
    where
        DefaultAllocator: Allocator<R, C>,
    {
        self.map(|e| e.simd_unscale(real.clone()))
    }

    /// Multiplies each component of the complex matrix `self` by the given real.
    #[inline]
    #[must_use = "Did you mean to use scale_mut()?"]
    pub fn scale(&self, real: T::SimdRealField) -> OMatrix<T, R, C>
    where
        DefaultAllocator: Allocator<R, C>,
    {
        self.map(|e| e.simd_scale(real.clone()))
    }
}

impl<T: SimdComplexField, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> Matrix<T, R, C, S> {
    /// The conjugate of the complex matrix `self` computed in-place.
    #[inline]
    pub fn conjugate_mut(&mut self) {
        self.apply(|e| *e = e.clone().simd_conjugate())
    }

    /// Divides each component of the complex matrix `self` by the given real.
    #[inline]
    pub fn unscale_mut(&mut self, real: T::SimdRealField) {
        self.apply(|e| *e = e.clone().simd_unscale(real.clone()))
    }

    /// Multiplies each component of the complex matrix `self` by the given real.
    #[inline]
    pub fn scale_mut(&mut self, real: T::SimdRealField) {
        self.apply(|e| *e = e.clone().simd_scale(real.clone()))
    }
}

impl<T: SimdComplexField, D: Dim, S: RawStorageMut<T, D, D>> Matrix<T, D, D, S> {
    /// Sets `self` to its adjoint.
    #[deprecated(note = "Renamed to `self.adjoint_mut()`.")]
    pub fn conjugate_transform_mut(&mut self) {
        self.adjoint_mut()
    }

    /// Sets `self` to its adjoint (aka. conjugate-transpose).
    pub fn adjoint_mut(&mut self) {
        assert!(
            self.is_square(),
            "Unable to transpose a non-square matrix in-place."
        );

        let dim = self.shape().0;

        for i in 0..dim {
            for j in 0..i {
                unsafe {
                    let ref_ij = self.get_unchecked((i, j)).clone();
                    let ref_ji = self.get_unchecked((j, i)).clone();
                    let conj_ij = ref_ij.simd_conjugate();
                    let conj_ji = ref_ji.simd_conjugate();
                    *self.get_unchecked_mut((i, j)) = conj_ji;
                    *self.get_unchecked_mut((j, i)) = conj_ij;
                }
            }

            {
                let diag = unsafe { self.get_unchecked_mut((i, i)) };
                *diag = diag.clone().simd_conjugate();
            }
        }
    }
}

impl<T: Scalar, D: Dim, S: RawStorage<T, D, D>> SquareMatrix<T, D, S> {
    /// The diagonal of this matrix.
    #[inline]
    #[must_use]
    pub fn diagonal(&self) -> OVector<T, D>
    where
        DefaultAllocator: Allocator<D>,
    {
        self.map_diagonal(|e| e)
    }

    /// Apply the given function to this matrix's diagonal and returns it.
    ///
    /// This is a more efficient version of `self.diagonal().map(f)` since this
    /// allocates only once.
    #[must_use]
    pub fn map_diagonal<T2: Scalar>(&self, mut f: impl FnMut(T) -> T2) -> OVector<T2, D>
    where
        DefaultAllocator: Allocator<D>,
    {
        assert!(
            self.is_square(),
            "Unable to get the diagonal of a non-square matrix."
        );

        let dim = self.shape_generic().0;
        let mut res = Matrix::uninit(dim, Const::<1>);

        for i in 0..dim.value() {
            // Safety: all indices are in range.
            unsafe {
                *res.vget_unchecked_mut(i) =
                    MaybeUninit::new(f(self.get_unchecked((i, i)).clone()));
            }
        }

        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }

    /// Computes a trace of a square matrix, i.e., the sum of its diagonal elements.
    #[inline]
    #[must_use]
    pub fn trace(&self) -> T
    where
        T: Scalar + Zero + ClosedAddAssign,
    {
        assert!(
            self.is_square(),
            "Cannot compute the trace of non-square matrix."
        );

        let dim = self.shape_generic().0;
        let mut res = T::zero();

        for i in 0..dim.value() {
            res += unsafe { self.get_unchecked((i, i)).clone() };
        }

        res
    }
}

impl<T: SimdComplexField, D: Dim, S: Storage<T, D, D>> SquareMatrix<T, D, S> {
    /// The symmetric part of `self`, i.e., `0.5 * (self + self.transpose())`.
    #[inline]
    #[must_use]
    pub fn symmetric_part(&self) -> OMatrix<T, D, D>
    where
        DefaultAllocator: Allocator<D, D>,
    {
        assert!(
            self.is_square(),
            "Cannot compute the symmetric part of a non-square matrix."
        );
        let mut tr = self.transpose();
        tr += self;
        tr *= crate::convert::<_, T>(0.5);
        tr
    }

    /// The hermitian part of `self`, i.e., `0.5 * (self + self.adjoint())`.
    #[inline]
    #[must_use]
    pub fn hermitian_part(&self) -> OMatrix<T, D, D>
    where
        DefaultAllocator: Allocator<D, D>,
    {
        assert!(
            self.is_square(),
            "Cannot compute the hermitian part of a non-square matrix."
        );

        let mut tr = self.adjoint();
        tr += self;
        tr *= crate::convert::<_, T>(0.5);
        tr
    }
}

impl<T: Scalar + Zero + One, D: DimAdd<U1> + IsNotStaticOne, S: RawStorage<T, D, D>>
    Matrix<T, D, D, S>
{
    /// Yields the homogeneous matrix for this matrix, i.e., appending an additional dimension and
    /// and setting the diagonal element to `1`.
    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> OMatrix<T, DimSum<D, U1>, DimSum<D, U1>>
    where
        DefaultAllocator: Allocator<DimSum<D, U1>, DimSum<D, U1>>,
    {
        assert!(
            self.is_square(),
            "Only square matrices can currently be transformed to homogeneous coordinates."
        );
        let dim = DimSum::<D, U1>::from_usize(self.nrows() + 1);
        let mut res = OMatrix::identity_generic(dim, dim);
        res.generic_view_mut::<D, D>((0, 0), self.shape_generic())
            .copy_from(self);
        res
    }
}

impl<T: Scalar + Zero, D: DimAdd<U1>, S: RawStorage<T, D>> Vector<T, D, S> {
    /// Computes the coordinates in projective space of this vector, i.e., appends a `0` to its
    /// coordinates.
    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> OVector<T, DimSum<D, U1>>
    where
        DefaultAllocator: Allocator<DimSum<D, U1>>,
    {
        self.push(T::zero())
    }

    /// Constructs a vector from coordinates in projective space, i.e., removes a `0` at the end of
    /// `self`. Returns `None` if this last component is not zero.
    #[inline]
    pub fn from_homogeneous<SB>(v: Vector<T, DimSum<D, U1>, SB>) -> Option<OVector<T, D>>
    where
        SB: RawStorage<T, DimSum<D, U1>>,
        DefaultAllocator: Allocator<D>,
    {
        if v[v.len() - 1].is_zero() {
            let nrows = D::from_usize(v.len() - 1);
            Some(v.generic_view((0, 0), (nrows, Const::<1>)).into_owned())
        } else {
            None
        }
    }
}

impl<T: Scalar, D: DimAdd<U1>, S: RawStorage<T, D>> Vector<T, D, S> {
    /// Constructs a new vector of higher dimension by appending `element` to the end of `self`.
    #[inline]
    #[must_use]
    pub fn push(&self, element: T) -> OVector<T, DimSum<D, U1>>
    where
        DefaultAllocator: Allocator<DimSum<D, U1>>,
    {
        let len = self.len();
        let hnrows = DimSum::<D, U1>::from_usize(len + 1);
        let mut res = Matrix::uninit(hnrows, Const::<1>);
        // This is basically a copy_from except that we warp the copied
        // values into MaybeUninit.
        res.generic_view_mut((0, 0), self.shape_generic())
            .zip_apply(self, |out, e| *out = MaybeUninit::new(e));
        res[(len, 0)] = MaybeUninit::new(element);

        // Safety: res has been fully initialized.
        unsafe { res.assume_init() }
    }
}

impl<T, R: Dim, C: Dim, S> AbsDiffEq for Matrix<T, R, C, S>
where
    T: Scalar + AbsDiffEq,
    S: RawStorage<T, R, C>,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        assert!(self.shape() == other.shape());
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.abs_diff_eq(b, epsilon.clone()))
    }
}

impl<T, R: Dim, C: Dim, S> RelativeEq for Matrix<T, R, C, S>
where
    T: Scalar + RelativeEq,
    S: Storage<T, R, C>,
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
        self.relative_eq(other, epsilon, max_relative)
    }
}

impl<T, R: Dim, C: Dim, S> UlpsEq for Matrix<T, R, C, S>
where
    T: Scalar + UlpsEq,
    S: RawStorage<T, R, C>,
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        assert!(self.shape() == other.shape());
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.ulps_eq(b, epsilon.clone(), max_ulps))
    }
}

impl<T, R: Dim, C: Dim, S> PartialOrd for Matrix<T, R, C, S>
where
    T: Scalar + PartialOrd,
    S: RawStorage<T, R, C>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.shape() != other.shape() {
            return None;
        }

        if self.nrows() == 0 || self.ncols() == 0 {
            return Some(Ordering::Equal);
        }

        let mut first_ord = unsafe {
            self.data
                .get_unchecked_linear(0)
                .partial_cmp(other.data.get_unchecked_linear(0))
        };

        if let Some(first_ord) = first_ord.as_mut() {
            let mut it = self.iter().zip(other.iter());
            let _ = it.next(); // Drop the first elements (we already tested it).

            for (left, right) in it {
                if let Some(ord) = left.partial_cmp(right) {
                    match ord {
                        Ordering::Equal => { /* Does not change anything. */ }
                        Ordering::Less => {
                            if *first_ord == Ordering::Greater {
                                return None;
                            }
                            *first_ord = ord
                        }
                        Ordering::Greater => {
                            if *first_ord == Ordering::Less {
                                return None;
                            }
                            *first_ord = ord
                        }
                    }
                } else {
                    return None;
                }
            }
        }

        first_ord
    }

    #[inline]
    fn lt(&self, right: &Self) -> bool {
        assert_eq!(
            self.shape(),
            right.shape(),
            "Matrix comparison error: dimensions mismatch."
        );
        self.iter().zip(right.iter()).all(|(a, b)| a.lt(b))
    }

    #[inline]
    fn le(&self, right: &Self) -> bool {
        assert_eq!(
            self.shape(),
            right.shape(),
            "Matrix comparison error: dimensions mismatch."
        );
        self.iter().zip(right.iter()).all(|(a, b)| a.le(b))
    }

    #[inline]
    fn gt(&self, right: &Self) -> bool {
        assert_eq!(
            self.shape(),
            right.shape(),
            "Matrix comparison error: dimensions mismatch."
        );
        self.iter().zip(right.iter()).all(|(a, b)| a.gt(b))
    }

    #[inline]
    fn ge(&self, right: &Self) -> bool {
        assert_eq!(
            self.shape(),
            right.shape(),
            "Matrix comparison error: dimensions mismatch."
        );
        self.iter().zip(right.iter()).all(|(a, b)| a.ge(b))
    }
}

impl<T, R: Dim, C: Dim, S> Eq for Matrix<T, R, C, S>
where
    T: Eq,
    S: RawStorage<T, R, C>,
{
}

impl<T, R, R2, C, C2, S, S2> PartialEq<Matrix<T, R2, C2, S2>> for Matrix<T, R, C, S>
where
    T: PartialEq,
    C: Dim,
    C2: Dim,
    R: Dim,
    R2: Dim,
    S: RawStorage<T, R, C>,
    S2: RawStorage<T, R2, C2>,
{
    #[inline]
    fn eq(&self, right: &Matrix<T, R2, C2, S2>) -> bool {
        self.shape() == right.shape() && self.iter().zip(right.iter()).all(|(l, r)| l == r)
    }
}

macro_rules! impl_fmt {
    ($trait: path, $fmt_str_without_precision: expr_2021, $fmt_str_with_precision: expr_2021) => {
        impl<T, R: Dim, C: Dim, S> $trait for Matrix<T, R, C, S>
        where
            T: Scalar + $trait,
            S: RawStorage<T, R, C>,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                #[cfg(feature = "std")]
                fn val_width<T: Scalar + $trait>(val: &T, f: &mut fmt::Formatter<'_>) -> usize {
                    match f.precision() {
                        Some(precision) => format!($fmt_str_with_precision, val, precision)
                            .chars()
                            .count(),
                        None => format!($fmt_str_without_precision, val).chars().count(),
                    }
                }

                #[cfg(not(feature = "std"))]
                fn val_width<T: Scalar + $trait>(_: &T, _: &mut fmt::Formatter<'_>) -> usize {
                    4
                }

                let (nrows, ncols) = self.shape();

                if nrows == 0 || ncols == 0 {
                    return write!(f, "[ ]");
                }

                let mut max_length = 0;

                for i in 0..nrows {
                    for j in 0..ncols {
                        max_length = crate::max(max_length, val_width(&self[(i, j)], f));
                    }
                }

                let max_length_with_space = max_length + 1;

                writeln!(f)?;
                writeln!(
                    f,
                    "  ┌ {:>width$} ┐",
                    "",
                    width = max_length_with_space * ncols - 1
                )?;

                for i in 0..nrows {
                    write!(f, "  │")?;
                    for j in 0..ncols {
                        let number_length = val_width(&self[(i, j)], f) + 1;
                        let pad = max_length_with_space - number_length;
                        write!(f, " {:>thepad$}", "", thepad = pad)?;
                        match f.precision() {
                            Some(precision) => {
                                write!(f, $fmt_str_with_precision, (*self)[(i, j)], precision)?
                            }
                            None => write!(f, $fmt_str_without_precision, (*self)[(i, j)])?,
                        }
                    }
                    writeln!(f, " │")?;
                }

                writeln!(
                    f,
                    "  └ {:>width$} ┘",
                    "",
                    width = max_length_with_space * ncols - 1
                )?;
                writeln!(f)
            }
        }
    };
}
impl_fmt!(fmt::Display, "{}", "{:.1$}");
impl_fmt!(fmt::LowerExp, "{:e}", "{:.1$e}");
impl_fmt!(fmt::UpperExp, "{:E}", "{:.1$E}");
impl_fmt!(fmt::Octal, "{:o}", "{:1$o}");
impl_fmt!(fmt::LowerHex, "{:x}", "{:1$x}");
impl_fmt!(fmt::UpperHex, "{:X}", "{:1$X}");
impl_fmt!(fmt::Binary, "{:b}", "{:.1$b}");
impl_fmt!(fmt::Pointer, "{:p}", "{:.1$p}");

#[cfg(test)]
mod tests {
    #[test]
    fn empty_display() {
        let vec: Vec<f64> = Vec::new();
        let dvector = crate::DVector::from_vec(vec);
        assert_eq!(format!("{}", dvector), "[ ]")
    }

    #[test]
    fn lower_exp() {
        let test = crate::Matrix2::new(1e6, 2e5, 2e-5, 1.);
        assert_eq!(
            format!("{:e}", test),
            r"
  ┌           ┐
  │  1e6  2e5 │
  │ 2e-5  1e0 │
  └           ┘

"
        )
    }
}

/// # Cross product
impl<
    T: Scalar + ClosedAddAssign + ClosedSubAssign + ClosedMulAssign,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
> Matrix<T, R, C, S>
{
    /// The perpendicular product between two 2D column vectors, i.e. `a.x * b.y - a.y * b.x`.
    #[inline]
    #[must_use]
    pub fn perp<R2, C2, SB>(&self, b: &Matrix<T, R2, C2, SB>) -> T
    where
        R2: Dim,
        C2: Dim,
        SB: RawStorage<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, U2>
            + SameNumberOfColumns<C, U1>
            + SameNumberOfRows<R2, U2>
            + SameNumberOfColumns<C2, U1>,
    {
        let shape = self.shape();
        assert_eq!(
            shape,
            b.shape(),
            "2D vector perpendicular product dimension mismatch."
        );
        assert_eq!(
            shape,
            (2, 1),
            "2D perpendicular product requires (2, 1) vectors {shape:?}",
        );

        // SAFETY: assertion above ensures correct shape
        let ax = unsafe { self.get_unchecked((0, 0)).clone() };
        let ay = unsafe { self.get_unchecked((1, 0)).clone() };
        let bx = unsafe { b.get_unchecked((0, 0)).clone() };
        let by = unsafe { b.get_unchecked((1, 0)).clone() };

        ax * by - ay * bx
    }

    // TODO: use specialization instead of an assertion.
    /// The 3D cross product between two vectors.
    ///
    /// Panics if the shape is not 3D vector. In the future, this will be implemented only for
    /// dynamically-sized matrices and statically-sized 3D matrices.
    #[inline]
    #[must_use]
    pub fn cross<R2, C2, SB>(&self, b: &Matrix<T, R2, C2, SB>) -> MatrixCross<T, R, C, R2, C2>
    where
        R2: Dim,
        C2: Dim,
        SB: RawStorage<T, R2, C2>,
        DefaultAllocator: SameShapeAllocator<R, C, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        let shape = self.shape();
        assert_eq!(shape, b.shape(), "Vector cross product dimension mismatch.");
        assert!(
            shape == (3, 1) || shape == (1, 3),
            "Vector cross product dimension mismatch: must be (3, 1) or (1, 3) but found {shape:?}.",
        );

        if shape.0 == 3 {
            unsafe {
                let mut res = Matrix::uninit(Dim::from_usize(3), Dim::from_usize(1));

                let ax = self.get_unchecked((0, 0));
                let ay = self.get_unchecked((1, 0));
                let az = self.get_unchecked((2, 0));

                let bx = b.get_unchecked((0, 0));
                let by = b.get_unchecked((1, 0));
                let bz = b.get_unchecked((2, 0));

                *res.get_unchecked_mut((0, 0)) =
                    MaybeUninit::new(ay.clone() * bz.clone() - az.clone() * by.clone());
                *res.get_unchecked_mut((1, 0)) =
                    MaybeUninit::new(az.clone() * bx.clone() - ax.clone() * bz.clone());
                *res.get_unchecked_mut((2, 0)) =
                    MaybeUninit::new(ax.clone() * by.clone() - ay.clone() * bx.clone());

                // Safety: res is now fully initialized.
                res.assume_init()
            }
        } else {
            unsafe {
                let mut res = Matrix::uninit(Dim::from_usize(1), Dim::from_usize(3));

                let ax = self.get_unchecked((0, 0));
                let ay = self.get_unchecked((0, 1));
                let az = self.get_unchecked((0, 2));

                let bx = b.get_unchecked((0, 0));
                let by = b.get_unchecked((0, 1));
                let bz = b.get_unchecked((0, 2));

                *res.get_unchecked_mut((0, 0)) =
                    MaybeUninit::new(ay.clone() * bz.clone() - az.clone() * by.clone());
                *res.get_unchecked_mut((0, 1)) =
                    MaybeUninit::new(az.clone() * bx.clone() - ax.clone() * bz.clone());
                *res.get_unchecked_mut((0, 2)) =
                    MaybeUninit::new(ax.clone() * by.clone() - ay.clone() * bx.clone());

                // Safety: res is now fully initialized.
                res.assume_init()
            }
        }
    }
}

impl<T: Scalar + Field, S: RawStorage<T, U3>> Vector<T, U3, S> {
    /// Computes the matrix `M` such that for all vector `v` we have `M * v == self.cross(&v)`.
    #[inline]
    #[must_use]
    pub fn cross_matrix(&self) -> OMatrix<T, U3, U3> {
        OMatrix::<T, U3, U3>::new(
            T::zero(),
            -self[2].clone(),
            self[1].clone(),
            self[2].clone(),
            T::zero(),
            -self[0].clone(),
            -self[1].clone(),
            self[0].clone(),
            T::zero(),
        )
    }
}

impl<T: SimdComplexField, R: Dim, C: Dim, S: Storage<T, R, C>> Matrix<T, R, C, S> {
    /// The smallest angle between two vectors.
    #[inline]
    #[must_use]
    pub fn angle<R2: Dim, C2: Dim, SB>(&self, other: &Matrix<T, R2, C2, SB>) -> T::SimdRealField
    where
        SB: Storage<T, R2, C2>,
        ShapeConstraint: DimEq<R, R2> + DimEq<C, C2>,
    {
        let prod = self.dotc(other);
        let n1 = self.norm();
        let n2 = other.norm();

        if n1.is_zero() || n2.is_zero() {
            T::SimdRealField::zero()
        } else {
            let cang = prod.simd_real() / (n1 * n2);
            cang.simd_clamp(-T::SimdRealField::one(), T::SimdRealField::one())
                .simd_acos()
        }
    }
}

impl<T, R: Dim, C: Dim, S> AbsDiffEq for Unit<Matrix<T, R, C, S>>
where
    T: Scalar + AbsDiffEq,
    S: RawStorage<T, R, C>,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.as_ref().abs_diff_eq(other.as_ref(), epsilon)
    }
}

impl<T, R: Dim, C: Dim, S> RelativeEq for Unit<Matrix<T, R, C, S>>
where
    T: Scalar + RelativeEq,
    S: Storage<T, R, C>,
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
        self.as_ref()
            .relative_eq(other.as_ref(), epsilon, max_relative)
    }
}

impl<T, R: Dim, C: Dim, S> UlpsEq for Unit<Matrix<T, R, C, S>>
where
    T: Scalar + UlpsEq,
    S: RawStorage<T, R, C>,
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.as_ref().ulps_eq(other.as_ref(), epsilon, max_ulps)
    }
}

impl<T, R, C, S> Hash for Matrix<T, R, C, S>
where
    T: Scalar + Hash,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (nrows, ncols) = self.shape();
        (nrows, ncols).hash(state);

        for j in 0..ncols {
            for i in 0..nrows {
                unsafe {
                    self.get_unchecked((i, j)).hash(state);
                }
            }
        }
    }
}

impl<T, D, S> Unit<Vector<T, D, S>>
where
    T: Scalar,
    D: Dim,
    S: RawStorage<T, D, U1>,
{
    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// let v = Vector3::<f64>::y_axis();
    /// let v2 = v.cast::<f32>();
    /// assert_eq!(v2, Vector3::<f32>::y_axis());
    /// ```
    pub fn cast<T2: Scalar>(self) -> Unit<OVector<T2, D>>
    where
        T: Scalar,
        OVector<T2, D>: SupersetOf<Vector<T, D, S>>,
        DefaultAllocator: Allocator<D, U1>,
    {
        Unit::new_unchecked(crate::convert_ref(self.as_ref()))
    }
}

impl<T, S> Matrix<T, U1, U1, S>
where
    S: RawStorage<T, U1, U1>,
{
    /// Returns a reference to the single element in this matrix.
    ///
    /// As opposed to indexing, using this provides type-safety
    /// when flattening dimensions.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// let v = Vector3::new(0., 0., 1.);
    /// let inner_product: f32 = *(v.transpose() * v).as_scalar();
    /// ```
    ///
    ///```compile_fail
    /// # use nalgebra::Vector3;
    /// let v = Vector3::new(0., 0., 1.);
    /// let inner_product = (v * v.transpose()).item(); // Typo, does not compile.
    ///```
    pub fn as_scalar(&self) -> &T {
        &self[(0, 0)]
    }
    /// Get a mutable reference to the single element in this matrix
    ///
    /// As opposed to indexing, using this provides type-safety
    /// when flattening dimensions.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// let v = Vector3::new(0., 0., 1.);
    /// let mut inner_product = (v.transpose() * v);
    /// *inner_product.as_scalar_mut() = 3.;
    /// ```
    ///
    ///```compile_fail
    /// # use nalgebra::Vector3;
    /// let v = Vector3::new(0., 0., 1.);
    /// let mut inner_product = (v * v.transpose());
    /// *inner_product.as_scalar_mut() = 3.;
    ///```
    pub fn as_scalar_mut(&mut self) -> &mut T
    where
        S: RawStorageMut<T, U1>,
    {
        &mut self[(0, 0)]
    }
    /// Convert this 1x1 matrix by reference into a scalar.
    ///
    /// As opposed to indexing, using this provides type-safety
    /// when flattening dimensions.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// let v = Vector3::new(0., 0., 1.);
    /// let mut inner_product: f32 = (v.transpose() * v).to_scalar();
    /// ```
    ///
    ///```compile_fail
    /// # use nalgebra::Vector3;
    /// let v = Vector3::new(0., 0., 1.);
    /// let mut inner_product: f32 = (v * v.transpose()).to_scalar();
    ///```
    pub fn to_scalar(&self) -> T
    where
        T: Clone,
    {
        self.as_scalar().clone()
    }
}

impl<T> super::alias::Matrix1<T> {
    /// Convert this 1x1 matrix into a scalar.
    ///
    /// As opposed to indexing, using this provides type-safety
    /// when flattening dimensions.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Vector3, Matrix2, U1};
    /// let v = Vector3::new(0., 0., 1.);
    /// let inner_product: f32 = (v.transpose() * v).into_scalar();
    /// assert_eq!(inner_product, 1.);
    /// ```
    ///
    ///```compile_fail
    /// # use nalgebra::Vector3;
    /// let v = Vector3::new(0., 0., 1.);
    /// let mut inner_product: f32 = (v * v.transpose()).into_scalar();
    ///```
    pub fn into_scalar(self) -> T {
        let [[scalar]] = self.data.0;
        scalar
    }
}
