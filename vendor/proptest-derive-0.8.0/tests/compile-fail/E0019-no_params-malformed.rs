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
#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0019]
                            //~| [proptest_derive, E0007]
#[proptest(no_params = 1, value("T0(u8)"))]
struct T0(u8);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0019]
#[proptest(no_params(2))]
struct T1(u8);
