#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use crate::allocator::Allocator;
use crate::base::{DefaultAllocator, Matrix, OMatrix, OVector, Unit};
use crate::dimension::{Const, Dim, DimDiff, DimMin, DimMinimum, DimSub, U1};
use simba::scalar::ComplexField;

use crate::geometry::Reflection;
use crate::linalg::householder;
use crate::num::Zero;
use std::mem::MaybeUninit;

/// The bidiagonalization of a general matrix.
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "DimMinimum<R, C>: DimSub<U1>,
         DefaultAllocator: Allocator<R, C>             +
                           Allocator<DimMinimum<R, C>> +
                           Allocator<DimDiff<DimMinimum<R, C>, U1>>,
         OMatrix<T, R, C>: Serialize,
         OVector<T, DimMinimum<R, C>>: Serialize,
         OVector<T, DimDiff<DimMinimum<R, C>, U1>>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(deserialize = "DimMinimum<R, C>: DimSub<U1>,
         DefaultAllocator: Allocator<R, C>             +
                           Allocator<DimMinimum<R, C>> +
                           Allocator<DimDiff<DimMinimum<R, C>, U1>>,
         OMatrix<T, R, C>: Deserialize<'de>,
         OVector<T, DimMinimum<R, C>>: Deserialize<'de>,
         OVector<T, DimDiff<DimMinimum<R, C>, U1>>: Deserialize<'de>"))
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug)]
pub struct Bidiagonal<T: ComplexField, R: DimMin<C>, C: Dim>
where
    DimMinimum<R, C>: DimSub<U1>,
    DefaultAllocator:
        Allocator<R, C> + Allocator<DimMinimum<R, C>> + Allocator<DimDiff<DimMinimum<R, C>, U1>>,
{
    // TODO: perhaps we should pack the axes into different vectors so that axes for `v_t` are
    // contiguous. This prevents some useless copies.
    uv: OMatrix<T, R, C>,
    /// The diagonal elements of the decomposed matrix.
    diagonal: OVector<T, DimMinimum<R, C>>,
    /// The off-diagonal elements of the decomposed matrix.
    off_diagonal: OVector<T, DimDiff<DimMinimum<R, C>, U1>>,
    upper_diagonal: bool,
}

impl<T: ComplexField, R: DimMin<C>, C: Dim> Copy for Bidiagonal<T, R, C>
where
    DimMinimum<R, C>: DimSub<U1>,
    DefaultAllocator:
        Allocator<R, C> + Allocator<DimMinimum<R, C>> + Allocator<DimDiff<DimMinimum<R, C>, U1>>,
    OMatrix<T, R, C>: Copy,
    OVector<T, DimMinimum<R, C>>: Copy,
    OVector<T, DimDiff<DimMinimum<R, C>, U1>>: Copy,
{
}

impl<T: ComplexField, R: DimMin<C>, C: Dim> Bidiagonal<T, R, C>
where
    DimMinimum<R, C>: DimSub<U1>,
    DefaultAllocator: Allocator<R, C>
        + Allocator<C>
        + Allocator<R>
        + Allocator<DimMinimum<R, C>>
        + Allocator<DimDiff<DimMinimum<R, C>, U1>>,
{
    /// Computes the Bidiagonal decomposition using householder reflections.
    pub fn new(mut matrix: OMatrix<T, R, C>) -> Self {
        let (nrows, ncols) = matrix.shape_generic();
        let min_nrows_ncols = nrows.min(ncols);
        let dim = min_nrows_ncols.value();
        assert!(
            dim != 0,
            "Cannot compute the bidiagonalization of an empty matrix."
        );

        let mut diagonal = Matrix::uninit(min_nrows_ncols, Const::<1>);
        let mut off_diagonal = Matrix::uninit(min_nrows_ncols.sub(Const::<1>), Const::<1>);
        let mut axis_packed = Matrix::zeros_generic(ncols, Const::<1>);
        let mut work = Matrix::zeros_generic(nrows, Const::<1>);

        let upper_diagonal = nrows.value() >= ncols.value();
        if upper_diagonal {
            for ite in 0..dim - 1 {
                diagonal[ite] = MaybeUninit::new(householder::clear_column_unchecked(
                    &mut matrix,
                    ite,
                    0,
                    None,
                ));
                off_diagonal[ite] = MaybeUninit::new(householder::clear_row_unchecked(
                    &mut matrix,
                    &mut axis_packed,
                    &mut work,
                    ite,
                    1,
                ));
            }

            diagonal[dim - 1] = MaybeUninit::new(householder::clear_column_unchecked(
                &mut matrix,
                dim - 1,
                0,
                None,
            ));
        } else {
            for ite in 0..dim - 1 {
                diagonal[ite] = MaybeUninit::new(householder::clear_row_unchecked(
                    &mut matrix,
                    &mut axis_packed,
                    &mut work,
                    ite,
                    0,
                ));
                off_diagonal[ite] = MaybeUninit::new(householder::clear_column_unchecked(
                    &mut matrix,
                    ite,
                    1,
                    None,
                ));
            }

            diagonal[dim - 1] = MaybeUninit::new(householder::clear_row_unchecked(
                &mut matrix,
                &mut axis_packed,
                &mut work,
                dim - 1,
                0,
            ));
        }

        // Safety: diagonal and off_diagonal have been fully initialized.
        let (diagonal, off_diagonal) =
            unsafe { (diagonal.assume_init(), off_diagonal.assume_init()) };

        Bidiagonal {
            uv: matrix,
            diagonal,
            off_diagonal,
            upper_diagonal,
        }
    }

    /// Indicates whether this decomposition contains an upper-diagonal matrix.
    #[inline]
    #[must_use]
    pub const fn is_upper_diagonal(&self) -> bool {
        self.upper_diagonal
    }

    #[inline]
    const fn axis_shift(&self) -> (usize, usize) {
        if self.upper_diagonal { (0, 1) } else { (1, 0) }
    }

    /// Unpacks this decomposition into its three matrix factors `(U, D, V^t)`.
    ///
    /// The decomposed matrix `M` is equal to `U * D * V^t`.
    #[inline]
    pub fn unpack(
        self,
    ) -> (
        OMatrix<T, R, DimMinimum<R, C>>,
        OMatrix<T, DimMinimum<R, C>, DimMinimum<R, C>>,
        OMatrix<T, DimMinimum<R, C>, C>,
    )
    where
        DefaultAllocator: Allocator<DimMinimum<R, C>, DimMinimum<R, C>>
            + Allocator<R, DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>, C>,
    {
        // TODO: optimize by calling a reallocator.
        (self.u(), self.d(), self.v_t())
    }

    /// Retrieves the upper trapezoidal submatrix `R` of this decomposition.
    #[inline]
    #[must_use]
    pub fn d(&self) -> OMatrix<T, DimMinimum<R, C>, DimMinimum<R, C>>
    where
        DefaultAllocator: Allocator<DimMinimum<R, C>, DimMinimum<R, C>>,
    {
        let (nrows, ncols) = self.uv.shape_generic();

        let d = nrows.min(ncols);
        let mut res = OMatrix::identity_generic(d, d);
        res.set_partial_diagonal(
            self.diagonal
                .iter()
                .map(|e| T::from_real(e.clone().modulus())),
        );

        let start = self.axis_shift();
        res.view_mut(start, (d.value() - 1, d.value() - 1))
            .set_partial_diagonal(
                self.off_diagonal
                    .iter()
                    .map(|e| T::from_real(e.clone().modulus())),
            );
        res
    }

    /// Computes the orthogonal matrix `U` of this `U * D * V` decomposition.
    // TODO: code duplication with householder::assemble_q.
    // Except that we are returning a rectangular matrix here.
    #[must_use]
    pub fn u(&self) -> OMatrix<T, R, DimMinimum<R, C>>
    where
        DefaultAllocator: Allocator<R, DimMinimum<R, C>>,
    {
        let (nrows, ncols) = self.uv.shape_generic();

        let mut res = Matrix::identity_generic(nrows, nrows.min(ncols));
        let dim = self.diagonal.len();
        let shift = self.axis_shift().0;

        for i in (0..dim - shift).rev() {
            let axis = self.uv.view_range(i + shift.., i);

            // Sometimes, the axis might have a zero magnitude.
            if axis.norm_squared().is_zero() {
                continue;
            }
            let refl = Reflection::new(Unit::new_unchecked(axis), T::zero());

            let mut res_rows = res.view_range_mut(i + shift.., i..);

            let sign = if self.upper_diagonal {
                self.diagonal[i].clone().signum()
            } else {
                self.off_diagonal[i].clone().signum()
            };

            refl.reflect_with_sign(&mut res_rows, sign);
        }

        res
    }

    /// Computes the orthogonal matrix `V_t` of this `U * D * V_t` decomposition.
    #[must_use]
    pub fn v_t(&self) -> OMatrix<T, DimMinimum<R, C>, C>
    where
        DefaultAllocator: Allocator<DimMinimum<R, C>, C>,
    {
        let (nrows, ncols) = self.uv.shape_generic();
        let min_nrows_ncols = nrows.min(ncols);

        let mut res = Matrix::identity_generic(min_nrows_ncols, ncols);
        let mut work = Matrix::zeros_generic(min_nrows_ncols, Const::<1>);
        let mut axis_packed = Matrix::zeros_generic(ncols, Const::<1>);

        let shift = self.axis_shift().1;

        for i in (0..min_nrows_ncols.value() - shift).rev() {
            let axis = self.uv.view_range(i, i + shift..);
            let mut axis_packed = axis_packed.rows_range_mut(i + shift..);
            axis_packed.tr_copy_from(&axis);

            // Sometimes, the axis might have a zero magnitude.
            if axis_packed.norm_squared().is_zero() {
                continue;
            }
            let refl = Reflection::new(Unit::new_unchecked(axis_packed), T::zero());

            let mut res_rows = res.view_range_mut(i.., i + shift..);

            let sign = if self.upper_diagonal {
                self.off_diagonal[i].clone().signum()
            } else {
                self.diagonal[i].clone().signum()
            };

            refl.reflect_rows_with_sign(&mut res_rows, &mut work.rows_range_mut(i..), sign);
        }

        res
    }

    /// The diagonal part of this decomposed matrix.
    #[must_use]
    pub fn diagonal(&self) -> OVector<T::RealField, DimMinimum<R, C>>
    where
        DefaultAllocator: Allocator<DimMinimum<R, C>>,
    {
        self.diagonal.map(|e| e.modulus())
    }

    /// The off-diagonal part of this decomposed matrix.
    #[must_use]
    pub fn off_diagonal(&self) -> OVector<T::RealField, DimDiff<DimMinimum<R, C>, U1>>
    where
        DefaultAllocator: Allocator<DimDiff<DimMinimum<R, C>, U1>>,
    {
        self.off_diagonal.map(|e| e.modulus())
    }

    #[doc(hidden)]
    pub const fn uv_internal(&self) -> &OMatrix<T, R, C> {
        &self.uv
    }
}

// impl<T: ComplexField, D: DimMin<D, Output = D> + DimSub<Dyn>> Bidiagonal<T, D, D>
//     where DefaultAllocator: Allocator<D, D> +
//                             Allocator<D> {
//     /// Solves the linear system `self * x = b`, where `x` is the unknown to be determined.
//     pub fn solve<R2: Dim, C2: Dim, S2>(&self, b: &Matrix<T, R2, C2, S2>) -> OMatrix<T, R2, C2>
//         where S2: StorageMut<T, R2, C2>,
//               ShapeConstraint: SameNumberOfRows<R2, D> {
//         let mut res = b.clone_owned();
//         self.solve_mut(&mut res);
//         res
//     }
//
//     /// Solves the linear system `self * x = b`, where `x` is the unknown to be determined.
//     pub fn solve_mut<R2: Dim, C2: Dim, S2>(&self, b: &mut Matrix<T, R2, C2, S2>)
//         where S2: StorageMut<T, R2, C2>,
//               ShapeConstraint: SameNumberOfRows<R2, D> {
//
//         assert_eq!(self.uv.nrows(), b.nrows(), "Bidiagonal solve matrix dimension mismatch.");
//         assert!(self.uv.is_square(), "Bidiagonal solve: unable to solve a non-square system.");
//
//         self.q_tr_mul(b);
//         self.solve_upper_triangular_mut(b);
//     }
//
//     // TODO: duplicate code from the `solve` module.
//     fn solve_upper_triangular_mut<R2: Dim, C2: Dim, S2>(&self, b: &mut Matrix<T, R2, C2, S2>)
//         where S2: StorageMut<T, R2, C2>,
//               ShapeConstraint: SameNumberOfRows<R2, D> {
//
//         let dim  = self.uv.nrows();
//
//         for k in 0 .. b.ncols() {
//             let mut b = b.column_mut(k);
//             for i in (0 .. dim).rev() {
//                 let coeff;
//
//                 unsafe {
//                     let diag = *self.diag.vget_unchecked(i);
//                     coeff = *b.vget_unchecked(i) / diag;
//                     *b.vget_unchecked_mut(i) = coeff;
//                 }
//
//                 b.rows_range_mut(.. i).axpy(-coeff, &self.uv.view_range(.. i, i), T::one());
//             }
//         }
//     }
//
//     /// Computes the inverse of the decomposed matrix.
//     pub fn inverse(&self) -> OMatrix<T, D, D> {
//         assert!(self.uv.is_square(), "Bidiagonal inverse: unable to compute the inverse of a non-square matrix.");
//
//         // TODO: is there a less naive method ?
//         let (nrows, ncols) = self.uv.shape_generic();
//         let mut res = OMatrix::identity_generic(nrows, ncols);
//         self.solve_mut(&mut res);
//         res
//     }
//
//     // /// Computes the determinant of the decomposed matrix.
//     // pub fn determinant(&self) -> T {
//     //     let dim = self.uv.nrows();
//     //     assert!(self.uv.is_square(), "Bidiagonal determinant: unable to compute the determinant of a non-square matrix.");
//
//     //     let mut res = T::one();
//     //     for i in 0 .. dim {
//     //         res *= unsafe { *self.diag.vget_unchecked(i) };
//     //     }
//
//     //     res self.q_determinant()
//     // }
// }
