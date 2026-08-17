use crate::base::{SecKeychainAttributeList, SecKeychainItemRef};
use core_foundation_sys::base::{CFTypeID, CFTypeRef, OSStatus};
use core_foundation_sys::dictionary::CFDictionaryRef;
use std::os::raw::c_void;

extern "C" {

    /// Returns the unique identifier of the opaque type to which a keychain item object belongs.
    pub fn SecKeychainItemGetTypeID() -> CFTypeID;

    /// Adds one or more items to a keychain.
    pub fn SecItemAdd(attributes: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;

    /// Returns one or more keychain items that match a search query, or copies attributes of specific keychain items.
    pub fn SecItemCopyMatching(query: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;

    /// Modifies items that match a search query.
    pub fn SecItemUpdate(query: CFDictionaryRef, attributesToUpdate: CFDictionaryRef) -> OSStatus;

    /// Deletes items that match a search query.
    pub fn SecItemDelete(query: CFDictionaryRef) -> OSStatus;

    /// # Legacy API
    pub fn SecKeychainItemModifyAttributesAndData(
        itemRef: SecKeychainItemRef,
        attrList: *const SecKeychainAttributeList,
        length: u32,
        data: *const c_void,
    ) -> OSStatus;

    pub fn SecKeychainItemFreeContent(
        attrList: *mut SecKeychainAttributeList,
        data: *mut c_void,
    ) -> OSStatus;

    pub fn SecKeychainItemDelete(itemRef: SecKeychainItemRef) -> OSStatus;
}
