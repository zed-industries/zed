mod dvec2_impl;
mod dvec3_impl;
mod dvec4_impl;

mod ivec2_impl;
mod ivec3_impl;
mod ivec4_impl;

mod i16vec2_impl;
mod i16vec3_impl;
mod i16vec4_impl;

mod u16vec2_impl;
mod u16vec3_impl;
mod u16vec4_impl;

mod i64vec2_impl;
mod i64vec3_impl;
mod i64vec4_impl;

mod u64vec2_impl;
mod u64vec3_impl;
mod u64vec4_impl;

mod uvec2_impl;
mod uvec3_impl;
mod uvec4_impl;

mod vec2_impl;
mod vec3_impl;

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

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
mod coresimd;

mod vec_traits;
pub use vec_traits::*;
