mod i64vec2;
mod i64vec3;
mod i64vec4;

pub use i64vec2::{i64vec2, I64Vec2};
pub use i64vec3::{i64vec3, I64Vec3};
pub use i64vec4::{i64vec4, I64Vec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;

    mod const_test_i64vec2 {
        const_assert_eq!(16, core::mem::size_of::<super::I64Vec2>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i64>(),
            core::mem::align_of::<super::I64Vec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::I64Vec2>());
    }

    mod const_test_i64vec3 {
        const_assert_eq!(24, core::mem::size_of::<super::I64Vec3>());

        const_assert_eq!(
            core::mem::align_of::<i64>(),
            core::mem::align_of::<super::I64Vec3>()
        );
    }

    mod const_test_i64vec4 {
        const_assert_eq!(32, core::mem::size_of::<super::I64Vec4>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i64>(),
            core::mem::align_of::<super::I64Vec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::I64Vec4>());
    }
}
