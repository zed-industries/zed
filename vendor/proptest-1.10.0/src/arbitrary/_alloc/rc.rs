//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::rc`.

use crate::std_facade::Rc;

// Weak would always give None on upgrade since there's no owned Rc.

wrap_from!(Rc);

#[cfg(test)]
mod test {
    no_panic_test!(rc => Rc<u8>);
}
