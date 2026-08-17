//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Defines the core traits used by Proptest.

mod filter;
mod filter_map;
mod flatten;
mod fuse;
mod just;
mod lazy;
mod map;
mod recursive;
mod shuffle;
mod traits;
mod unions;

pub use self::filter::*;
pub use self::filter_map::*;
pub use self::flatten::*;
pub use self::fuse::*;
pub use self::just::*;
pub use self::lazy::*;
pub use self::map::*;
pub use self::recursive::*;
pub use self::shuffle::*;
pub use self::traits::*;
pub use self::unions::*;

pub mod statics;
