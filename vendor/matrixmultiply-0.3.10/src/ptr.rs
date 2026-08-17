// Copyright 2020, 2022 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rawpointer::PointerExt;

/// A Send + Sync raw pointer wrapper
#[derive(Copy, Clone)]
#[repr(transparent)]
pub(crate) struct Ptr<T> { ptr: T }
unsafe impl<T> Sync for Ptr<*const T> { }
unsafe impl<T> Sync for Ptr<*mut T> { }
unsafe impl<T> Send for Ptr<*const T> { }
unsafe impl<T> Send for Ptr<*mut T> { }

/// Create a Ptr
///
/// # Safety
///
/// Unsafe since it is thread safety critical to use the raw pointer correctly.
#[allow(non_snake_case)]
pub(crate) unsafe fn Ptr<T>(ptr: T) -> Ptr<T> { Ptr { ptr } }

impl<T> Ptr<T> {
    /// Get the pointer
    pub(crate) fn ptr(self) -> T
        where T: Copy
    {
        self.ptr
    }
}

impl<T> Ptr<*mut T> {
    /// Get as *const T
    pub(crate) fn to_const(self) -> Ptr<*const T> {
        Ptr { ptr: self.ptr }
    }
}

impl<T> PointerExt for Ptr<*const T> {
    #[inline(always)]
    unsafe fn offset(self, i: isize) -> Self {
        Ptr(self.ptr.offset(i))
    }
}

impl<T> PointerExt for Ptr<*mut T> {
    #[inline(always)]
    unsafe fn offset(self, i: isize) -> Self {
        Ptr(self.ptr.offset(i))
    }
}
