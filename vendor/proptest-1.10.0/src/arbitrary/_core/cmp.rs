//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::cmp`.

use core::cmp::{Ordering, Reverse};

use crate::strategy::{Just, TupleUnion, WA};

wrap_ctor!(Reverse, Reverse);

type WAJO = WA<Just<Ordering>>;
arbitrary!(Ordering, TupleUnion<(WAJO, WAJO, WAJO)>;
    prop_oneof![
        Just(Ordering::Equal),
        Just(Ordering::Less),
        Just(Ordering::Greater)
    ]
);

#[cfg(test)]
mod test {
    no_panic_test!(
        reverse => Reverse<u8>,
        ordering => Ordering
    );
}
