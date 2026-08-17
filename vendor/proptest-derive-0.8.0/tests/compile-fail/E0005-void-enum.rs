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

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0005]
enum T0 {
    V0(!),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0005]
enum T1 {
    V0(!, bool),
    V1([!; 1]),
    V2([(!, bool); 1])
}
