//! This is a separate file containing helpers for tests of this library. It is built into a
//! dynamic library by the build.rs script.
#![crate_type="cdylib"]

#[no_mangle]
pub static mut TEST_STATIC_U32: u32 = 0;

#[no_mangle]
pub static mut TEST_STATIC_PTR: *mut () = 0 as *mut _;

#[no_mangle]
pub extern "C" fn test_identity_u32(x: u32) -> u32 {
    x
}

#[repr(C)]
pub struct S {
    a: u64,
    b: u32,
    c: u16,
    d: u8
}

#[no_mangle]
pub extern "C" fn test_identity_struct(x: S) -> S {
    x
}

#[no_mangle]
pub unsafe extern "C" fn test_get_static_u32() -> u32 {
    TEST_STATIC_U32
}

#[no_mangle]
pub unsafe extern "C" fn test_check_static_ptr() -> bool {
    TEST_STATIC_PTR == (&mut TEST_STATIC_PTR as *mut *mut _ as *mut _)
}
