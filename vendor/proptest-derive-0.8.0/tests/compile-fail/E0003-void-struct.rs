// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(never_type)]

extern crate proptest_derive;
use proptest_derive::Arbitrary;

fn main() {}

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0003]
                            //~| [proptest_derive, E0008]
struct NonFatal {
    #[proptest(skip)]
    x: !,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0003]
struct Ty0 { x: ! }

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0003]
struct Ty1 {
    x: usize,
    y: !,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0003]
struct Ty2 {
    x: (!, usize),
    y: bool,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0003]
struct Ty3 {
    x: [!; 1]
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0003]
struct Ty4 {
    x: [::std::string::ParseError; 1],
}
