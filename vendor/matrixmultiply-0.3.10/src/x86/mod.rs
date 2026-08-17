
#[cfg(target_arch="x86")]
use core::arch::x86::*;
#[cfg(target_arch="x86_64")]
use core::arch::x86_64::*;

#[macro_use]
mod macros;

pub(crate) struct FusedMulAdd;
pub(crate) struct AvxMulAdd;

pub(crate) trait SMultiplyAdd {
    const IS_FUSED: bool;
    unsafe fn multiply_add(a: __m256, b: __m256, c: __m256) -> __m256;
}

impl SMultiplyAdd for AvxMulAdd {
    const IS_FUSED: bool = false;
    #[inline(always)]
    unsafe fn multiply_add(a: __m256, b: __m256, c: __m256) -> __m256 {
        _mm256_add_ps(_mm256_mul_ps(a, b), c)
    }
}

impl SMultiplyAdd for FusedMulAdd {
    const IS_FUSED: bool = true;
    #[inline(always)]
    unsafe fn multiply_add(a: __m256, b: __m256, c: __m256) -> __m256 {
        _mm256_fmadd_ps(a, b, c)
    }
}

pub(crate) trait DMultiplyAdd {
    const IS_FUSED: bool;
    unsafe fn multiply_add(a: __m256d, b: __m256d, c: __m256d) -> __m256d;
}

impl DMultiplyAdd for AvxMulAdd {
    const IS_FUSED: bool = false;
    #[inline(always)]
    unsafe fn multiply_add(a: __m256d, b: __m256d, c: __m256d) -> __m256d {
        _mm256_add_pd(_mm256_mul_pd(a, b), c)
    }
}

impl DMultiplyAdd for FusedMulAdd {
    const IS_FUSED: bool = true;
    #[inline(always)]
    unsafe fn multiply_add(a: __m256d, b: __m256d, c: __m256d) -> __m256d {
        _mm256_fmadd_pd(a, b, c)
    }
}

