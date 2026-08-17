//! Provides thread-safe, concurrent cache implementations.

mod base_cache;
mod builder;
mod cache;
mod entry_selector;
mod invalidator;
mod key_lock;
mod segment;
mod value_initializer;

/// The type of the unique ID to identify a predicate used by
/// [`Cache::invalidate_entries_if`][invalidate-if] method.
///
/// A `PredicateId` is a `String` of UUID (version 4).
///
/// [invalidate-if]: ./struct.Cache.html#method.invalidate_entries_if
pub type PredicateId = String;

pub(crate) type PredicateIdStr<'a> = &'a str;

pub use crate::common::iter::Iter;
pub use {
    builder::CacheBuilder,
    cache::Cache,
    entry_selector::{OwnedKeyEntrySelector, RefKeyEntrySelector},
    segment::SegmentedCache,
};

/// Provides extra methods that will be useful for testing.
pub trait ConcurrentCacheExt<K, V> {
    /// Performs any pending maintenance operations needed by the cache.
    fn sync(&self);
}

// Empty struct to be used in `InitResult::InitErr` to represent the Option None.
pub(crate) struct OptionallyNone;

// Empty struct to be used in `InitResult::InitErr`` to represent the Compute None.
pub(crate) struct ComputeNone;
