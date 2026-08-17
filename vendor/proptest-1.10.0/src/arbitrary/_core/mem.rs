//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::mem`.

use core::mem::*;

use crate::arbitrary::*;
use crate::strategy::statics::static_map;

arbitrary!([A: Arbitrary] Discriminant<A>,
    SMapped<A, Self>, A::Parameters;
    args => static_map(any_with::<A>(args), |x| discriminant(&x))
);

lift1!(['static] Discriminant<A>;
    base => static_map(base, |x| discriminant(&x))
);

// Not supported at the moment since the user won't be able to call
// https://doc.rust-lang.org/nightly/std/mem/union.ManuallyDrop.html#method.drop
// in any case so the use case is not great for this.
//wrap_ctor!(ManuallyDrop);

#[cfg(test)]
mod test {
    #[derive(Copy, Clone, Debug)]
    struct DummyStruct;
    arbitrary!(DummyStruct; DummyStruct);

    no_panic_test!(
        //manually_drop       => ManuallyDrop<u8>, // Trivial destructor.
        discriminant_struct => Discriminant<super::DummyStruct>,
        discriminant_enum   => Discriminant<::std::num::FpCategory>
    );
}
