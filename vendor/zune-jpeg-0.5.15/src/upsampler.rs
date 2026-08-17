/*
 * Copyright (c) 2023.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

//! Up-sampling routines
//!
//! The main upsampling method is a bi-linear interpolation or a "triangle
//! filter " or libjpeg turbo `fancy_upsampling` which is a good compromise
//! between speed and visual quality
//!
//! # The filter
//! Each output pixel is made from `(3*A+B)/4` where A is the original
//! pixel closer to the output and B is the one further.
//!
//! ```text
//!+---+---+
//! | A | B |
//! +---+---+
//! +-+-+-+-+
//! | |P| | |
//! +-+-+-+-+
//! ```
//!
//! # Horizontal Bi-linear filter
//! ```text
//! |---+-----------+---+
//! |   |           |   |
//! | A | |p1 | p2| | B |
//! |   |           |   |
//! |---+-----------+---+
//!
//! ```
//! For a horizontal bi-linear it's trivial to implement,
//!
//! `A` becomes the input closest to the output.
//!
//! `B` varies depending on output.
//!  - For odd positions, input is the `next` pixel after A
//!  - For even positions, input is the `previous` value before A.
//!
//! We iterate in a classic 1-D sliding window with a window of 3.
//! For our sliding window approach, `A` is the 1st and `B` is either the 0th term or 2nd term
//! depending on position we are writing.(see scalar code).
//!
//! For vector code see module sse for explanation.
//!
//! # Vertical bi-linear.
//! Vertical up-sampling is a bit trickier.
//!
//! ```text
//! +----+----+
//! | A1 | A2 |
//! +----+----+
//! +----+----+
//! | p1 | p2 |
//! +----+-+--+
//! +----+-+--+
//! | p3 | p4 |
//! +----+-+--+
//! +----+----+
//! | B1 | B2 |
//! +----+----+
//! ```
//!
//! For `p1`
//! - `A1` is given a weight of `3` and `B1` is given a weight of 1.
//!
//! For `p3`
//! - `B1` is given a weight of `3` and `A1` is given a weight of 1
//!
//! # Horizontal vertical downsampling/chroma quartering.
//!
//! Carry out a vertical filter in the first pass, then a horizontal filter in the second pass.
#![allow(unreachable_code)]
use zune_core::options::DecoderOptions;

use crate::components::UpSampler;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "x86")]
mod avx2;
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "neon")]
mod neon;
#[cfg(feature = "portable_simd")]
mod portable_simd;
mod scalar;

// choose the best possible implementation for this platform
#[allow(unused_variables)]
pub fn choose_horizontal_samp_function(options: &DecoderOptions) -> UpSampler {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[cfg(feature = "x86")]
    if options.use_avx2() {
        return |a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
            // SAFETY: `options.use_avx2()` only returns true if avx2 is supported.
            unsafe { avx2::upsample_horizontal_avx2(a, b, c, d, e) }
        };
    }
    #[cfg(target_arch = "aarch64")]
    #[cfg(feature = "neon")]
    if options.use_neon() {
        return |a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
            // SAFETY: `options.use_neon()` only returns true if neon is supported.
            unsafe { neon::upsample_horizontal_neon(a, b, c, d, e) }
        };
    }
    #[cfg(feature = "portable_simd")]
    return portable_simd::upsample_horizontal_simd;
    return scalar::upsample_horizontal;
}

#[allow(unused_variables)]
pub fn choose_hv_samp_function(options: &DecoderOptions) -> UpSampler {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[cfg(feature = "x86")]
    if options.use_avx2() {
        return |a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
            // SAFETY: `options.use_avx2()` only returns true if avx2 is supported.
            unsafe { avx2::upsample_hv_avx2(a, b, c, d, e) }
        };
    }
    #[cfg(target_arch = "aarch64")]
    #[cfg(feature = "neon")]
    if options.use_neon() {
        return |a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
            // SAFETY: `options.use_neon()` only returns true if neon is supported.
            unsafe { neon::upsample_hv_neon(a, b, c, d, e) }
        };
    }
    #[cfg(feature = "portable_simd")]
    return portable_simd::upsample_hv_simd;
    return scalar::upsample_hv;
}

#[allow(unused_variables)]
pub fn choose_v_samp_function(options: &DecoderOptions) -> UpSampler {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[cfg(feature = "x86")]
    if options.use_avx2() {
        return |a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
            // SAFETY: `options.use_avx2()` only returns true if avx2 is supported.
            unsafe { avx2::upsample_vertical_avx2(a, b, c, d, e) }
        };
    }
    #[cfg(target_arch = "aarch64")]
    #[cfg(feature = "neon")]
    if options.use_neon() {
        return |a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
            // SAFETY: `options.use_neon()` only returns true if neon is supported.
            unsafe { neon::upsample_vertical_neon(a, b, c, d, e) }
        };
    }
    #[cfg(feature = "portable_simd")]
    return portable_simd::upsample_vertical_simd;
    return scalar::upsample_vertical;
}

/// Upsample nothing

pub fn upsample_no_op(
    _input: &[i16],
    _in_ref: &[i16],
    _in_near: &[i16],
    _scratch_space: &mut [i16],
    _output: &mut [i16],
) {
}

pub fn generic_sampler() -> UpSampler {
    scalar::upsample_generic
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "portable_simd")]
    mod portable_simd_impl {
        use super::*;

        #[test]
        fn portable_simd_vertical() {
            _test_vertical(portable_simd::upsample_vertical_simd)
        }

        #[test]
        fn portable_simd_horizontal() {
            _test_horizontal(portable_simd::upsample_horizontal_simd)
        }

        #[test]
        fn portable_simd_hv() {
            _test_hv(portable_simd::upsample_hv_simd)
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[cfg(feature = "x86")]
    #[cfg(target_feature = "avx2")]
    mod avx2_impl {
        use super::*;

        #[test]
        fn avx2_vertical() {
            _test_vertical(|a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
                // SAFETY: Test guarded behind `target_feature`
                unsafe { avx2::upsample_vertical_avx2(a, b, c, d, e) }
            })
        }

        #[test]
        fn avx2_horizontal() {
            _test_horizontal(|a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
                // SAFETY: Test guarded behind `target_feature`
                unsafe { avx2::upsample_horizontal_avx2(a, b, c, d, e) }
            })
        }

        #[test]
        fn avx2_hv() {
            _test_hv(|a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
                // SAFETY: Test guarded behind `target_feature`
                unsafe { avx2::upsample_hv_avx2(a, b, c, d, e) }
            })
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[cfg(feature = "neon")]
    #[cfg(target_feature = "neon")]
    mod neon_impl {
        use super::*;

        #[test]
        fn neon_vertical() {
            _test_vertical(|a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
                // SAFETY: Test guarded behind `target_feature`
                unsafe { neon::upsample_vertical_neon(a, b, c, d, e) }
            })
        }

        #[test]
        fn neon_horizontal() {
            _test_horizontal(|a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
                // SAFETY: Test guarded behind `target_feature`
                unsafe { neon::upsample_horizontal_neon(a, b, c, d, e) }
            })
        }

        #[test]
        fn neon_hv() {
            _test_hv(|a: &[i16], b: &[i16], c: &[i16], d: &mut [i16], e: &mut [i16]| {
                // SAFETY: Test guarded behind `target_feature`
                unsafe { neon::upsample_hv_neon(a, b, c, d, e) }
            })
        }
    }

    fn _test_vertical(upsampler: UpSampler) {
        let width = 1024;
        let input: Vec<i16> = (0..width).map(|x| ((x + 10) % 256) as i16).collect();
        let in_near: Vec<i16> = (0..width).map(|x| ((x + 20) % 256) as i16).collect();
        let in_far: Vec<i16> = (0..width).map(|x| ((x + 30) % 256) as i16).collect();
        let mut scratch = vec![0i16; width];

        let mut output_scalar = vec![0i16; width * 2];
        let mut output_fast = vec![0i16; width * 2];

        scalar::upsample_vertical(&input, &in_near, &in_far, &mut scratch, &mut output_scalar);
        upsampler(&input, &in_near, &in_far, &mut scratch, &mut output_fast);

        assert_eq!(output_scalar, output_fast);
    }

    fn _test_horizontal(upsampler: UpSampler) {
        _test_horizontal_even_width(upsampler);
        _test_horizontal_odd_width(upsampler);
    }

    fn _test_horizontal_even_width(upsampler: UpSampler) {
        let width = 1024;
        let input: Vec<i16> = (0..width).map(|x| ((x + 10) % 256) as i16).collect();

        let mut scratch = vec![0i16; width];

        let mut output_scalar = vec![0i16; width * 2];
        let mut output_fast = vec![0i16; width * 2];

        scalar::upsample_horizontal(&input, &[], &[], &mut scratch, &mut output_scalar);
        upsampler(&input, &[], &[], &mut scratch, &mut output_fast);

        assert_eq!(output_scalar, output_fast);
    }

    fn _test_horizontal_odd_width(upsampler: UpSampler) {
        let width = 33;
        let input: Vec<i16> = (0..width).map(|x| ((x + 10) % 256) as i16).collect();
        let mut scratch = vec![0i16; width];
        let mut output_scalar = vec![0i16; width * 2];
        let mut output_fast = vec![0i16; width * 2];

        scalar::upsample_horizontal(&input, &[], &[], &mut scratch, &mut output_scalar);
        upsampler(&input, &[], &[], &mut scratch, &mut output_fast);

        assert_eq!(output_scalar, output_fast);
    }

    fn _test_hv(upsampler: UpSampler) {
        let width = 512;
        let input: Vec<i16> = (0..width).map(|x| ((x + 10) % 256) as i16).collect();
        let in_near: Vec<i16> = (0..width).map(|x| ((x + 20) % 256) as i16).collect();
        let in_far: Vec<i16> = (0..width).map(|x| ((x + 30) % 256) as i16).collect();

        // Output len is width * 4 for HV (vertical * 2, then horizontal * 2 for each row)
        // scratch is width * 2
        let mut scratch_scalar = vec![0i16; width * 2];
        let mut scratch_fast = vec![0i16; width * 2];
        let mut output_scalar = vec![0i16; width * 4];
        let mut output_fast = vec![0i16; width * 4];

        scalar::upsample_hv(
            &input,
            &in_near,
            &in_far,
            &mut scratch_scalar,
            &mut output_scalar,
        );
        upsampler(
            &input,
            &in_near,
            &in_far,
            &mut scratch_fast,
            &mut output_fast,
        );

        assert_eq!(output_scalar, output_fast);
    }
}
