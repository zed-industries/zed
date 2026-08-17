// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(never_type)]

extern crate proptest_derive;
use proptest_derive::Arbitrary;

fn main() {}

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0006]
                            //~| [proptest_derive, E0008]
enum NonFatal<#[proptest(skip)] T> {
    #[proptest(skip)]
    Unit(T),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0006]
enum T0 {
    #[proptest(skip)]
    Unit,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0006]
enum T1 {
    #[proptest(skip)]
    V0,
    #[proptest(skip)]
    V1,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0006]
enum T2 {
    #[proptest(skip)]
    V0,
    #[proptest(skip)]
    V1,
    V2(!),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0006]
enum T3 {
    #[proptest(skip)]
    V0,
    #[proptest(skip)]
    V1,
    V2([!; 1 + 2 + (3 / 3) + (1 << 3)]),
}
