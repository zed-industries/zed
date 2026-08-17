// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::marker::PhantomData;

use proptest::prelude::Arbitrary;
use proptest_derive::Arbitrary;

#[derive(Debug)]
struct NotArbitrary;

#[derive(Debug, Arbitrary)]
// Generic types are not in alphabetical order on purpose.
struct Foo<V, T, U> {
    v: V,
    t: T,
    u: PhantomData<U>,
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<Foo<i32, i32, NotArbitrary>>();
}
