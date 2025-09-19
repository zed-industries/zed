#[cfg(feature = "test-support")]
pub type HashMap<K, V> = FxHashMap<K, V>;

#[cfg(feature = "test-support")]
pub type HashSet<T> = FxHashSet<T>;

#[cfg(feature = "test-support")]
pub type IndexMap<K, V> = indexmap::IndexMap<K, V, rustc_hash::FxBuildHasher>;

#[cfg(feature = "test-support")]
pub type IndexSet<T> = indexmap::IndexSet<T, rustc_hash::FxBuildHasher>;

#[cfg(not(feature = "test-support"))]
pub type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(not(feature = "test-support"))]
pub type HashSet<T> = std::collections::HashSet<T>;

#[cfg(not(feature = "test-support"))]
pub type IndexMap<K, V> = indexmap::IndexMap<K, V>;

#[cfg(not(feature = "test-support"))]
pub type IndexSet<T> = indexmap::IndexSet<T>;

pub use indexmap::Equivalent;
pub use rustc_hash::FxHasher;
pub use rustc_hash::{FxHashMap, FxHashSet};
pub use std::collections::*;
