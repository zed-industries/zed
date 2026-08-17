mod uvec2;
mod uvec3;
mod uvec4;

pub use uvec2::{uvec2, UVec2};
pub use uvec3::{uvec3, UVec3};
pub use uvec4::{uvec4, UVec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;
    mod const_test_uvec2 {
        const_assert_eq!(8, core::mem::size_of::<super::UVec2>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u32>(),
            core::mem::align_of::<super::UVec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(8, core::mem::align_of::<super::UVec2>());
    }

    mod const_test_uvec3 {
        const_assert_eq!(12, core::mem::size_of::<super::UVec3>());

        const_assert_eq!(
            core::mem::align_of::<u32>(),
            core::mem::align_of::<super::UVec3>()
        );
    }

    mod const_test_uvec4 {
        const_assert_eq!(16, core::mem::size_of::<super::UVec4>());

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u32>(),
            core::mem::align_of::<super::UVec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::UVec4>());
    }
}
