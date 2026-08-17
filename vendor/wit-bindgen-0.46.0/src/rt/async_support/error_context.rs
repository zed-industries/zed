//! Raw bindings to `error-context` in the canonical ABI.

use std::fmt::{self, Debug, Display};
use std::ptr;
use std::string::String;

/// Represents the Component Model `error-context` type.
#[derive(PartialEq, Eq)]
pub struct ErrorContext {
    handle: u32,
}

impl ErrorContext {
    /// Call the `error-context.new` canonical built-in function.
    pub fn new(debug_message: &str) -> ErrorContext {
        unsafe {
            let handle = new(debug_message.as_ptr(), debug_message.len());
            ErrorContext::from_handle(handle)
        }
    }

    #[doc(hidden)]
    pub fn from_handle(handle: u32) -> Self {
        Self { handle }
    }

    #[doc(hidden)]
    pub fn handle(&self) -> u32 {
        self.handle
    }

    /// Extract the debug message from a given [`ErrorContext`]
    pub fn debug_message(&self) -> String {
        unsafe {
            let mut ret = RetPtr {
                ptr: ptr::null_mut(),
                len: 0,
            };
            debug_message(self.handle, &mut ret);
            String::from_raw_parts(ret.ptr, ret.len, ret.len)
        }
    }
}

impl Debug for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorContext")
            .field("debug_message", &self.debug_message())
            .finish()
    }
}

impl Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.debug_message(), f)
    }
}

impl std::error::Error for ErrorContext {}

impl Drop for ErrorContext {
    fn drop(&mut self) {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            drop(self.handle)
        }
    }
}

#[repr(C)]
struct RetPtr {
    ptr: *mut u8,
    len: usize,
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn new(_: *const u8, _: usize) -> u32 {
    unreachable!()
}
#[cfg(not(target_arch = "wasm32"))]
fn debug_message(_: u32, _: &mut RetPtr) {
    unreachable!()
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "$root")]
extern "C" {
    #[link_name = "[error-context-new-utf8]"]
    fn new(_: *const u8, _: usize) -> u32;
    #[link_name = "[error-context-drop]"]
    fn drop(_: u32);
    #[link_name = "[error-context-debug-message-utf8]"]
    fn debug_message(_: u32, _: &mut RetPtr);
}
