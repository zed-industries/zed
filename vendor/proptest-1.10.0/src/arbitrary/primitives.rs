//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for primitive types.

use crate::bool;
use crate::char;
use crate::num::{
    f32, f64, i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize,
};

arbitrary!(bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, i128, u128);

// Note that for floating point types we limit the space since a lot of code
// isn't prepared for (and is not intended to be) things like NaN and infinity.
arbitrary!(f32, f32::Any; {
    f32::POSITIVE | f32::NEGATIVE | f32::ZERO | f32::SUBNORMAL | f32::NORMAL
});
arbitrary!(f64, f64::Any; {
    f64::POSITIVE | f64::NEGATIVE | f64::ZERO | f64::SUBNORMAL | f64::NORMAL
});

arbitrary!(char, char::CharStrategy<'static>; char::any());

#[cfg(test)]
mod test {
    no_panic_test!(
        bool => bool,
        char => char,
        f32 => f32, f64 => f64,
        isize => isize, usize => usize,
        i8 => i8, i16 => i16, i32 => i32, i64 => i64, i128 => i128,
        u8 => u8, u16 => u16, u32 => u32, u64 => u64, u128 => u128
    );
}
