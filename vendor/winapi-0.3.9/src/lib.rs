// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
#![cfg(windows)]
#![deny(unused, unused_qualifications)]
#![warn(unused_attributes)]
#![allow(bad_style, overflowing_literals, unused_macros, deprecated, unused_crate_dependencies)]
#![recursion_limit = "2563"]
#![no_std]
//Uncomment as needed or once minimum Rust version is bumped to 1.18
//#![cfg_attr(feature = "cargo-clippy", warn(clippy::pedantic))]
//#![cfg_attr(feature = "cargo-clippy", allow(clippy::absurd_extreme_comparisons, clippy::cast_lossless, clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss, clippy::cast_ptr_alignment, clippy::cast_sign_loss, clippy::doc_markdown, clippy::empty_enum, clippy::erasing_op, clippy::excessive_precision, clippy::expl_impl_clone_on_copy, clippy::identity_op, clippy::if_not_else, clippy::many_single_char_names, clippy::module_inception, clippy::cast_possible_truncation, clippy::too_many_arguments, clippy::transmute_int_to_float, clippy::trivially_copy_pass_by_ref, clippy::unreadable_literal, clippy::unseparated_literal_suffix, clippy::used_underscore_binding, clippy::redundant_static_lifetimes, clippy::missing_safety_doc))]

#[cfg(feature = "std")]
extern crate std;

/// Hack for exported macros
#[doc(hidden)]
pub extern crate core as _core;

// Modules
#[macro_use]
mod macros;
pub mod km;
pub mod shared;
pub mod ucrt;
pub mod um;
pub mod vc;
pub mod winrt;

/// Built in primitive types provided by the C language
pub mod ctypes {
    #[cfg(feature = "std")]
    pub use std::os::raw::c_void;
    #[cfg(not(feature = "std"))]
    pub enum c_void {}
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
    pub type __int8 = i8;
    pub type __uint8 = u8;
    pub type __int16 = i16;
    pub type __uint16 = u16;
    pub type __int32 = i32;
    pub type __uint32 = u32;
    pub type __int64 = i64;
    pub type __uint64 = u64;
    pub type wchar_t = u16;
}
// This trait should be implemented for all COM interfaces
pub trait Interface {
    // Returns the IID of the Interface
    fn uuidof() -> shared::guiddef::GUID;
}
// This trait should be implemented for all COM classes
pub trait Class {
    // Returns the CLSID of the Class
    fn uuidof() -> shared::guiddef::GUID;
}
