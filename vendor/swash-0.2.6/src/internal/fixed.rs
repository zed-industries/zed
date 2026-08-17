//! Minimal fixed point math types and functions used internally.

use super::parse::FromBeData;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// Fixed point value in 16.16 format.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct Fixed(pub i32);

impl Fixed {
    pub const MIN: Self = Self(0x80000000u32 as i32);
    pub const MAX: Self = Self(0x7FFFFFFF);
    pub const EPSILON: Self = Self(1);
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(0x10000);

    pub const fn from_i32(x: i32) -> Self {
        Self(x << 16)
    }

    pub fn from_f32(x: f32) -> Self {
        Self((x * 65536. + 0.5) as i32)
    }

    pub fn from_f2dot14(x: i16) -> Self {
        Self(x as i32 * 4)
    }

    pub fn round(self) -> Self {
        Self(((self.0 + 0x8000) as u32 & 0xFFFF0000) as i32)
    }

    pub fn floor(self) -> Self {
        Self((self.0 as u32 & 0xFFFF0000) as i32)
    }

    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    pub fn min(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0))
    }

    pub fn max(self, rhs: Self) -> Self {
        Self(self.0.max(rhs.0))
    }

    pub fn fract(self) -> Self {
        Self(self.0 - self.floor().0)
    }

    pub fn to_i32(self) -> i32 {
        (self.0 + 0x8000) >> 16
    }

    pub fn to_f32(self) -> f32 {
        self.0 as f32 / 65536.
    }

    pub fn to_f2dot14(self) -> i16 {
        ((self.0 + 2) >> 2) as i16
    }
}

impl Add for Fixed {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self((self.0 as u32).wrapping_add(rhs.0 as u32) as i32)
    }
}

impl AddAssign for Fixed {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_add(rhs.0);
    }
}

impl Sub for Fixed {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self((self.0 as u32).wrapping_sub(rhs.0 as u32) as i32)
    }
}

impl Mul for Fixed {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self(mul(self.0, rhs.0))
    }
}

impl Div for Fixed {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self(div(self.0, rhs.0))
    }
}

impl Div<i32> for Fixed {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: i32) -> Self {
        Self(self.0 / rhs)
    }
}

impl Neg for Fixed {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl From<f32> for Fixed {
    fn from(f: f32) -> Self {
        Self::from_f32(f)
    }
}

impl From<Fixed> for f32 {
    fn from(f: Fixed) -> Self {
        f.to_f32()
    }
}

impl FromBeData for Fixed {
    unsafe fn from_be_data_unchecked(data: &[u8], offset: usize) -> Self {
        Self(i32::from_be_data_unchecked(data, offset))
    }
}

/// Fixed point floor.
pub fn floor(x: i32) -> i32 {
    x & !63
}

/// Fixed point ceil.
pub fn ceil(x: i32) -> i32 {
    floor(x + 63)
}

/// Fixed point round.
pub fn round(x: i32) -> i32 {
    floor(x + 32)
}

/// Fixed point multiply.
#[inline(always)]
pub fn mul(a: i32, b: i32) -> i32 {
    let ab = a as i64 * b as i64;
    ((ab + 0x8000 - if ab < 0 { 1 } else { 0 }) >> 16) as i32
}

/// Fixed point divide.
pub fn div(mut a: i32, mut b: i32) -> i32 {
    let mut s = 1;
    if a < 0 {
        a = -a;
        s = -1;
    }
    if b < 0 {
        b = -b;
        s = -s;
    }
    let q = if b == 0 {
        0x7FFFFFFF
    } else {
        ((((a as u64) << 16) + ((b as u64) >> 1)) / (b as u64)) as u32
    };
    if s < 0 {
        -(q as i32)
    } else {
        q as i32
    }
}

/// Fixed point multiply/divide.
pub fn muldiv(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    let mut s = 1;
    if a < 0 {
        a = -a;
        s = -1;
    }
    if b < 0 {
        b = -b;
        s = -s;
    }
    if c < 0 {
        c = -c;
        s = -s;
    }
    let d = if c > 0 {
        ((a as i64) * (b as i64) + ((c as i64) >> 1)) / c as i64
    } else {
        0x7FFFFFFF
    };
    if s < 0 {
        -(d as i32)
    } else {
        d as i32
    }
}
