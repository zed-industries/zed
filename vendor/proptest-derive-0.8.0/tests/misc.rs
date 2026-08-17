// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::Arbitrary;
use proptest::strategy::Just;
use proptest_derive::Arbitrary;

// TODO: An idea.
/*
#[derive(Debug, Arbitrary)]
#[proptest(with = "Foo::ctor(1337, :usize:.other_fn(:f64:, #0..7#))")]
struct Foo {
    //..
}
*/

#[derive(Default)]
struct Complex;

#[derive(Debug, Arbitrary)]
#[proptest(params(Complex))]
enum Foo {
    #[proptest(value = "Foo::F0(1, 1)")]
    F0(usize, u8),
}

#[derive(Clone, Debug, Arbitrary)]
#[proptest(params = "usize")]
enum A {
    B,
    #[proptest(strategy = "Just(A::C(1))")]
    C(usize),
}

#[derive(Clone, Debug, Arbitrary)]
enum Bobby {
    #[proptest(no_params)]
    B(usize),
    #[proptest(no_params, value = "Bobby::C(1)")]
    C(usize),
    #[proptest(no_params, strategy = "Just(Bobby::D(1))")]
    D(usize),
    #[proptest(params(Complex), value = "Bobby::E(1)")]
    E(usize),
    #[proptest(params(Complex), strategy = "Just(Bobby::F(1))")]
    F(usize),
}

#[derive(Clone, Debug, Arbitrary)]
enum Quux {
    B(#[proptest(no_params)] usize),
    C(usize, String),
    #[proptest(value = "Quux::D(2, \"a\".into())")]
    D(usize, String),
    #[proptest(strategy = "Just(Quux::E(1337))")]
    E(u32),
    F {
        #[proptest(strategy = "10usize..20usize")]
        _foo: usize,
    },
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<Foo>();
    assert_arbitrary::<A>();
    assert_arbitrary::<Bobby>();
    assert_arbitrary::<Quux>();
}
