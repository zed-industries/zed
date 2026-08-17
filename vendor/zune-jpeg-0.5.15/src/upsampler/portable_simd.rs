/*
 * Copyright (c) 2023.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

use std::simd::prelude::*;

const LANES: usize = 16;
type V = Simd<i16, LANES>;

pub fn upsample_horizontal_simd(
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

    let v_three = V::splat(3);
    let v_two = V::splat(2);

    let upsample16 = |input: &[i16; 18], output: &mut [i16; 32]| {
        let v_prev = V::from_slice(&input[0..LANES]);
        let v_curr = V::from_slice(&input[1..LANES + 1]);
        let v_next = V::from_slice(&input[2..LANES + 2]);

        let v_common = v_curr * v_three + v_two;

        let v_even = (v_common + v_prev) >> 2;
        let v_odd = (v_common + v_next) >> 2;

        let (v_res_1, v_res_2) = v_even.interleave(v_odd);

        v_res_1.copy_to_slice(&mut output[0..LANES]);
        v_res_2.copy_to_slice(&mut output[LANES..2 * LANES]);
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

pub fn upsample_vertical_simd(
    input: &[i16],
    in_near: &[i16],
    in_far: &[i16],
    _scratch_space: &mut [i16],
    output: &mut [i16],
) {
    assert_eq!(input.len() * 2, output.len());
    assert_eq!(in_near.len(), input.len());
    assert_eq!(in_far.len(), input.len());

    let len = input.len();

    if len < 16 {
        return super::scalar::upsample_vertical(input, in_near, in_far, _scratch_space, output);
    }

    let middle = output.len() / 2;
    let (out_top, out_bottom) = output.split_at_mut(middle);

    let v_three = V::splat(3);
    let v_two = V::splat(2);

    let upsample16 = |input: &[i16; 16],
                      in_near: &[i16; 16],
                      in_far: &[i16; 16],
                      out_top: &mut [i16; 16],
                      out_bottom: &mut [i16; 16]| {
        let v_in = V::from(*input);
        let v_near = V::from(*in_near);
        let v_far = V::from(*in_far);

        let v_common = v_in * v_three + v_two;

        let v_out_top = (v_common + v_near) >> 2;
        let v_out_bottom = (v_common + v_far) >> 2;

        v_out_top.copy_to_slice(out_top.as_mut_slice());
        v_out_bottom.copy_to_slice(out_bottom.as_mut_slice());
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
        if let Some( rest_near) = in_near.last_chunk::<16>() {
            if let Some( rest_far) = in_far.last_chunk::<16>() {
                if let Some( rest_top) = out_top.last_chunk_mut::<16>() {
                    if let Some(  rest_bottom) = out_bottom.last_chunk_mut::<16>() {
                        upsample16(rest, rest_near, rest_far, rest_top, rest_bottom);
                    }
                }
            }
        }
    }
}

pub fn upsample_hv_simd(
    input: &[i16],
    in_near: &[i16],
    in_far: &[i16],
    scratch_space: &mut [i16],
    output: &mut [i16],
) {
    assert_eq!(input.len() * 4, output.len());
    
    assert!(input.len() * 2 <= scratch_space.len());
    let scratch_space = &mut scratch_space[..input.len() * 2];


    upsample_vertical_simd(input, in_near, in_far, &mut [], scratch_space);

    let scratch_half = scratch_space.len() / 2;
    let output_half = output.len() / 2;

    let (scratch_top, scratch_bottom) = scratch_space.split_at_mut(scratch_half);
    let (out_top, out_bottom) = output.split_at_mut(output_half);

    let mut t = [0];
    upsample_horizontal_simd(scratch_top, &[], &[], &mut t, out_top);
    upsample_horizontal_simd(scratch_bottom, &[], &[], &mut t, out_bottom);
}
