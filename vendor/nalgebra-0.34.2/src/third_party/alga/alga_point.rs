use alga::general::{Field, JoinSemilattice, Lattice, MeetSemilattice, RealField};
use alga::linear::{AffineSpace, EuclideanSpace};

use crate::base::{SVector, Scalar};

use crate::geometry::Point;

impl<T: Scalar + Field, const D: usize> AffineSpace for Point<T, D>
where
    T: Scalar + Field,
{
    type Translation = SVector<T, D>;
}

impl<T: RealField + simba::scalar::RealField, const D: usize> EuclideanSpace for Point<T, D> {
    type Coordinates = SVector<T, D>;
    type RealField = T;

    #[inline]
    fn origin() -> Self {
        Self::origin()
    }

    #[inline]
    fn coordinates(&self) -> Self::Coordinates {
        self.coords
    }

    #[inline]
    fn from_coordinates(coords: Self::Coordinates) -> Self {
        Self::from(coords)
    }

    #[inline]
    fn scale_by(&self, n: T) -> Self {
        self * n
    }
}

/*
 *
 * Ordering
 *
 */
impl<T, const D: usize> MeetSemilattice for Point<T, D>
where
    T: Scalar + MeetSemilattice,
{
    #[inline]
    fn meet(&self, other: &Self) -> Self {
        Self::from(self.coords.meet(&other.coords))
    }
}

impl<T, const D: usize> JoinSemilattice for Point<T, D>
where
    T: Scalar + JoinSemilattice,
{
    #[inline]
    fn join(&self, other: &Self) -> Self {
        Self::from(self.coords.join(&other.coords))
    }
}

impl<T, const D: usize> Lattice for Point<T, D>
where
    T: Scalar + Lattice,
{
    #[inline]
    fn meet_join(&self, other: &Self) -> (Self, Self) {
        let (meet, join) = self.coords.meet_join(&other.coords);

        (Self::from(meet), Self::from(join))
    }
}
