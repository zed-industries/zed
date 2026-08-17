// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::Arbitrary;
use proptest_derive::Arbitrary;

use std::marker;
use std::marker::PhantomData;

#[derive(Debug)]
struct NotArbitrary;

#[derive(Debug, Arbitrary)]
struct T1<T>(::std::marker::PhantomData<T>);

#[derive(Debug, Arbitrary)]
struct T2(T1<NotArbitrary>);

#[derive(Debug, Arbitrary)]
struct T3<T>(marker::PhantomData<T>);

#[derive(Debug, Arbitrary)]
struct T4(T3<NotArbitrary>);

#[derive(Debug, Arbitrary)]
struct T5<T>(PhantomData<T>);

#[derive(Debug, Arbitrary)]
struct T6(T5<NotArbitrary>);

#[derive(Debug, Arbitrary)]
struct T7<T>(std::marker::PhantomData<T>);

#[derive(Debug, Arbitrary)]
struct T8(T7<NotArbitrary>);

#[derive(Debug, Arbitrary)]
struct T9<A, B, C> {
    _a: A,
    _b: B,
    c: PhantomData<C>,
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<T1<NotArbitrary>>();
    assert_arbitrary::<T2>();
    assert_arbitrary::<T3<NotArbitrary>>();
    assert_arbitrary::<T4>();
    assert_arbitrary::<T5<NotArbitrary>>();
    assert_arbitrary::<T6>();
    assert_arbitrary::<T7<NotArbitrary>>();
    assert_arbitrary::<T8>();
    assert_arbitrary::<T9<u8, usize, NotArbitrary>>();
}
