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

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
#[proptest(params = "1/2")]
enum T0 {
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
#[proptest(params = ";;;")]
struct T1 {
    field: String,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
enum T2 {
    #[proptest(params = "Vec<1 + u8>")]
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
enum T3 {
    V1 {
        #[proptest(params = "!!")]
        field: Box<str>,
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
struct T4 {
    #[proptest(params = "~")]
    field: String,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
#[proptest(params("1/2"))]
enum T5 {
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
#[proptest(params(";;;"))]
struct T6 {
    field: String,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
enum T7 {
    #[proptest(params("Vec<1 + u8>"))]
    V1
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
enum T8 {
    V1 {
        #[proptest(params("!!"))]
        field: Box<str>,
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0023]
struct T9 {
    #[proptest(params("~"))]
    field: String,
}
