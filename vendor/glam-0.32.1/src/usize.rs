mod usizevec2;
mod usizevec3;
mod usizevec4;

pub use usizevec2::{usizevec2, USizeVec2};
pub use usizevec3::{usizevec3, USizeVec3};
pub use usizevec4::{usizevec4, USizeVec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;
    mod const_test_usizevec2 {
        const_assert_eq!(
            core::mem::size_of::<usize>() * 2,
            core::mem::size_of::<super::USizeVec2>()
        );

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<usize>(),
            core::mem::align_of::<super::USizeVec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::USizeVec2>());
    }

    mod const_test_usizevec3 {
        const_assert_eq!(
            core::mem::size_of::<usize>() * 3,
            core::mem::size_of::<super::USizeVec3>()
        );

        const_assert_eq!(
            core::mem::align_of::<usize>(),
            core::mem::align_of::<super::USizeVec3>()
        );
    }

    mod const_test_usizevec4 {
        const_assert_eq!(
            core::mem::size_of::<usize>() * 4,
            core::mem::size_of::<super::USizeVec4>()
        );

        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<usize>(),
            core::mem::align_of::<super::USizeVec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::USizeVec4>());
    }
}
