#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

#[cfg(feature = "arbitrary")]
use crate::base::storage::Owned;
#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

use num::{Bounded, One, Zero};
#[cfg(feature = "rand-no-std")]
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use typenum::{self, Cmp, Greater};

use simba::scalar::{ClosedAddAssign, ClosedMulAssign};

use crate::UninitMatrix;
use crate::base::allocator::Allocator;
use crate::base::dimension::{Dim, DimName, Dyn, ToTypenum};
use crate::base::storage::RawStorage;
use crate::base::{
    ArrayStorage, Const, DefaultAllocator, Matrix, OMatrix, OVector, Scalar, Unit, Vector,
};
use std::mem::MaybeUninit;

impl<T: Scalar, R: Dim, C: Dim> UninitMatrix<T, R, C>
where
    DefaultAllocator: Allocator<R, C>,
{
    /// Builds a matrix with uninitialized elements of type `MaybeUninit<T>`.
    #[inline(always)]
    pub fn uninit(nrows: R, ncols: C) -> Self {
        // SAFETY: this is OK because the dimension automatically match the storage
        //         because we are building an owned storage.
        unsafe {
            Self::from_data_statically_unchecked(DefaultAllocator::allocate_uninit(nrows, ncols))
        }
    }
}

/// # Generic constructors
/// This set of matrix and vector construction functions are all generic
/// with-regard to the matrix dimensions. They all expect to be given
/// the dimension as inputs.
///
/// These functions should only be used when working on dimension-generic code.
impl<T: Scalar, R: Dim, C: Dim> OMatrix<T, R, C>
where
    DefaultAllocator: Allocator<R, C>,
{
    /// Creates a matrix with all its elements set to `elem`.
    #[inline]
    pub fn from_element_generic(nrows: R, ncols: C, elem: T) -> Self {
        let len = nrows.value() * ncols.value();
        Self::from_iterator_generic(nrows, ncols, std::iter::repeat_n(elem, len))
    }

    /// Creates a matrix with all its elements set to `elem`.
    ///
    /// Same as `from_element_generic`.
    #[inline]
    pub fn repeat_generic(nrows: R, ncols: C, elem: T) -> Self {
        let len = nrows.value() * ncols.value();
        Self::from_iterator_generic(nrows, ncols, std::iter::repeat_n(elem, len))
    }

    /// Creates a matrix with all its elements set to 0.
    #[inline]
    pub fn zeros_generic(nrows: R, ncols: C) -> Self
    where
        T: Zero,
    {
        Self::from_element_generic(nrows, ncols, T::zero())
    }

    /// Creates a matrix with all its elements filled by an iterator.
    #[inline]
    pub fn from_iterator_generic<I>(nrows: R, ncols: C, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::from_data(DefaultAllocator::allocate_from_iterator(nrows, ncols, iter))
    }

    /// Creates a matrix with all its elements filled by an row-major order iterator.
    #[inline]
    pub fn from_row_iterator_generic<I>(nrows: R, ncols: C, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::from_data(DefaultAllocator::allocate_from_row_iterator(
            nrows, ncols, iter,
        ))
    }

    /// Creates a matrix with its elements filled with the components provided by a slice in
    /// row-major order.
    ///
    /// The order of elements in the slice must follow the usual mathematic writing, i.e.,
    /// row-by-row.
    #[inline]
    pub fn from_row_slice_generic(nrows: R, ncols: C, slice: &[T]) -> Self {
        assert!(
            slice.len() == nrows.value() * ncols.value(),
            "Matrix init. error: the slice did not contain the right number of elements."
        );

        let mut res = Matrix::uninit(nrows, ncols);
        let mut iter = slice.iter();

        unsafe {
            for i in 0..nrows.value() {
                for j in 0..ncols.value() {
                    *res.get_unchecked_mut((i, j)) = MaybeUninit::new(iter.next().unwrap().clone())
                }
            }

            // SAFETY: the result has been fully initialized above.
            res.assume_init()
        }
    }

    /// Creates a matrix with its elements filled with the components provided by a slice. The
    /// components must have the same layout as the matrix data storage (i.e. column-major).
    #[inline]
    pub fn from_column_slice_generic(nrows: R, ncols: C, slice: &[T]) -> Self {
        Self::from_iterator_generic(nrows, ncols, slice.iter().cloned())
    }

    /// Creates a matrix filled with the results of a function applied to each of its component
    /// coordinates.
    #[inline]
    pub fn from_fn_generic<F>(nrows: R, ncols: C, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut res = Matrix::uninit(nrows, ncols);

        unsafe {
            for j in 0..ncols.value() {
                for i in 0..nrows.value() {
                    *res.get_unchecked_mut((i, j)) = MaybeUninit::new(f(i, j));
                }
            }

            // SAFETY: the result has been fully initialized above.
            res.assume_init()
        }
    }

    /// Creates a new identity matrix.
    ///
    /// If the matrix is not square, the largest square submatrix starting at index `(0, 0)` is set
    /// to the identity matrix. All other entries are set to zero.
    #[inline]
    pub fn identity_generic(nrows: R, ncols: C) -> Self
    where
        T: Zero + One,
    {
        Self::from_diagonal_element_generic(nrows, ncols, T::one())
    }

    /// Creates a new matrix with its diagonal filled with copies of `elt`.
    ///
    /// If the matrix is not square, the largest square submatrix starting at index `(0, 0)` is set
    /// to the identity matrix. All other entries are set to zero.
    #[inline]
    pub fn from_diagonal_element_generic(nrows: R, ncols: C, elt: T) -> Self
    where
        T: Zero + One,
    {
        let mut res = Self::zeros_generic(nrows, ncols);

        for i in 0..crate::min(nrows.value(), ncols.value()) {
            unsafe { *res.get_unchecked_mut((i, i)) = elt.clone() }
        }

        res
    }

    /// Creates a new matrix that may be rectangular. The first `elts.len()` diagonal elements are
    /// filled with the content of `elts`. Others are set to 0.
    ///
    /// Panics if `elts.len()` is larger than the minimum among `nrows` and `ncols`.
    #[inline]
    pub fn from_partial_diagonal_generic(nrows: R, ncols: C, elts: &[T]) -> Self
    where
        T: Zero,
    {
        let mut res = Self::zeros_generic(nrows, ncols);
        assert!(
            elts.len() <= crate::min(nrows.value(), ncols.value()),
            "Too many diagonal elements provided."
        );

        for (i, elt) in elts.iter().enumerate() {
            unsafe { *res.get_unchecked_mut((i, i)) = elt.clone() }
        }

        res
    }

    /// Builds a new matrix from its rows.
    ///
    /// Panics if not enough rows are provided (for statically-sized matrices), or if all rows do
    /// not have the same dimensions.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{RowVector3, Matrix3};
    /// # use std::iter;
    ///
    /// let m = Matrix3::from_rows(&[ RowVector3::new(1.0, 2.0, 3.0),  RowVector3::new(4.0, 5.0, 6.0),  RowVector3::new(7.0, 8.0, 9.0) ]);
    ///
    /// assert!(m.m11 == 1.0 && m.m12 == 2.0 && m.m13 == 3.0 &&
    ///         m.m21 == 4.0 && m.m22 == 5.0 && m.m23 == 6.0 &&
    ///         m.m31 == 7.0 && m.m32 == 8.0 && m.m33 == 9.0);
    /// ```
    #[inline]
    pub fn from_rows<SB>(rows: &[Matrix<T, Const<1>, C, SB>]) -> Self
    where
        SB: RawStorage<T, Const<1>, C>,
    {
        assert!(!rows.is_empty(), "At least one row must be given.");
        let nrows = R::try_to_usize().unwrap_or(rows.len());
        let ncols = rows[0].len();
        assert!(
            rows.len() == nrows,
            "Invalid number of rows provided to build this matrix."
        );

        if C::try_to_usize().is_none() {
            assert!(
                rows.iter().all(|r| r.len() == ncols),
                "The provided rows must all have the same dimension."
            );
        }

        // TODO: optimize that.
        Self::from_fn_generic(R::from_usize(nrows), C::from_usize(ncols), |i, j| {
            rows[i][(0, j)].clone()
        })
    }

    /// Builds a new matrix from its columns.
    ///
    /// Panics if not enough columns are provided (for statically-sized matrices), or if all
    /// columns do not have the same dimensions.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Vector3, Matrix3};
    /// # use std::iter;
    ///
    /// let m = Matrix3::from_columns(&[ Vector3::new(1.0, 2.0, 3.0),  Vector3::new(4.0, 5.0, 6.0),  Vector3::new(7.0, 8.0, 9.0) ]);
    ///
    /// assert!(m.m11 == 1.0 && m.m12 == 4.0 && m.m13 == 7.0 &&
    ///         m.m21 == 2.0 && m.m22 == 5.0 && m.m23 == 8.0 &&
    ///         m.m31 == 3.0 && m.m32 == 6.0 && m.m33 == 9.0);
    /// ```
    #[inline]
    pub fn from_columns<SB>(columns: &[Vector<T, R, SB>]) -> Self
    where
        SB: RawStorage<T, R>,
    {
        assert!(!columns.is_empty(), "At least one column must be given.");
        let ncols = C::try_to_usize().unwrap_or(columns.len());
        let nrows = columns[0].len();
        assert!(
            columns.len() == ncols,
            "Invalid number of columns provided to build this matrix."
        );

        if R::try_to_usize().is_none() {
            assert!(
                columns.iter().all(|r| r.len() == nrows),
                "The columns provided must all have the same dimension."
            );
        }

        // TODO: optimize that.
        Self::from_fn_generic(R::from_usize(nrows), C::from_usize(ncols), |i, j| {
            columns[j][i].clone()
        })
    }

    /// Creates a matrix filled with random values.
    #[inline]
    #[cfg(feature = "rand")]
    pub fn new_random_generic(nrows: R, ncols: C) -> Self
    where
        StandardUniform: Distribution<T>,
    {
        let mut rng = rand::rng();
        Self::from_fn_generic(nrows, ncols, |_, _| rng.random())
    }

    /// Creates a matrix filled with random values from the given distribution.
    #[inline]
    #[cfg(feature = "rand-no-std")]
    pub fn from_distribution_generic<Distr: Distribution<T> + ?Sized, G: Rng + ?Sized>(
        nrows: R,
        ncols: C,
        distribution: &Distr,
        rng: &mut G,
    ) -> Self {
        Self::from_fn_generic(nrows, ncols, |_, _| distribution.sample(rng))
    }

    /// Creates a matrix backed by a given `Vec`.
    ///
    /// The output matrix is filled column-by-column.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Dyn, DMatrix, Matrix, Const};
    ///
    /// let vec = vec![0, 1, 2, 3, 4, 5];
    /// let vec_ptr = vec.as_ptr();
    ///
    /// let matrix = Matrix::from_vec_generic(Dyn(vec.len()), Const::<1>, vec);
    /// let matrix_storage_ptr = matrix.data.as_vec().as_ptr();
    ///
    /// // `matrix` is backed by exactly the same `Vec` as it was constructed from.
    /// assert_eq!(matrix_storage_ptr, vec_ptr);
    /// ```
    #[inline]
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn from_vec_generic(nrows: R, ncols: C, data: Vec<T>) -> Self {
        Self::from_iterator_generic(nrows, ncols, data)
    }
}

impl<T, D: Dim> OMatrix<T, D, D>
where
    T: Scalar,
    DefaultAllocator: Allocator<D, D>,
{
    /// Creates a square matrix with its diagonal set to `diag` and all other entries set to 0.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Vector3, DVector, Matrix3, DMatrix};
    /// # use std::iter;
    ///
    /// let m = Matrix3::from_diagonal(&Vector3::new(1.0, 2.0, 3.0));
    /// // The two additional arguments represent the matrix dimensions.
    /// let dm = DMatrix::from_diagonal(&DVector::from_row_slice(&[1.0, 2.0, 3.0]));
    ///
    /// assert!(m.m11 == 1.0 && m.m12 == 0.0 && m.m13 == 0.0 &&
    ///         m.m21 == 0.0 && m.m22 == 2.0 && m.m23 == 0.0 &&
    ///         m.m31 == 0.0 && m.m32 == 0.0 && m.m33 == 3.0);
    /// assert!(dm[(0, 0)] == 1.0 && dm[(0, 1)] == 0.0 && dm[(0, 2)] == 0.0 &&
    ///         dm[(1, 0)] == 0.0 && dm[(1, 1)] == 2.0 && dm[(1, 2)] == 0.0 &&
    ///         dm[(2, 0)] == 0.0 && dm[(2, 1)] == 0.0 && dm[(2, 2)] == 3.0);
    /// ```
    #[inline]
    pub fn from_diagonal<SB: RawStorage<T, D>>(diag: &Vector<T, D, SB>) -> Self
    where
        T: Zero,
    {
        let (dim, _) = diag.shape_generic();
        let mut res = Self::zeros_generic(dim, dim);

        for i in 0..diag.len() {
            unsafe {
                *res.get_unchecked_mut((i, i)) = diag.vget_unchecked(i).clone();
            }
        }

        res
    }
}

/*
 *
 * Generate constructors with varying number of arguments, depending on the object type.
 *
 */
macro_rules! impl_constructors(
    ($($Dims: ty),*; $(=> $DimIdent: ident: $DimBound: ident),*; $($gargs: expr_2021),*; $($args: ident),*) => {
        /// Creates a matrix or vector with all its elements set to `elem`.
        ///
        /// # Example
        /// ```
        /// # use nalgebra::{Matrix2x3, Vector3, DVector, DMatrix};
        ///
        /// let v = Vector3::from_element(2.0);
        /// // The additional argument represents the vector dimension.
        /// let dv = DVector::from_element(3, 2.0);
        /// let m = Matrix2x3::from_element(2.0);
        /// // The two additional arguments represent the matrix dimensions.
        /// let dm = DMatrix::from_element(2, 3, 2.0);
        ///
        /// assert!(v.x == 2.0 && v.y == 2.0 && v.z == 2.0);
        /// assert!(dv[0] == 2.0 && dv[1] == 2.0 && dv[2] == 2.0);
        /// assert!(m.m11 == 2.0 && m.m12 == 2.0 && m.m13 == 2.0 &&
        ///         m.m21 == 2.0 && m.m22 == 2.0 && m.m23 == 2.0);
        /// assert!(dm[(0, 0)] == 2.0 && dm[(0, 1)] == 2.0 && dm[(0, 2)] == 2.0 &&
        ///         dm[(1, 0)] == 2.0 && dm[(1, 1)] == 2.0 && dm[(1, 2)] == 2.0);
        /// ```
        #[inline]
        pub fn from_element($($args: usize,)* elem: T) -> Self {
            Self::from_element_generic($($gargs, )* elem)
        }

        /// Creates a matrix or vector with all its elements set to `elem`.
        ///
        /// Same as `.from_element`.
        ///
        /// # Example
        /// ```
        /// # use nalgebra::{Matrix2x3, Vector3, DVector, DMatrix};
        ///
        /// let v = Vector3::repeat(2.0);
        /// // The additional argument represents the vector dimension.
        /// let dv = DVector::repeat(3, 2.0);
        /// let m = Matrix2x3::repeat(2.0);
        /// // The two additional arguments represent the matrix dimensions.
        /// let dm = DMatrix::repeat(2, 3, 2.0);
        ///
        /// assert!(v.x == 2.0 && v.y == 2.0 && v.z == 2.0);
        /// assert!(dv[0] == 2.0 && dv[1] == 2.0 && dv[2] == 2.0);
        /// assert!(m.m11 == 2.0 && m.m12 == 2.0 && m.m13 == 2.0 &&
        ///         m.m21 == 2.0 && m.m22 == 2.0 && m.m23 == 2.0);
        /// assert!(dm[(0, 0)] == 2.0 && dm[(0, 1)] == 2.0 && dm[(0, 2)] == 2.0 &&
        ///         dm[(1, 0)] == 2.0 && dm[(1, 1)] == 2.0 && dm[(1, 2)] == 2.0);
        /// ```
        #[inline]
        pub fn repeat($($args: usize,)* elem: T) -> Self {
            Self::repeat_generic($($gargs, )* elem)
        }

        /// Creates a matrix or vector with all its elements set to `0`.
        ///
        /// # Example
        /// ```
        /// # use nalgebra::{Matrix2x3, Vector3, DVector, DMatrix};
        ///
        /// let v = Vector3::<f32>::zeros();
        /// // The argument represents the vector dimension.
        /// let dv = DVector::<f32>::zeros(3);
        /// let m = Matrix2x3::<f32>::zeros();
        /// // The two arguments represent the matrix dimensions.
        /// let dm = DMatrix::<f32>::zeros(2, 3);
        ///
        /// assert!(v.x == 0.0 && v.y == 0.0 && v.z == 0.0);
        /// assert!(dv[0] == 0.0 && dv[1] == 0.0 && dv[2] == 0.0);
        /// assert!(m.m11 == 0.0 && m.m12 == 0.0 && m.m13 == 0.0 &&
        ///         m.m21 == 0.0 && m.m22 == 0.0 && m.m23 == 0.0);
        /// assert!(dm[(0, 0)] == 0.0 && dm[(0, 1)] == 0.0 && dm[(0, 2)] == 0.0 &&
        ///         dm[(1, 0)] == 0.0 && dm[(1, 1)] == 0.0 && dm[(1, 2)] == 0.0);
        /// ```
        #[inline]
        pub fn zeros($($args: usize),*) -> Self
            where T: Zero {
            Self::zeros_generic($($gargs),*)
        }

        /// Creates a matrix or vector with all its elements filled by an iterator.
        ///
        /// The output matrix is filled column-by-column.
        ///
        /// # Example
        /// ```
        /// # use nalgebra::{Matrix2x3, Vector3, DVector, DMatrix};
        /// # use std::iter;
        ///
        /// let v = Vector3::from_iterator((0..3).into_iter());
        /// // The additional argument represents the vector dimension.
        /// let dv = DVector::from_iterator(3, (0..3).into_iter());
        /// let m = Matrix2x3::from_iterator((0..6).into_iter());
        /// // The two additional arguments represent the matrix dimensions.
        /// let dm = DMatrix::from_iterator(2, 3, (0..6).into_iter());
        ///
        /// assert!(v.x == 0 && v.y == 1 && v.z == 2);
        /// assert!(dv[0] == 0 && dv[1] == 1 && dv[2] == 2);
        /// assert!(m.m11 == 0 && m.m12 == 2 && m.m13 == 4 &&
        ///         m.m21 == 1 && m.m22 == 3 && m.m23 == 5);
        /// assert!(dm[(0, 0)] == 0 && dm[(0, 1)] == 2 && dm[(0, 2)] == 4 &&
        ///         dm[(1, 0)] == 1 && dm[(1, 1)] == 3 && dm[(1, 2)] == 5);
        /// ```
        #[inline]
        pub fn from_iterator<I>($($args: usize,)* iter: I) -> Self
            where I: IntoIterator<Item = T> {
            Self::from_iterator_generic($($gargs, )* iter)
        }

        /// Creates a matrix or vector with all its elements filled by a row-major iterator.
        ///
        /// The output matrix is filled row-by-row.
        ///
        /// ## Example
        /// ```
        /// # use nalgebra::{Matrix2x3, Vector3, DVector, DMatrix};
        /// # use std::iter;
        ///
        /// let v = Vector3::from_row_iterator((0..3).into_iter());
        /// // The additional argument represents the vector dimension.
        /// let dv = DVector::from_row_iterator(3, (0..3).into_iter());
        /// let m = Matrix2x3::from_row_iterator((0..6).into_iter());
        /// // The two additional arguments represent the matrix dimensions.
        /// let dm = DMatrix::from_row_iterator(2, 3, (0..6).into_iter());
        ///
        /// // For Vectors from_row_iterator is identical to from_iterator
        /// assert!(v.x == 0 && v.y == 1 && v.z == 2);
        /// assert!(dv[0] == 0 && dv[1] == 1 && dv[2] == 2);
        /// assert!(m.m11 == 0 && m.m12 == 1 && m.m13 == 2 &&
        ///         m.m21 == 3 && m.m22 == 4 && m.m23 == 5);
        /// assert!(dm[(0, 0)] == 0 && dm[(0, 1)] == 1 && dm[(0, 2)] == 2 &&
        ///         dm[(1, 0)] == 3 && dm[(1, 1)] == 4 && dm[(1, 2)] == 5);
        /// ```
        #[inline]
        pub fn from_row_iterator<I>($($args: usize,)* iter: I) -> Self
            where I: IntoIterator<Item = T> {
            Self::from_row_iterator_generic($($gargs, )* iter)
        }

        /// Creates a matrix or vector filled with the results of a function applied to each of its
        /// element's indices.
        ///
        /// The `from_fn` syntax changes depending if the type's rows or columns are dynamically or statically sized.
        /// The dimension must only be supplied iff it is dynamically sized, e.g.
        /// - `from_fn(f)` when both dimensions are statically sized
        /// - `from_fn(nrows, f)` when only row dimension is dynamically sized
        /// - `from_fn(ncols, f)` when only column dimension is dynamically sized
        /// - `from_fn(nrows, ncols, f)` when both dimensions are dynamically sized
        ///
        /// The function object `f` (i.e. the last argument to `from_fn`) receives the row and column indices as arguments (e.g. `f(row, col)`).
        /// For vectors, the column index is always zero (e.g. `f(row, 0)`).
        ///
        /// # Arguments
        ///
        /// - `nrows`: The number of rows. Must only be supplied for types with dynamic number of rows.
        ///
        /// - `ncols`: The number of columns. Must only be supplied for types with dynamic number of columns.
        ///
        /// - `f`: A function accepting the zero-based indices `(row, col)`.
        ///
        /// # Examples
        ///
        /// ```
        /// use nalgebra::{DMatrix, DVector, Matrix2x3, Matrix2xX, MatrixXx3, Vector3};
        /// use nalgebra_macros::{matrix, vector};
        ///
        /// // statically sized dimensions -> both inferred
        /// let v_3 = Vector3::from_fn(|i, _| i);
        /// let m_2_3 = Matrix2x3::from_fn(|i, j| i * 3 + j);
        /// assert_eq!(v_3, vector![0, 1, 2]);
        /// assert_eq!(m_2_3, matrix![0, 1, 2; 3, 4, 5]);
        ///
        /// // mixed sized dimensions -> static dimensions inferred, dynamic dimension must be given explicitly
        /// let m_2_d = Matrix2xX::from_fn(3, |i, j| i * 3 + j); // explicit: ncols=3
        /// let m_d_3 = MatrixXx3::from_fn(2, |i, j| i * 3 + j); // explicit: nrows=2
        /// assert_eq!(m_2_d, matrix![0, 1, 2; 3, 4, 5]);
        /// assert_eq!(m_d_3, matrix![0, 1, 2; 3, 4, 5]);
        ///
        /// // dynamically sized dimensions -> both must be given explicitly
        /// let v_d = DVector::from_fn(3, |i, _| i);
        /// let m_d_d = DMatrix::from_fn(2, 3, |i, j| i * 3 + j); // explicit: nrows=2, ncols=3
        /// assert_eq!(v_d, vector![0, 1, 2]);
        /// assert_eq!(m_d_d, matrix![0, 1, 2; 3, 4, 5]);
        /// ```
        #[inline]
        pub fn from_fn<F>($($args: usize,)* f: F) -> Self
            where F: FnMut(usize, usize) -> T {
            Self::from_fn_generic($($gargs, )* f)
        }

        /// Creates an identity matrix. If the matrix is not square, the largest square
        /// submatrix (starting at the first row and column) is set to the identity while all
        /// other entries are set to zero.
        ///
        /// # Example
        /// ```
        /// # use nalgebra::{Matrix2x3, DMatrix};
        /// # use std::iter;
        ///
        /// let m = Matrix2x3::<f32>::identity();
        /// // The two additional arguments represent the matrix dimensions.
        /// let dm = DMatrix::<f32>::identity(2, 3);
        ///
        /// assert!(m.m11 == 1.0 && m.m12 == 0.0 && m.m13 == 0.0 &&
        ///         m.m21 == 0.0 && m.m22 == 1.0 && m.m23 == 0.0);
        /// assert!(dm[(0, 0)] == 1.0 && dm[(0, 1)] == 0.0 && dm[(0, 2)] == 0.0 &&
        ///         dm[(1, 0)] == 0.0 && dm[(1, 1)] == 1.0 && dm[(1, 2)] == 0.0);
        /// ```
        #[inline]
        pub fn identity($($args: usize,)*) -> Self
            where T: Zero + One {
            Self::identity_generic($($gargs),* )
        }

        /// Creates a matrix filled with its diagonal filled with `elt` and all other
        /// components set to zero.
        ///
        /// # Example
        /// ```
        /// # use nalgebra::{Matrix2x3, DMatrix};
        /// # use std::iter;
        ///
        /// let m = Matrix2x3::from_diagonal_element(5.0);
        /// // The two additional arguments represent the matrix dimensions.
        /// let dm = DMatrix::from_diagonal_element(2, 3, 5.0);
        ///
        /// assert!(m.m11 == 5.0 && m.m12 == 0.0 && m.m13 == 0.0 &&
        ///         m.m21 == 0.0 && m.m22 == 5.0 && m.m23 == 0.0);
        /// assert!(dm[(0, 0)] == 5.0 && dm[(0, 1)] == 0.0 && dm[(0, 2)] == 0.0 &&
        ///         dm[(1, 0)] == 0.0 && dm[(1, 1)] == 5.0 && dm[(1, 2)] == 0.0);
        /// ```
        #[inline]
        pub fn from_diagonal_element($($args: usize,)* elt: T) -> Self
            where T: Zero + One {
            Self::from_diagonal_element_generic($($gargs, )* elt)
        }

        /// Creates a new matrix that may be rectangular. The first `elts.len()` diagonal
        /// elements are filled with the content of `elts`. Others are set to 0.
        ///
        /// Panics if `elts.len()` is larger than the minimum among `nrows` and `ncols`.
        ///
        /// # Example
        /// ```
        /// # use nalgebra::{Matrix3, DMatrix};
        /// # use std::iter;
        ///
        /// let m = Matrix3::from_partial_diagonal(&[1.0, 2.0]);
        /// // The two additional arguments represent the matrix dimensions.
        /// let dm = DMatrix::from_partial_diagonal(3, 3, &[1.0, 2.0]);
        ///
        /// assert!(m.m11 == 1.0 && m.m12 == 0.0 && m.m13 == 0.0 &&
        ///         m.m21 == 0.0 && m.m22 == 2.0 && m.m23 == 0.0 &&
        ///         m.m31 == 0.0 && m.m32 == 0.0 && m.m33 == 0.0);
        /// assert!(dm[(0, 0)] == 1.0 && dm[(0, 1)] == 0.0 && dm[(0, 2)] == 0.0 &&
        ///         dm[(1, 0)] == 0.0 && dm[(1, 1)] == 2.0 && dm[(1, 2)] == 0.0 &&
        ///         dm[(2, 0)] == 0.0 && dm[(2, 1)] == 0.0 && dm[(2, 2)] == 0.0);
        /// ```
        #[inline]
        pub fn from_partial_diagonal($($args: usize,)* elts: &[T]) -> Self
            where T: Zero {
            Self::from_partial_diagonal_generic($($gargs, )* elts)
        }

        /// Creates a matrix or vector filled with random values from the given distribution.
        #[inline]
        #[cfg(feature = "rand-no-std")]
        pub fn from_distribution<Distr: Distribution<T> + ?Sized, G: Rng + ?Sized>(
            $($args: usize,)*
            distribution: &Distr,
            rng: &mut G,
        ) -> Self {
            Self::from_distribution_generic($($gargs, )* distribution, rng)
        }

        /// Creates a matrix filled with random values.
        #[inline]
        #[cfg(feature = "rand")]
        pub fn new_random($($args: usize),*) -> Self
            where StandardUniform: Distribution<T> {
            Self::new_random_generic($($gargs),*)
        }
    }
);

/// # Constructors of statically-sized vectors or statically-sized matrices
impl<T: Scalar, R: DimName, C: DimName> OMatrix<T, R, C>
where
    DefaultAllocator: Allocator<R, C>,
{
    // TODO: this is not very pretty. We could find a better call syntax.
    impl_constructors!(R, C;                         // Arguments for Matrix<T, ..., S>
    => R: DimName, => C: DimName; // Type parameters for impl<T, ..., S>
    R::name(), C::name();         // Arguments for `_generic` constructors.
    ); // Arguments for non-generic constructors.
}

/// # Constructors of matrices with a dynamic number of columns
impl<T: Scalar, R: DimName> OMatrix<T, R, Dyn>
where
    DefaultAllocator: Allocator<R, Dyn>,
{
    impl_constructors!(R, Dyn;
                   => R: DimName;
                   R::name(), Dyn(ncols);
                   ncols);
}

/// # Constructors of dynamic vectors and matrices with a dynamic number of rows
impl<T: Scalar, C: DimName> OMatrix<T, Dyn, C>
where
    DefaultAllocator: Allocator<Dyn, C>,
{
    impl_constructors!(Dyn, C;
                   => C: DimName;
                   Dyn(nrows), C::name();
                   nrows);
}

/// # Constructors of fully dynamic matrices
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar> OMatrix<T, Dyn, Dyn>
where
    DefaultAllocator: Allocator<Dyn, Dyn>,
{
    impl_constructors!(Dyn, Dyn;
                   ;
                   Dyn(nrows), Dyn(ncols);
                   nrows, ncols);
}

/*
 *
 * Constructors that don't necessarily require all dimensions
 * to be specified when one dimension is already known.
 *
 */
macro_rules! impl_constructors_from_data(
    ($data: ident; $($Dims: ty),*; $(=> $DimIdent: ident: $DimBound: ident),*; $($gargs: expr_2021),*; $($args: ident),*) => {
        impl<T: Scalar, $($DimIdent: $DimBound, )*> OMatrix<T $(, $Dims)*>
        where DefaultAllocator: Allocator<$($Dims),*> {
            /// Creates a matrix with its elements filled with the components provided by a slice
            /// in row-major order.
            ///
            /// The order of elements in the slice must follow the usual mathematic writing, i.e.,
            /// row-by-row.
            ///
            /// # Example
            /// ```
            /// # use nalgebra::{Matrix2x3, Vector3, DVector, DMatrix};
            /// # use std::iter;
            ///
            /// let v = Vector3::from_row_slice(&[0, 1, 2]);
            /// // The additional argument represents the vector dimension.
            /// let dv = DVector::from_row_slice(&[0, 1, 2]);
            /// let m = Matrix2x3::from_row_slice(&[0, 1, 2, 3, 4, 5]);
            /// // The two additional arguments represent the matrix dimensions.
            /// let dm = DMatrix::from_row_slice(2, 3, &[0, 1, 2, 3, 4, 5]);
            ///
            /// assert!(v.x == 0 && v.y == 1 && v.z == 2);
            /// assert!(dv[0] == 0 && dv[1] == 1 && dv[2] == 2);
            /// assert!(m.m11 == 0 && m.m12 == 1 && m.m13 == 2 &&
            ///         m.m21 == 3 && m.m22 == 4 && m.m23 == 5);
            /// assert!(dm[(0, 0)] == 0 && dm[(0, 1)] == 1 && dm[(0, 2)] == 2 &&
            ///         dm[(1, 0)] == 3 && dm[(1, 1)] == 4 && dm[(1, 2)] == 5);
            /// ```
            #[inline]
            pub fn from_row_slice($($args: usize,)* $data: &[T]) -> Self {
                Self::from_row_slice_generic($($gargs, )* $data)
            }

            /// Creates a matrix with its elements filled with the components provided by a slice
            /// in column-major order.
            ///
            /// # Example
            /// ```
            /// # use nalgebra::{Matrix2x3, Vector3, DVector, DMatrix};
            /// # use std::iter;
            ///
            /// let v = Vector3::from_column_slice(&[0, 1, 2]);
            /// // The additional argument represents the vector dimension.
            /// let dv = DVector::from_column_slice(&[0, 1, 2]);
            /// let m = Matrix2x3::from_column_slice(&[0, 1, 2, 3, 4, 5]);
            /// // The two additional arguments represent the matrix dimensions.
            /// let dm = DMatrix::from_column_slice(2, 3, &[0, 1, 2, 3, 4, 5]);
            ///
            /// assert!(v.x == 0 && v.y == 1 && v.z == 2);
            /// assert!(dv[0] == 0 && dv[1] == 1 && dv[2] == 2);
            /// assert!(m.m11 == 0 && m.m12 == 2 && m.m13 == 4 &&
            ///         m.m21 == 1 && m.m22 == 3 && m.m23 == 5);
            /// assert!(dm[(0, 0)] == 0 && dm[(0, 1)] == 2 && dm[(0, 2)] == 4 &&
            ///         dm[(1, 0)] == 1 && dm[(1, 1)] == 3 && dm[(1, 2)] == 5);
            /// ```
            #[inline]
            pub fn from_column_slice($($args: usize,)* $data: &[T]) -> Self {
                Self::from_column_slice_generic($($gargs, )* $data)
            }

            /// Creates a matrix backed by a given `Vec`.
            ///
            /// The output matrix is filled column-by-column.
            ///
            /// # Example
            /// ```
            /// # use nalgebra::{DMatrix, Matrix2x3};
            ///
            /// let m = Matrix2x3::from_vec(vec![0, 1, 2, 3, 4, 5]);
            ///
            /// assert!(m.m11 == 0 && m.m12 == 2 && m.m13 == 4 &&
            ///         m.m21 == 1 && m.m22 == 3 && m.m23 == 5);
            ///
            ///
            /// // The two additional arguments represent the matrix dimensions.
            /// let dm = DMatrix::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
            ///
            /// assert!(dm[(0, 0)] == 0 && dm[(0, 1)] == 2 && dm[(0, 2)] == 4 &&
            ///         dm[(1, 0)] == 1 && dm[(1, 1)] == 3 && dm[(1, 2)] == 5);
            /// ```
            #[inline]
            #[cfg(any(feature = "std", feature = "alloc"))]
            pub fn from_vec($($args: usize,)* $data: Vec<T>) -> Self {
                Self::from_vec_generic($($gargs, )* $data)
            }
        }
    }
);

// TODO: this is not very pretty. We could find a better call syntax.
impl_constructors_from_data!(data; R, C;                  // Arguments for Matrix<T, ..., S>
=> R: DimName, => C: DimName; // Type parameters for impl<T, ..., S>
R::name(), C::name();         // Arguments for `_generic` constructors.
); // Arguments for non-generic constructors.

impl_constructors_from_data!(data; R, Dyn;
=> R: DimName;
R::name(), Dyn(data.len() / R::DIM);
);

impl_constructors_from_data!(data; Dyn, C;
=> C: DimName;
Dyn(data.len() / C::DIM), C::name();
);

#[cfg(any(feature = "std", feature = "alloc"))]
impl_constructors_from_data!(data; Dyn, Dyn;
                            ;
                            Dyn(nrows), Dyn(ncols);
                            nrows, ncols);

/*
 *
 * Zero, One, Rand traits.
 *
 */
impl<T, R: DimName, C: DimName> Zero for OMatrix<T, R, C>
where
    T: Scalar + Zero + ClosedAddAssign,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn zero() -> Self {
        Self::from_element(T::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.iter().all(|e| e.is_zero())
    }
}

impl<T, D: DimName> One for OMatrix<T, D, D>
where
    T: Scalar + Zero + One + ClosedMulAssign + ClosedAddAssign,
    DefaultAllocator: Allocator<D, D>,
{
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

impl<T, R: DimName, C: DimName> Bounded for OMatrix<T, R, C>
where
    T: Scalar + Bounded,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn max_value() -> Self {
        Self::from_element(T::max_value())
    }

    #[inline]
    fn min_value() -> Self {
        Self::from_element(T::min_value())
    }
}

#[cfg(feature = "rand-no-std")]
impl<T: Scalar, R: Dim, C: Dim> Distribution<OMatrix<T, R, C>> for StandardUniform
where
    DefaultAllocator: Allocator<R, C>,
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<G: Rng + ?Sized>(&self, rng: &mut G) -> OMatrix<T, R, C> {
        let nrows = R::try_to_usize().unwrap_or_else(|| rng.random_range(0..10));
        let ncols = C::try_to_usize().unwrap_or_else(|| rng.random_range(0..10));

        OMatrix::from_fn_generic(R::from_usize(nrows), C::from_usize(ncols), |_, _| {
            rng.random()
        })
    }
}

#[cfg(feature = "arbitrary")]
impl<T, R, C> Arbitrary for OMatrix<T, R, C>
where
    R: Dim,
    C: Dim,
    T: Scalar + Arbitrary + Send,
    DefaultAllocator: Allocator<R, C>,
    Owned<T, R, C>: Clone + Send,
{
    #[inline]
    fn arbitrary(g: &mut Gen) -> Self {
        let nrows = R::try_to_usize().unwrap_or(usize::arbitrary(g) % 10);
        let ncols = C::try_to_usize().unwrap_or(usize::arbitrary(g) % 10);

        Self::from_fn_generic(R::from_usize(nrows), C::from_usize(ncols), |_, _| {
            T::arbitrary(g)
        })
    }
}

// TODO(specialization): faster impls possible for D≤4 (see rand_distr::{UnitCircle, UnitSphere})
#[cfg(feature = "rand")]
impl<T: crate::RealField, D: DimName> Distribution<Unit<OVector<T, D>>> for StandardUniform
where
    DefaultAllocator: Allocator<D>,
    rand_distr::StandardNormal: Distribution<T>,
{
    /// Generate a uniformly distributed random unit vector.
    #[inline]
    fn sample<G: Rng + ?Sized>(&self, rng: &mut G) -> Unit<OVector<T, D>> {
        Unit::new_normalize(OVector::from_distribution_generic(
            D::name(),
            Const::<1>,
            &rand_distr::StandardNormal,
            rng,
        ))
    }
}

/*
 *
 * Constructors for small matrices and vectors.
 *
 */

macro_rules! transpose_array(
    [$($a: ident),*;] => {
        [$([$a]),*]
    };
    [$($a: ident),*; $($b: ident),*;] => {
        [$([$a, $b]),*]
    };
    [$($a: ident),*; $($b: ident),*; $($c: ident),*;] => {
        [$([$a, $b, $c]),*]
    };
    [$($a: ident),*; $($b: ident),*; $($c: ident),*; $($d: ident),*;] => {
        [$([$a, $b, $c, $d]),*]
    };
    [$($a: ident),*; $($b: ident),*; $($c: ident),*; $($d: ident),*; $($e: ident),*;] => {
        [$([$a, $b, $c, $d, $e]),*]
    };
    [$($a: ident),*; $($b: ident),*; $($c: ident),*; $($d: ident),*; $($e: ident),*; $($f: ident),*;] => {
        [$([$a, $b, $c, $d, $e, $f]),*]
    };
);

macro_rules! componentwise_constructors_impl(
    ($($R: expr_2021, $C: expr_2021, [$($($args: ident),*);*] $(;)*)*) => {$(
        impl<T> Matrix<T, Const<$R>, Const<$C>, ArrayStorage<T, $R, $C>> {
            /// Initializes this matrix from its components.
            #[inline]
            #[allow(clippy::too_many_arguments)]
            pub const fn new($($($args: T),*),*) -> Self {
                unsafe {
                    Self::from_data_statically_unchecked(
                        ArrayStorage(
                            transpose_array![
                                $(
                                    $($args),*
                                ;)*
                            ]
                        )
                    )
                }
            }
        }
    )*}
);

componentwise_constructors_impl!(
    /*
     * Square matrices 1 .. 6.
     */
    2, 2, [m11, m12;
           m21, m22];
    3, 3, [m11, m12, m13;
          m21, m22, m23;
          m31, m32, m33];
    4, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24;
          m31, m32, m33, m34;
          m41, m42, m43, m44];
    5, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25;
          m31, m32, m33, m34, m35;
          m41, m42, m43, m44, m45;
          m51, m52, m53, m54, m55];
    6, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26;
          m31, m32, m33, m34, m35, m36;
          m41, m42, m43, m44, m45, m46;
          m51, m52, m53, m54, m55, m56;
          m61, m62, m63, m64, m65, m66];

    /*
     * Rectangular matrices with 2 rows.
     */
    2, 3, [m11, m12, m13;
          m21, m22, m23];
    2, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24];
    2, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25];
    2, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26];

    /*
     * Rectangular matrices with 3 rows.
     */
    3, 2, [m11, m12;
          m21, m22;
          m31, m32];
    3, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24;
          m31, m32, m33, m34];
    3, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25;
          m31, m32, m33, m34, m35];
    3, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26;
          m31, m32, m33, m34, m35, m36];

    /*
     * Rectangular matrices with 4 rows.
     */
    4, 2, [m11, m12;
          m21, m22;
          m31, m32;
          m41, m42];
    4, 3, [m11, m12, m13;
          m21, m22, m23;
          m31, m32, m33;
          m41, m42, m43];
    4, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25;
          m31, m32, m33, m34, m35;
          m41, m42, m43, m44, m45];
    4, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26;
          m31, m32, m33, m34, m35, m36;
          m41, m42, m43, m44, m45, m46];

    /*
     * Rectangular matrices with 5 rows.
     */
    5, 2, [m11, m12;
          m21, m22;
          m31, m32;
          m41, m42;
          m51, m52];
    5, 3, [m11, m12, m13;
          m21, m22, m23;
          m31, m32, m33;
          m41, m42, m43;
          m51, m52, m53];
    5, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24;
          m31, m32, m33, m34;
          m41, m42, m43, m44;
          m51, m52, m53, m54];
    5, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26;
          m31, m32, m33, m34, m35, m36;
          m41, m42, m43, m44, m45, m46;
          m51, m52, m53, m54, m55, m56];

    /*
     * Rectangular matrices with 6 rows.
     */
    6, 2, [m11, m12;
          m21, m22;
          m31, m32;
          m41, m42;
          m51, m52;
          m61, m62];
    6, 3, [m11, m12, m13;
          m21, m22, m23;
          m31, m32, m33;
          m41, m42, m43;
          m51, m52, m53;
          m61, m62, m63];
    6, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24;
          m31, m32, m33, m34;
          m41, m42, m43, m44;
          m51, m52, m53, m54;
          m61, m62, m63, m64];
    6, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25;
          m31, m32, m33, m34, m35;
          m41, m42, m43, m44, m45;
          m51, m52, m53, m54, m55;
          m61, m62, m63, m64, m65];

    /*
     * Row vectors 1 .. 6.
     */
    1, 1, [x];
    1, 2, [x, y];
    1, 3, [x, y, z];
    1, 4, [x, y, z, w];
    1, 5, [x, y, z, w, a];
    1, 6, [x, y, z, w, a, b];

    /*
     * Column vectors 1 .. 6.
     */
    2, 1, [x; y];
    3, 1, [x; y; z];
    4, 1, [x; y; z; w];
    5, 1, [x; y; z; w; a];
    6, 1, [x; y; z; w; a; b];
);

/*
 *
 * Axis constructors.
 *
 */
impl<T, R: DimName> OVector<T, R>
where
    R: ToTypenum,
    T: Scalar + Zero + One,
    DefaultAllocator: Allocator<R>,
{
    /// The column vector with `val` as its i-th component.
    #[inline]
    pub fn ith(i: usize, val: T) -> Self {
        let mut res = Self::zeros();
        res[i] = val;
        res
    }

    /// The column unit vector with `T::one()` as its i-th component.
    #[inline]
    pub fn ith_axis(i: usize) -> Unit<Self> {
        Unit::new_unchecked(Self::ith(i, T::one()))
    }

    /// The column vector with a 1 as its first component, and zero elsewhere.
    #[inline]
    pub fn x() -> Self
    where
        R::Typenum: Cmp<typenum::U0, Output = Greater>,
    {
        let mut res = Self::zeros();
        unsafe {
            *res.vget_unchecked_mut(0) = T::one();
        }

        res
    }

    /// The column vector with a 1 as its second component, and zero elsewhere.
    #[inline]
    pub fn y() -> Self
    where
        R::Typenum: Cmp<typenum::U1, Output = Greater>,
    {
        let mut res = Self::zeros();
        unsafe {
            *res.vget_unchecked_mut(1) = T::one();
        }

        res
    }

    /// The column vector with a 1 as its third component, and zero elsewhere.
    #[inline]
    pub fn z() -> Self
    where
        R::Typenum: Cmp<typenum::U2, Output = Greater>,
    {
        let mut res = Self::zeros();
        unsafe {
            *res.vget_unchecked_mut(2) = T::one();
        }

        res
    }

    /// The column vector with a 1 as its fourth component, and zero elsewhere.
    #[inline]
    pub fn w() -> Self
    where
        R::Typenum: Cmp<typenum::U3, Output = Greater>,
    {
        let mut res = Self::zeros();
        unsafe {
            *res.vget_unchecked_mut(3) = T::one();
        }

        res
    }

    /// The column vector with a 1 as its fifth component, and zero elsewhere.
    #[inline]
    pub fn a() -> Self
    where
        R::Typenum: Cmp<typenum::U4, Output = Greater>,
    {
        let mut res = Self::zeros();
        unsafe {
            *res.vget_unchecked_mut(4) = T::one();
        }

        res
    }

    /// The column vector with a 1 as its sixth component, and zero elsewhere.
    #[inline]
    pub fn b() -> Self
    where
        R::Typenum: Cmp<typenum::U5, Output = Greater>,
    {
        let mut res = Self::zeros();
        unsafe {
            *res.vget_unchecked_mut(5) = T::one();
        }

        res
    }

    /// The unit column vector with a 1 as its first component, and zero elsewhere.
    #[inline]
    pub fn x_axis() -> Unit<Self>
    where
        R::Typenum: Cmp<typenum::U0, Output = Greater>,
    {
        Unit::new_unchecked(Self::x())
    }

    /// The unit column vector with a 1 as its second component, and zero elsewhere.
    #[inline]
    pub fn y_axis() -> Unit<Self>
    where
        R::Typenum: Cmp<typenum::U1, Output = Greater>,
    {
        Unit::new_unchecked(Self::y())
    }

    /// The unit column vector with a 1 as its third component, and zero elsewhere.
    #[inline]
    pub fn z_axis() -> Unit<Self>
    where
        R::Typenum: Cmp<typenum::U2, Output = Greater>,
    {
        Unit::new_unchecked(Self::z())
    }

    /// The unit column vector with a 1 as its fourth component, and zero elsewhere.
    #[inline]
    pub fn w_axis() -> Unit<Self>
    where
        R::Typenum: Cmp<typenum::U3, Output = Greater>,
    {
        Unit::new_unchecked(Self::w())
    }

    /// The unit column vector with a 1 as its fifth component, and zero elsewhere.
    #[inline]
    pub fn a_axis() -> Unit<Self>
    where
        R::Typenum: Cmp<typenum::U4, Output = Greater>,
    {
        Unit::new_unchecked(Self::a())
    }

    /// The unit column vector with a 1 as its sixth component, and zero elsewhere.
    #[inline]
    pub fn b_axis() -> Unit<Self>
    where
        R::Typenum: Cmp<typenum::U5, Output = Greater>,
    {
        Unit::new_unchecked(Self::b())
    }
}
