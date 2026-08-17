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
                            //~| [proptest_derive, E0035]
                            //~| [proptest_derive, E0008]
#[proptest(skip)]
struct NonFatal {
    #[proptest(params(u8), regex = "a+")]
    field: String
}

// structs:

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
struct T0 {
    #[proptest(params(u8), regex = "a*")]
    field: String
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
struct T1 {
    #[proptest(params = "u8", regex("b+"))]
    field: String
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
struct T2(
    #[proptest(params("u8"), regex = "a|b")]
    Vec<u8>
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
struct T3(
    #[proptest(params("u8"), regex = "a+")]
    Vec<u8>
);

// enum fields:

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
enum T4 {
    V0 {
        #[proptest(params(u8), regex = "a*")]
        field: String
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
enum T5 {
    V0 {
        #[proptest(params = "u8", regex("b+"))]
        field: String
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
enum T6 {
    V0(
        #[proptest(params("u8"), regex = "a|b")]
        Vec<u8>
    )
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
enum T7 {
    V0(
        #[proptest(params("u8"), regex = "a+")]
        Vec<u8>
    )
}

// enum variants:

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
enum T8 {
    #[proptest(params(u8), regex = "a*")]
    V0 {
        field: String
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
enum T9 {
    #[proptest(params = "u8", regex("b+"))]
    V0 {
        field: String
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
enum T10 {
    #[proptest(params("u8"), regex = "a|b")]
    V0(
        Vec<u8>
    )
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0035]
enum T11 {
    #[proptest(params("u8"), regex = "a+")]
    V0(
        Vec<u8>
    )
}
