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

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0029]
                            //~| [proptest_derive, E0008]
enum NonFatal {
    #[proptest(strategy = "Just(T0::V0)")]
    V0,
    V1 {
        #[proptest(skip)]
        field: usize,
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T0 {
    #[proptest(strategy = "Just(T0::V0)")]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T1 {
    #[proptest(value = "T0::V0")]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T2 {
    #[proptest(strategy = "Just(T0::V0)")]
    V0 {},
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T3 {
    #[proptest(value = "T0::V0")]
    V0 {},
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T4 {
    #[proptest(strategy = "Just(T0::V0)")]
    V0(),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T5 {
    #[proptest(value = "T0::V0")]
    V0(),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T6 {
    #[proptest(regex = "a+")]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T7 {
    #[proptest(regex = "b*")]
    V0 {},
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T8 {
    #[proptest(regex = "a|b")]
    V0(),
}
