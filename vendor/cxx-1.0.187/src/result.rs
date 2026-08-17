#![cfg(feature = "alloc")]
#![allow(missing_docs)]

use crate::exception::Exception;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::Display;
use core::ptr::{self, NonNull};
use core::result::Result as StdResult;
use core::slice;
use core::str;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct PtrLen {
    pub ptr: NonNull<u8>,
    pub len: usize,
}

#[repr(C)]
pub union Result {
    err: PtrLen,
    ok: *const u8, // null
}

pub unsafe fn r#try<T, E>(ret: *mut T, result: StdResult<T, E>) -> Result
where
    E: Display,
{
    match result {
        Ok(ok) => {
            unsafe { ptr::write(ret, ok) }
            Result { ok: ptr::null() }
        }
        Err(err) => unsafe { to_c_error(err.to_string()) },
    }
}

unsafe fn to_c_error(msg: String) -> Result {
    let ptr = msg.as_ptr();
    let len = msg.len();

    extern "C" {
        #[link_name = "cxxbridge1$error"]
        fn error(ptr: *const u8, len: usize) -> NonNull<u8>;
    }

    let copy = unsafe { error(ptr, len) };
    let err = PtrLen { ptr: copy, len };
    Result { err }
}

impl Result {
    pub unsafe fn exception(self) -> StdResult<(), Exception> {
        unsafe {
            if self.ok.is_null() {
                Ok(())
            } else {
                let err = self.err;
                let slice = slice::from_raw_parts_mut(err.ptr.as_ptr(), err.len);
                let s = str::from_utf8_unchecked_mut(slice);
                Err(Exception {
                    what: Box::from_raw(s),
                })
            }
        }
    }
}
