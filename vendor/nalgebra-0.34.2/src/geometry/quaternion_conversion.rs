use num::Zero;

use simba::scalar::{RealField, SubsetOf, SupersetOf};
use simba::simd::{PrimitiveSimdValue, SimdRealField, SimdValue};

use crate::base::{Matrix3, Matrix4, Scalar, Vector4};
use crate::geometry::{
    AbstractRotation, Isometry, Quaternion, Rotation, Rotation3, Similarity, SuperTCategoryOf,
    TAffine, Transform, Translation, UnitDualQuaternion, UnitQuaternion,
};

/*
 * This file provides the following conversions:
 * =============================================
 *
 * Quaternion     -> Quaternion
 * UnitQuaternion -> UnitQuaternion
 * UnitQuaternion -> Rotation<3>
 * UnitQuaternion -> Isometry<3>
 * UnitQuaternion -> UnitDualQuaternion
 * UnitQuaternion -> Similarity<3>
 * UnitQuaternion -> Transform<3>
 * UnitQuaternion -> Matrix<U4> (homogeneous)
 *
 * NOTE:
 * UnitQuaternion -> Quaternion is already provided by: Unit<T> -> T
 */

impl<T1, T2> SubsetOf<Quaternion<T2>> for Quaternion<T1>
where
    T1: Scalar,
    T2: Scalar + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> Quaternion<T2> {
        Quaternion::from(self.coords.to_superset())
    }

    #[inline]
    fn is_in_subset(q: &Quaternion<T2>) -> bool {
        crate::is_convertible::<_, Vector4<T1>>(&q.coords)
    }

    #[inline]
    fn from_superset_unchecked(q: &Quaternion<T2>) -> Self {
        Self {
            coords: q.coords.to_subset_unchecked(),
        }
    }
}

impl<T1, T2> SubsetOf<UnitQuaternion<T2>> for UnitQuaternion<T1>
where
    T1: Scalar,
    T2: Scalar + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> UnitQuaternion<T2> {
        UnitQuaternion::new_unchecked(self.as_ref().to_superset())
    }

    #[inline]
    fn is_in_subset(uq: &UnitQuaternion<T2>) -> bool {
        crate::is_convertible::<_, Quaternion<T1>>(uq.as_ref())
    }

    #[inline]
    fn from_superset_unchecked(uq: &UnitQuaternion<T2>) -> Self {
        Self::new_unchecked(crate::convert_ref_unchecked(uq.as_ref()))
    }
}

impl<T1, T2> SubsetOf<Rotation<T2, 3>> for UnitQuaternion<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> Rotation3<T2> {
        let q: UnitQuaternion<T2> = self.to_superset();
        q.to_rotation_matrix()
    }

    #[inline]
    fn is_in_subset(rot: &Rotation3<T2>) -> bool {
        crate::is_convertible::<_, Rotation3<T1>>(rot)
    }

    #[inline]
    fn from_superset_unchecked(rot: &Rotation3<T2>) -> Self {
        let q = UnitQuaternion::<T2>::from_rotation_matrix(rot);
        crate::convert_unchecked(q)
    }
}

impl<T1, T2, R> SubsetOf<Isometry<T2, R, 3>> for UnitQuaternion<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T2, 3> + SupersetOf<Self>,
{
    #[inline]
    fn to_superset(&self) -> Isometry<T2, R, 3> {
        Isometry::from_parts(Translation::identity(), crate::convert_ref(self))
    }

    #[inline]
    fn is_in_subset(iso: &Isometry<T2, R, 3>) -> bool {
        iso.translation.vector.is_zero()
    }

    #[inline]
    fn from_superset_unchecked(iso: &Isometry<T2, R, 3>) -> Self {
        crate::convert_ref_unchecked(&iso.rotation)
    }
}

impl<T1, T2> SubsetOf<UnitDualQuaternion<T2>> for UnitQuaternion<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> UnitDualQuaternion<T2> {
        let q: UnitQuaternion<T2> = crate::convert_ref(self);
        UnitDualQuaternion::from_rotation(q)
    }

    #[inline]
    fn is_in_subset(dq: &UnitDualQuaternion<T2>) -> bool {
        dq.translation().vector.is_zero()
    }

    #[inline]
    fn from_superset_unchecked(dq: &UnitDualQuaternion<T2>) -> Self {
        crate::convert_unchecked(dq.rotation())
    }
}

impl<T1, T2, R> SubsetOf<Similarity<T2, R, 3>> for UnitQuaternion<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T2, 3> + SupersetOf<Self>,
{
    #[inline]
    fn to_superset(&self) -> Similarity<T2, R, 3> {
        Similarity::from_isometry(crate::convert_ref(self), T2::one())
    }

    #[inline]
    fn is_in_subset(sim: &Similarity<T2, R, 3>) -> bool {
        sim.isometry.translation.vector.is_zero() && sim.scaling() == T2::one()
    }

    #[inline]
    fn from_superset_unchecked(sim: &Similarity<T2, R, 3>) -> Self {
        crate::convert_ref_unchecked(&sim.isometry)
    }
}

impl<T1, T2, C> SubsetOf<Transform<T2, C, 3>> for UnitQuaternion<T1>
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

impl<T1: RealField, T2: RealField + SupersetOf<T1>> SubsetOf<Matrix4<T2>> for UnitQuaternion<T1> {
    #[inline]
    fn to_superset(&self) -> Matrix4<T2> {
        self.clone().to_homogeneous().to_superset()
    }

    #[inline]
    fn is_in_subset(m: &Matrix4<T2>) -> bool {
        crate::is_convertible::<_, Rotation3<T1>>(m)
    }

    #[inline]
    fn from_superset_unchecked(m: &Matrix4<T2>) -> Self {
        let rot: Rotation3<T1> = crate::convert_ref_unchecked(m);
        Self::from_rotation_matrix(&rot)
    }
}

impl<T: SimdRealField> From<UnitQuaternion<T>> for Matrix4<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(q: UnitQuaternion<T>) -> Self {
        q.to_homogeneous()
    }
}

impl<T: SimdRealField> From<UnitQuaternion<T>> for Rotation3<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(q: UnitQuaternion<T>) -> Self {
        q.to_rotation_matrix()
    }
}

impl<T: SimdRealField> From<Rotation3<T>> for UnitQuaternion<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(q: Rotation3<T>) -> Self {
        Self::from_rotation_matrix(&q)
    }
}

impl<T: SimdRealField> From<UnitQuaternion<T>> for Matrix3<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(q: UnitQuaternion<T>) -> Self {
        q.to_rotation_matrix().into_inner()
    }
}

impl<T: Scalar> From<Vector4<T>> for Quaternion<T> {
    #[inline]
    fn from(coords: Vector4<T>) -> Self {
        Self { coords }
    }
}

impl<T: Scalar> From<[T; 4]> for Quaternion<T> {
    #[inline]
    fn from(coords: [T; 4]) -> Self {
        Self {
            coords: coords.into(),
        }
    }
}

impl<T: Scalar + PrimitiveSimdValue> From<[Quaternion<T::Element>; 2]> for Quaternion<T>
where
    T: From<[<T as SimdValue>::Element; 2]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Quaternion<T::Element>; 2]) -> Self {
        Self::from(Vector4::from([arr[0].coords, arr[1].coords]))
    }
}

impl<T: Scalar + PrimitiveSimdValue> From<[Quaternion<T::Element>; 4]> for Quaternion<T>
where
    T: From<[<T as SimdValue>::Element; 4]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Quaternion<T::Element>; 4]) -> Self {
        Self::from(Vector4::from([
            arr[0].coords,
            arr[1].coords,
            arr[2].coords,
            arr[3].coords,
        ]))
    }
}

impl<T: Scalar + PrimitiveSimdValue> From<[Quaternion<T::Element>; 8]> for Quaternion<T>
where
    T: From<[<T as SimdValue>::Element; 8]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Quaternion<T::Element>; 8]) -> Self {
        Self::from(Vector4::from([
            arr[0].coords,
            arr[1].coords,
            arr[2].coords,
            arr[3].coords,
            arr[4].coords,
            arr[5].coords,
            arr[6].coords,
            arr[7].coords,
        ]))
    }
}

impl<T: Scalar + PrimitiveSimdValue> From<[Quaternion<T::Element>; 16]> for Quaternion<T>
where
    T: From<[<T as SimdValue>::Element; 16]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Quaternion<T::Element>; 16]) -> Self {
        Self::from(Vector4::from([
            arr[0].coords,
            arr[1].coords,
            arr[2].coords,
            arr[3].coords,
            arr[4].coords,
            arr[5].coords,
            arr[6].coords,
            arr[7].coords,
            arr[8].coords,
            arr[9].coords,
            arr[10].coords,
            arr[11].coords,
            arr[12].coords,
            arr[13].coords,
            arr[14].coords,
            arr[15].coords,
        ]))
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue> From<[UnitQuaternion<T::Element>; 2]>
    for UnitQuaternion<T>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 2]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [UnitQuaternion<T::Element>; 2]) -> Self {
        Self::new_unchecked(Quaternion::from([arr[0].into_inner(), arr[1].into_inner()]))
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue> From<[UnitQuaternion<T::Element>; 4]>
    for UnitQuaternion<T>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 4]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [UnitQuaternion<T::Element>; 4]) -> Self {
        Self::new_unchecked(Quaternion::from([
            arr[0].into_inner(),
            arr[1].into_inner(),
            arr[2].into_inner(),
            arr[3].into_inner(),
        ]))
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue> From<[UnitQuaternion<T::Element>; 8]>
    for UnitQuaternion<T>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 8]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [UnitQuaternion<T::Element>; 8]) -> Self {
        Self::new_unchecked(Quaternion::from([
            arr[0].into_inner(),
            arr[1].into_inner(),
            arr[2].into_inner(),
            arr[3].into_inner(),
            arr[4].into_inner(),
            arr[5].into_inner(),
            arr[6].into_inner(),
            arr[7].into_inner(),
        ]))
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue> From<[UnitQuaternion<T::Element>; 16]>
    for UnitQuaternion<T>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 16]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [UnitQuaternion<T::Element>; 16]) -> Self {
        Self::new_unchecked(Quaternion::from([
            arr[0].into_inner(),
            arr[1].into_inner(),
            arr[2].into_inner(),
            arr[3].into_inner(),
            arr[4].into_inner(),
            arr[5].into_inner(),
            arr[6].into_inner(),
            arr[7].into_inner(),
            arr[8].into_inner(),
            arr[9].into_inner(),
            arr[10].into_inner(),
            arr[11].into_inner(),
            arr[12].into_inner(),
            arr[13].into_inner(),
            arr[14].into_inner(),
            arr[15].into_inner(),
        ]))
    }
}
