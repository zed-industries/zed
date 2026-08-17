/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::bindgen::ir::Path;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl DeclarationType {
    pub fn to_str(self) -> &'static str {
        match self {
            DeclarationType::Struct => "struct",
            DeclarationType::Enum => "enum",
            DeclarationType::Union => "union",
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum DeclarationType {
    Struct,
    Enum,
    Union,
}

#[derive(Default)]
pub struct DeclarationTypeResolver {
    types: HashMap<Path, Option<DeclarationType>>,
}

impl DeclarationTypeResolver {
    fn insert(&mut self, path: &Path, ty: Option<DeclarationType>) {
        if let Entry::Vacant(vacant_entry) = self.types.entry(path.clone()) {
            vacant_entry.insert(ty);
        }
    }

    pub fn add_enum(&mut self, path: &Path) {
        self.insert(path, Some(DeclarationType::Enum));
    }

    pub fn add_struct(&mut self, path: &Path) {
        self.insert(path, Some(DeclarationType::Struct));
    }

    pub fn add_union(&mut self, path: &Path) {
        self.insert(path, Some(DeclarationType::Union));
    }

    pub fn add_none(&mut self, path: &Path) {
        self.insert(path, None);
    }

    pub fn type_for(&self, path: &Path) -> Option<DeclarationType> {
        *self.types.get(path)?
    }
}
