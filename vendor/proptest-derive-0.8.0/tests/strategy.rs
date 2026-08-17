// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::{proptest, Arbitrary, Strategy};
use proptest_derive::Arbitrary;

fn make_strategy(start: usize) -> impl Strategy<Value = usize> {
    (start..100).prop_map(|x| x * 2)
}

fn make_strategy2() -> impl Strategy<Value = usize> {
    make_strategy(88)
}

#[derive(Debug, Arbitrary)]
struct T0 {
    #[proptest(strategy = "make_strategy(0)")]
    foo: usize,
    #[proptest(strategy("make_strategy(11)"))]
    bar: usize,
    #[proptest(strategy(make_strategy2))]
    baz: usize,
}

#[derive(Debug, Arbitrary)]
struct T1(
    #[proptest(strategy = "make_strategy(22)")] usize,
    #[proptest(strategy("make_strategy(33)"))] usize,
    #[proptest(strategy(make_strategy2))] usize,
);

#[derive(Debug, Arbitrary)]
enum T2 {
    V0(#[proptest(strategy("make_strategy(44)"))] usize),
    V1 {
        #[proptest(strategy = "make_strategy(55)")]
        field: usize,
    },
    V2(#[proptest(strategy = "make_strategy(66)")] usize),
    V3 {
        #[proptest(strategy("make_strategy(77)"))]
        field: usize,
    },
    V4(#[proptest(strategy(make_strategy2))] usize),
    V5 {
        #[proptest(strategy(make_strategy2))]
        field: usize,
    },
}

fn assert_consistency(start: usize, val: usize) {
    assert!(val % 2 == 0 && val < 200 && val >= (start * 2));
}

proptest! {
    #[test]
    fn t0_test(v: T0) {
        assert_consistency(0, v.foo);
        assert_consistency(11, v.bar);
        assert_consistency(88, v.baz);
    }

    #[test]
    fn t1_test(v: T1) {
        assert_consistency(22, v.0);
        assert_consistency(33, v.1);
        assert_consistency(88, v.2);
    }

    #[test]
    fn t2_test(v: T2) {
        match v {
            T2::V0(v) => assert_consistency(44, v),
            T2::V1 { field } => assert_consistency(55, field),
            T2::V2(v) => assert_consistency(66, v),
            T2::V3 { field } => assert_consistency(77, field),
            T2::V4(v) => assert_consistency(88, v),
            T2::V5 { field } => assert_consistency(88, field),
        }
    }
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<T0>();
    assert_arbitrary::<T1>();
    assert_arbitrary::<T2>();
}
