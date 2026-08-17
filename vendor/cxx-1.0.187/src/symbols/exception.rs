#![cfg(feature = "alloc")]

use crate::result::PtrLen;
use alloc::boxed::Box;
use alloc::string::String;
use core::ptr::NonNull;
use core::slice;

#[export_name = "cxxbridge1$exception"]
unsafe extern "C" fn exception(ptr: *const u8, len: usize) -> PtrLen {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    let string = String::from_utf8_lossy(slice);
    let len = string.len();
    let raw_str = Box::into_raw(string.into_owned().into_boxed_str());
    let raw_u8 = raw_str.cast::<u8>();
    let nonnull = unsafe { NonNull::new_unchecked(raw_u8) };
    PtrLen { ptr: nonnull, len }
}
