//! Indexing
#![allow(clippy::reversed_empty_ranges)]

use crate::base::storage::{RawStorage, RawStorageMut};
use crate::base::{
    Const, Dim, DimDiff, DimName, DimSub, Dyn, Matrix, MatrixView, MatrixViewMut, Scalar, U1,
};

use std::ops;

// N.B.: Not a public trait!
trait DimRange<D: Dim> {
    /// The number of elements indexed by this range.
    type Length: Dim;

    /// The lower bound of the range, inclusive.
    fn lower(&self, dimension: D) -> usize;

    /// The number of elements included in the range.
    fn length(&self, dimension: D) -> Self::Length;

    /// Produces true if `Self` is contained within `dimension`.
    fn contained_by(&self, dimension: D) -> bool;
}

impl<D: Dim> DimRange<D> for usize {
    type Length = U1;

    #[inline(always)]
    fn lower(&self, _: D) -> usize {
        *self
    }

    #[inline(always)]
    fn length(&self, _: D) -> Self::Length {
        Const::<1>
    }

    #[inline(always)]
    fn contained_by(&self, dimension: D) -> bool {
        *self < dimension.value()
    }
}

#[test]
fn dimrange_usize() {
    assert!(!DimRange::contained_by(&0, Const::<0>));
    assert!(DimRange::contained_by(&0, Const::<1>));
}

impl<D: Dim> DimRange<D> for ops::Range<usize> {
    type Length = Dyn;

    #[inline(always)]
    fn lower(&self, _: D) -> usize {
        self.start
    }

    #[inline(always)]
    fn length(&self, _: D) -> Self::Length {
        Dyn(self.end.saturating_sub(self.start))
    }

    #[inline(always)]
    fn contained_by(&self, dimension: D) -> bool {
        (self.start < dimension.value()) && (self.end <= dimension.value())
    }
}

#[test]
fn dimrange_range_usize() {
    assert!(!DimRange::contained_by(&(0..0), Const::<0>));
    assert!(!DimRange::contained_by(&(0..1), Const::<0>));
    assert!(DimRange::contained_by(&(0..1), Const::<1>));
    assert!(DimRange::contained_by(
        &((usize::MAX - 1)..usize::MAX),
        Dyn(usize::MAX)
    ));
    assert_eq!(
        DimRange::length(&((usize::MAX - 1)..usize::MAX), Dyn(usize::MAX)),
        Dyn(1)
    );
    assert_eq!(
        DimRange::length(&(usize::MAX..(usize::MAX - 1)), Dyn(usize::MAX)),
        Dyn(0)
    );
    assert_eq!(
        DimRange::length(&(usize::MAX..usize::MAX), Dyn(usize::MAX)),
        Dyn(0)
    );
}

impl<D: Dim> DimRange<D> for ops::RangeFrom<usize> {
    type Length = Dyn;

    #[inline(always)]
    fn lower(&self, _: D) -> usize {
        self.start
    }

    #[inline(always)]
    fn length(&self, dimension: D) -> Self::Length {
        (self.start..dimension.value()).length(dimension)
    }

    #[inline(always)]
    fn contained_by(&self, dimension: D) -> bool {
        self.start < dimension.value()
    }
}

#[test]
fn dimrange_rangefrom_usize() {
    assert!(!DimRange::contained_by(&(0..), Const::<0>));
    assert!(!DimRange::contained_by(&(0..), Const::<0>));
    assert!(DimRange::contained_by(&(0..), Const::<1>));
    assert!(DimRange::contained_by(
        &((usize::MAX - 1)..),
        Dyn(usize::MAX)
    ));
    assert_eq!(
        DimRange::length(&((usize::MAX - 1)..), Dyn(usize::MAX)),
        Dyn(1)
    );
    assert_eq!(DimRange::length(&(usize::MAX..), Dyn(usize::MAX)), Dyn(0));
}

impl<D: Dim, T: Dim> DimRange<D> for ops::RangeFrom<T>
where
    D: DimSub<T>,
{
    type Length = DimDiff<D, T>;

    #[inline(always)]
    fn lower(&self, _: D) -> usize {
        self.start.value()
    }

    #[inline(always)]
    fn length(&self, dimension: D) -> Self::Length {
        dimension.sub(self.start)
    }

    #[inline(always)]
    fn contained_by(&self, _: D) -> bool {
        true
    }
}

#[test]
fn dimrange_rangefrom_dimname() {
    assert_eq!(DimRange::length(&(Const::<1>..), Const::<5>), Const::<4>);
}

impl<D: Dim> DimRange<D> for ops::RangeFull {
    type Length = D;

    #[inline(always)]
    fn lower(&self, _: D) -> usize {
        0
    }

    #[inline(always)]
    fn length(&self, dimension: D) -> Self::Length {
        dimension
    }

    #[inline(always)]
    fn contained_by(&self, _: D) -> bool {
        true
    }
}

#[test]
fn dimrange_rangefull() {
    assert!(DimRange::contained_by(&(..), Const::<0>));
    assert_eq!(DimRange::length(&(..), Const::<1>), Const::<1>);
}

impl<D: Dim> DimRange<D> for ops::RangeInclusive<usize> {
    type Length = Dyn;

    #[inline(always)]
    fn lower(&self, _: D) -> usize {
        *self.start()
    }

    #[inline(always)]
    fn length(&self, _: D) -> Self::Length {
        Dyn(if self.end() < self.start() {
            0
        } else {
            self.end().wrapping_sub(self.start().wrapping_sub(1))
        })
    }

    #[inline(always)]
    fn contained_by(&self, dimension: D) -> bool {
        (*self.start() < dimension.value()) && (*self.end() < dimension.value())
    }
}

#[test]
fn dimrange_rangeinclusive_usize() {
    assert!(!DimRange::contained_by(&(0..=0), Const::<0>));
    assert!(DimRange::contained_by(&(0..=0), Const::<1>));
    assert!(!DimRange::contained_by(
        &(usize::MAX..=usize::MAX),
        Dyn(usize::MAX)
    ));
    assert!(!DimRange::contained_by(
        &((usize::MAX - 1)..=usize::MAX),
        Dyn(usize::MAX)
    ));
    assert!(DimRange::contained_by(
        &((usize::MAX - 1)..=(usize::MAX - 1)),
        Dyn(usize::MAX)
    ));
    assert_eq!(DimRange::length(&(0..=0), Const::<1>), Dyn(1));
    assert_eq!(
        DimRange::length(&((usize::MAX - 1)..=usize::MAX), Dyn(usize::MAX)),
        Dyn(2)
    );
    assert_eq!(
        DimRange::length(&(usize::MAX..=(usize::MAX - 1)), Dyn(usize::MAX)),
        Dyn(0)
    );
    assert_eq!(
        DimRange::length(&(usize::MAX..=usize::MAX), Dyn(usize::MAX)),
        Dyn(1)
    );
}

impl<D: Dim> DimRange<D> for ops::RangeTo<usize> {
    type Length = Dyn;

    #[inline(always)]
    fn lower(&self, _: D) -> usize {
        0
    }

    #[inline(always)]
    fn length(&self, _: D) -> Self::Length {
        Dyn(self.end)
    }

    #[inline(always)]
    fn contained_by(&self, dimension: D) -> bool {
        self.end <= dimension.value()
    }
}

#[test]
fn dimrange_rangeto_usize() {
    assert!(DimRange::contained_by(&(..0), Const::<0>));
    assert!(!DimRange::contained_by(&(..1), Const::<0>));
    assert!(DimRange::contained_by(&(..0), Const::<1>));
    assert!(DimRange::contained_by(
        &(..(usize::MAX - 1)),
        Dyn(usize::MAX)
    ));
    assert_eq!(
        DimRange::length(&(..(usize::MAX - 1)), Dyn(usize::MAX)),
        Dyn(usize::MAX - 1)
    );
    assert_eq!(
        DimRange::length(&(..usize::MAX), Dyn(usize::MAX)),
        Dyn(usize::MAX)
    );
}

impl<D: Dim> DimRange<D> for ops::RangeToInclusive<usize> {
    type Length = Dyn;

    #[inline(always)]
    fn lower(&self, _: D) -> usize {
        0
    }

    #[inline(always)]
    fn length(&self, _: D) -> Self::Length {
        Dyn(self.end + 1)
    }

    #[inline(always)]
    fn contained_by(&self, dimension: D) -> bool {
        self.end < dimension.value()
    }
}

#[test]
fn dimrange_rangetoinclusive_usize() {
    assert!(!DimRange::contained_by(&(..=0), Const::<0>));
    assert!(!DimRange::contained_by(&(..=1), Const::<0>));
    assert!(DimRange::contained_by(&(..=0), Const::<1>));
    assert!(!DimRange::contained_by(&(..=(usize::MAX)), Dyn(usize::MAX)));
    assert!(DimRange::contained_by(
        &(..=(usize::MAX - 1)),
        Dyn(usize::MAX)
    ));
    assert_eq!(
        DimRange::length(&(..=(usize::MAX - 1)), Dyn(usize::MAX)),
        Dyn(usize::MAX)
    );
}

/// A helper trait used for indexing operations.
pub trait MatrixIndex<'a, T, R: Dim, C: Dim, S: RawStorage<T, R, C>>: Sized {
    /// The output type returned by methods.
    type Output: 'a;

    /// Produces true if the given matrix is contained by this index.
    #[doc(hidden)]
    fn contained_by(&self, matrix: &Matrix<T, R, C, S>) -> bool;

    /// Produces a shared view of the data at this location if in bounds,
    /// or `None`, otherwise.
    #[doc(hidden)]
    #[inline(always)]
    fn get(self, matrix: &'a Matrix<T, R, C, S>) -> Option<Self::Output> {
        if self.contained_by(matrix) {
            Some(unsafe { self.get_unchecked(matrix) })
        } else {
            None
        }
    }

    /// Produces a shared view of the data at this location if in bounds
    /// without any bounds checking.
    #[doc(hidden)]
    unsafe fn get_unchecked(self, matrix: &'a Matrix<T, R, C, S>) -> Self::Output;

    /// Produces a shared view to the data at this location, or panics
    /// if out of bounds.
    #[doc(hidden)]
    #[inline(always)]
    fn index(self, matrix: &'a Matrix<T, R, C, S>) -> Self::Output {
        self.get(matrix).expect("Index out of bounds.")
    }
}

/// A helper trait used for indexing operations.
pub trait MatrixIndexMut<'a, T, R: Dim, C: Dim, S: RawStorageMut<T, R, C>>:
    MatrixIndex<'a, T, R, C, S>
{
    /// The output type returned by methods.
    type OutputMut: 'a;

    /// Produces a mutable view of the data at this location, without
    /// performing any bounds checking.
    #[doc(hidden)]
    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T, R, C, S>) -> Self::OutputMut;

    /// Produces a mutable view of the data at this location, if in
    /// bounds.
    #[doc(hidden)]
    #[inline(always)]
    fn get_mut(self, matrix: &'a mut Matrix<T, R, C, S>) -> Option<Self::OutputMut> {
        if self.contained_by(matrix) {
            Some(unsafe { self.get_unchecked_mut(matrix) })
        } else {
            None
        }
    }

    /// Produces a mutable view of the data at this location, or panics
    /// if out of bounds.
    #[doc(hidden)]
    #[inline(always)]
    fn index_mut(self, matrix: &'a mut Matrix<T, R, C, S>) -> Self::OutputMut {
        self.get_mut(matrix).expect("Index out of bounds.")
    }
}

/// # Views based on ranges
/// ## Indices to Individual Elements
/// ### Two-Dimensional Indices
/// ```
/// # use nalgebra::*;
/// let matrix = Matrix2::new(0, 2,
///                           1, 3);
///
/// assert_eq!(matrix.index((0, 0)), &0);
/// assert_eq!(matrix.index((1, 0)), &1);
/// assert_eq!(matrix.index((0, 1)), &2);
/// assert_eq!(matrix.index((1, 1)), &3);
/// ```
///
/// ### Linear Address Indexing
/// ```
/// # use nalgebra::*;
/// let matrix = Matrix2::new(0, 2,
///                           1, 3);
///
/// assert_eq!(matrix.get(0), Some(&0));
/// assert_eq!(matrix.get(1), Some(&1));
/// assert_eq!(matrix.get(2), Some(&2));
/// assert_eq!(matrix.get(3), Some(&3));
/// ```
///
/// ## Indices to Individual Rows and Columns
/// ### Index to a Row
/// ```
/// # use nalgebra::*;
/// let matrix = Matrix2::new(0, 2,
///                           1, 3);
///
/// assert!(matrix.index((0, ..))
///     .eq(&Matrix1x2::new(0, 2)));
/// ```
///
/// ### Index to a Column
/// ```
/// # use nalgebra::*;
/// let matrix = Matrix2::new(0, 2,
///                           1, 3);
///
/// assert!(matrix.index((.., 0))
///     .eq(&Matrix2x1::new(0,
///                         1)));
/// ```
///
/// ## Indices to Parts of Individual Rows and Columns
/// ### Index to a Partial Row
/// ```
/// # use nalgebra::*;
/// let matrix = Matrix3::new(0, 3, 6,
///                           1, 4, 7,
///                           2, 5, 8);
///
/// assert!(matrix.index((0, ..2))
///     .eq(&Matrix1x2::new(0, 3)));
/// ```
///
/// ### Index to a Partial Column
/// ```
/// # use nalgebra::*;
/// let matrix = Matrix3::new(0, 3, 6,
///                           1, 4, 7,
///                           2, 5, 8);
///
/// assert!(matrix.index((..2, 0))
///     .eq(&Matrix2x1::new(0,
///                         1)));
///
/// assert!(matrix.index((Const::<1>.., 0))
///     .eq(&Matrix2x1::new(1,
///                         2)));
/// ```
/// ## Indices to Ranges of Rows and Columns
/// ### Index to a Range of Rows
/// ```
/// # use nalgebra::*;
/// let matrix = Matrix3::new(0, 3, 6,
///                           1, 4, 7,
///                           2, 5, 8);
///
/// assert!(matrix.index((1..3, ..))
///     .eq(&Matrix2x3::new(1, 4, 7,
///                         2, 5, 8)));
/// ```
/// ### Index to a Range of Columns
/// ```
/// # use nalgebra::*;
/// let matrix = Matrix3::new(0, 3, 6,
///                           1, 4, 7,
///                           2, 5, 8);
///
/// assert!(matrix.index((.., 1..3))
///     .eq(&Matrix3x2::new(3, 6,
///                         4, 7,
///                         5, 8)));
/// ```
impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S> {
    /// Produces a view of the data at the given index, or
    /// `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get<'a, I>(&'a self, index: I) -> Option<I::Output>
    where
        I: MatrixIndex<'a, T, R, C, S>,
    {
        index.get(self)
    }

    /// Produces a mutable view of the data at the given index, or
    /// `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get_mut<'a, I>(&'a mut self, index: I) -> Option<I::OutputMut>
    where
        S: RawStorageMut<T, R, C>,
        I: MatrixIndexMut<'a, T, R, C, S>,
    {
        index.get_mut(self)
    }

    /// Produces a view of the data at the given index, or
    /// panics if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn index<'a, I>(&'a self, index: I) -> I::Output
    where
        I: MatrixIndex<'a, T, R, C, S>,
    {
        index.index(self)
    }

    /// Produces a mutable view of the data at the given index, or
    /// panics if the index is out of bounds.
    #[inline]
    pub fn index_mut<'a, I>(&'a mut self, index: I) -> I::OutputMut
    where
        S: RawStorageMut<T, R, C>,
        I: MatrixIndexMut<'a, T, R, C, S>,
    {
        index.index_mut(self)
    }

    /// Produces a view of the data at the given index, without doing
    /// any bounds checking.
    ///
    /// # Safety
    ///
    /// `index` must within bounds of the array.
    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked<'a, I>(&'a self, index: I) -> I::Output
    where
        I: MatrixIndex<'a, T, R, C, S>,
    {
        unsafe { index.get_unchecked(self) }
    }

    /// Returns a mutable view of the data at the given index, without doing
    /// any bounds checking.
    /// # Safety
    ///
    /// `index` must within bounds of the array.
    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked_mut<'a, I>(&'a mut self, index: I) -> I::OutputMut
    where
        S: RawStorageMut<T, R, C>,
        I: MatrixIndexMut<'a, T, R, C, S>,
    {
        unsafe { index.get_unchecked_mut(self) }
    }
}

// EXTRACT A SINGLE ELEMENT BY 1D LINEAR ADDRESS

impl<'a, T, R, C, S> MatrixIndex<'a, T, R, C, S> for usize
where
    T: Scalar,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    type Output = &'a T;

    #[doc(hidden)]
    #[inline(always)]
    fn contained_by(&self, matrix: &Matrix<T, R, C, S>) -> bool {
        *self < matrix.len()
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn get_unchecked(self, matrix: &'a Matrix<T, R, C, S>) -> Self::Output {
        unsafe {
            let nrows = matrix.shape().0;
            let row = self % nrows;
            let col = self / nrows;
            matrix.data.get_unchecked(row, col)
        }
    }
}

impl<'a, T, R, C, S> MatrixIndexMut<'a, T, R, C, S> for usize
where
    T: Scalar,
    R: Dim,
    C: Dim,
    S: RawStorageMut<T, R, C>,
{
    type OutputMut = &'a mut T;

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T, R, C, S>) -> Self::OutputMut
    where
        S: RawStorageMut<T, R, C>,
    {
        unsafe {
            let nrows = matrix.shape().0;
            let row = self % nrows;
            let col = self / nrows;
            matrix.data.get_unchecked_mut(row, col)
        }
    }
}

// EXTRACT A SINGLE ELEMENT BY 2D COORDINATES

impl<'a, T: 'a, R, C, S> MatrixIndex<'a, T, R, C, S> for (usize, usize)
where
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    type Output = &'a T;

    #[doc(hidden)]
    #[inline(always)]
    fn contained_by(&self, matrix: &Matrix<T, R, C, S>) -> bool {
        let (rows, cols) = self;
        let (nrows, ncols) = matrix.shape_generic();
        DimRange::contained_by(rows, nrows) && DimRange::contained_by(cols, ncols)
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn get_unchecked(self, matrix: &'a Matrix<T, R, C, S>) -> Self::Output {
        unsafe {
            let (row, col) = self;
            matrix.data.get_unchecked(row, col)
        }
    }
}

impl<'a, T: 'a, R, C, S> MatrixIndexMut<'a, T, R, C, S> for (usize, usize)
where
    R: Dim,
    C: Dim,
    S: RawStorageMut<T, R, C>,
{
    type OutputMut = &'a mut T;

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T, R, C, S>) -> Self::OutputMut
    where
        S: RawStorageMut<T, R, C>,
    {
        unsafe {
            let (row, col) = self;
            matrix.data.get_unchecked_mut(row, col)
        }
    }
}

macro_rules! impl_index_pair {
    (
      $R: ident,
      $C: ident,
      [<$($RTyP: ident : $RTyPB: ty,)*> usize => $ROut: ty
        $(where $RConstraintType: ty: $RConstraintBound: ident<$($RConstraintBoundParams: ty $( = $REqBound: ty )*),*>)*],
      [<$($CTyP: ident : $CTyPB: ty,)*> usize => $COut: ty
        $(where $CConstraintType: ty: $CConstraintBound: ident<$($CConstraintBoundParams: ty $( = $CEqBound: ty )*),*>)*]
    ) => {};

    (
      $R: ident,
      $C: ident,
      [<$($RTyP: ident: $RTyPB: tt),*> $RIdx: ty => $ROut: ty
        $(where $RConstraintType: ty: $RConstraintBound: ident $(<$($RConstraintBoundParams: ty $( = $REqBound: ty )*),*>)* )*],
      [<$($CTyP: ident: $CTyPB: tt),*> $CIdx: ty => $COut: ty
        $(where $CConstraintType: ty: $CConstraintBound: ident $(<$($CConstraintBoundParams: ty $( = $CEqBound: ty )*),*>)* )*]
    ) =>
    {
        impl<'a, T, $R, $C, S, $($RTyP : $RTyPB,)* $($CTyP : $CTyPB),*> MatrixIndex<'a, T, $R, $C, S> for ($RIdx, $CIdx)
        where
            T: Scalar,
            $R: Dim,
            $C: Dim,
            S: RawStorage<T, R, C>,
            $( $RConstraintType: $RConstraintBound $(<$( $RConstraintBoundParams $( = $REqBound )*),*>)* ,)*
            $( $CConstraintType: $CConstraintBound $(<$( $CConstraintBoundParams $( = $CEqBound )*),*>)* ),*
        {
            type Output = MatrixView<'a, T, $ROut, $COut, S::RStride, S::CStride>;

            #[doc(hidden)]
            #[inline(always)]
            fn contained_by(&self, matrix: &Matrix<T, $R, $C, S>) -> bool {
                let (rows, cols) = self;
                let (nrows, ncols) = matrix.shape_generic();
                DimRange::contained_by(rows, nrows) && DimRange::contained_by(cols, ncols)
            }

            #[doc(hidden)]
            #[inline(always)]
            unsafe fn get_unchecked(self, matrix: &'a Matrix<T, $R, $C, S>) -> Self::Output { unsafe {
                use crate::base::ViewStorage;

                let (rows, cols) = self;
                let (nrows, ncols) = matrix.shape_generic();

                let data =
                    ViewStorage::new_unchecked(&matrix.data,
                        (rows.lower(nrows),  cols.lower(ncols)),
                        (rows.length(nrows), cols.length(ncols)));

                Matrix::from_data_statically_unchecked(data)
            }}
        }

        impl<'a, T, $R, $C, S, $($RTyP : $RTyPB,)* $($CTyP : $CTyPB),*> MatrixIndexMut<'a, T, $R, $C, S> for ($RIdx, $CIdx)
        where
            T: Scalar,
            $R: Dim,
            $C: Dim,
            S: RawStorageMut<T, R, C>,
            $( $RConstraintType: $RConstraintBound $(<$( $RConstraintBoundParams $( = $REqBound )*),*>)* ,)*
            $( $CConstraintType: $CConstraintBound $(<$( $CConstraintBoundParams $( = $CEqBound )*),*>)* ),*
        {
            type OutputMut = MatrixViewMut<'a, T, $ROut, $COut, S::RStride, S::CStride>;

            #[doc(hidden)]
            #[inline(always)]
            unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T, $R, $C, S>) -> Self::OutputMut { unsafe {
                use crate::base::ViewStorageMut;

                let (rows, cols) = self;
                let (nrows, ncols) = matrix.shape_generic();

                let data =
                    ViewStorageMut::new_unchecked(&mut matrix.data,
                        (rows.lower(nrows),  cols.lower(ncols)),
                        (rows.length(nrows), cols.length(ncols)));

                Matrix::from_data_statically_unchecked(data)
            }}
        }
    }
}

macro_rules! impl_index_pairs {
    (index $R: ident with {} index $C: ident with {$($r: tt,)* }) => {};

    (index $R: ident with {$lh : tt, $($lt : tt,)*}
     index $C: ident with { $($r: tt,)* }) =>
    {
        $(
            impl_index_pair!{$R, $C, $lh, $r}
        )*
        impl_index_pairs!{index $R with {$($lt,)*} index $C with {$($r,)*}}
    }
}

impl_index_pairs! {
    index R with {
        [<> usize                         =>  U1],
        [<> ops::Range<usize>             =>  Dyn],
        [<> ops::RangeFrom<usize>         =>  Dyn],
        [<> ops::RangeFull                =>  R],
        [<> ops::RangeInclusive<usize>    =>  Dyn],
        [<> ops::RangeTo<usize>           =>  Dyn],
        [<> ops::RangeToInclusive<usize>  =>  Dyn],

        [<I: Dim> ops::RangeFrom<I>
          =>  DimDiff<R, I>
          where R: DimSub<I>],
    }
    index C with {
        [<> usize                         =>  U1],
        [<> ops::Range<usize>             =>  Dyn],
        [<> ops::RangeFrom<usize>         =>  Dyn],
        [<> ops::RangeFull                =>  C],
        [<> ops::RangeInclusive<usize>    =>  Dyn],
        [<> ops::RangeTo<usize>           =>  Dyn],
        [<> ops::RangeToInclusive<usize>  =>  Dyn],

        [<J: DimName> ops::RangeFrom<J>
          =>  DimDiff<C, J>
          where C: DimSub<J>],
    }
}
