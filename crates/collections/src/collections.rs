#[cfg(feature = "test-support")]
pub type HashMap<K, V> = FxHashMap<K, V>;

#[cfg(feature = "test-support")]
pub type HashSet<T> = FxHashSet<T>;

#[cfg(not(feature = "test-support"))]
pub type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(not(feature = "test-support"))]
pub type HashSet<T> = std::collections::HashSet<T>;

pub use rustc_hash::FxHasher;
pub use rustc_hash::{FxHashMap, FxHashSet};
pub use std::collections::*;
