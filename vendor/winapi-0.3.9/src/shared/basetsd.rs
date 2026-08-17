// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Type definitions for the basic sized types.
use ctypes::{__int64, __uint64, c_int, c_schar, c_short, c_uchar, c_uint, c_ushort};
pub type POINTER_64_INT = usize;
pub type INT8 = c_schar;
pub type PINT8 = *mut c_schar;
pub type INT16 = c_short;
pub type PINT16 = *mut c_short;
pub type INT32 = c_int;
pub type PINT32 = *mut c_int;
pub type INT64 = __int64;
pub type PINT64 = *mut __int64;
pub type UINT8 = c_uchar;
pub type PUINT8 = *mut c_uchar;
pub type UINT16 = c_ushort;
pub type PUINT16 = *mut c_ushort;
pub type UINT32 = c_uint;
pub type PUINT32 = *mut c_uint;
pub type UINT64 = __uint64;
pub type PUINT64 = *mut __uint64;
pub type LONG32 = c_int;
pub type PLONG32 = *mut c_int;
pub type ULONG32 = c_uint;
pub type PULONG32 = *mut c_uint;
pub type DWORD32 = c_uint;
pub type PDWORD32 = *mut c_uint;
pub type INT_PTR = isize;
pub type PINT_PTR = *mut isize;
pub type UINT_PTR = usize;
pub type PUINT_PTR = *mut usize;
pub type LONG_PTR = isize;
pub type PLONG_PTR = *mut isize;
pub type ULONG_PTR = usize;
pub type PULONG_PTR = *mut usize;
pub type SHANDLE_PTR = isize;
pub type HANDLE_PTR = usize;
#[cfg(target_pointer_width = "32")]
pub type UHALF_PTR = c_ushort;
#[cfg(target_pointer_width = "64")]
pub type UHALF_PTR = c_uint;
#[cfg(target_pointer_width = "32")]
pub type PUHALF_PTR = *mut c_ushort;
#[cfg(target_pointer_width = "64")]
pub type PUHALF_PTR = *mut c_uint;
#[cfg(target_pointer_width = "32")]
pub type HALF_PTR = c_short;
#[cfg(target_pointer_width = "64")]
pub type HALF_PTR = c_int;
#[cfg(target_pointer_width = "32")]
pub type PHALF_PTR = *mut c_short;
#[cfg(target_pointer_width = "64")]
pub type PHALF_PTR = *mut c_int;
pub type SIZE_T = ULONG_PTR;
pub type PSIZE_T = *mut ULONG_PTR;
pub type SSIZE_T = LONG_PTR;
pub type PSSIZE_T = *mut LONG_PTR;
pub type DWORD_PTR = ULONG_PTR;
pub type PDWORD_PTR = *mut ULONG_PTR;
pub type LONG64 = __int64;
pub type PLONG64 = *mut __int64;
pub type ULONG64 = __uint64;
pub type PULONG64 = *mut __uint64;
pub type DWORD64 = __uint64;
pub type PDWORD64 = *mut __uint64;
pub type KAFFINITY = ULONG_PTR;
pub type PKAFFINITY = *mut KAFFINITY;
