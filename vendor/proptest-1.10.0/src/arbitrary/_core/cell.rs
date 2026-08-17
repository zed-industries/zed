//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::cell`.

use core::cell::{BorrowError, BorrowMutError, Cell, RefCell, UnsafeCell};

wrap_from!([Copy] Cell);
wrap_from!(RefCell);
wrap_from!(UnsafeCell);

lazy_just!(BorrowError, || {
    // False positive:
    #[cfg_attr(clippy, allow(let_and_return))]
    {
        let _rc = RefCell::new(());
        let _bm = _rc.borrow_mut();
        let _tb = _rc.try_borrow();
        let ret = _rc.try_borrow().expect_err("reborrowed RefCell");
        ret
    }
});
lazy_just!(BorrowMutError, || {
    // False positive:
    #[cfg_attr(clippy, allow(let_and_return))]
    {
        let _rc = RefCell::new(());
        let _bm = _rc.borrow_mut();
        let _tb = _rc.try_borrow();
        let ret = _rc.try_borrow_mut().expect_err("reborrowed RefCell");
        ret
    }
});

#[cfg(test)]
mod test {
    no_panic_test!(
        cell => Cell<u8>,
        ref_cell => RefCell<u8>,
        unsafe_cell => UnsafeCell<u8>
    );
}
