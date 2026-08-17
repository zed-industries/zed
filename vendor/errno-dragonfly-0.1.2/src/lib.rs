#![no_std]

#[link(name = "errno", kind = "static")]
extern "C" {
    pub fn errno_location() -> *mut libc::c_int;
}
