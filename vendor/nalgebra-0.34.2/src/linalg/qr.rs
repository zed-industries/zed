use num::Zero;
#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use crate::allocator::{Allocator, Reallocator};
use crate::base::{DefaultAllocator, Matrix, OMatrix, OVector, Unit};
use crate::constraint::{SameNumberOfRows, ShapeConstraint};
use crate::dimension::{Const, Dim, DimMin, DimMinimum};
use crate::storage::{Storage, StorageMut};
use simba::scalar::ComplexField;

use crate::geometry::Reflection;
use crate::linalg::householder;
use std::mem::MaybeUninit;

/// The QR decomposition of a general matrix.
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Serialize,
         OVector<T, DimMinimum<R, C>>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(deserialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Deserialize<'de>,
         OVector<T, DimMinimum<R, C>>: Deserialize<'de>"))
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug)]
pub struct QR<T: ComplexField, R: DimMin<C>, C: Dim>
where
    DefaultAllocator: Allocator<R, C> + Allocator<DimMinimum<R, C>>,
{
    qr: OMatrix<T, R, C>,
    diag: OVector<T, DimMinimum<R, C>>,
}

impl<T: ComplexField, R: DimMin<C>, C: Dim> Copy for QR<T, R, C>
where
    DefaultAllocator: Allocator<R, C> + Allocator<DimMinimum<R, C>>,
    OMatrix<T, R, C>: Copy,
    OVector<T, DimMinimum<R, C>>: Copy,
{
}

impl<T: ComplexField, R: DimMin<C>, C: Dim> QR<T, R, C>
where
    DefaultAllocator: Allocator<R, C> + Allocator<R> + Allocator<DimMinimum<R, C>>,
{
    /// Computes the QR decomposition using householder reflections.
    pub fn new(mut matrix: OMatrix<T, R, C>) -> Self {
        let (nrows, ncols) = matrix.shape_generic();
        let min_nrows_ncols = nrows.min(ncols);

        if min_nrows_ncols.value() == 0 {
            return QR {
                qr: matrix,
                diag: Matrix::zeros_generic(min_nrows_ncols, Const::<1>),
            };
        }

        let mut diag = Matrix::uninit(min_nrows_ncols, Const::<1>);

        for i in 0..min_nrows_ncols.value() {
            diag[i] =
                MaybeUninit::new(householder::clear_column_unchecked(&mut matrix, i, 0, None));
        }

        // Safety: diag is now fully initialized.
        let diag = unsafe { diag.assume_init() };
        QR { qr: matrix, diag }
    }

    /// Retrieves the upper trapezoidal submatrix `R` of this decomposition.
    #[inline]
    #[must_use]
    pub fn r(&self) -> OMatrix<T, DimMinimum<R, C>, C>
    where
        DefaultAllocator: Allocator<DimMinimum<R, C>, C>,
    {
        let (nrows, ncols) = self.qr.shape_generic();
        let mut res = self.qr.rows_generic(0, nrows.min(ncols)).upper_triangle();
        res.set_partial_diagonal(self.diag.iter().map(|e| T::from_real(e.clone().modulus())));
        res
    }

    /// Retrieves the upper trapezoidal submatrix `R` of this decomposition.
    ///
    /// This is usually faster than `r` but consumes `self`.
    #[inline]
    pub fn unpack_r(self) -> OMatrix<T, DimMinimum<R, C>, C>
    where
        DefaultAllocator: Reallocator<T, R, C, DimMinimum<R, C>, C>,
    {
        let (nrows, ncols) = self.qr.shape_generic();
        let mut res = self.qr.resize_generic(nrows.min(ncols), ncols, T::zero());
        res.fill_lower_triangle(T::zero(), 1);
        res.set_partial_diagonal(self.diag.iter().map(|e| T::from_real(e.clone().modulus())));
        res
    }

    /// Computes the orthogonal matrix `Q` of this decomposition.
    #[must_use]
    pub fn q(&self) -> OMatrix<T, R, DimMinimum<R, C>>
    where
        DefaultAllocator: Allocator<R, DimMinimum<R, C>>,
    {
        let (nrows, ncols) = self.qr.shape_generic();

        // NOTE: we could build the identity matrix and call q_mul on it.
        // Instead we don't so that we take in account the matrix sparseness.
        let mut res = Matrix::identity_generic(nrows, nrows.min(ncols));
        let dim = self.diag.len();

        for i in (0..dim).rev() {
            let axis = self.qr.view_range(i.., i);
            // TODO: sometimes, the axis might have a zero magnitude.
            let refl = Reflection::new(Unit::new_unchecked(axis), T::zero());

            let mut res_rows = res.view_range_mut(i.., i..);
            refl.reflect_with_sign(&mut res_rows, self.diag[i].clone().signum());
        }

        res
    }

    /// Unpacks this decomposition into its two matrix factors.
    pub fn unpack(
        self,
    ) -> (
        OMatrix<T, R, DimMinimum<R, C>>,
        OMatrix<T, DimMinimum<R, C>, C>,
    )
    where
        DimMinimum<R, C>: DimMin<C, Output = DimMinimum<R, C>>,
        DefaultAllocator:
            Allocator<R, DimMinimum<R, C>> + Reallocator<T, R, C, DimMinimum<R, C>, C>,
    {
        (self.q(), self.unpack_r())
    }

    #[doc(hidden)]
    pub const fn qr_internal(&self) -> &OMatrix<T, R, C> {
        &self.qr
    }

    #[must_use]
    pub(crate) const fn diag_internal(&self) -> &OVector<T, DimMinimum<R, C>> {
        &self.diag
    }

    /// Multiplies the provided matrix by the transpose of the `Q` matrix of this decomposition.
    pub fn q_tr_mul<R2: Dim, C2: Dim, S2>(&self, rhs: &mut Matrix<T, R2, C2, S2>)
    // TODO: do we need a static constraint on the number of rows of rhs?
    where
        S2: StorageMut<T, R2, C2>,
    {
        let dim = self.diag.len();

        for i in 0..dim {
            let axis = self.qr.view_range(i.., i);
            let refl = Reflection::new(Unit::new_unchecked(axis), T::zero());

            let mut rhs_rows = rhs.rows_range_mut(i..);
            refl.reflect_with_sign(&mut rhs_rows, self.diag[i].clone().signum().conjugate());
        }
    }
}

impl<T: ComplexField, D: DimMin<D, Output = D>> QR<T, D, D>
where
    DefaultAllocator: Allocator<D, D> + Allocator<D>,
{
    /// Solves the linear system `self * x = b`, where `x` is the unknown to be determined.
    ///
    /// Returns `None` if `self` is not invertible.
    #[must_use = "Did you mean to use solve_mut()?"]
    pub fn solve<R2: Dim, C2: Dim, S2>(
        &self,
        b: &Matrix<T, R2, C2, S2>,
    ) -> Option<OMatrix<T, R2, C2>>
    where
        S2: Storage<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
        DefaultAllocator: Allocator<R2, C2>,
    {
        let mut res = b.clone_owned();

        if self.solve_mut(&mut res) {
            Some(res)
        } else {
            None
        }
    }

    /// Solves the linear system `self * x = b`, where `x` is the unknown to be determined.
    ///
    /// If the decomposed matrix is not invertible, this returns `false` and its input `b` is
    /// overwritten with garbage.
    pub fn solve_mut<R2: Dim, C2: Dim, S2>(&self, b: &mut Matrix<T, R2, C2, S2>) -> bool
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        assert_eq!(
            self.qr.nrows(),
            b.nrows(),
            "QR solve matrix dimension mismatch."
        );
        assert!(
            self.qr.is_square(),
            "QR solve: unable to solve a non-square system."
        );

        self.q_tr_mul(b);
        self.solve_upper_triangular_mut(b)
    }

    // TODO: duplicate code from the `solve` module.
    fn solve_upper_triangular_mut<R2: Dim, C2: Dim, S2>(
        &self,
        b: &mut Matrix<T, R2, C2, S2>,
    ) -> bool
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let dim = self.qr.nrows();

        for k in 0..b.ncols() {
            let mut b = b.column_mut(k);
            for i in (0..dim).rev() {
                let coeff;

                unsafe {
                    let diag = self.diag.vget_unchecked(i).clone().modulus();

                    if diag.is_zero() {
                        return false;
                    }

                    coeff = b.vget_unchecked(i).clone().unscale(diag);
                    *b.vget_unchecked_mut(i) = coeff.clone();
                }

                b.rows_range_mut(..i)
                    .axpy(-coeff, &self.qr.view_range(..i, i), T::one());
            }
        }

        true
    }

    /// Computes the inverse of the decomposed matrix.
    ///
    /// Returns `None` if the decomposed matrix is not invertible.
    #[must_use]
    pub fn try_inverse(&self) -> Option<OMatrix<T, D, D>> {
        assert!(
            self.qr.is_square(),
            "QR inverse: unable to compute the inverse of a non-square matrix."
        );

        // TODO: is there a less naive method ?
        let (nrows, ncols) = self.qr.shape_generic();
        let mut res = OMatrix::identity_generic(nrows, ncols);

        if self.solve_mut(&mut res) {
            Some(res)
        } else {
            None
        }
    }

    /// Indicates if the decomposed matrix is invertible.
    #[must_use]
    pub fn is_invertible(&self) -> bool {
        assert!(
            self.qr.is_square(),
            "QR: unable to test the invertibility of a non-square matrix."
        );

        for i in 0..self.diag.len() {
            if self.diag[i].is_zero() {
                return false;
            }
        }

        true
    }

    // /// Computes the determinant of the decomposed matrix.
    // pub fn determinant(&self) -> T {
    //     let dim = self.qr.nrows();
    //     assert!(self.qr.is_square(), "QR determinant: unable to compute the determinant of a non-square matrix.");

    //     let mut res = T::one();
    //     for i in 0 .. dim {
    //         res *= unsafe { *self.diag.vget_unchecked(i) };
    //     }

    //     res self.q_determinant()
    // }
}
