/*
 * Copyright (c) 2025.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub unsafe fn upsample_horizontal_avx2(
    input: &[i16],
    in_near: &[i16],
    in_far: &[i16],
    scratch: &mut [i16],
    output: &mut [i16],
) {
    assert_eq!(input.len() * 2, output.len());
    assert!(input.len() > 2);

    let len = input.len();

    if len < 18 {
        return super::scalar::upsample_horizontal(input, in_near, in_far, scratch, output);
    }

    // First two pixels
    output[0] = input[0];
    output[1] = (input[0] * 3 + input[1] + 2) >> 2;

    let v_three = _mm256_set1_epi16(3);
    let v_two = _mm256_set1_epi16(2);

    let upsample16 = |input: &[i16; 18], output: &mut [i16; 32]| {
        let in_ptr = input.as_ptr();
        let out_ptr = output.as_mut_ptr();

        // SAFETY: The input is 18 * 16 bit long, so the loads are safe.
        let (v_prev, v_curr, v_next) = unsafe {
            (
                _mm256_loadu_si256(in_ptr.add(0) as *const __m256i),
                _mm256_loadu_si256(in_ptr.add(1) as *const __m256i),
                _mm256_loadu_si256(in_ptr.add(2) as *const __m256i),
            )
        };

        let v_common = _mm256_add_epi16(_mm256_mullo_epi16(v_curr, v_three), v_two);

        let v_even = _mm256_srai_epi16(_mm256_add_epi16(v_common, v_prev), 2);
        let v_odd = _mm256_srai_epi16(_mm256_add_epi16(v_common, v_next), 2);

        let v_res_1 = _mm256_unpacklo_epi16(v_even, v_odd);
        let v_res_2 = _mm256_unpackhi_epi16(v_even, v_odd);

        let v_final_1 = _mm256_permute2x128_si256(v_res_1, v_res_2, 0x20);
        let v_final_2 = _mm256_permute2x128_si256(v_res_1, v_res_2, 0x31);

        // SAFETY: The output is 32 * 16 bit long, so the stores are safe.
        unsafe {
            _mm256_storeu_si256(out_ptr as *mut __m256i, v_final_1);
            _mm256_storeu_si256(out_ptr.add(16) as *mut __m256i, v_final_2);
        }
    };

    for (input, output) in input
        .windows(18)
        .step_by(16)
        .zip(output[2..].chunks_exact_mut(32))
    {
        upsample16(input.try_into().unwrap(), output.try_into().unwrap());
    }

    // Upsample the remainder. This may have some overlap, but that's fine.
    if let Some(rest_input) = input.last_chunk::<18>() {
        let end = output.len() - 2;
        if let Some(rest_output) = output[..end].last_chunk_mut::<32>() {
            upsample16(rest_input, rest_output);
        }
    }

    // Last two pixels.
    output[output.len() - 2] = (3 * input[len - 1] + input[len - 2] + 2) >> 2;
    output[output.len() - 1] = input[len - 1];
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub unsafe fn upsample_vertical_avx2(
    input: &[i16],
    in_near: &[i16],
    in_far: &[i16],
    scratch: &mut [i16],
    output: &mut [i16],
) {
    assert_eq!(input.len() * 2, output.len());
    assert_eq!(in_near.len(), input.len());
    assert_eq!(in_far.len(), input.len());

    let len = input.len();

    if len < 16 {
        return super::scalar::upsample_vertical(input, in_near, in_far, scratch, output);
    }

    let middle = output.len() / 2;
    let (out_top, out_bottom) = output.split_at_mut(middle);

    let v_three = _mm256_set1_epi16(3);
    let v_two = _mm256_set1_epi16(2);

    let upsample16 = |input: &[i16; 16],
                      in_near: &[i16; 16],
                      in_far: &[i16; 16],
                      out_top: &mut [i16; 16],
                      out_bottom: &mut [i16; 16]| {
        // SAFETY: Inputs are all 16 * 16 bit long, so the loads are safe.
        let (v_in, v_near, v_far) = unsafe {
            (
                _mm256_loadu_si256(input.as_ptr() as *const __m256i),
                _mm256_loadu_si256(in_near.as_ptr() as *const __m256i),
                _mm256_loadu_si256(in_far.as_ptr() as *const __m256i),
            )
        };

        let v_common = _mm256_add_epi16(_mm256_mullo_epi16(v_in, v_three), v_two);

        let v_out_top = _mm256_srai_epi16(_mm256_add_epi16(v_common, v_near), 2);
        let v_out_bottom = _mm256_srai_epi16(_mm256_add_epi16(v_common, v_far), 2);

        // SAFETY: Outputs are 16 * 16 bit long, so the stores are safe.
        unsafe {
            _mm256_storeu_si256(out_top.as_mut_ptr() as *mut __m256i, v_out_top);
            _mm256_storeu_si256(out_bottom.as_mut_ptr() as *mut __m256i, v_out_bottom);
        }
    };

    let chunks = input
        .chunks_exact(16)
        .zip(in_near.chunks_exact(16))
        .zip(in_far.chunks_exact(16))
        .zip(out_top.chunks_exact_mut(16))
        .zip(out_bottom.chunks_exact_mut(16));

    for ((((input, in_near), in_far), out_top), out_bottom) in chunks {
        upsample16(
            input.try_into().unwrap(),
            in_near.try_into().unwrap(),
            in_far.try_into().unwrap(),
            out_top.try_into().unwrap(),
            out_bottom.try_into().unwrap(),
        );
    }

    // Upsample the remainder. This may have some overlap, but that's fine.
    // Edition upgrade will fix this nested awfulness.
    if let Some(rest) = input.last_chunk::<16>() {
        if let Some(rest_near) = in_near.last_chunk::<16>() {
            if let Some(rest_far) = in_far.last_chunk::<16>() {
                if let Some(mut rest_top) = out_top.last_chunk_mut::<16>() {
                    if let Some(mut rest_bottom) = out_bottom.last_chunk_mut::<16>() {
                        upsample16(rest, rest_near, rest_far, &mut rest_top, &mut rest_bottom);
                    }
                }
            }
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub unsafe fn upsample_hv_avx2(
    input: &[i16],
    in_near: &[i16],
    in_far: &[i16],
    scratch_space: &mut [i16],
    output: &mut [i16],
) {
    assert_eq!(input.len() * 4, output.len());
    assert!(input.len() * 2 <= scratch_space.len());
    let scratch_space = &mut scratch_space[..input.len() * 2];


    upsample_vertical_avx2(input, in_near, in_far, &mut [], scratch_space);

    let scratch_half = scratch_space.len() / 2;
    let output_half = output.len() / 2;

    let (scratch_top, scratch_bottom) = scratch_space.split_at_mut(scratch_half);
    let (out_top, out_bottom) = output.split_at_mut(output_half);

    let mut t = [0];
    upsample_horizontal_avx2(scratch_top, &[], &[], &mut t, out_top);
    upsample_horizontal_avx2(scratch_bottom, &[], &[], &mut t, out_bottom);
}
