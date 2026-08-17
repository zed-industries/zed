// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(never_type)]

use proptest::prelude::{prop_assert_eq, proptest, Arbitrary};
use proptest_derive::Arbitrary;

// Various arithmetic and basic things.
#[allow(unreachable_code)]
#[derive(Debug, Arbitrary, PartialEq)]
enum Ty1 {
    // Ensure that all of the types below are deemed uninhabited:
    _V2(!),
    _V3([!; 1]),
    _V4([!; 2 - 1]),
    _V5([!; 2 * 1]),
    _V6([!; 2 / 2]),
    _V7([!; 0b0 ^ 0b1]),
    _V8([!; 0b1 & 0b1]),
    _V9([!; 0b1 | 0b0]),
    _V10([!; 0b10 << 1]),
    _V11([!; 0b10 >> 1]),
    _V12([!; !0 - 18446744073709551614]),
    _V13([!; 1 + 2 * (3 / 3)]),
    V1,
}

proptest! {
    #[test]
    fn ty1_always_v1(v1: Ty1) {
        prop_assert_eq!(v1, Ty1::V1);
    }
}

// Can't inspect type macros called as  mac!(uninhabited_type).
macro_rules! tymac {
    ($ignore: ty) => {
        u8
    };
}

#[derive(Debug, Arbitrary)]
struct TyMac0 {
    _field: tymac!(!),
}

#[derive(Debug, Arbitrary)]
struct TyMac1 {
    _baz: tymac!([!; 3 + 4]),
}

enum _TyMac2 {
    #[deny(dead_code)]
    V0(tymac!((u8, !, usize))),
}

// Can't inspect projections through associated types:
trait Fun {
    type Prj;
}
impl Fun for ! {
    type Prj = u8;
}
impl Fun for (!, usize, !) {
    type Prj = u8;
}

#[derive(Debug, Arbitrary)]
enum UsePrj0 {
    V0(<! as Fun>::Prj),
}

#[derive(Debug, Arbitrary)]
enum UsePrj1 {
    V0(<(!, usize, !) as Fun>::Prj),
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<Ty1>();
    assert_arbitrary::<TyMac0>();
    assert_arbitrary::<TyMac1>();
    assert_arbitrary::<UsePrj0>();
    assert_arbitrary::<UsePrj1>();
}
