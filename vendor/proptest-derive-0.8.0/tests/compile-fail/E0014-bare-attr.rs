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
#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0014]
                            //~| [proptest_derive, E0007]
#[proptest]
#[proptest(value("foobar"))]
struct T0;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
#[proptest]
struct T1();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
#[proptest]
struct T2 {}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
struct T3 {
    #[proptest]
    field: Vec<String>,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
struct T4(
    #[proptest]
    usize,
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
#[proptest]
enum T5 {
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
enum T6 {
    #[proptest]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
enum T7 {
    V0 {
        #[proptest]
        foo: &'static str,
    },
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
enum T8 {
    V0(#[proptest] bool)
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
enum T9 {
    #[proptest]
    V0(bool),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0014]
enum T10 {
    #[proptest]
    V0 { bar: bool },
}
