// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[allow(dead_code, reason = "unused if std and libm are both around")]
pub(crate) trait FloatFuncs: Sized {
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn cos(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn atan2(self, other: Self) -> Self;
    fn powi(self, pow: i32) -> Self;
}

impl FloatFuncs for f64 {
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }
    fn cbrt(self) -> Self {
        libm::cbrt(self)
    }
    fn cos(self) -> Self {
        libm::cos(self)
    }
    fn sin_cos(self) -> (Self, Self) {
        libm::sincos(self)
    }
    fn atan2(self, other: Self) -> Self {
        libm::atan2(self, other)
    }
    fn powi(self, pow: i32) -> Self {
        libm::pow(self, pow as _)
    }
}
