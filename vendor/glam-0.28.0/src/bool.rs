mod bvec2;
mod bvec3;
mod bvec4;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
mod coresimd;

#[cfg(all(
    target_arch = "aarch64",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
mod neon;

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
        target_arch = "aarch64",
        target_feature = "sse2",
        target_feature = "simd128"
    )),
    feature = "scalar-math"
))]
mod scalar;

pub use bvec2::BVec2;
pub use bvec3::BVec3;
pub use bvec4::BVec4;

#[cfg(all(
    target_arch = "aarch64",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
pub use neon::bvec3a::BVec3A;
#[cfg(all(
    target_arch = "aarch64",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
pub use neon::bvec4a::BVec4A;

#[cfg(all(
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
pub use sse2::bvec3a::BVec3A;
#[cfg(all(
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
pub use sse2::bvec4a::BVec4A;

#[cfg(all(
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
pub use wasm32::bvec3a::BVec3A;
#[cfg(all(
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
pub use wasm32::bvec4a::BVec4A;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
pub use coresimd::bvec3a::BVec3A;
#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
pub use coresimd::bvec4a::BVec4A;

#[cfg(any(
    not(any(
        feature = "core-simd",
        target_arch = "aarch64",
        target_feature = "sse2",
        target_feature = "simd128"
    )),
    feature = "scalar-math"
))]
pub use scalar::bvec3a::BVec3A;

#[cfg(not(any(
    feature = "scalar-math",
    feature = "core-simd",
    target_arch = "aarch64",
    target_feature = "sse2",
    target_feature = "simd128"
),))]
pub use scalar::bvec4a::BVec4A;

mod const_test_bvec2 {
    const_assert_eq!(1, core::mem::align_of::<super::BVec2>());
    const_assert_eq!(2, core::mem::size_of::<super::BVec2>());
}

mod const_test_bvec3 {
    const_assert_eq!(1, core::mem::align_of::<super::BVec3>());
    const_assert_eq!(3, core::mem::size_of::<super::BVec3>());
}

mod const_test_bvec4 {
    const_assert_eq!(1, core::mem::align_of::<super::BVec4>());
    const_assert_eq!(4, core::mem::size_of::<super::BVec4>());
}

#[cfg(not(feature = "scalar-math"))]
mod const_test_bvec3a {
    const_assert_eq!(16, core::mem::align_of::<super::BVec3A>());
    const_assert_eq!(16, core::mem::size_of::<super::BVec3A>());
}

#[cfg(not(feature = "scalar-math"))]
mod const_test_bvec4a {
    const_assert_eq!(16, core::mem::align_of::<super::BVec4A>());
    const_assert_eq!(16, core::mem::size_of::<super::BVec4A>());
}
