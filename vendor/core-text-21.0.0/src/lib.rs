// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "core_text"]
#![crate_type = "rlib"]
#![allow(non_snake_case)]

/*!
Many of these functions will add objects to the autorelease pool.
If you don't have one this will cause leaks.
*/

pub mod font;
pub mod font_collection;
pub mod font_descriptor;
pub mod font_manager;
pub mod frame;
pub mod framesetter;
pub mod line;
pub mod run;
pub mod string_attributes;
