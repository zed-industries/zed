// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bindings to [`SCDynamicStore`].
//!
//! See the examples directory for examples how to use this module.
//!
//! [`SCDynamicStore`]: https://developer.apple.com/documentation/systemconfiguration/scdynamicstore?language=objc

use crate::sys::{
    dynamic_store::{
        kSCDynamicStoreUseSessionKeys, SCDynamicStoreCallBack, SCDynamicStoreContext,
        SCDynamicStoreCopyKeyList, SCDynamicStoreCopyValue, SCDynamicStoreCreateRunLoopSource,
        SCDynamicStoreCreateWithOptions, SCDynamicStoreGetTypeID, SCDynamicStoreRef,
        SCDynamicStoreRemoveValue, SCDynamicStoreSetNotificationKeys, SCDynamicStoreSetValue,
    },
    dynamic_store_copy_specific::SCDynamicStoreCopyProxies,
};
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{kCFAllocatorDefault, CFType, TCFType},
    boolean::CFBoolean,
    dictionary::CFDictionary,
    propertylist::{CFPropertyList, CFPropertyListSubClass},
    runloop::CFRunLoopSource,
    string::CFString,
};
use std::{ffi::c_void, ptr};

/// Struct describing the callback happening when a watched value in the dynamic store is changed.
pub struct SCDynamicStoreCallBackContext<T> {
    /// The callback function that will be called when a watched value in the dynamic store is
    /// changed.
    pub callout: SCDynamicStoreCallBackT<T>,

    /// The argument passed to each `callout` call. Can be used to keep state between
    /// callbacks.
    pub info: T,
}

/// Signature for callback functions getting called when a watched value in the dynamic store is
/// changed.
///
/// This is the safe callback definition, abstracting over the lower level `SCDynamicStoreCallBack`
/// from the `system-configuration-sys` crate.
pub type SCDynamicStoreCallBackT<T> =
    fn(store: SCDynamicStore, changed_keys: CFArray<CFString>, info: &mut T);

/// Builder for [`SCDynamicStore`] sessions.
///
/// [`SCDynamicStore`]: struct.SCDynamicStore.html
pub struct SCDynamicStoreBuilder<T> {
    name: CFString,
    session_keys: bool,
    callback_context: Option<SCDynamicStoreCallBackContext<T>>,
}

impl SCDynamicStoreBuilder<()> {
    /// Creates a new builder. `name` is used as the name parameter when creating the
    /// [`SCDynamicStore`] session.
    ///
    /// [`SCDynamicStore`]: struct.SCDynamicStore.html
    pub fn new<S: Into<CFString>>(name: S) -> Self {
        SCDynamicStoreBuilder {
            name: name.into(),
            session_keys: false,
            callback_context: None,
        }
    }
}

impl<T> SCDynamicStoreBuilder<T> {
    /// Set wether or not the created [`SCDynamicStore`] should have session keys or not.
    /// See [`SCDynamicStoreCreateWithOptions`] for details.
    ///
    /// Defaults to `false`.
    ///
    /// [`SCDynamicStore`]: struct.SCDynamicStore.html
    /// [`SCDynamicStoreCreateWithOptions`]: https://developer.apple.com/documentation/systemconfiguration/1437818-scdynamicstorecreatewithoptions?language=objc
    pub fn session_keys(mut self, session_keys: bool) -> Self {
        self.session_keys = session_keys;
        self
    }

    /// Set a callback context (callback function and data to pass to each callback call).
    ///
    /// Defaults to having callbacks disabled.
    pub fn callback_context<T2>(
        self,
        callback_context: SCDynamicStoreCallBackContext<T2>,
    ) -> SCDynamicStoreBuilder<T2> {
        SCDynamicStoreBuilder {
            name: self.name,
            session_keys: self.session_keys,
            callback_context: Some(callback_context),
        }
    }

    /// Create the dynamic store session.
    pub fn build(mut self) -> SCDynamicStore {
        let store_options = self.create_store_options();
        if let Some(callback_context) = self.callback_context.take() {
            SCDynamicStore::create(
                &self.name,
                &store_options,
                Some(convert_callback::<T>),
                &mut self.create_context(callback_context),
            )
        } else {
            SCDynamicStore::create(&self.name, &store_options, None, ptr::null_mut())
        }
    }

    fn create_store_options(&self) -> CFDictionary {
        let key = unsafe { CFString::wrap_under_create_rule(kSCDynamicStoreUseSessionKeys) };
        let value = CFBoolean::from(self.session_keys);
        let typed_dict = CFDictionary::from_CFType_pairs(&[(key, value)]);
        unsafe { CFDictionary::wrap_under_get_rule(typed_dict.as_concrete_TypeRef()) }
    }

    fn create_context(
        &self,
        callback_context: SCDynamicStoreCallBackContext<T>,
    ) -> SCDynamicStoreContext {
        // move the callback context struct to the heap and "forget" it.
        // It will later be brought back into the Rust typesystem and freed in
        // `release_callback_context`
        let info_ptr = Box::into_raw(Box::new(callback_context));

        SCDynamicStoreContext {
            version: 0,
            info: info_ptr as *mut _ as *mut c_void,
            retain: None,
            release: Some(release_callback_context::<T>),
            copyDescription: None,
        }
    }
}

declare_TCFType! {
    /// Access to the key-value pairs in the dynamic store of a running system.
    ///
    /// Use the [`SCDynamicStoreBuilder`] to create instances of this.
    ///
    /// [`SCDynamicStoreBuilder`]: struct.SCDynamicStoreBuilder.html
    SCDynamicStore, SCDynamicStoreRef
}

impl_TCFType!(SCDynamicStore, SCDynamicStoreRef, SCDynamicStoreGetTypeID);

impl SCDynamicStore {
    /// Creates a new session used to interact with the dynamic store maintained by the System
    /// Configuration server.
    fn create(
        name: &CFString,
        store_options: &CFDictionary,
        callout: SCDynamicStoreCallBack,
        context: *mut SCDynamicStoreContext,
    ) -> Self {
        unsafe {
            let store = SCDynamicStoreCreateWithOptions(
                kCFAllocatorDefault,
                name.as_concrete_TypeRef(),
                store_options.as_concrete_TypeRef(),
                callout,
                context,
            );
            SCDynamicStore::wrap_under_create_rule(store)
        }
    }

    /// Returns the keys that represent the current dynamic store entries that match the specified
    /// pattern. Or `None` if an error occured.
    ///
    /// `pattern` - A regular expression pattern used to match the dynamic store keys.
    pub fn get_keys<S: Into<CFString>>(&self, pattern: S) -> Option<CFArray<CFString>> {
        let cf_pattern = pattern.into();
        unsafe {
            let array_ref = SCDynamicStoreCopyKeyList(
                self.as_concrete_TypeRef(),
                cf_pattern.as_concrete_TypeRef(),
            );
            if !array_ref.is_null() {
                Some(CFArray::wrap_under_create_rule(array_ref))
            } else {
                None
            }
        }
    }

    /// Returns the key-value pairs that represent the current internet proxy settings. Or `None` if
    /// no proxy settings have been defined or if an error occured.
    pub fn get_proxies(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let dictionary_ref = SCDynamicStoreCopyProxies(self.as_concrete_TypeRef());
            if !dictionary_ref.is_null() {
                Some(CFDictionary::wrap_under_create_rule(dictionary_ref))
            } else {
                None
            }
        }
    }

    /// If the given key exists in the store, the associated value is returned.
    ///
    /// Use `CFPropertyList::downcast_into` to cast the result into the correct type.
    pub fn get<S: Into<CFString>>(&self, key: S) -> Option<CFPropertyList> {
        let cf_key = key.into();
        unsafe {
            let dict_ref =
                SCDynamicStoreCopyValue(self.as_concrete_TypeRef(), cf_key.as_concrete_TypeRef());
            if !dict_ref.is_null() {
                Some(CFPropertyList::wrap_under_create_rule(dict_ref))
            } else {
                None
            }
        }
    }

    /// Sets the value of the given key. Overwrites existing values.
    /// Returns `true` on success, false on failure.
    pub fn set<S: Into<CFString>, V: CFPropertyListSubClass>(&self, key: S, value: V) -> bool {
        self.set_raw(key, &value.into_CFPropertyList())
    }

    /// Sets the value of the given key. Overwrites existing values.
    /// Returns `true` on success, false on failure.
    pub fn set_raw<S: Into<CFString>>(&self, key: S, value: &CFPropertyList) -> bool {
        let cf_key = key.into();
        let success = unsafe {
            SCDynamicStoreSetValue(
                self.as_concrete_TypeRef(),
                cf_key.as_concrete_TypeRef(),
                value.as_concrete_TypeRef(),
            )
        };
        success != 0
    }

    /// Removes the value of the specified key from the dynamic store.
    pub fn remove<S: Into<CFString>>(&self, key: S) -> bool {
        let cf_key = key.into();
        let success = unsafe {
            SCDynamicStoreRemoveValue(self.as_concrete_TypeRef(), cf_key.as_concrete_TypeRef())
        };
        success != 0
    }

    /// Specifies a set of keys and key patterns that should be monitored for changes.
    pub fn set_notification_keys<T1, T2>(
        &self,
        keys: &CFArray<T1>,
        patterns: &CFArray<T2>,
    ) -> bool {
        let success = unsafe {
            SCDynamicStoreSetNotificationKeys(
                self.as_concrete_TypeRef(),
                keys.as_concrete_TypeRef(),
                patterns.as_concrete_TypeRef(),
            )
        };
        success != 0
    }

    /// Creates a run loop source object that can be added to the application's run loop.
    pub fn create_run_loop_source(&self) -> CFRunLoopSource {
        unsafe {
            let run_loop_source_ref = SCDynamicStoreCreateRunLoopSource(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                0,
            );
            CFRunLoopSource::wrap_under_create_rule(run_loop_source_ref)
        }
    }
}

/// The raw callback used by the safe `SCDynamicStore` to convert from the `SCDynamicStoreCallBack`
/// to the `SCDynamicStoreCallBackT`
unsafe extern "C" fn convert_callback<T>(
    store_ref: SCDynamicStoreRef,
    changed_keys_ref: CFArrayRef,
    context_ptr: *mut c_void,
) {
    let store = SCDynamicStore::wrap_under_get_rule(store_ref);
    let changed_keys = CFArray::<CFString>::wrap_under_get_rule(changed_keys_ref);
    let context = &mut *(context_ptr as *mut _ as *mut SCDynamicStoreCallBackContext<T>);

    (context.callout)(store, changed_keys, &mut context.info);
}

// Release function called by core foundation on release of the dynamic store context.
unsafe extern "C" fn release_callback_context<T>(context_ptr: *const c_void) {
    // Bring back the context object from raw ptr so it is correctly freed.
    let _context = Box::from_raw(context_ptr as *mut SCDynamicStoreCallBackContext<T>);
}
