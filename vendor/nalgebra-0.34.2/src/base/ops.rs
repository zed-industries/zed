use num::{One, Zero};
use std::iter;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use simba::scalar::{
    ClosedAddAssign, ClosedDivAssign, ClosedMulAssign, ClosedNeg, ClosedSubAssign,
};

use crate::base::allocator::{Allocator, SameShapeAllocator, SameShapeC, SameShapeR};
use crate::base::blas_uninit::gemm_uninit;
use crate::base::constraint::{
    AreMultipliable, DimEq, SameNumberOfColumns, SameNumberOfRows, ShapeConstraint,
};
use crate::base::dimension::{Dim, DimMul, DimName, DimProd, Dyn};
use crate::base::storage::{Storage, StorageMut};
use crate::base::uninit::Uninit;
use crate::base::{DefaultAllocator, Matrix, MatrixSum, OMatrix, Scalar, VectorView};
use crate::storage::IsContiguous;
use crate::uninit::{Init, InitStatus};
use crate::{RawStorage, RawStorageMut, SimdComplexField};
use std::mem::MaybeUninit;

/*
 *
 * Indexing.
 *
 */
impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Index<usize> for Matrix<T, R, C, S> {
    type Output = T;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        let ij = self.vector_to_matrix_index(i);
        &self[ij]
    }
}

impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Index<(usize, usize)> for Matrix<T, R, C, S> {
    type Output = T;

    #[inline]
    fn index(&self, ij: (usize, usize)) -> &Self::Output {
        let shape = self.shape();
        assert!(
            ij.0 < shape.0 && ij.1 < shape.1,
            "Matrix index out of bounds."
        );

        unsafe { self.get_unchecked((ij.0, ij.1)) }
    }
}

// Mutable versions.
impl<T, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> IndexMut<usize> for Matrix<T, R, C, S> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut T {
        let ij = self.vector_to_matrix_index(i);
        &mut self[ij]
    }
}

impl<T, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> IndexMut<(usize, usize)> for Matrix<T, R, C, S> {
    #[inline]
    fn index_mut(&mut self, ij: (usize, usize)) -> &mut T {
        let shape = self.shape();
        assert!(
            ij.0 < shape.0 && ij.1 < shape.1,
            "Matrix index out of bounds."
        );

        unsafe { self.get_unchecked_mut((ij.0, ij.1)) }
    }
}

/*
 *
 * Neg
 *
 */
impl<T, R: Dim, C: Dim, S> Neg for Matrix<T, R, C, S>
where
    T: Scalar + ClosedNeg,
    S: Storage<T, R, C>,
    DefaultAllocator: Allocator<R, C>,
{
    type Output = OMatrix<T, R, C>;

    #[inline]
    fn neg(self) -> Self::Output {
        let mut res = self.into_owned();
        res.neg_mut();
        res
    }
}

impl<T, R: Dim, C: Dim, S> Neg for &Matrix<T, R, C, S>
where
    T: Scalar + ClosedNeg,
    S: Storage<T, R, C>,
    DefaultAllocator: Allocator<R, C>,
{
    type Output = OMatrix<T, R, C>;

    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone_owned()
    }
}

impl<T, R: Dim, C: Dim, S> Matrix<T, R, C, S>
where
    T: Scalar + ClosedNeg,
    S: StorageMut<T, R, C>,
{
    /// Negates `self` in-place.
    #[inline]
    pub fn neg_mut(&mut self) {
        for e in self.iter_mut() {
            *e = -e.clone()
        }
    }
}

/*
 *
 * Addition & Subtraction
 *
 */

macro_rules! componentwise_binop_impl(
    ($Trait: ident, $method: ident, $bound: ident;
     $TraitAssign: ident, $method_assign: ident, $method_assign_statically_unchecked: ident,
     $method_assign_statically_unchecked_rhs: ident;
     $method_to: ident, $method_to_statically_unchecked_uninit: ident) => {

        impl<T, R1: Dim, C1: Dim, SA: Storage<T, R1, C1>> Matrix<T, R1, C1, SA>
            where T: Scalar + $bound {

            /*
             *
             * Methods without dimension checking at compile-time.
             * This is useful for code reuse because the sum representative system does not plays
             * easily with static checks.
             *
             */
            #[inline]
            fn $method_to_statically_unchecked_uninit<Status, R2: Dim, C2: Dim, SB,
                                                       R3: Dim, C3: Dim, SC>(&self,
                                                                     _status: Status,
                                                                     rhs: &Matrix<T, R2, C2, SB>,
                                                                     out: &mut Matrix<Status::Value, R3, C3, SC>)
                where Status: InitStatus<T>,
                      SB: RawStorage<T, R2, C2>,
                      SC: RawStorageMut<Status::Value, R3, C3> {
                assert_eq!(self.shape(), rhs.shape(), "Matrix addition/subtraction dimensions mismatch.");
                assert_eq!(self.shape(), out.shape(), "Matrix addition/subtraction output dimensions mismatch.");

                // This is the most common case and should be deduced at compile-time.
                // TODO: use specialization instead?
                unsafe {
                    if self.data.is_contiguous() && rhs.data.is_contiguous() && out.data.is_contiguous() {
                        let arr1 = self.data.as_slice_unchecked();
                        let arr2 = rhs.data.as_slice_unchecked();
                        let out  = out.data.as_mut_slice_unchecked();
                        for i in 0 .. arr1.len() {
                            Status::init(out.get_unchecked_mut(i), arr1.get_unchecked(i).clone().$method(arr2.get_unchecked(i).clone()));
                        }
                    } else {
                        for j in 0 .. self.ncols() {
                            for i in 0 .. self.nrows() {
                                let val = self.get_unchecked((i, j)).clone().$method(rhs.get_unchecked((i, j)).clone());
                                Status::init(out.get_unchecked_mut((i, j)), val);
                            }
                        }
                    }
                }
            }


            #[inline]
            fn $method_assign_statically_unchecked<R2, C2, SB>(&mut self, rhs: &Matrix<T, R2, C2, SB>)
                where R2: Dim,
                      C2: Dim,
                      SA: StorageMut<T, R1, C1>,
                      SB: Storage<T, R2, C2> {
                assert_eq!(self.shape(), rhs.shape(), "Matrix addition/subtraction dimensions mismatch.");

                // This is the most common case and should be deduced at compile-time.
                // TODO: use specialization instead?
                unsafe {
                    if self.data.is_contiguous() && rhs.data.is_contiguous() {
                        let arr1 = self.data.as_mut_slice_unchecked();
                        let arr2 = rhs.data.as_slice_unchecked();

                        for i in 0 .. arr2.len() {
                            arr1.get_unchecked_mut(i).$method_assign(arr2.get_unchecked(i).clone());
                        }
                    } else {
                        for j in 0 .. rhs.ncols() {
                            for i in 0 .. rhs.nrows() {
                                self.get_unchecked_mut((i, j)).$method_assign(rhs.get_unchecked((i, j)).clone())
                            }
                        }
                    }
                }
            }


            #[inline]
            fn $method_assign_statically_unchecked_rhs<R2, C2, SB>(&self, rhs: &mut Matrix<T, R2, C2, SB>)
                where R2: Dim,
                      C2: Dim,
                      SB: StorageMut<T, R2, C2> {
                assert_eq!(self.shape(), rhs.shape(), "Matrix addition/subtraction dimensions mismatch.");

                // This is the most common case and should be deduced at compile-time.
                // TODO: use specialization instead?
                unsafe {
                    if self.data.is_contiguous() && rhs.data.is_contiguous() {
                        let arr1 = self.data.as_slice_unchecked();
                        let arr2 = rhs.data.as_mut_slice_unchecked();

                        for i in 0 .. arr1.len() {
                            let res = arr1.get_unchecked(i).clone().$method(arr2.get_unchecked(i).clone());
                            *arr2.get_unchecked_mut(i) = res;
                        }
                    } else {
                        for j in 0 .. self.ncols() {
                            for i in 0 .. self.nrows() {
                                let r = rhs.get_unchecked_mut((i, j));
                                *r = self.get_unchecked((i, j)).clone().$method(r.clone())
                            }
                        }
                    }
                }
            }


            /*
             *
             * Methods without dimension checking at compile-time.
             * This is useful for code reuse because the sum representative system does not plays
             * easily with static checks.
             *
             */
            /// Equivalent to `self + rhs` but stores the result into `out` to avoid allocations.
            #[inline]
            pub fn $method_to<R2: Dim, C2: Dim, SB,
                              R3: Dim, C3: Dim, SC>(&self,
                                                    rhs: &Matrix<T, R2, C2, SB>,
                                                    out: &mut Matrix<T, R3, C3, SC>)
                where SB: Storage<T, R2, C2>,
                      SC: StorageMut<T, R3, C3>,
                      ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> +
                                       SameNumberOfRows<R1, R3> + SameNumberOfColumns<C1, C3> {
                self.$method_to_statically_unchecked_uninit(Init, rhs, out)
            }
        }

        impl<'b, T, R1, C1, R2, C2, SA, SB> $Trait<&'b Matrix<T, R2, C2, SB>> for Matrix<T, R1, C1, SA>
            where R1: Dim, C1: Dim, R2: Dim, C2: Dim,
                  T: Scalar + $bound,
                  SA: Storage<T, R1, C1>,
                  SB: Storage<T, R2, C2>,
                  DefaultAllocator: SameShapeAllocator<R1, C1, R2, C2>,
                  ShapeConstraint:  SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {
            type Output = MatrixSum<T, R1, C1, R2, C2>;

            #[inline]
            fn $method(self, rhs: &'b Matrix<T, R2, C2, SB>) -> Self::Output {
                assert_eq!(self.shape(), rhs.shape(), "Matrix addition/subtraction dimensions mismatch.");
                let mut res = self.into_owned_sum::<R2, C2>();
                res.$method_assign_statically_unchecked(rhs);
                res
            }
        }

        impl<'a, T, R1, C1, R2, C2, SA, SB> $Trait<Matrix<T, R2, C2, SB>> for &'a Matrix<T, R1, C1, SA>
            where R1: Dim, C1: Dim, R2: Dim, C2: Dim,
                  T: Scalar + $bound,
                  SA: Storage<T, R1, C1>,
                  SB: Storage<T, R2, C2>,
                  DefaultAllocator: SameShapeAllocator<R2, C2, R1, C1>,
                  ShapeConstraint:  SameNumberOfRows<R2, R1> + SameNumberOfColumns<C2, C1> {
            type Output = MatrixSum<T, R2, C2, R1, C1>;

            #[inline]
            fn $method(self, rhs: Matrix<T, R2, C2, SB>) -> Self::Output {
                let mut rhs = rhs.into_owned_sum::<R1, C1>();
                assert_eq!(self.shape(), rhs.shape(), "Matrix addition/subtraction dimensions mismatch.");
                self.$method_assign_statically_unchecked_rhs(&mut rhs);
                rhs
            }
        }

        impl<T, R1, C1, R2, C2, SA, SB> $Trait<Matrix<T, R2, C2, SB>> for Matrix<T, R1, C1, SA>
            where R1: Dim, C1: Dim, R2: Dim, C2: Dim,
                  T: Scalar + $bound,
                  SA: Storage<T, R1, C1>,
                  SB: Storage<T, R2, C2>,
                  DefaultAllocator: SameShapeAllocator<R1, C1, R2, C2>,
                  ShapeConstraint:  SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {
            type Output = MatrixSum<T, R1, C1, R2, C2>;

            #[inline]
            fn $method(self, rhs: Matrix<T, R2, C2, SB>) -> Self::Output {
                self.$method(&rhs)
            }
        }

        impl<'a, 'b, T, R1, C1, R2, C2, SA, SB> $Trait<&'b Matrix<T, R2, C2, SB>> for &'a Matrix<T, R1, C1, SA>
            where R1: Dim, C1: Dim, R2: Dim, C2: Dim,
                  T: Scalar + $bound,
                  SA: Storage<T, R1, C1>,
                  SB: Storage<T, R2, C2>,
                  DefaultAllocator: SameShapeAllocator<R1, C1, R2, C2>,
                  ShapeConstraint:  SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {
            type Output = MatrixSum<T, R1, C1, R2, C2>;

            #[inline]
            fn $method(self, rhs: &'b Matrix<T, R2, C2, SB>) -> Self::Output {
                let (nrows, ncols) = self.shape();
                let nrows: SameShapeR<R1, R2> = Dim::from_usize(nrows);
                let ncols: SameShapeC<C1, C2> = Dim::from_usize(ncols);
                let mut res = Matrix::uninit(nrows, ncols);
                self.$method_to_statically_unchecked_uninit(Uninit, rhs, &mut res);
                // SAFETY: the output has been initialized above.
                unsafe { res.assume_init() }
            }
        }

        impl<'b, T, R1, C1, R2, C2, SA, SB> $TraitAssign<&'b Matrix<T, R2, C2, SB>> for Matrix<T, R1, C1, SA>
            where R1: Dim, C1: Dim, R2: Dim, C2: Dim,
                  T: Scalar + $bound,
                  SA: StorageMut<T, R1, C1>,
                  SB: Storage<T, R2, C2>,
                  ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {

            #[inline]
            fn $method_assign(&mut self, rhs: &'b Matrix<T, R2, C2, SB>) {
                self.$method_assign_statically_unchecked(rhs)
            }
        }

        impl<T, R1, C1, R2, C2, SA, SB> $TraitAssign<Matrix<T, R2, C2, SB>> for Matrix<T, R1, C1, SA>
            where R1: Dim, C1: Dim, R2: Dim, C2: Dim,
                  T: Scalar + $bound,
                  SA: StorageMut<T, R1, C1>,
                  SB: Storage<T, R2, C2>,
                  ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {

            #[inline]
            fn $method_assign(&mut self, rhs: Matrix<T, R2, C2, SB>) {
                self.$method_assign(&rhs)
            }
        }
    }
);

componentwise_binop_impl!(Add, add, ClosedAddAssign;
                          AddAssign, add_assign, add_assign_statically_unchecked, add_assign_statically_unchecked_mut;
                          add_to, add_to_statically_unchecked_uninit);
componentwise_binop_impl!(Sub, sub, ClosedSubAssign;
                          SubAssign, sub_assign, sub_assign_statically_unchecked, sub_assign_statically_unchecked_mut;
                          sub_to, sub_to_statically_unchecked_uninit);

impl<T, R: DimName, C: DimName> iter::Sum for OMatrix<T, R, C>
where
    T: Scalar + ClosedAddAssign + Zero,
    DefaultAllocator: Allocator<R, C>,
{
    fn sum<I: Iterator<Item = OMatrix<T, R, C>>>(iter: I) -> OMatrix<T, R, C> {
        iter.fold(Matrix::zero(), |acc, x| acc + x)
    }
}

impl<T, C: Dim> iter::Sum for OMatrix<T, Dyn, C>
where
    T: Scalar + ClosedAddAssign + Zero,
    DefaultAllocator: Allocator<Dyn, C>,
{
    /// # Example
    /// ```
    /// # use nalgebra::DVector;
    /// assert_eq!(vec![DVector::repeat(3, 1.0f64),
    ///                 DVector::repeat(3, 1.0f64),
    ///                 DVector::repeat(3, 1.0f64)].into_iter().sum::<DVector<f64>>(),
    ///            DVector::repeat(3, 1.0f64) + DVector::repeat(3, 1.0f64) + DVector::repeat(3, 1.0f64));
    /// ```
    ///
    /// # Panics
    /// Panics if the iterator is empty:
    /// ```should_panic
    /// # use std::iter;
    /// # use nalgebra::DMatrix;
    /// iter::empty::<DMatrix<f64>>().sum::<DMatrix<f64>>(); // panics!
    /// ```
    fn sum<I: Iterator<Item = OMatrix<T, Dyn, C>>>(mut iter: I) -> OMatrix<T, Dyn, C> {
        match iter.next() {
            Some(first) => iter.fold(first, |acc, x| acc + x),
            None => {
                panic!("Cannot compute `sum` of empty iterator.")
            }
        }
    }
}

impl<'a, T, R: DimName, C: DimName> iter::Sum<&'a OMatrix<T, R, C>> for OMatrix<T, R, C>
where
    T: Scalar + ClosedAddAssign + Zero,
    DefaultAllocator: Allocator<R, C>,
{
    fn sum<I: Iterator<Item = &'a OMatrix<T, R, C>>>(iter: I) -> OMatrix<T, R, C> {
        iter.fold(Matrix::zero(), |acc, x| acc + x)
    }
}

impl<'a, T, C: Dim> iter::Sum<&'a OMatrix<T, Dyn, C>> for OMatrix<T, Dyn, C>
where
    T: Scalar + ClosedAddAssign + Zero,
    DefaultAllocator: Allocator<Dyn, C>,
{
    /// # Example
    /// ```
    /// # use nalgebra::DVector;
    /// let v = &DVector::repeat(3, 1.0f64);
    ///
    /// assert_eq!(vec![v, v, v].into_iter().sum::<DVector<f64>>(),
    ///            v + v + v);
    /// ```
    ///
    /// # Panics
    /// Panics if the iterator is empty:
    /// ```should_panic
    /// # use std::iter;
    /// # use nalgebra::DMatrix;
    /// iter::empty::<&DMatrix<f64>>().sum::<DMatrix<f64>>(); // panics!
    /// ```
    fn sum<I: Iterator<Item = &'a OMatrix<T, Dyn, C>>>(mut iter: I) -> OMatrix<T, Dyn, C> {
        if let Some(first) = iter.next() {
            iter.fold(first.clone(), |acc, x| acc + x)
        } else {
            panic!("Cannot compute `sum` of empty iterator.")
        }
    }
}

/*
 *
 * Multiplication
 *
 */

// Matrix × Scalar
// Matrix / Scalar
macro_rules! componentwise_scalarop_impl(
    ($Trait: ident, $method: ident, $bound: ident;
     $TraitAssign: ident, $method_assign: ident) => {
        impl<T, R: Dim, C: Dim, S> $Trait<T> for Matrix<T, R, C, S>
            where T: Scalar + $bound,
                  S: Storage<T, R, C>,
                  DefaultAllocator: Allocator<R, C> {
            type Output = OMatrix<T, R, C>;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                let mut res = self.into_owned();

                // XXX: optimize our iterator!
                //
                // Using our own iterator prevents loop unrolling, which breaks some optimization
                // (like SIMD). On the other hand, using the slice iterator is 4x faster.

                // for left in res.iter_mut() {
                for left in res.as_mut_slice().iter_mut() {
                    *left = left.clone().$method(rhs.clone())
                }

                res
            }
        }

        impl<'a, T, R: Dim, C: Dim, S> $Trait<T> for &'a Matrix<T, R, C, S>
            where T: Scalar + $bound,
                  S: Storage<T, R, C>,
                  DefaultAllocator: Allocator<R, C> {
            type Output = OMatrix<T, R, C>;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                self.clone_owned().$method(rhs)
            }
        }

        impl<T, R: Dim, C: Dim, S> $TraitAssign<T> for Matrix<T, R, C, S>
            where T: Scalar + $bound,
                  S: StorageMut<T, R, C> {
            #[inline]
            fn $method_assign(&mut self, rhs: T) {
                for j in 0 .. self.ncols() {
                    for i in 0 .. self.nrows() {
                        unsafe { self.get_unchecked_mut((i, j)).$method_assign(rhs.clone()) };
                    }
                }
            }
        }
    }
);

componentwise_scalarop_impl!(Mul, mul, ClosedMulAssign; MulAssign, mul_assign);
componentwise_scalarop_impl!(Div, div, ClosedDivAssign; DivAssign, div_assign);

macro_rules! left_scalar_mul_impl(
    ($($T: ty),* $(,)*) => {$(
        impl<R: Dim, C: Dim, S: Storage<$T, R, C>> Mul<Matrix<$T, R, C, S>> for $T
            where DefaultAllocator: Allocator<R, C> {
            type Output = OMatrix<$T, R, C>;

            #[inline]
            fn mul(self, rhs: Matrix<$T, R, C, S>) -> Self::Output {
                let mut res = rhs.into_owned();

                // XXX: optimize our iterator!
                //
                // Using our own iterator prevents loop unrolling, which breaks some optimization
                // (like SIMD). On the other hand, using the slice iterator is 4x faster.

                // for rhs in res.iter_mut() {
                for rhs in res.as_mut_slice().iter_mut() {
                    *rhs *= self
                }

                res
            }
        }

        impl<'b, R: Dim, C: Dim, S: Storage<$T, R, C>> Mul<&'b Matrix<$T, R, C, S>> for $T
            where DefaultAllocator: Allocator<R, C> {
            type Output = OMatrix<$T, R, C>;

            #[inline]
            fn mul(self, rhs: &'b Matrix<$T, R, C, S>) -> Self::Output {
                self * rhs.clone_owned()
            }
        }
    )*}
);

left_scalar_mul_impl!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

// Matrix × Matrix
impl<'b, T, R1: Dim, C1: Dim, R2: Dim, C2: Dim, SA, SB> Mul<&'b Matrix<T, R2, C2, SB>>
    for &Matrix<T, R1, C1, SA>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    SA: Storage<T, R1, C1>,
    SB: Storage<T, R2, C2>,
    DefaultAllocator: Allocator<R1, C2>,
    ShapeConstraint: AreMultipliable<R1, C1, R2, C2>,
{
    type Output = OMatrix<T, R1, C2>;

    #[inline]
    fn mul(self, rhs: &'b Matrix<T, R2, C2, SB>) -> Self::Output {
        let mut res = Matrix::uninit(self.shape_generic().0, rhs.shape_generic().1);
        unsafe {
            // SAFETY: this is OK because status = Uninit && bevy == 0
            gemm_uninit(Uninit, &mut res, T::one(), self, rhs, T::zero());
            res.assume_init()
        }
    }
}

impl<T, R1: Dim, C1: Dim, R2: Dim, C2: Dim, SA, SB> Mul<Matrix<T, R2, C2, SB>>
    for &Matrix<T, R1, C1, SA>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    SB: Storage<T, R2, C2>,
    SA: Storage<T, R1, C1>,
    DefaultAllocator: Allocator<R1, C2>,
    ShapeConstraint: AreMultipliable<R1, C1, R2, C2>,
{
    type Output = OMatrix<T, R1, C2>;

    #[inline]
    fn mul(self, rhs: Matrix<T, R2, C2, SB>) -> Self::Output {
        self * &rhs
    }
}

impl<'b, T, R1: Dim, C1: Dim, R2: Dim, C2: Dim, SA, SB> Mul<&'b Matrix<T, R2, C2, SB>>
    for Matrix<T, R1, C1, SA>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    SB: Storage<T, R2, C2>,
    SA: Storage<T, R1, C1>,
    DefaultAllocator: Allocator<R1, C2>,
    ShapeConstraint: AreMultipliable<R1, C1, R2, C2>,
{
    type Output = OMatrix<T, R1, C2>;

    #[inline]
    fn mul(self, rhs: &'b Matrix<T, R2, C2, SB>) -> Self::Output {
        &self * rhs
    }
}

impl<T, R1: Dim, C1: Dim, R2: Dim, C2: Dim, SA, SB> Mul<Matrix<T, R2, C2, SB>>
    for Matrix<T, R1, C1, SA>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    SB: Storage<T, R2, C2>,
    SA: Storage<T, R1, C1>,
    DefaultAllocator: Allocator<R1, C2>,
    ShapeConstraint: AreMultipliable<R1, C1, R2, C2>,
{
    type Output = OMatrix<T, R1, C2>;

    #[inline]
    fn mul(self, rhs: Matrix<T, R2, C2, SB>) -> Self::Output {
        &self * &rhs
    }
}

// TODO: this is too restrictive:
//    − we can't use `a *= b` when `a` is a mutable slice.
//    − we can't use `a *= b` when C2 is not equal to C1.
impl<T, R1, C1, R2, SA, SB> MulAssign<Matrix<T, R2, C1, SB>> for Matrix<T, R1, C1, SA>
where
    R1: Dim,
    C1: Dim,
    R2: Dim,
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    SB: Storage<T, R2, C1>,
    SA: StorageMut<T, R1, C1> + IsContiguous + Clone, // TODO: get rid of the IsContiguous
    ShapeConstraint: AreMultipliable<R1, C1, R2, C1>,
    DefaultAllocator: Allocator<R1, C1, Buffer<T> = SA>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Matrix<T, R2, C1, SB>) {
        *self = &*self * rhs
    }
}

impl<'b, T, R1, C1, R2, SA, SB> MulAssign<&'b Matrix<T, R2, C1, SB>> for Matrix<T, R1, C1, SA>
where
    R1: Dim,
    C1: Dim,
    R2: Dim,
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    SB: Storage<T, R2, C1>,
    SA: StorageMut<T, R1, C1> + IsContiguous + Clone, // TODO: get rid of the IsContiguous
    ShapeConstraint: AreMultipliable<R1, C1, R2, C1>,
    // TODO: this is too restrictive. See comments for the non-ref version.
    DefaultAllocator: Allocator<R1, C1, Buffer<T> = SA>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: &'b Matrix<T, R2, C1, SB>) {
        *self = &*self * rhs
    }
}

/// # Special multiplications.
impl<T, R1: Dim, C1: Dim, SA> Matrix<T, R1, C1, SA>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    SA: Storage<T, R1, C1>,
{
    /// Equivalent to `self.transpose() * rhs`.
    #[inline]
    #[must_use]
    pub fn tr_mul<R2: Dim, C2: Dim, SB>(&self, rhs: &Matrix<T, R2, C2, SB>) -> OMatrix<T, C1, C2>
    where
        SB: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<C1, C2>,
        ShapeConstraint: SameNumberOfRows<R1, R2>,
    {
        let mut res = Matrix::uninit(self.shape_generic().1, rhs.shape_generic().1);
        self.xx_mul_to_uninit(Uninit, rhs, &mut res, |a, b| a.dot(b));
        // SAFETY: this is OK because the result is now initialized.
        unsafe { res.assume_init() }
    }

    /// Equivalent to `self.adjoint() * rhs`.
    #[inline]
    #[must_use]
    pub fn ad_mul<R2: Dim, C2: Dim, SB>(&self, rhs: &Matrix<T, R2, C2, SB>) -> OMatrix<T, C1, C2>
    where
        T: SimdComplexField,
        SB: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<C1, C2>,
        ShapeConstraint: SameNumberOfRows<R1, R2>,
    {
        let mut res = Matrix::uninit(self.shape_generic().1, rhs.shape_generic().1);
        self.xx_mul_to_uninit(Uninit, rhs, &mut res, |a, b| a.dotc(b));
        // SAFETY: this is OK because the result is now initialized.
        unsafe { res.assume_init() }
    }

    #[inline(always)]
    fn xx_mul_to_uninit<Status, R2: Dim, C2: Dim, SB, R3: Dim, C3: Dim, SC>(
        &self,
        _status: Status,
        rhs: &Matrix<T, R2, C2, SB>,
        out: &mut Matrix<Status::Value, R3, C3, SC>,
        dot: impl Fn(
            &VectorView<'_, T, R1, SA::RStride, SA::CStride>,
            &VectorView<'_, T, R2, SB::RStride, SB::CStride>,
        ) -> T,
    ) where
        Status: InitStatus<T>,
        SB: RawStorage<T, R2, C2>,
        SC: RawStorageMut<Status::Value, R3, C3>,
        ShapeConstraint: SameNumberOfRows<R1, R2> + DimEq<C1, R3> + DimEq<C2, C3>,
    {
        let (nrows1, ncols1) = self.shape();
        let (nrows2, ncols2) = rhs.shape();
        let (nrows3, ncols3) = out.shape();

        assert!(
            nrows1 == nrows2,
            "Matrix multiplication dimensions mismatch {:?} and {:?}: left rows != right rows.",
            self.shape(),
            rhs.shape()
        );
        assert!(
            ncols1 == nrows3,
            "Matrix multiplication output dimensions mismatch {:?} and {:?}: left cols != right rows.",
            self.shape(),
            out.shape()
        );
        assert!(
            ncols2 == ncols3,
            "Matrix multiplication output dimensions mismatch {:?} and {:?}: left cols != right cols",
            rhs.shape(),
            out.shape()
        );

        for i in 0..ncols1 {
            for j in 0..ncols2 {
                let dot = dot(&self.column(i), &rhs.column(j));
                let elt = unsafe { out.get_unchecked_mut((i, j)) };
                Status::init(elt, dot);
            }
        }
    }

    /// Equivalent to `self.transpose() * rhs` but stores the result into `out` to avoid
    /// allocations.
    #[inline]
    pub fn tr_mul_to<R2: Dim, C2: Dim, SB, R3: Dim, C3: Dim, SC>(
        &self,
        rhs: &Matrix<T, R2, C2, SB>,
        out: &mut Matrix<T, R3, C3, SC>,
    ) where
        SB: Storage<T, R2, C2>,
        SC: StorageMut<T, R3, C3>,
        ShapeConstraint: SameNumberOfRows<R1, R2> + DimEq<C1, R3> + DimEq<C2, C3>,
    {
        self.xx_mul_to_uninit(Init, rhs, out, |a, b| a.dot(b))
    }

    /// Equivalent to `self.adjoint() * rhs` but stores the result into `out` to avoid
    /// allocations.
    #[inline]
    pub fn ad_mul_to<R2: Dim, C2: Dim, SB, R3: Dim, C3: Dim, SC>(
        &self,
        rhs: &Matrix<T, R2, C2, SB>,
        out: &mut Matrix<T, R3, C3, SC>,
    ) where
        T: SimdComplexField,
        SB: Storage<T, R2, C2>,
        SC: StorageMut<T, R3, C3>,
        ShapeConstraint: SameNumberOfRows<R1, R2> + DimEq<C1, R3> + DimEq<C2, C3>,
    {
        self.xx_mul_to_uninit(Init, rhs, out, |a, b| a.dotc(b))
    }

    /// Equivalent to `self * rhs` but stores the result into `out` to avoid allocations.
    #[inline]
    pub fn mul_to<R2: Dim, C2: Dim, SB, R3: Dim, C3: Dim, SC>(
        &self,
        rhs: &Matrix<T, R2, C2, SB>,
        out: &mut Matrix<T, R3, C3, SC>,
    ) where
        SB: Storage<T, R2, C2>,
        SC: StorageMut<T, R3, C3>,
        ShapeConstraint: SameNumberOfRows<R3, R1>
            + SameNumberOfColumns<C3, C2>
            + AreMultipliable<R1, C1, R2, C2>,
    {
        out.gemm(T::one(), self, rhs, T::zero());
    }

    /// The kronecker product of two matrices (aka. tensor product of the corresponding linear
    /// maps).
    #[must_use]
    pub fn kronecker<R2: Dim, C2: Dim, SB>(
        &self,
        rhs: &Matrix<T, R2, C2, SB>,
    ) -> OMatrix<T, DimProd<R1, R2>, DimProd<C1, C2>>
    where
        T: ClosedMulAssign,
        R1: DimMul<R2>,
        C1: DimMul<C2>,
        SB: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<DimProd<R1, R2>, DimProd<C1, C2>>,
    {
        let (nrows1, ncols1) = self.shape_generic();
        let (nrows2, ncols2) = rhs.shape_generic();

        let mut res = Matrix::uninit(nrows1.mul(nrows2), ncols1.mul(ncols2));
        let mut data_res = res.data.ptr_mut();

        unsafe {
            for j1 in 0..ncols1.value() {
                for j2 in 0..ncols2.value() {
                    for i1 in 0..nrows1.value() {
                        let coeff = self.get_unchecked((i1, j1)).clone();

                        for i2 in 0..nrows2.value() {
                            *data_res = MaybeUninit::new(
                                coeff.clone() * rhs.get_unchecked((i2, j2)).clone(),
                            );
                            data_res = data_res.offset(1);
                        }
                    }
                }
            }

            // SAFETY: the result matrix has been initialized by the loop above.
            res.assume_init()
        }
    }
}

impl<T, D: DimName> iter::Product for OMatrix<T, D, D>
where
    T: Scalar + Zero + One + ClosedMulAssign + ClosedAddAssign,
    DefaultAllocator: Allocator<D, D>,
{
    fn product<I: Iterator<Item = OMatrix<T, D, D>>>(iter: I) -> OMatrix<T, D, D> {
        iter.fold(Matrix::one(), |acc, x| acc * x)
    }
}

impl<'a, T, D: DimName> iter::Product<&'a OMatrix<T, D, D>> for OMatrix<T, D, D>
where
    T: Scalar + Zero + One + ClosedMulAssign + ClosedAddAssign,
    DefaultAllocator: Allocator<D, D>,
{
    fn product<I: Iterator<Item = &'a OMatrix<T, D, D>>>(iter: I) -> OMatrix<T, D, D> {
        iter.fold(Matrix::one(), |acc, x| acc * x)
    }
}
