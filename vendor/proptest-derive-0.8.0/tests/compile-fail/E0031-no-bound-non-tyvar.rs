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

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0031]
                            //~| [proptest_derive, E0008]
struct T1<T> {
    #[proptest(no_bound)]
    #[proptest(skip)]
    field: ::std::marker::PhantomData<T>,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0031]
struct T2<T>(
    #[proptest(no_bound)]
    ::std::marker::PhantomData<T>,
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0031]
enum T3<T> {
    #[proptest(no_bound)]
    V1(T),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0031]
enum T4<T> {
    #[proptest(no_bound)]
    V1 {
        field: T
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0031]
enum T5<T> {
    V1(
        #[proptest(no_bound)]
        T
    ),
}
