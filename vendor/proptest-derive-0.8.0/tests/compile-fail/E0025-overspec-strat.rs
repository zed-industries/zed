// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest_derive;
use proptest_derive::Arbitrary;

fn main() {}

// value + strategy:

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
#[proptest(value = "T0(0)", strategy = "(0..6).prop_map(T1)")]
struct T0(u8);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
struct T1 {
    #[proptest(value = "1", strategy = "(0..1).prop_map(T1)")]
    field: u8
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
struct T2(
    #[proptest(value = "1", strategy = "(0..1).prop_map(T1)")]
    u8
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
enum T3 {
    V0 {
        #[proptest(value = "1", strategy = "0..1")]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
enum T4 {
    V0(
        #[proptest(value = "1", strategy = "0..1")]
        u8
    ),
}

// value + regex:

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
#[proptest(value = "T6(String::new())", regex = "a")]
struct T6(String);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
struct T7 {
    #[proptest(value = "Vec::new()", regex = "a(b)")]
    field: Vec<u8>
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
struct T8(
    // We test with a type that won't work to ensure that the test fails before.
    #[proptest(value = "1", regex = "a|b")]
    u8
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
enum T9 {
    V0 {
        #[proptest(value = "2", regex = "[\n\t]")]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
enum T10 {
    V0(
        #[proptest(value = "3", regex = "a+")]
        u8
    ),
}

// regex + strategy:

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
#[proptest(strategy = "0..1", regex = "a")]
struct T11(String);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
struct T12 {
    #[proptest(regex = "a(b)", strategy = "1..2")]
    field: Vec<u8>
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
struct T13(
    #[proptest(strategy = "1", regex = "a|b")]
    u8
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
enum T14 {
    V0 {
        #[proptest(regex = "[\n\t]", strategy = "1..=2")]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0025]
enum T15 {
    V0(
        #[proptest(strategy = "3", regex = "a+")]
        u8
    ),
}
