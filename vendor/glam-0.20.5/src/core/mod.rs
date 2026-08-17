// the core module provides traits for implementing vector, quaternion and matrix operations,
// storage structs for scalar vector, quaternion and matrix data and implementations of the traits
// for those structs and for supported SIMD types such as SSE2's `__m128`.
//
// The higher level glam library types have an inner type which either uses one of these storage
// structs, or `__m128` and the actual implementation is provided by the core module.
//
// This architecture allows the public API to not require generics or traits, while still
// supporting a number of Rust primitive types and SIMD architectures such as SSE2.
//
pub mod storage;
pub mod traits;

mod scalar;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod sse2;
#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
mod wasm32;
