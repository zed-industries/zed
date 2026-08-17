extern crate num as num_renamed;
#[macro_use]
extern crate num_derive;

use crate::num_renamed::{
    Float, FromPrimitive, Num, NumCast, One, Signed, ToPrimitive, Unsigned, Zero,
};
use std::ops::Neg;

#[derive(PartialEq, Zero, One, NumOps, Num, Unsigned)]
struct MyNum(u32);

#[test]
fn test_derive_unsigned_works() {
    fn do_nothing_on_unsigned(_input: impl Unsigned) {}

    let x = MyNum(42);
    do_nothing_on_unsigned(x);
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    ToPrimitive,
    FromPrimitive,
    NumOps,
    NumCast,
    One,
    Zero,
    Num,
    Float,
    Signed,
)]
struct MyFloat(f64);

impl Neg for MyFloat {
    type Output = MyFloat;
    fn neg(self) -> Self {
        MyFloat(self.0.neg())
    }
}

#[test]
fn test_from_primitive() {
    assert_eq!(MyFloat::from_u32(25), Some(MyFloat(25.0)));
}

#[test]
fn test_from_primitive_128() {
    assert_eq!(
        MyFloat::from_i128(std::i128::MIN),
        Some(MyFloat((-2.0).powi(127)))
    );
}

#[test]
fn test_to_primitive() {
    assert_eq!(MyFloat(25.0).to_u32(), Some(25));
}

#[test]
fn test_to_primitive_128() {
    let f = MyFloat::from_f32(std::f32::MAX).unwrap();
    assert_eq!(f.to_i128(), None);
    assert_eq!(f.to_u128(), Some(0xffff_ff00_0000_0000_0000_0000_0000_0000));
}

#[test]
fn test_num_ops() {
    assert_eq!(MyFloat(25.0) + MyFloat(10.0), MyFloat(35.0));
    assert_eq!(MyFloat(25.0) - MyFloat(10.0), MyFloat(15.0));
    assert_eq!(MyFloat(25.0) * MyFloat(2.0), MyFloat(50.0));
    assert_eq!(MyFloat(25.0) / MyFloat(10.0), MyFloat(2.5));
    assert_eq!(MyFloat(25.0) % MyFloat(10.0), MyFloat(5.0));
}

#[test]
fn test_num_cast() {
    assert_eq!(<MyFloat as NumCast>::from(25u8), Some(MyFloat(25.0)));
}

#[test]
fn test_zero() {
    assert_eq!(MyFloat::zero(), MyFloat(0.0));
}

#[test]
fn test_one() {
    assert_eq!(MyFloat::one(), MyFloat(1.0));
}

#[test]
fn test_num() {
    assert_eq!(MyFloat::from_str_radix("25", 10).ok(), Some(MyFloat(25.0)));
}

#[test]
fn test_float() {
    assert_eq!(MyFloat(4.0).log(MyFloat(2.0)), MyFloat(2.0));
}

#[test]
fn test_signed() {
    assert!(MyFloat(-2.0).is_negative())
}
