//! A lock-free hash map implemented with bucket pointer arrays, open addressing,
//! and linear probing.

pub(crate) mod bucket;
pub(crate) mod bucket_array_ref;

use std::collections::hash_map::RandomState;

/// Default hasher for `HashMap`.
pub type DefaultHashBuilder = RandomState;
