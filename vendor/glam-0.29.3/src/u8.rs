mod u8vec2;
mod u8vec3;
mod u8vec4;

pub use u8vec2::{u8vec2, U8Vec2};
pub use u8vec3::{u8vec3, U8Vec3};
pub use u8vec4::{u8vec4, U8Vec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;

    mod const_test_u8vec2 {
        const_assert_eq!(2, core::mem::size_of::<super::U8Vec2>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u8>(),
            core::mem::align_of::<super::U8Vec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(2, core::mem::align_of::<super::U8Vec2>());
    }

    mod const_test_u8vec3 {
        const_assert_eq!(
            core::mem::align_of::<u8>(),
            core::mem::align_of::<super::U8Vec3>()
        );
        const_assert_eq!(3, core::mem::size_of::<super::U8Vec3>());
    }

    mod const_test_u8vec4 {
        const_assert_eq!(4, core::mem::size_of::<super::U8Vec4>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u8>(),
            core::mem::align_of::<super::U8Vec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(4, core::mem::align_of::<super::U8Vec4>());
    }
}
