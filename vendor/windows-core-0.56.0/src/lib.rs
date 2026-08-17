/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![doc(html_no_source)]
#![allow(non_snake_case)]
#![cfg_attr(windows_debugger_visualizer, debugger_visualizer(natvis_file = "../.natvis"))]

extern crate self as windows_core;

#[doc(hidden)]
pub mod imp;

mod agile_reference;
mod array;
mod as_impl;
mod event;
mod guid;
mod inspectable;
mod interface;
mod param;
mod runtime_name;
mod runtime_type;
mod scoped_interface;
mod strings;
mod r#type;
mod unknown;
mod variant;
mod weak;

pub use agile_reference::*;
pub use array::*;
pub use as_impl::*;
pub use event::*;
pub use guid::*;
pub use inspectable::*;
pub use interface::*;
pub use param::*;
pub use r#type::*;
pub use runtime_name::*;
pub use runtime_type::*;
pub use scoped_interface::*;
pub use strings::*;
pub use unknown::*;
pub use variant::*;
pub use weak::*;
pub use windows_implement::implement;
pub use windows_interface::interface;
pub use windows_result::*;

/// Attempts to load the factory object for the given WinRT class.
/// This can be used to access COM interfaces implemented on a Windows Runtime class factory.
pub fn factory<C: RuntimeName, I: Interface>() -> Result<I> {
    imp::factory::<C, I>()
}
