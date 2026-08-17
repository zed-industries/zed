/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cmp::Ordering;
use std::collections::HashSet;

use crate::bindgen::ir::{ItemContainer, Path};

/// A dependency list is used for gathering what order to output the types.
#[derive(Default)]
pub struct Dependencies {
    pub order: Vec<ItemContainer>,
    pub items: HashSet<Path>,
}

impl Dependencies {
    pub fn new() -> Dependencies {
        Dependencies {
            order: Vec::new(),
            items: HashSet::new(),
        }
    }

    pub fn sort(&mut self) {
        // Sort untagged enums and opaque structs into their own layers because they don't
        // depend on each other or anything else.
        let ordering = |a: &ItemContainer, b: &ItemContainer| match (a, b) {
            (ItemContainer::Enum(x), ItemContainer::Enum(y))
                if x.tag.is_none() && y.tag.is_none() =>
            {
                x.path.cmp(&y.path)
            }
            (ItemContainer::Enum(x), _) if x.tag.is_none() => Ordering::Less,
            (_, ItemContainer::Enum(x)) if x.tag.is_none() => Ordering::Greater,

            (ItemContainer::OpaqueItem(x), ItemContainer::OpaqueItem(y)) => x.path.cmp(&y.path),
            (&ItemContainer::OpaqueItem(_), _) => Ordering::Less,
            (_, &ItemContainer::OpaqueItem(_)) => Ordering::Greater,

            _ => Ordering::Equal,
        };

        self.order.sort_by(ordering);
    }
}
