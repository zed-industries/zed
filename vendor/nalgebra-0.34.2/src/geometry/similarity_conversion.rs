use num::Zero;

use crate::ArrayStorage;
use simba::scalar::{RealField, SubsetOf, SupersetOf};
use simba::simd::{PrimitiveSimdValue, SimdRealField, SimdValue};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimMin, DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, OMatrix, Scalar};

use crate::geometry::{
    AbstractRotation, Isometry, Similarity, SuperTCategoryOf, TAffine, Transform, Translation,
};

/*
 * This file provides the following conversions:
 * =============================================
 *
 * Similarity -> Similarity
 * Similarity -> Transform
 * Similarity -> Matrix (homogeneous)
 */

impl<T1, T2, R1, R2, const D: usize> SubsetOf<Similarity<T2, R2, D>> for Similarity<T1, R1, D>
where
    T1: RealField + SubsetOf<T2>,
    T2: RealField + SupersetOf<T1>,
    R1: AbstractRotation<T1, D> + SubsetOf<R2>,
    R2: AbstractRotation<T2, D>,
{
    #[inline]
    fn to_superset(&self) -> Similarity<T2, R2, D> {
        Similarity::from_isometry(self.isometry.to_superset(), self.scaling().to_superset())
    }

    #[inline]
    fn is_in_subset(sim: &Similarity<T2, R2, D>) -> bool {
        crate::is_convertible::<_, Isometry<T1, R1, D>>(&sim.isometry)
            && crate::is_convertible::<_, T1>(&sim.scaling())
    }

    #[inline]
    fn from_superset_unchecked(sim: &Similarity<T2, R2, D>) -> Self {
        Similarity::from_isometry(
            sim.isometry.to_subset_unchecked(),
            sim.scaling().to_subset_unchecked(),
        )
    }
}

impl<T1, T2, R, C, const D: usize> SubsetOf<Transform<T2, C, D>> for Similarity<T1, R, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    C: SuperTCategoryOf<TAffine>,
    R: AbstractRotation<T1, D>
        + SubsetOf<OMatrix<T1, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>
        + SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>,
    Const<D>: DimNameAdd<U1> + DimMin<Const<D>, Output = Const<D>>, // needed by .determinant()
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    // + Allocator<D>
    // + Allocator<T1, D>
    // + Allocator<T1, D, D>
    // + Allocator<D, D>
    // + Allocator<D>,
{
    #[inline]
    fn to_superset(&self) -> Transform<T2, C, D> {
        Transform::from_matrix_unchecked(self.to_homogeneous().to_superset())
    }

    #[inline]
    fn is_in_subset(t: &Transform<T2, C, D>) -> bool {
        <Self as SubsetOf<_>>::is_in_subset(t.matrix())
    }

    #[inline]
    fn from_superset_unchecked(t: &Transform<T2, C, D>) -> Self {
        Self::from_superset_unchecked(t.matrix())
    }
}

impl<T1, T2, R, const D: usize>
    SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>
    for Similarity<T1, R, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T1, D>
        + SubsetOf<OMatrix<T1, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>
        + SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>,
    Const<D>: DimNameAdd<U1> + DimMin<Const<D>, Output = Const<D>>, // needed by .determinant()
    DefaultAllocator: Allocator<Const<D>, Const<1>, Buffer<T1> = ArrayStorage<T1, D, 1>>
        + Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn to_superset(&self) -> OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        self.to_homogeneous().to_superset()
    }

    #[inline]
    fn is_in_subset(m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>) -> bool {
        let mut rot = m.fixed_view::<D, D>(0, 0).clone_owned();
        if rot
            .fixed_columns_mut::<1>(0)
            .try_normalize_mut(T2::zero())
            .is_some()
            && rot
                .fixed_columns_mut::<1>(1)
                .try_normalize_mut(T2::zero())
                .is_some()
            && rot
                .fixed_columns_mut::<1>(2)
                .try_normalize_mut(T2::zero())
                .is_some()
        {
            // TODO: could we avoid explicit the computation of the determinant?
            // (its sign is needed to see if the scaling factor is negative).
            if rot.determinant() < T2::zero() {
                rot.fixed_columns_mut::<1>(0).neg_mut();
                rot.fixed_columns_mut::<1>(1).neg_mut();
                rot.fixed_columns_mut::<1>(2).neg_mut();
            }

            let bottom = m.fixed_view::<1, D>(D, 0);
            // Scalar types agree.
            m.iter().all(|e| SupersetOf::<T1>::is_in_subset(e)) &&
            // The normalized block part is a rotation.
            // rot.is_special_orthogonal(T2::default_epsilon().sqrt()) &&
            // The bottom row is (0, 0, ..., 1)
            bottom.iter().all(|e| e.is_zero()) && m[(D, D)] == T2::one()
        } else {
            false
        }
    }

    #[inline]
    fn from_superset_unchecked(
        m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    ) -> Self {
        let mut mm = m.clone_owned();
        let na = mm.fixed_view_mut::<D, 1>(0, 0).normalize_mut();
        let nb = mm.fixed_view_mut::<D, 1>(0, 1).normalize_mut();
        let nc = mm.fixed_view_mut::<D, 1>(0, 2).normalize_mut();

        let mut scale = (na + nb + nc) / crate::convert(3.0); // We take the mean, for robustness.

        // TODO: could we avoid the explicit computation of the determinant?
        // (its sign is needed to see if the scaling factor is negative).
        if mm.fixed_view::<D, D>(0, 0).determinant() < T2::zero() {
            mm.fixed_view_mut::<D, 1>(0, 0).neg_mut();
            mm.fixed_view_mut::<D, 1>(0, 1).neg_mut();
            mm.fixed_view_mut::<D, 1>(0, 2).neg_mut();
            scale = -scale;
        }

        let t = m.fixed_view::<D, 1>(0, D).into_owned();
        let t = Translation {
            vector: crate::convert_unchecked(t),
        };

        Self::from_parts(
            t,
            crate::convert_unchecked(mm),
            crate::convert_unchecked(scale),
        )
    }
}

impl<T: SimdRealField, R, const D: usize> From<Similarity<T, R, D>>
    for OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
where
    Const<D>: DimNameAdd<U1>,
    R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>, // + Allocator<D>
{
    #[inline]
    fn from(sim: Similarity<T, R, D>) -> Self {
        sim.to_homogeneous()
    }
}

impl<T: Scalar + Zero + PrimitiveSimdValue, R, const D: usize>
    From<[Similarity<T::Element, R::Element, D>; 2]> for Similarity<T, R, D>
where
    T: From<[<T as SimdValue>::Element; 2]>,
    R: SimdValue + AbstractRotation<T, D> + From<[<R as SimdValue>::Element; 2]>,
    R::Element: AbstractRotation<T::Element, D>,
    T::Element: Scalar + Zero + Copy,
    R::Element: Scalar + Zero + Copy,
{
    #[inline]
    fn from(arr: [Similarity<T::Element, R::Element, D>; 2]) -> Self {
        let iso = Isometry::from([arr[0].isometry, arr[1].isometry]);
        let scale = T::from([arr[0].scaling(), arr[1].scaling()]);

        Self::from_isometry(iso, scale)
    }
}

impl<T: Scalar + Zero + PrimitiveSimdValue, R, const D: usize>
    From<[Similarity<T::Element, R::Element, D>; 4]> for Similarity<T, R, D>
where
    T: From<[<T as SimdValue>::Element; 4]>,
    R: SimdValue + AbstractRotation<T, D> + From<[<R as SimdValue>::Element; 4]>,
    R::Element: AbstractRotation<T::Element, D>,
    T::Element: Scalar + Zero + Copy,
    R::Element: Scalar + Zero + Copy,
{
    #[inline]
    fn from(arr: [Similarity<T::Element, R::Element, D>; 4]) -> Self {
        let iso = Isometry::from([
            arr[0].isometry,
            arr[1].isometry,
            arr[2].isometry,
            arr[3].isometry,
        ]);
        let scale = T::from([
            arr[0].scaling(),
            arr[1].scaling(),
            arr[2].scaling(),
            arr[3].scaling(),
        ]);

        Self::from_isometry(iso, scale)
    }
}

impl<T: Scalar + Zero + PrimitiveSimdValue, R, const D: usize>
    From<[Similarity<T::Element, R::Element, D>; 8]> for Similarity<T, R, D>
where
    T: From<[<T as SimdValue>::Element; 8]>,
    R: SimdValue + AbstractRotation<T, D> + From<[<R as SimdValue>::Element; 8]>,
    R::Element: AbstractRotation<T::Element, D>,
    T::Element: Scalar + Zero + Copy,
    R::Element: Scalar + Zero + Copy,
{
    #[inline]
    fn from(arr: [Similarity<T::Element, R::Element, D>; 8]) -> Self {
        let iso = Isometry::from([
            arr[0].isometry,
            arr[1].isometry,
            arr[2].isometry,
            arr[3].isometry,
            arr[4].isometry,
            arr[5].isometry,
            arr[6].isometry,
            arr[7].isometry,
        ]);
        let scale = T::from([
            arr[0].scaling(),
            arr[1].scaling(),
            arr[2].scaling(),
            arr[3].scaling(),
            arr[4].scaling(),
            arr[5].scaling(),
            arr[6].scaling(),
            arr[7].scaling(),
        ]);

        Self::from_isometry(iso, scale)
    }
}

impl<T: Scalar + Zero + PrimitiveSimdValue, R, const D: usize>
    From<[Similarity<T::Element, R::Element, D>; 16]> for Similarity<T, R, D>
where
    T: From<[<T as SimdValue>::Element; 16]>,
    R: SimdValue + AbstractRotation<T, D> + From<[<R as SimdValue>::Element; 16]>,
    R::Element: AbstractRotation<T::Element, D>,
    T::Element: Scalar + Zero + Copy,
    R::Element: Scalar + Zero + Copy,
{
    #[inline]
    fn from(arr: [Similarity<T::Element, R::Element, D>; 16]) -> Self {
        let iso = Isometry::from([
            arr[0].isometry,
            arr[1].isometry,
            arr[2].isometry,
            arr[3].isometry,
            arr[4].isometry,
            arr[5].isometry,
            arr[6].isometry,
            arr[7].isometry,
            arr[8].isometry,
            arr[9].isometry,
            arr[10].isometry,
            arr[11].isometry,
            arr[12].isometry,
            arr[13].isometry,
            arr[14].isometry,
            arr[15].isometry,
        ]);
        let scale = T::from([
            arr[0].scaling(),
            arr[1].scaling(),
            arr[2].scaling(),
            arr[3].scaling(),
            arr[4].scaling(),
            arr[5].scaling(),
            arr[6].scaling(),
            arr[7].scaling(),
            arr[8].scaling(),
            arr[9].scaling(),
            arr[10].scaling(),
            arr[11].scaling(),
            arr[12].scaling(),
            arr[13].scaling(),
            arr[14].scaling(),
            arr[15].scaling(),
        ]);

        Self::from_isometry(iso, scale)
    }
}
