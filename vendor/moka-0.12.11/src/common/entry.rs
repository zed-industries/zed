use std::{fmt::Debug, sync::Arc};

/// A snapshot of a single entry in the cache.
///
/// `Entry` is constructed from the methods like `or_insert` on the struct returned
/// by cache's `entry` or `entry_by_ref` methods. `Entry` holds the cached key and
/// value at the time it was constructed. It also carries extra information about the
/// entry; [`is_fresh`](#method.is_fresh) method returns `true` if the value was not
/// cached and was freshly computed.
///
/// See the followings for more information about `entry` and `entry_by_ref` methods:
///
/// - `sync::Cache`:
///     - [`entry`](./sync/struct.Cache.html#method.entry)
///     - [`entry_by_ref`](./sync/struct.Cache.html#method.entry_by_ref)
/// - `future::Cache`:
///     - [`entry`](./future/struct.Cache.html#method.entry)
///     - [`entry_by_ref`](./future/struct.Cache.html#method.entry_by_ref)
///
pub struct Entry<K, V> {
    key: Option<Arc<K>>,
    value: V,
    is_fresh: bool,
    is_old_value_replaced: bool,
}

impl<K, V> Debug for Entry<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("key", self.key())
            .field("value", &self.value)
            .field("is_fresh", &self.is_fresh)
            .field("is_old_value_replaced", &self.is_old_value_replaced)
            .finish()
    }
}

impl<K, V> Entry<K, V> {
    pub(crate) fn new(
        key: Option<Arc<K>>,
        value: V,
        is_fresh: bool,
        is_old_value_replaced: bool,
    ) -> Self {
        Self {
            key,
            value,
            is_fresh,
            is_old_value_replaced,
        }
    }

    /// Returns a reference to the wrapped key.
    pub fn key(&self) -> &K {
        self.key.as_ref().expect("Bug: Key is None")
    }

    /// Returns a reference to the wrapped value.
    ///
    /// Note that the returned reference is _not_ pointing to the original value in
    /// the cache. Instead, it is pointing to the cloned value in this `Entry`.
    pub fn value(&self) -> &V {
        &self.value
    }

    /// Consumes this `Entry`, returning the wrapped value.
    ///
    /// Note that the returned value is a clone of the original value in the cache.
    /// It was cloned when this `Entry` was constructed.
    pub fn into_value(self) -> V {
        self.value
    }

    /// Returns `true` if the value in this `Entry` was not cached and was freshly
    /// computed.
    pub fn is_fresh(&self) -> bool {
        self.is_fresh
    }

    /// Returns `true` if an old value existed in the cache and was replaced by the
    /// value in this `Entry`.
    ///
    /// Note that the new value can be the same as the old value. This method still
    /// returns `true` in that case.
    pub fn is_old_value_replaced(&self) -> bool {
        self.is_old_value_replaced
    }
}
