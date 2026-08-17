//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::marker`.

use core::marker::PhantomData;

arbitrary!([T: ?Sized] PhantomData<T>; PhantomData);

#[cfg(test)]
mod test {
    no_panic_test!(phantom_data => PhantomData<u8>);
}
