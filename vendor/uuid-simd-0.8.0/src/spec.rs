use vsimd::vector::{V128, V256};
use vsimd::{SIMD128, SIMD256};

#[cfg(any(
    any(target_arch = "x86", target_arch = "x86_64"),
    any(all(feature = "unstable", target_arch = "arm"), target_arch = "aarch64"),
    target_arch = "wasm32"
))]
vsimd::item_group! {
    use vsimd::matches_isa;
    use core::mem::transmute as t;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use vsimd::isa::{AVX2, SSE2, SSE41};

#[cfg(any(all(feature = "unstable", target_arch = "arm"), target_arch = "aarch64"))]
use vsimd::isa::NEON;

#[cfg(target_arch = "wasm32")]
use vsimd::isa::WASM128;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(all(feature = "unstable", target_arch = "arm"))]
use core::arch::arm::*;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::*;

#[inline(always)]
pub fn i16x16_set_lane7<S: SIMD256>(s: S, a: V256, x: i16) -> V256 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if matches_isa!(S, AVX2) {
        return unsafe { t(_mm256_insert_epi16::<7>(t(a), x)) };
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if matches_isa!(S, SSE2) {
        let a = a.to_v128x2();
        let a0 = unsafe { t(_mm_insert_epi16::<7>(t(a.0), x as i32)) };
        return V256::from_v128x2((a0, a.1));
    }
    #[cfg(any(all(feature = "unstable", target_arch = "arm"), target_arch = "aarch64"))]
    if matches_isa!(S, NEON) {
        return unsafe {
            let a: uint8x16x2_t = t(a);
            let a0 = vsetq_lane_s16::<7>(x, t(a.0));
            t(uint8x16x2_t(t(a0), a.1))
        };
    }
    #[cfg(target_arch = "wasm32")]
    if matches_isa!(S, WASM128) {
        let a = a.to_v128x2();
        let a0 = unsafe { t(i16x8_replace_lane::<7>(t(a.0), x)) };
        return V256::from_v128x2((a0, a.1));
    }
    {
        let _ = (s, a, x);
        unreachable!()
    }
}

#[inline(always)]
pub fn i32x8_set_lane7<S: SIMD256>(s: S, a: V256, x: i32) -> V256 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if matches_isa!(S, AVX2) {
        return unsafe { t(_mm256_insert_epi32::<7>(t(a), x)) };
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if matches_isa!(S, SSE41) {
        let a = a.to_v128x2();
        let a1 = unsafe { t(_mm_insert_epi32::<3>(t(a.1), x)) };
        return V256::from_v128x2((a.0, a1));
    }
    #[cfg(any(all(feature = "unstable", target_arch = "arm"), target_arch = "aarch64"))]
    if matches_isa!(S, NEON) {
        return unsafe {
            let a: uint8x16x2_t = t(a);
            let a1 = vsetq_lane_s32::<3>(x, t(a.1));
            t(uint8x16x2_t(a.0, t(a1)))
        };
    }
    #[cfg(target_arch = "wasm32")]
    if matches_isa!(S, WASM128) {
        let a = a.to_v128x2();
        let a1 = unsafe { t(i32x4_replace_lane::<3>(t(a.1), x)) };
        return V256::from_v128x2((a.0, a1));
    }
    {
        let _ = (s, a, x);
        unreachable!()
    }
}

#[inline(always)]
pub fn i32x4_get_lane3<S: SIMD128>(s: S, a: V128) -> i32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if matches_isa!(S, SSE41) {
        return unsafe { _mm_extract_epi32::<3>(t(a)) };
    }
    #[cfg(any(all(feature = "unstable", target_arch = "arm"), target_arch = "aarch64"))]
    if matches_isa!(S, NEON) {
        return unsafe { vgetq_lane_s32::<3>(t(a)) };
    }
    #[cfg(target_arch = "wasm32")]
    if matches_isa!(S, WASM128) {
        return unsafe { i32x4_extract_lane::<3>(t(a)) };
    }
    {
        let _ = (s, a);
        unreachable!()
    }
}

#[inline(always)]
pub fn i16x8_get_lane7<S: SIMD128>(s: S, a: V128) -> i16 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if matches_isa!(S, SSE2) {
        return unsafe { _mm_extract_epi16::<7>(t(a)) as i16 };
    }
    #[cfg(any(all(feature = "unstable", target_arch = "arm"), target_arch = "aarch64"))]
    if matches_isa!(S, NEON) {
        return unsafe { vgetq_lane_s16::<7>(t(a)) };
    }
    #[cfg(target_arch = "wasm32")]
    if matches_isa!(S, WASM128) {
        return unsafe { i16x8_extract_lane::<7>(t(a)) };
    }
    {
        let _ = (s, a);
        unreachable!()
    }
}
