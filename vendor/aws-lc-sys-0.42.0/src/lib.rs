// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg_attr(not(clippy), allow(unexpected_cfgs))]
#![cfg_attr(not(clippy), allow(unknown_lints))]

#[allow(unused_macros)]
macro_rules! use_bindings {
    ($bindings:ident) => {
        mod $bindings;
        pub use $bindings::*;
    };
}

macro_rules! platform_binding {
    ($platform:ident, $platform_crypto:ident) => {
        #[cfg(all($platform, not(feature = "ssl"), not(use_bindgen_pregenerated)))]
        use_bindings!($platform_crypto);
    };
}

platform_binding!(universal_prefixed, universal_prefixed_crypto);
platform_binding!(universal_no_u1_prefixed, universal_no_u1_prefixed_crypto);
platform_binding!(universal_no_u1, universal_no_u1_crypto);
platform_binding!(universal, universal_crypto);

platform_binding!(aarch64_linux_android, aarch64_linux_android_crypto);
platform_binding!(aarch64_apple_darwin, aarch64_apple_darwin_crypto);
platform_binding!(aarch64_pc_windows_msvc, aarch64_pc_windows_msvc_crypto);
platform_binding!(aarch64_unknown_linux_gnu, aarch64_unknown_linux_gnu_crypto);
platform_binding!(
    aarch64_unknown_linux_musl,
    aarch64_unknown_linux_musl_crypto
);
platform_binding!(i686_pc_windows_msvc, i686_pc_windows_msvc_crypto);
platform_binding!(i686_unknown_linux_gnu, i686_unknown_linux_gnu_crypto);
platform_binding!(
    riscv64gc_unknown_linux_gnu,
    riscv64gc_unknown_linux_gnu_crypto
);
platform_binding!(x86_64_apple_darwin, x86_64_apple_darwin_crypto);
platform_binding!(x86_64_pc_windows_gnu, x86_64_pc_windows_gnu_crypto);
platform_binding!(x86_64_pc_windows_msvc, x86_64_pc_windows_msvc_crypto);
platform_binding!(x86_64_unknown_linux_gnu, x86_64_unknown_linux_gnu_crypto);
platform_binding!(x86_64_unknown_linux_musl, x86_64_unknown_linux_musl_crypto);

#[cfg(use_bindgen_pregenerated)]
#[allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::missing_safety_doc,
    clippy::must_use_candidate,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::ptr_as_ptr,
    clippy::ptr_offset_with_cast,
    clippy::pub_underscore_fields,
    clippy::semicolon_if_nothing_returned,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::used_underscore_binding,
    clippy::useless_transmute,
    dead_code,
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unpredictable_function_pointer_comparisons,
    unused_imports
)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
#[cfg(use_bindgen_pregenerated)]
pub use generated::*;

#[allow(non_snake_case, clippy::cast_possible_wrap)]
#[must_use]
pub fn ERR_GET_LIB(packed_error: u32) -> i32 {
    ((packed_error >> 24) & 0xFF) as i32
}

#[allow(non_snake_case, clippy::cast_possible_wrap)]
#[must_use]
pub fn ERR_GET_REASON(packed_error: u32) -> i32 {
    (packed_error & 0x0FFF) as i32
}

#[allow(non_snake_case)]
#[must_use]
pub fn ERR_GET_FUNC(_packed_error: u32) -> i32 {
    0
}

#[cfg(feature = "all-bindings")]
use std::os::raw::{c_char, c_long, c_void};

#[cfg(feature = "all-bindings")]
#[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
pub fn BIO_get_mem_data(b: *mut BIO, pp: *mut *mut c_char) -> c_long {
    unsafe { BIO_ctrl(b, BIO_CTRL_INFO, 0, pp.cast::<c_void>()) }
}

pub fn init() {
    unsafe { CRYPTO_library_init() }
}
