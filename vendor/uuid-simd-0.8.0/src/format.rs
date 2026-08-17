use crate::spec::*;

use vsimd::ascii::AsciiCase;
use vsimd::is_isa_type;
use vsimd::isa::{InstructionSet, SSE2};
use vsimd::tools::{read, write};
use vsimd::vector::V256;
use vsimd::{SIMD128, SIMD256};

#[inline(always)]
const fn char_lut_fallback(case: AsciiCase) -> &'static [u8; 16] {
    match case {
        AsciiCase::Lower => vsimd::hex::LOWER_CHARSET,
        AsciiCase::Upper => vsimd::hex::UPPER_CHARSET,
    }
}

#[inline(always)]
pub unsafe fn format_simple_fallback(src: *const u8, dst: *mut u8, case: AsciiCase) {
    let lut = char_lut_fallback(case).as_ptr();
    for i in 0..16 {
        let x = read(src, i);
        let hi = read(lut, (x >> 4) as usize);
        let lo = read(lut, (x & 0x0f) as usize);
        write(dst, i * 2, hi);
        write(dst, i * 2 + 1, lo);
    }
}

#[inline(always)]
pub unsafe fn format_hyphenated_fallback(src: *const u8, dst: *mut u8, case: AsciiCase) {
    let lut = char_lut_fallback(case).as_ptr();
    let groups = [(0, 8), (9, 13), (14, 18), (19, 23), (24, 36)];

    let mut g = 0;
    let mut i = 0;
    while g < 5 {
        let (start, end) = groups[g];

        let mut j = start;
        while j < end {
            let x = read(src, i);
            i += 1;

            let hi = read(lut, (x >> 4) as usize);
            let lo = read(lut, (x & 0x0f) as usize);
            write(dst, j, hi);
            write(dst, j + 1, lo);
            j += 2;
        }

        if g < 4 {
            write(dst, end, b'-');
        }

        g += 1;
    }
}

#[inline(always)]
const fn char_lut_simd(case: AsciiCase) -> V256 {
    match case {
        AsciiCase::Lower => vsimd::hex::ENCODE_LOWER_LUT,
        AsciiCase::Upper => vsimd::hex::ENCODE_UPPER_LUT,
    }
}

#[inline(always)]
pub unsafe fn format_simple_simd<S: SIMD256>(s: S, src: *const u8, dst: *mut u8, case: AsciiCase) {
    if is_isa_type!(S, SSE2) {
        return format_simple_simd_sse2(SSE2::new(), src, dst, case);
    }
    {
        let lut = char_lut_simd(case);
        let x = s.v128_load_unaligned(src);
        let y = vsimd::hex::encode_bytes16(s, x, lut);
        s.v256_store_unaligned(dst, y);
    }
}

#[inline(always)]
pub unsafe fn format_hyphenated_simd<S: SIMD256>(s: S, src: *const u8, dst: *mut u8, case: AsciiCase) {
    const SWIZZLE: V256 = V256::from_bytes([
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, //
        0x80, 0x08, 0x09, 0x0a, 0x0b, 0x80, 0x0c, 0x0d, //
        0x80, 0x80, 0x80, 0x00, 0x01, 0x02, 0x03, 0x80, //
        0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, //
    ]);

    const DASH: V256 = V256::from_bytes([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x2d, 0x00, 0x00, 0x00, 0x00, 0x2d, 0x00, 0x00, //
        0x00, 0x00, 0x2d, 0x00, 0x00, 0x00, 0x00, 0x2d, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ]);

    let lut = char_lut_simd(case);
    let a = vsimd::hex::encode_bytes16(s, s.v128_load_unaligned(src), lut);

    let a1 = s.u8x16x2_swizzle(a, SWIZZLE);
    let a2 = s.v256_or(a1, DASH);
    s.v256_store_unaligned(dst, a2);

    let a = a.to_v128x2();
    let bytes_14_15 = i16x8_get_lane7(s, a.0) as u16;
    let bytes_28_31 = i32x4_get_lane3(s, a.1) as u32;
    core::ptr::write_unaligned(dst.add(16).cast(), bytes_14_15);
    core::ptr::write_unaligned(dst.add(32).cast(), bytes_28_31);
}

#[inline(always)]
pub unsafe fn format_simple_simd_sse2(s: SSE2, src: *const u8, dst: *mut u8, case: AsciiCase) {
    let offset = match case {
        AsciiCase::Lower => vsimd::hex::sse2::LOWER_OFFSET,
        AsciiCase::Upper => vsimd::hex::sse2::UPPER_OFFSET,
    };

    let x = s.v128_load_unaligned(src);
    let (y1, y2) = vsimd::hex::sse2::encode16(s, x, offset);

    s.v128_store_unaligned(dst, y1);
    s.v128_store_unaligned(dst.add(16), y2);
}
