#![doc = include_str!("../readme.md")]
#![doc(html_no_source)]
#![allow(non_snake_case)]
#![debugger_visualizer(natvis_file = "../windows-core.natvis")]
#![cfg_attr(all(not(feature = "std")), no_std)]

#[cfg(windows)]
include!("windows.rs");

extern crate self as windows_core;

extern crate alloc;

use alloc::boxed::Box;

#[doc(hidden)]
pub mod imp;

mod as_impl;
mod com_object;
mod guid;
mod inspectable;
mod interface;
mod out_param;
mod out_ref;
mod param;
mod param_value;
mod r#ref;
mod runtime_name;
mod runtime_type;
mod scoped_interface;
mod r#type;
mod unknown;
mod weak;

pub use as_impl::*;
pub use com_object::*;
pub use guid::*;
pub use inspectable::*;
pub use interface::*;
pub use out_param::*;
pub use out_ref::*;
pub use param::*;
pub use param_value::*;
pub use r#ref::*;
pub use r#type::*;
pub use runtime_name::*;
pub use runtime_type::*;
pub use scoped_interface::*;
pub use unknown::*;
pub use weak::*;
pub use windows_implement::implement;
pub use windows_interface::interface;
pub use windows_result::*;
