// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::{proptest, Arbitrary, BoxedStrategy, Strategy};
use proptest::string::StrategyFromRegex;
use proptest_derive::Arbitrary;

fn mk_regex() -> &'static str {
    "[0-9][0-9]"
}

// struct:

#[derive(Debug, Arbitrary)]
struct T0 {
    #[proptest(regex = "a+")]
    foo: String,
    #[proptest(regex("b+"))]
    bar: String,
    #[proptest(regex(mk_regex))]
    baz: String,
    #[proptest(regex = "(a|b)+")]
    quux: Vec<u8>,
    #[proptest(regex("[abc]+"), filter("|c| c.len() < 4"))]
    wibble: Vec<u8>,
    #[proptest(regex(mk_regex))]
    wobble: Vec<u8>,
}

#[derive(Debug, Arbitrary)]
struct T1(
    #[proptest(regex = "a+")] String,
    #[proptest(regex("b+"))] String,
    #[proptest(regex(mk_regex))] String,
    #[proptest(regex = "(a|b)+")] Vec<u8>,
    #[proptest(regex("[abc]+"), filter("|c| c.len() < 4"))] Vec<u8>,
    #[proptest(regex(mk_regex))] Vec<u8>,
);

#[derive(Debug, Arbitrary)]
struct T1r(
    #[proptest(regex = r"a+")] String,
    #[proptest(regex(r"b+"))] String,
    #[proptest(regex(mk_regex))] String,
    #[proptest(regex = r"(a|b)+")] Vec<u8>,
    #[proptest(regex(r"[abc]+"), filter("|c| c.len() < 4"))] Vec<u8>,
    #[proptest(regex(mk_regex))] Vec<u8>,
);

// enum:

#[derive(Debug, Arbitrary)]
enum T2 {
    V0 {
        #[proptest(regex = "a+")]
        foo: String,
        #[proptest(regex("b+"))]
        bar: String,
        #[proptest(regex(mk_regex))]
        baz: String,
        #[proptest(regex = "(a|b)+")]
        quux: Vec<u8>,
        #[proptest(regex("[abc]+"), filter("|c| c.len() < 4"))]
        wibble: Vec<u8>,
        #[proptest(regex(mk_regex))]
        wobble: Vec<u8>,
    },
}

#[derive(Debug, Arbitrary)]
enum T3 {
    V0(
        #[proptest(regex = "a+")] String,
        #[proptest(regex("b+"))] String,
        #[proptest(regex(mk_regex))] String,
        #[proptest(regex = "(a|b)+")] Vec<u8>,
        #[proptest(regex("[abc]+"), filter("|c| c.len() < 4"))] Vec<u8>,
        #[proptest(regex(mk_regex))] Vec<u8>,
    ),
}

// Show that it works for new types and that `String` | `Vec<u8>` isn't
// hardcoded into the logic:

#[derive(Debug)]
struct NewString(String);

impl StrategyFromRegex for NewString {
    type Strategy = BoxedStrategy<Self>;

    fn from_regex(regex: &str) -> Self::Strategy {
        String::from_regex(regex).prop_map(NewString).boxed()
    }
}

#[derive(Debug, Arbitrary)]
struct T4(#[proptest(regex = "a+")] NewString);

fn check_aplus(x0: String) {
    assert!(x0.chars().count() > 0);
    assert!(x0.chars().all(|c: char| c == 'a'));
}

fn assert_adherence(
    x0: String,
    x1: String,
    x2: String,
    y0: Vec<u8>,
    y1: Vec<u8>,
    y2: Vec<u8>,
) {
    check_aplus(x0);

    assert!(x1.chars().count() > 0);
    assert!(x1.chars().all(|c: char| c == 'b'));

    assert!(x2.parse::<u8>().unwrap() < 100);

    assert!(y0.len() > 0);
    assert!(y0.iter().all(|c: &u8| [b'a', b'b'].contains(c)));

    assert!(y1.len() > 0 && y1.len() < 4);
    assert!(y1.iter().all(|c: &u8| [b'a', b'b', b'c'].contains(c)));

    assert!(y2.len() > 0);
    let test = y2
        .iter()
        .all(|c: &u8| if let b'0'..=b'9' = c { true } else { false });
    assert!(test);
}

proptest! {
    #[test]
    fn t0_adhering_to_regex(v: T0) {
        let T0 {
            foo: x0, bar: x1, baz: x2,
            quux: y0, wibble: y1, wobble: y2
        } = v;
        assert_adherence(x0, x1, x2, y0, y1, y2);
    }

    #[test]
    fn t1_adhering_to_regex(v: T1) {
        let T1(x0, x1, x2, y0, y1, y2) = v;
        assert_adherence(x0, x1, x2, y0, y1, y2);
    }

    #[test]
    fn t1_r_adhering_to_regex(v: T1r) {
        let T1r(x0, x1, x2, y0, y1, y2) = v;
        assert_adherence(x0, x1, x2, y0, y1, y2);
    }

    #[test]
    fn t2_adhering_to_regex(v: T2) {
        let T2::V0 {
            foo: x0, bar: x1, baz: x2,
            quux: y0, wibble: y1, wobble: y2
        } = v;
        assert_adherence(x0, x1, x2, y0, y1, y2);
    }

    #[test]
    fn t3_adhering_to_regex(v: T3) {
        let T3::V0(x0, x1, x2, y0, y1, y2) = v;
        assert_adherence(x0, x1, x2, y0, y1, y2);
    }

    #[test]
    fn t4_adhering_to_regex(v: T4) {
        check_aplus((v.0).0);
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
}
