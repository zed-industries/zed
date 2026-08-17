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

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors
                            //~| [proptest_derive, E0028]
                            //~| [proptest_derive, E0006]
enum NonFatal {
    #[proptest(skip, filter(foo))]
    V1(u8),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T0 {
    #[proptest(skip, filter(foo))]
    V1(u8),
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T1 {
    #[proptest(
        skip,
        filter(foo)
    )]
    V1 {
        field: u8
    },
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T2 {
    #[proptest(skip)]
    V1(
        #[proptest(filter(foo))]
        u8
    ),
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T3 {
    #[proptest(skip)]
    V1 {
        #[proptest(filter(foo))]
        field: u8
    },
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T4 {
    #[proptest(skip, filter(foo))]
    V1(u8),
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T5 {
    #[proptest(skip, filter(foo))]
    V1 {
        field: u8
    },
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T6 {
    #[proptest(skip)]
    V1(
        #[proptest(filter(foo))]
        u8
    ),
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T7 {
    #[proptest(skip)]
    V1 {
        #[proptest(filter(foo))]
        field: usize
    },
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T8 {
    #[proptest(skip)]
    V1 {
        #[proptest(filter(foo))]
        field: usize
    },
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T9 {
    #[proptest(skip)]
    V1 {
        #[proptest(filter(foo))]
        field: usize
    },
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T10 {
    #[proptest(skip)]
    V1 {
        #[proptest(filter(foo))]
        field: usize
    },
    V2,
}
