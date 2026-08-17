#![cfg(windows)]
extern crate libloading;
use libloading::os::windows::*;
use std::ffi::CStr;
use std::os::raw::c_void;
// The ordinal DLL contains exactly one function (other than DllMain, that is) with ordinal number
// 1. This function has the sugnature `fn() -> *const c_char` and returns a string "bunny\0" (in
// reference to WindowsBunny).
//
// Both x86_64 and x86 versions of the .dll are functionally the same. Ideally we would compile the
// dlls with well known ordinals from our own testing helpers library, but rustc does not allow
// specifying a custom .def file (https://github.com/rust-lang/rust/issues/35089)
//
// The DLLs were kindly compiled by WindowsBunny (aka. @retep998).

#[cfg(target_arch = "x86")]
fn load_ordinal_lib() -> Library {
    unsafe { Library::new("tests/nagisa32.dll").expect("nagisa32.dll") }
}

#[cfg(target_arch = "x86_64")]
fn load_ordinal_lib() -> Library {
    unsafe { Library::new("tests/nagisa64.dll").expect("nagisa64.dll") }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn test_ordinal() {
    let lib = load_ordinal_lib();
    unsafe {
        let windows: Symbol<unsafe fn() -> *const i8> = lib.get_ordinal(1).expect("function");
        assert_eq!(CStr::from_ptr(windows()).to_bytes(), b"bunny");
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn test_try_into_ptr() {
    let lib = load_ordinal_lib();
    unsafe {
        let windows: Symbol<unsafe fn() -> *const i8> = lib.get_ordinal(1).expect("function");
        let ptr: *mut c_void = windows.as_raw_ptr();
        assert!(!ptr.is_null());
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn test_ordinal_missing_fails() {
    let lib = load_ordinal_lib();
    unsafe {
        let r: Result<Symbol<unsafe fn() -> *const i8>, _> = lib.get_ordinal(2);
        r.err().unwrap();
        let r: Result<Symbol<unsafe fn() -> *const i8>, _> = lib.get_ordinal(!0);
        r.err().unwrap();
    }
}

#[test]
fn test_new_kernel23() {
    unsafe {
        Library::new("kernel23").err().unwrap();
    }
}

#[test]
fn test_new_kernel32_no_ext() {
    unsafe {
        Library::new("kernel32").unwrap();
    }
}
