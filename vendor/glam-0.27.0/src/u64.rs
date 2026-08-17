mod u64vec2;
mod u64vec3;
mod u64vec4;

pub use u64vec2::{u64vec2, U64Vec2};
pub use u64vec3::{u64vec3, U64Vec3};
pub use u64vec4::{u64vec4, U64Vec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;
    mod const_test_u64vec2 {
        const_assert_eq!(16, core::mem::size_of::<super::U64Vec2>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u64>(),
            core::mem::align_of::<super::U64Vec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::U64Vec2>());
    }

    mod const_test_u64vec3 {
        const_assert_eq!(24, core::mem::size_of::<super::U64Vec3>());

        const_assert_eq!(
            core::mem::align_of::<u64>(),
            core::mem::align_of::<super::U64Vec3>()
        );
    }

    mod const_test_u64vec4 {
        const_assert_eq!(32, core::mem::size_of::<super::U64Vec4>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u64>(),
            core::mem::align_of::<super::U64Vec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::U64Vec4>());
    }
}
