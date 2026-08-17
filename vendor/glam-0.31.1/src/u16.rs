mod u16vec2;
mod u16vec3;
mod u16vec4;

pub use u16vec2::{u16vec2, U16Vec2};
pub use u16vec3::{u16vec3, U16Vec3};
pub use u16vec4::{u16vec4, U16Vec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;

    mod const_test_u16vec2 {
        const_assert_eq!(4, core::mem::size_of::<super::U16Vec2>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u16>(),
            core::mem::align_of::<super::U16Vec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(4, core::mem::align_of::<super::U16Vec2>());
    }

    mod const_test_u16vec3 {
        const_assert_eq!(
            core::mem::align_of::<u16>(),
            core::mem::align_of::<super::U16Vec3>()
        );
        const_assert_eq!(6, core::mem::size_of::<super::U16Vec3>());
    }

    mod const_test_u16vec4 {
        const_assert_eq!(8, core::mem::size_of::<super::U16Vec4>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u16>(),
            core::mem::align_of::<super::U16Vec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(8, core::mem::align_of::<super::U16Vec4>());
    }
}
