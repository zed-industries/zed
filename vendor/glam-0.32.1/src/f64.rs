mod daffine2;
mod daffine3;
mod dmat2;
mod dmat3;
mod dmat4;
mod dquat;
mod dvec2;
mod dvec3;
mod dvec4;
mod float;
pub(crate) mod math;

pub use daffine2::DAffine2;
pub use daffine3::DAffine3;
pub use dmat2::{dmat2, DMat2};
pub use dmat3::{dmat3, DMat3};
pub use dmat4::{dmat4, DMat4};
pub use dquat::{dquat, DQuat};
pub use dvec2::{dvec2, DVec2};
pub use dvec3::{dvec3, DVec3};
pub use dvec4::{dvec4, DVec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    pub use super::*;
    mod const_test_daffine2 {
        const_assert_eq!(
            core::mem::align_of::<super::DVec2>(),
            core::mem::align_of::<super::DAffine2>()
        );
        const_assert_eq!(48, core::mem::size_of::<super::DAffine2>());
    }

    mod const_test_dmat2 {
        const_assert_eq!(
            core::mem::align_of::<super::DVec2>(),
            core::mem::align_of::<super::DMat2>()
        );
        const_assert_eq!(32, core::mem::size_of::<super::DMat2>());
    }

    mod const_test_dmat3 {
        const_assert_eq!(
            core::mem::align_of::<f64>(),
            core::mem::align_of::<super::DMat3>()
        );
        const_assert_eq!(72, core::mem::size_of::<super::DMat3>());
    }

    mod const_test_dmat4 {
        const_assert_eq!(
            core::mem::align_of::<super::DVec4>(),
            core::mem::align_of::<super::DMat4>()
        );
        const_assert_eq!(128, core::mem::size_of::<super::DMat4>());
    }

    mod const_test_dquat {
        #[cfg(not(target_arch = "spirv"))]
        const_assert_eq!(
            core::mem::align_of::<f64>(),
            core::mem::align_of::<super::DQuat>()
        );
        #[cfg(target_arch = "spirv")]
        const_assert_eq!(32, core::mem::align_of::<super::DQuat>());
        const_assert_eq!(32, core::mem::size_of::<super::DQuat>());
    }

    mod const_test_dvec2 {
        #[cfg(not(any(feature = "cuda", target_arch = "spirv")))]
        const_assert_eq!(
            core::mem::align_of::<f64>(),
            core::mem::align_of::<super::DVec2>()
        );
        #[cfg(any(feature = "cuda", target_arch = "spirv"))]
        const_assert_eq!(16, core::mem::align_of::<super::DVec2>());
        const_assert_eq!(16, core::mem::size_of::<super::DVec2>());
    }

    mod const_test_dvec3 {
        #[cfg(not(target_arch = "spirv"))]
        const_assert_eq!(
            core::mem::align_of::<f64>(),
            core::mem::align_of::<super::DVec3>()
        );
        #[cfg(target_arch = "spirv")]
        const_assert_eq!(16, core::mem::align_of::<super::DVec3>());
        const_assert_eq!(24, core::mem::size_of::<super::DVec3>());
    }

    mod const_test_dvec4 {
        #[cfg(not(any(feature = "cuda", target_arch = "spirv")))]
        const_assert_eq!(
            core::mem::align_of::<f64>(),
            core::mem::align_of::<super::DVec4>()
        );
        #[cfg(any(feature = "cuda", target_arch = "spirv"))]
        const_assert_eq!(16, core::mem::align_of::<super::DVec4>());
        const_assert_eq!(32, core::mem::size_of::<super::DVec4>());
    }
}
