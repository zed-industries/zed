use num_traits::float::FloatCore;
use num_traits::{Num, NumCast, One, ToPrimitive, Zero};
use ordered_float::NotNan;
use std::num::FpCategory;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct EvilFloat(f32);

impl Zero for EvilFloat {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl Add for EvilFloat {
    type Output = Self;

    fn add(self, _: Self) -> Self::Output {
        todo!()
    }
}

impl One for EvilFloat {
    fn one() -> Self {
        todo!()
    }
}

impl Mul for EvilFloat {
    type Output = Self;

    fn mul(self, _: Self) -> Self::Output {
        todo!()
    }
}

impl Sub for EvilFloat {
    type Output = Self;

    fn sub(self, _: Self) -> Self::Output {
        todo!()
    }
}

impl Div for EvilFloat {
    type Output = Self;

    fn div(self, _: Self) -> Self::Output {
        todo!()
    }
}

impl Rem for EvilFloat {
    type Output = Self;

    fn rem(self, _: Self) -> Self::Output {
        todo!()
    }
}

impl NumCast for EvilFloat {
    fn from<T: ToPrimitive>(_: T) -> Option<Self> {
        todo!()
    }
}

impl ToPrimitive for EvilFloat {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}

impl Neg for EvilFloat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl FloatCore for EvilFloat {
    fn is_nan(self) -> bool {
        false
    }
    fn infinity() -> Self {
        todo!()
    }
    fn neg_infinity() -> Self {
        todo!()
    }
    fn nan() -> Self {
        todo!()
    }
    fn neg_zero() -> Self {
        todo!()
    }
    fn min_value() -> Self {
        todo!()
    }
    fn min_positive_value() -> Self {
        todo!()
    }
    fn epsilon() -> Self {
        todo!()
    }
    fn max_value() -> Self {
        todo!()
    }
    fn classify(self) -> FpCategory {
        todo!()
    }
    fn to_degrees(self) -> Self {
        todo!()
    }
    fn to_radians(self) -> Self {
        todo!()
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        todo!()
    }
}

impl Num for EvilFloat {
    type FromStrRadixErr = ();

    fn from_str_radix(_: &str, _: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

#[test]
#[should_panic]
fn test_cmp_panic() {
    let evil_value = NotNan::new(EvilFloat(f32::NAN)).unwrap();
    let x = NotNan::new(EvilFloat(0.0)).unwrap();
    let _ = evil_value.cmp(&x);
}
