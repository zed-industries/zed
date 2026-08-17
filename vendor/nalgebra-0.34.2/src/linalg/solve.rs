use simba::scalar::ComplexField;
use simba::simd::SimdComplexField;

use crate::base::allocator::Allocator;
use crate::base::constraint::{SameNumberOfRows, ShapeConstraint};
use crate::base::dimension::{Dim, U1};
use crate::base::storage::{Storage, StorageMut};
use crate::base::{DVectorView, DefaultAllocator, Matrix, OMatrix, SquareMatrix, Vector};

impl<T: ComplexField, D: Dim, S: Storage<T, D, D>> SquareMatrix<T, D, S> {
    /// Computes the solution of the linear system `self . x = b` where `x` is the unknown and only
    /// the lower-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use solve_lower_triangular_mut()?"]
    #[inline]
    pub fn solve_lower_triangular<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> Option<OMatrix<T, R2, C2>>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        if self.solve_lower_triangular_mut(&mut res) {
            Some(res)
        } else {
            None
        }
    }

    /// Computes the solution of the linear system `self . x = b` where `x` is the unknown and only
    /// the upper-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use solve_upper_triangular_mut()?"]
    #[inline]
    pub fn solve_upper_triangular<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> Option<OMatrix<T, R2, C2>>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        if self.solve_upper_triangular_mut(&mut res) {
            Some(res)
        } else {
            None
        }
    }

    /// Solves the linear system `self . x = b` where `x` is the unknown and only the
    /// lower-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn solve_lower_triangular_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) -> bool
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let cols = b.ncols();

        for i in 0..cols {
            if !self.solve_lower_triangular_vector_mut(&mut b.column_mut(i)) {
                return false;
            }
        }

        true
    }

    fn solve_lower_triangular_vector_mut<R2: Dim, S2>(&self, b: &mut Vector<T, R2, S2>) -> bool
    where
        S2: StorageMut<T, R2, U1>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let dim = self.nrows();

        for i in 0..dim {
            let coeff;

            unsafe {
                let diag = self.get_unchecked((i, i)).clone();

                if diag.is_zero() {
                    return false;
                }

                coeff = b.vget_unchecked(i).clone() / diag;
                *b.vget_unchecked_mut(i) = coeff.clone();
            }

            b.rows_range_mut(i + 1..)
                .axpy(-coeff, &self.view_range(i + 1.., i), T::one());
        }

        true
    }

    // TODO: add the same but for solving upper-triangular.
    /// Solves the linear system `self . x = b` where `x` is the unknown and only the
    /// lower-triangular part of `self` is considered not-zero. The diagonal is never read as it is
    /// assumed to be equal to `diag`. Returns `false` and does not modify its inputs if `diag` is zero.
    pub fn solve_lower_triangular_with_diag_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
        diag: T,
    ) -> bool
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        if diag.is_zero() {
            return false;
        }

        let dim = self.nrows();
        let cols = b.ncols();

        for k in 0..cols {
            let mut bcol = b.column_mut(k);

            for i in 0..dim - 1 {
                let coeff = unsafe { bcol.vget_unchecked(i).clone() } / diag.clone();
                bcol.rows_range_mut(i + 1..)
                    .axpy(-coeff, &self.view_range(i + 1.., i), T::one());
            }
        }

        true
    }

    /// Solves the linear system `self . x = b` where `x` is the unknown and only the
    /// upper-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn solve_upper_triangular_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) -> bool
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let cols = b.ncols();

        for i in 0..cols {
            if !self.solve_upper_triangular_vector_mut(&mut b.column_mut(i)) {
                return false;
            }
        }

        true
    }

    fn solve_upper_triangular_vector_mut<R2: Dim, S2>(&self, b: &mut Vector<T, R2, S2>) -> bool
    where
        S2: StorageMut<T, R2, U1>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let dim = self.nrows();

        for i in (0..dim).rev() {
            let coeff;

            unsafe {
                let diag = self.get_unchecked((i, i)).clone();

                if diag.is_zero() {
                    return false;
                }

                coeff = b.vget_unchecked(i).clone() / diag;
                *b.vget_unchecked_mut(i) = coeff.clone();
            }

            b.rows_range_mut(..i)
                .axpy(-coeff, &self.view_range(..i, i), T::one());
        }

        true
    }

    /*
     *
     * Transpose and adjoint versions
     *
     */
    /// Computes the solution of the linear system `self.transpose() . x = b` where `x` is the unknown and only
    /// the lower-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use tr_solve_lower_triangular_mut()?"]
    #[inline]
    pub fn tr_solve_lower_triangular<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> Option<OMatrix<T, R2, C2>>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        if self.tr_solve_lower_triangular_mut(&mut res) {
            Some(res)
        } else {
            None
        }
    }

    /// Computes the solution of the linear system `self.transpose() . x = b` where `x` is the unknown and only
    /// the upper-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use tr_solve_upper_triangular_mut()?"]
    #[inline]
    pub fn tr_solve_upper_triangular<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> Option<OMatrix<T, R2, C2>>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        if self.tr_solve_upper_triangular_mut(&mut res) {
            Some(res)
        } else {
            None
        }
    }

    /// Solves the linear system `self.transpose() . x = b` where `x` is the unknown and only the
    /// lower-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn tr_solve_lower_triangular_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) -> bool
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let cols = b.ncols();

        for i in 0..cols {
            if !self.xx_solve_lower_triangular_vector_mut(
                &mut b.column_mut(i),
                |e| e,
                |a, b| a.dot(b),
            ) {
                return false;
            }
        }

        true
    }

    /// Solves the linear system `self.transpose() . x = b` where `x` is the unknown and only the
    /// upper-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn tr_solve_upper_triangular_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) -> bool
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let cols = b.ncols();

        for i in 0..cols {
            if !self.xx_solve_upper_triangular_vector_mut(
                &mut b.column_mut(i),
                |e| e,
                |a, b| a.dot(b),
            ) {
                return false;
            }
        }

        true
    }

    /// Computes the solution of the linear system `self.adjoint() . x = b` where `x` is the unknown and only
    /// the lower-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use ad_solve_lower_triangular_mut()?"]
    #[inline]
    pub fn ad_solve_lower_triangular<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> Option<OMatrix<T, R2, C2>>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        if self.ad_solve_lower_triangular_mut(&mut res) {
            Some(res)
        } else {
            None
        }
    }

    /// Computes the solution of the linear system `self.adjoint() . x = b` where `x` is the unknown and only
    /// the upper-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use ad_solve_upper_triangular_mut()?"]
    #[inline]
    pub fn ad_solve_upper_triangular<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> Option<OMatrix<T, R2, C2>>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        if self.ad_solve_upper_triangular_mut(&mut res) {
            Some(res)
        } else {
            None
        }
    }

    /// Solves the linear system `self.adjoint() . x = b` where `x` is the unknown and only the
    /// lower-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn ad_solve_lower_triangular_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) -> bool
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let cols = b.ncols();

        for i in 0..cols {
            if !self.xx_solve_lower_triangular_vector_mut(
                &mut b.column_mut(i),
                |e| e.conjugate(),
                |a, b| a.dotc(b),
            ) {
                return false;
            }
        }

        true
    }

    /// Solves the linear system `self.adjoint() . x = b` where `x` is the unknown and only the
    /// upper-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn ad_solve_upper_triangular_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) -> bool
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let cols = b.ncols();

        for i in 0..cols {
            if !self.xx_solve_upper_triangular_vector_mut(
                &mut b.column_mut(i),
                |e| e.conjugate(),
                |a, b| a.dotc(b),
            ) {
                return false;
            }
        }

        true
    }

    #[inline(always)]
    fn xx_solve_lower_triangular_vector_mut<R2: Dim, S2>(
        &self,
        b: &mut Vector<T, R2, S2>,
        conjugate: impl Fn(T) -> T,
        dot: impl Fn(
            &DVectorView<'_, T, S::RStride, S::CStride>,
            &DVectorView<'_, T, S2::RStride, S2::CStride>,
        ) -> T,
    ) -> bool
    where
        S2: StorageMut<T, R2, U1>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let dim = self.nrows();

        for i in (0..dim).rev() {
            let dot = dot(&self.view_range(i + 1.., i), &b.view_range(i + 1.., 0));

            unsafe {
                let b_i = b.vget_unchecked_mut(i);

                let diag = conjugate(self.get_unchecked((i, i)).clone());

                if diag.is_zero() {
                    return false;
                }

                *b_i = (b_i.clone() - dot) / diag;
            }
        }

        true
    }

    #[inline(always)]
    fn xx_solve_upper_triangular_vector_mut<R2: Dim, S2>(
        &self,
        b: &mut Vector<T, R2, S2>,
        conjugate: impl Fn(T) -> T,
        dot: impl Fn(
            &DVectorView<'_, T, S::RStride, S::CStride>,
            &DVectorView<'_, T, S2::RStride, S2::CStride>,
        ) -> T,
    ) -> bool
    where
        S2: StorageMut<T, R2, U1>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let dim = self.nrows();

        for i in 0..dim {
            let dot = dot(&self.view_range(..i, i), &b.view_range(..i, 0));

            unsafe {
                let b_i = b.vget_unchecked_mut(i);
                let diag = conjugate(self.get_unchecked((i, i)).clone());

                if diag.is_zero() {
                    return false;
                }

                *b_i = (b_i.clone() - dot) / diag;
            }
        }

        true
    }
}

/*
 *
 * SIMD-compatible unchecked versions.
 *
 */

impl<T: SimdComplexField, D: Dim, S: Storage<T, D, D>> SquareMatrix<T, D, S> {
    /// Computes the solution of the linear system `self . x = b` where `x` is the unknown and only
    /// the lower-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use solve_lower_triangular_unchecked_mut()?"]
    #[inline]
    pub fn solve_lower_triangular_unchecked<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> OMatrix<T, R2, C2>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        self.solve_lower_triangular_unchecked_mut(&mut res);
        res
    }

    /// Computes the solution of the linear system `self . x = b` where `x` is the unknown and only
    /// the upper-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use solve_upper_triangular_unchecked_mut()?"]
    #[inline]
    pub fn solve_upper_triangular_unchecked<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> OMatrix<T, R2, C2>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        self.solve_upper_triangular_unchecked_mut(&mut res);
        res
    }

    /// Solves the linear system `self . x = b` where `x` is the unknown and only the
    /// lower-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn solve_lower_triangular_unchecked_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        for i in 0..b.ncols() {
            self.solve_lower_triangular_vector_unchecked_mut(&mut b.column_mut(i));
        }
    }

    fn solve_lower_triangular_vector_unchecked_mut<R2: Dim, S2>(&self, b: &mut Vector<T, R2, S2>)
    where
        S2: StorageMut<T, R2, U1>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let dim = self.nrows();

        for i in 0..dim {
            let coeff;

            unsafe {
                let diag = self.get_unchecked((i, i)).clone();
                coeff = b.vget_unchecked(i).clone() / diag;
                *b.vget_unchecked_mut(i) = coeff.clone();
            }

            b.rows_range_mut(i + 1..)
                .axpy(-coeff.clone(), &self.view_range(i + 1.., i), T::one());
        }
    }

    // TODO: add the same but for solving upper-triangular.
    /// Solves the linear system `self . x = b` where `x` is the unknown and only the
    /// lower-triangular part of `self` is considered not-zero. The diagonal is never read as it is
    /// assumed to be equal to `diag`. Returns `false` and does not modify its inputs if `diag` is zero.
    pub fn solve_lower_triangular_with_diag_unchecked_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
        diag: T,
    ) where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let dim = self.nrows();
        let cols = b.ncols();

        for k in 0..cols {
            let mut bcol = b.column_mut(k);

            for i in 0..dim - 1 {
                let coeff = unsafe { bcol.vget_unchecked(i).clone() } / diag.clone();
                bcol.rows_range_mut(i + 1..)
                    .axpy(-coeff, &self.view_range(i + 1.., i), T::one());
            }
        }
    }

    /// Solves the linear system `self . x = b` where `x` is the unknown and only the
    /// upper-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn solve_upper_triangular_unchecked_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        for i in 0..b.ncols() {
            self.solve_upper_triangular_vector_unchecked_mut(&mut b.column_mut(i))
        }
    }

    fn solve_upper_triangular_vector_unchecked_mut<R2: Dim, S2>(&self, b: &mut Vector<T, R2, S2>)
    where
        S2: StorageMut<T, R2, U1>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let dim = self.nrows();

        for i in (0..dim).rev() {
            let coeff;

            unsafe {
                let diag = self.get_unchecked((i, i)).clone();
                coeff = b.vget_unchecked(i).clone() / diag;
                *b.vget_unchecked_mut(i) = coeff.clone();
            }

            b.rows_range_mut(..i)
                .axpy(-coeff, &self.view_range(..i, i), T::one());
        }
    }

    /*
     *
     * Transpose and adjoint versions
     *
     */
    /// Computes the solution of the linear system `self.transpose() . x = b` where `x` is the unknown and only
    /// the lower-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use tr_solve_lower_triangular_unchecked_mut()?"]
    #[inline]
    pub fn tr_solve_lower_triangular_unchecked<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> OMatrix<T, R2, C2>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        self.tr_solve_lower_triangular_unchecked_mut(&mut res);
        res
    }

    /// Computes the solution of the linear system `self.transpose() . x = b` where `x` is the unknown and only
    /// the upper-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use tr_solve_upper_triangular_unchecked_mut()?"]
    #[inline]
    pub fn tr_solve_upper_triangular_unchecked<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> OMatrix<T, R2, C2>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        self.tr_solve_upper_triangular_unchecked_mut(&mut res);
        res
    }

    /// Solves the linear system `self.transpose() . x = b` where `x` is the unknown and only the
    /// lower-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn tr_solve_lower_triangular_unchecked_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        for i in 0..b.ncols() {
            self.xx_solve_lower_triangular_vector_unchecked_mut(
                &mut b.column_mut(i),
                |e| e,
                |a, b| a.dot(b),
            )
        }
    }

    /// Solves the linear system `self.transpose() . x = b` where `x` is the unknown and only the
    /// upper-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn tr_solve_upper_triangular_unchecked_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        for i in 0..b.ncols() {
            self.xx_solve_upper_triangular_vector_unchecked_mut(
                &mut b.column_mut(i),
                |e| e,
                |a, b| a.dot(b),
            )
        }
    }

    /// Computes the solution of the linear system `self.adjoint() . x = b` where `x` is the unknown and only
    /// the lower-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use ad_solve_lower_triangular_unchecked_mut()?"]
    #[inline]
    pub fn ad_solve_lower_triangular_unchecked<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> OMatrix<T, R2, C2>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        self.ad_solve_lower_triangular_unchecked_mut(&mut res);
        res
    }

    /// Computes the solution of the linear system `self.adjoint() . x = b` where `x` is the unknown and only
    /// the upper-triangular part of `self` (including the diagonal) is considered not-zero.
    #[must_use = "Did you mean to use ad_solve_upper_triangular_unchecked_mut()?"]
    #[inline]
    pub fn ad_solve_upper_triangular_unchecked<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> OMatrix<T, R2, C2>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        self.ad_solve_upper_triangular_unchecked_mut(&mut res);
        res
    }

    /// Solves the linear system `self.adjoint() . x = b` where `x` is the unknown and only the
    /// lower-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn ad_solve_lower_triangular_unchecked_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        for i in 0..b.ncols() {
            self.xx_solve_lower_triangular_vector_unchecked_mut(
                &mut b.column_mut(i),
                |e| e.simd_conjugate(),
                |a, b| a.dotc(b),
            )
        }
    }

    /// Solves the linear system `self.adjoint() . x = b` where `x` is the unknown and only the
    /// upper-triangular part of `self` (including the diagonal) is considered not-zero.
    pub fn ad_solve_upper_triangular_unchecked_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        for i in 0..b.ncols() {
            self.xx_solve_upper_triangular_vector_unchecked_mut(
                &mut b.column_mut(i),
                |e| e.simd_conjugate(),
                |a, b| a.dotc(b),
            )
        }
    }

    #[inline(always)]
    fn xx_solve_lower_triangular_vector_unchecked_mut<R2: Dim, S2>(
        &self,
        b: &mut Vector<T, R2, S2>,
        conjugate: impl Fn(T) -> T,
        dot: impl Fn(
            &DVectorView<'_, T, S::RStride, S::CStride>,
            &DVectorView<'_, T, S2::RStride, S2::CStride>,
        ) -> T,
    ) where
        S2: StorageMut<T, R2, U1>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let dim = self.nrows();

        for i in (0..dim).rev() {
            let dot = dot(&self.view_range(i + 1.., i), &b.view_range(i + 1.., 0));

            unsafe {
                let b_i = b.vget_unchecked_mut(i);
                let diag = conjugate(self.get_unchecked((i, i)).clone());
                *b_i = (b_i.clone() - dot) / diag;
            }
        }
    }

    #[inline(always)]
    fn xx_solve_upper_triangular_vector_unchecked_mut<R2: Dim, S2>(
        &self,
        b: &mut Vector<T, R2, S2>,
        conjugate: impl Fn(T) -> T,
        dot: impl Fn(
            &DVectorView<'_, T, S::RStride, S::CStride>,
            &DVectorView<'_, T, S2::RStride, S2::CStride>,
        ) -> T,
    ) where
        S2: StorageMut<T, R2, U1>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        for i in 0..self.nrows() {
            let dot = dot(&self.view_range(..i, i), &b.view_range(..i, 0));

            unsafe {
                let b_i = b.vget_unchecked_mut(i);
                let diag = conjugate(self.get_unchecked((i, i)).clone());
                *b_i = (b_i.clone() - dot) / diag;
            }
        }
    }
}
