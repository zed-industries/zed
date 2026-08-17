// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::Arbitrary;
use proptest_derive::Arbitrary;

#[derive(Debug)]
struct NotArbitrary;

/// Ensure that we can't determine that this is PhantomData syntactically.
type HidePH<T> = ::std::marker::PhantomData<T>;

/*
// TODO handle this...

#[derive(Debug, Arbitrary)]
struct T1<#[proptest(no_bound)] T>(HidePH<T>);

#[derive(Debug, Arbitrary)]
struct T2(T1<NotArbitrary>);

#[derive(Debug, Arbitrary)]
struct T3<
    #[proptest(no_bound)] A,
    B,
    #[proptest(no_bound)] G,
> {
    alpha: HidePH<A>,
    beta: B,
    gamma: HidePH<G>,
}

#[derive(Debug, Arbitrary)]
struct T4(T3<NotArbitrary, bool, NotArbitrary>);
*/

#[derive(Debug, Arbitrary)]
#[proptest(no_bound)]
struct T5<A, B, C>(HidePH<(A, B, C)>);

#[derive(Debug, Arbitrary)]
struct T6(T5<NotArbitrary, NotArbitrary, NotArbitrary>);

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    /*
    assert_arbitrary::<T2>();
    assert_arbitrary::<T4>();
    */
    assert_arbitrary::<T6>();
}
