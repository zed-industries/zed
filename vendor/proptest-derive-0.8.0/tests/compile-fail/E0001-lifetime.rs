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

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0001]
                            //~| [proptest_derive, E0008]
#[proptest(skip)]
struct NonFatal<'a>(&'a ());

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0001]
struct T0<'a>(&'a ());

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0001]
enum T1<'a> {
    V0(&'a ())
}
