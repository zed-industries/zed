// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::proxy::macos::system_configuration_sys_extra::{
    kSCDynamicStoreUseSessionKeys, SCDynamicStoreCallBack, SCDynamicStoreContext,
    SCDynamicStoreCopyProxies, SCDynamicStoreCreateRunLoopSource, SCDynamicStoreCreateWithOptions,
    SCDynamicStoreGetTypeID, SCDynamicStoreRef, SCDynamicStoreSetNotificationKeys,
};
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{kCFAllocatorDefault, CFType, TCFType},
    boolean::CFBoolean,
    declare_TCFType,
    dictionary::CFDictionary,
    impl_TCFType,
    runloop::CFRunLoopSource,
    string::CFString,
};
use std::{ffi::c_void, ptr};

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
pub type SCDynamicStoreCallBackT<T> =
    fn(store: SCDynamicStore, changed_keys: CFArray<CFString>, info: &mut T);

pub struct SCDynamicStoreBuilder<T> {
    name: CFString,
    session_keys: bool,
    callback_context: Option<SCDynamicStoreCallBackContext<T>>,
}

impl SCDynamicStoreBuilder<()> {
    pub fn new<S: Into<CFString>>(name: S) -> Self {
        SCDynamicStoreBuilder {
            name: name.into(),
            session_keys: false,
            callback_context: None,
        }
    }
}

impl<T: Send + Sync + 'static> SCDynamicStoreBuilder<T> {
    /// Set a callback context (callback function and data to pass to each callback call).
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
