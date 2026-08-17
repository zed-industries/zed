//! Cache operations.

/// Operations used by the `and_compute_with` and similar methods.
pub mod compute {
    use std::sync::Arc;

    use crate::Entry;

    /// Instructs the `and_compute_with` and similar methods how to modify the cached
    /// entry.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Op<V> {
        /// No-operation. Do not modify the cached entry.
        Nop,
        /// Insert or update the value of the cached entry.
        Put(V),
        /// Remove the cached entry.
        Remove,
    }

    /// The result of the `and_compute_with` and similar methods.
    #[derive(Debug)]
    pub enum CompResult<K, V> {
        /// The entry did not exist and still does not exist.
        StillNone(Arc<K>),
        /// The entry already existed and was not modified. The returned entry
        /// contains the existing value.
        Unchanged(Entry<K, V>),
        /// The entry did not exist and was inserted. The returned entry contains
        /// the inserted value.
        Inserted(Entry<K, V>),
        /// The entry already existed and its value was replaced with a new one. The
        /// returned entry contains the new value (not the replaced value).
        ReplacedWith(Entry<K, V>),
        /// The entry already existed and was removed. The returned entry contains
        /// the removed value.
        ///
        /// Note: `StillNone` is returned instead of `Removed` if `Op::Remove` was
        /// requested but the entry did not exist.
        Removed(Entry<K, V>),
    }

    impl<K, V> CompResult<K, V> {
        /// Returns the contained `Some(Entry)` if any. Otherwise returns `None`.
        /// Consumes the `self` value.
        pub fn into_entry(self) -> Option<Entry<K, V>> {
            match self {
                CompResult::StillNone(_) => None,
                CompResult::Unchanged(entry) => Some(entry),
                CompResult::Inserted(entry) => Some(entry),
                CompResult::ReplacedWith(entry) => Some(entry),
                CompResult::Removed(entry) => Some(entry),
            }
        }

        /// Unwraps the contained `Entry`, consuming the `self` value.
        ///
        /// # Panics
        ///
        /// Panics if the `self` value is `StillNone`.
        pub fn unwrap(self) -> Entry<K, V> {
            match self {
                CompResult::StillNone(_) => panic!("`CompResult::unwrap` called on `StillNone`"),
                CompResult::Unchanged(entry) => entry,
                CompResult::Inserted(entry) => entry,
                CompResult::ReplacedWith(entry) => entry,
                CompResult::Removed(entry) => entry,
            }
        }
    }
}
