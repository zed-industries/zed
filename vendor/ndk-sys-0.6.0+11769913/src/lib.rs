//! Raw FFI bindings to the Android NDK.
//!
//! The bindings are pre-generated and the right one for the platform is selected at compile time.

// Bindgen lints
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(clippy::all)]
// Temporarily allow UB nullptr dereference in bindgen layout tests until fixed upstream:
// https://github.com/rust-lang/rust-bindgen/pull/2055
// https://github.com/rust-lang/rust-bindgen/pull/2064
#![allow(deref_nullptr)]
// Test setup lints
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use jni_sys::*;

#[cfg(not(any(target_os = "android", feature = "test")))]
compile_error!("ndk-sys only supports compiling for Android");

#[cfg(all(
    any(target_os = "android", feature = "test"),
    any(target_arch = "arm", target_arch = "armv7")
))]
include!("ffi_arm.rs");

#[cfg(all(any(target_os = "android", feature = "test"), target_arch = "aarch64"))]
include!("ffi_aarch64.rs");

#[cfg(all(any(target_os = "android", feature = "test"), target_arch = "x86"))]
include!("ffi_i686.rs");

#[cfg(all(any(target_os = "android", feature = "test"), target_arch = "x86_64"))]
include!("ffi_x86_64.rs");

#[cfg(target_os = "android")]
#[link(name = "android")]
extern "C" {}

#[cfg(all(feature = "nativewindow", target_os = "android"))]
#[link(name = "nativewindow")]
extern "C" {}

#[cfg(all(feature = "media", target_os = "android"))]
#[link(name = "mediandk")]
extern "C" {}

#[cfg(all(feature = "bitmap", target_os = "android"))]
#[link(name = "jnigraphics")]
extern "C" {}

#[cfg(all(feature = "audio", target_os = "android"))]
#[link(name = "aaudio")]
extern "C" {}

#[cfg(all(feature = "sync", target_os = "android"))]
#[link(name = "sync")]
extern "C" {}
