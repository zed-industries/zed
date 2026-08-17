//! Archived versions of standard library containers.

pub mod btree_map;
pub mod btree_set;
pub mod hash_index;
pub mod hash_map;
pub mod hash_set;
// TODO: move these into a separate crate when indexmap adds rkyv support
pub mod index_map;
pub mod index_set;
pub mod util;

pub use self::btree_map::ArchivedBTreeMap;
pub use self::hash_index::ArchivedHashIndex;
pub use self::hash_map::ArchivedHashMap;
pub use self::hash_set::ArchivedHashSet;
// TODO: move these into a separate crate when indexmap adds rkyv support
pub use self::index_map::ArchivedIndexMap;
pub use self::index_set::ArchivedIndexSet;
