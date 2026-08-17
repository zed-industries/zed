// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation property lists

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use crate::base::{CFType, TCFType, TCFTypeRef};
use crate::data::CFData;
use crate::error::CFError;

use core_foundation_sys::base::{
    kCFAllocatorDefault, CFGetRetainCount, CFGetTypeID, CFIndex, CFRetain, CFShow, CFTypeID,
};
use core_foundation_sys::error::CFErrorRef;
pub use core_foundation_sys::propertylist::*;

pub fn create_with_data(
    data: CFData,
    options: CFPropertyListMutabilityOptions,
) -> Result<(*const c_void, CFPropertyListFormat), CFError> {
    unsafe {
        let mut error: CFErrorRef = ptr::null_mut();
        let mut format: CFPropertyListFormat = 0;
        let property_list = CFPropertyListCreateWithData(
            kCFAllocatorDefault,
            data.as_concrete_TypeRef(),
            options,
            &mut format,
            &mut error,
        );
        if property_list.is_null() {
            Err(TCFType::wrap_under_create_rule(error))
        } else {
            Ok((property_list, format))
        }
    }
}

pub fn create_data(
    property_list: *const c_void,
    format: CFPropertyListFormat,
) -> Result<CFData, CFError> {
    unsafe {
        let mut error: CFErrorRef = ptr::null_mut();
        let data_ref =
            CFPropertyListCreateData(kCFAllocatorDefault, property_list, format, 0, &mut error);
        if data_ref.is_null() {
            Err(TCFType::wrap_under_create_rule(error))
        } else {
            Ok(TCFType::wrap_under_create_rule(data_ref))
        }
    }
}

/// Trait for all subclasses of [`CFPropertyList`].
///
/// [`CFPropertyList`]: struct.CFPropertyList.html
pub trait CFPropertyListSubClass: TCFType {
    /// Create an instance of the superclass type [`CFPropertyList`] for this instance.
    ///
    /// [`CFPropertyList`]: struct.CFPropertyList.html
    #[inline]
    fn to_CFPropertyList(&self) -> CFPropertyList {
        unsafe { CFPropertyList::wrap_under_get_rule(self.as_concrete_TypeRef().as_void_ptr()) }
    }

    /// Equal to [`to_CFPropertyList`], but consumes self and avoids changing the reference count.
    ///
    /// [`to_CFPropertyList`]: #method.to_CFPropertyList
    #[inline]
    fn into_CFPropertyList(self) -> CFPropertyList
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef().as_void_ptr();
        mem::forget(self);
        unsafe { CFPropertyList::wrap_under_create_rule(reference) }
    }
}

impl CFPropertyListSubClass for crate::data::CFData {}
impl CFPropertyListSubClass for crate::string::CFString {}
impl CFPropertyListSubClass for crate::array::CFArray {}
impl CFPropertyListSubClass for crate::dictionary::CFDictionary {}
impl CFPropertyListSubClass for crate::date::CFDate {}
impl CFPropertyListSubClass for crate::boolean::CFBoolean {}
impl CFPropertyListSubClass for crate::number::CFNumber {}

declare_TCFType! {
    /// A CFPropertyList struct. This is superclass to [`CFData`], [`CFString`], [`CFArray`],
    /// [`CFDictionary`], [`CFDate`], [`CFBoolean`], and [`CFNumber`].
    ///
    /// This superclass type does not have its own `CFTypeID`, instead each instance has the `CFTypeID`
    /// of the subclass it is an instance of. Thus, this type cannot implement the [`TCFType`] trait,
    /// since it cannot implement the static [`TCFType::type_id()`] method.
    ///
    /// [`CFData`]: ../data/struct.CFData.html
    /// [`CFString`]: ../string/struct.CFString.html
    /// [`CFArray`]: ../array/struct.CFArray.html
    /// [`CFDictionary`]: ../dictionary/struct.CFDictionary.html
    /// [`CFDate`]: ../date/struct.CFDate.html
    /// [`CFBoolean`]: ../boolean/struct.CFBoolean.html
    /// [`CFNumber`]: ../number/struct.CFNumber.html
    /// [`TCFType`]: ../base/trait.TCFType.html
    /// [`TCFType::type_id()`]: ../base/trait.TCFType.html#method.type_of
    CFPropertyList, CFPropertyListRef
}

impl_CFTypeDescription!(CFPropertyList);

impl CFPropertyList {
    #[inline]
    pub fn as_concrete_TypeRef(&self) -> CFPropertyListRef {
        self.0
    }

    #[inline]
    pub unsafe fn wrap_under_get_rule(reference: CFPropertyListRef) -> CFPropertyList {
        assert!(!reference.is_null(), "Attempted to create a NULL object.");
        let reference = CFRetain(reference);
        CFPropertyList(reference)
    }

    #[inline]
    pub fn as_CFType(&self) -> CFType {
        unsafe { CFType::wrap_under_get_rule(self.as_CFTypeRef()) }
    }

    #[inline]
    pub fn into_CFType(self) -> CFType
    where
        Self: Sized,
    {
        let reference = self.as_CFTypeRef();
        mem::forget(self);
        unsafe { TCFType::wrap_under_create_rule(reference) }
    }

    #[inline]
    pub fn as_CFTypeRef(&self) -> ::core_foundation_sys::base::CFTypeRef {
        self.as_concrete_TypeRef()
    }

    #[inline]
    pub unsafe fn wrap_under_create_rule(obj: CFPropertyListRef) -> CFPropertyList {
        assert!(!obj.is_null(), "Attempted to create a NULL object.");
        CFPropertyList(obj)
    }

    /// Returns the reference count of the object. It is unwise to do anything other than test
    /// whether the return value of this method is greater than zero.
    #[inline]
    pub fn retain_count(&self) -> CFIndex {
        unsafe { CFGetRetainCount(self.as_CFTypeRef()) }
    }

    /// Returns the type ID of this object. Will be one of `CFData`, `CFString`, `CFArray`,
    /// `CFDictionary`, `CFDate`, `CFBoolean`, or `CFNumber`.
    #[inline]
    pub fn type_of(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_CFTypeRef()) }
    }

    /// Writes a debugging version of this object on standard error.
    pub fn show(&self) {
        unsafe { CFShow(self.as_CFTypeRef()) }
    }

    /// Returns `true` if this value is an instance of another type.
    #[inline]
    pub fn instance_of<OtherCFType: TCFType>(&self) -> bool {
        self.type_of() == OtherCFType::type_id()
    }
}

impl Clone for CFPropertyList {
    #[inline]
    fn clone(&self) -> CFPropertyList {
        unsafe { CFPropertyList::wrap_under_get_rule(self.0) }
    }
}

impl PartialEq for CFPropertyList {
    #[inline]
    fn eq(&self, other: &CFPropertyList) -> bool {
        self.as_CFType().eq(&other.as_CFType())
    }
}

impl Eq for CFPropertyList {}

impl CFPropertyList {
    /// Try to downcast the [`CFPropertyList`] to a subclass. Checking if the instance is the
    /// correct subclass happens at runtime and `None` is returned if it is not the correct type.
    /// Works similar to [`Box::downcast`] and [`CFType::downcast`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use core_foundation::string::CFString;
    /// # use core_foundation::propertylist::{CFPropertyList, CFPropertyListSubClass};
    /// #
    /// // Create a string.
    /// let string: CFString = CFString::from_static_string("FooBar");
    /// // Cast it up to a property list.
    /// let propertylist: CFPropertyList = string.to_CFPropertyList();
    /// // Cast it down again.
    /// assert_eq!(propertylist.downcast::<CFString>().unwrap().to_string(), "FooBar");
    /// ```
    ///
    /// [`CFPropertyList`]: struct.CFPropertyList.html
    /// [`Box::downcast`]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast
    pub fn downcast<T: CFPropertyListSubClass>(&self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let subclass_ref = T::Ref::from_void_ptr(self.0);
                Some(T::wrap_under_get_rule(subclass_ref))
            }
        } else {
            None
        }
    }

    /// Similar to [`downcast`], but consumes self and can thus avoid touching the retain count.
    ///
    /// [`downcast`]: #method.downcast
    pub fn downcast_into<T: CFPropertyListSubClass>(self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let subclass_ref = T::Ref::from_void_ptr(self.0);
                mem::forget(self);
                Some(T::wrap_under_create_rule(subclass_ref))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::boolean::CFBoolean;
    use crate::string::CFString;

    #[test]
    fn test_property_list_serialization() {
        use super::*;
        use crate::base::{CFEqual, TCFType};
        use crate::boolean::CFBoolean;
        use crate::dictionary::CFDictionary;
        use crate::number::CFNumber;
        use crate::string::CFString;

        let bar = CFString::from_static_string("Bar");
        let baz = CFString::from_static_string("Baz");
        let boo = CFString::from_static_string("Boo");
        let foo = CFString::from_static_string("Foo");
        let tru = CFBoolean::true_value();
        let n42 = CFNumber::from(1i64 << 33);

        let dict1 = CFDictionary::from_CFType_pairs(&[
            (bar.as_CFType(), boo.as_CFType()),
            (baz.as_CFType(), tru.as_CFType()),
            (foo.as_CFType(), n42.as_CFType()),
        ]);

        let data = create_data(dict1.as_CFTypeRef(), kCFPropertyListXMLFormat_v1_0).unwrap();
        let (dict2, _) = create_with_data(data, kCFPropertyListImmutable).unwrap();
        unsafe {
            assert_eq!(CFEqual(dict1.as_CFTypeRef(), dict2), 1);
        }
    }

    #[test]
    fn to_propertylist_retain_count() {
        let string = CFString::from_static_string("alongerstring");
        assert_eq!(string.retain_count(), 1);

        let propertylist = string.to_CFPropertyList();
        assert_eq!(string.retain_count(), 2);
        assert_eq!(propertylist.retain_count(), 2);

        mem::drop(string);
        assert_eq!(propertylist.retain_count(), 1);
    }

    #[test]
    fn downcast_string() {
        let propertylist = CFString::from_static_string("Bar").to_CFPropertyList();
        assert_eq!(
            propertylist.downcast::<CFString>().unwrap().to_string(),
            "Bar"
        );
        assert!(propertylist.downcast::<CFBoolean>().is_none());
    }

    #[test]
    fn downcast_boolean() {
        let propertylist = CFBoolean::true_value().to_CFPropertyList();
        assert!(propertylist.downcast::<CFBoolean>().is_some());
        assert!(propertylist.downcast::<CFString>().is_none());
    }

    #[test]
    fn downcast_into_fail() {
        let string = CFString::from_static_string("alongerstring");
        let propertylist = string.to_CFPropertyList();
        assert_eq!(string.retain_count(), 2);

        assert!(propertylist.downcast_into::<CFBoolean>().is_none());
        assert_eq!(string.retain_count(), 1);
    }

    #[test]
    fn downcast_into() {
        let string = CFString::from_static_string("alongerstring");
        let propertylist = string.to_CFPropertyList();
        assert_eq!(string.retain_count(), 2);

        let string2 = propertylist.downcast_into::<CFString>().unwrap();
        assert_eq!(string2.to_string(), "alongerstring");
        assert_eq!(string2.retain_count(), 2);
    }
}
