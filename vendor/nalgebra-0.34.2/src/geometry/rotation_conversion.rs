use num::Zero;

use crate::ArrayStorage;
use simba::scalar::{RealField, SubsetOf, SupersetOf};
use simba::simd::{PrimitiveSimdValue, SimdValue};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimMin, DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, Matrix2, Matrix3, Matrix4, OMatrix, SMatrix, Scalar};

use crate::geometry::{
    AbstractRotation, Isometry, Rotation, Rotation2, Rotation3, Similarity, SuperTCategoryOf,
    TAffine, Transform, Translation, UnitComplex, UnitDualQuaternion, UnitQuaternion,
};

/*
 * This file provides the following conversions:
 * =============================================
 *
 * Rotation  -> Rotation
 * Rotation3 -> UnitQuaternion
 * Rotation3 -> UnitDualQuaternion
 * Rotation2 -> UnitComplex
 * Rotation  -> Isometry
 * Rotation  -> Similarity
 * Rotation  -> Transform
 * Rotation  -> Matrix (homogeneous)

*/

impl<T1, T2, const D: usize> SubsetOf<Rotation<T2, D>> for Rotation<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> Rotation<T2, D> {
        Rotation::from_matrix_unchecked(self.matrix().to_superset())
    }

    #[inline]
    fn is_in_subset(rot: &Rotation<T2, D>) -> bool {
        crate::is_convertible::<_, SMatrix<T1, D, D>>(rot.matrix())
    }

    #[inline]
    fn from_superset_unchecked(rot: &Rotation<T2, D>) -> Self {
        Rotation::from_matrix_unchecked(rot.matrix().to_subset_unchecked())
    }
}

impl<T1, T2> SubsetOf<UnitQuaternion<T2>> for Rotation3<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> UnitQuaternion<T2> {
        let q = UnitQuaternion::<T1>::from_rotation_matrix(self);
        q.to_superset()
    }

    #[inline]
    fn is_in_subset(q: &UnitQuaternion<T2>) -> bool {
        crate::is_convertible::<_, UnitQuaternion<T1>>(q)
    }

    #[inline]
    fn from_superset_unchecked(q: &UnitQuaternion<T2>) -> Self {
        let q: UnitQuaternion<T1> = crate::convert_ref_unchecked(q);
        q.to_rotation_matrix()
    }
}

impl<T1, T2> SubsetOf<UnitDualQuaternion<T2>> for Rotation3<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> UnitDualQuaternion<T2> {
        let q = UnitQuaternion::<T1>::from_rotation_matrix(self);
        let dq = UnitDualQuaternion::from_rotation(q);
        dq.to_superset()
    }

    #[inline]
    fn is_in_subset(dq: &UnitDualQuaternion<T2>) -> bool {
        crate::is_convertible::<_, UnitQuaternion<T1>>(&dq.rotation())
            && dq.translation().vector.is_zero()
    }

    #[inline]
    fn from_superset_unchecked(dq: &UnitDualQuaternion<T2>) -> Self {
        let dq: UnitDualQuaternion<T1> = crate::convert_ref_unchecked(dq);
        dq.rotation().to_rotation_matrix()
    }
}

impl<T1, T2> SubsetOf<UnitComplex<T2>> for Rotation2<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> UnitComplex<T2> {
        let q = UnitComplex::<T1>::from_rotation_matrix(self);
        q.to_superset()
    }

    #[inline]
    fn is_in_subset(q: &UnitComplex<T2>) -> bool {
        crate::is_convertible::<_, UnitComplex<T1>>(q)
    }

    #[inline]
    fn from_superset_unchecked(q: &UnitComplex<T2>) -> Self {
        let q: UnitComplex<T1> = crate::convert_ref_unchecked(q);
        q.to_rotation_matrix()
    }
}

impl<T1, T2, R, const D: usize> SubsetOf<Isometry<T2, R, D>> for Rotation<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T2, D> + SupersetOf<Self>,
{
    #[inline]
    fn to_superset(&self) -> Isometry<T2, R, D> {
        Isometry::from_parts(Translation::identity(), crate::convert_ref(self))
    }

    #[inline]
    fn is_in_subset(iso: &Isometry<T2, R, D>) -> bool {
        iso.translation.vector.is_zero()
    }

    #[inline]
    fn from_superset_unchecked(iso: &Isometry<T2, R, D>) -> Self {
        crate::convert_ref_unchecked(&iso.rotation)
    }
}

impl<T1, T2, R, const D: usize> SubsetOf<Similarity<T2, R, D>> for Rotation<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T2, D> + SupersetOf<Self>,
{
    #[inline]
    fn to_superset(&self) -> Similarity<T2, R, D> {
        Similarity::from_parts(Translation::identity(), crate::convert_ref(self), T2::one())
    }

    #[inline]
    fn is_in_subset(sim: &Similarity<T2, R, D>) -> bool {
        sim.isometry.translation.vector.is_zero() && sim.scaling() == T2::one()
    }

    #[inline]
    fn from_superset_unchecked(sim: &Similarity<T2, R, D>) -> Self {
        crate::convert_ref_unchecked(&sim.isometry.rotation)
    }
}

impl<T1, T2, C, const D: usize> SubsetOf<Transform<T2, C, D>> for Rotation<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    C: SuperTCategoryOf<TAffine>,
    Const<D>: DimNameAdd<U1> + DimMin<Const<D>, Output = Const<D>>, // needed by .is_special_orthogonal()
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    // + Allocator<D>,
    //     + Allocator<D, D>
{
    // needed by .is_special_orthogonal()
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

impl<T1, T2, const D: usize>
    SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>> for Rotation<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    Const<D>: DimNameAdd<U1> + DimMin<Const<D>, Output = Const<D>>, // needed by .is_special_orthogonal()
    DefaultAllocator: Allocator<Const<D>, Const<D>, Buffer<T1> = ArrayStorage<T1, D, D>>
        + Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>, // + Allocator<D>,
                                                                    // + Allocator<D, D>
{
    // needed by .is_special_orthogonal()
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
        let r = m.fixed_view::<D, D>(0, 0);
        Self::from_matrix_unchecked(crate::convert_unchecked(r.into_owned()))
    }
}

impl<T: RealField> From<Rotation2<T>> for Matrix3<T> {
    #[inline]
    fn from(q: Rotation2<T>) -> Self {
        q.to_homogeneous()
    }
}

impl<T: RealField> From<Rotation2<T>> for Matrix2<T> {
    #[inline]
    fn from(q: Rotation2<T>) -> Self {
        q.into_inner()
    }
}

impl<T: RealField> From<Rotation3<T>> for Matrix4<T> {
    #[inline]
    fn from(q: Rotation3<T>) -> Self {
        q.to_homogeneous()
    }
}

impl<T: RealField> From<Rotation3<T>> for Matrix3<T> {
    #[inline]
    fn from(q: Rotation3<T>) -> Self {
        q.into_inner()
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Rotation<T::Element, D>; 2]>
    for Rotation<T, D>
where
    T: From<[<T as SimdValue>::Element; 2]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Rotation<T::Element, D>; 2]) -> Self {
        Self::from_matrix_unchecked(OMatrix::from([arr[0].into_inner(), arr[1].into_inner()]))
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Rotation<T::Element, D>; 4]>
    for Rotation<T, D>
where
    T: From<[<T as SimdValue>::Element; 4]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Rotation<T::Element, D>; 4]) -> Self {
        Self::from_matrix_unchecked(OMatrix::from([
            arr[0].into_inner(),
            arr[1].into_inner(),
            arr[2].into_inner(),
            arr[3].into_inner(),
        ]))
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Rotation<T::Element, D>; 8]>
    for Rotation<T, D>
where
    T: From<[<T as SimdValue>::Element; 8]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Rotation<T::Element, D>; 8]) -> Self {
        Self::from_matrix_unchecked(OMatrix::from([
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

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Rotation<T::Element, D>; 16]>
    for Rotation<T, D>
where
    T: From<[<T as SimdValue>::Element; 16]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [Rotation<T::Element, D>; 16]) -> Self {
        Self::from_matrix_unchecked(OMatrix::from([
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
