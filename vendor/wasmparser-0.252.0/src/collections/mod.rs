//! Type definitions for maps and sets used by the `wasmparser` crate.
//!
//! This module contains type definitions for [`Map`], [`Set`], [`IndexMap`], and [`IndexSet`].
//! These types are thin-wrappers around either hash-map based or B-tree-map based data structures.
//! Users can strictly use the `btree`-map based variants by enabling the `no-hash-maps` crate feature.
//!
//! - [`Map`]: Either backed by [`hashbrown::HashMap`] or Rust's [`BTreeMap`].
//! - [`Set`]: Either backed by [`hashbrown::HashSet`] or Rust's [`BTreeSet`].
//! - [`IndexMap`]: Either backed by [`indexmap::IndexMap`] or a custom implementation based on Rust's [`BTreeMap`].
//! - [`IndexSet`]: Either backed by [`indexmap::IndexSet`] or a custom implementation based on Rust's [`BTreeMap`].
//!
//! For the hash-map based type definitions the hash algorithm type parameter is fixed.
//!
//! [`BTreeMap`]: alloc::collections::BTreeMap
//! [`BTreeSet`]: alloc::collections::BTreeSet

// Which collections will be used feature matrix:
//
// `hash-collections` | `prefer-btree-collections` |      usage
// ------------------ | -------------------------- | -------------------
//            false   |                    false   |      btree
//             true   |                    false   |      hash
//            false   |                     true   |      btree
//             true   |                     true   |      btree

#[cfg(feature = "hash-collections")]
pub mod hash;
pub mod index_map;
pub mod index_set;
pub mod map;
pub mod set;

#[doc(inline)]
pub use self::{index_map::IndexMap, index_set::IndexSet, map::Map, set::Set};
