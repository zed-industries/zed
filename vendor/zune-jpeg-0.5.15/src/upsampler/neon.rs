/*
 * Copyright (c) 2025.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn upsample_horizontal_neon(
    input: &[i16], in_near: &[i16], in_far: &[i16], scratch: &mut [i16], output: &mut [i16]
) {
    assert_eq!(input.len() * 2, output.len());
    assert!(input.len() > 2);

    let len = input.len();

    if len < 10 {
        return super::scalar::upsample_horizontal(input, in_near, in_far, scratch, output);
    }

    // First two pixels
    output[0] = input[0];
    output[1] = (input[0] * 3 + input[1] + 2) >> 2;

    let v_three = vdupq_n_s16(3);
    let v_two = vdupq_n_s16(2);

    let upsample8 = |input: &[i16; 10], output: &mut [i16; 16]| {
        let in_ptr = input.as_ptr();
        let out_ptr = output.as_mut_ptr();

        // SAFETY: The input is 10 * 16 bit long, so the loads are safe.
        let (v_prev, v_curr, v_next) = unsafe {
            (
                vld1q_s16(in_ptr),
                vld1q_s16(in_ptr.add(1)),
                vld1q_s16(in_ptr.add(2))
            )
        };

        let v_common = vaddq_s16(vmulq_s16(v_curr, v_three), v_two);
        let v_even = vshrq_n_s16::<2>(vaddq_s16(v_common, v_prev));
        let v_odd = vshrq_n_s16::<2>(vaddq_s16(v_common, v_next));

        let v_res_1 = vzip1q_s16(v_even, v_odd);
        let v_res_2 = vzip2q_s16(v_even, v_odd);

        // SAFETY: The output is 16 * 16 bit long, so the stores are safe.
        unsafe {
            vst1q_s16(out_ptr, v_res_1);
            vst1q_s16(out_ptr.add(8), v_res_2);
        }
    };

    for (input, output) in input
        .windows(10)
        .step_by(8)
        .zip(output[2..].chunks_exact_mut(16))
    {
        upsample8(input.try_into().unwrap(), output.try_into().unwrap());
    }

    // Upsample the remainder. This may have some overlap, but that's fine.
    if let Some(rest_input) = input.last_chunk::<10>() {
        let end = output.len() - 2;
        if let Some(rest_output) = output[..end].last_chunk_mut::<16>() {
            upsample8(rest_input, rest_output);
        }
    }

    // Last two pixels.
    output[output.len() - 2] = (3 * input[len - 1] + input[len - 2] + 2) >> 2;
    output[output.len() - 1] = input[len - 1];
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn upsample_vertical_neon(
    input: &[i16], in_near: &[i16], in_far: &[i16], scratch: &mut [i16], output: &mut [i16]
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

    let v_three = vdupq_n_s16(3);
    let v_two = vdupq_n_s16(2);

    let upsample8 = |input: &[i16; 8],
                     in_near: &[i16; 8],
                     in_far: &[i16; 8],
                     out_top: &mut [i16; 8],
                     out_bottom: &mut [i16; 8]| {
        // SAFETY: Inputs are all 8 * 16 bit long, so the loads are safe.
        let (v_in, v_near, v_far) = unsafe {
            (
                vld1q_s16(input.as_ptr()),
                vld1q_s16(in_near.as_ptr()),
                vld1q_s16(in_far.as_ptr())
            )
        };

        let v_common = vaddq_s16(vmulq_s16(v_in, v_three), v_two);
        let v_out_top = vshrq_n_s16::<2>(vaddq_s16(v_common, v_near));
        let v_out_bottom = vshrq_n_s16::<2>(vaddq_s16(v_common, v_far));

        // SAFETY: Outputs are 8 * 16 bit long, so the stores are safe.
        unsafe {
            vst1q_s16(out_top.as_mut_ptr(), v_out_top);
            vst1q_s16(out_bottom.as_mut_ptr(), v_out_bottom);
        }
    };

    let chunks = input
        .chunks_exact(8)
        .zip(in_near.chunks_exact(8))
        .zip(in_far.chunks_exact(8))
        .zip(out_top.chunks_exact_mut(8))
        .zip(out_bottom.chunks_exact_mut(8));

    for ((((input, in_near), in_far), out_top), out_bottom) in chunks {
        upsample8(
            input.try_into().unwrap(),
            in_near.try_into().unwrap(),
            in_far.try_into().unwrap(),
            out_top.try_into().unwrap(),
            out_bottom.try_into().unwrap()
        );
    }

    // Upsample the remainder.
    if let Some(rest) = input.last_chunk::<8>() {
        if let Some(rest_near) = in_near.last_chunk::<8>() {
            if let Some(rest_far) = in_far.last_chunk::<8>() {
                if let Some(mut rest_top) = out_top.last_chunk_mut::<8>() {
                    if let Some(mut rest_bottom) = out_bottom.last_chunk_mut::<8>() {
                        upsample8(rest, rest_near, rest_far, &mut rest_top, &mut rest_bottom);
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn upsample_hv_neon(
    input: &[i16], in_near: &[i16], in_far: &[i16], scratch_space: &mut [i16], output: &mut [i16]
) {
    assert_eq!(input.len() * 4, output.len());

    assert!(input.len() * 2 <= scratch_space.len());
    let scratch_space = &mut scratch_space[..input.len() * 2];

    // SAFETY: caller guarantees NEON is available
    unsafe {
        upsample_vertical_neon(input, in_near, in_far, &mut [], scratch_space);

        let scratch_half = scratch_space.len() / 2;
        let output_half = output.len() / 2;

        let (scratch_top, scratch_bottom) = scratch_space.split_at_mut(scratch_half);
        let (out_top, out_bottom) = output.split_at_mut(output_half);

        let mut t = [0];
        upsample_horizontal_neon(scratch_top, &[], &[], &mut t, out_top);
        upsample_horizontal_neon(scratch_bottom, &[], &[], &mut t, out_bottom);
    }
}
