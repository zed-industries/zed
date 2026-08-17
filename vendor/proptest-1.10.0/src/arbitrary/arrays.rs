//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for arrays.

use crate::arbitrary::{any_with, Arbitrary};
use crate::array::UniformArrayStrategy;

impl<A: Arbitrary, const N: usize> Arbitrary for [A; N] {
    type Parameters = A::Parameters;
    type Strategy = UniformArrayStrategy<A::Strategy, [A; N]>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let base = any_with::<A>(args);
        UniformArrayStrategy::new(base)
    }
}

#[cfg(test)]
mod test {
    no_panic_test!(
        array_16 => [u8; 16]
    );

    no_panic_test!(
        array_1024 => [u8; 1024]
    );
}
