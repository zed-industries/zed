// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use objc::runtime;

pub use objc::runtime::{BOOL, NO, YES};

pub type Class = *mut runtime::Class;
#[allow(non_camel_case_types)]
pub type id = *mut runtime::Object;
pub type SEL = runtime::Sel;

#[allow(non_upper_case_globals)]
pub const nil: id = 0 as id;
#[allow(non_upper_case_globals)]
pub const Nil: Class = 0 as Class;

/// A convenience method to convert the name of a selector to the selector object.
#[inline]
pub fn selector(name: &str) -> SEL {
    runtime::Sel::register(name)
}
