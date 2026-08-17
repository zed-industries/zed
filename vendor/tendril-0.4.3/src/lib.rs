// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(all(test, feature = "bench"), feature(test))]
//#![cfg_attr(test, deny(warnings))]

#[cfg(feature = "encoding")]
pub extern crate encoding;
#[cfg(feature = "encoding_rs")]
pub extern crate encoding_rs;
#[cfg(all(test, feature = "bench"))]
extern crate test;
#[macro_use]
extern crate mac;
extern crate futf;
extern crate utf8;

pub use fmt::Format;
pub use stream::TendrilSink;
pub use tendril::{Atomic, Atomicity, NonAtomic, SendTendril};
pub use tendril::{ByteTendril, ReadExt, SliceExt, StrTendril, SubtendrilError, Tendril};
pub use utf8_decode::IncompleteUtf8;

pub mod fmt;
pub mod stream;

mod buf32;
mod tendril;
mod utf8_decode;
mod util;

static OFLOW: &'static str = "tendril: overflow in buffer arithmetic";
