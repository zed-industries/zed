// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Heterogeneous immutable arrays.

use crate::ConcreteCFType;
pub use core_foundation_sys::array::*;
pub use core_foundation_sys::base::CFIndex;
use core_foundation_sys::base::{kCFAllocatorDefault, CFRelease, CFTypeRef};
use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use crate::base::{CFIndexConvertible, CFRange, TCFType};
use crate::base::{FromVoid, ItemRef};

/// A heterogeneous immutable array.
pub struct CFArray<T = *const c_void>(CFArrayRef, PhantomData<T>);

impl<T> Drop for CFArray<T> {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

pub struct CFArrayIterator<'a, T: 'a> {
    array: &'a CFArray<T>,
    index: CFIndex,
    len: CFIndex,
}

impl<'a, T: FromVoid> Iterator for CFArrayIterator<'a, T> {
    type Item = ItemRef<'a, T>;

    fn next(&mut self) -> Option<ItemRef<'a, T>> {
        if self.index >= self.len {
            None
        } else {
            let value = unsafe { self.array.get_unchecked(self.index) };
            self.index += 1;
            Some(value)
        }
    }
}

impl<'a, T: FromVoid> ExactSizeIterator for CFArrayIterator<'a, T> {
    fn len(&self) -> usize {
        (self.array.len() - self.index) as usize
    }
}

impl_TCFType!(CFArray<T>, CFArrayRef, CFArrayGetTypeID);
impl_CFTypeDescription!(CFArray<T>);

unsafe impl ConcreteCFType for CFArray<*const c_void> {}

impl<T> CFArray<T> {
    /// Creates a new `CFArray` with the given elements, which must implement `Copy`.
    pub fn from_copyable(elems: &[T]) -> CFArray<T>
    where
        T: Copy,
    {
        unsafe {
            let array_ref = CFArrayCreate(
                kCFAllocatorDefault,
                elems.as_ptr() as *const *const c_void,
                elems.len().to_CFIndex(),
                ptr::null(),
            );
            TCFType::wrap_under_create_rule(array_ref)
        }
    }

    /// Creates a new `CFArray` with the given elements, which must be `CFType` objects.
    pub fn from_CFTypes(elems: &[T]) -> CFArray<T>
    where
        T: TCFType,
    {
        unsafe {
            let elems: Vec<CFTypeRef> = elems.iter().map(|elem| elem.as_CFTypeRef()).collect();
            let array_ref = CFArrayCreate(
                kCFAllocatorDefault,
                elems.as_ptr(),
                elems.len().to_CFIndex(),
                &kCFTypeArrayCallBacks,
            );
            TCFType::wrap_under_create_rule(array_ref)
        }
    }

    #[inline]
    pub fn to_untyped(&self) -> CFArray {
        unsafe { CFArray::wrap_under_get_rule(self.0) }
    }

    /// Returns the same array, but with the type reset to void pointers.
    /// Equal to `to_untyped`, but is faster since it does not increment the retain count.
    #[inline]
    pub fn into_untyped(self) -> CFArray {
        let reference = self.0;
        mem::forget(self);
        unsafe { CFArray::wrap_under_create_rule(reference) }
    }

    /// Iterates over the elements of this `CFArray`.
    ///
    /// Careful; the loop body must wrap the reference properly. Generally, when array elements are
    /// Core Foundation objects (not always true), they need to be wrapped with
    /// `TCFType::wrap_under_get_rule()`.
    #[inline]
    pub fn iter(&self) -> CFArrayIterator<'_, T> {
        CFArrayIterator {
            array: self,
            index: 0,
            len: self.len(),
        }
    }

    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe { CFArrayGetCount(self.0) }
    }

    /// Returns `true` if the array contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: CFIndex) -> ItemRef<'_, T>
    where
        T: FromVoid,
    {
        T::from_void(CFArrayGetValueAtIndex(self.0, index))
    }

    #[inline]
    pub fn get(&self, index: CFIndex) -> Option<ItemRef<'_, T>>
    where
        T: FromVoid,
    {
        if index < self.len() {
            Some(unsafe { T::from_void(CFArrayGetValueAtIndex(self.0, index)) })
        } else {
            None
        }
    }

    pub fn get_values(&self, range: CFRange) -> Vec<*const c_void> {
        let mut vec = Vec::with_capacity(range.length as usize);
        unsafe {
            CFArrayGetValues(self.0, range, vec.as_mut_ptr());
            vec.set_len(range.length as usize);
            vec
        }
    }

    pub fn get_all_values(&self) -> Vec<*const c_void> {
        self.get_values(CFRange {
            location: 0,
            length: self.len(),
        })
    }
}

impl<'a, T: FromVoid> IntoIterator for &'a CFArray<T> {
    type Item = ItemRef<'a, T>;
    type IntoIter = CFArrayIterator<'a, T>;

    fn into_iter(self) -> CFArrayIterator<'a, T> {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::number::CFNumber;

    use super::*;
    use crate::base::CFType;
    use std::mem;

    #[test]
    fn to_untyped_correct_retain_count() {
        let array = CFArray::<CFType>::from_CFTypes(&[CFNumber::from(4).as_CFType()]);
        assert_eq!(array.retain_count(), 1);

        let untyped_array = array.to_untyped();
        assert_eq!(array.retain_count(), 2);
        assert_eq!(untyped_array.retain_count(), 2);

        mem::drop(array);
        assert_eq!(untyped_array.retain_count(), 1);
    }

    #[test]
    fn into_untyped() {
        let array = CFArray::<CFType>::from_CFTypes(&[CFNumber::from(4).as_CFType()]);
        let array2 = array.to_untyped();
        assert_eq!(array.retain_count(), 2);

        let untyped_array = array.into_untyped();
        assert_eq!(untyped_array.retain_count(), 2);

        mem::drop(array2);
        assert_eq!(untyped_array.retain_count(), 1);
    }

    #[test]
    fn borrow() {
        use crate::string::CFString;

        let string = CFString::from_static_string("alongerstring");
        assert_eq!(string.retain_count(), 1);
        let x;
        {
            let arr: CFArray<CFString> = CFArray::from_CFTypes(&[string]);
            {
                let p = arr.get(0).unwrap();
                assert_eq!(p.retain_count(), 1);
            }
            {
                x = arr.get(0).unwrap().clone();
                assert_eq!(x.retain_count(), 2);
                assert_eq!(x.to_string(), "alongerstring");
            }
        }
        assert_eq!(x.retain_count(), 1);
    }

    #[test]
    fn iter_untyped_array() {
        use crate::base::TCFTypeRef;
        use crate::string::{CFString, CFStringRef};

        let cf_string = CFString::from_static_string("alongerstring");
        let array: CFArray = CFArray::from_CFTypes(&[cf_string.clone()]).into_untyped();

        let cf_strings = array
            .iter()
            .map(|ptr| unsafe { CFString::wrap_under_get_rule(CFStringRef::from_void_ptr(*ptr)) })
            .collect::<Vec<_>>();
        let strings = cf_strings.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(cf_string.retain_count(), 3);
        assert_eq!(&strings[..], &["alongerstring"]);
    }

    #[test]
    fn should_box_and_unbox() {
        use crate::number::CFNumber;

        let n0 = CFNumber::from(0);
        let n1 = CFNumber::from(1);
        let n2 = CFNumber::from(2);
        let n3 = CFNumber::from(3);
        let n4 = CFNumber::from(4);
        let n5 = CFNumber::from(5);

        let arr = CFArray::from_CFTypes(&[
            n0.as_CFType(),
            n1.as_CFType(),
            n2.as_CFType(),
            n3.as_CFType(),
            n4.as_CFType(),
            n5.as_CFType(),
        ]);

        assert_eq!(
            arr.get_all_values(),
            &[
                n0.as_CFTypeRef(),
                n1.as_CFTypeRef(),
                n2.as_CFTypeRef(),
                n3.as_CFTypeRef(),
                n4.as_CFTypeRef(),
                n5.as_CFTypeRef()
            ]
        );

        let mut sum = 0;

        let mut iter = arr.iter();
        assert_eq!(iter.len(), 6);
        assert!(iter.next().is_some());
        assert_eq!(iter.len(), 5);

        for elem in iter {
            let number: CFNumber = elem.downcast::<CFNumber>().unwrap();
            sum += number.to_i64().unwrap()
        }

        assert_eq!(sum, 15);

        for elem in arr.iter() {
            let number: CFNumber = elem.downcast::<CFNumber>().unwrap();
            sum += number.to_i64().unwrap()
        }

        assert_eq!(sum, 30);
    }
}
