use simba::scalar::{RealField, SubsetOf};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, OMatrix};

use crate::geometry::{SuperTCategoryOf, TCategory, Transform};

impl<T1, T2, C1, C2, const D: usize> SubsetOf<Transform<T2, C2, D>> for Transform<T1, C1, D>
where
    T1: RealField + SubsetOf<T2>,
    T2: RealField,
    C1: TCategory,
    C2: SuperTCategoryOf<C1>,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    T1::Epsilon: Copy,
    T2::Epsilon: Copy,
{
    #[inline]
    fn to_superset(&self) -> Transform<T2, C2, D> {
        Transform::from_matrix_unchecked(self.to_homogeneous().to_superset())
    }

    #[inline]
    fn is_in_subset(t: &Transform<T2, C2, D>) -> bool {
        <Self as SubsetOf<_>>::is_in_subset(t.matrix())
    }

    #[inline]
    fn from_superset_unchecked(t: &Transform<T2, C2, D>) -> Self {
        Self::from_superset_unchecked(t.matrix())
    }
}

impl<T1, T2, C, const D: usize>
    SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>
    for Transform<T1, C, D>
where
    T1: RealField + SubsetOf<T2>,
    T2: RealField,
    C: TCategory,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    T1::Epsilon: Copy,
    T2::Epsilon: Copy,
{
    #[inline]
    fn to_superset(&self) -> OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        self.matrix().to_superset()
    }

    #[inline]
    fn is_in_subset(m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>) -> bool {
        C::check_homogeneous_invariants(m)
    }

    #[inline]
    fn from_superset_unchecked(
        m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    ) -> Self {
        Self::from_matrix_unchecked(crate::convert_ref_unchecked(m))
    }
}

impl<T: RealField, C, const D: usize> From<Transform<T, C, D>>
    for OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
where
    Const<D>: DimNameAdd<U1>,
    C: TCategory,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn from(t: Transform<T, C, D>) -> Self {
        t.to_homogeneous()
    }
}
