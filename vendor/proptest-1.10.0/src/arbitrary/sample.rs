//-
// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::arbitrary::Arbitrary;
use crate::sample::{Index, IndexStrategy, Selector, SelectorStrategy};

impl Arbitrary for Index {
    type Parameters = ();

    type Strategy = IndexStrategy;

    fn arbitrary_with(_: ()) -> IndexStrategy {
        IndexStrategy::new()
    }
}

impl Arbitrary for Selector {
    type Parameters = ();

    type Strategy = SelectorStrategy;

    fn arbitrary_with(_: ()) -> SelectorStrategy {
        SelectorStrategy::new()
    }
}
