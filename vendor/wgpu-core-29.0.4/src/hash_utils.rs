//! Module for hashing utilities.
//!
//! Named hash_utils to prevent clashing with the core::hash module.

/// HashMap using a fast, non-cryptographic hash algorithm.
pub type FastHashMap<K, V> =
    hashbrown::HashMap<K, V, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
/// HashSet using a fast, non-cryptographic hash algorithm.
pub type FastHashSet<K> =
    hashbrown::HashSet<K, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

/// IndexMap using a fast, non-cryptographic hash algorithm.
pub type FastIndexMap<K, V> =
    indexmap::IndexMap<K, V, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
