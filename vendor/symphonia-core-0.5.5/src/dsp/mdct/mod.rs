// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `mdct` module implements the Modified Discrete Cosine Transform (MDCT).
//!
//! The MDCT in this module is implemented in-terms of a forward FFT.

#[cfg(any(feature = "opt-simd-sse", feature = "opt-simd-avx", feature = "opt-simd-neon"))]
mod simd;

#[cfg(any(feature = "opt-simd-sse", feature = "opt-simd-avx", feature = "opt-simd-neon"))]
pub use simd::*;

#[cfg(not(any(feature = "opt-simd-sse", feature = "opt-simd-avx", feature = "opt-simd-neon")))]
mod no_simd;
#[cfg(not(any(
    feature = "opt-simd-sse",
    feature = "opt-simd-avx",
    feature = "opt-simd-neon"
)))]
pub use no_simd::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    fn imdct_analytical(x: &[f32], y: &mut [f32], scale: f64) {
        assert!(y.len() == 2 * x.len());

        // Generates 2N outputs from N inputs.
        let n_in = x.len();
        let n_out = x.len() << 1;

        let pi_2n = f64::consts::PI / (2 * n_out) as f64;

        for (i, item) in y.iter_mut().enumerate().take(n_out) {
            let accum: f64 = x
                .iter()
                .copied()
                .map(f64::from)
                .enumerate()
                .take(n_in)
                .map(|(j, jtem)| jtem * (pi_2n * ((2 * i + 1 + n_in) * (2 * j + 1)) as f64).cos())
                .sum();

            *item = (scale * accum) as f32;
        }
    }

    #[test]
    fn verify_imdct() {
        #[rustfmt::skip]
        const TEST_VECTOR: [f32; 32] = [
             1.0,  2.0,  3.0,  4.0,  5.0,  6.0,  7.0,  8.0,
             9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0,
            25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0,
        ];

        let mut actual = [0f32; 64];
        let mut expected = [0f32; 64];

        let scale = (2.0f64 / 64.0).sqrt();

        imdct_analytical(&TEST_VECTOR, &mut expected, scale);

        let mut mdct = Imdct::new_scaled(32, scale);
        mdct.imdct(&TEST_VECTOR, &mut actual);

        for i in 0..64 {
            let delta = f64::from(actual[i]) - f64::from(expected[i]);
            assert!(delta.abs() < 0.00001);
        }
    }
}
