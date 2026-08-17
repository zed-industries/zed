#[cfg(feature = "alloc")]
use alloc::string::String;
use core::mem::MaybeUninit;
use core::ptr;
use core::slice;
use core::str;

#[export_name = "cxxbridge1$str$new"]
unsafe extern "C" fn str_new(this: &mut MaybeUninit<&str>) {
    let this = this.as_mut_ptr();
    unsafe { ptr::write(this, "") }
}

#[cfg(feature = "alloc")]
#[export_name = "cxxbridge1$str$ref"]
unsafe extern "C" fn str_ref<'a>(this: &mut MaybeUninit<&'a str>, string: &'a String) {
    let this = this.as_mut_ptr();
    let s = string.as_str();
    unsafe { ptr::write(this, s) }
}

#[export_name = "cxxbridge1$str$from"]
unsafe extern "C" fn str_from(this: &mut MaybeUninit<&str>, ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    match str::from_utf8(slice) {
        Ok(s) => {
            let this = this.as_mut_ptr();
            unsafe { ptr::write(this, s) }
            true
        }
        Err(_) => false,
    }
}

#[export_name = "cxxbridge1$str$ptr"]
unsafe extern "C" fn str_ptr(this: &&str) -> *const u8 {
    this.as_ptr()
}

#[export_name = "cxxbridge1$str$len"]
unsafe extern "C" fn str_len(this: &&str) -> usize {
    this.len()
}
