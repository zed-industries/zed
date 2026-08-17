#![no_implicit_prelude]

use ::num_derive::*;

#[derive(FromPrimitive, ToPrimitive)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(FromPrimitive, ToPrimitive, NumCast, PartialEq, Zero, One, NumOps, Num)]
struct NewI32(i32);
