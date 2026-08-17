use super::glam::{
    BVec2, BVec3, BVec4, DMat2, DMat3, DMat4, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3,
    Mat4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A, Vec4,
};
use crate::storage::RawStorage;
use crate::{
    Matrix, Matrix2, Matrix3, Matrix4, U2, U3, U4, Unit, UnitVector2, UnitVector3, UnitVector4,
    Vector, Vector2, Vector3, Vector4,
};
use std::convert::TryFrom;

macro_rules! impl_vec_conversion(
    ($N: ty, $Vec2: ty, $Vec3: ty, $Vec4: ty) => {
        impl From<$Vec2> for Vector2<$N> {
            #[inline]
            fn from(e: $Vec2) -> Vector2<$N> {
                <[$N;2]>::from(e).into()
            }
        }

        impl<S> From<Vector<$N, U2, S>> for $Vec2
        where
            S: RawStorage<$N, U2>,
        {
            #[inline]
            fn from(e: Vector<$N, U2, S>) -> $Vec2 {
                <$Vec2>::new(e[0], e[1])
            }
        }

        impl From<$Vec3> for Vector3<$N> {
            #[inline]
            fn from(e: $Vec3) -> Vector3<$N> {
                <[$N;3]>::from(e).into()
            }
        }

        impl<S> From<Vector<$N, U3, S>> for $Vec3
        where
            S: RawStorage<$N, U3>,
        {
            #[inline]
            fn from(e: Vector<$N, U3, S>) -> $Vec3 {
                <$Vec3>::new(e[0], e[1], e[2])
            }
        }

        impl From<$Vec4> for Vector4<$N> {
            #[inline]
            fn from(e: $Vec4) -> Vector4<$N> {
                <[$N;4]>::from(e).into()
            }
        }

        impl<S> From<Vector<$N, U4, S>> for $Vec4
        where
            S: RawStorage<$N, U4>,
        {
            #[inline]
            fn from(e: Vector<$N, U4, S>) -> $Vec4 {
                <$Vec4>::new(e[0], e[1], e[2], e[3])
            }
        }
    }
);

impl_vec_conversion!(f32, Vec2, Vec3, Vec4);
impl_vec_conversion!(f64, DVec2, DVec3, DVec4);
impl_vec_conversion!(i32, IVec2, IVec3, IVec4);
impl_vec_conversion!(u32, UVec2, UVec3, UVec4);
impl_vec_conversion!(bool, BVec2, BVec3, BVec4);

const ERR: &'static str = "Normalization failed.";

macro_rules! impl_unit_vec_conversion(
    ($N: ty, $Vec2: ty, $Vec3: ty, $Vec4: ty) => {
        impl TryFrom<$Vec2> for UnitVector2<$N> {
            type Error = &'static str;
            #[inline]
            fn try_from(e: $Vec2) -> Result<Self, Self::Error> {
                Unit::try_new(e.into(), 0.0).ok_or(ERR)
            }
        }

        impl From<UnitVector2<$N>> for $Vec2
        {
            #[inline]
            fn from(e: UnitVector2<$N>) -> $Vec2 {
                e.into_inner().into()
            }
        }

        impl TryFrom<$Vec3> for UnitVector3<$N> {
            type Error = &'static str;
            #[inline]
            fn try_from(e: $Vec3) -> Result<Self, Self::Error> {
                Unit::try_new(e.into(), 0.0).ok_or(ERR)
            }
        }

        impl From<UnitVector3<$N>> for $Vec3
        {
            #[inline]
            fn from(e: UnitVector3<$N>) -> $Vec3 {
                e.into_inner().into()
            }
        }

        impl TryFrom<$Vec4> for UnitVector4<$N> {
            type Error = &'static str;
            #[inline]
            fn try_from(e: $Vec4) -> Result<Self, Self::Error> {
                Unit::try_new(e.into(), 0.0).ok_or(ERR)
            }
        }

        impl From<UnitVector4<$N>> for $Vec4
        {
            #[inline]
            fn from(e: UnitVector4<$N>) -> $Vec4 {
                e.into_inner().into()
            }
        }
    }
);

impl_unit_vec_conversion!(f32, Vec2, Vec3, Vec4);
impl_unit_vec_conversion!(f64, DVec2, DVec3, DVec4);

impl From<Vec3A> for Vector3<f32> {
    #[inline]
    fn from(e: Vec3A) -> Vector3<f32> {
        (*e.as_ref()).into()
    }
}

impl<S> From<Vector<f32, U3, S>> for Vec3A
where
    S: RawStorage<f32, U3>,
{
    #[inline]
    fn from(e: Vector<f32, U3, S>) -> Vec3A {
        Vec3A::new(e[0], e[1], e[2])
    }
}

impl TryFrom<Vec3A> for UnitVector3<f32> {
    type Error = &'static str;
    #[inline]
    fn try_from(e: Vec3A) -> Result<Self, Self::Error> {
        Unit::try_new(e.into(), 0.0).ok_or(ERR)
    }
}

impl From<UnitVector3<f32>> for Vec3A {
    #[inline]
    fn from(e: UnitVector3<f32>) -> Vec3A {
        e.into_inner().into()
    }
}

impl From<Mat2> for Matrix2<f32> {
    #[inline]
    fn from(e: Mat2) -> Matrix2<f32> {
        e.to_cols_array_2d().into()
    }
}

impl<S> From<Matrix<f32, U2, U2, S>> for Mat2
where
    S: RawStorage<f32, U2, U2>,
{
    #[inline]
    fn from(e: Matrix<f32, U2, U2, S>) -> Mat2 {
        Mat2::from_cols(
            Vec2::new(e[(0, 0)], e[(1, 0)]),
            Vec2::new(e[(0, 1)], e[(1, 1)]),
        )
    }
}

impl From<Mat3> for Matrix3<f32> {
    #[inline]
    fn from(e: Mat3) -> Matrix3<f32> {
        e.to_cols_array_2d().into()
    }
}

impl<S> From<Matrix<f32, U3, U3, S>> for Mat3
where
    S: RawStorage<f32, U3, U3>,
{
    #[inline]
    fn from(e: Matrix<f32, U3, U3, S>) -> Mat3 {
        Mat3::from_cols(
            Vec3::new(e[(0, 0)], e[(1, 0)], e[(2, 0)]),
            Vec3::new(e[(0, 1)], e[(1, 1)], e[(2, 1)]),
            Vec3::new(e[(0, 2)], e[(1, 2)], e[(2, 2)]),
        )
    }
}

impl From<Mat4> for Matrix4<f32> {
    #[inline]
    fn from(e: Mat4) -> Matrix4<f32> {
        e.to_cols_array_2d().into()
    }
}

impl<S> From<Matrix<f32, U4, U4, S>> for Mat4
where
    S: RawStorage<f32, U4, U4>,
{
    #[inline]
    fn from(e: Matrix<f32, U4, U4, S>) -> Mat4 {
        Mat4::from_cols(
            Vec4::new(e[(0, 0)], e[(1, 0)], e[(2, 0)], e[(3, 0)]),
            Vec4::new(e[(0, 1)], e[(1, 1)], e[(2, 1)], e[(3, 1)]),
            Vec4::new(e[(0, 2)], e[(1, 2)], e[(2, 2)], e[(3, 2)]),
            Vec4::new(e[(0, 3)], e[(1, 3)], e[(2, 3)], e[(3, 3)]),
        )
    }
}

impl From<DMat2> for Matrix2<f64> {
    #[inline]
    fn from(e: DMat2) -> Matrix2<f64> {
        e.to_cols_array_2d().into()
    }
}

impl<S> From<Matrix<f64, U2, U2, S>> for DMat2
where
    S: RawStorage<f64, U2, U2>,
{
    #[inline]
    fn from(e: Matrix<f64, U2, U2, S>) -> DMat2 {
        DMat2::from_cols(
            DVec2::new(e[(0, 0)], e[(1, 0)]),
            DVec2::new(e[(0, 1)], e[(1, 1)]),
        )
    }
}

impl From<DMat3> for Matrix3<f64> {
    #[inline]
    fn from(e: DMat3) -> Matrix3<f64> {
        e.to_cols_array_2d().into()
    }
}

impl<S> From<Matrix<f64, U3, U3, S>> for DMat3
where
    S: RawStorage<f64, U3, U3>,
{
    #[inline]
    fn from(e: Matrix<f64, U3, U3, S>) -> DMat3 {
        DMat3::from_cols(
            DVec3::new(e[(0, 0)], e[(1, 0)], e[(2, 0)]),
            DVec3::new(e[(0, 1)], e[(1, 1)], e[(2, 1)]),
            DVec3::new(e[(0, 2)], e[(1, 2)], e[(2, 2)]),
        )
    }
}

impl From<DMat4> for Matrix4<f64> {
    #[inline]
    fn from(e: DMat4) -> Matrix4<f64> {
        e.to_cols_array_2d().into()
    }
}

impl<S> From<Matrix<f64, U4, U4, S>> for DMat4
where
    S: RawStorage<f64, U4, U4>,
{
    #[inline]
    fn from(e: Matrix<f64, U4, U4, S>) -> DMat4 {
        DMat4::from_cols(
            DVec4::new(e[(0, 0)], e[(1, 0)], e[(2, 0)], e[(3, 0)]),
            DVec4::new(e[(0, 1)], e[(1, 1)], e[(2, 1)], e[(3, 1)]),
            DVec4::new(e[(0, 2)], e[(1, 2)], e[(2, 2)], e[(3, 2)]),
            DVec4::new(e[(0, 3)], e[(1, 3)], e[(2, 3)], e[(3, 3)]),
        )
    }
}
