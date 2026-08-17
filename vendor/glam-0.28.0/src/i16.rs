mod i16vec2;
mod i16vec3;
mod i16vec4;

pub use i16vec2::{i16vec2, I16Vec2};
pub use i16vec3::{i16vec3, I16Vec3};
pub use i16vec4::{i16vec4, I16Vec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;

    mod const_test_i16vec2 {
        const_assert_eq!(4, core::mem::size_of::<super::I16Vec2>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i16>(),
            core::mem::align_of::<super::I16Vec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(4, core::mem::align_of::<super::I16Vec2>());
    }

    mod const_test_i16vec3 {
        const_assert_eq!(
            core::mem::align_of::<i16>(),
            core::mem::align_of::<super::I16Vec3>()
        );
        const_assert_eq!(6, core::mem::size_of::<super::I16Vec3>());
    }

    mod const_test_i16vec4 {
        const_assert_eq!(8, core::mem::size_of::<super::I16Vec4>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i16>(),
            core::mem::align_of::<super::I16Vec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(8, core::mem::align_of::<super::I16Vec4>());
    }
}
