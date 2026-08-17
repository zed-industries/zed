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

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0011]
enum T0 {
    #[proptest(params = "String")]
    V0(
        #[proptest(no_params)]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0011]
enum T1 {
    #[proptest(params = "(u8, u8)")]
    V0 {
        #[proptest(no_params)]
        field: Vec<u8>
    },
}
