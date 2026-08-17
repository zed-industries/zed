#![cfg_attr(docsrs, doc = "This is a stub. The latest API documentation is here: <https://microsoft.github.io/windows-docs-rs/>")]
#![cfg_attr(docsrs, doc = "")]
/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>

[Feature search](https://microsoft.github.io/windows-rs/features/#/0.58.0)
*/

#![cfg(windows)]
#![doc(html_no_source)]
#![allow(non_snake_case, clashing_extern_declarations, non_upper_case_globals, non_camel_case_types, missing_docs, clippy::all)]
#![cfg_attr(not(feature = "docs"), doc(hidden))]
// TODO: workaround for https://github.com/rust-lang/rust/issues/126169
#![allow(unused)]

#[allow(unused_extern_crates)]
extern crate self as windows;

pub use windows_core as core;

mod extensions;

include!("Windows/mod.rs");
