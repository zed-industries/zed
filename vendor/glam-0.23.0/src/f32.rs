mod affine2;
mod affine3a;
mod mat3;
mod vec2;
mod vec3;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
mod coresimd;

#[cfg(any(
    not(any(
        feature = "core-simd",
        target_feature = "sse2",
        target_feature = "simd128"
    )),
    feature = "scalar-math"
))]
mod scalar;

#[cfg(all(
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
mod sse2;

#[cfg(all(
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
mod wasm32;

#[cfg(any(
    not(any(
        feature = "core-simd",
        target_feature = "sse2",
        target_feature = "simd128"
    )),
    feature = "scalar-math"
))]
use scalar::*;

#[cfg(all(
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
use sse2::*;

#[cfg(all(
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
use wasm32::*;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
use coresimd::*;

pub use affine2::Affine2;
pub use affine3a::Affine3A;
pub use mat2::{mat2, Mat2};
pub use mat3::{mat3, Mat3};
pub use mat3a::{mat3a, Mat3A};
pub use mat4::{mat4, Mat4};
pub use quat::{quat, Quat};
pub use vec2::{vec2, Vec2};
pub use vec3::{vec3, Vec3};
pub use vec3a::{vec3a, Vec3A};
pub use vec4::{vec4, Vec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;

    #[cfg(all(not(feature = "cuda"), feature = "scalar-math"))]
    mod const_test_affine2 {
        const_assert_eq!(
            core::mem::align_of::<super::Vec2>(),
            core::mem::align_of::<super::Affine2>()
        );
        const_assert_eq!(24, core::mem::size_of::<super::Affine2>());
    }

    #[cfg(not(feature = "scalar-math"))]
    mod const_test_affine2 {
        const_assert_eq!(16, core::mem::align_of::<super::Affine2>());
        const_assert_eq!(32, core::mem::size_of::<super::Affine2>());
    }

    mod const_test_mat2 {
        #[cfg(feature = "scalar-math")]
        const_assert_eq!(
            core::mem::align_of::<super::Vec2>(),
            core::mem::align_of::<super::Mat2>()
        );
        #[cfg(not(any(feature = "scalar-math", target_arch = "spirv")))]
        const_assert_eq!(16, core::mem::align_of::<super::Mat2>());
        const_assert_eq!(16, core::mem::size_of::<super::Mat2>());
    }

    mod const_test_mat3 {
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Mat3>()
        );
        const_assert_eq!(36, core::mem::size_of::<super::Mat3>());
    }

    mod const_test_mat3a {
        const_assert_eq!(16, core::mem::align_of::<super::Mat3A>());
        const_assert_eq!(48, core::mem::size_of::<super::Mat3A>());
    }

    mod const_test_mat4 {
        const_assert_eq!(
            core::mem::align_of::<super::Vec4>(),
            core::mem::align_of::<super::Mat4>()
        );
        const_assert_eq!(64, core::mem::size_of::<super::Mat4>());
    }

    mod const_test_quat {
        #[cfg(feature = "scalar-math")]
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Quat>()
        );
        #[cfg(not(any(feature = "scalar-math", target_arch = "spirv")))]
        const_assert_eq!(16, core::mem::align_of::<super::Quat>());
        const_assert_eq!(16, core::mem::size_of::<super::Quat>());
    }

    mod const_test_vec2 {
        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Vec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(8, core::mem::align_of::<super::Vec2>());
        const_assert_eq!(8, core::mem::size_of::<super::Vec2>());
    }

    mod const_test_vec3 {
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Vec3>()
        );
        const_assert_eq!(12, core::mem::size_of::<super::Vec3>());
    }

    mod const_test_vec3a {
        const_assert_eq!(16, core::mem::align_of::<super::Vec3A>());
        const_assert_eq!(16, core::mem::size_of::<super::Vec3A>());
    }

    mod const_test_vec4 {
        #[cfg(all(feature = "scalar-math", not(feature = "cuda")))]
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Vec4>()
        );
        #[cfg(not(feature = "scalar-math"))]
        const_assert_eq!(16, core::mem::align_of::<super::Vec4>());
        const_assert_eq!(16, core::mem::size_of::<super::Vec4>());
    }
}
