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

// Show non-fatal:
#[derive(Debug, Arbitrary)] //~ 2 errors:
                            //~| [proptest_derive, E0015]
                            //~| [proptest_derive, E0008]
#[proptest = 1]
#[proptest(skip)]
struct T0;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
#[proptest = 1]
struct T1();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
#[proptest = 1]
struct T2 {}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
struct T3 {
    #[proptest = 1]
    field: Vec<String>,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
struct T4(
    #[proptest = 1]
    usize,
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
#[proptest = 1]
enum T5 {
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
enum T6 {
    #[proptest = 1]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
enum T7 {
    V0 {
        #[proptest = 1]
        foo: &'static str,
    },
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
enum T8 {
    V0(#[proptest = 1] bool)
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
enum T9 {
    #[proptest = 1]
    V0(bool),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0015]
enum T10 {
    #[proptest = 1]
    V0 { bar: bool },
}
