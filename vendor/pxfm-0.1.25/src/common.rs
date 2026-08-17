/*
 * // Copyright (c) Radzivon Bartoshyk 4/2025. All rights reserved.
 * //
 * // Redistribution and use in source and binary forms, with or without modification,
 * // are permitted provided that the following conditions are met:
 * //
 * // 1.  Redistributions of source code must retain the above copyright notice, this
 * // list of conditions and the following disclaimer.
 * //
 * // 2.  Redistributions in binary form must reproduce the above copyright notice,
 * // this list of conditions and the following disclaimer in the documentation
 * // and/or other materials provided with the distribution.
 * //
 * // 3.  Neither the name of the copyright holder nor the names of its
 * // contributors may be used to endorse or promote products derived from
 * // this software without specific prior written permission.
 * //
 * // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * // AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * // IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * // DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * // FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * // DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * // SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * // CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * // OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * // OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use crate::bits::EXP_MASK;
use num_traits::MulAdd;
use std::ops::{Add, Mul};

#[inline]
pub(crate) fn is_integerf(x: f32) -> bool {
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1"
        ),
        target_arch = "aarch64"
    ))]
    {
        x.round_ties_even() == x
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1"
        ),
        target_arch = "aarch64"
    )))]
    {
        let x_u = x.to_bits();
        let x_e = (x_u & EXP_MASK_F32) >> 23;
        let lsb = (x_u | EXP_MASK_F32).trailing_zeros();
        const E_BIAS: u32 = (1u32 << (8 - 1u32)) - 1u32;
        const UNIT_EXPONENT: u32 = E_BIAS + 23;
        x_e + lsb >= UNIT_EXPONENT
    }
}

#[inline]
pub(crate) fn is_odd_integerf(x: f32) -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        (x as i32 & 1) != 0
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let x_u = x.to_bits();
        let x_e = (x_u & EXP_MASK_F32) >> 23;
        let lsb = (x_u | EXP_MASK_F32).trailing_zeros();
        const E_BIAS: u32 = (1u32 << (8 - 1u32)) - 1u32;

        const UNIT_EXPONENT: u32 = E_BIAS + 23;
        x_e + lsb == UNIT_EXPONENT
    }
}

#[inline]
pub(crate) fn is_integer(n: f64) -> bool {
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1"
        ),
        target_arch = "aarch64"
    ))]
    {
        n == n.round_ties_even()
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1"
        ),
        target_arch = "aarch64"
    )))]
    {
        let x_u = n.to_bits();
        let x_e = (x_u & EXP_MASK) >> 52;
        let lsb = (x_u | EXP_MASK).trailing_zeros();
        const E_BIAS: u64 = (1u64 << (11 - 1u64)) - 1u64;

        const UNIT_EXPONENT: u64 = E_BIAS + 52;
        x_e + lsb as u64 >= UNIT_EXPONENT
    }
}

#[inline]
pub(crate) fn is_odd_integer(x: f64) -> bool {
    let x_u = x.to_bits();
    let x_e = (x_u & EXP_MASK) >> 52;
    let lsb = (x_u | EXP_MASK).trailing_zeros();
    const E_BIAS: u64 = (1u64 << (11 - 1u64)) - 1u64;

    const UNIT_EXPONENT: u64 = E_BIAS + 52;
    x_e + lsb as u64 == UNIT_EXPONENT
}

#[cfg(any(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "fma"
    ),
    target_arch = "aarch64"
))]
#[inline(always)]
pub(crate) fn mlaf<T: Copy + Mul<T, Output = T> + Add<T, Output = T> + MulAdd<T, Output = T>>(
    acc: T,
    a: T,
    b: T,
) -> T {
    MulAdd::mul_add(a, b, acc)
}

#[inline(always)]
#[cfg(not(any(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "fma"
    ),
    target_arch = "aarch64"
)))]
pub(crate) fn mlaf<T: Copy + Mul<T, Output = T> + Add<T, Output = T> + MulAdd<T, Output = T>>(
    acc: T,
    a: T,
    b: T,
) -> T {
    acc + a * b
}

#[inline]
pub(crate) const fn rintfk(x: f32) -> f32 {
    (if x < 0. { x - 0.5 } else { x + 0.5 }) as i32 as f32
}

#[inline(always)]
pub(crate) const fn fmlaf(a: f32, b: f32, c: f32) -> f32 {
    c + a * b
}

#[inline(always)]
pub(crate) fn f_fmlaf(a: f32, b: f32, c: f32) -> f32 {
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    ))]
    {
        f32::mul_add(a, b, c)
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    )))]
    {
        a * b + c
    }
}

/// Optional FMA, if it is available hardware FMA will use, if not then just scalar `c + a * b`
#[inline(always)]
pub(crate) fn f_fmla(a: f64, b: f64, c: f64) -> f64 {
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    ))]
    {
        f64::mul_add(a, b, c)
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    )))]
    {
        a * b + c
    }
}

#[inline(always)]
pub(crate) const fn fmla(a: f64, b: f64, c: f64) -> f64 {
    c + a * b
}

/// Executes mandatory FMA
/// if not available will be simulated through Dekker and Veltkamp
#[inline(always)]
pub(crate) fn dd_fmla(a: f64, b: f64, c: f64) -> f64 {
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    ))]
    {
        f_fmla(a, b, c)
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    )))]
    {
        use crate::double_double::DoubleDouble;
        DoubleDouble::dd_f64_mul_add(a, b, c)
    }
}

// Executes mandatory FMA
// if not available will be simulated through dyadic float 128
#[inline(always)]
pub(crate) fn dyad_fmla(a: f64, b: f64, c: f64) -> f64 {
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    ))]
    {
        f_fmla(a, b, c)
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    )))]
    {
        use crate::dyadic_float::DyadicFloat128;
        let z = DyadicFloat128::new_from_f64(a);
        let k = DyadicFloat128::new_from_f64(b);
        let p = z * k + DyadicFloat128::new_from_f64(c);
        p.fast_as_f64()
    }
}

// Executes mandatory FMA
// if not available will be simulated through Dekker and Veltkamp
#[inline(always)]
#[allow(unused)]
pub(crate) fn dd_fmlaf(a: f32, b: f32, c: f32) -> f32 {
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    ))]
    {
        f_fmlaf(a, b, c)
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    )))]
    {
        (a as f64 * b as f64 + c as f64) as f32
    }
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn c_mlaf<T: Copy + Mul<T, Output = T> + Add<T, Output = T> + MulAdd<T, Output = T>>(
    a: T,
    b: T,
    c: T,
) -> T {
    mlaf(c, a, b)
}

/// Copies sign from `y` to `x`
#[inline]
pub const fn copysignfk(x: f32, y: f32) -> f32 {
    f32::from_bits((x.to_bits() & !(1 << 31)) ^ (y.to_bits() & (1 << 31)))
}

// #[inline]
// // Founds n in ln(𝑥)=ln(𝑎)+𝑛ln(2)
// pub(crate) const fn ilogb2kf(d: f32) -> i32 {
//     (((d.to_bits() as i32) >> 23) & 0xff) - 0x7f
// }
//
// #[inline]
// // Founds a in x=a+𝑛ln(2)
// pub(crate) const fn ldexp3kf(d: f32, n: i32) -> f32 {
//     f32::from_bits(((d.to_bits() as i32) + (n << 23)) as u32)
// }

#[inline]
pub(crate) const fn pow2if(q: i32) -> f32 {
    f32::from_bits((q.wrapping_add(0x7f) as u32) << 23)
}

/// Round towards whole integral number
#[inline]
pub(crate) const fn rintk(x: f64) -> f64 {
    (if x < 0. { x - 0.5 } else { x + 0.5 }) as i64 as f64
}

/// Computes 2^n
#[inline(always)]
pub(crate) const fn pow2i(q: i32) -> f64 {
    f64::from_bits((q.wrapping_add(0x3ff) as u64) << 52)
}

// #[inline]
// pub(crate) const fn ilogb2k(d: f64) -> i32 {
//     (((d.to_bits() >> 52) & 0x7ff) as i32) - 0x3ff
// }
//
// #[inline]
// pub(crate) const fn ldexp3k(d: f64, e: i32) -> f64 {
//     f64::from_bits(((d.to_bits() as i64) + ((e as i64) << 52)) as u64)
// }

/// Copies sign from `y` to `x`
#[inline]
pub const fn copysignk(x: f64, y: f64) -> f64 {
    f64::from_bits((x.to_bits() & !(1 << 63)) ^ (y.to_bits() & (1 << 63)))
}

#[inline]
pub(crate) const fn min_normal_f64() -> f64 {
    let exponent_bits = 1u64 << 52;
    let bits = exponent_bits;

    f64::from_bits(bits)
}

#[inline]
const fn mask_trailing_ones_u32(len: u32) -> u32 {
    if len >= 32 {
        u32::MAX // All ones if length is 64 or more
    } else {
        (1u32 << len).wrapping_sub(1)
    }
}

pub(crate) const EXP_MASK_F32: u32 = mask_trailing_ones_u32(8) << 23;

#[inline]
pub(crate) fn set_exponent_f32(x: u32, new_exp: u32) -> u32 {
    let encoded_mask = new_exp.wrapping_shl(23) & EXP_MASK_F32;
    x ^ ((x ^ encoded_mask) & EXP_MASK_F32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_integer() {
        assert_eq!(is_integer(5.), true);
        assert_eq!(is_integer(6.), true);
        assert_eq!(is_integer(6.01), false);
        assert_eq!(is_odd_integer(5.), true);
        assert_eq!(is_odd_integer(6.), false);
        assert_eq!(is_odd_integer(6.01), false);
        assert_eq!(is_integerf(5.), true);
        assert_eq!(is_integerf(6.), true);
        assert_eq!(is_integerf(6.01), false);
        assert_eq!(is_odd_integerf(5.), true);
        assert_eq!(is_odd_integerf(6.), false);
        assert_eq!(is_odd_integerf(6.01), false);
    }
}
