//! A thin rust wrapper for Android system properties.
//!
//! This crate is similar to the `android-properties` crate with the exception that
//! the necessary Android libc symbols are loaded dynamically instead of linked
//! statically. In practice this means that the same binary will work with old and
//! new versions of Android, even though the API for reading system properties changed
//! around Android L.
//!
//! ## Example
//!
//! ```rust
//! use android_system_properties::AndroidSystemProperties;
//!
//! let properties = AndroidSystemProperties::new();
//!
//! if let Some(value) = properties.get("persist.sys.timezone") {
//!    println!("{}", value);
//! }
//! ```
//!
//! ## Listing and setting properties
//!
//! For the sake of simplicity this crate currently only contains what's needed by wgpu.
//! The implementations for listing and setting properties can be added back if anyone needs
//! them (let me know by filing an issue).
//!
//! ## License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
//!  * MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! [LICENSE-APACHE]: https://github.com/nical/android_system_properties/blob/804681c5c1c93d4fab29c1a2f47b7d808dc70fd3/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/nical/android_system_properties/blob/804681c5c1c93d4fab29c1a2f47b7d808dc70fd3/LICENSE-MIT

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_void},
};

#[cfg(target_os = "android")]
use std::mem;

unsafe fn property_callback(payload: *mut String, _name: *const c_char, value: *const c_char, _serial: u32) {
    let cvalue = CStr::from_ptr(value);
    (*payload) = cvalue.to_str().unwrap().to_string();
}

type Callback = unsafe fn(*mut String, *const c_char, *const c_char, u32);

type SystemPropertyGetFn = unsafe extern "C" fn(*const c_char, *mut c_char) -> c_int;
type SystemPropertyFindFn = unsafe extern "C" fn(*const c_char) -> *const c_void;
type SystemPropertyReadCallbackFn = unsafe extern "C" fn(*const c_void, Callback, *mut String) -> *const c_void;

#[derive(Debug)]
/// An object that can retrieve android system properties.
///
/// ## Example
///
/// ```
/// use android_system_properties::AndroidSystemProperties;
///
/// let properties = AndroidSystemProperties::new();
///
/// if let Some(value) = properties.get("persist.sys.timezone") {
///    println!("{}", value);
/// }
/// ```
pub struct AndroidSystemProperties {
    libc_so: *mut c_void,
    get_fn: Option<SystemPropertyGetFn>,
    find_fn: Option<SystemPropertyFindFn>,
    read_callback_fn: Option<SystemPropertyReadCallbackFn>,
}

unsafe impl Send for AndroidSystemProperties {}
unsafe impl Sync for AndroidSystemProperties {}

impl AndroidSystemProperties {
    #[cfg(not(target_os = "android"))]
    /// Create an entry point for accessing Android properties.
    pub fn new() -> Self {
        AndroidSystemProperties {
            libc_so: std::ptr::null_mut(),
            find_fn: None,
            read_callback_fn: None,
            get_fn: None,
        }
    }

    #[cfg(target_os = "android")]
    /// Create an entry point for accessing Android properties.
    pub fn new() -> Self {
        let libc_so = unsafe { libc::dlopen(b"libc.so\0".as_ptr().cast(), libc::RTLD_NOLOAD) };

        let mut properties = AndroidSystemProperties {
            libc_so,
            find_fn: None,
            read_callback_fn: None,
            get_fn: None,
        };

        if libc_so.is_null() {
            return properties;
        }


        unsafe fn load_fn(libc_so: *mut c_void, name: &[u8]) -> Option<*const c_void> {
            let fn_ptr = libc::dlsym(libc_so, name.as_ptr().cast());

            if fn_ptr.is_null() {
                return None;
            }

            Some(fn_ptr)
        }

        unsafe {
            properties.read_callback_fn = load_fn(libc_so, b"__system_property_read_callback\0")
                .map(|raw| mem::transmute::<*const c_void, SystemPropertyReadCallbackFn>(raw));

            properties.find_fn = load_fn(libc_so, b"__system_property_find\0")
                .map(|raw| mem::transmute::<*const c_void, SystemPropertyFindFn>(raw));

            // Fallback for old versions of Android.
            if properties.read_callback_fn.is_none() || properties.find_fn.is_none() {
                properties.get_fn = load_fn(libc_so, b"__system_property_get\0")
                    .map(|raw| mem::transmute::<*const c_void, SystemPropertyGetFn>(raw));
            }
        }

        properties
    }

    /// Retrieve a system property.
    ///
    /// Returns None if the operation fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use android_system_properties::AndroidSystemProperties;
    /// let properties = AndroidSystemProperties::new();
    ///
    /// if let Some(value) = properties.get("persist.sys.timezone") {
    ///     println!("{}", value);
    /// }
    /// ```
    pub fn get(&self, name: &str) -> Option<String> {
        let cname = CString::new(name).ok()?;
        self.get_from_cstr(&cname)
    }

    /// Retrieve a system property using a [`CStr`] key.
    ///
    /// Returns None if the operation fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use android_system_properties::AndroidSystemProperties;
    /// # use std::ffi::CStr;
    /// let properties = AndroidSystemProperties::new();
    ///
    /// let key = unsafe { CStr::from_bytes_with_nul_unchecked(b"persist.sys.timezone\0") };
    /// if let Some(value) = properties.get_from_cstr(key) {
    ///     println!("{}", value);
    /// }
    /// ```
    pub fn get_from_cstr(&self, cname: &std::ffi::CStr) -> Option<String> {
        // If available, use the recommended approach to accessing properties (Android L and onward).
        if let (Some(find_fn), Some(read_callback_fn)) = (self.find_fn, self.read_callback_fn) {
            let info = unsafe { (find_fn)(cname.as_ptr()) };

            if info.is_null() {
                return None;
            }

            let mut result = String::new();

            unsafe {
                (read_callback_fn)(info, property_callback, &mut result);
            }

            return Some(result);
        }

        // Fall back to the older approach.
        if let Some(get_fn) = self.get_fn {
            // The constant is PROP_VALUE_MAX in Android's libc/include/sys/system_properties.h
            const PROPERTY_VALUE_MAX: usize = 92;
            let mut buffer: Vec<u8> = Vec::with_capacity(PROPERTY_VALUE_MAX);
            let raw = buffer.as_mut_ptr() as *mut c_char;

            let len = unsafe { (get_fn)(cname.as_ptr(), raw) };

            if len > 0 {
                assert!(len as usize <= buffer.capacity());
                unsafe { buffer.set_len(len as usize); }
                String::from_utf8(buffer).ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Drop for AndroidSystemProperties {
    fn drop(&mut self) {
        if !self.libc_so.is_null() {
            unsafe {
                libc::dlclose(self.libc_so);
            }
        }
    }
}
