/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>

[Feature search](https://microsoft.github.io/windows-rs/features/#/0.59.0)
*/

#![no_std]
#![doc(html_no_source)]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, missing_docs, clippy::all)]
#![cfg_attr(not(feature = "docs"), doc(hidden))]

#[allow(unused_extern_crates)]
extern crate self as windows_sys;

pub mod core;

include!("Windows/mod.rs");
