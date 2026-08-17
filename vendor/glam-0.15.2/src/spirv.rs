#[cfg(target_arch = "spirv")]
macro_rules! unsupported_features {
    ($($feature:literal),+ $(,)?) => {
        $(
            #[cfg(feature = $feature)]
            compile_error!(
                concat!(
                    "`",
                    $feature,
                    "`",
                    " feature is not supported when building for SPIR-V.",
                )
            );
        )+
    }
}

#[cfg(target_arch = "spirv")]
unsupported_features! {
    "bytemuck",
    "debug-glam-assert",
    "glam-assert",
    "rand",
    "serde",
    "std",
}

use spirv_std::vector::Vector;

unsafe impl Vector<bool, 2> for crate::BVec2 {}
unsafe impl Vector<bool, 3> for crate::BVec3 {}
unsafe impl Vector<bool, 4> for crate::BVec4 {}

unsafe impl Vector<f32, 2> for crate::Vec2 {}
unsafe impl Vector<f32, 3> for crate::Vec3 {}
unsafe impl Vector<f32, 3> for crate::Vec3A {}
unsafe impl Vector<f32, 4> for crate::Vec4 {}

unsafe impl Vector<f64, 2> for crate::DVec2 {}
unsafe impl Vector<f64, 3> for crate::DVec3 {}
unsafe impl Vector<f64, 4> for crate::DVec4 {}

unsafe impl Vector<u32, 2> for crate::UVec2 {}
unsafe impl Vector<u32, 3> for crate::UVec3 {}
unsafe impl Vector<u32, 4> for crate::UVec4 {}

unsafe impl Vector<i32, 2> for crate::IVec2 {}
unsafe impl Vector<i32, 3> for crate::IVec3 {}
unsafe impl Vector<i32, 4> for crate::IVec4 {}
