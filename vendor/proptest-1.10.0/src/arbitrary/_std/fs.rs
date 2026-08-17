//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::fs`.

use std::fs::DirBuilder;

use crate::arbitrary::{any, SMapped};
use crate::strategy::statics::static_map;

// TODO: other parts (figure out workable semantics).

arbitrary!(DirBuilder, SMapped<bool, Self>; {
    static_map(any::<bool>(), |recursive| {
        let mut db = DirBuilder::new();
        db.recursive(recursive);
        db
    })
});

#[cfg(test)]
mod test {
    no_panic_test!(dir_builder => DirBuilder);
}
