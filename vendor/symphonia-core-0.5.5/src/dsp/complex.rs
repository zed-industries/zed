// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `complex` module implements a 32-bit floating point complex number.

/// A complex number.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Complex {
    /// The real component.
    pub re: f32,
    /// The imaginary component.
    pub im: f32,
}

impl Complex {
    /// Create a new complex number.
    #[inline(always)]
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    /// Create a complex number with a value of `0 + j1`.
    #[inline(always)]
    pub fn j() -> Self {
        Self { re: 0.0, im: 1.0 }
    }

    /// Scale the complex number.
    #[inline(always)]
    pub fn scale(&self, scale: f32) -> Self {
        Self { re: self.re * scale, im: self.im * scale }
    }

    /// Take the complex conjugate of `self`.
    ///
    /// For a complex number defined as `a + jb` the complex conjugate is defined to be `a - jb`.
    #[inline(always)]
    pub fn conj(&self) -> Self {
        Self { re: self.re, im: -self.im }
    }
}

impl core::ops::Add for Complex {
    type Output = Complex;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl core::ops::AddAssign for Complex {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::Sub for Complex {
    type Output = Complex;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { re: self.re - rhs.re, im: self.im - rhs.im }
    }
}

impl core::ops::SubAssign for Complex {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl core::ops::Mul for Complex {
    type Output = Complex;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            re: (self.re * rhs.re) - (self.im * rhs.im),
            im: (self.re * rhs.im) + (self.im * rhs.re),
        }
    }
}

impl core::ops::MulAssign for Complex {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl core::ops::Div for Complex {
    type Output = Complex;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;

        Self::Output {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl core::ops::DivAssign for Complex {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl core::ops::Mul<f32> for Complex {
    type Output = Complex;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { re: self.re * rhs, im: self.im * rhs }
    }
}

impl core::ops::Div<f32> for Complex {
    type Output = Complex;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { re: self.re / rhs, im: self.im / rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_complex() {
        assert_eq!(Complex::j(), Complex::new(0.0, 1.0));

        // Conjugate
        assert_eq!(Complex::new(1.0, 10.0).conj(), Complex::new(1.0, -10.0));

        // Scale
        assert_eq!(Complex::new(5.0, 2.0).scale(3.0), Complex::new(15.0, 6.0));

        // Addition
        assert_eq!(Complex::new(3.0, 13.0) + Complex::new(7.0, 17.0), Complex::new(10.0, 30.0));

        // Subtraction
        assert_eq!(Complex::new(3.0, 13.0) - Complex::new(7.0, 17.0), Complex::new(-4.0, -4.0));

        // Multiplication
        assert_eq!(Complex::new(3.0, 13.0) * Complex::new(7.0, 17.0), Complex::new(-200.0, 142.0));

        // Division
        assert_eq!(
            Complex::new(3.0, 13.0) / Complex::new(7.0, 17.0),
            Complex::new(121.0 / 169.0, 20.0 / 169.0)
        );

        // Scalar Multiplication
        assert_eq!(Complex::new(5.0, 2.0) * 3.0, Complex::new(15.0, 6.0));

        // Scalar Division
        assert_eq!(Complex::new(4.0, 2.0) / 2.0, Complex::new(2.0, 1.0));
    }
}
