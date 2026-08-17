use crate::Cached;
use std::cmp::Eq;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(feature = "async")]
use {super::CachedAsync, async_trait::async_trait, futures::Future};

#[cfg(feature = "disk_store")]
mod disk;
mod expiring_sized;
mod expiring_value_cache;
#[cfg(feature = "redis_store")]
mod redis;
mod sized;
mod timed;
mod timed_sized;
mod unbound;

#[cfg(feature = "disk_store")]
pub use crate::stores::disk::{DiskCache, DiskCacheBuildError, DiskCacheBuilder, DiskCacheError};
#[cfg(feature = "redis_store")]
#[cfg_attr(docsrs, doc(cfg(feature = "redis_store")))]
pub use crate::stores::redis::{
    RedisCache, RedisCacheBuildError, RedisCacheBuilder, RedisCacheError,
};
pub use expiring_sized::ExpiringSizedCache;
pub use expiring_value_cache::{CanExpire, ExpiringValueCache};
pub use sized::SizedCache;
pub use timed::TimedCache;
pub use timed_sized::TimedSizedCache;
pub use unbound::UnboundCache;

#[cfg(all(
    feature = "async",
    feature = "redis_store",
    any(feature = "redis_async_std", feature = "redis_tokio")
))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(
        feature = "async",
        feature = "redis_store",
        any(feature = "redis_async_std", feature = "redis_tokio")
    )))
)]
pub use crate::stores::redis::{AsyncRedisCache, AsyncRedisCacheBuilder};

impl<K, V, S> Cached<K, V> for HashMap<K, V, S>
where
    K: Hash + Eq,
    S: std::hash::BuildHasher + Default,
{
    fn cache_get<Q>(&mut self, k: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.get(k)
    }
    fn cache_get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.get_mut(k)
    }
    fn cache_set(&mut self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }
    fn cache_get_or_set_with<F: FnOnce() -> V>(&mut self, key: K, f: F) -> &mut V {
        self.entry(key).or_insert_with(f)
    }
    fn cache_try_get_or_set_with<F: FnOnce() -> Result<V, E>, E>(
        &mut self,
        k: K,
        f: F,
    ) -> Result<&mut V, E> {
        let v = match self.entry(k) {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => vacant.insert(f()?),
        };

        Ok(v)
    }
    fn cache_remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.remove(k)
    }
    fn cache_clear(&mut self) {
        self.clear();
    }
    fn cache_reset(&mut self) {
        *self = HashMap::default();
    }
    fn cache_size(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<K, V, S> CachedAsync<K, V> for HashMap<K, V, S>
where
    K: Hash + Eq + Clone + Send,
    S: std::hash::BuildHasher + Send,
{
    async fn get_or_set_with<F, Fut>(&mut self, k: K, f: F) -> &mut V
    where
        V: Send,
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = V> + Send,
    {
        match self.entry(k) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(f().await),
        }
    }

    async fn try_get_or_set_with<F, Fut, E>(&mut self, k: K, f: F) -> Result<&mut V, E>
    where
        V: Send,
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<V, E>> + Send,
    {
        let v = match self.entry(k) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(f().await?),
        };

        Ok(v)
    }
}

#[cfg(test)]
/// Cache store tests
mod tests {
    use super::*;

    #[test]
    fn hashmap() {
        let mut c = std::collections::HashMap::new();
        assert!(c.cache_get(&1).is_none());
        assert_eq!(c.cache_misses(), None);

        assert_eq!(c.cache_set(1, 100), None);
        assert_eq!(c.cache_get(&1), Some(&100));
        assert_eq!(c.cache_hits(), None);
        assert_eq!(c.cache_misses(), None);
    }
}
