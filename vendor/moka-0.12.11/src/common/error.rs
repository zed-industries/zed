use std::{error::Error, fmt::Display};

/// The error type for the functionalities around
/// [`Cache::invalidate_entries_if`][invalidate-if] method.
///
/// [invalidate-if]: ./sync/struct.Cache.html#method.invalidate_entries_if
#[derive(Debug)]
pub enum PredicateError {
    /// This cache does not have a necessary configuration enabled to support
    /// invalidating entries with a closure.
    ///
    /// To enable the configuration, call
    /// [`CacheBuilder::support_invalidation_closures`][support-invalidation-closures]
    /// method at the cache creation time.
    ///
    /// [support-invalidation-closures]: ./sync/struct.CacheBuilder.html#method.support_invalidation_closures
    InvalidationClosuresDisabled,
}

impl Display for PredicateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Support for invalidation closures is disabled in this cache. \
            Please enable it by calling the support_invalidation_closures \
            method of the builder at the cache creation time",
        )
    }
}

impl Error for PredicateError {}
