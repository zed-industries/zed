//! This module corresponds to `mach/i386/boolean.h`.

#[cfg(target_arch = "x86_64")]
pub type boolean_t = ::libc::c_uint;

#[cfg(not(target_arch = "x86_64"))]
pub type boolean_t = ::libc::c_int;
