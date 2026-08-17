// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `fft` module implements the Fast Fourier Transform (FFT).
//!
//! The complex (I)FFT in this module supports a size up-to 65536. The FFT is implemented using the
//! radix-2 Cooley-Tukey algorithm.

use std::convert::TryInto;
use std::f32;

use lazy_static::lazy_static;

use super::complex::Complex;

macro_rules! fft_twiddle_table {
    ($bi:expr, $name:ident) => {
        lazy_static! {
            static ref $name: [Complex; (1 << $bi) >> 1] = {
                const N: usize = 1 << $bi;

                let mut table = [Default::default(); N >> 1];

                let theta = std::f64::consts::PI / (N >> 1) as f64;

                for (k, t) in table.iter_mut().enumerate() {
                    let angle = theta * k as f64;
                    *t = Complex::new(angle.cos() as f32, -angle.sin() as f32);
                }

                table
            };
        }
    };
}

fft_twiddle_table!(6, FFT_TWIDDLE_TABLE_64);
fft_twiddle_table!(7, FFT_TWIDDLE_TABLE_128);
fft_twiddle_table!(8, FFT_TWIDDLE_TABLE_256);
fft_twiddle_table!(9, FFT_TWIDDLE_TABLE_512);
fft_twiddle_table!(10, FFT_TWIDDLE_TABLE_1024);
fft_twiddle_table!(11, FFT_TWIDDLE_TABLE_2048);
fft_twiddle_table!(12, FFT_TWIDDLE_TABLE_4096);
fft_twiddle_table!(13, FFT_TWIDDLE_TABLE_8192);
fft_twiddle_table!(14, FFT_TWIDDLE_TABLE_16384);
fft_twiddle_table!(15, FFT_TWIDDLE_TABLE_32768);
fft_twiddle_table!(16, FFT_TWIDDLE_TABLE_65536);

/// Get the twiddle factors for a FFT of size `n`.
fn fft_twiddle_factors(n: usize) -> &'static [Complex] {
    // FFT sizes <= 32 use unrolled FFT implementations with hard-coded twiddle factors.
    match n {
        64 => FFT_TWIDDLE_TABLE_64.as_ref(),
        128 => FFT_TWIDDLE_TABLE_128.as_ref(),
        256 => FFT_TWIDDLE_TABLE_256.as_ref(),
        512 => FFT_TWIDDLE_TABLE_512.as_ref(),
        1024 => FFT_TWIDDLE_TABLE_1024.as_ref(),
        2048 => FFT_TWIDDLE_TABLE_2048.as_ref(),
        4096 => FFT_TWIDDLE_TABLE_4096.as_ref(),
        8192 => FFT_TWIDDLE_TABLE_8192.as_ref(),
        16384 => FFT_TWIDDLE_TABLE_16384.as_ref(),
        32768 => FFT_TWIDDLE_TABLE_32768.as_ref(),
        65536 => FFT_TWIDDLE_TABLE_65536.as_ref(),
        _ => panic!("fft size too large"),
    }
}

/// The complex Fast Fourier Transform (FFT).
pub struct Fft {
    perm: Box<[u16]>,
}

impl Fft {
    /// The maximum FFT size.
    pub const MAX_SIZE: usize = 1 << 16;

    pub fn new(n: usize) -> Self {
        // The FFT size must be a power of two.
        assert!(n.is_power_of_two());
        // The permutation table uses 16-bit indicies. Therefore, the absolute maximum FFT size is
        // limited to 2^16.
        assert!(n <= Fft::MAX_SIZE);

        // Calculate the bit reversal table.
        let n = n as u16;
        let shift = n.leading_zeros() + 1;
        let perm = (0..n).map(|i| i.reverse_bits() >> shift).collect();

        Self { perm }
    }

    /// Get the size of the FFT.
    pub fn size(&self) -> usize {
        self.perm.len()
    }

    /// Calculate the inverse FFT.
    pub fn ifft(&self, x: &[Complex], y: &mut [Complex]) {
        let n = x.len();
        assert_eq!(n, y.len());
        assert_eq!(n, self.perm.len());

        // Bit reversal using pre-computed permutation table.
        for (x, y) in self.perm.iter().map(|&i| x[usize::from(i)]).zip(y.iter_mut()) {
            *y = Complex { re: x.im, im: x.re };
        }

        // Do the forward FFT.
        Self::transform(y, n);

        // Output scale.
        let c = 1.0 / n as f32;

        for y in y.iter_mut() {
            *y = Complex { re: c * y.im, im: c * y.re };
        }
    }

    /// Calculate the inverse FFT in-place.
    pub fn ifft_inplace(&self, x: &mut [Complex]) {
        let n = x.len();
        assert_eq!(n, self.perm.len());

        // Bit reversal using pre-computed permutation table.
        for (i, &j) in self.perm.iter().enumerate() {
            let j = usize::from(j);

            if i <= j {
                // Swap real and imaginary components while swapping for bit-reversal.
                let xi = x[i];
                let xj = x[j];
                x[i] = Complex::new(xj.im, xj.re);
                x[j] = Complex::new(xi.im, xi.re);
            }
        }

        // Do the forward FFT.
        Self::transform(x, n);

        // Output scale.
        let c = 1.0 / n as f32;

        for x in x.iter_mut() {
            *x = Complex { re: c * x.im, im: c * x.re };
        }
    }

    /// Calculate the FFT in-place.
    pub fn fft_inplace(&self, x: &mut [Complex]) {
        let n = x.len();
        assert_eq!(n, x.len());
        assert_eq!(n, self.perm.len());

        for (i, &j) in self.perm.iter().enumerate() {
            let j = usize::from(j);

            if i < j {
                x.swap(i, j);
            }
        }

        // Start FFT recursion.
        match n {
            1 => (),
            2 => fft2(x.try_into().unwrap()),
            4 => fft4(x.try_into().unwrap()),
            8 => fft8(x.try_into().unwrap()),
            16 => fft16(x.try_into().unwrap()),
            _ => Self::transform(x, n),
        }
    }

    /// Calculate the FFT.
    pub fn fft(&self, x: &[Complex], y: &mut [Complex]) {
        let n = x.len();
        assert_eq!(n, y.len());
        assert_eq!(n, self.perm.len());

        // Bit reversal using pre-computed permutation table.
        for (x, y) in self.perm.iter().map(|&i| x[usize::from(i)]).zip(y.iter_mut()) {
            *y = x;
        }

        // Start FFT recursion.
        match n {
            1 => (),
            2 => fft2(y.try_into().unwrap()),
            4 => fft4(y.try_into().unwrap()),
            8 => fft8(y.try_into().unwrap()),
            16 => fft16(y.try_into().unwrap()),
            _ => Self::transform(y, n),
        }
    }

    fn transform(x: &mut [Complex], n: usize) {
        fn to_arr(x: &mut [Complex]) -> Option<&mut [Complex; 32]> {
            x.try_into().ok()
        }

        if let Some(x) = to_arr(x) {
            fft32(x);
        }
        else {
            let n_half = n >> 1;

            let (even, odd) = x.split_at_mut(n_half);

            Self::transform(even, n_half);
            Self::transform(odd, n_half);

            let twiddle = fft_twiddle_factors(n);

            for ((e, o), w) in
                even.chunks_exact_mut(2).zip(odd.chunks_exact_mut(2)).zip(twiddle.chunks_exact(2))
            {
                let p0 = e[0];
                let q0 = o[0] * w[0];

                e[0] = p0 + q0;
                o[0] = p0 - q0;

                let p1 = e[1];
                let q1 = o[1] * w[1];

                e[1] = p1 + q1;
                o[1] = p1 - q1;
            }
        }
    }
}

macro_rules! complex {
    ($re:expr, $im:expr) => {
        Complex { re: $re, im: $im }
    };
}

fn fft32(x: &mut [Complex; 32]) {
    let mut x0 = [
        x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7], x[8], x[9], x[10], x[11], x[12], x[13],
        x[14], x[15],
    ];
    let mut x1 = [
        x[16], x[17], x[18], x[19], x[20], x[21], x[22], x[23], x[24], x[25], x[26], x[27], x[28],
        x[29], x[30], x[31],
    ];

    fft16(&mut x0);
    fft16(&mut x1);

    let a4 = f32::consts::FRAC_1_SQRT_2 * x1[4].re;
    let b4 = f32::consts::FRAC_1_SQRT_2 * x1[4].im;
    let a12 = -f32::consts::FRAC_1_SQRT_2 * x1[12].re;
    let b12 = -f32::consts::FRAC_1_SQRT_2 * x1[12].im;

    let x1p = [
        x1[0],
        complex!(0.98078528040323044913, -0.19509032201612826785) * x1[1],
        complex!(0.92387953251128675613, -0.38268343236508977173) * x1[2],
        complex!(0.83146961230254523708, -0.55557023301960222474) * x1[3],
        complex!(a4 + b4, b4 - a4),
        complex!(0.55557023301960222474, -0.83146961230254523708) * x1[5],
        complex!(0.38268343236508977173, -0.92387953251128675613) * x1[6],
        complex!(0.19509032201612826785, -0.98078528040323044913) * x1[7],
        complex!(x1[8].im, -x1[8].re),
        complex!(-0.19509032201612826785, -0.98078528040323044913) * x1[9],
        complex!(-0.38268343236508977173, -0.92387953251128675613) * x1[10],
        complex!(-0.55557023301960222474, -0.83146961230254523708) * x1[11],
        complex!(a12 - b12, a12 + b12),
        complex!(-0.83146961230254523708, -0.55557023301960222474) * x1[13],
        complex!(-0.92387953251128675613, -0.38268343236508977173) * x1[14],
        complex!(-0.98078528040323044913, -0.19509032201612826785) * x1[15],
    ];

    x[0] = x0[0] + x1p[0];
    x[1] = x0[1] + x1p[1];
    x[2] = x0[2] + x1p[2];
    x[3] = x0[3] + x1p[3];
    x[4] = x0[4] + x1p[4];
    x[5] = x0[5] + x1p[5];
    x[6] = x0[6] + x1p[6];
    x[7] = x0[7] + x1p[7];
    x[8] = x0[8] + x1p[8];
    x[9] = x0[9] + x1p[9];
    x[10] = x0[10] + x1p[10];
    x[11] = x0[11] + x1p[11];
    x[12] = x0[12] + x1p[12];
    x[13] = x0[13] + x1p[13];
    x[14] = x0[14] + x1p[14];
    x[15] = x0[15] + x1p[15];

    x[16] = x0[0] - x1p[0];
    x[17] = x0[1] - x1p[1];
    x[18] = x0[2] - x1p[2];
    x[19] = x0[3] - x1p[3];
    x[20] = x0[4] - x1p[4];
    x[21] = x0[5] - x1p[5];
    x[22] = x0[6] - x1p[6];
    x[23] = x0[7] - x1p[7];
    x[24] = x0[8] - x1p[8];
    x[25] = x0[9] - x1p[9];
    x[26] = x0[10] - x1p[10];
    x[27] = x0[11] - x1p[11];
    x[28] = x0[12] - x1p[12];
    x[29] = x0[13] - x1p[13];
    x[30] = x0[14] - x1p[14];
    x[31] = x0[15] - x1p[15];
}

#[inline(always)]
fn fft16(x: &mut [Complex; 16]) {
    let mut x0 = [x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]];
    let mut x1 = [x[8], x[9], x[10], x[11], x[12], x[13], x[14], x[15]];

    fft8(&mut x0);
    fft8(&mut x1);

    let a2 = f32::consts::FRAC_1_SQRT_2 * x1[2].re;
    let b2 = f32::consts::FRAC_1_SQRT_2 * x1[2].im;
    let a6 = -f32::consts::FRAC_1_SQRT_2 * x1[6].re;
    let b6 = -f32::consts::FRAC_1_SQRT_2 * x1[6].im;

    let x1p = [
        x1[0],
        complex!(0.92387953251128675613, -0.38268343236508977173) * x1[1],
        complex!(a2 + b2, b2 - a2),
        complex!(0.38268343236508977173, -0.92387953251128675613) * x1[3],
        complex!(x1[4].im, -x1[4].re),
        complex!(-0.38268343236508977173, -0.92387953251128675613) * x1[5],
        complex!(a6 - b6, a6 + b6),
        complex!(-0.92387953251128675613, -0.38268343236508977173) * x1[7],
    ];

    x[0] = x0[0] + x1p[0];
    x[1] = x0[1] + x1p[1];
    x[2] = x0[2] + x1p[2];
    x[3] = x0[3] + x1p[3];
    x[4] = x0[4] + x1p[4];
    x[5] = x0[5] + x1p[5];
    x[6] = x0[6] + x1p[6];
    x[7] = x0[7] + x1p[7];

    x[8] = x0[0] - x1p[0];
    x[9] = x0[1] - x1p[1];
    x[10] = x0[2] - x1p[2];
    x[11] = x0[3] - x1p[3];
    x[12] = x0[4] - x1p[4];
    x[13] = x0[5] - x1p[5];
    x[14] = x0[6] - x1p[6];
    x[15] = x0[7] - x1p[7];
}

#[inline(always)]
fn fft8(x: &mut [Complex; 8]) {
    let mut x0 = [x[0], x[1], x[2], x[3]];
    let mut x1 = [x[4], x[5], x[6], x[7]];

    fft4(&mut x0);
    fft4(&mut x1);

    let a1 = f32::consts::FRAC_1_SQRT_2 * x1[1].re;
    let b1 = f32::consts::FRAC_1_SQRT_2 * x1[1].im;
    let a3 = -f32::consts::FRAC_1_SQRT_2 * x1[3].re;
    let b3 = -f32::consts::FRAC_1_SQRT_2 * x1[3].im;

    let x1p = [
        x1[0],
        complex!(a1 + b1, b1 - a1),
        complex!(x1[2].im, -x1[2].re),
        complex!(a3 - b3, a3 + b3),
    ];

    x[0] = x0[0] + x1p[0];
    x[1] = x0[1] + x1p[1];
    x[2] = x0[2] + x1p[2];
    x[3] = x0[3] + x1p[3];

    x[4] = x0[0] - x1p[0];
    x[5] = x0[1] - x1p[1];
    x[6] = x0[2] - x1p[2];
    x[7] = x0[3] - x1p[3];
}

#[inline(always)]
fn fft4(x: &mut [Complex; 4]) {
    let x0 = [x[0] + x[1], x[0] - x[1]];
    let x1 = [x[2] + x[3], x[2] - x[3]];

    let x1p1 = complex!(x1[1].im, -x1[1].re);

    x[0] = x0[0] + x1[0];
    x[1] = x0[1] + x1p1;

    x[2] = x0[0] - x1[0];
    x[3] = x0[1] - x1p1;
}

#[inline(always)]
fn fft2(x: &mut [Complex; 2]) {
    let x0 = x[0];
    x[0] = x0 + x[1];
    x[1] = x0 - x[1];
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    /// Compute a naive DFT.
    fn dft_naive(x: &[Complex], y: &mut [Complex]) {
        assert_eq!(x.len(), y.len());

        let n = x.len() as u64;

        let theta = 2.0 * f64::consts::PI / (x.len() as f64);

        for (i, y) in y.iter_mut().enumerate() {
            let mut re = 0f64;
            let mut im = 0f64;

            for (j, &x) in x.iter().enumerate() {
                let xre = f64::from(x.re);
                let xim = f64::from(x.im);

                let ij = ((i as u64) * (j as u64)) & (n - 1);

                let wre = (theta * ij as f64).cos();
                let wim = -(theta * ij as f64).sin();

                re += (xre * wre) - (xim * wim);
                im += (xre * wim) + (xim * wre);
            }

            *y = Complex { re: re as f32, im: im as f32 };
        }
    }

    /// Compute a naive IDFT.
    fn idft_naive(x: &[Complex], y: &mut [Complex]) {
        let n = x.len() as u64;

        let z = x.iter().map(|x| Complex { re: x.im, im: x.re }).collect::<Vec<Complex>>();

        dft_naive(&z, y);

        let c = 1.0 / n as f32;

        for y in y.iter_mut() {
            *y = Complex { re: c * y.im, im: c * y.re };
        }
    }

    /// Returns true if both real and imaginary complex number components deviate by less than
    /// `epsilon` between the left-hand side and right-hand side.
    fn check_complex(lhs: Complex, rhs: Complex, epsilon: f32) -> bool {
        (lhs.re - rhs.re).abs() < epsilon && (lhs.im - rhs.im).abs() < epsilon
    }

    #[rustfmt::skip]
    const TEST_VECTOR: [Complex; 64] = [
        Complex { re: -1.82036, im: -0.72591 },
        Complex { re: 1.21002, im: 0.75897 },
        Complex { re: 1.31084, im: -0.51285 },
        Complex { re: 1.26443, im: 1.57430 },
        Complex { re: -1.93680, im: 0.69987 },
        Complex { re: 0.85269, im: -0.20148 },
        Complex { re: 1.10078, im: 0.88904 },
        Complex { re: -1.20634, im: -0.07612 },
        Complex { re: 1.43358, im: -1.91248 },
        Complex { re: 0.10594, im: -0.30743 },
        Complex { re: 1.51258, im: 0.99538 },
        Complex { re: -1.33673, im: 0.23797 },
        Complex { re: 0.43738, im: -1.69900 },
        Complex { re: -0.95355, im: -0.33534 },
        Complex { re: -0.05479, im: -0.32739 },
        Complex { re: -1.85529, im: -1.93157 },
        Complex { re: -1.04220, im: 1.04277 },
        Complex { re: -0.17585, im: 0.40640 },
        Complex { re: 0.09893, im: 1.89538 },
        Complex { re: 1.25018, im: -0.85052 },
        Complex { re: -1.60696, im: -1.41320 },
        Complex { re: -0.25171, im: -0.13830 },
        Complex { re: 1.17782, im: -1.41225 },
        Complex { re: -0.35389, im: -0.30323 },
        Complex { re: -0.16485, im: -0.83675 },
        Complex { re: -1.66729, im: -0.52132 },
        Complex { re: 1.41246, im: 1.58295 },
        Complex { re: -1.84041, im: 0.15331 },
        Complex { re: -1.38897, im: 1.16180 },
        Complex { re: 0.27927, im: -1.84254 },
        Complex { re: -0.46229, im: 0.09699 },
        Complex { re: 1.21027, im: -0.31551 },
        Complex { re: 0.26195, im: -1.19340 },
        Complex { re: 1.60673, im: 1.07094 },
        Complex { re: -0.07456, im: -0.63058 },
        Complex { re: -1.77397, im: 1.39608 },
        Complex { re: -0.80300, im: 0.08858 },
        Complex { re: -0.06289, im: 1.48840 },
        Complex { re: 0.66399, im: 0.49451 },
        Complex { re: -1.49827, im: 1.61856 },
        Complex { re: -1.39006, im: 0.67652 },
        Complex { re: -0.90232, im: 0.18255 },
        Complex { re: 0.00525, im: -1.05797 },
        Complex { re: 0.53688, im: 0.88532 },
        Complex { re: 0.52712, im: -0.58205 },
        Complex { re: -1.77624, im: -0.66799 },
        Complex { re: 1.65335, im: -1.72668 },
        Complex { re: -0.24568, im: 1.50477 },
        Complex { re: -0.15051, im: 0.67824 },
        Complex { re: -1.96744, im: 0.81734 },
        Complex { re: -1.62630, im: -0.73233 },
        Complex { re: -1.98698, im: 0.63824 },
        Complex { re: 0.78115, im: 0.97909 },
        Complex { re: 0.97392, im: 1.82166 },
        Complex { re: 1.30982, im: -1.23975 },
        Complex { re: 0.85813, im: 0.80842 },
        Complex { re: -1.13934, im: 0.81352 },
        Complex { re: -1.22092, im: 0.98348 },
        Complex { re: -1.67949, im: 0.78278 },
        Complex { re: -1.77411, im: 0.00424 },
        Complex { re: 1.93204, im: -0.03273 },
        Complex { re: 1.38529, im: 1.31798 },
        Complex { re: 0.61666, im: -0.09798 },
        Complex { re: 1.02132, im: 1.70293 },
    ];

    #[test]
    fn verify_fft() {
        let mut actual = [Default::default(); TEST_VECTOR.len()];
        let mut expected = [Default::default(); TEST_VECTOR.len()];

        // Expected
        dft_naive(&TEST_VECTOR, &mut expected);

        // Actual
        Fft::new(TEST_VECTOR.len()).fft(&TEST_VECTOR, &mut actual);

        for (&a, &e) in actual.iter().zip(expected.iter()) {
            assert!(check_complex(a, e, 0.00001));
        }
    }

    #[test]
    fn verify_fft_inplace() {
        let mut actual = [Default::default(); TEST_VECTOR.len()];
        let mut expected = [Default::default(); TEST_VECTOR.len()];

        // Expected
        dft_naive(&TEST_VECTOR, &mut expected);

        // Actual
        actual.copy_from_slice(&TEST_VECTOR);
        Fft::new(TEST_VECTOR.len()).fft_inplace(&mut actual);

        for (&a, &e) in actual.iter().zip(expected.iter()) {
            assert!(check_complex(a, e, 0.00001));
        }
    }

    #[test]
    fn verify_ifft() {
        let mut actual = [Default::default(); TEST_VECTOR.len()];
        let mut expected = [Default::default(); TEST_VECTOR.len()];

        // Expected
        idft_naive(&TEST_VECTOR, &mut expected);

        // Actual
        Fft::new(TEST_VECTOR.len()).ifft(&TEST_VECTOR, &mut actual);

        for (&a, &e) in actual.iter().zip(expected.iter()) {
            assert!(check_complex(a, e, 0.00001));
        }
    }

    #[test]
    fn verify_ifft_inplace() {
        let mut actual = [Default::default(); TEST_VECTOR.len()];
        let mut expected = [Default::default(); TEST_VECTOR.len()];

        // Expected
        idft_naive(&TEST_VECTOR, &mut expected);

        // Actual
        actual.copy_from_slice(&TEST_VECTOR);
        Fft::new(TEST_VECTOR.len()).ifft_inplace(&mut actual);

        for (&a, &e) in actual.iter().zip(expected.iter()) {
            assert!(check_complex(a, e, 0.00001));
        }
    }

    #[test]
    fn verify_fft_reversible() {
        let mut fft_out = [Default::default(); TEST_VECTOR.len()];
        let mut ifft_out = [Default::default(); TEST_VECTOR.len()];

        let fft = Fft::new(TEST_VECTOR.len());
        fft.fft(&TEST_VECTOR, &mut fft_out);
        fft.ifft(&fft_out, &mut ifft_out);

        for (&a, &e) in ifft_out.iter().zip(TEST_VECTOR.iter()) {
            assert!(check_complex(a, e, 0.000001));
        }
    }

    #[test]
    fn verify_fft_inplace_reversible() {
        let mut out = [Default::default(); TEST_VECTOR.len()];
        out.copy_from_slice(&TEST_VECTOR);

        let fft = Fft::new(TEST_VECTOR.len());
        fft.fft_inplace(&mut out);
        fft.ifft_inplace(&mut out);

        for (&a, &e) in out.iter().zip(TEST_VECTOR.iter()) {
            assert!(check_complex(a, e, 0.000001));
        }
    }
}
