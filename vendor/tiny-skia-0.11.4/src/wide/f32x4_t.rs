// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Based on https://github.com/Lokathor/wide (Zlib)

use bytemuck::cast;

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use tiny_skia_path::NoStdFloat;

use super::i32x4;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(16))]
        pub struct f32x4(__m128);
    } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
        use core::arch::wasm32::*;

        // repr(transparent) allows for directly passing the v128 on the WASM stack.
        #[derive(Clone, Copy, Debug)]
        #[repr(transparent)]
        pub struct f32x4(v128);
    } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
        use core::arch::aarch64::*;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(16))]
        pub struct f32x4(float32x4_t);
    } else {
        use super::FasterMinMax;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(16))]
        pub struct f32x4([f32; 4]);
    }
}

unsafe impl bytemuck::Zeroable for f32x4 {}
unsafe impl bytemuck::Pod for f32x4 {}

impl Default for f32x4 {
    fn default() -> Self {
        Self::splat(0.0)
    }
}

impl f32x4 {
    pub fn splat(n: f32) -> Self {
        Self::from([n, n, n, n])
    }

    pub fn floor(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_floor(self.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vrndmq_f32(self.0) })
            } else {
                let roundtrip: f32x4 = cast(self.trunc_int().to_f32x4());
                roundtrip - roundtrip.cmp_gt(self).blend(f32x4::splat(1.0), f32x4::default())
            }
        }
    }

    pub fn abs(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_abs(self.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vabsq_f32(self.0) })
            } else {
                let non_sign_bits = f32x4::splat(f32::from_bits(i32::MAX as u32));
                self & non_sign_bits
            }
        }
    }

    pub fn max(self, rhs: Self) -> Self {
        // These technically don't have the same semantics for NaN and 0, but it
        // doesn't seem to matter as Skia does it the same way.
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_max_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_pmax(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vmaxq_f32(self.0, rhs.0) })
            } else {
                Self([
                    self.0[0].faster_max(rhs.0[0]),
                    self.0[1].faster_max(rhs.0[1]),
                    self.0[2].faster_max(rhs.0[2]),
                    self.0[3].faster_max(rhs.0[3]),
                ])
            }
        }
    }

    pub fn min(self, rhs: Self) -> Self {
        // These technically don't have the same semantics for NaN and 0, but it
        // doesn't seem to matter as Skia does it the same way.
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_min_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_pmin(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vminq_f32(self.0, rhs.0) })
            } else {
                Self([
                    self.0[0].faster_min(rhs.0[0]),
                    self.0[1].faster_min(rhs.0[1]),
                    self.0[2].faster_min(rhs.0[2]),
                    self.0[3].faster_min(rhs.0[3]),
                ])
            }
        }
    }

    pub fn cmp_eq(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_cmpeq_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_eq(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { vceqq_f32(self.0, rhs.0) }))
            } else {
                Self([
                    if self.0[0] == rhs.0[0] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[1] == rhs.0[1] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[2] == rhs.0[2] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[3] == rhs.0[3] { f32::from_bits(u32::MAX) } else { 0.0 },
                ])
            }
        }
    }

    pub fn cmp_ne(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_cmpneq_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_ne(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { vmvnq_u32(vceqq_f32(self.0, rhs.0)) }))
            } else {
                Self([
                    if self.0[0] != rhs.0[0] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[1] != rhs.0[1] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[2] != rhs.0[2] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[3] != rhs.0[3] { f32::from_bits(u32::MAX) } else { 0.0 },
                ])
            }
        }
    }

    pub fn cmp_ge(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_cmpge_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_ge(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { vcgeq_f32(self.0, rhs.0) }))
            } else {
                Self([
                    if self.0[0] >= rhs.0[0] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[1] >= rhs.0[1] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[2] >= rhs.0[2] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[3] >= rhs.0[3] { f32::from_bits(u32::MAX) } else { 0.0 },
                ])
            }
        }
    }

    pub fn cmp_gt(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_cmpgt_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_gt(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { vcgtq_f32(self.0, rhs.0) }))
            } else {
                Self([
                    if self.0[0] > rhs.0[0] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[1] > rhs.0[1] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[2] > rhs.0[2] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[3] > rhs.0[3] { f32::from_bits(u32::MAX) } else { 0.0 },
                ])
            }
        }
    }

    pub fn cmp_le(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_cmple_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_le(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { vcleq_f32(self.0, rhs.0) }))
            } else {
                Self([
                    if self.0[0] <= rhs.0[0] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[1] <= rhs.0[1] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[2] <= rhs.0[2] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[3] <= rhs.0[3] { f32::from_bits(u32::MAX) } else { 0.0 },
                ])
            }
        }
    }

    pub fn cmp_lt(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_cmplt_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_lt(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { vcltq_f32(self.0, rhs.0) }))
            } else {
                Self([
                    if self.0[0] < rhs.0[0] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[1] < rhs.0[1] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[2] < rhs.0[2] { f32::from_bits(u32::MAX) } else { 0.0 },
                    if self.0[3] < rhs.0[3] { f32::from_bits(u32::MAX) } else { 0.0 },
                ])
            }
        }
    }

    #[inline]
    pub fn blend(self, t: Self, f: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse4.1"))] {
                Self(unsafe { _mm_blendv_ps(f.0, t.0, self.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(v128_bitselect(t.0, f.0, self.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { cast(vbslq_u32( cast(self.0), cast(t.0), cast(f.0))) })
            } else {
                super::generic_bit_blend(self, t, f)
            }
        }
    }

    pub fn round(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse4.1"))] {
                Self(
                    unsafe { _mm_round_ps(self.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEAREST_INT) },
                )
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_nearest(self.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vrndnq_f32(self.0) })
            } else {
                use super::u32x4;

                let to_int = f32x4::splat(1.0 / f32::EPSILON);
                let u: u32x4 = cast(self);
                let e: i32x4 = cast(u.shr::<23>() & u32x4::splat(0xff));
                let mut y: f32x4;

                let no_op_magic = i32x4::splat(0x7f + 23);
                let no_op_mask: f32x4 = cast(e.cmp_gt(no_op_magic) | e.cmp_eq(no_op_magic));
                let no_op_val: f32x4 = self;

                let zero_magic = i32x4::splat(0x7f - 1);
                let zero_mask: f32x4 = cast(e.cmp_lt(zero_magic));
                let zero_val: f32x4 = self * f32x4::splat(0.0);

                let neg_bit: f32x4 = cast(cast::<u32x4, i32x4>(u).cmp_lt(i32x4::default()));
                let x: f32x4 = neg_bit.blend(-self, self);
                y = x + to_int - to_int - x;
                y = y.cmp_gt(f32x4::splat(0.5)).blend(
                    y + x - f32x4::splat(-1.0),
                    y.cmp_lt(f32x4::splat(-0.5)).blend(y + x + f32x4::splat(1.0), y + x),
                );
                y = neg_bit.blend(-y, y);

                no_op_mask.blend(no_op_val, zero_mask.blend(zero_val, y))
            }
        }
    }

    pub fn round_int(self) -> i32x4 {
        // These technically don't have the same semantics for NaN and out of
        // range values, but it doesn't seem to matter as Skia does it the same
        // way.
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                i32x4(unsafe { _mm_cvtps_epi32(self.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                i32x4(i32x4_trunc_sat_f32x4(self.round().0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                i32x4(unsafe { vcvtnq_s32_f32(self.0) } )
            } else {
                let rounded: [f32; 4] = cast(self.round());
                cast([
                    rounded[0] as i32,
                    rounded[1] as i32,
                    rounded[2] as i32,
                    rounded[3] as i32,
                ])
            }
        }
    }

    pub fn trunc_int(self) -> i32x4 {
        // These technically don't have the same semantics for NaN and out of
        // range values, but it doesn't seem to matter as Skia does it the same
        // way.
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                i32x4(unsafe { _mm_cvttps_epi32(self.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                i32x4(i32x4_trunc_sat_f32x4(self.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                i32x4(unsafe { vcvtq_s32_f32(self.0) })
            } else {
                cast([
                    self.0[0] as i32,
                    self.0[1] as i32,
                    self.0[2] as i32,
                    self.0[3] as i32,
                ])
            }
        }
    }

    pub fn recip_fast(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_rcp_ps(self.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_div(f32x4_splat(1.0), self.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                unsafe {
                    let a = vrecpeq_f32(self.0);
                    let a = vmulq_f32(vrecpsq_f32(self.0, a), a);
                    Self(a)
                }
            } else {
                Self::from([
                    1.0 / self.0[0],
                    1.0 / self.0[1],
                    1.0 / self.0[2],
                    1.0 / self.0[3],
                ])
            }
        }
    }

    pub fn recip_sqrt(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_rsqrt_ps(self.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_div(f32x4_splat(1.0), f32x4_sqrt(self.0)))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                unsafe {
                    let a = vrsqrteq_f32(self.0);
                    let a = vmulq_f32(vrsqrtsq_f32(self.0, vmulq_f32(a, a)), a);
                    Self(a)
                }
            } else {
                Self::from([
                    1.0 / self.0[0].sqrt(),
                    1.0 / self.0[1].sqrt(),
                    1.0 / self.0[2].sqrt(),
                    1.0 / self.0[3].sqrt(),
                ])
            }
        }
    }

    pub fn sqrt(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_sqrt_ps(self.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_sqrt(self.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vsqrtq_f32(self.0) })
            } else {
                Self::from([
                    self.0[0].sqrt(),
                    self.0[1].sqrt(),
                    self.0[2].sqrt(),
                    self.0[3].sqrt(),
                ])
            }
        }
    }
}

impl From<[f32; 4]> for f32x4 {
    fn from(v: [f32; 4]) -> Self {
        cast(v)
    }
}

impl From<f32x4> for [f32; 4] {
    fn from(v: f32x4) -> Self {
        cast(v)
    }
}

impl core::ops::Add for f32x4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_add_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_add(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vaddq_f32(self.0, rhs.0) })
            } else {
                Self([
                    self.0[0] + rhs.0[0],
                    self.0[1] + rhs.0[1],
                    self.0[2] + rhs.0[2],
                    self.0[3] + rhs.0[3],
                ])
            }
        }
    }
}

impl core::ops::AddAssign for f32x4 {
    fn add_assign(&mut self, rhs: f32x4) {
        *self = *self + rhs;
    }
}

impl core::ops::Sub for f32x4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_sub_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_sub(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vsubq_f32(self.0, rhs.0) })
            } else {
                Self([
                    self.0[0] - rhs.0[0],
                    self.0[1] - rhs.0[1],
                    self.0[2] - rhs.0[2],
                    self.0[3] - rhs.0[3],
                ])
            }
        }
    }
}

impl core::ops::Mul for f32x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_mul_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_mul(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vmulq_f32(self.0, rhs.0) })
            } else {
                Self([
                    self.0[0] * rhs.0[0],
                    self.0[1] * rhs.0[1],
                    self.0[2] * rhs.0[2],
                    self.0[3] * rhs.0[3],
                ])
            }
        }
    }
}

impl core::ops::MulAssign for f32x4 {
    fn mul_assign(&mut self, rhs: f32x4) {
        *self = *self * rhs;
    }
}

impl core::ops::Div for f32x4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_div_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(f32x4_div(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vdivq_f32(self.0, rhs.0) })
            } else {
                Self([
                    self.0[0] / rhs.0[0],
                    self.0[1] / rhs.0[1],
                    self.0[2] / rhs.0[2],
                    self.0[3] / rhs.0[3],
                ])
            }
        }
    }
}

impl core::ops::BitAnd for f32x4 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_and_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(v128_and(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { vandq_u32(cast(self.0), cast(rhs.0)) }))
            } else {
                Self([
                    f32::from_bits(self.0[0].to_bits() & rhs.0[0].to_bits()),
                    f32::from_bits(self.0[1].to_bits() & rhs.0[1].to_bits()),
                    f32::from_bits(self.0[2].to_bits() & rhs.0[2].to_bits()),
                    f32::from_bits(self.0[3].to_bits() & rhs.0[3].to_bits()),
                ])
            }
        }
    }
}

impl core::ops::BitOr for f32x4 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_or_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(v128_or(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { vorrq_u32(cast(self.0), cast(rhs.0)) }))
            } else {
                Self([
                    f32::from_bits(self.0[0].to_bits() | rhs.0[0].to_bits()),
                    f32::from_bits(self.0[1].to_bits() | rhs.0[1].to_bits()),
                    f32::from_bits(self.0[2].to_bits() | rhs.0[2].to_bits()),
                    f32::from_bits(self.0[3].to_bits() | rhs.0[3].to_bits()),
                ])
            }
        }
    }
}

impl core::ops::BitXor for f32x4 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_xor_ps(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(v128_xor(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { veorq_u32(cast(self.0), cast(rhs.0)) }))
            } else {
                Self([
                    f32::from_bits(self.0[0].to_bits() ^ rhs.0[0].to_bits()),
                    f32::from_bits(self.0[1].to_bits() ^ rhs.0[1].to_bits()),
                    f32::from_bits(self.0[2].to_bits() ^ rhs.0[2].to_bits()),
                    f32::from_bits(self.0[3].to_bits() ^ rhs.0[3].to_bits()),
                ])
            }
        }
    }
}

impl core::ops::Neg for f32x4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::default() - self
    }
}

impl core::ops::Not for f32x4 {
    type Output = Self;

    fn not(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                unsafe {
                    let all_bits = _mm_set1_ps(f32::from_bits(u32::MAX));
                    Self(_mm_xor_ps(self.0, all_bits))
                }
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(v128_not(self.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(cast(unsafe { vmvnq_u32(cast(self.0)) }))
            } else {
                self ^ Self::splat(cast(u32::MAX))
            }
        }
    }
}

impl core::cmp::PartialEq for f32x4 {
    fn eq(&self, rhs: &Self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                unsafe { _mm_movemask_ps(_mm_cmpeq_ps(self.0, rhs.0)) == 0b1111 }
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                unsafe { vminvq_u32(vceqq_f32(self.0, rhs.0)) != 0 }
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                u32x4_all_true(f32x4_eq(self.0, rhs.0))
            } else {
                self.0 == rhs.0
            }
        }
    }
}
