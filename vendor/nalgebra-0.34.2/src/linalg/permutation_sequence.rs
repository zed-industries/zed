#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use num::One;
use simba::scalar::ClosedNeg;

use crate::allocator::Allocator;
use crate::base::{DefaultAllocator, Matrix, OVector, Scalar};
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::dimension::Dyn;
use crate::dimension::{Const, Dim, DimName};
use crate::storage::StorageMut;

/// A sequence of row or column permutations.
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "DefaultAllocator: Allocator<D>,
         OVector<(usize, usize), D>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(deserialize = "DefaultAllocator: Allocator<D>,
         OVector<(usize, usize), D>: Deserialize<'de>"))
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug)]
pub struct PermutationSequence<D: Dim>
where
    DefaultAllocator: Allocator<D>,
{
    len: usize,
    ipiv: OVector<(usize, usize), D>,
}

impl<D: Dim> Copy for PermutationSequence<D>
where
    DefaultAllocator: Allocator<D>,
    OVector<(usize, usize), D>: Copy,
{
}

impl<D: DimName> PermutationSequence<D>
where
    DefaultAllocator: Allocator<D>,
{
    /// Creates a new statically-allocated sequence of `D` identity permutations.
    #[inline]
    pub fn identity() -> Self {
        Self::identity_generic(D::name())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl PermutationSequence<Dyn>
where
    DefaultAllocator: Allocator<Dyn>,
{
    /// Creates a new dynamically-allocated sequence of `n` identity permutations.
    #[inline]
    pub fn identity(n: usize) -> Self {
        Self::identity_generic(Dyn(n))
    }
}

impl<D: Dim> PermutationSequence<D>
where
    DefaultAllocator: Allocator<D>,
{
    /// Creates a new sequence of D identity permutations.
    #[inline]
    pub fn identity_generic(dim: D) -> Self {
        Self {
            len: 0,
            // TODO: using a uninitialized matrix would save some computation, but
            //       that loos difficult to setup with MaybeUninit.
            ipiv: Matrix::repeat_generic(dim, Const::<1>, (0, 0)),
        }
    }

    /// Adds the interchange of the row (or column) `i` with the row (or column) `i2` to this
    /// sequence of permutations.
    #[inline]
    pub fn append_permutation(&mut self, i: usize, i2: usize) {
        if i != i2 {
            assert!(
                self.len < self.ipiv.len(),
                "Maximum number of permutations exceeded."
            );
            self.ipiv[self.len] = (i, i2);
            self.len += 1;
        }
    }

    /// Applies this sequence of permutations to the rows of `rhs`.
    #[inline]
    pub fn permute_rows<T: Scalar, R2: Dim, C2: Dim, S2>(&self, rhs: &mut Matrix<T, R2, C2, S2>)
    where
        S2: StorageMut<T, R2, C2>,
    {
        for i in self.ipiv.rows_range(..self.len).iter() {
            rhs.swap_rows(i.0, i.1)
        }
    }

    /// Applies this sequence of permutations in reverse to the rows of `rhs`.
    #[inline]
    pub fn inv_permute_rows<T: Scalar, R2: Dim, C2: Dim, S2>(&self, rhs: &mut Matrix<T, R2, C2, S2>)
    where
        S2: StorageMut<T, R2, C2>,
    {
        for i in 0..self.len {
            let (i1, i2) = self.ipiv[self.len - i - 1];
            rhs.swap_rows(i1, i2)
        }
    }

    /// Applies this sequence of permutations to the columns of `rhs`.
    #[inline]
    pub fn permute_columns<T: Scalar, R2: Dim, C2: Dim, S2>(&self, rhs: &mut Matrix<T, R2, C2, S2>)
    where
        S2: StorageMut<T, R2, C2>,
    {
        for i in self.ipiv.rows_range(..self.len).iter() {
            rhs.swap_columns(i.0, i.1)
        }
    }

    /// Applies this sequence of permutations in reverse to the columns of `rhs`.
    #[inline]
    pub fn inv_permute_columns<T: Scalar, R2: Dim, C2: Dim, S2>(
        &self,
        rhs: &mut Matrix<T, R2, C2, S2>,
    ) where
        S2: StorageMut<T, R2, C2>,
    {
        for i in 0..self.len {
            let (i1, i2) = self.ipiv[self.len - i - 1];
            rhs.swap_columns(i1, i2)
        }
    }

    /// The number of non-identity permutations applied by this sequence.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the permutation sequence contains no elements.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The determinant of the matrix corresponding to this permutation.
    #[inline]
    #[must_use]
    pub fn determinant<T: One + ClosedNeg>(&self) -> T {
        if self.len.is_multiple_of(2) {
            T::one()
        } else {
            -T::one()
        }
    }
}
