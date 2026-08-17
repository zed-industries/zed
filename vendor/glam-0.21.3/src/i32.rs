mod ivec2;
mod ivec3;
mod ivec4;

pub use ivec2::{ivec2, IVec2};
pub use ivec3::{ivec3, IVec3};
pub use ivec4::{ivec4, IVec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;

    mod const_test_ivec2 {
        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i32>(),
            core::mem::align_of::<super::IVec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(8, core::mem::align_of::<super::IVec2>());
        const_assert_eq!(8, core::mem::size_of::<super::IVec2>());
    }

    mod const_test_ivec3 {
        const_assert_eq!(
            core::mem::align_of::<i32>(),
            core::mem::align_of::<super::IVec3>()
        );
        const_assert_eq!(12, core::mem::size_of::<super::IVec3>());
    }

    mod const_test_ivec4 {
        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i32>(),
            core::mem::align_of::<super::IVec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::IVec4>());
        const_assert_eq!(16, core::mem::size_of::<super::IVec4>());
    }
}
