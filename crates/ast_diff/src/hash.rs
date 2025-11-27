use std::hash::BuildHasherDefault;

use rustc_hash::{FxHashSet, FxHasher};

/// A fast hashmap with no hash DoS protection. This is used in
/// extremely hot code.
///
/// Wrapping FxHasher (the fastest hash algorithm in benchmarks) in a
/// hashbrown::HashMap rather than std HashMap is a little faster, and
/// it also allows us to use the entry_ref API which is unavailable in
/// stable Rust.
pub(crate) type DftHashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<FxHasher>>;

/// A fast hash set with no hash DoS protection. This is a simple
/// alias, but added for consistency with `DftHashMap`.
pub(crate) type DftHashSet<V> = FxHashSet<V>;
