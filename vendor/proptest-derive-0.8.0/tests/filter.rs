// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::*;
use proptest_derive::Arbitrary;

fn even(x: &usize) -> bool {
    x % 2 == 0
}

fn rem3(x: &usize) -> bool {
    x % 3 == 0
}

#[derive(Copy, Clone)]
struct Param(usize);

impl Default for Param {
    fn default() -> Self {
        Param(100)
    }
}

#[derive(Debug, Arbitrary)]
#[proptest(filter("|x| x.foo % 3 == 0"))]
struct T0 {
    #[proptest(no_params, filter(even))]
    foo: usize,
    #[proptest(filter("|x| x % 2 == 1"))]
    bar: usize,
    #[proptest(strategy = "0..100usize", filter = "|x| x % 2 == 1")]
    baz: usize,
    #[proptest(value = "42", filter(even))]
    quux: usize,
    #[proptest(params(Param), strategy("0..=params.0"), filter("|x| *x > 2"))]
    wibble: usize,
}

#[derive(Debug, Arbitrary)]
#[proptest(params(Param))]
#[proptest(filter("|x| x.foo % 3 == 0"))]
struct T1 {
    #[proptest(filter(even))]
    foo: usize,
    #[proptest(filter("|x| x % 2 == 1"))]
    bar: usize,
    #[proptest(strategy = "0..100usize", filter = "|x| x % 2 == 1")]
    baz: usize,
    #[proptest(value = "42", filter(even))]
    quux: usize,
    #[proptest(strategy("0..=params.0"), filter("|x| *x > 2"))]
    wibble: usize,
}

#[derive(Debug, Arbitrary)]
#[proptest(filter("|x| x.0 % 3 == 0"))]
struct T2(
    #[proptest(no_params, filter(even))] usize,
    #[proptest(filter("|x| x % 2 == 1"))] usize,
    #[proptest(strategy = "0..100usize", filter = "|x| x % 2 == 1")] usize,
    #[proptest(value = "42", filter(even))] usize,
    #[proptest(params(Param), strategy("0..=params.0"), filter("|x| *x > 2"))]
    usize,
);

#[derive(Debug, Arbitrary)]
#[proptest(filter("|x| x.0 % 3 == 0"))]
struct T3(
    #[proptest(no_params, filter(even))] usize,
    #[proptest(filter("|x| x % 2 == 1"))] usize,
    #[proptest(strategy = "0..100usize", filter = "|x| x % 2 == 1")] usize,
    #[proptest(value = "42", filter(even))] usize,
    #[proptest(params(Param), strategy("0..=params.0"), filter("|x| *x > 2"))]
    usize,
);

fn is_v0(v: &T4) -> bool {
    if let T4::V0 { .. } = v {
        true
    } else {
        false
    }
}

#[derive(Debug, Arbitrary)]
#[proptest(filter(is_v0))]
enum T4 {
    V0 {
        #[proptest(filter(even))]
        field: usize,
    },
    V1,
}

fn t5_v0_rem_3(v: &T5) -> bool {
    if let T5::V0 { field } = v {
        rem3(&field)
    } else {
        false
    }
}

fn t5_v1_rem_5(v: &T5) -> bool {
    if let T5::V1(field) = v {
        field % 5 == 0
    } else {
        false
    }
}

#[derive(Debug, Arbitrary)]
enum T5 {
    #[proptest(filter(t5_v0_rem_3))]
    V0 {
        #[proptest(filter(even))]
        field: usize,
    },
    #[proptest(
        strategy("(0..1000usize).prop_map(T5::V1)"),
        filter(t5_v1_rem_5)
    )]
    V1(usize),
}

fn t6_v0_rem_3(v: &T6) -> bool {
    if let T6::V0 { field } = v {
        rem3(&field)
    } else {
        false
    }
}

fn t6_v1_rem_5(v: &T6) -> bool {
    if let T6::V1(field) = v {
        field % 5 == 0
    } else {
        false
    }
}

#[derive(Debug, Arbitrary)]
#[proptest(params(Param))]
enum T6 {
    #[proptest(filter(t6_v0_rem_3))]
    V0 {
        #[proptest(filter(even))]
        field: usize,
    },
    #[proptest(
        strategy("(0..params.0).prop_map(T6::V1)"),
        filter(t6_v1_rem_5)
    )]
    V1(usize),
}

#[derive(Debug, Arbitrary)]
struct T7 {
    #[proptest(filter(even), filter(rem3))]
    foo: usize,
}

proptest! {
    #[test]
    fn t0_test(v: T0) {
        assert!(even(&v.foo) && rem3(&v.foo));
        assert!(!even(&v.bar));
        assert!(!even(&v.baz) && v.baz < 100);
        assert!(even(&v.quux) && v.quux == 42);
        assert!(even(&v.quux) && v.quux == 42);
        assert!(v.wibble > 2 && v.wibble <= 100);
    }

    #[test]
    fn t1_test(v: T1) {
        assert!(even(&v.foo) && v.foo % 3 == 0);
        assert!(!even(&v.bar));
        assert!(!even(&v.baz) && v.baz < 100);
        assert!(even(&v.quux) && v.quux == 42);
        assert!(v.wibble > 2 && v.wibble <= 100);
    }

    #[test]
    fn t2_test(v: T2) {
        assert!(even(&v.0) && v.0 % 3 == 0);
        assert!(!even(&v.1));
        assert!(!even(&v.2) && v.2 < 100);
        assert!(even(&v.3) && v.3 == 42);
        assert!(v.4 > 2 && v.4 <= 100);
    }

    #[test]
    fn t3_test(v: T3) {
        assert!(even(&v.0) && v.0 % 3 == 0);
        assert!(!even(&v.1));
        assert!(!even(&v.2) && v.2 < 100);
        assert!(even(&v.3) && v.3 == 42);
        assert!(v.4 > 2 && v.4 <= 100);
    }

    #[test]
    fn t4_test(v: T4) {
        assert!(if let T4::V0 { field } = v { even(&field) } else { false });
    }

    #[test]
    fn t5_test(v: T5) {
        match v {
            T5::V0 { field } => assert!(rem3(&field) && even(&field)),
            T5::V1(field) => assert!(field < 1000 && field % 5 == 0),
        }
    }

    #[test]
    fn t6_test(v: T6) {
        match v {
            T6::V0 { field } => assert!(rem3(&field) && even(&field)),
            T6::V1(field) => assert!(field < 100 && field % 5 == 0),
        }
    }

    #[test]
    fn t7_test(v: T7) {
        assert!(even(&v.foo) && rem3(&v.foo));
    }
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<T0>();
    assert_arbitrary::<T1>();
    assert_arbitrary::<T2>();
    assert_arbitrary::<T3>();
    assert_arbitrary::<T4>();
    assert_arbitrary::<T5>();
    assert_arbitrary::<T6>();
    assert_arbitrary::<T7>();
}
