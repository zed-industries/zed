use simba::scalar::{RealField, SubsetOf, SupersetOf};
use simba::simd::{PrimitiveSimdValue, SimdRealField, SimdValue};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimMin, DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, OMatrix, Scalar};

use crate::geometry::{
    AbstractRotation, Isometry, Isometry3, Similarity, SuperTCategoryOf, TAffine, Transform,
    Translation, UnitDualQuaternion, UnitQuaternion,
};
use crate::{ArrayStorage, Point, SVector};

/*
 * This file provides the following conversions:
 * =============================================
 *
 * Isometry -> Isometry
 * Isometry3 -> UnitDualQuaternion
 * Isometry -> Similarity
 * Isometry -> Transform
 * Isometry -> Matrix (homogeneous)
 */

impl<T1, T2, R1, R2, const D: usize> SubsetOf<Isometry<T2, R2, D>> for Isometry<T1, R1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R1: AbstractRotation<T1, D> + SubsetOf<R2>,
    R2: AbstractRotation<T2, D>,
{
    #[inline]
    fn to_superset(&self) -> Isometry<T2, R2, D> {
        Isometry::from_parts(self.translation.to_superset(), self.rotation.to_superset())
    }

    #[inline]
    fn is_in_subset(iso: &Isometry<T2, R2, D>) -> bool {
        crate::is_convertible::<_, Translation<T1, D>>(&iso.translation)
            && crate::is_convertible::<_, R1>(&iso.rotation)
    }

    #[inline]
    fn from_superset_unchecked(iso: &Isometry<T2, R2, D>) -> Self {
        Isometry::from_parts(
            iso.translation.to_subset_unchecked(),
            iso.rotation.to_subset_unchecked(),
        )
    }
}

impl<T1, T2> SubsetOf<UnitDualQuaternion<T2>> for Isometry3<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> UnitDualQuaternion<T2> {
        let dq = UnitDualQuaternion::<T1>::from_isometry(self);
        dq.to_superset()
    }

    #[inline]
    fn is_in_subset(dq: &UnitDualQuaternion<T2>) -> bool {
        crate::is_convertible::<_, UnitQuaternion<T1>>(&dq.rotation())
            && crate::is_convertible::<_, Translation<T1, 3>>(&dq.translation())
    }

    #[inline]
    fn from_superset_unchecked(dq: &UnitDualQuaternion<T2>) -> Self {
        let dq: UnitDualQuaternion<T1> = crate::convert_ref_unchecked(dq);
        dq.to_isometry()
    }
}

impl<T1, T2, R1, R2, const D: usize> SubsetOf<Similarity<T2, R2, D>> for Isometry<T1, R1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R1: AbstractRotation<T1, D> + SubsetOf<R2>,
    R2: AbstractRotation<T2, D>,
{
    #[inline]
    fn to_superset(&self) -> Similarity<T2, R2, D> {
        Similarity::from_isometry(self.to_superset(), T2::one())
    }

    #[inline]
    fn is_in_subset(sim: &Similarity<T2, R2, D>) -> bool {
        crate::is_convertible::<_, Isometry<T1, R1, D>>(&sim.isometry) && sim.scaling() == T2::one()
    }

    #[inline]
    fn from_superset_unchecked(sim: &Similarity<T2, R2, D>) -> Self {
        crate::convert_ref_unchecked(&sim.isometry)
    }
}

impl<T1, T2, R, C, const D: usize> SubsetOf<Transform<T2, C, D>> for Isometry<T1, R, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    C: SuperTCategoryOf<TAffine>,
    R: AbstractRotation<T1, D>
        + SubsetOf<OMatrix<T1, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>
        + SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>,
    Const<D>: DimNameAdd<U1> + DimMin<Const<D>, Output = Const<D>>, // needed by .is_special_orthogonal()
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    // + Allocator<T1, D>
    // + Allocator<D>
    // + Allocator<D, D>
    // + Allocator<D>
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
    SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>> for Isometry<T1, R, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T1, D>
        + SubsetOf<OMatrix<T1, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>
        + SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>,
    Const<D>: DimNameAdd<U1> + DimMin<Const<D>, Output = Const<D>>, // needed by .is_special_orthogonal()
    DefaultAllocator: Allocator<Const<D>, Const<1>, Buffer<T1> = ArrayStorage<T1, D, 1>>
        + Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn to_superset(&self) -> OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        self.to_homogeneous().to_superset()
    }

    #[inline]
    fn is_in_subset(m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>) -> bool {
        let rot = m.fixed_view::<D, D>(0, 0);
        let bottom = m.fixed_view::<1, D>(D, 0);

        // Scalar types agree.
        m.iter().all(|e| SupersetOf::<T1>::is_in_subset(e)) &&
        // The block part is a rotation.
        rot.is_special_orthogonal(T2::default_epsilon() * crate::convert(100.0)) &&
        // The bottom row is (0, 0, ..., 1)
        bottom.iter().all(|e| e.is_zero()) && m[(D, D)] == T2::one()
    }

    #[inline]
    fn from_superset_unchecked(
        m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    ) -> Self {
        let t = m.fixed_view::<D, 1>(0, D).into_owned();
        let t = Translation {
            vector: crate::convert_unchecked(t),
        };

        Self::from_parts(t, crate::convert_unchecked(m.clone_owned()))
    }
}

impl<T: SimdRealField, R: AbstractRotation<T, D>, const D: usize> From<Translation<T, D>>
    for Isometry<T, R, D>
{
    #[inline]
    fn from(tra: Translation<T, D>) -> Self {
        Self::from_parts(tra, R::identity())
    }
}

impl<T: SimdRealField, R, const D: usize> From<Isometry<T, R, D>>
    for OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
where
    Const<D>: DimNameAdd<U1>,
    R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>, // + Allocator<D>,
{
    #[inline]
    fn from(iso: Isometry<T, R, D>) -> Self {
        iso.to_homogeneous()
    }
}

impl<T: SimdRealField, R, const D: usize> From<[T; D]> for Isometry<T, R, D>
where
    R: AbstractRotation<T, D>,
{
    #[inline]
    fn from(coords: [T; D]) -> Self {
        Self::from_parts(coords.into(), R::identity())
    }
}

impl<T: SimdRealField, R, const D: usize> From<SVector<T, D>> for Isometry<T, R, D>
where
    R: AbstractRotation<T, D>,
{
    #[inline]
    fn from(coords: SVector<T, D>) -> Self {
        Self::from_parts(coords.into(), R::identity())
    }
}
impl<T: SimdRealField, R, const D: usize> From<Point<T, D>> for Isometry<T, R, D>
where
    R: AbstractRotation<T, D>,
{
    #[inline]
    fn from(coords: Point<T, D>) -> Self {
        Self::from_parts(coords.into(), R::identity())
    }
}

impl<T: Scalar + PrimitiveSimdValue, R, const D: usize>
    From<[Isometry<T::Element, R::Element, D>; 2]> for Isometry<T, R, D>
where
    T: From<[<T as SimdValue>::Element; 2]>,
    R: SimdValue + AbstractRotation<T, D> + From<[<R as SimdValue>::Element; 2]>,
    R::Element: AbstractRotation<T::Element, D>,
    T::Element: Scalar + Copy,
    R::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Isometry<T::Element, R::Element, D>; 2]) -> Self {
        let tra = Translation::from([arr[0].translation, arr[1].translation]);
        let rot = R::from([arr[0].rotation, arr[0].rotation]);

        Self::from_parts(tra, rot)
    }
}

impl<T: Scalar + PrimitiveSimdValue, R, const D: usize>
    From<[Isometry<T::Element, R::Element, D>; 4]> for Isometry<T, R, D>
where
    T: From<[<T as SimdValue>::Element; 4]>,
    R: SimdValue + AbstractRotation<T, D> + From<[<R as SimdValue>::Element; 4]>,
    R::Element: AbstractRotation<T::Element, D>,
    T::Element: Scalar + Copy,
    R::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Isometry<T::Element, R::Element, D>; 4]) -> Self {
        let tra = Translation::from([
            arr[0].translation,
            arr[1].translation,
            arr[2].translation,
            arr[3].translation,
        ]);
        let rot = R::from([
            arr[0].rotation,
            arr[1].rotation,
            arr[2].rotation,
            arr[3].rotation,
        ]);

        Self::from_parts(tra, rot)
    }
}

impl<T: Scalar + PrimitiveSimdValue, R, const D: usize>
    From<[Isometry<T::Element, R::Element, D>; 8]> for Isometry<T, R, D>
where
    T: From<[<T as SimdValue>::Element; 8]>,
    R: SimdValue + AbstractRotation<T, D> + From<[<R as SimdValue>::Element; 8]>,
    R::Element: AbstractRotation<T::Element, D>,
    T::Element: Scalar + Copy,
    R::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Isometry<T::Element, R::Element, D>; 8]) -> Self {
        let tra = Translation::from([
            arr[0].translation,
            arr[1].translation,
            arr[2].translation,
            arr[3].translation,
            arr[4].translation,
            arr[5].translation,
            arr[6].translation,
            arr[7].translation,
        ]);
        let rot = R::from([
            arr[0].rotation,
            arr[1].rotation,
            arr[2].rotation,
            arr[3].rotation,
            arr[4].rotation,
            arr[5].rotation,
            arr[6].rotation,
            arr[7].rotation,
        ]);

        Self::from_parts(tra, rot)
    }
}

impl<T: Scalar + PrimitiveSimdValue, R, const D: usize>
    From<[Isometry<T::Element, R::Element, D>; 16]> for Isometry<T, R, D>
where
    T: From<[<T as SimdValue>::Element; 16]>,
    R: SimdValue + AbstractRotation<T, D> + From<[<R as SimdValue>::Element; 16]>,
    R::Element: AbstractRotation<T::Element, D>,
    T::Element: Scalar + Copy,
    R::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Isometry<T::Element, R::Element, D>; 16]) -> Self {
        let tra = Translation::from([
            arr[0].translation,
            arr[1].translation,
            arr[2].translation,
            arr[3].translation,
            arr[4].translation,
            arr[5].translation,
            arr[6].translation,
            arr[7].translation,
            arr[8].translation,
            arr[9].translation,
            arr[10].translation,
            arr[11].translation,
            arr[12].translation,
            arr[13].translation,
            arr[14].translation,
            arr[15].translation,
        ]);
        let rot = R::from([
            arr[0].rotation,
            arr[1].rotation,
            arr[2].rotation,
            arr[3].rotation,
            arr[4].rotation,
            arr[5].rotation,
            arr[6].rotation,
            arr[7].rotation,
            arr[8].rotation,
            arr[9].rotation,
            arr[10].rotation,
            arr[11].rotation,
            arr[12].rotation,
            arr[13].rotation,
            arr[14].rotation,
            arr[15].rotation,
        ]);

        Self::from_parts(tra, rot)
    }
}
