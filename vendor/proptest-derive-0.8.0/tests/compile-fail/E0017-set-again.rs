// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate proptest_derive;
use proptest_derive::Arbitrary;

fn main() {}

#[derive(Debug, Arbitrary)] //~  ERROR: 2 errors
                            //~| # [proptest_derive, E0017]
                            //~| # [proptest_derive, E0030]
#[proptest(no_params)]
#[proptest(no_params)]
struct T0;

#[derive(Debug, Arbitrary)] //~  ERROR: 2 errors
                            //~| # [proptest_derive, E0017]
                            //~| # [proptest_derive, E0030]
#[proptest(no_params)]
#[proptest(no_params)]
struct T1();

#[derive(Debug, Arbitrary)] //~  ERROR: 2 errors
                            //~| # [proptest_derive, E0017]
                            //~| # [proptest_derive, E0030]
#[proptest(no_params)]
#[proptest(no_params)]
struct T2 {}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
struct T3 {
    #[proptest(no_params)]
    #[proptest(no_params)]
    field: Vec<String>,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
struct T4(
    #[proptest(no_params)]
    #[proptest(no_params)]
    usize,
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
#[proptest(no_params)]
#[proptest(no_params)]
enum T5 {
    V0,
}

#[derive(Debug, Arbitrary)] //~  ERROR: 2 errors
                            //~| # [proptest_derive, E0017]
                            //~| # [proptest_derive, E0029]
enum T6 {
    #[proptest(no_params)]
    #[proptest(no_params)]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T7 {
    V0 {
        #[proptest(no_params)]
        #[proptest(no_params)]
        foo: &'static str,
    },
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T8 {
    V0(#[proptest(no_params)] #[proptest(no_params)] bool)
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T9 {
    #[proptest(no_params)]
    #[proptest(no_params)]
    V0(bool),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T10 {
    #[proptest(no_params)]
    #[proptest(no_params)]
    V0 { bar: bool },
}

#[derive(Debug, Arbitrary)] //~  ERROR: 2 errors
                            //~| # [proptest_derive, E0017]
                            //~| # [proptest_derive, E0030]
#[proptest(no_params, no_params)]
struct T11;

#[derive(Debug, Arbitrary)] //~  ERROR: 2 errors
                            //~| # [proptest_derive, E0017]
                            //~| # [proptest_derive, E0030]
#[proptest(no_params, no_params)]
struct T12();

#[derive(Debug, Arbitrary)] //~  ERROR: 2 errors
                            //~| # [proptest_derive, E0017]
                            //~| # [proptest_derive, E0030]
#[proptest(no_params, no_params)]
struct T13 {}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
struct T14 {
    #[proptest(no_params, no_params)]
    field: Vec<String>,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
struct T15(
    #[proptest(no_params, no_params)]
    usize,
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
#[proptest(no_params, no_params)]
enum T16 {
    V0,
}

#[derive(Debug, Arbitrary)] //~  ERROR: 2 errors
                            //~| # [proptest_derive, E0017]
                            //~| # [proptest_derive, E0029]
enum T17 {
    #[proptest(no_params, no_params)]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T18 {
    V0 {
        #[proptest(no_params, no_params)]
        foo: &'static str,
    },
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T19 {
    V0(#[proptest(no_params, no_params)] bool)
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T20 {
    #[proptest(no_params, no_params)]
    V0(bool),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T21 {
    #[proptest(no_params, no_params)]
    V0 { bar: bool },
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T22 {
    #[proptest(skip, skip)]
    V0,
    V1,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T23 {
    V0,
    #[proptest(w = 1, w = 2)]
    V1,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
enum T24 {
    V0,
    #[proptest(weight = 1, weight = 2)]
    V1,
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
#[proptest(params = "String", params = "u8")]
enum T25 {
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
#[proptest(params = "String", params = "u8")]
struct T26 {
    field: String,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0017]
struct T27 {
    #[proptest(value = "1", value = "3")]
    field: u8,
}

#[derive(Debug, Arbitrary)] //~  ERROR: [proptest_derive, E0017]
#[proptest(no_bound, no_bound)]
struct T28<T> {
    field: T,
}
