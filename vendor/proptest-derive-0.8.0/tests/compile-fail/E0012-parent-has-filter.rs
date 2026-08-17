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

fn even(x: &u8) -> bool {
    x % 2 == 0
}

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors
                            //~| [proptest_derive, E0012]
                            //~| [proptest_derive, E0008]
enum NonFatal<#[proptest(skip)] T> {
    #[proptest(strategy = "(0..10u8).prop_map(T0::V0)")]
    V0(
        #[proptest(filter(even))]
        u8,
        T
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T0 {
    #[proptest(strategy = "(0..10u8).prop_map(T0::V0)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T1 {
    #[proptest(strategy = "(0..10u8).prop_map(|field| T1::V0 { field })")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T2 {
    #[proptest(value = "T2::V0(1)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T3 {
    #[proptest(value = "T3::V0 { field: 1 }")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
#[proptest(no_params)]
enum T4 {
    #[proptest(strategy = "(0..10u8).prop_map(T4::V0)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
#[proptest(no_params)]
enum T5 {
    #[proptest(strategy = "(0..10u8).prop_map(|field| T5::V0 { field })")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
#[proptest(no_params)]
enum T6 {
    #[proptest(value = "T6::V0(1)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
#[proptest(no_params)]
enum T7 {
    #[proptest(value = "T7::V0 { field: 1 }")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}

struct Unit;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
#[proptest(params(Unit))]
enum T8 {
    #[proptest(strategy = "(0..10u8).prop_map(T8::V0)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
#[proptest(params(Unit))]
enum T9 {
    #[proptest(strategy = "(0..10u8).prop_map(|field| T9::V0 { field })")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
#[proptest(params(Unit))]
enum T10 {
    #[proptest(value = "T10::V0(1)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
#[proptest(params(Unit))]
enum T11 {
    #[proptest(value = "T11::V0 { field: 1 }")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T12 {
    #[proptest(params(Unit))]
    #[proptest(strategy = "(0..10u8).prop_map(T12::V0)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T13 {
    #[proptest(params(Unit))]
    #[proptest(strategy = "(0..10u8).prop_map(|field| T13::V0 { field })")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T14 {
    #[proptest(params(Unit))]
    #[proptest(value = "T14::V0(1)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T15 {
    #[proptest(params(Unit))]
    #[proptest(value = "T15::V0 { field: 1 }")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T16 {
    #[proptest(no_params)]
    #[proptest(strategy = "(0..10u8).prop_map(T16::V0)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T17 {
    #[proptest(no_params)]
    #[proptest(strategy = "(0..10u8).prop_map(|field| T17::V0 { field })")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T18 {
    #[proptest(no_params)]
    #[proptest(value = "T18::V0(1)")]
    V0(
        #[proptest(filter(even))]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0012]
enum T19 {
    #[proptest(no_params)]
    #[proptest(value = "T19::V0 { field: 1 }")]
    V0 {
        #[proptest(filter(even))]
        field: u8
    }
}
