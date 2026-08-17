// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(never_type)]

use proptest::prelude::Arbitrary;
use proptest_derive::Arbitrary;

#[derive(Debug, Arbitrary)]
enum T1 {
    V1,
}

#[derive(Debug, Arbitrary)]
enum T2 {
    V1(),
    V2 {},
}

#[derive(Debug, Arbitrary)]
enum T3 {
    V1(),
    V2 {},
    V3,
}

#[derive(Debug, Arbitrary)]
enum T4 {
    V1,
    V2(),
    V3,
    V4 {},
}

#[derive(Debug, Arbitrary)]
enum T5 {
    V1,
    V2,
    V3,
    V4 {},
    V5(),
}

#[derive(Debug, Arbitrary)]
enum T6 {
    V1(),
    V2,
    V3 {},
    V4,
    V5,
    V6,
}

#[derive(Debug, Arbitrary)]
enum T7 {
    V1,
    V2,
    V3,
    V4 {},
    V5,
    V6,
    V7(),
}

#[derive(Debug, Arbitrary)]
enum T8 {
    V1,
    V2,
    V3(),
    V4,
    V5,
    V6 {},
    V7,
    V8,
}

#[derive(Debug, Arbitrary)]
enum T9 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
}

#[derive(Debug, Arbitrary)]
enum T10 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
}

#[derive(Debug, Arbitrary)]
enum T11 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
}

#[derive(Debug, Arbitrary)]
enum T12 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
}

#[derive(Debug, Arbitrary)]
enum T13 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
}

#[derive(Debug, Arbitrary)]
enum T14 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
}

#[derive(Debug, Arbitrary)]
enum T15 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
}

#[derive(Debug, Arbitrary)]
enum T16 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
}

#[derive(Debug, Arbitrary)]
enum T17 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
}

#[derive(Debug, Arbitrary)]
enum T18 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
}

#[derive(Debug, Arbitrary)]
enum T19 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
}

#[derive(Debug, Arbitrary)]
enum T20 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
}

#[derive(Debug, Arbitrary)]
enum T21 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
}

#[derive(Debug, Arbitrary)]
enum T22 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
}

#[derive(Debug, Arbitrary)]
enum T23 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
}

#[derive(Debug, Arbitrary)]
enum T24 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
}

#[derive(Debug, Arbitrary)]
enum T25 {
    V1,
    V2,
    V3,
    V4,
    V5 {},
    V6(),
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
}

#[derive(Clone, Debug, Arbitrary)]
enum Alan {
    A(usize),
    B(String),
    C(()),
    D(u32),
    E(f64),
    F(char),
}

#[derive(Clone, Debug, Arbitrary)]
enum SameType {
    A(usize),
    B(usize),
}

#[derive(Arbitrary, Debug)]
enum OneTwo {
    One(u8),
    Two(u8, u8),
}

#[derive(Arbitrary, Debug)]
enum ZeroOneTwo {
    Zero,
    One(u8),
    Two(u8, u8),
}

#[derive(Arbitrary, Debug)]
enum Nested {
    First(SameType),
    Second(ZeroOneTwo, OneTwo),
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<T1>();
    assert_arbitrary::<T2>();
    assert_arbitrary::<T3>();
    assert_arbitrary::<T4>();
    assert_arbitrary::<T5>();
    assert_arbitrary::<T6>();
    assert_arbitrary::<T7>();
    assert_arbitrary::<T8>();
    assert_arbitrary::<T9>();
    assert_arbitrary::<T10>();
    assert_arbitrary::<T11>();
    assert_arbitrary::<T12>();
    assert_arbitrary::<T13>();
    assert_arbitrary::<T14>();
    assert_arbitrary::<T15>();
    assert_arbitrary::<T16>();
    assert_arbitrary::<T17>();
    assert_arbitrary::<T18>();
    assert_arbitrary::<T19>();
    assert_arbitrary::<T20>();
    assert_arbitrary::<T21>();
    assert_arbitrary::<T22>();
    assert_arbitrary::<T23>();
    assert_arbitrary::<T24>();
    assert_arbitrary::<T25>();
    assert_arbitrary::<Alan>();
    assert_arbitrary::<SameType>();
    assert_arbitrary::<OneTwo>();
    assert_arbitrary::<ZeroOneTwo>();
    assert_arbitrary::<Nested>();
}
