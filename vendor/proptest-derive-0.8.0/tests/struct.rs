// Copyright 2019 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::Arbitrary;
use proptest_derive::Arbitrary;

#[derive(Debug, Arbitrary)]
struct T1 {
    _f1: u8,
}

#[derive(Debug, Arbitrary)]
struct T10 {
    _f1: char,
    _f2: String,
    _f3: u8,
    _f4: u16,
    _f5: u32,
    _f6: u64,
    _f7: u128,
    _f8: f32,
    _f9: f64,
    _f10: bool,
}

#[derive(Debug, Arbitrary)]
struct T11 {
    _f1: char,
    _f2: String,
    _f3: u8,
    _f4: u16,
    _f5: u32,
    _f6: u64,
    _f7: u128,
    _f8: f32,
    _f9: f64,
    _f10: bool,
    _f11: char,
}

#[derive(Debug, Arbitrary)]
struct T13 {
    _f1: char,
    _f2: String,
    _f3: u8,
    _f4: u16,
    _f5: u32,
    _f6: u64,
    _f7: u128,
    _f8: f32,
    _f9: f64,
    _f10: bool,
    _f11: char,
    _f12: String,
    _f13: u8,
}

#[derive(Debug, Arbitrary)]
struct T19 {
    _f1: char,
    _f2: String,
    _f3: u8,
    _f4: u16,
    _f5: u32,
    _f6: u64,
    _f7: u128,
    _f8: f32,
    _f9: f64,
    _f10: bool,
    _f11: char,
    _f12: String,
    _f13: u8,
    _f14: u16,
    _f15: u32,
    _f16: u64,
    _f17: u128,
    _f18: f32,
    _f19: f64,
}

#[derive(Debug, Arbitrary)]
struct T20 {
    _f1: char,
    _f2: String,
    _f3: u8,
    _f4: u16,
    _f5: u32,
    _f6: u64,
    _f7: u128,
    _f8: f32,
    _f9: f64,
    _f10: bool,
    _f11: char,
    _f12: String,
    _f13: u8,
    _f14: u16,
    _f15: u32,
    _f16: u64,
    _f17: u128,
    _f18: f32,
    _f19: f64,
    _f20: bool,
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<T1>();
    assert_arbitrary::<T10>();
    assert_arbitrary::<T11>();
    assert_arbitrary::<T13>();
    assert_arbitrary::<T19>();
    assert_arbitrary::<T20>();
}
