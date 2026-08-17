use super::glam::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec3A, Vec4};
use crate::{Translation2, Translation3, Translation4};

macro_rules! impl_translation_conversion(
    ($N: ty, $Vec2: ty, $Vec3: ty, $Vec4: ty) => {
        impl From<$Vec2> for Translation2<$N> {
            #[inline]
            fn from(e: $Vec2) -> Translation2<$N> {
                (*e.as_ref()).into()
            }
        }

        impl From<Translation2<$N>> for $Vec2 {
            #[inline]
            fn from(e: Translation2<$N>) -> $Vec2 {
                e.vector.into()
            }
        }

        impl From<$Vec3> for Translation3<$N> {
            #[inline]
            fn from(e: $Vec3) -> Translation3<$N> {
                (*e.as_ref()).into()
            }
        }

        impl From<Translation3<$N>> for $Vec3 {
            #[inline]
            fn from(e: Translation3<$N>) -> $Vec3 {
                e.vector.into()
            }
        }

        impl From<$Vec4> for Translation4<$N> {
            #[inline]
            fn from(e: $Vec4) -> Translation4<$N> {
                (*e.as_ref()).into()
            }
        }

        impl From<Translation4<$N>> for $Vec4 {
            #[inline]
            fn from(e: Translation4<$N>) -> $Vec4 {
                e.vector.into()
            }
        }
    }
);

impl_translation_conversion!(f32, Vec2, Vec3, Vec4);
impl_translation_conversion!(f64, DVec2, DVec3, DVec4);

impl From<Vec3A> for Translation3<f32> {
    #[inline]
    fn from(e: Vec3A) -> Translation3<f32> {
        (*e.as_ref()).into()
    }
}

impl From<Translation3<f32>> for Vec3A {
    #[inline]
    fn from(e: Translation3<f32>) -> Vec3A {
        e.vector.into()
    }
}
