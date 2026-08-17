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

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
#[proptest(no_bounds)]
struct T0<T>(T);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
enum T1 {
    #[proptest(weights = 1)]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
enum T2 {
    #[proptest(weighted = 1)]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
enum T3 {
    V0(
        #[proptest(strat = "1..0")]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
enum T4 {
    V0(
        #[proptest(strategies = "1..0")]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
struct T5 {
    #[proptest(values = "0")]
    field: u8,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
struct T6 {
    #[proptest(valued = "0")]
    field: u8,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
struct T7 {
    #[proptest(fix = "0")]
    field: u8,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
struct T8 {
    #[proptest(fixed = "0")]
    field: u8,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
#[proptest(param = "u8")]
enum T9 {
    V0(u8),
}

// Show that E0018 is non-fatal.
#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0018]
                            //~| [proptest_derive, E0011]
#[proptest(parameters = "u8")]
enum T10 {
    #[proptest(params = "u8")]
    V0(u8),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
#[proptest(no_param)]
struct T11;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
#[proptest(no_parameters)]
struct T12;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0018]
#[proptest(foobar)]
struct T13;
