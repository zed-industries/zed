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

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T0 {
    V1 {
        #[proptest(strategy = "random garbage")]
        field: u8,
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T1 {
    V1(
        #[proptest(strategy = "random garbage")]
        u8,
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T2 {
    #[proptest(strategy = "random garbage")]
    field: String,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T3(
    #[proptest(strategy = "random garbage")]
    String
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T4 {
    V1 {
        #[proptest(value = "random garbage")]
        field: u8,
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T5 {
    V1(
        #[proptest(value = "random garbage")]
        u8,
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T6 {
    #[proptest(value = "random garbage")]
    field: String,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T7(
    #[proptest(value = "random garbage")]
    String
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T8 {
    V1 {
        #[proptest(strategy("random garbage"))]
        field: u8,
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T9 {
    V1(
        #[proptest(strategy("random garbage"))]
        u8,
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T10 {
    #[proptest(strategy("random garbage"))]
    field: String,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T11(
    #[proptest(strategy("random garbage"))]
    String
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T12 {
    V1 {
        #[proptest(value("random garbage"))]
        field: u8,
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
enum T13 {
    V1(
        #[proptest(value("random garbage"))]
        u8,
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T14 {
    #[proptest(value("random garbage"))]
    field: String,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0026]
struct T15(
    #[proptest(value("random garbage"))]
    String
);
