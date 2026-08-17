#[cfg(feature = "generic-simd")]
pub mod generic;

// This is like generic, but written explicitly
// because generic SIMD requires nightly.
#[cfg(all(
    feature = "runtime-dispatch-simd",
    any(target_arch = "x86", target_arch = "x86_64"),
    not(feature = "generic-simd")
))]
pub mod x86_sse2;

// Modern x86 machines can do lots of fun stuff;
// this is where the *real* optimizations go.
// Runtime feature detection is not available with no_std.
#[cfg(all(feature = "runtime-dispatch-simd", target_arch = "x86_64"))]
pub mod x86_avx2;

/// Modern ARM machines are also quite capable thanks to NEON
#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "wasm32")]
pub mod wasm;