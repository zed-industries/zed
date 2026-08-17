/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use indexmap::IndexMap;
use std::mem;

use crate::bindgen::config::Config;
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{
    AnnotationSet, Cfg, Constant, Documentation, Enum, GenericArgument, GenericParams, OpaqueItem,
    Path, Static, Struct, Typedef, Union,
};
use crate::bindgen::library::Library;
use crate::bindgen::monomorph::Monomorphs;

/// An item is any type of rust item besides a function
pub trait Item {
    fn path(&self) -> &Path;
    fn name(&self) -> &str {
        self.path().name()
    }
    fn export_name(&self) -> &str {
        self.name()
    }
    fn cfg(&self) -> Option<&Cfg>;
    fn annotations(&self) -> &AnnotationSet;
    fn annotations_mut(&mut self) -> &mut AnnotationSet;
    fn documentation(&self) -> &Documentation;

    fn container(&self) -> ItemContainer;

    fn collect_declaration_types(&self, _resolver: &mut DeclarationTypeResolver) {
        unimplemented!()
    }
    fn resolve_declaration_types(&mut self, _resolver: &DeclarationTypeResolver) {
        unimplemented!()
    }
    fn generic_params(&self) -> &GenericParams;

    fn is_generic(&self) -> bool {
        !self.generic_params().is_empty()
    }

    fn rename_for_config(&mut self, _config: &Config) {}
    fn add_dependencies(&self, _library: &Library, _out: &mut Dependencies) {}
    fn instantiate_monomorph(
        &self,
        _generics: &[GenericArgument],
        _library: &Library,
        _out: &mut Monomorphs,
    ) {
        unreachable!("Cannot instantiate {} as a generic.", self.name())
    }
}

#[derive(Debug, Clone)]
pub enum ItemContainer {
    Constant(Constant),
    Static(Static),
    OpaqueItem(OpaqueItem),
    Struct(Struct),
    Union(Union),
    Enum(Enum),
    Typedef(Typedef),
}

impl ItemContainer {
    pub fn deref(&self) -> &dyn Item {
        match *self {
            ItemContainer::Constant(ref x) => x,
            ItemContainer::Static(ref x) => x,
            ItemContainer::OpaqueItem(ref x) => x,
            ItemContainer::Struct(ref x) => x,
            ItemContainer::Union(ref x) => x,
            ItemContainer::Enum(ref x) => x,
            ItemContainer::Typedef(ref x) => x,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemValue<T: Item> {
    Cfg(Vec<T>),
    Single(T),
}

#[derive(Debug, Clone)]
pub struct ItemMap<T: Item> {
    data: IndexMap<Path, ItemValue<T>>,
}

impl<T: Item> Default for ItemMap<T> {
    fn default() -> ItemMap<T> {
        ItemMap {
            data: Default::default(),
        }
    }
}

impl<T: Item + Clone> ItemMap<T> {
    pub fn rebuild(&mut self) {
        let old = mem::take(self);
        old.for_all_items(|x| {
            self.try_insert(x.clone());
        });
    }

    pub fn try_insert(&mut self, item: T) -> bool {
        match (item.cfg().is_some(), self.data.get_mut(item.path())) {
            (true, Some(&mut ItemValue::Cfg(ref mut items))) => {
                items.push(item);
                return true;
            }
            (false, Some(&mut ItemValue::Cfg(_))) => {
                return false;
            }
            (true, Some(&mut ItemValue::Single(_))) => {
                return false;
            }
            (false, Some(&mut ItemValue::Single(_))) => {
                return false;
            }
            _ => {}
        }

        let path = item.path().clone();
        if item.cfg().is_some() {
            self.data.insert(path, ItemValue::Cfg(vec![item]));
        } else {
            self.data.insert(path, ItemValue::Single(item));
        }

        true
    }

    pub fn extend_with(&mut self, other: &ItemMap<T>) {
        other.for_all_items(|x| {
            self.try_insert(x.clone());
        });
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.data.len());
        for container in self.data.values() {
            match *container {
                ItemValue::Cfg(ref items) => result.extend_from_slice(items),
                ItemValue::Single(ref item) => {
                    result.push(item.clone());
                }
            }
        }
        result
    }

    pub fn get_items(&self, path: &Path) -> Option<Vec<ItemContainer>> {
        Some(match *self.data.get(path)? {
            ItemValue::Cfg(ref items) => items.iter().map(|x| x.container()).collect(),
            ItemValue::Single(ref item) => vec![item.container()],
        })
    }

    pub fn filter<F>(&mut self, callback: F)
    where
        F: Fn(&T) -> bool,
    {
        self.data.retain(|_, container| match *container {
            ItemValue::Cfg(ref mut items) => {
                items.retain(|item| !callback(item));
                !items.is_empty()
            }
            ItemValue::Single(ref item) => !callback(item),
        });
    }

    pub fn for_all_items<F>(&self, mut callback: F)
    where
        F: FnMut(&T),
    {
        for container in self.data.values() {
            match *container {
                ItemValue::Cfg(ref items) => {
                    for item in items {
                        callback(item);
                    }
                }
                ItemValue::Single(ref item) => callback(item),
            }
        }
    }

    pub fn for_all_items_mut<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut T),
    {
        for container in self.data.values_mut() {
            match *container {
                ItemValue::Cfg(ref mut items) => {
                    for item in items {
                        callback(item);
                    }
                }
                ItemValue::Single(ref mut item) => callback(item),
            }
        }
    }

    pub fn for_items<F>(&self, path: &Path, mut callback: F)
    where
        F: FnMut(&T),
    {
        match self.data.get(path) {
            Some(ItemValue::Cfg(items)) => {
                for item in items {
                    callback(item);
                }
            }
            Some(ItemValue::Single(item)) => {
                callback(item);
            }
            None => {}
        }
    }

    pub fn for_items_mut<F>(&mut self, path: &Path, mut callback: F)
    where
        F: FnMut(&mut T),
    {
        match self.data.get_mut(path) {
            Some(&mut ItemValue::Cfg(ref mut items)) => {
                for item in items {
                    callback(item);
                }
            }
            Some(&mut ItemValue::Single(ref mut item)) => {
                callback(item);
            }
            None => {}
        }
    }
}
