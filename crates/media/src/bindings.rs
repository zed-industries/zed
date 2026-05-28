#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

#[cfg(target_os = "macos")]
use objc::*;

#[cfg(target_os = "macos")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
