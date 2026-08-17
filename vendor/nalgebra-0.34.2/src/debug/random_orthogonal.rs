#[cfg(feature = "arbitrary")]
use crate::base::storage::Owned;
#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

use crate::base::Scalar;
use crate::base::allocator::Allocator;
use crate::base::dimension::{Dim, Dyn};
use crate::base::{DefaultAllocator, OMatrix};
use crate::linalg::givens::GivensRotation;
use simba::scalar::ComplexField;

/// A random orthogonal matrix.
#[derive(Clone, Debug)]
pub struct RandomOrthogonal<T: Scalar, D: Dim = Dyn>
where
    DefaultAllocator: Allocator<D, D>,
{
    m: OMatrix<T, D, D>,
}

impl<T: ComplexField, D: Dim> RandomOrthogonal<T, D>
where
    DefaultAllocator: Allocator<D, D>,
{
    /// Retrieve the generated matrix.
    pub fn unwrap(self) -> OMatrix<T, D, D> {
        self.m
    }

    /// Creates a new random orthogonal matrix from its dimension and a random reals generators.
    pub fn new<Rand: FnMut() -> T>(dim: D, mut rand: Rand) -> Self {
        let mut res = OMatrix::identity_generic(dim, dim);

        // Create an orthogonal matrix by composing random Givens rotations rotations.
        for i in 0..dim.value() - 1 {
            let rot = GivensRotation::new(rand(), rand()).0;
            rot.rotate(&mut res.fixed_rows_mut::<2>(i));
        }

        RandomOrthogonal { m: res }
    }
}

#[cfg(feature = "arbitrary")]
impl<T: ComplexField + Arbitrary + Send, D: Dim> Arbitrary for RandomOrthogonal<T, D>
where
    DefaultAllocator: Allocator<D, D>,
    Owned<T, D, D>: Clone + Send,
{
    fn arbitrary(g: &mut Gen) -> Self {
        let dim = D::try_to_usize().unwrap_or(1 + usize::arbitrary(g) % 50);
        Self::new(D::from_usize(dim), || T::arbitrary(g))
    }
}
