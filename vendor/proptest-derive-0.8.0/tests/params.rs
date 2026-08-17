// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::{
    any_with, prop_assert, prop_assert_eq, proptest, Arbitrary,
};
use proptest_derive::Arbitrary;

struct ComplexType {
    max: u64,
}

impl Default for ComplexType {
    fn default() -> Self {
        Self { max: 10 }
    }
}

#[derive(Debug, Arbitrary)]
#[proptest(params(ComplexType))]
struct TopHasParams {
    _string: usize,
    #[proptest(strategy = "0..params.max")]
    int: u64,
}

#[derive(Debug, Arbitrary)]
#[proptest(no_params)]
struct TopNoParams {
    _stuff: usize,
}

#[derive(Debug, Arbitrary)]
struct InnerNoParams {
    string: String,
    #[proptest(no_params)]
    has: TopHasParams,
}

#[derive(Debug, Arbitrary)]
#[proptest(params(u64))]
struct TPIS {
    #[proptest(strategy = "\"a+\"")]
    string: String,
    #[proptest(strategy = "3..=params")]
    int: u64,
}

#[derive(Debug, Arbitrary)]
struct Parallel {
    #[proptest(params = "&'static str", strategy = "params")]
    string: String,
    #[proptest(params(u8), strategy = "0i64..params as i64")]
    int: i64,
}

#[derive(Debug, Arbitrary)]
struct Parallel2 {
    #[proptest(params("&'static str"), strategy = "params")]
    _string: String,
    #[proptest(params("u8"), strategy = "0i64..params as i64")]
    _int: i64,
}

const MAX: ComplexType = ComplexType { max: 5 };

proptest! {
    #[test]
    fn top_has_params(v in any_with::<TopHasParams>(MAX)) {
        prop_assert!(v.int < 5);
    }

    #[test]
    fn top_no_params(_ in any_with::<TopNoParams>(())) {}

    #[test]
    fn inner_params(inner in any_with::<InnerNoParams>("\\s+".into())) {
        prop_assert!(inner.has.int < 10);
        prop_assert!(inner.string.trim().is_empty());
    }

    #[test]
    fn top_param_inner_strat(inner in any_with::<TPIS>(6)) {
        prop_assert!(inner.int <= 6);
        prop_assert!(inner.int >= 3);
        prop_assert_eq!(
            0,
            inner.string.split("a").filter(|s| !s.is_empty()).count()
        );
    }

    #[test]
    fn parallel_params(inner in any_with::<Parallel>(("[0-9]", 3))) {
        prop_assert!(inner.int >= 0);
        prop_assert!(inner.int < 3);
        prop_assert!(inner.string.chars().next().unwrap().is_digit(10));
    }

    #[test]
    fn parallel_params2(inner in any_with::<Parallel>(("[0-9]", 3))) {
        prop_assert!(inner.int >= 0);
        prop_assert!(inner.int < 3);
        prop_assert!(inner.string.chars().next().unwrap().is_digit(10));
    }
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<TopHasParams>();
    assert_arbitrary::<TopNoParams>();
    assert_arbitrary::<InnerNoParams>();
    assert_arbitrary::<TPIS>();
    assert_arbitrary::<Parallel>();
    assert_arbitrary::<Parallel2>();
}
