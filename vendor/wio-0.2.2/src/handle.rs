// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use error::{Error, Result};
use std::{
    ops::Deref,
    os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle},
    ptr::null_mut,
};
use winapi::{
    um::{
        handleapi::{CloseHandle, DuplicateHandle},
        processthreadsapi::GetCurrentProcess,
        winnt::{DUPLICATE_SAME_ACCESS, HANDLE},
    },
    shared::minwindef::FALSE,
};

pub struct Handle(HANDLE);
impl Handle {
    // Takes ownership of the handle
    pub unsafe fn new(handle: HANDLE) -> Handle {
        Handle(handle)
    }
    pub fn close(self) -> Result<()> {
        match unsafe { CloseHandle(self.into_raw_handle()) } {
            0 => Error::last(),
            _ => Ok(()),
        }
    }
    // Duplicates the handle without taking ownership
    pub unsafe fn duplicate_from(handle: HANDLE) -> Result<Handle> {
        let mut new_handle = null_mut();
        let res = DuplicateHandle(
            GetCurrentProcess(), handle, GetCurrentProcess(),
            &mut new_handle, 0, FALSE, DUPLICATE_SAME_ACCESS,
        );
        match res {
            0 => Error::last(),
            _ => Ok(Handle(new_handle)),
        }
    }
}
impl AsRawHandle for Handle {
    fn as_raw_handle(&self) -> HANDLE {
        self.0
    }
}
impl Deref for Handle {
    type Target = HANDLE;
    fn deref(&self) -> &HANDLE { &self.0 }
}
impl Drop for Handle {
    fn drop(&mut self) {
        let ret = unsafe { CloseHandle(self.0) };
        let err: Result<()> = Error::last();
        assert!(ret != 0, "{:?}", err);
    }
}
impl FromRawHandle for Handle {
    unsafe fn from_raw_handle(handle: HANDLE) -> Handle {
        Handle(handle)
    }
}
impl IntoRawHandle for Handle {
    fn into_raw_handle(self) -> HANDLE {
        self.0
    }
}
