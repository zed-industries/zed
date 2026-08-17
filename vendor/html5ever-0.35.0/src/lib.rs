// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "html5ever"]
#![crate_type = "dylib"]
#![cfg_attr(test, deny(warnings))]
#![allow(unused_parens)]
#![warn(unreachable_pub)]

pub use driver::{parse_document, parse_fragment, ParseOpts, Parser};
pub use markup5ever::*;

pub use serialize::serialize;

mod util {
    pub(crate) mod str;
}

pub(crate) mod macros;

pub mod driver;
pub mod serialize;
pub mod tokenizer;
pub mod tree_builder;
