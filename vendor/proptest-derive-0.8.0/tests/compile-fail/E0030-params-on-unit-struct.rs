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

// It happens that no other error will follow E0030 so this is not as proper
// a check that we wanted to ensure that E0030 is non-fatal.

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors
                            //~| [proptest_derive, E0008]
                            //~| [proptest_derive, E0030]
#[proptest(params = "u8")]
#[proptest(skip)]
struct T0;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0030]
#[proptest(no_params)]
struct T1;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0030]
#[proptest(params = "u8")]
struct T2 {}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0030]
#[proptest(no_params)]
struct T3 {}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0030]
#[proptest(params = "u8")]
struct T4();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0030]
#[proptest(no_params)]
struct T5();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0030]
#[proptest(filter(foo))]
struct T6;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0030]
#[proptest(filter(foo))]
struct T7();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0030]
#[proptest(filter(foo))]
struct T8 {}
