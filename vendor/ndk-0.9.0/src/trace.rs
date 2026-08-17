//! Bindings for the NDK tracing API.
//!
//! See also [the NDK docs](https://developer.android.com/ndk/reference/group/tracing)
#![cfg(feature = "api-level-23")]
use std::ffi::{CString, NulError};
use std::marker::PhantomData;

pub fn is_trace_enabled() -> bool {
    unsafe { ffi::ATrace_isEnabled() }
}

#[derive(Debug)]
pub struct Section {
    // Section is !Sync and !Send
    _pd: PhantomData<*mut ()>,
}

impl Section {
    pub fn new(name: &str) -> Result<Self, NulError> {
        let section_name = CString::new(name)?;
        unsafe { ffi::ATrace_beginSection(section_name.as_ptr()) };

        Ok(Self { _pd: PhantomData })
    }

    pub fn end(self) {
        drop(self)
    }
}

impl Drop for Section {
    fn drop(&mut self) {
        unsafe { ffi::ATrace_endSection() };
    }
}

/// Unique identifier for distinguishing simultaneous events
#[derive(Debug)]
#[cfg(feature = "api-level-29")]
pub struct Cookie(pub i32);

#[derive(Debug)]
#[cfg(feature = "api-level-29")]
pub struct AsyncSection {
    section_name: CString,
    cookie: Cookie,
    // AsyncSection is !Sync
    _pd: PhantomData<&'static ()>,
}

#[cfg(feature = "api-level-29")]
impl AsyncSection {
    pub fn new(name: &str, cookie: Cookie) -> Result<Self, NulError> {
        let section_name = CString::new(name)?;
        unsafe { ffi::ATrace_beginAsyncSection(section_name.as_ptr(), cookie.0) };

        Ok(Self {
            section_name,
            cookie,
            _pd: PhantomData,
        })
    }

    pub fn end(self) {
        drop(self)
    }
}

#[cfg(feature = "api-level-29")]
impl Drop for AsyncSection {
    fn drop(&mut self) {
        unsafe { ffi::ATrace_endAsyncSection(self.section_name.as_ptr(), self.cookie.0) };
    }
}

#[cfg(feature = "api-level-29")]
#[derive(Debug)]
pub struct Counter {
    name: CString,
}

#[cfg(feature = "api-level-29")]
impl Counter {
    pub fn new(name: &str) -> Result<Self, NulError> {
        let name = CString::new(name)?;
        Ok(Self { name })
    }

    pub fn set_value(&self, value: i64) {
        unsafe { ffi::ATrace_setCounter(self.name.as_ptr(), value) }
    }
}
