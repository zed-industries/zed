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
                            //~| [proptest_derive, E0021]
                            //~| [proptest_derive, E0008]
#[proptest(weight)]
#[proptest(skip)]
struct NonFatal;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0021]
enum T0 {
    #[proptest(weight)]
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0021]
enum T1 {
    #[proptest(weight("abcd"))]
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0021]
enum T2 {
    #[proptest(weight("1.0"))]
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0021]
enum T3 {
    #[proptest(weight("true"))]
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0021]
enum T4 {
    #[proptest(weight = "true")]
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0021]
enum T5 {
    #[proptest(weight = true)]
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0021]
enum T6 {
    #[proptest(weight(true))]
    V1
}
