// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest_derive;
use proptest::prelude::*;

#[derive(Debug, Arbitrary)]
enum T0 {
    #[proptest(params = "u8", value = "T0::V0(params / 2)")]
    V0(u8),
}

#[derive(Debug, Arbitrary)]
enum T1 {
    #[proptest(params = "u8", value = "T1::V0 { field: params * 2 }")]
    V0 { field: u8 },
}

#[derive(Debug, Arbitrary)]
enum T2 {
    V0(#[proptest(params = "u8", value = "params.is_power_of_two()")] bool),
}

#[derive(Debug, Arbitrary)]
enum T3 {
    V0 {
        #[proptest(params = "u8", value = "params * params")]
        field: u8,
    },
}

#[derive(Debug, Arbitrary)]
struct T4 {
    #[proptest(params = "u8", value = "params - 3")]
    field: u8,
}

fn add(x: u8) -> u8 {
    x + 1
}

#[derive(Debug, Arbitrary)]
struct T5(#[proptest(params = "u8", value = "add(params)")] u8);

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<T0>();
    assert_arbitrary::<T1>();
    assert_arbitrary::<T2>();
    assert_arbitrary::<T3>();
    assert_arbitrary::<T4>();
    assert_arbitrary::<T5>();
}

proptest! {
    #[test]
    fn t0_test(v in any_with::<T0>(4)) {
        let T0::V0(x) = v;
        assert_eq!(x, 2);
    }

    #[test]
    fn t1_test(v in any_with::<T1>(4)) {
        let T1::V0 { field: x } = v;
        assert_eq!(x, 8);
    }

    #[test]
    fn t2_test_true(v in any_with::<T2>(4)) {
        let T2::V0(x) = v;
        assert!(x);
    }

    #[test]
    fn t2_test_false(v in any_with::<T2>(10)) {
        let T2::V0(x) = v;
        assert!(!x);
    }

    #[test]
    fn t3_test(v in any_with::<T3>(4)) {
        let T3::V0 { field: x } = v;
        assert_eq!(x, 16);
    }

    #[test]
    fn t4_test(v in any_with::<T4>(4)) {
        assert_eq!(v.field, 1);
    }

    #[test]
    fn t5_test(v in any_with::<T5>(4)) {
        assert_eq!(v.0, 5);
    }
}
