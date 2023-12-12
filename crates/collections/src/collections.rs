#[cfg(feature = "test-support")]
pub type HashMap<K, V> = FxHashMap<K, V>;

#[cfg(feature = "test-support")]
pub type HashSet<T> = FxHashSet<T>;

#[cfg(not(feature = "test-support"))]
pub type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(not(feature = "test-support"))]
pub type HashSet<T> = std::collections::HashSet<T>;

pub use rustc_hash::{FxHashMap, FxHashSet};
use std::any::TypeId;
pub use std::collections::*;

// NEW TYPES

#[derive(Default)]
pub struct CommandPaletteFilter {
    pub hidden_namespaces: HashSet<&'static str>,
    pub hidden_action_types: HashSet<TypeId>,
}
