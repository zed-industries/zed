// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;

pub use core_foundation_sys::base::*;

use crate::string::CFString;
use crate::ConcreteCFType;

pub trait CFIndexConvertible {
    /// Always use this method to construct a `CFIndex` value. It performs bounds checking to
    /// ensure the value is in range.
    fn to_CFIndex(self) -> CFIndex;
}

impl CFIndexConvertible for usize {
    #[inline]
    fn to_CFIndex(self) -> CFIndex {
        let max_CFIndex = CFIndex::max_value();
        if self > (max_CFIndex as usize) {
            panic!("value out of range")
        }
        self as CFIndex
    }
}

declare_TCFType! {
    /// Superclass of all Core Foundation objects.
    CFType, CFTypeRef
}

impl CFType {
    /// Try to downcast the `CFType` to a subclass. Checking if the instance is the
    /// correct subclass happens at runtime and `None` is returned if it is not the correct type.
    /// Works similar to [`Box::downcast`] and [`CFPropertyList::downcast`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use core_foundation::string::CFString;
    /// # use core_foundation::boolean::CFBoolean;
    /// # use core_foundation::base::{CFType, TCFType};
    /// #
    /// // Create a string.
    /// let string: CFString = CFString::from_static_string("FooBar");
    /// // Cast it up to a CFType.
    /// let cf_type: CFType = string.as_CFType();
    /// // Cast it down again.
    /// assert_eq!(cf_type.downcast::<CFString>().unwrap().to_string(), "FooBar");
    /// // Casting it to some other type will yield `None`
    /// assert!(cf_type.downcast::<CFBoolean>().is_none());
    /// ```
    ///
    /// ```compile_fail
    /// # use core_foundation::array::CFArray;
    /// # use core_foundation::base::TCFType;
    /// # use core_foundation::boolean::CFBoolean;
    /// # use core_foundation::string::CFString;
    /// #
    /// let boolean_array = CFArray::from_CFTypes(&[CFBoolean::true_value()]).into_CFType();
    ///
    /// // This downcast is not allowed and causes compiler error, since it would cause undefined
    /// // behavior to access the elements of the array as a CFString:
    /// let invalid_string_array = boolean_array
    ///     .downcast_into::<CFArray<CFString>>()
    ///     .unwrap();
    /// ```
    ///
    /// [`Box::downcast`]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast
    /// [`CFPropertyList::downcast`]: ../propertylist/struct.CFPropertyList.html#method.downcast
    #[inline]
    pub fn downcast<T: ConcreteCFType>(&self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let reference = T::Ref::from_void_ptr(self.0);
                Some(T::wrap_under_get_rule(reference))
            }
        } else {
            None
        }
    }

    /// Similar to [`downcast`], but consumes self and can thus avoid touching the retain count.
    ///
    /// [`downcast`]: #method.downcast
    #[inline]
    pub fn downcast_into<T: ConcreteCFType>(self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let reference = T::Ref::from_void_ptr(self.0);
                mem::forget(self);
                Some(T::wrap_under_create_rule(reference))
            }
        } else {
            None
        }
    }
}

impl fmt::Debug for CFType {
    /// Formats the value using [`CFCopyDescription`].
    ///
    /// [`CFCopyDescription`]: https://developer.apple.com/documentation/corefoundation/1521252-cfcopydescription?language=objc
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = unsafe { CFString::wrap_under_create_rule(CFCopyDescription(self.0)) };
        desc.fmt(f)
    }
}

impl Clone for CFType {
    #[inline]
    fn clone(&self) -> CFType {
        unsafe { TCFType::wrap_under_get_rule(self.0) }
    }
}

impl PartialEq for CFType {
    #[inline]
    fn eq(&self, other: &CFType) -> bool {
        unsafe { CFEqual(self.as_CFTypeRef(), other.as_CFTypeRef()) != 0 }
    }
}

declare_TCFType!(CFAllocator, CFAllocatorRef);
impl_TCFType!(CFAllocator, CFAllocatorRef, CFAllocatorGetTypeID);

impl CFAllocator {
    #[inline]
    pub fn new(mut context: CFAllocatorContext) -> CFAllocator {
        unsafe {
            let allocator_ref = CFAllocatorCreate(kCFAllocatorDefault, &mut context);
            TCFType::wrap_under_create_rule(allocator_ref)
        }
    }
}

/// All Core Foundation types implement this trait. The associated type `Ref` specifies the
/// associated Core Foundation type: e.g. for `CFType` this is `CFTypeRef`; for `CFArray` this is
/// `CFArrayRef`.
///
/// Most structs that implement this trait will do so via the [`impl_TCFType`] macro.
///
/// [`impl_TCFType`]: ../macro.impl_TCFType.html
pub trait TCFType {
    /// The reference type wrapped inside this type.
    type Ref: TCFTypeRef;

    /// Returns the object as its concrete `TypeRef`.
    fn as_concrete_TypeRef(&self) -> Self::Ref;

    /// Returns an instance of the object, wrapping the underlying `CFTypeRef` subclass. Use this
    /// when following Core Foundation's "Create Rule". The reference count is *not* bumped.
    unsafe fn wrap_under_create_rule(obj: Self::Ref) -> Self;

    /// Returns the type ID for this class.
    fn type_id() -> CFTypeID;

    /// Returns the object as a wrapped `CFType`. The reference count is incremented by one.
    #[inline]
    fn as_CFType(&self) -> CFType {
        unsafe { TCFType::wrap_under_get_rule(self.as_CFTypeRef()) }
    }

    /// Returns the object as a wrapped `CFType`. Consumes self and avoids changing the reference
    /// count.
    #[inline]
    fn into_CFType(self) -> CFType
    where
        Self: Sized,
    {
        let reference = self.as_CFTypeRef();
        mem::forget(self);
        unsafe { TCFType::wrap_under_create_rule(reference) }
    }

    /// Returns the object as a raw `CFTypeRef`. The reference count is not adjusted.
    fn as_CFTypeRef(&self) -> CFTypeRef;

    /// Returns an instance of the object, wrapping the underlying `CFTypeRef` subclass. Use this
    /// when following Core Foundation's "Get Rule". The reference count *is* bumped.
    unsafe fn wrap_under_get_rule(reference: Self::Ref) -> Self;

    /// Returns the reference count of the object. It is unwise to do anything other than test
    /// whether the return value of this method is greater than zero.
    #[inline]
    fn retain_count(&self) -> CFIndex {
        unsafe { CFGetRetainCount(self.as_CFTypeRef()) }
    }

    /// Returns the type ID of this object.
    #[inline]
    fn type_of(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_CFTypeRef()) }
    }

    /// Writes a debugging version of this object on standard error.
    fn show(&self) {
        unsafe { CFShow(self.as_CFTypeRef()) }
    }

    /// Returns `true` if this value is an instance of another type.
    #[inline]
    fn instance_of<OtherCFType: TCFType>(&self) -> bool {
        self.type_of() == OtherCFType::type_id()
    }
}

impl TCFType for CFType {
    type Ref = CFTypeRef;

    #[inline]
    fn as_concrete_TypeRef(&self) -> CFTypeRef {
        self.0
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CFTypeRef) -> CFType {
        assert!(!reference.is_null(), "Attempted to create a NULL object.");
        let reference: CFTypeRef = CFRetain(reference);
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        self.as_concrete_TypeRef()
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: CFTypeRef) -> CFType {
        assert!(!obj.is_null(), "Attempted to create a NULL object.");
        CFType(obj)
    }

    #[inline]
    fn type_id() -> CFTypeID {
        // FIXME(pcwalton): Is this right?
        0
    }
}

/// A reference to an element inside a container
pub struct ItemRef<'a, T: 'a>(ManuallyDrop<T>, PhantomData<&'a T>);

impl<'a, T> Deref for ItemRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for ItemRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl<'a, T: PartialEq> PartialEq for ItemRef<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

/// A reference to a mutable element inside a container
pub struct ItemMutRef<'a, T: 'a>(ManuallyDrop<T>, PhantomData<&'a T>);

impl<'a, T> Deref for ItemMutRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<'a, T> DerefMut for ItemMutRef<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for ItemMutRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl<'a, T: PartialEq> PartialEq for ItemMutRef<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

/// A trait describing how to convert from the stored `*mut c_void` to the desired `T`
pub unsafe trait FromMutVoid {
    unsafe fn from_mut_void<'a>(x: *mut c_void) -> ItemMutRef<'a, Self>
    where
        Self: std::marker::Sized;
}

unsafe impl FromMutVoid for u32 {
    unsafe fn from_mut_void<'a>(x: *mut c_void) -> ItemMutRef<'a, Self> {
        ItemMutRef(ManuallyDrop::new(x as u32), PhantomData)
    }
}

unsafe impl FromMutVoid for *const c_void {
    unsafe fn from_mut_void<'a>(x: *mut c_void) -> ItemMutRef<'a, Self> {
        ItemMutRef(ManuallyDrop::new(x), PhantomData)
    }
}

unsafe impl<T: TCFType> FromMutVoid for T {
    unsafe fn from_mut_void<'a>(x: *mut c_void) -> ItemMutRef<'a, Self> {
        ItemMutRef(
            ManuallyDrop::new(TCFType::wrap_under_create_rule(T::Ref::from_void_ptr(x))),
            PhantomData,
        )
    }
}

/// A trait describing how to convert from the stored `*const c_void` to the desired `T`
pub unsafe trait FromVoid {
    unsafe fn from_void<'a>(x: *const c_void) -> ItemRef<'a, Self>
    where
        Self: std::marker::Sized;
}

unsafe impl FromVoid for u32 {
    unsafe fn from_void<'a>(x: *const c_void) -> ItemRef<'a, Self> {
        // Functions like CGFontCopyTableTags treat the void*'s as u32's
        // so we convert by casting directly
        ItemRef(ManuallyDrop::new(x as u32), PhantomData)
    }
}

unsafe impl FromVoid for *const c_void {
    unsafe fn from_void<'a>(x: *const c_void) -> ItemRef<'a, Self> {
        ItemRef(ManuallyDrop::new(x), PhantomData)
    }
}

unsafe impl<T: TCFType> FromVoid for T {
    unsafe fn from_void<'a>(x: *const c_void) -> ItemRef<'a, Self> {
        ItemRef(
            ManuallyDrop::new(TCFType::wrap_under_create_rule(T::Ref::from_void_ptr(x))),
            PhantomData,
        )
    }
}

/// A trait describing how to convert from the stored `*const c_void` to the desired `T`
pub unsafe trait ToVoid<T> {
    fn to_void(&self) -> *const c_void;
}

unsafe impl ToVoid<*const c_void> for *const c_void {
    fn to_void(&self) -> *const c_void {
        *self
    }
}

unsafe impl<'a> ToVoid<CFType> for &'a CFType {
    fn to_void(&self) -> *const ::std::os::raw::c_void {
        self.as_concrete_TypeRef().as_void_ptr()
    }
}

unsafe impl ToVoid<CFType> for CFType {
    fn to_void(&self) -> *const ::std::os::raw::c_void {
        self.as_concrete_TypeRef().as_void_ptr()
    }
}

unsafe impl ToVoid<CFType> for CFTypeRef {
    fn to_void(&self) -> *const ::std::os::raw::c_void {
        self.as_void_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boolean::CFBoolean;
    use std::mem;

    #[test]
    fn cftype_instance_of() {
        let string = CFString::from_static_string("foo");
        let cftype = string.as_CFType();

        assert!(cftype.instance_of::<CFString>());
        assert!(!cftype.instance_of::<CFBoolean>());
    }

    #[test]
    fn as_cftype_retain_count() {
        let string = CFString::from_static_string("alongerstring");
        assert_eq!(string.retain_count(), 1);
        let cftype = string.as_CFType();
        assert_eq!(cftype.retain_count(), 2);
        mem::drop(string);
        assert_eq!(cftype.retain_count(), 1);
    }

    #[test]
    fn into_cftype_retain_count() {
        let string = CFString::from_static_string("alongerstring");
        assert_eq!(string.retain_count(), 1);
        let cftype = string.into_CFType();
        assert_eq!(cftype.retain_count(), 1);
    }

    #[test]
    fn as_cftype_and_downcast() {
        let string = CFString::from_static_string("alongerstring");
        let cftype = string.as_CFType();
        let string2 = cftype.downcast::<CFString>().unwrap();
        assert_eq!(string2.to_string(), "alongerstring");

        assert_eq!(string.retain_count(), 3);
        assert_eq!(cftype.retain_count(), 3);
        assert_eq!(string2.retain_count(), 3);
    }

    #[test]
    fn into_cftype_and_downcast_into() {
        let string = CFString::from_static_string("alongerstring");
        let cftype = string.into_CFType();
        let string2 = cftype.downcast_into::<CFString>().unwrap();
        assert_eq!(string2.to_string(), "alongerstring");
        assert_eq!(string2.retain_count(), 1);
    }
}
