mod i8vec2;
mod i8vec3;
mod i8vec4;

pub use i8vec2::{i8vec2, I8Vec2};
pub use i8vec3::{i8vec3, I8Vec3};
pub use i8vec4::{i8vec4, I8Vec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;

    mod const_test_i8vec2 {
        const_assert_eq!(2, core::mem::size_of::<super::I8Vec2>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i8>(),
            core::mem::align_of::<super::I8Vec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(2, core::mem::align_of::<super::I8Vec2>());
    }

    mod const_test_i8vec3 {
        const_assert_eq!(
            core::mem::align_of::<i8>(),
            core::mem::align_of::<super::I8Vec3>()
        );
        const_assert_eq!(3, core::mem::size_of::<super::I8Vec3>());
    }

    mod const_test_i8vec4 {
        const_assert_eq!(4, core::mem::size_of::<super::I8Vec4>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i8>(),
            core::mem::align_of::<super::I8Vec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(4, core::mem::align_of::<super::I8Vec4>());
    }
}
