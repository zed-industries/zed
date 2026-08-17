#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use crate::allocator::Allocator;
use crate::base::{DefaultAllocator, OMatrix, OVector};
use crate::dimension::{Const, DimDiff, DimSub, U1};
use simba::scalar::ComplexField;

use crate::Matrix;
use crate::linalg::householder;
use std::mem::MaybeUninit;

/// Tridiagonalization of a symmetric matrix.
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<DimDiff<D, U1>>,
         OMatrix<T, D, D>: Serialize,
         OVector<T, DimDiff<D, U1>>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(deserialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<DimDiff<D, U1>>,
         OMatrix<T, D, D>: Deserialize<'de>,
         OVector<T, DimDiff<D, U1>>: Deserialize<'de>"))
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug)]
pub struct SymmetricTridiagonal<T: ComplexField, D: DimSub<U1>>
where
    DefaultAllocator: Allocator<D, D> + Allocator<DimDiff<D, U1>>,
{
    tri: OMatrix<T, D, D>,
    off_diagonal: OVector<T, DimDiff<D, U1>>,
}

impl<T: ComplexField, D: DimSub<U1>> Copy for SymmetricTridiagonal<T, D>
where
    DefaultAllocator: Allocator<D, D> + Allocator<DimDiff<D, U1>>,
    OMatrix<T, D, D>: Copy,
    OVector<T, DimDiff<D, U1>>: Copy,
{
}

impl<T: ComplexField, D: DimSub<U1>> SymmetricTridiagonal<T, D>
where
    DefaultAllocator: Allocator<D, D> + Allocator<DimDiff<D, U1>>,
{
    /// Computes the tridiagonalization of the symmetric matrix `m`.
    ///
    /// Only the lower-triangular part (including the diagonal) of `m` is read.
    pub fn new(mut m: OMatrix<T, D, D>) -> Self {
        let dim = m.shape_generic().0;

        assert!(
            m.is_square(),
            "Unable to compute the symmetric tridiagonal decomposition of a non-square matrix."
        );
        assert!(
            dim.value() != 0,
            "Unable to compute the symmetric tridiagonal decomposition of an empty matrix."
        );

        let mut off_diagonal = Matrix::uninit(dim.sub(Const::<1>), Const::<1>);
        let mut p = Matrix::zeros_generic(dim.sub(Const::<1>), Const::<1>);

        for i in 0..dim.value() - 1 {
            let mut m = m.rows_range_mut(i + 1..);
            let (mut axis, mut m) = m.columns_range_pair_mut(i, i + 1..);

            let (norm, not_zero) = householder::reflection_axis_mut(&mut axis);
            off_diagonal[i] = MaybeUninit::new(norm);

            if not_zero {
                let mut p = p.rows_range_mut(i..);

                p.hegemv(crate::convert(2.0), &m, &axis, T::zero());

                let dot = axis.dotc(&p);
                m.hegerc(-T::one(), &p, &axis, T::one());
                m.hegerc(-T::one(), &axis, &p, T::one());
                m.hegerc(dot * crate::convert(2.0), &axis, &axis, T::one());
            }
        }

        // Safety: off_diagonal has been fully initialized.
        let off_diagonal = unsafe { off_diagonal.assume_init() };
        Self {
            tri: m,
            off_diagonal,
        }
    }

    #[doc(hidden)]
    // For debugging.
    pub const fn internal_tri(&self) -> &OMatrix<T, D, D> {
        &self.tri
    }

    /// Retrieve the orthogonal transformation, diagonal, and off diagonal elements of this
    /// decomposition.
    pub fn unpack(
        self,
    ) -> (
        OMatrix<T, D, D>,
        OVector<T::RealField, D>,
        OVector<T::RealField, DimDiff<D, U1>>,
    )
    where
        DefaultAllocator: Allocator<D> + Allocator<DimDiff<D, U1>>,
    {
        let diag = self.diagonal();
        let q = self.q();

        (q, diag, self.off_diagonal.map(T::modulus))
    }

    /// Retrieve the diagonal, and off diagonal elements of this decomposition.
    pub fn unpack_tridiagonal(
        self,
    ) -> (
        OVector<T::RealField, D>,
        OVector<T::RealField, DimDiff<D, U1>>,
    )
    where
        DefaultAllocator: Allocator<D> + Allocator<DimDiff<D, U1>>,
    {
        (self.diagonal(), self.off_diagonal.map(T::modulus))
    }

    /// The diagonal components of this decomposition.
    #[must_use]
    pub fn diagonal(&self) -> OVector<T::RealField, D>
    where
        DefaultAllocator: Allocator<D>,
    {
        self.tri.map_diagonal(|e| e.real())
    }

    /// The off-diagonal components of this decomposition.
    #[must_use]
    pub fn off_diagonal(&self) -> OVector<T::RealField, DimDiff<D, U1>>
    where
        DefaultAllocator: Allocator<DimDiff<D, U1>>,
    {
        self.off_diagonal.map(T::modulus)
    }

    /// Computes the orthogonal matrix `Q` of this decomposition.
    #[must_use]
    pub fn q(&self) -> OMatrix<T, D, D> {
        householder::assemble_q(&self.tri, self.off_diagonal.as_slice())
    }

    /// Recomputes the original symmetric matrix.
    pub fn recompose(mut self) -> OMatrix<T, D, D> {
        let q = self.q();
        self.tri.fill_lower_triangle(T::zero(), 2);
        self.tri.fill_upper_triangle(T::zero(), 2);

        for i in 0..self.off_diagonal.len() {
            let val = T::from_real(self.off_diagonal[i].clone().modulus());
            self.tri[(i + 1, i)] = val.clone();
            self.tri[(i, i + 1)] = val;
        }

        &q * self.tri * q.adjoint()
    }
}
