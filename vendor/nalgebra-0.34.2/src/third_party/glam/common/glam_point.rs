use super::glam::{
    BVec2, BVec3, BVec4, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3,
    Vec3A, Vec4,
};
use crate::{Point2, Point3, Point4};

macro_rules! impl_point_conversion(
    ($N: ty, $Vec2: ty, $Vec3: ty, $Vec4: ty) => {
        impl From<$Vec2> for Point2<$N> {
            #[inline]
            fn from(e: $Vec2) -> Point2<$N> {
                <[$N;2]>::from(e).into()
            }
        }

        impl From<Point2<$N>> for $Vec2 {
            #[inline]
            fn from(e: Point2<$N>) -> $Vec2 {
                <$Vec2>::new(e[0], e[1])
            }
        }

        impl From<$Vec3> for Point3<$N> {
            #[inline]
            fn from(e: $Vec3) -> Point3<$N> {
                <[$N;3]>::from(e).into()
            }
        }

        impl From<Point3<$N>> for $Vec3 {
            #[inline]
            fn from(e: Point3<$N>) -> $Vec3 {
                <$Vec3>::new(e[0], e[1], e[2])
            }
        }

        impl From<$Vec4> for Point4<$N> {
            #[inline]
            fn from(e: $Vec4) -> Point4<$N> {
                <[$N;4]>::from(e).into()
            }
        }

        impl From<Point4<$N>> for $Vec4 {
            #[inline]
            fn from(e: Point4<$N>) -> $Vec4 {
                <$Vec4>::new(e[0], e[1], e[2], e[3])
            }
        }
    }
);

impl_point_conversion!(f32, Vec2, Vec3, Vec4);
impl_point_conversion!(f64, DVec2, DVec3, DVec4);
impl_point_conversion!(i32, IVec2, IVec3, IVec4);
impl_point_conversion!(u32, UVec2, UVec3, UVec4);
impl_point_conversion!(bool, BVec2, BVec3, BVec4);

impl From<Vec3A> for Point3<f32> {
    #[inline]
    fn from(e: Vec3A) -> Point3<f32> {
        (*e.as_ref()).into()
    }
}

impl From<Point3<f32>> for Vec3A {
    #[inline]
    fn from(e: Point3<f32>) -> Vec3A {
        Vec3A::new(e[0], e[1], e[2])
    }
}
