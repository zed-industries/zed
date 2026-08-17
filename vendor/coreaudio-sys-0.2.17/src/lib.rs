#![cfg(any(target_os = "macos", target_os = "ios"))]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/coreaudio.rs"));
