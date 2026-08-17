/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![no_std]
#![doc(html_no_source)]
#![allow(non_snake_case, clashing_extern_declarations)]
#![cfg_attr(windows_raw_dylib, feature(raw_dylib))]

extern crate self as windows_sys;
mod Windows;
pub mod core;
pub use Windows::*;
