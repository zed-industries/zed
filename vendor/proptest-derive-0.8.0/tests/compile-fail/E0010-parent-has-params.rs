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

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors
                            //~| [proptest_derive, E0010]
                            //~| [proptest_derive, E0008]
#[proptest(no_params)]
struct NonFatal<#[proptest(skip)] T> {
    #[proptest(no_params)]
    field: T
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
struct T0 {
    #[proptest(no_params)]
    field: String
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
struct T1(
    #[proptest(no_params)]
    String
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(params = "u8")]
struct T2 {
    #[proptest(no_params)]
    bar: String
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(params = "usize")]
struct T3(
    #[proptest(no_params)]
    String
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
struct T4 {
    #[proptest(params = "usize")]
    baz: String
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
struct T5(
    #[proptest(params = "String")]
    String
);

#[derive(Debug, Arbitrary)] // ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
enum T6 {
    #[proptest(params = "String")]
    V0(u8),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
enum T7 {
    #[proptest(no_params)]
    V0(
        #[proptest(params = "String")]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
enum T8 {
    V0(
        #[proptest(params = "String")]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(params = "String")]
enum T9 {
    V0(
        #[proptest(no_params)]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(params = "String")]
enum T10 {
    V0 {
        #[proptest(no_params)]
        batman: u8
    },
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
enum T11 {
    V0 {
        #[proptest(params = "String")]
        batman: u8
    },
}
