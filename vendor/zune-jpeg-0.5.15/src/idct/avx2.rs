/*
 * Copyright (c) 2023.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//! AVX optimised IDCT.
//!
//! Okay not thaat optimised.
//!
//!
//! # The implementation
//! The implementation is neatly broken down into two operations.
//!
//! 1. Test for zeroes
//! > There is a shortcut method for idct  where when all AC values are zero, we can get the answer really quickly.
//!  by scaling the 1/8th of the DCT coefficient of the block to the whole block and level shifting.
//!
//! 2. If above fails, we proceed to carry out IDCT as a two pass one dimensional algorithm.
//! IT does two whole scans where it carries out IDCT on all items
//! After each successive scan, data is transposed in register(thank you x86 SIMD powers). and the second
//! pass is carried out.
//!
//! The code is not super optimized, it produces bit identical results with scalar code hence it's
//! `mm256_add_epi16`
//! and it also has the advantage of making this implementation easy to maintain.

#![cfg(feature = "x86")]
#![allow(dead_code)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::unsafe_utils::{transpose, YmmRegister};

const SCALE_BITS: i32 = 512 + 65536 + (128 << 17);

// Pack i32 to i16's,
// clamp them to be between 0-255
// Undo shuffling
// Store back to array
macro_rules! permute_store {
    ($x:tt,$y:tt,$index:tt,$out:tt,$stride:tt) => {
        let a = _mm256_packs_epi32($x, $y);

        // Clamp the values after packing, we can clamp more values at once
        let b = clamp_avx(a);
        let mut tmp = [0;8];
        // /Undo shuffling
        let c = _mm256_permute4x64_epi64(b, shuffle(3, 1, 2, 0));

        // store first vector
        _mm_storeu_si128(
            ($out)
                .get_mut($index..$index + 8)
                .unwrap_or(&mut tmp)
                .as_mut_ptr()
                .cast(),
            _mm256_extractf128_si256::<0>(c),
        );
        $index += $stride;
        // second vector
        _mm_storeu_si128(
            ($out)
                .get_mut($index..$index + 8)
                .unwrap()
                .as_mut_ptr()
                .cast(),
            _mm256_extractf128_si256::<1>(c),
        );
        $index += $stride;
    };
}

#[target_feature(enable = "avx2")]
#[allow(
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::similar_names,
    clippy::op_ref,
    unused_assignments,
    clippy::zero_prefixed_literal
)]
pub unsafe fn idct_avx2(
    in_vector: &mut [i32; 64], out_vector: &mut [i16], stride: usize,
) {
    let mut pos = 0;

    // load into registers
    //
    // We sign extend i16's to i32's and calculate them with extended precision and
    // later reduce them to i16's when we are done carrying out IDCT

    let rw0 = _mm256_loadu_si256(in_vector[00..].as_ptr().cast());
    let rw1 = _mm256_loadu_si256(in_vector[08..].as_ptr().cast());
    let rw2 = _mm256_loadu_si256(in_vector[16..].as_ptr().cast());
    let rw3 = _mm256_loadu_si256(in_vector[24..].as_ptr().cast());
    let rw4 = _mm256_loadu_si256(in_vector[32..].as_ptr().cast());
    let rw5 = _mm256_loadu_si256(in_vector[40..].as_ptr().cast());
    let rw6 = _mm256_loadu_si256(in_vector[48..].as_ptr().cast());
    let rw7 = _mm256_loadu_si256(in_vector[56..].as_ptr().cast());

    // Forward DCT and quantization may cause all the AC terms to be zero, for such
    // cases we can try to accelerate it

    // Basically the poop is that whenever the array has 63 zeroes, its idct is
    // (arr[0]>>3)or (arr[0]/8) propagated to all the elements.
    // We first test to see if the array contains zero elements and if it does, we go the
    // short way.
    //
    // This reduces IDCT overhead from about 39% to 18 %, almost half

    // Do another load for the first row, we don't want to check DC value, because
    // we only care about AC terms
    let rw8 = _mm256_loadu_si256(in_vector[1..].as_ptr().cast());

    let mut bitmap = _mm256_or_si256(rw1, rw2);
    bitmap = _mm256_or_si256(bitmap, rw3);
    bitmap = _mm256_or_si256(bitmap, rw4);
    bitmap = _mm256_or_si256(bitmap, rw5);
    bitmap = _mm256_or_si256(bitmap, rw6);
    bitmap = _mm256_or_si256(bitmap, rw7);
    bitmap = _mm256_or_si256(bitmap, rw8);

    if _mm256_testz_si256(bitmap, bitmap) == 1 {
        // AC terms all zero, idct of the block is ( coeff[0] * qt[0] )/8 + 128 (bias)
        // (and clamped to 255)
        // Round by adding 0.5 * (1 << 3) and offset by adding (128 << 3) before scaling
        let coeff = ((in_vector[0] + 4 + 1024) >> 3).clamp(0, 255) as i16;
        let idct_value = _mm_set1_epi16(coeff);

        macro_rules! store {
            ($pos:tt,$value:tt) => {
                // store
                _mm_storeu_si128(
                    out_vector
                        .get_mut($pos..$pos + 8)
                        .unwrap()
                        .as_mut_ptr()
                        .cast(),
                    $value,
                );
                $pos += stride;
            };
        }
        store!(pos, idct_value);
        store!(pos, idct_value);
        store!(pos, idct_value);
        store!(pos, idct_value);

        store!(pos, idct_value);
        store!(pos, idct_value);
        store!(pos, idct_value);
        store!(pos, idct_value);

        return;
    }

    let mut row0 = YmmRegister { mm256: rw0 };
    let mut row1 = YmmRegister { mm256: rw1 };
    let mut row2 = YmmRegister { mm256: rw2 };
    let mut row3 = YmmRegister { mm256: rw3 };

    let mut row4 = YmmRegister { mm256: rw4 };
    let mut row5 = YmmRegister { mm256: rw5 };
    let mut row6 = YmmRegister { mm256: rw6 };
    let mut row7 = YmmRegister { mm256: rw7 };

    macro_rules! dct_pass {
        ($SCALE_BITS:tt,$scale:tt) => {
            // There are a lot of ways to do this
            // but to keep it simple(and beautiful), ill make a direct translation of the
            // scalar code to also make this code fully transparent(this version and the non
            // avx one should produce identical code.)

            // even part
            let p1 = (row2 + row6) * 2217;

            let mut t2 = p1 + row6 * -7567;
            let mut t3 = p1 + row2 * 3135;

            let mut t0 = YmmRegister {
                mm256: _mm256_slli_epi32((row0 + row4).mm256, 12),
            };
            let mut t1 = YmmRegister {
                mm256: _mm256_slli_epi32((row0 - row4).mm256, 12),
            };

            let x0 = t0 + t3 + $SCALE_BITS;
            let x3 = t0 - t3 + $SCALE_BITS;
            let x1 = t1 + t2 + $SCALE_BITS;
            let x2 = t1 - t2 + $SCALE_BITS;

            let p3 = row7 + row3;
            let p4 = row5 + row1;
            let p1 = row7 + row1;
            let p2 = row5 + row3;
            let p5 = (p3 + p4) * 4816;

            t0 = row7 * 1223;
            t1 = row5 * 8410;
            t2 = row3 * 12586;
            t3 = row1 * 6149;

            let p1 = p5 + p1 * -3685;
            let p2 = p5 + (p2 * -10497);
            let p3 = p3 * -8034;
            let p4 = p4 * -1597;

            t3 += p1 + p4;
            t2 += p2 + p3;
            t1 += p2 + p4;
            t0 += p1 + p3;

            row0.mm256 = _mm256_srai_epi32((x0 + t3).mm256, $scale);
            row1.mm256 = _mm256_srai_epi32((x1 + t2).mm256, $scale);
            row2.mm256 = _mm256_srai_epi32((x2 + t1).mm256, $scale);
            row3.mm256 = _mm256_srai_epi32((x3 + t0).mm256, $scale);

            row4.mm256 = _mm256_srai_epi32((x3 - t0).mm256, $scale);
            row5.mm256 = _mm256_srai_epi32((x2 - t1).mm256, $scale);
            row6.mm256 = _mm256_srai_epi32((x1 - t2).mm256, $scale);
            row7.mm256 = _mm256_srai_epi32((x0 - t3).mm256, $scale);
        };
    }

    // Process rows
    dct_pass!(512, 10);
    transpose(
        &mut row0, &mut row1, &mut row2, &mut row3, &mut row4, &mut row5, &mut row6, &mut row7,
    );

    // process columns
    dct_pass!(SCALE_BITS, 17);
    transpose(
        &mut row0, &mut row1, &mut row2, &mut row3, &mut row4, &mut row5, &mut row6, &mut row7,
    );
    // Pack and write the values back to the array
    permute_store!((row0.mm256), (row1.mm256), pos, out_vector, stride);
    permute_store!((row2.mm256), (row3.mm256), pos, out_vector, stride);
    permute_store!((row4.mm256), (row5.mm256), pos, out_vector, stride);
    permute_store!((row6.mm256), (row7.mm256), pos, out_vector, stride);
}


#[target_feature(enable = "avx2")]
#[allow(
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::similar_names,
    clippy::op_ref,
    unused_assignments,
    clippy::zero_prefixed_literal
)]
pub unsafe fn idct_avx2_4x4(
    in_vector: &mut [i32; 64], out_vector: &mut [i16], stride: usize,
) {
    let rw0 = _mm256_loadu_si256(in_vector[00..].as_ptr().cast());
    let rw1 = _mm256_loadu_si256(in_vector[08..].as_ptr().cast());
    let rw2 = _mm256_loadu_si256(in_vector[16..].as_ptr().cast());
    let rw3 = _mm256_loadu_si256(in_vector[24..].as_ptr().cast());

    let mut row0 = YmmRegister { mm256: rw0 };
    let mut row1 = YmmRegister { mm256: rw1 };
    let mut row2 = YmmRegister { mm256: rw2 };
    let mut row3 = YmmRegister { mm256: rw3 };

    let mut row4 = YmmRegister { mm256: rw0 };
    let mut row5 = YmmRegister { mm256: rw0 };
    let mut row6 = YmmRegister { mm256: rw0 };
    let mut row7 = YmmRegister { mm256: rw0 };

    {
        row0.mm256 = _mm256_slli_epi32(row0.mm256, 12);
        row0 += 512;

        let i2 = row2;

        let p1 = i2 * 2217;
        let p3 = i2 * 5352;

        let x0 = row0 + p3;
        let x1 = row0 + p1;
        let x2 = row0 - p1;
        let x3 = row0 - p3;

        // odd part
        let i4 = row3;
        let i3 = row1;

        let p5 = (i4 + i3) * 4816;

        let p1 = p5 + i3 * -3685;
        let p2 = p5 + i4 * -10497;

        let t3 = p5 + i3 * 867;
        let t2 = p5 + i4 * -5945;

        let t1 = p2 + i3 * -1597;
        let t0 = p1 + i4 * -8034;

        row0.mm256 = _mm256_srai_epi32((x0 + t3).mm256, 10);
        row1.mm256 = _mm256_srai_epi32((x1 + t2).mm256, 10);
        row2.mm256 = _mm256_srai_epi32((x2 + t1).mm256, 10);
        row3.mm256 = _mm256_srai_epi32((x3 + t0).mm256, 10);

        row4.mm256 = _mm256_srai_epi32((x3 - t0).mm256, 10);
        row5.mm256 = _mm256_srai_epi32((x2 - t1).mm256, 10);
        row6.mm256 = _mm256_srai_epi32((x1 - t2).mm256, 10);
        row7.mm256 = _mm256_srai_epi32((x0 - t3).mm256, 10);
    }

    transpose(
        &mut row0, &mut row1, &mut row2, &mut row3, &mut row4, &mut row5, &mut row6, &mut row7,
    );

    {
        let i2 = row2;
        let i0 = row0;

        row0.mm256 = _mm256_slli_epi32(i0.mm256, 12);
        let t0 = row0 + SCALE_BITS;

        let t2 = i2 * 2217;
        let t3 = i2 * 5352;

        // constants scaled things up by 1<<12, plus we had 1<<2 from first
        // loop, plus horizontal and vertical each scale by sqrt(8) so together
        // we've got an extra 1<<3, so 1<<17 total we need to remove.
        // so we want to round that, which means adding 0.5 * 1<<17,
        // aka 65536. Also, we'll end up with -128 to 127 that we want
        // to encode as 0..255 by adding 128, so we'll add that before the shift
        // Rounding constant is already added into `t0`
        let x0 = t0 + t3;
        let x3 = t0 - t3;
        let x1 = t0 + t2;
        let x2 = t0 - t2;

        // odd part
        let i3 = row3;
        let i1 = row1;

        let p5 = (i3 + i1) * 4816;

        let p1 = p5 + i1 * -3685;
        let p2 = p5 + i3 * -10497;

        let t3 = p5 + i1 * 867;
        let t2 = p5 + i3 * -5945;

        let t1 = p2 + i1 * -1597;
        let t0 = p1 + i3 * -8034;

        row0.mm256 = _mm256_srai_epi32((x0 + t3).mm256, 17);
        row1.mm256 = _mm256_srai_epi32((x1 + t2).mm256, 17);
        row2.mm256 = _mm256_srai_epi32((x2 + t1).mm256, 17);
        row3.mm256 = _mm256_srai_epi32((x3 + t0).mm256, 17);
        row4.mm256 = _mm256_srai_epi32((x3 - t0).mm256, 17);
        row5.mm256 = _mm256_srai_epi32((x2 - t1).mm256, 17);
        row6.mm256 = _mm256_srai_epi32((x1 - t2).mm256, 17);
        row7.mm256 = _mm256_srai_epi32((x0 - t3).mm256, 17);
    }

    transpose(
        &mut row0, &mut row1, &mut row2, &mut row3, &mut row4, &mut row5, &mut row6, &mut row7,
    );

    let mut pos = 0;

    // Pack and write the values back to the array
    permute_store!((row0.mm256), (row1.mm256), pos, out_vector, stride);
    permute_store!((row2.mm256), (row3.mm256), pos, out_vector, stride);
    permute_store!((row4.mm256), (row5.mm256), pos, out_vector, stride);
    permute_store!((row6.mm256), (row7.mm256), pos, out_vector, stride);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn clamp_avx(reg: __m256i) -> __m256i {
    let min_s = _mm256_set1_epi16(0);
    let max_s = _mm256_set1_epi16(255);

    let max_v = _mm256_max_epi16(reg, min_s); //max(a,0)
    let min_v = _mm256_min_epi16(max_v, max_s); //min(max(a,0),255)
    return min_v;
}

/// A copy of `_MM_SHUFFLE()` that doesn't require
/// a nightly compiler
#[inline]
const fn shuffle(z: i32, y: i32, x: i32, w: i32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w)
}
