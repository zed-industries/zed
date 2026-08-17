/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Mutex, MutexGuard, OnceLock};

/// A data structure for persisting and sharing state between multiple clients.
///
/// Some state should be shared between multiple clients. For example, when creating multiple clients
/// for the same service, it's desirable to share a client rate limiter. This way, when one client
/// receives a throttling response, the other clients will be aware of it as well.
///
/// Whether clients share state is dependent on their partition key `K`. Going back to the client
/// rate limiter example, `K` would be a struct containing the name of the service as well as the
/// client's configured region, since receiving throttling responses in `us-east-1` shouldn't
/// throttle requests to the same service made in other regions.
///
/// Values stored in a `StaticPartitionMap` will be cloned whenever they are requested. Values must
/// be initialized before they can be retrieved, and the `StaticPartitionMap::get_or_init` method is
/// how you can ensure this.
///
/// # Example
///
/// ```
///use std::sync::{Arc, Mutex};
/// use aws_smithy_runtime::static_partition_map::StaticPartitionMap;
///
/// // The shared state must be `Clone` and will be internally mutable. Deriving `Default` isn't
/// // necessary, but allows us to use the `StaticPartitionMap::get_or_init_default` method.
/// #[derive(Clone, Default)]
/// pub struct SomeSharedState {
///     inner: Arc<Mutex<Inner>>
/// }
///
/// #[derive(Default)]
/// struct Inner {
///     // Some shared state...
/// }
///
/// // `Clone`, `Hash`, and `Eq` are all required trait impls for partition keys
/// #[derive(Clone, Hash, PartialEq, Eq)]
/// pub struct SharedStatePartition {
///     region: String,
///     service_name: String,
/// }
///
/// impl SharedStatePartition {
///     pub fn new(region: impl Into<String>, service_name: impl Into<String>) -> Self {
///         Self { region: region.into(), service_name: service_name.into() }
///     }
/// }
///
/// static SOME_SHARED_STATE: StaticPartitionMap<SharedStatePartition, SomeSharedState> = StaticPartitionMap::new();
///
/// struct Client {
///     shared_state: SomeSharedState,
/// }
///
/// impl Client {
///     pub fn new() -> Self {
///         let key = SharedStatePartition::new("us-east-1", "example_service_20230628");
///         Self {
///             // If the stored value implements `Default`, you can call the
///             // `StaticPartitionMap::get_or_init_default` convenience method.
///             shared_state: SOME_SHARED_STATE.get_or_init_default(key),
///         }
///     }
/// }
/// ```
#[derive(Debug, Default)]
pub struct StaticPartitionMap<K, V> {
    inner: OnceLock<Mutex<HashMap<K, V>>>,
}

impl<K, V> StaticPartitionMap<K, V> {
    /// Creates a new `StaticPartitionMap`.
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }
}

impl<K, V> StaticPartitionMap<K, V>
where
    K: Eq + Hash,
{
    fn get_or_init_inner(&self) -> MutexGuard<'_, HashMap<K, V>> {
        self.inner
            // At the very least, we'll always be storing the default state.
            .get_or_init(|| Mutex::new(HashMap::with_capacity(1)))
            .lock()
            .unwrap()
    }
}

impl<K, V> StaticPartitionMap<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    /// Gets the value for the given partition key.
    #[must_use]
    pub fn get(&self, partition_key: K) -> Option<V> {
        self.get_or_init_inner().get(&partition_key).cloned()
    }

    /// Gets the value for the given partition key, initializing it with `init` if it doesn't exist.
    #[must_use]
    pub fn get_or_init<F>(&self, partition_key: K, init: F) -> V
    where
        F: FnOnce() -> V,
    {
        let mut inner = self.get_or_init_inner();
        let v = inner.entry(partition_key).or_insert_with(init);
        v.clone()
    }
}

impl<K, V> StaticPartitionMap<K, V>
where
    K: Eq + Hash,
    V: Clone + Default,
{
    /// Gets the value for the given partition key, initializing it if it doesn't exist.
    #[must_use]
    pub fn get_or_init_default(&self, partition_key: K) -> V {
        self.get_or_init(partition_key, V::default)
    }
}

#[cfg(test)]
mod tests {
    use super::StaticPartitionMap;

    #[test]
    fn test_keyed_partition_returns_same_value_for_same_key() {
        let kp = StaticPartitionMap::new();
        let _ = kp.get_or_init("A", || "A".to_owned());
        let actual = kp.get_or_init("A", || "B".to_owned());
        let expected = "A".to_owned();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_keyed_partition_returns_different_value_for_different_key() {
        let kp = StaticPartitionMap::new();
        let _ = kp.get_or_init("A", || "A".to_owned());
        let actual = kp.get_or_init("B", || "B".to_owned());

        let expected = "B".to_owned();
        assert_eq!(expected, actual);

        let actual = kp.get("A").unwrap();
        let expected = "A".to_owned();
        assert_eq!(expected, actual);
    }
}
