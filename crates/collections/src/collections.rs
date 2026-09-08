pub type HashMap<K, V> = FxHashMap<K, V>;
pub type HashSet<T> = FxHashSet<T>;
pub type IndexMap<K, V> = indexmap::IndexMap<K, V, rustc_hash::FxBuildHasher>;
pub type IndexSet<T> = indexmap::IndexSet<T, rustc_hash::FxBuildHasher>;
pub type TypeIdHashMap<V> =
    std::collections::HashMap<std::any::TypeId, V, gpui_util::TypeIdHashBuilder>;
pub type TypeIdHashSet = std::collections::HashSet<std::any::TypeId, gpui_util::TypeIdHashBuilder>;

pub use indexmap::Equivalent;
pub use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet, FxHasher};
pub use std::collections::*;

pub mod vecmap;
#[cfg(test)]
mod vecmap_tests;
