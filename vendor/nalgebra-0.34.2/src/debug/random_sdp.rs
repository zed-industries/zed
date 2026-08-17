#[cfg(feature = "arbitrary")]
use crate::base::storage::Owned;
#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

use crate::base::Scalar;
use crate::base::allocator::Allocator;
use crate::base::dimension::{Dim, Dyn};
use crate::base::{DefaultAllocator, OMatrix};
use simba::scalar::ComplexField;

use crate::debug::RandomOrthogonal;

/// A random, well-conditioned, symmetric definite-positive matrix.
#[derive(Clone, Debug)]
pub struct RandomSDP<T: Scalar, D: Dim = Dyn>
where
    DefaultAllocator: Allocator<D, D>,
{
    m: OMatrix<T, D, D>,
}

impl<T: ComplexField, D: Dim> RandomSDP<T, D>
where
    DefaultAllocator: Allocator<D, D>,
{
    /// Retrieve the generated matrix.
    pub fn unwrap(self) -> OMatrix<T, D, D> {
        self.m
    }

    /// Creates a new well conditioned symmetric definite-positive matrix from its dimension and a
    /// random reals generators.
    pub fn new<Rand: FnMut() -> T>(dim: D, mut rand: Rand) -> Self {
        let mut m = RandomOrthogonal::new(dim, &mut rand).unwrap();
        let mt = m.adjoint();

        for i in 0..dim.value() {
            let mut col = m.column_mut(i);
            let eigenval = T::one() + T::from_real(rand().modulus());
            col *= eigenval;
        }

        RandomSDP { m: m * mt }
    }
}

#[cfg(feature = "arbitrary")]
impl<T: ComplexField + Arbitrary + Send, D: Dim> Arbitrary for RandomSDP<T, D>
where
    DefaultAllocator: Allocator<D, D>,
    Owned<T, D, D>: Clone + Send,
{
    fn arbitrary(g: &mut Gen) -> Self {
        let dim = D::try_to_usize().unwrap_or(1 + usize::arbitrary(g) % 50);
        Self::new(D::from_usize(dim), || T::arbitrary(g))
    }
}
