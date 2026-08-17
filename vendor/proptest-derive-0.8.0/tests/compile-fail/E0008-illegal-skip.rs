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

#[derive(Clone, Debug, Arbitrary)] //~ ERROR: 2 errors:
                                   //~| [proptest_derive, E0008]
                                   //~| [proptest_derive, E0007]
#[proptest(skip)]
#[proptest(strategy = "Just(A {})")]
struct A {}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0008]
#[proptest(skip)]
struct B;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0008]
#[proptest(skip)]
struct C();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0008]
#[proptest(skip)]
struct D { field: String }

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0008]
#[proptest(skip)]
struct E(Vec<u8>);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0008]
#[proptest(skip)]
enum F { V1, V2, }

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0008]
struct G(
    #[proptest(skip)]
    Vec<u8>
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0008]
struct H {
    #[proptest(skip)]
    field: Vec<u8>
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0008]
enum I {
    V0 {
        #[proptest(skip)]
        field: Vec<u8>
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0008]
enum J {
    V0(#[proptest(skip)] Vec<u8>)
}
