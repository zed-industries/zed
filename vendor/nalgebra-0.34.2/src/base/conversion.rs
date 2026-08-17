#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
use simba::scalar::{SubsetOf, SupersetOf};
use std::borrow::{Borrow, BorrowMut};
use std::convert::{AsMut, AsRef, From, Into};

use simba::simd::{PrimitiveSimdValue, SimdValue};

use crate::base::allocator::{Allocator, SameShapeAllocator};
use crate::base::constraint::{SameNumberOfColumns, SameNumberOfRows, ShapeConstraint};
use crate::base::dimension::{
    Const, Dim, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16,
};
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::base::dimension::{DimName, Dyn};
use crate::base::iter::{MatrixIter, MatrixIterMut};
use crate::base::storage::{IsContiguous, RawStorage, RawStorageMut};
use crate::base::{
    ArrayStorage, DVectorView, DVectorViewMut, DefaultAllocator, Matrix, MatrixView, MatrixViewMut,
    OMatrix, Scalar,
};
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::base::{DVector, RowDVector, VecStorage};
use crate::base::{ViewStorage, ViewStorageMut};
use crate::constraint::DimEq;
use crate::{IsNotStaticOne, RowSVector, SMatrix, SVector, VectorView, VectorViewMut};
use std::mem::MaybeUninit;

// TODO: too bad this won't work for slice conversions.
impl<T1, T2, R1, C1, R2, C2> SubsetOf<OMatrix<T2, R2, C2>> for OMatrix<T1, R1, C1>
where
    R1: Dim,
    C1: Dim,
    R2: Dim,
    C2: Dim,
    T1: Scalar,
    T2: Scalar + SupersetOf<T1>,
    DefaultAllocator: Allocator<R2, C2> + Allocator<R1, C1> + SameShapeAllocator<R1, C1, R2, C2>,
    ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2>,
{
    #[inline]
    fn to_superset(&self) -> OMatrix<T2, R2, C2> {
        let (nrows, ncols) = self.shape();
        let nrows2 = R2::from_usize(nrows);
        let ncols2 = C2::from_usize(ncols);
        let mut res = Matrix::uninit(nrows2, ncols2);

        for i in 0..nrows {
            for j in 0..ncols {
                // Safety: all indices are in range.
                unsafe {
                    *res.get_unchecked_mut((i, j)) =
                        MaybeUninit::new(T2::from_subset(self.get_unchecked((i, j))));
                }
            }
        }

        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }

    #[inline]
    fn is_in_subset(m: &OMatrix<T2, R2, C2>) -> bool {
        m.iter().all(|e| e.is_in_subset())
    }

    #[inline]
    fn from_superset_unchecked(m: &OMatrix<T2, R2, C2>) -> Self {
        let (nrows2, ncols2) = m.shape();
        let nrows = R1::from_usize(nrows2);
        let ncols = C1::from_usize(ncols2);
        let mut res = Matrix::uninit(nrows, ncols);

        for i in 0..nrows2 {
            for j in 0..ncols2 {
                // Safety: all indices are in range.
                unsafe {
                    *res.get_unchecked_mut((i, j)) =
                        MaybeUninit::new(m.get_unchecked((i, j)).to_subset_unchecked())
                }
            }
        }

        unsafe { res.assume_init() }
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, S: RawStorage<T, R, C>> IntoIterator
    for &'a Matrix<T, R, C, S>
{
    type Item = &'a T;
    type IntoIter = MatrixIter<'a, T, R, C, S>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, RStride: Dim, CStride: Dim> IntoIterator
    for Matrix<T, R, C, ViewStorage<'a, T, R, C, RStride, CStride>>
{
    type Item = &'a T;
    type IntoIter = MatrixIter<'a, T, R, C, ViewStorage<'a, T, R, C, RStride, CStride>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        MatrixIter::new_owned(self.data)
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> IntoIterator
    for &'a mut Matrix<T, R, C, S>
{
    type Item = &'a mut T;
    type IntoIter = MatrixIterMut<'a, T, R, C, S>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, RStride: Dim, CStride: Dim> IntoIterator
    for Matrix<T, R, C, ViewStorageMut<'a, T, R, C, RStride, CStride>>
{
    type Item = &'a mut T;
    type IntoIter = MatrixIterMut<'a, T, R, C, ViewStorageMut<'a, T, R, C, RStride, CStride>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        MatrixIterMut::new_owned_mut(self.data)
    }
}

impl<T: Scalar, const D: usize> From<[T; D]> for SVector<T, D> {
    #[inline]
    fn from(arr: [T; D]) -> Self {
        unsafe { Self::from_data_statically_unchecked(ArrayStorage([arr; 1])) }
    }
}

impl<T: Scalar, const D: usize> From<SVector<T, D>> for [T; D] {
    #[inline]
    fn from(vec: SVector<T, D>) -> Self {
        // TODO: unfortunately, we must clone because we can move out of an array.
        vec.data.0[0].clone()
    }
}

impl<'a, T: Scalar, RStride: Dim, CStride: Dim, const D: usize>
    From<VectorView<'a, T, Const<D>, RStride, CStride>> for [T; D]
{
    #[inline]
    fn from(vec: VectorView<'a, T, Const<D>, RStride, CStride>) -> Self {
        vec.into_owned().into()
    }
}

impl<'a, T: Scalar, RStride: Dim, CStride: Dim, const D: usize>
    From<VectorViewMut<'a, T, Const<D>, RStride, CStride>> for [T; D]
{
    #[inline]
    fn from(vec: VectorViewMut<'a, T, Const<D>, RStride, CStride>) -> Self {
        vec.into_owned().into()
    }
}

impl<T: Scalar, const D: usize> From<[T; D]> for RowSVector<T, D>
where
    Const<D>: IsNotStaticOne,
{
    #[inline]
    fn from(arr: [T; D]) -> Self {
        SVector::<T, D>::from(arr).transpose()
    }
}

impl<T: Scalar, const D: usize> From<RowSVector<T, D>> for [T; D]
where
    Const<D>: IsNotStaticOne,
{
    #[inline]
    fn from(vec: RowSVector<T, D>) -> [T; D] {
        vec.transpose().into()
    }
}

macro_rules! impl_from_into_asref_1D(
    ($(($NRows: ident, $NCols: ident) => $SZ: expr_2021);* $(;)*) => {$(
        impl<T, S> AsRef<[T; $SZ]> for Matrix<T, $NRows, $NCols, S>
        where T: Scalar,
              S: RawStorage<T, $NRows, $NCols> + IsContiguous {
            #[inline]
            fn as_ref(&self) -> &[T; $SZ] {
                // Safety: this is OK thanks to the IsContiguous trait.
                unsafe {
                    &*(self.data.ptr() as *const [T; $SZ])
                }
            }
        }

        impl<T, S> AsMut<[T; $SZ]> for Matrix<T, $NRows, $NCols, S>
        where T: Scalar,
              S: RawStorageMut<T, $NRows, $NCols> + IsContiguous {
            #[inline]
            fn as_mut(&mut self) -> &mut [T; $SZ] {
                // Safety: this is OK thanks to the IsContiguous trait.
                unsafe {
                    &mut *(self.data.ptr_mut() as *mut [T; $SZ])
                }
            }
        }
    )*}
);

// Implement for vectors of dimension 1 .. 16.
impl_from_into_asref_1D!(
    // Row vectors.
    (U1, U1 ) => 1;  (U1, U2 ) => 2;  (U1, U3 ) => 3;  (U1, U4 ) => 4;
    (U1, U5 ) => 5;  (U1, U6 ) => 6;  (U1, U7 ) => 7;  (U1, U8 ) => 8;
    (U1, U9 ) => 9;  (U1, U10) => 10; (U1, U11) => 11; (U1, U12) => 12;
    (U1, U13) => 13; (U1, U14) => 14; (U1, U15) => 15; (U1, U16) => 16;

    // Column vectors.
                     (U2 , U1) => 2;  (U3 , U1) => 3;  (U4 , U1) => 4;
    (U5 , U1) => 5;  (U6 , U1) => 6;  (U7 , U1) => 7;  (U8 , U1) => 8;
    (U9 , U1) => 9;  (U10, U1) => 10; (U11, U1) => 11; (U12, U1) => 12;
    (U13, U1) => 13; (U14, U1) => 14; (U15, U1) => 15; (U16, U1) => 16;
);

impl<T: Scalar, const R: usize, const C: usize> From<[[T; R]; C]> for SMatrix<T, R, C> {
    #[inline]
    fn from(arr: [[T; R]; C]) -> Self {
        unsafe { Self::from_data_statically_unchecked(ArrayStorage(arr)) }
    }
}

impl<T: Scalar, const R: usize, const C: usize> From<SMatrix<T, R, C>> for [[T; R]; C] {
    #[inline]
    fn from(mat: SMatrix<T, R, C>) -> Self {
        mat.data.0
    }
}

impl<'a, T: Scalar, RStride: Dim, CStride: Dim, const R: usize, const C: usize>
    From<MatrixView<'a, T, Const<R>, Const<C>, RStride, CStride>> for [[T; R]; C]
{
    #[inline]
    fn from(mat: MatrixView<'a, T, Const<R>, Const<C>, RStride, CStride>) -> Self {
        mat.into_owned().into()
    }
}

impl<'a, T: Scalar, RStride: Dim, CStride: Dim, const R: usize, const C: usize>
    From<MatrixViewMut<'a, T, Const<R>, Const<C>, RStride, CStride>> for [[T; R]; C]
{
    #[inline]
    fn from(mat: MatrixViewMut<'a, T, Const<R>, Const<C>, RStride, CStride>) -> Self {
        mat.into_owned().into()
    }
}

macro_rules! impl_from_into_asref_borrow_2D(

    //does the impls on one case for either AsRef/AsMut and Borrow/BorrowMut
    (
        ($NRows: ty, $NCols: ty) => ($SZRows: expr_2021, $SZCols: expr_2021);
        $Ref:ident.$ref:ident(), $Mut:ident.$mut:ident()
    ) => {
        impl<T: Scalar, S> $Ref<[[T; $SZRows]; $SZCols]> for Matrix<T, $NRows, $NCols, S>
        where S: RawStorage<T, $NRows, $NCols> + IsContiguous {
            #[inline]
            fn $ref(&self) -> &[[T; $SZRows]; $SZCols] {
                // Safety: OK thanks to the IsContiguous trait.
                unsafe {
                    &*(self.data.ptr() as *const [[T; $SZRows]; $SZCols])
                }
            }
        }

        impl<T: Scalar, S> $Mut<[[T; $SZRows]; $SZCols]> for Matrix<T, $NRows, $NCols, S>
        where S: RawStorageMut<T, $NRows, $NCols> + IsContiguous {
            #[inline]
            fn $mut(&mut self) -> &mut [[T; $SZRows]; $SZCols] {
                // Safety: OK thanks to the IsContiguous trait.
                unsafe {
                    &mut *(self.data.ptr_mut() as *mut [[T; $SZRows]; $SZCols])
                }
            }
        }
    };

    //collects the mappings from typenum pairs to consts
    ($(($NRows: ty, $NCols: ty) => ($SZRows: expr_2021, $SZCols: expr_2021));* $(;)*) => {$(
        impl_from_into_asref_borrow_2D!(
            ($NRows, $NCols) => ($SZRows, $SZCols); AsRef.as_ref(), AsMut.as_mut()
        );
        impl_from_into_asref_borrow_2D!(
            ($NRows, $NCols) => ($SZRows, $SZCols); Borrow.borrow(), BorrowMut.borrow_mut()
        );
    )*}
);

// Implement for matrices with shape 2x2 .. 6x6.
impl_from_into_asref_borrow_2D!(
    (U2, U2) => (2, 2); (U2, U3) => (2, 3); (U2, U4) => (2, 4); (U2, U5) => (2, 5); (U2, U6) => (2, 6);
    (U3, U2) => (3, 2); (U3, U3) => (3, 3); (U3, U4) => (3, 4); (U3, U5) => (3, 5); (U3, U6) => (3, 6);
    (U4, U2) => (4, 2); (U4, U3) => (4, 3); (U4, U4) => (4, 4); (U4, U5) => (4, 5); (U4, U6) => (4, 6);
    (U5, U2) => (5, 2); (U5, U3) => (5, 3); (U5, U4) => (5, 4); (U5, U5) => (5, 5); (U5, U6) => (5, 6);
    (U6, U2) => (6, 2); (U6, U3) => (6, 3); (U6, U4) => (6, 4); (U6, U5) => (6, 5); (U6, U6) => (6, 6);
);

impl<'a, T, RStride, CStride, const R: usize, const C: usize>
    From<MatrixView<'a, T, Const<R>, Const<C>, RStride, CStride>>
    for Matrix<T, Const<R>, Const<C>, ArrayStorage<T, R, C>>
where
    T: Scalar,
    RStride: Dim,
    CStride: Dim,
{
    fn from(matrix_view: MatrixView<'a, T, Const<R>, Const<C>, RStride, CStride>) -> Self {
        matrix_view.into_owned()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T, C, RStride, CStride> From<MatrixView<'a, T, Dyn, C, RStride, CStride>>
    for Matrix<T, Dyn, C, VecStorage<T, Dyn, C>>
where
    T: Scalar,
    C: Dim,
    RStride: Dim,
    CStride: Dim,
{
    fn from(matrix_view: MatrixView<'a, T, Dyn, C, RStride, CStride>) -> Self {
        matrix_view.into_owned()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T, R, RStride, CStride> From<MatrixView<'a, T, R, Dyn, RStride, CStride>>
    for Matrix<T, R, Dyn, VecStorage<T, R, Dyn>>
where
    T: Scalar,
    R: DimName,
    RStride: Dim,
    CStride: Dim,
{
    fn from(matrix_view: MatrixView<'a, T, R, Dyn, RStride, CStride>) -> Self {
        matrix_view.into_owned()
    }
}

impl<'a, T, RStride, CStride, const R: usize, const C: usize>
    From<MatrixViewMut<'a, T, Const<R>, Const<C>, RStride, CStride>>
    for Matrix<T, Const<R>, Const<C>, ArrayStorage<T, R, C>>
where
    T: Scalar,
    RStride: Dim,
    CStride: Dim,
{
    fn from(matrix_view: MatrixViewMut<'a, T, Const<R>, Const<C>, RStride, CStride>) -> Self {
        matrix_view.into_owned()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T, C, RStride, CStride> From<MatrixViewMut<'a, T, Dyn, C, RStride, CStride>>
    for Matrix<T, Dyn, C, VecStorage<T, Dyn, C>>
where
    T: Scalar,
    C: Dim,
    RStride: Dim,
    CStride: Dim,
{
    fn from(matrix_view: MatrixViewMut<'a, T, Dyn, C, RStride, CStride>) -> Self {
        matrix_view.into_owned()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T, R, RStride, CStride> From<MatrixViewMut<'a, T, R, Dyn, RStride, CStride>>
    for Matrix<T, R, Dyn, VecStorage<T, R, Dyn>>
where
    T: Scalar,
    R: DimName,
    RStride: Dim,
    CStride: Dim,
{
    fn from(matrix_view: MatrixViewMut<'a, T, R, Dyn, RStride, CStride>) -> Self {
        matrix_view.into_owned()
    }
}

impl<'a, T, R, C, RView, CView, RStride, CStride, S> From<&'a Matrix<T, R, C, S>>
    for MatrixView<'a, T, RView, CView, RStride, CStride>
where
    R: Dim,
    C: Dim,
    RView: Dim,
    CView: Dim,
    RStride: Dim,
    CStride: Dim,
    S: RawStorage<T, R, C>,
    ShapeConstraint:
        DimEq<R, RView> + DimEq<C, CView> + DimEq<RStride, S::RStride> + DimEq<CStride, S::CStride>,
{
    fn from(m: &'a Matrix<T, R, C, S>) -> Self {
        let (row, col) = m.shape_generic();
        let rows_result = RView::from_usize(row.value());
        let cols_result = CView::from_usize(col.value());

        let (rstride, cstride) = m.strides();

        let rstride_result = RStride::from_usize(rstride);
        let cstride_result = CStride::from_usize(cstride);

        unsafe {
            let data = ViewStorage::from_raw_parts(
                m.data.ptr(),
                (rows_result, cols_result),
                (rstride_result, cstride_result),
            );
            Matrix::from_data_statically_unchecked(data)
        }
    }
}

impl<'a, T, R, C, RView, CView, RStride, CStride, S> From<&'a mut Matrix<T, R, C, S>>
    for MatrixView<'a, T, RView, CView, RStride, CStride>
where
    R: Dim,
    C: Dim,
    RView: Dim,
    CView: Dim,
    RStride: Dim,
    CStride: Dim,
    S: RawStorage<T, R, C>,
    ShapeConstraint:
        DimEq<R, RView> + DimEq<C, CView> + DimEq<RStride, S::RStride> + DimEq<CStride, S::CStride>,
{
    fn from(m: &'a mut Matrix<T, R, C, S>) -> Self {
        let (row, col) = m.shape_generic();
        let rows_result = RView::from_usize(row.value());
        let cols_result = CView::from_usize(col.value());

        let (rstride, cstride) = m.strides();

        let rstride_result = RStride::from_usize(rstride);
        let cstride_result = CStride::from_usize(cstride);

        unsafe {
            let data = ViewStorage::from_raw_parts(
                m.data.ptr(),
                (rows_result, cols_result),
                (rstride_result, cstride_result),
            );
            Matrix::from_data_statically_unchecked(data)
        }
    }
}

impl<'a, T, R, C, RView, CView, RStride, CStride, S> From<&'a mut Matrix<T, R, C, S>>
    for MatrixViewMut<'a, T, RView, CView, RStride, CStride>
where
    R: Dim,
    C: Dim,
    RView: Dim,
    CView: Dim,
    RStride: Dim,
    CStride: Dim,
    S: RawStorageMut<T, R, C>,
    ShapeConstraint:
        DimEq<R, RView> + DimEq<C, CView> + DimEq<RStride, S::RStride> + DimEq<CStride, S::CStride>,
{
    fn from(m: &'a mut Matrix<T, R, C, S>) -> Self {
        let (row, col) = m.shape_generic();
        let rows_result = RView::from_usize(row.value());
        let cols_result = CView::from_usize(col.value());

        let (rstride, cstride) = m.strides();

        let rstride_result = RStride::from_usize(rstride);
        let cstride_result = CStride::from_usize(cstride);

        unsafe {
            let data = ViewStorageMut::from_raw_parts(
                m.data.ptr_mut(),
                (rows_result, cols_result),
                (rstride_result, cstride_result),
            );
            Matrix::from_data_statically_unchecked(data)
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar> From<Vec<T>> for DVector<T> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar> From<Vec<T>> for RowDVector<T> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}

impl<'a, T: Scalar + Copy, R: Dim, C: Dim, S: RawStorage<T, R, C> + IsContiguous>
    From<&'a Matrix<T, R, C, S>> for &'a [T]
{
    #[inline]
    fn from(matrix: &'a Matrix<T, R, C, S>) -> Self {
        matrix.as_slice()
    }
}

impl<'a, T: Scalar + Copy, R: Dim, C: Dim, S: RawStorageMut<T, R, C> + IsContiguous>
    From<&'a mut Matrix<T, R, C, S>> for &'a mut [T]
{
    #[inline]
    fn from(matrix: &'a mut Matrix<T, R, C, S>) -> Self {
        matrix.as_mut_slice()
    }
}

impl<'a, T: Scalar + Copy> From<&'a [T]> for DVectorView<'a, T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        Self::from_slice(slice, slice.len())
    }
}

impl<'a, T: Scalar> From<DVectorView<'a, T>> for &'a [T] {
    fn from(vec: DVectorView<'a, T>) -> &'a [T] {
        vec.data.into_slice()
    }
}

impl<'a, T: Scalar + Copy> From<&'a mut [T]> for DVectorViewMut<'a, T> {
    #[inline]
    fn from(slice: &'a mut [T]) -> Self {
        Self::from_slice(slice, slice.len())
    }
}

impl<'a, T: Scalar> From<DVectorViewMut<'a, T>> for &'a mut [T] {
    fn from(vec: DVectorViewMut<'a, T>) -> &'a mut [T] {
        vec.data.into_slice_mut()
    }
}

impl<T: Scalar + PrimitiveSimdValue, R: Dim, C: Dim> From<[OMatrix<T::Element, R, C>; 2]>
    for OMatrix<T, R, C>
where
    T: From<[<T as SimdValue>::Element; 2]>,
    T::Element: Scalar + SimdValue,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn from(arr: [OMatrix<T::Element, R, C>; 2]) -> Self {
        let (nrows, ncols) = arr[0].shape_generic();

        Self::from_fn_generic(nrows, ncols, |i, j| {
            [arr[0][(i, j)].clone(), arr[1][(i, j)].clone()].into()
        })
    }
}

impl<T: Scalar + PrimitiveSimdValue, R: Dim, C: Dim> From<[OMatrix<T::Element, R, C>; 4]>
    for OMatrix<T, R, C>
where
    T: From<[<T as SimdValue>::Element; 4]>,
    T::Element: Scalar + SimdValue,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn from(arr: [OMatrix<T::Element, R, C>; 4]) -> Self {
        let (nrows, ncols) = arr[0].shape_generic();

        Self::from_fn_generic(nrows, ncols, |i, j| {
            [
                arr[0][(i, j)].clone(),
                arr[1][(i, j)].clone(),
                arr[2][(i, j)].clone(),
                arr[3][(i, j)].clone(),
            ]
            .into()
        })
    }
}

impl<T: Scalar + PrimitiveSimdValue, R: Dim, C: Dim> From<[OMatrix<T::Element, R, C>; 8]>
    for OMatrix<T, R, C>
where
    T: From<[<T as SimdValue>::Element; 8]>,
    T::Element: Scalar + SimdValue,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn from(arr: [OMatrix<T::Element, R, C>; 8]) -> Self {
        let (nrows, ncols) = arr[0].shape_generic();

        Self::from_fn_generic(nrows, ncols, |i, j| {
            [
                arr[0][(i, j)].clone(),
                arr[1][(i, j)].clone(),
                arr[2][(i, j)].clone(),
                arr[3][(i, j)].clone(),
                arr[4][(i, j)].clone(),
                arr[5][(i, j)].clone(),
                arr[6][(i, j)].clone(),
                arr[7][(i, j)].clone(),
            ]
            .into()
        })
    }
}

impl<T: Scalar + PrimitiveSimdValue, R: Dim, C: Dim> From<[OMatrix<T::Element, R, C>; 16]>
    for OMatrix<T, R, C>
where
    T: From<[<T as SimdValue>::Element; 16]>,
    T::Element: Scalar + SimdValue,
    DefaultAllocator: Allocator<R, C>,
{
    fn from(arr: [OMatrix<T::Element, R, C>; 16]) -> Self {
        let (nrows, ncols) = arr[0].shape_generic();

        Self::from_fn_generic(nrows, ncols, |i, j| {
            [
                arr[0][(i, j)].clone(),
                arr[1][(i, j)].clone(),
                arr[2][(i, j)].clone(),
                arr[3][(i, j)].clone(),
                arr[4][(i, j)].clone(),
                arr[5][(i, j)].clone(),
                arr[6][(i, j)].clone(),
                arr[7][(i, j)].clone(),
                arr[8][(i, j)].clone(),
                arr[9][(i, j)].clone(),
                arr[10][(i, j)].clone(),
                arr[11][(i, j)].clone(),
                arr[12][(i, j)].clone(),
                arr[13][(i, j)].clone(),
                arr[14][(i, j)].clone(),
                arr[15][(i, j)].clone(),
            ]
            .into()
        })
    }
}
