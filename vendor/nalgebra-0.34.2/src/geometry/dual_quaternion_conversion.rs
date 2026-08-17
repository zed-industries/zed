use simba::scalar::{RealField, SubsetOf, SupersetOf};
use simba::simd::SimdRealField;

use crate::base::{Matrix4, Vector4};
use crate::geometry::{
    DualQuaternion, Isometry3, Similarity3, SuperTCategoryOf, TAffine, Transform, Translation3,
    UnitDualQuaternion, UnitQuaternion,
};

/*
 * This file provides the following conversions:
 * =============================================
 *
 * DualQuaternion     -> DualQuaternion
 * UnitDualQuaternion -> UnitDualQuaternion
 * UnitDualQuaternion -> Isometry<3>
 * UnitDualQuaternion -> Similarity<3>
 * UnitDualQuaternion -> Transform<3>
 * UnitDualQuaternion -> Matrix<U4> (homogeneous)
 *
 * NOTE:
 * UnitDualQuaternion -> DualQuaternion is already provided by: Unit<T> -> T
 */

impl<T1, T2> SubsetOf<DualQuaternion<T2>> for DualQuaternion<T1>
where
    T1: SimdRealField,
    T2: SimdRealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> DualQuaternion<T2> {
        DualQuaternion::from_real_and_dual(self.real.to_superset(), self.dual.to_superset())
    }

    #[inline]
    fn is_in_subset(dq: &DualQuaternion<T2>) -> bool {
        crate::is_convertible::<_, Vector4<T1>>(&dq.real.coords)
            && crate::is_convertible::<_, Vector4<T1>>(&dq.dual.coords)
    }

    #[inline]
    fn from_superset_unchecked(dq: &DualQuaternion<T2>) -> Self {
        DualQuaternion::from_real_and_dual(
            dq.real.to_subset_unchecked(),
            dq.dual.to_subset_unchecked(),
        )
    }
}

impl<T1, T2> SubsetOf<UnitDualQuaternion<T2>> for UnitDualQuaternion<T1>
where
    T1: SimdRealField,
    T2: SimdRealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> UnitDualQuaternion<T2> {
        UnitDualQuaternion::new_unchecked(self.as_ref().to_superset())
    }

    #[inline]
    fn is_in_subset(dq: &UnitDualQuaternion<T2>) -> bool {
        crate::is_convertible::<_, DualQuaternion<T1>>(dq.as_ref())
    }

    #[inline]
    fn from_superset_unchecked(dq: &UnitDualQuaternion<T2>) -> Self {
        Self::new_unchecked(crate::convert_ref_unchecked(dq.as_ref()))
    }
}

impl<T1, T2> SubsetOf<Isometry3<T2>> for UnitDualQuaternion<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> Isometry3<T2> {
        let dq: UnitDualQuaternion<T2> = self.to_superset();
        let iso = dq.to_isometry();
        crate::convert_unchecked(iso)
    }

    #[inline]
    fn is_in_subset(iso: &Isometry3<T2>) -> bool {
        crate::is_convertible::<_, UnitQuaternion<T1>>(&iso.rotation)
            && crate::is_convertible::<_, Translation3<T1>>(&iso.translation)
    }

    #[inline]
    fn from_superset_unchecked(iso: &Isometry3<T2>) -> Self {
        let dq = UnitDualQuaternion::<T2>::from_isometry(iso);
        crate::convert_unchecked(dq)
    }
}

impl<T1, T2> SubsetOf<Similarity3<T2>> for UnitDualQuaternion<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> Similarity3<T2> {
        Similarity3::from_isometry(crate::convert_ref(self), T2::one())
    }

    #[inline]
    fn is_in_subset(sim: &Similarity3<T2>) -> bool {
        sim.scaling() == T2::one()
    }

    #[inline]
    fn from_superset_unchecked(sim: &Similarity3<T2>) -> Self {
        crate::convert_ref_unchecked(&sim.isometry)
    }
}

impl<T1, T2, C> SubsetOf<Transform<T2, C, 3>> for UnitDualQuaternion<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    C: SuperTCategoryOf<TAffine>,
{
    #[inline]
    fn to_superset(&self) -> Transform<T2, C, 3> {
        Transform::from_matrix_unchecked(self.clone().to_homogeneous().to_superset())
    }

    #[inline]
    fn is_in_subset(t: &Transform<T2, C, 3>) -> bool {
        <Self as SubsetOf<_>>::is_in_subset(t.matrix())
    }

    #[inline]
    fn from_superset_unchecked(t: &Transform<T2, C, 3>) -> Self {
        Self::from_superset_unchecked(t.matrix())
    }
}

impl<T1: RealField, T2: RealField + SupersetOf<T1>> SubsetOf<Matrix4<T2>>
    for UnitDualQuaternion<T1>
{
    #[inline]
    fn to_superset(&self) -> Matrix4<T2> {
        self.clone().to_homogeneous().to_superset()
    }

    #[inline]
    fn is_in_subset(m: &Matrix4<T2>) -> bool {
        crate::is_convertible::<_, Isometry3<T1>>(m)
    }

    #[inline]
    fn from_superset_unchecked(m: &Matrix4<T2>) -> Self {
        let iso: Isometry3<T1> = crate::convert_ref_unchecked(m);
        Self::from_isometry(&iso)
    }
}

impl<T: SimdRealField + RealField> From<UnitDualQuaternion<T>> for Matrix4<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(dq: UnitDualQuaternion<T>) -> Self {
        dq.to_homogeneous()
    }
}

impl<T: SimdRealField> From<UnitDualQuaternion<T>> for Isometry3<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(dq: UnitDualQuaternion<T>) -> Self {
        dq.to_isometry()
    }
}

impl<T: SimdRealField> From<Isometry3<T>> for UnitDualQuaternion<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(iso: Isometry3<T>) -> Self {
        Self::from_isometry(&iso)
    }
}
