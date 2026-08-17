mod isizevec2;
mod isizevec3;
mod isizevec4;

pub use isizevec2::{isizevec2, ISizeVec2};
pub use isizevec3::{isizevec3, ISizeVec3};
pub use isizevec4::{isizevec4, ISizeVec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;
    mod const_test_isizevec2 {
        const_assert_eq!(
            core::mem::size_of::<isize>() * 2,
            core::mem::size_of::<super::ISizeVec2>()
        );

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<isize>(),
            core::mem::align_of::<super::ISizeVec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::ISizeVec2>());
    }

    mod const_test_isizevec3 {
        const_assert_eq!(
            core::mem::size_of::<isize>() * 3,
            core::mem::size_of::<super::ISizeVec3>()
        );

        const_assert_eq!(
            core::mem::align_of::<isize>(),
            core::mem::align_of::<super::ISizeVec3>()
        );
    }

    mod const_test_isizevec4 {
        const_assert_eq!(
            core::mem::size_of::<isize>() * 4,
            core::mem::size_of::<super::ISizeVec4>()
        );

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<isize>(),
            core::mem::align_of::<super::ISizeVec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::ISizeVec4>());
    }
}
