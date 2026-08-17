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

#[derive(Debug)]
struct T0;

#[derive(Debug, Arbitrary)] //~ the trait bound `T0: Arbitrary` is not satisfied [E0277]
struct T1 {
    f0: T0, //~ the trait bound `T0: Arbitrary` is not satisfied [E0277]
            //~^ the trait bound `T0: Arbitrary` is not satisfied [E0277]
}
