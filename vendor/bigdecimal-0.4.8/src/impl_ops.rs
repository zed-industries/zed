//! Implement math operations: Add,Sub, etc

use crate::*;


macro_rules! impl_add_for_primitive {
    ($t:ty) => {
        impl_add_for_primitive!(IMPL:ADD $t);
        impl_add_for_primitive!(IMPL:ADD-ASSIGN $t);
        impl_add_for_primitive!(IMPL:ADD &$t);
        impl_add_for_primitive!(IMPL:ADD-ASSIGN &$t);
    };
    (IMPL:ADD $t:ty) => {
        impl Add<$t> for BigDecimal {
            type Output = BigDecimal;

            fn add(mut self, rhs: $t) -> BigDecimal {
                self += rhs;
                self
            }
        }

        impl Add<$t> for &BigDecimal {
            type Output = BigDecimal;

            fn add(self, rhs: $t) -> BigDecimal {
                self.to_ref() + rhs
            }
        }

        impl Add<$t> for BigDecimalRef<'_> {
            type Output = BigDecimal;

            fn add(self, rhs: $t) -> BigDecimal {
                BigDecimal::from(rhs) + self
            }
        }

        impl Add<BigDecimal> for $t {
            type Output = BigDecimal;

            fn add(self, rhs: BigDecimal) -> BigDecimal {
                rhs + self
            }
        }

        impl Add<&BigDecimal> for $t {
            type Output = BigDecimal;

            fn add(self, rhs: &BigDecimal) -> BigDecimal {
                rhs + self
            }
        }
    };
    (IMPL:ADD-ASSIGN &$t:ty) => {
        // special case for the ref types
        impl AddAssign<&$t> for BigDecimal {
            fn add_assign(&mut self, rhs: &$t) {
                *self += *rhs;
            }
        }
    };
    (IMPL:ADD-ASSIGN $t:ty) => {
        impl AddAssign<$t> for BigDecimal {
            fn add_assign(&mut self, rhs: $t) {
                if rhs == 0 {
                    // no-op
                } else if self.scale == 0 {
                    self.int_val += rhs;
                } else {
                    *self += BigDecimal::from(rhs);
                }
            }
        }
    };
}

impl_add_for_primitive!(u8);
impl_add_for_primitive!(u16);
impl_add_for_primitive!(u32);
impl_add_for_primitive!(u64);
impl_add_for_primitive!(u128);
impl_add_for_primitive!(i8);
impl_add_for_primitive!(i16);
impl_add_for_primitive!(i32);
impl_add_for_primitive!(i64);
impl_add_for_primitive!(i128);


macro_rules! impl_sub_for_primitive {
    ($t:ty) => {
        impl_sub_for_primitive!(IMPL:SUB $t);
        impl_sub_for_primitive!(IMPL:SUB-ASSIGN $t);
        impl_sub_for_primitive!(IMPL:SUB &$t);
        impl_sub_for_primitive!(IMPL:SUB-ASSIGN &$t);
    };
    (IMPL:SUB $t:ty) => {
        impl Sub<$t> for BigDecimal {
            type Output = BigDecimal;

            fn sub(mut self, rhs: $t) -> BigDecimal {
                self -= rhs;
                self
            }
        }

        impl Sub<$t> for &BigDecimal {
            type Output = BigDecimal;

            fn sub(self, rhs: $t) -> BigDecimal {
                let res = BigDecimal::from(rhs).neg();
                res + self
            }
        }

        impl Sub<BigDecimal> for $t {
            type Output = BigDecimal;

            fn sub(self, rhs: BigDecimal) -> BigDecimal {
                rhs.neg() + self
            }
        }

        impl Sub<&BigDecimal> for $t {
            type Output = BigDecimal;

            fn sub(self, rhs: &BigDecimal) -> BigDecimal {
                rhs.neg() + self
            }
        }
    };
    (IMPL:SUB-ASSIGN &$t:ty) => {
        impl SubAssign<&$t> for BigDecimal {
            fn sub_assign(&mut self, rhs: &$t) {
                *self -= *rhs;
            }
        }
    };
    (IMPL:SUB-ASSIGN $t:ty) => {
        impl SubAssign<$t> for BigDecimal {
            fn sub_assign(&mut self, rhs: $t) {
                if self.scale == 0 {
                    self.int_val -= rhs;
                } else {
                    *self -= BigDecimal::from(rhs);
                }
            }
        }
    };
}


impl_sub_for_primitive!(u8);
impl_sub_for_primitive!(u16);
impl_sub_for_primitive!(u32);
impl_sub_for_primitive!(u64);
impl_sub_for_primitive!(u128);
impl_sub_for_primitive!(i8);
impl_sub_for_primitive!(i16);
impl_sub_for_primitive!(i32);
impl_sub_for_primitive!(i64);
impl_sub_for_primitive!(i128);


macro_rules! impl_mul_for_primitive {
    ($t:ty) => {
        impl_mul_for_primitive!(IMPL:MUL $t);
        impl_mul_for_primitive!(IMPL:MUL-ASSIGN $t);
        impl_mul_for_primitive!(IMPL:MUL &$t);
        impl_mul_for_primitive!(IMPL:MUL-ASSIGN &$t);
    };
    (IMPL:MUL $t:ty) => {
        impl Mul<$t> for BigDecimal {
            type Output = BigDecimal;

            fn mul(mut self, rhs: $t) -> BigDecimal {
                self *= rhs;
                self
            }
        }

        impl Mul<$t> for &BigDecimal {
            type Output = BigDecimal;

            fn mul(self, rhs: $t) -> BigDecimal {
                let res = BigDecimal::from(rhs);
                res * self
            }
        }

        impl Mul<BigDecimal> for $t {
            type Output = BigDecimal;

            fn mul(self, rhs: BigDecimal) -> BigDecimal {
                rhs * self
            }
        }

        impl Mul<&BigDecimal> for $t {
            type Output = BigDecimal;

            fn mul(self, rhs: &BigDecimal) -> BigDecimal {
                rhs * self
            }
        }
    };
    (IMPL:MUL-ASSIGN $t:ty) => {
        impl MulAssign<$t> for BigDecimal {
            fn mul_assign(&mut self, rhs: $t) {
                if rhs.is_zero() {
                    *self = BigDecimal::zero()
                } else if rhs.is_one() {
                    // no-op
                } else {
                    *self *= BigDecimal::from(rhs);
                }
            }
        }
    };
}


impl_mul_for_primitive!(u8);
impl_mul_for_primitive!(u16);
impl_mul_for_primitive!(u32);
impl_mul_for_primitive!(u64);
impl_mul_for_primitive!(u128);
impl_mul_for_primitive!(i8);
impl_mul_for_primitive!(i16);
impl_mul_for_primitive!(i32);
impl_mul_for_primitive!(i64);
impl_mul_for_primitive!(i128);

macro_rules! impl_div_for_primitive {
    (f32) => {
        impl_div_for_primitive!(IMPL:DIV:FLOAT f32);
        impl_div_for_primitive!(IMPL:DIV:REF &f32);
    };
    (f64) => {
        impl_div_for_primitive!(IMPL:DIV:FLOAT f64);
        impl_div_for_primitive!(IMPL:DIV:REF &f64);
    };
    ($t:ty) => {
        impl_div_for_primitive!(IMPL:DIV $t);
        impl_div_for_primitive!(IMPL:DIV:REF &$t);
        impl_div_for_primitive!(IMPL:DIV-ASSIGN $t);
    };
    (IMPL:DIV $t:ty) => {
        impl Div<$t> for BigDecimal {
            type Output = BigDecimal;

            #[cfg(rustc_1_70)]  // Option::is_some_and
            #[allow(clippy::incompatible_msrv)]
            fn div(self, denom: $t) -> BigDecimal {
                if denom.is_one() {
                    self
                } else if denom.checked_neg().is_some_and(|n| n == 1) {
                    self.neg()
                } else if denom.clone() == 2 {
                    self.half()
                } else if denom.checked_neg().is_some_and(|n| n == 2) {
                    self.half().neg()
                } else {
                    self / BigDecimal::from(denom)
                }
            }

            #[cfg(not(rustc_1_70))]
            fn div(self, denom: $t) -> BigDecimal {
                if denom.is_one() {
                    self
                } else if denom.checked_neg().map(|n| n == 1).unwrap_or(false) {
                    self.neg()
                } else if denom.clone() == 2 {
                    self.half()
                } else if denom.checked_neg().map(|n| n == 2).unwrap_or(false) {
                    self.half().neg()
                } else {
                    self / BigDecimal::from(denom)
                }
            }
        }

        impl Div<$t> for &BigDecimal {
            type Output = BigDecimal;

            fn div(self, denom: $t) -> BigDecimal {
                self.clone() / denom
            }
        }

        impl Div<BigDecimal> for $t {
            type Output = BigDecimal;

            fn div(self, denom: BigDecimal) -> BigDecimal {
                if self.is_one() {
                    denom.inverse()
                } else {
                    BigDecimal::from(self) / denom
                }
            }
        }

        impl Div<&BigDecimal> for $t {
            type Output = BigDecimal;

            fn div(self, denom: &BigDecimal) -> BigDecimal {
                self / denom.clone()
            }
        }
    };
    (IMPL:DIV-ASSIGN $t:ty) => {
        impl DivAssign<$t> for BigDecimal {
            fn div_assign(&mut self, rhs: $t) {
                if rhs.is_zero() {
                    *self = BigDecimal::zero()
                } else if rhs.is_one() {
                    // no-op
                } else {
                    *self = self.clone() / BigDecimal::from(rhs);
                }
            }
        }
    };
    (IMPL:DIV:REF $t:ty) => {
        impl Div<$t> for BigDecimal {
            type Output = BigDecimal;

            fn div(self, denom: $t) -> BigDecimal {
                self / *denom
            }
        }

        impl Div<BigDecimal> for $t {
            type Output = BigDecimal;

            fn div(self, denom: BigDecimal) -> Self::Output {
                *self / denom
            }
        }

        impl Div<&BigDecimal> for $t {
            type Output = BigDecimal;

            fn div(self, denom: &BigDecimal) -> Self::Output {
                *self / denom
            }
        }

        impl DivAssign<$t> for BigDecimal {
            fn div_assign(&mut self, denom: $t) {
                self.div_assign(*denom)
            }
        }
    };
    (IMPL:DIV:FLOAT $t:ty) => {
        impl Div<$t> for BigDecimal {
            type Output = BigDecimal;

            #[allow(clippy::float_cmp)]
            fn div(self, denom: $t) -> BigDecimal {
                if !denom.is_normal() {
                    BigDecimal::zero()
                } else if denom == (1.0 as $t) {
                    self
                } else if denom == (-1.0 as $t) {
                    self.neg()
                } else if denom == (2.0 as $t) {
                    self.half()
                } else if denom == (-2.0 as $t) {
                    self.half().neg()
                } else {
                    self / BigDecimal::try_from(denom).unwrap()
                }
            }
        }

        impl Div<$t> for &BigDecimal {
            type Output = BigDecimal;

            fn div(self, denom: $t) -> BigDecimal {
                self.clone() / denom
            }
        }

        impl Div<BigDecimal> for $t {
            type Output = BigDecimal;

            fn div(self, denom: BigDecimal) -> Self::Output {
                if !self.is_normal() {
                    BigDecimal::zero()
                } else if self.is_one() {
                    denom.inverse()
                } else {
                    BigDecimal::try_from(self).unwrap() / denom
                }
            }
        }

        impl Div<&BigDecimal> for $t {
            type Output = BigDecimal;

            fn div(self, denom: &BigDecimal) -> Self::Output {
                if !self.is_normal() {
                    BigDecimal::zero()
                } else if self.is_one() {
                    denom.inverse()
                } else {
                    BigDecimal::try_from(self).unwrap() / denom
                }
            }
        }

        impl DivAssign<$t> for BigDecimal {
            fn div_assign(&mut self, denom: $t) {
                if !denom.is_normal() {
                    *self = BigDecimal::zero()
                } else {
                    *self = self.clone() / BigDecimal::try_from(denom).unwrap()
                };
            }
        }
    };
}


impl_div_for_primitive!(u8);
impl_div_for_primitive!(u16);
impl_div_for_primitive!(u32);
impl_div_for_primitive!(u64);
impl_div_for_primitive!(u128);
impl_div_for_primitive!(i8);
impl_div_for_primitive!(i16);
impl_div_for_primitive!(i32);
impl_div_for_primitive!(i64);
impl_div_for_primitive!(i128);

impl_div_for_primitive!(f32);
impl_div_for_primitive!(f64);


impl Neg for BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn neg(mut self) -> BigDecimal {
        self.int_val = -self.int_val;
        self
    }
}

impl Neg for &BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn neg(self) -> BigDecimal {
        -self.clone()
    }
}

impl Neg for BigDecimalRef<'_> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            sign: self.sign.neg(),
            digits: self.digits,
            scale: self.scale,
        }
    }
}
