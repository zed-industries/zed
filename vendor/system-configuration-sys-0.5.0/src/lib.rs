// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Low level bindings to the Apple [SystemConfiguration] framework. Generated with bindgen.
//! For a safe, higher level, API, check out the [`system-configuration`] crate.
//!
//! [SystemConfiguration]: https://developer.apple.com/documentation/systemconfiguration?language=objc
//! [`system-configuration`]: https://crates.io/crates/system-configuration

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

pub use core_foundation_sys;
pub use libc;

/// This is a temporary solution.
pub type dispatch_queue_t = *mut libc::c_void;

pub mod dynamic_store;
pub mod dynamic_store_copy_specific;
pub mod network_configuration;
pub mod network_reachability;
pub mod preferences;
pub mod schema_definitions;
