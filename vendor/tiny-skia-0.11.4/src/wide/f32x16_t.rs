// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use super::{f32x8, u16x16};

#[derive(Copy, Clone, Debug)]
#[repr(C, align(32))]
pub struct f32x16(pub f32x8, pub f32x8);

unsafe impl bytemuck::Zeroable for f32x16 {}
unsafe impl bytemuck::Pod for f32x16 {}

impl Default for f32x16 {
    fn default() -> Self {
        Self::splat(0.0)
    }
}

impl f32x16 {
    pub fn splat(n: f32) -> Self {
        Self(f32x8::splat(n), f32x8::splat(n))
    }

    #[inline]
    pub fn abs(&self) -> Self {
        // Yes, Skia does it in the same way.
        let abs = |x| bytemuck::cast::<i32, f32>(bytemuck::cast::<f32, i32>(x) & 0x7fffffff);

        let n0: [f32; 8] = self.0.into();
        let n1: [f32; 8] = self.1.into();
        Self(
            f32x8::from([
                abs(n0[0]),
                abs(n0[1]),
                abs(n0[2]),
                abs(n0[3]),
                abs(n0[4]),
                abs(n0[5]),
                abs(n0[6]),
                abs(n0[7]),
            ]),
            f32x8::from([
                abs(n1[0]),
                abs(n1[1]),
                abs(n1[2]),
                abs(n1[3]),
                abs(n1[4]),
                abs(n1[5]),
                abs(n1[6]),
                abs(n1[7]),
            ]),
        )
    }

    pub fn cmp_gt(self, rhs: &Self) -> Self {
        Self(self.0.cmp_gt(rhs.0), self.1.cmp_gt(rhs.1))
    }

    pub fn blend(self, t: Self, f: Self) -> Self {
        Self(self.0.blend(t.0, f.0), self.1.blend(t.1, f.1))
    }

    pub fn normalize(&self) -> Self {
        Self(self.0.normalize(), self.1.normalize())
    }

    pub fn floor(&self) -> Self {
        // Yes, Skia does it in the same way.
        let roundtrip = self.round();
        roundtrip
            - roundtrip
                .cmp_gt(self)
                .blend(f32x16::splat(1.0), f32x16::splat(0.0))
    }

    pub fn sqrt(&self) -> Self {
        Self(self.0.sqrt(), self.1.sqrt())
    }

    pub fn round(&self) -> Self {
        Self(self.0.round(), self.1.round())
    }

    // This method is too heavy and shouldn't be inlined.
    pub fn save_to_u16x16(&self, dst: &mut u16x16) {
        // Do not use to_i32x8, because it involves rounding,
        // and Skia cast's without it.

        let n0: [f32; 8] = self.0.into();
        let n1: [f32; 8] = self.1.into();

        dst.0[0] = n0[0] as u16;
        dst.0[1] = n0[1] as u16;
        dst.0[2] = n0[2] as u16;
        dst.0[3] = n0[3] as u16;

        dst.0[4] = n0[4] as u16;
        dst.0[5] = n0[5] as u16;
        dst.0[6] = n0[6] as u16;
        dst.0[7] = n0[7] as u16;

        dst.0[8] = n1[0] as u16;
        dst.0[9] = n1[1] as u16;
        dst.0[10] = n1[2] as u16;
        dst.0[11] = n1[3] as u16;

        dst.0[12] = n1[4] as u16;
        dst.0[13] = n1[5] as u16;
        dst.0[14] = n1[6] as u16;
        dst.0[15] = n1[7] as u16;
    }
}

impl core::ops::Add<f32x16> for f32x16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl core::ops::Sub<f32x16> for f32x16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl core::ops::Mul<f32x16> for f32x16 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}
