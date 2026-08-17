// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An immutable bag of elements.

use core_foundation_sys::base::{kCFAllocatorDefault, CFRelease, CFTypeRef};
pub use core_foundation_sys::set::*;

use crate::base::{CFIndexConvertible, TCFType};

use std::marker::PhantomData;
use std::os::raw::c_void;

/// An immutable bag of elements.
pub struct CFSet<T = *const c_void>(CFSetRef, PhantomData<T>);

impl<T> Drop for CFSet<T> {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(CFSet<T>, CFSetRef, CFSetGetTypeID);
impl_CFTypeDescription!(CFSet);

impl CFSet {
    /// Creates a new set from a list of `CFType` instances.
    pub fn from_slice<T>(elems: &[T]) -> CFSet<T>
    where
        T: TCFType,
    {
        unsafe {
            let elems: Vec<CFTypeRef> = elems.iter().map(|elem| elem.as_CFTypeRef()).collect();
            let set_ref = CFSetCreate(
                kCFAllocatorDefault,
                elems.as_ptr(),
                elems.len().to_CFIndex(),
                &kCFTypeSetCallBacks,
            );
            TCFType::wrap_under_create_rule(set_ref)
        }
    }
}

impl<T> CFSet<T> {
    /// Get the number of elements in the `CFSet`.
    pub fn len(&self) -> usize {
        unsafe { CFSetGetCount(self.0) as usize }
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
