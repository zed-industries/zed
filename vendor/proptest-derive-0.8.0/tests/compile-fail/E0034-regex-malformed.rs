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
                            //~| [proptest_derive, E0034]
                            //~| [proptest_derive, E0008]
#[proptest(regex)]
#[proptest(skip)]
struct NonFatal;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0034]
enum T1 {
    V1 {
        #[proptest(regex)]
        batman: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0034]
struct T3 {
    #[proptest(regex)]
    field: usize,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0034]
struct T4(
    #[proptest(regex)]
    usize,
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0034]
enum T5 {
    V1 {
        #[proptest(regex)]
        batman: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0034]
struct T6 {
    #[proptest(regex)]
    field: usize,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0034]
struct T7(
    #[proptest(regex)]
    usize,
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0034]
struct T8(
    #[proptest(regex = 1)]
    String,
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0034]
struct T9(
    #[proptest(regex = true)]
    String,
);
