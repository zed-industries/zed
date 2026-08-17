// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use tendril;

/// Create a [`SmallCharSet`], with each space-separated number stored in the set.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate markup5ever;
/// # fn main() {
/// let set = small_char_set!(12 54 42);
/// assert_eq!(set.bits,
///            0b00000000_01000000_00000100_00000000_00000000_00000000_00010000_00000000);
/// # }
/// ```
///
/// [`SmallCharSet`]: struct.SmallCharSet.html
#[macro_export]
macro_rules! small_char_set ( ($($e:expr)+) => (
    $ crate ::SmallCharSet {
        bits: $( (1 << ($e as usize)) )|+
    }
));

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub mod data;
#[macro_use]
pub mod interface;
pub mod serialize;
mod util {
    pub mod buffer_queue;
    pub mod smallcharset;
}

pub use interface::{Attribute, ExpandedName, QualName};
pub use util::smallcharset::SmallCharSet;
pub use util::*;
