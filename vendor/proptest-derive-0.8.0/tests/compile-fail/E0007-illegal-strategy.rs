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
                            //~| [proptest_derive, E0007]
                            //~| [proptest_derive, E0030]
#[proptest(params = "u8")]
#[proptest(strategy = "1u8..")]
struct A {}

// strategy:

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(strategy = "1u8..")]
struct B;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(strategy = "1u8..")]
struct C();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(strategy = "1u8..")]
struct D { field: String }

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(strategy = "1u8..")]
struct E(Vec<u8>);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(strategy = "1u8..")]
enum F { V1, V2, }

// value:

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(value = "1")]
struct G;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(value = "2")]
struct H();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(value = "1 + 2")]
struct I { field: String }

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(value = "2 * 3")]
struct J(Vec<u8>);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(value = "1..2")]
enum K { V1, V2, }

// regex:

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(regex = "1")]
struct L;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(regex = "\\d\\d")]
struct M();

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(regex = "3")]
struct N { field: String }

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(regex = "a+")]
struct O(Vec<u8>);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0007]
#[proptest(regex = "b*")]
enum P { V1, V2, }
