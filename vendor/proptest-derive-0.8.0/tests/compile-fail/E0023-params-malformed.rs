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
                            //~| [proptest_derive, E0023]
                            //~| [proptest_derive, E0008]
#[proptest(skip)]
#[proptest(params)]
struct NonFatal;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
#[proptest(params)]
enum T0 {
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
enum T2 {
    #[proptest(params)]
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
enum T3 {
    V1 {
        #[proptest(params)]
        field: Box<str>,
    }
}
