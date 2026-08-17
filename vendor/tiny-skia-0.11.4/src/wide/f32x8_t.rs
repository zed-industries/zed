// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Based on https://github.com/Lokathor/wide (Zlib)

use bytemuck::cast;

use super::{i32x8, u32x8};

cfg_if::cfg_if! {
    if #[cfg(all(feature = "simd", target_feature = "avx"))] {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(32))]
        pub struct f32x8(__m256);
    } else {
        use super::f32x4;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(32))]
        pub struct f32x8(pub f32x4, pub f32x4);
    }
}

unsafe impl bytemuck::Zeroable for f32x8 {}
unsafe impl bytemuck::Pod for f32x8 {}

impl Default for f32x8 {
    fn default() -> Self {
        Self::splat(0.0)
    }
}

impl f32x8 {
    pub fn splat(n: f32) -> Self {
        cast([n, n, n, n, n, n, n, n])
    }

    pub fn floor(self) -> Self {
        let roundtrip: f32x8 = cast(self.trunc_int().to_f32x8());
        roundtrip
            - roundtrip
                .cmp_gt(self)
                .blend(f32x8::splat(1.0), f32x8::default())
    }

    pub fn fract(self) -> Self {
        self - self.floor()
    }

    pub fn normalize(self) -> Self {
        self.max(f32x8::default()).min(f32x8::splat(1.0))
    }

    pub fn to_i32x8_bitcast(self) -> i32x8 {
        bytemuck::cast(self)
    }

    pub fn to_u32x8_bitcast(self) -> u32x8 {
        bytemuck::cast(self)
    }

    pub fn cmp_eq(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_cmp_ps(self.0, rhs.0, _CMP_EQ_OQ) })
            } else {
                Self(self.0.cmp_eq(rhs.0), self.1.cmp_eq(rhs.1))
            }
        }
    }

    pub fn cmp_ne(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_cmp_ps(self.0, rhs.0, _CMP_NEQ_OQ) })
            } else {
                Self(self.0.cmp_ne(rhs.0), self.1.cmp_ne(rhs.1))
            }
        }
    }

    pub fn cmp_ge(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_cmp_ps(self.0, rhs.0, _CMP_GE_OQ) })
            } else {
                Self(self.0.cmp_ge(rhs.0), self.1.cmp_ge(rhs.1))
            }
        }
    }

    pub fn cmp_gt(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_cmp_ps(self.0, rhs.0, _CMP_GT_OQ) })
            } else {
                Self(self.0.cmp_gt(rhs.0), self.1.cmp_gt(rhs.1))
            }
        }
    }

    pub fn cmp_le(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_cmp_ps(self.0, rhs.0, _CMP_LE_OQ) })
            } else {
                Self(self.0.cmp_le(rhs.0), self.1.cmp_le(rhs.1))
            }
        }
    }

    pub fn cmp_lt(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_cmp_ps(self.0, rhs.0, _CMP_LT_OQ) })
            } else {
                Self(self.0.cmp_lt(rhs.0), self.1.cmp_lt(rhs.1))
            }
        }
    }

    #[inline]
    pub fn blend(self, t: Self, f: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_blendv_ps(f.0, t.0, self.0) })
            } else {
                Self(self.0.blend(t.0, f.0), self.1.blend(t.1, f.1))
            }
        }
    }

    pub fn abs(self) -> Self {
        let non_sign_bits = f32x8::splat(f32::from_bits(i32::MAX as u32));
        self & non_sign_bits
    }

    pub fn max(self, rhs: Self) -> Self {
        // These technically don't have the same semantics for NaN and 0, but it
        // doesn't seem to matter as Skia does it the same way.
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_max_ps(self.0, rhs.0) })
            } else {
                Self(self.0.max(rhs.0), self.1.max(rhs.1))
            }
        }
    }

    pub fn min(self, rhs: Self) -> Self {
        // These technically don't have the same semantics for NaN and 0, but it
        // doesn't seem to matter as Skia does it the same way.
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_min_ps(self.0, rhs.0) })
            } else {
                Self(self.0.min(rhs.0), self.1.min(rhs.1))
            }
        }
    }

    pub fn is_finite(self) -> Self {
        let shifted_exp_mask = u32x8::splat(0xFF000000);
        let u: u32x8 = cast(self);
        let shift_u = u.shl::<1>();
        let out = !(shift_u & shifted_exp_mask).cmp_eq(shifted_exp_mask);
        cast(out)
    }

    pub fn round(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_round_ps(self.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEAREST_INT) })
            } else {
                Self(self.0.round(), self.1.round())
            }
        }
    }

    pub fn round_int(self) -> i32x8 {
        // These technically don't have the same semantics for NaN and out of
        // range values, but it doesn't seem to matter as Skia does it the same
        // way.
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                cast(unsafe { _mm256_cvtps_epi32(self.0) })
            } else {
                i32x8(self.0.round_int(), self.1.round_int())
            }
        }
    }

    pub fn trunc_int(self) -> i32x8 {
        // These technically don't have the same semantics for NaN and out of
        // range values, but it doesn't seem to matter as Skia does it the same
        // way.
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                cast(unsafe { _mm256_cvttps_epi32(self.0) })
            } else {
                i32x8(self.0.trunc_int(), self.1.trunc_int())
            }
        }
    }

    pub fn recip_fast(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_rcp_ps(self.0) })
            } else {
                Self(self.0.recip_fast(), self.1.recip_fast())
            }
        }
    }

    pub fn recip_sqrt(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_rsqrt_ps(self.0) })
            } else {
                Self(self.0.recip_sqrt(), self.1.recip_sqrt())
            }
        }
    }

    pub fn sqrt(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_sqrt_ps(self.0) })
            } else {
                Self(self.0.sqrt(), self.1.sqrt())
            }
        }
    }
}

impl From<[f32; 8]> for f32x8 {
    fn from(v: [f32; 8]) -> Self {
        cast(v)
    }
}

impl From<f32x8> for [f32; 8] {
    fn from(v: f32x8) -> Self {
        cast(v)
    }
}

impl core::ops::Add for f32x8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_add_ps(self.0, rhs.0) })
            } else {
                Self(self.0 + rhs.0, self.1 + rhs.1)
            }
        }
    }
}

impl core::ops::AddAssign for f32x8 {
    fn add_assign(&mut self, rhs: f32x8) {
        *self = *self + rhs;
    }
}

impl core::ops::Sub for f32x8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_sub_ps(self.0, rhs.0) })
            } else {
                Self(self.0 - rhs.0, self.1 - rhs.1)
            }
        }
    }
}

impl core::ops::Mul for f32x8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_mul_ps(self.0, rhs.0) })
            } else {
                Self(self.0 * rhs.0, self.1 * rhs.1)
            }
        }
    }
}

impl core::ops::MulAssign for f32x8 {
    fn mul_assign(&mut self, rhs: f32x8) {
        *self = *self * rhs;
    }
}

impl core::ops::Div for f32x8 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_div_ps(self.0, rhs.0) })
            } else {
                Self(self.0 / rhs.0, self.1 / rhs.1)
            }
        }
    }
}

impl core::ops::BitAnd for f32x8 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_and_ps(self.0, rhs.0) })
            } else {
                Self(self.0 & rhs.0, self.1 & rhs.1)
            }
        }
    }
}

impl core::ops::BitOr for f32x8 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_or_ps(self.0, rhs.0) })
            } else {
                Self(self.0 | rhs.0, self.1 | rhs.1)
            }
        }
    }
}

impl core::ops::BitXor for f32x8 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                Self(unsafe { _mm256_xor_ps(self.0, rhs.0) })
            } else {
                Self(self.0 ^ rhs.0, self.1 ^ rhs.1)
            }
        }
    }
}

impl core::ops::Neg for f32x8 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::default() - self
    }
}

impl core::ops::Not for f32x8 {
    type Output = Self;

    fn not(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                let all_bits = unsafe { _mm256_set1_ps(f32::from_bits(u32::MAX)) };
                Self(unsafe { _mm256_xor_ps(self.0, all_bits) })
            } else {
                Self(!self.0, !self.1)
            }
        }
    }
}

impl core::cmp::PartialEq for f32x8 {
    fn eq(&self, rhs: &Self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                let mask = unsafe { _mm256_cmp_ps(self.0, rhs.0, _CMP_EQ_OQ) };
                unsafe { _mm256_movemask_ps(mask) == 0b1111_1111 }
            } else {
                self.0 == rhs.0 && self.1 == rhs.1
            }
        }
    }
}
