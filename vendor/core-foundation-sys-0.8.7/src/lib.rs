// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes
)]
#![cfg_attr(
    all(feature = "mac_os_10_7_support", feature = "mac_os_10_8_features"),
    feature(linkage)
)] // back-compat requires weak linkage

// Link to CoreFoundation on any Apple device.
//
// We don't use `target_vendor` since that is going to be deprecated:
// https://github.com/rust-lang/lang-team/issues/102
#[cfg_attr(
    all(
        any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "watchos",
            target_os = "visionos"
        ),
        feature = "link"
    ),
    link(name = "CoreFoundation", kind = "framework")
)]
extern "C" {}

pub mod array;
pub mod attributed_string;
pub mod bag;
pub mod base;
pub mod binary_heap;
pub mod bit_vector;
pub mod bundle;
pub mod calendar;
pub mod characterset;
pub mod data;
pub mod date;
pub mod date_formatter;
pub mod dictionary;
pub mod error;
pub mod file_security;
pub mod filedescriptor;
pub mod locale;
pub mod mach_port;
pub mod messageport;
pub mod notification_center;
pub mod number;
pub mod number_formatter;
pub mod plugin;
pub mod preferences;
pub mod propertylist;
pub mod runloop;
pub mod set;
pub mod socket;
pub mod stream;
pub mod string;
pub mod string_tokenizer;
pub mod timezone;
pub mod tree;
pub mod url;
pub mod url_enumerator;
#[cfg(target_os = "macos")]
pub mod user_notification;
pub mod uuid;
#[cfg(target_os = "macos")]
pub mod xml_node;
#[cfg(target_os = "macos")]
pub mod xml_parser;
