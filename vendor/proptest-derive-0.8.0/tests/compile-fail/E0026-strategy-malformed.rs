// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest_derive;

fn main() {}

// Show non-fatal:
#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0026]
                            //~| [proptest_derive, E0008]
#[proptest(strategy)]
#[proptest(skip)]
struct NonFatal;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T1 {
    V1 {
        #[proptest(strategy)]
        batman: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T3 {
    #[proptest(strategy("///"))]
    field: usize,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T4(
    #[proptest(strategy)]
    usize,
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T5 {
    V1 {
        #[proptest(value)]
        batman: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T6 {
    #[proptest(value)]
    field: usize,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T7(
    #[proptest(value)]
    usize,
);
