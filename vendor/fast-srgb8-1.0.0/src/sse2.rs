use super::TO_SRGB8_TABLE;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::mem::transmute;

const MAXV: __m128 = unsafe { transmute([0x3f7fffffu32; 4]) };
const MINV: __m128 = unsafe { transmute([0x39000000u32; 4]) };
const MANT_MASK: __m128i = unsafe { transmute([0xffu32; 4]) };
const TOP_SCALE: __m128i = unsafe { transmute([0x02000000u32; 4]) };

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn simd_to_srgb8_sse2(input: __m128) -> __m128i {
    // clamp between minv/maxv
    let clamped = _mm_min_ps(_mm_max_ps(input, MINV), MAXV);
    // Table index
    let tab_index = _mm_srli_epi32(_mm_castps_si128(clamped), 20);
    // without gather instructions (which might not be a good idea to use
    // anyway), we need to still do 4 separate lookups (despite this). This
    // reduces SIMD parallelism, but it could be a lot worse.
    let indices: [u32; 4] = transmute(tab_index);
    #[cfg(all(not(unstable_bench), test))]
    {
        for &i in &indices {
            debug_assert!(TO_SRGB8_TABLE
                .get(i.checked_sub((127 - 13) * 8).unwrap() as usize)
                .is_some());
        }
    }
    let loaded: [u32; 4] = [
        *TO_SRGB8_TABLE.get_unchecked(*indices.get_unchecked(0) as usize - (127 - 13) * 8),
        *TO_SRGB8_TABLE.get_unchecked(*indices.get_unchecked(1) as usize - (127 - 13) * 8),
        *TO_SRGB8_TABLE.get_unchecked(*indices.get_unchecked(2) as usize - (127 - 13) * 8),
        *TO_SRGB8_TABLE.get_unchecked(*indices.get_unchecked(3) as usize - (127 - 13) * 8),
    ];
    let entry: __m128i = transmute(loaded);

    let tabmult1 = _mm_srli_epi32(_mm_castps_si128(clamped), 12);
    let tabmult2 = _mm_and_si128(tabmult1, MANT_MASK);
    let tabmult3 = _mm_or_si128(tabmult2, TOP_SCALE);
    let tabprod = _mm_madd_epi16(entry, tabmult3);
    _mm_srli_epi32(tabprod, 16)
}

#[inline]
pub unsafe fn simd_to_srgb8(input: [f32; 4]) -> [u8; 4] {
    let res: __m128i = simd_to_srgb8_sse2(transmute(input));
    let [a, b, c, d]: [u32; 4] = transmute(res);
    #[cfg(all(not(unstable_bench), test))]
    {
        debug_assert!([a, b, c, d].iter().all(|v| *v < 256), "{:?}", [a, b, c, d]);
    }
    [a as u8, b as u8, c as u8, d as u8]
    // [vals[0] as u8, vals[1] as u8, vals[2] as u8, vals[3] as u8]
}
