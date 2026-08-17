use num::Zero;
use num_complex::Complex;

use simba::scalar::{RealField, SubsetOf, SupersetOf};
use simba::simd::{PrimitiveSimdValue, SimdRealField};

use crate::base::{Matrix2, Matrix3, Scalar};
use crate::geometry::{
    AbstractRotation, Isometry, Rotation2, Similarity, SuperTCategoryOf, TAffine, Transform,
    Translation, UnitComplex,
};

/*
 * This file provides the following conversions:
 * =============================================
 *
 * UnitComplex -> UnitComplex
 * UnitComplex -> Rotation<U1>
 * UnitComplex -> Isometry<2>
 * UnitComplex -> Similarity<2>
 * UnitComplex -> Transform<2>
 * UnitComplex -> Matrix<U3> (homogeneous)
 *
 * NOTE:
 * UnitComplex -> Complex is already provided by: Unit<T> -> T
 */

impl<T1, T2> SubsetOf<UnitComplex<T2>> for UnitComplex<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> UnitComplex<T2> {
        UnitComplex::new_unchecked(self.as_ref().to_superset())
    }

    #[inline]
    fn is_in_subset(uq: &UnitComplex<T2>) -> bool {
        crate::is_convertible::<_, Complex<T1>>(uq.as_ref())
    }

    #[inline]
    fn from_superset_unchecked(uq: &UnitComplex<T2>) -> Self {
        Self::new_unchecked(crate::convert_ref_unchecked(uq.as_ref()))
    }
}

impl<T1, T2> SubsetOf<Rotation2<T2>> for UnitComplex<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> Rotation2<T2> {
        let q: UnitComplex<T2> = self.to_superset();
        q.to_rotation_matrix().to_superset()
    }

    #[inline]
    fn is_in_subset(rot: &Rotation2<T2>) -> bool {
        crate::is_convertible::<_, Rotation2<T1>>(rot)
    }

    #[inline]
    fn from_superset_unchecked(rot: &Rotation2<T2>) -> Self {
        let q = UnitComplex::<T2>::from_rotation_matrix(rot);
        crate::convert_unchecked(q)
    }
}

impl<T1, T2, R> SubsetOf<Isometry<T2, R, 2>> for UnitComplex<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T2, 2> + SupersetOf<Self>,
{
    #[inline]
    fn to_superset(&self) -> Isometry<T2, R, 2> {
        Isometry::from_parts(Translation::identity(), crate::convert_ref(self))
    }

    #[inline]
    fn is_in_subset(iso: &Isometry<T2, R, 2>) -> bool {
        iso.translation.vector.is_zero()
    }

    #[inline]
    fn from_superset_unchecked(iso: &Isometry<T2, R, 2>) -> Self {
        crate::convert_ref_unchecked(&iso.rotation)
    }
}

impl<T1, T2, R> SubsetOf<Similarity<T2, R, 2>> for UnitComplex<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T2, 2> + SupersetOf<Self>,
{
    #[inline]
    fn to_superset(&self) -> Similarity<T2, R, 2> {
        Similarity::from_isometry(crate::convert_ref(self), T2::one())
    }

    #[inline]
    fn is_in_subset(sim: &Similarity<T2, R, 2>) -> bool {
        sim.isometry.translation.vector.is_zero() && sim.scaling() == T2::one()
    }

    #[inline]
    fn from_superset_unchecked(sim: &Similarity<T2, R, 2>) -> Self {
        crate::convert_ref_unchecked(&sim.isometry)
    }
}

impl<T1, T2, C> SubsetOf<Transform<T2, C, 2>> for UnitComplex<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    C: SuperTCategoryOf<TAffine>,
{
    #[inline]
    fn to_superset(&self) -> Transform<T2, C, 2> {
        Transform::from_matrix_unchecked(self.clone().to_homogeneous().to_superset())
    }

    #[inline]
    fn is_in_subset(t: &Transform<T2, C, 2>) -> bool {
        <Self as SubsetOf<_>>::is_in_subset(t.matrix())
    }

    #[inline]
    fn from_superset_unchecked(t: &Transform<T2, C, 2>) -> Self {
        Self::from_superset_unchecked(t.matrix())
    }
}

impl<T1: RealField, T2: RealField + SupersetOf<T1>> SubsetOf<Matrix3<T2>> for UnitComplex<T1> {
    #[inline]
    fn to_superset(&self) -> Matrix3<T2> {
        self.clone().to_homogeneous().to_superset()
    }

    #[inline]
    fn is_in_subset(m: &Matrix3<T2>) -> bool {
        crate::is_convertible::<_, Rotation2<T1>>(m)
    }

    #[inline]
    fn from_superset_unchecked(m: &Matrix3<T2>) -> Self {
        let rot: Rotation2<T1> = crate::convert_ref_unchecked(m);
        Self::from_rotation_matrix(&rot)
    }
}

impl<T: SimdRealField> From<UnitComplex<T>> for Rotation2<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(q: UnitComplex<T>) -> Self {
        q.to_rotation_matrix()
    }
}

impl<T: SimdRealField> From<Rotation2<T>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(q: Rotation2<T>) -> Self {
        Self::from_rotation_matrix(&q)
    }
}

impl<T: SimdRealField> From<UnitComplex<T>> for Matrix3<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(q: UnitComplex<T>) -> Matrix3<T> {
        q.to_homogeneous()
    }
}

impl<T: SimdRealField> From<UnitComplex<T>> for Matrix2<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn from(q: UnitComplex<T>) -> Self {
        q.to_rotation_matrix().into_inner()
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue> From<[UnitComplex<T::Element>; 2]> for UnitComplex<T>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 2]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [UnitComplex<T::Element>; 2]) -> Self {
        Self::new_unchecked(Complex {
            re: T::from([arr[0].re, arr[1].re]),
            im: T::from([arr[0].im, arr[1].im]),
        })
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue> From<[UnitComplex<T::Element>; 4]> for UnitComplex<T>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 4]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [UnitComplex<T::Element>; 4]) -> Self {
        Self::new_unchecked(Complex {
            re: T::from([arr[0].re, arr[1].re, arr[2].re, arr[3].re]),
            im: T::from([arr[0].im, arr[1].im, arr[2].im, arr[3].im]),
        })
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue> From<[UnitComplex<T::Element>; 8]> for UnitComplex<T>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 8]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [UnitComplex<T::Element>; 8]) -> Self {
        Self::new_unchecked(Complex {
            re: T::from([
                arr[0].re, arr[1].re, arr[2].re, arr[3].re, arr[4].re, arr[5].re, arr[6].re,
                arr[7].re,
            ]),
            im: T::from([
                arr[0].im, arr[1].im, arr[2].im, arr[3].im, arr[4].im, arr[5].im, arr[6].im,
                arr[7].im,
            ]),
        })
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue> From<[UnitComplex<T::Element>; 16]> for UnitComplex<T>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 16]>,
    T::Element: Scalar + Copy,
{
    #[inline]
    fn from(arr: [UnitComplex<T::Element>; 16]) -> Self {
        Self::new_unchecked(Complex {
            re: T::from([
                arr[0].re, arr[1].re, arr[2].re, arr[3].re, arr[4].re, arr[5].re, arr[6].re,
                arr[7].re, arr[8].re, arr[9].re, arr[10].re, arr[11].re, arr[12].re, arr[13].re,
                arr[14].re, arr[15].re,
            ]),
            im: T::from([
                arr[0].im, arr[1].im, arr[2].im, arr[3].im, arr[4].im, arr[5].im, arr[6].im,
                arr[7].im, arr[8].im, arr[9].im, arr[10].im, arr[11].im, arr[12].im, arr[13].im,
                arr[14].im, arr[15].im,
            ]),
        })
    }
}
