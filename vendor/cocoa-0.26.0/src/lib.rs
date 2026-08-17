// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "cocoa"]
#![crate_type = "rlib"]
#![allow(non_snake_case)]

#[cfg(target_os = "macos")]
pub mod appkit;
pub mod base;
pub mod foundation;
#[cfg(target_os = "macos")]
pub mod quartzcore;
#[macro_use]
mod macros;
