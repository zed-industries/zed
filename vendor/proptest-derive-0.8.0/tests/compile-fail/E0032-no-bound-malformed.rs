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

// Show non-fatal:
#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0032]
                            //~| [proptest_derive, E0007]
#[proptest(no_bound = "...", value("TU0"))]
struct TU0;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound("..."))]
struct TU1;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound = "...")]
struct TU2 {}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound("..."))]
struct TU3 {}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound = "...")]
struct TU4();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound("..."))]
struct TU5();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound = "...")]
struct T0 {
    field: u8
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound("..."))]
struct T1 {
    field: u8
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound = "...")]
struct T2(u8);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound("..."))]
struct T3(u8);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
struct T4 {
    #[proptest(no_bound = "...")]
    field: u8
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
struct T5 {
    #[proptest(no_bound("..."))]
    field: u8
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
struct T6(
    #[proptest(no_bound = "...")]
    u8
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
struct T7(
    #[proptest(no_bound("..."))]
    u8
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
#[proptest(no_bound("..."))]
enum T8 {
    V1,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0032]
enum T9 {
    #[proptest(no_bound("..."))]
    V1,
}
