use crate::IOCached;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use std::{fmt::Display, time::Duration};

pub struct RedisCacheBuilder<K, V> {
    ttl: Duration,
    refresh: bool,
    namespace: String,
    prefix: String,
    connection_string: Option<String>,
    pool_max_size: Option<u32>,
    pool_min_idle: Option<u32>,
    pool_max_lifetime: Option<std::time::Duration>,
    pool_idle_timeout: Option<std::time::Duration>,
    _phantom: PhantomData<(K, V)>,
}

const ENV_KEY: &str = "CACHED_REDIS_CONNECTION_STRING";
const DEFAULT_NAMESPACE: &str = "cached-redis-store:";

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisCacheBuildError {
    #[error("redis connection error")]
    Connection(#[from] redis::RedisError),
    #[error("redis pool error")]
    Pool(#[from] r2d2::Error),
    #[error("Connection string not specified or invalid in env var {env_key:?}: {error:?}")]
    MissingConnectionString {
        env_key: String,
        error: std::env::VarError,
    },
}

impl<K, V> RedisCacheBuilder<K, V>
where
    K: Display,
    V: Serialize + DeserializeOwned,
{
    /// Initialize a `RedisCacheBuilder`
    pub fn new<S: AsRef<str>>(prefix: S, ttl: Duration) -> RedisCacheBuilder<K, V> {
        Self {
            ttl,
            refresh: false,
            namespace: DEFAULT_NAMESPACE.to_string(),
            prefix: prefix.as_ref().to_string(),
            connection_string: None,
            pool_max_size: None,
            pool_min_idle: None,
            pool_max_lifetime: None,
            pool_idle_timeout: None,
            _phantom: PhantomData,
        }
    }

    /// Specify the cache TTL/lifespan in seconds
    #[must_use]
    pub fn set_lifespan(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }

    /// Specify whether cache hits refresh the TTL
    #[must_use]
    pub fn set_refresh(mut self, refresh: bool) -> Self {
        self.refresh = refresh;
        self
    }

    /// Set the namespace for cache keys. Defaults to `cached-redis-store:`.
    /// Used to generate keys formatted as: `{namespace}{prefix}{key}`
    /// Note that no delimiters are implicitly added so you may pass
    /// an empty string if you want there to be no namespace on keys.
    #[must_use]
    pub fn set_namespace<S: AsRef<str>>(mut self, namespace: S) -> Self {
        self.namespace = namespace.as_ref().to_string();
        self
    }

    /// Set the prefix for cache keys.
    /// Used to generate keys formatted as: `{namespace}{prefix}{key}`
    /// Note that no delimiters are implicitly added so you may pass
    /// an empty string if you want there to be no prefix on keys.
    #[must_use]
    pub fn set_prefix<S: AsRef<str>>(mut self, prefix: S) -> Self {
        self.prefix = prefix.as_ref().to_string();
        self
    }

    /// Set the connection string for redis
    #[must_use]
    pub fn set_connection_string(mut self, cs: &str) -> Self {
        self.connection_string = Some(cs.to_string());
        self
    }

    /// Set the max size of the underlying redis connection pool
    #[must_use]
    pub fn set_connection_pool_max_size(mut self, max_size: u32) -> Self {
        self.pool_max_size = Some(max_size);
        self
    }

    /// Set the minimum number of idle redis connections that should be maintained by the
    /// underlying redis connection pool
    #[must_use]
    pub fn set_connection_pool_min_idle(mut self, min_idle: u32) -> Self {
        self.pool_min_idle = Some(min_idle);
        self
    }

    /// Set the max lifetime of connections used by the underlying redis connection pool
    #[must_use]
    pub fn set_connection_pool_max_lifetime(mut self, max_lifetime: std::time::Duration) -> Self {
        self.pool_max_lifetime = Some(max_lifetime);
        self
    }

    /// Set the max lifetime of idle connections maintained by the underlying redis connection pool
    #[must_use]
    pub fn set_connection_pool_idle_timeout(mut self, idle_timeout: std::time::Duration) -> Self {
        self.pool_idle_timeout = Some(idle_timeout);
        self
    }

    /// Return the current connection string or load from the env var: `CACHED_REDIS_CONNECTION_STRING`
    ///
    /// # Errors
    ///
    /// Will return `RedisCacheBuildError::MissingConnectionString` if connection string is not set
    pub fn connection_string(&self) -> Result<String, RedisCacheBuildError> {
        match self.connection_string {
            Some(ref s) => Ok(s.to_string()),
            None => {
                std::env::var(ENV_KEY).map_err(|e| RedisCacheBuildError::MissingConnectionString {
                    env_key: ENV_KEY.to_string(),
                    error: e,
                })
            }
        }
    }

    fn create_pool(&self) -> Result<r2d2::Pool<redis::Client>, RedisCacheBuildError> {
        let s = self.connection_string()?;
        let client: redis::Client = redis::Client::open(s)?;
        // some pool-builder defaults are set when the builder is initialized
        // so we can't overwrite any values with Nones...
        let pool_builder = r2d2::Pool::builder();
        let pool_builder = if let Some(max_size) = self.pool_max_size {
            pool_builder.max_size(max_size)
        } else {
            pool_builder
        };
        let pool_builder = if let Some(min_idle) = self.pool_min_idle {
            pool_builder.min_idle(Some(min_idle))
        } else {
            pool_builder
        };
        let pool_builder = if let Some(max_lifetime) = self.pool_max_lifetime {
            pool_builder.max_lifetime(Some(max_lifetime))
        } else {
            pool_builder
        };
        let pool_builder = if let Some(idle_timeout) = self.pool_idle_timeout {
            pool_builder.idle_timeout(Some(idle_timeout))
        } else {
            pool_builder
        };

        let pool: r2d2::Pool<redis::Client> = pool_builder.build(client)?;
        Ok(pool)
    }

    /// The last step in building a `RedisCache` is to call `build()`
    ///
    /// # Errors
    ///
    /// Will return a `RedisCacheBuildError`, depending on the error
    pub fn build(self) -> Result<RedisCache<K, V>, RedisCacheBuildError> {
        Ok(RedisCache {
            ttl: self.ttl,
            refresh: self.refresh,
            connection_string: self.connection_string()?,
            pool: self.create_pool()?,
            namespace: self.namespace,
            prefix: self.prefix,
            _phantom: PhantomData,
        })
    }
}

/// Cache store backed by redis
///
/// Values have a ttl applied and enforced by redis.
/// Uses an r2d2 connection pool under the hood.
pub struct RedisCache<K, V> {
    pub(super) ttl: Duration,
    pub(super) refresh: bool,
    pub(super) namespace: String,
    pub(super) prefix: String,
    connection_string: String,
    pool: r2d2::Pool<redis::Client>,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> RedisCache<K, V>
where
    K: Display,
    V: Serialize + DeserializeOwned,
{
    #[allow(clippy::new_ret_no_self)]
    /// Initialize a `RedisCacheBuilder`
    pub fn new<S: AsRef<str>>(prefix: S, ttl: Duration) -> RedisCacheBuilder<K, V> {
        RedisCacheBuilder::new(prefix, ttl)
    }

    fn generate_key(&self, key: &K) -> String {
        format!("{}{}{}", self.namespace, self.prefix, key)
    }

    /// Return the redis connection string used
    #[must_use]
    pub fn connection_string(&self) -> String {
        self.connection_string.clone()
    }
}

#[derive(Error, Debug)]
pub enum RedisCacheError {
    #[error("redis error")]
    RedisCacheError(#[from] redis::RedisError),
    #[error("redis pool error")]
    PoolError(#[from] r2d2::Error),
    #[error("Error deserializing cached value {cached_value:?}: {error:?}")]
    CacheDeserializationError {
        cached_value: String,
        error: serde_json::Error,
    },
    #[error("Error serializing cached value: {error:?}")]
    CacheSerializationError { error: serde_json::Error },
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CachedRedisValue<V> {
    pub(crate) value: V,
    pub(crate) version: Option<u64>,
}
impl<V> CachedRedisValue<V> {
    fn new(value: V) -> Self {
        Self {
            value,
            version: Some(1),
        }
    }
}

impl<K, V> IOCached<K, V> for RedisCache<K, V>
where
    K: Display,
    V: Serialize + DeserializeOwned,
{
    type Error = RedisCacheError;

    fn cache_get(&self, key: &K) -> Result<Option<V>, RedisCacheError> {
        let mut conn = self.pool.get()?;
        let mut pipe = redis::pipe();
        let key = self.generate_key(key);

        pipe.get(&key);
        if self.refresh {
            pipe.expire(key, self.ttl.as_secs() as i64).ignore();
        }
        // ugh: https://github.com/mitsuhiko/redis-rs/pull/388#issuecomment-910919137
        let res: (Option<String>,) = pipe.query(&mut *conn)?;
        match res.0 {
            None => Ok(None),
            Some(s) => {
                let v: CachedRedisValue<V> = serde_json::from_str(&s).map_err(|e| {
                    RedisCacheError::CacheDeserializationError {
                        cached_value: s,
                        error: e,
                    }
                })?;
                Ok(Some(v.value))
            }
        }
    }

    fn cache_set(&self, key: K, val: V) -> Result<Option<V>, RedisCacheError> {
        let mut conn = self.pool.get()?;
        let mut pipe = redis::pipe();
        let key = self.generate_key(&key);

        let val = CachedRedisValue::new(val);
        pipe.get(&key);
        pipe.set_ex::<String, String>(
            key,
            serde_json::to_string(&val)
                .map_err(|e| RedisCacheError::CacheSerializationError { error: e })?,
            self.ttl.as_secs(),
        )
        .ignore();

        let res: (Option<String>,) = pipe.query(&mut *conn)?;
        match res.0 {
            None => Ok(None),
            Some(s) => {
                let v: CachedRedisValue<V> = serde_json::from_str(&s).map_err(|e| {
                    RedisCacheError::CacheDeserializationError {
                        cached_value: s,
                        error: e,
                    }
                })?;
                Ok(Some(v.value))
            }
        }
    }

    fn cache_remove(&self, key: &K) -> Result<Option<V>, RedisCacheError> {
        let mut conn = self.pool.get()?;
        let mut pipe = redis::pipe();
        let key = self.generate_key(key);

        pipe.get(&key);
        pipe.del::<String>(key).ignore();
        let res: (Option<String>,) = pipe.query(&mut *conn)?;
        match res.0 {
            None => Ok(None),
            Some(s) => {
                let v: CachedRedisValue<V> = serde_json::from_str(&s).map_err(|e| {
                    RedisCacheError::CacheDeserializationError {
                        cached_value: s,
                        error: e,
                    }
                })?;
                Ok(Some(v.value))
            }
        }
    }

    fn cache_lifespan(&self) -> Option<Duration> {
        Some(self.ttl)
    }

    fn cache_set_lifespan(&mut self, ttl: Duration) -> Option<Duration> {
        let old = self.ttl;
        self.ttl = ttl;
        Some(old)
    }

    fn cache_set_refresh(&mut self, refresh: bool) -> bool {
        let old = self.refresh;
        self.refresh = refresh;
        old
    }
}

#[cfg(all(
    feature = "async",
    any(feature = "redis_async_std", feature = "redis_tokio")
))]
mod async_redis {
    use std::time::Duration;

    use super::{
        CachedRedisValue, DeserializeOwned, Display, PhantomData, RedisCacheBuildError,
        RedisCacheError, Serialize, DEFAULT_NAMESPACE, ENV_KEY,
    };
    use {crate::IOCachedAsync, async_trait::async_trait};

    pub struct AsyncRedisCacheBuilder<K, V> {
        ttl: Duration,
        refresh: bool,
        namespace: String,
        prefix: String,
        connection_string: Option<String>,
        _phantom: PhantomData<(K, V)>,
    }

    impl<K, V> AsyncRedisCacheBuilder<K, V>
    where
        K: Display,
        V: Serialize + DeserializeOwned,
    {
        /// Initialize a `RedisCacheBuilder`
        pub fn new<S: AsRef<str>>(prefix: S, ttl: Duration) -> AsyncRedisCacheBuilder<K, V> {
            Self {
                ttl,
                refresh: false,
                namespace: DEFAULT_NAMESPACE.to_string(),
                prefix: prefix.as_ref().to_string(),
                connection_string: None,
                _phantom: PhantomData,
            }
        }

        /// Specify the cache TTL/lifespan in seconds
        #[must_use]
        pub fn set_lifespan(mut self, ttl: Duration) -> Self {
            self.ttl = ttl;
            self
        }

        /// Specify whether cache hits refresh the TTL
        #[must_use]
        pub fn set_refresh(mut self, refresh: bool) -> Self {
            self.refresh = refresh;
            self
        }

        /// Set the namespace for cache keys. Defaults to `cached-redis-store:`.
        /// Used to generate keys formatted as: `{namespace}{prefix}{key}`
        /// Note that no delimiters are implicitly added so you may pass
        /// an empty string if you want there to be no namespace on keys.
        #[must_use]
        pub fn set_namespace<S: AsRef<str>>(mut self, namespace: S) -> Self {
            self.namespace = namespace.as_ref().to_string();
            self
        }

        /// Set the prefix for cache keys
        /// Used to generate keys formatted as: `{namespace}{prefix}{key}`
        /// Note that no delimiters are implicitly added so you may pass
        /// an empty string if you want there to be no prefix on keys.
        #[must_use]
        pub fn set_prefix<S: AsRef<str>>(mut self, prefix: S) -> Self {
            self.prefix = prefix.as_ref().to_string();
            self
        }

        /// Set the connection string for redis
        #[must_use]
        pub fn set_connection_string(mut self, cs: &str) -> Self {
            self.connection_string = Some(cs.to_string());
            self
        }

        /// Return the current connection string or load from the env var: `CACHED_REDIS_CONNECTION_STRING`
        ///
        /// # Errors
        ///
        /// Will return `RedisCacheBuildError::MissingConnectionString` if connection string is not set
        pub fn connection_string(&self) -> Result<String, RedisCacheBuildError> {
            match self.connection_string {
                Some(ref s) => Ok(s.to_string()),
                None => std::env::var(ENV_KEY).map_err(|e| {
                    RedisCacheBuildError::MissingConnectionString {
                        env_key: ENV_KEY.to_string(),
                        error: e,
                    }
                }),
            }
        }

        /// Create a multiplexed redis connection. This is a single connection that can
        /// be used asynchronously by multiple futures.
        #[cfg(not(feature = "redis_connection_manager"))]
        async fn create_multiplexed_connection(
            &self,
        ) -> Result<redis::aio::MultiplexedConnection, RedisCacheBuildError> {
            let s = self.connection_string()?;
            let client = redis::Client::open(s)?;
            let conn = client.get_multiplexed_async_connection().await?;
            Ok(conn)
        }

        /// Create a multiplexed connection wrapped in a manager. The manager provides access
        /// to a multiplexed connection and will automatically reconnect to the server when
        /// necessary.
        #[cfg(feature = "redis_connection_manager")]
        async fn create_connection_manager(
            &self,
        ) -> Result<redis::aio::ConnectionManager, RedisCacheBuildError> {
            let s = self.connection_string()?;
            let client = redis::Client::open(s)?;
            let conn = redis::aio::ConnectionManager::new(client).await?;
            Ok(conn)
        }

        /// The last step in building a `RedisCache` is to call `build()`
        ///
        /// # Errors
        ///
        /// Will return a `RedisCacheBuildError`, depending on the error
        pub async fn build(self) -> Result<AsyncRedisCache<K, V>, RedisCacheBuildError> {
            Ok(AsyncRedisCache {
                ttl: self.ttl,
                refresh: self.refresh,
                connection_string: self.connection_string()?,
                #[cfg(not(feature = "redis_connection_manager"))]
                connection: self.create_multiplexed_connection().await?,
                #[cfg(feature = "redis_connection_manager")]
                connection: self.create_connection_manager().await?,
                namespace: self.namespace,
                prefix: self.prefix,
                _phantom: PhantomData,
            })
        }
    }

    /// Cache store backed by redis
    ///
    /// Values have a ttl applied and enforced by redis.
    /// Uses a `redis::aio::MultiplexedConnection` or `redis::aio::ConnectionManager`
    /// under the hood depending if feature `redis_connection_manager` is used or not.
    pub struct AsyncRedisCache<K, V> {
        pub(super) ttl: Duration,
        pub(super) refresh: bool,
        pub(super) namespace: String,
        pub(super) prefix: String,
        connection_string: String,
        #[cfg(not(feature = "redis_connection_manager"))]
        connection: redis::aio::MultiplexedConnection,
        #[cfg(feature = "redis_connection_manager")]
        connection: redis::aio::ConnectionManager,
        _phantom: PhantomData<(K, V)>,
    }

    impl<K, V> AsyncRedisCache<K, V>
    where
        K: Display + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        #[allow(clippy::new_ret_no_self)]
        /// Initialize an `AsyncRedisCacheBuilder`
        pub fn new<S: AsRef<str>>(prefix: S, ttl: Duration) -> AsyncRedisCacheBuilder<K, V> {
            AsyncRedisCacheBuilder::new(prefix, ttl)
        }

        fn generate_key(&self, key: &K) -> String {
            format!("{}{}{}", self.namespace, self.prefix, key)
        }

        /// Return the redis connection string used
        #[must_use]
        pub fn connection_string(&self) -> String {
            self.connection_string.clone()
        }
    }

    #[async_trait]
    impl<K, V> IOCachedAsync<K, V> for AsyncRedisCache<K, V>
    where
        K: Display + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        type Error = RedisCacheError;

        /// Get a cached value
        async fn cache_get(&self, key: &K) -> Result<Option<V>, Self::Error> {
            let mut conn = self.connection.clone();
            let mut pipe = redis::pipe();
            let key = self.generate_key(key);

            pipe.get(&key);
            if self.refresh {
                pipe.expire(key, self.ttl.as_secs() as i64).ignore();
            }
            let res: (Option<String>,) = pipe.query_async(&mut conn).await?;
            match res.0 {
                None => Ok(None),
                Some(s) => {
                    let v: CachedRedisValue<V> = serde_json::from_str(&s).map_err(|e| {
                        RedisCacheError::CacheDeserializationError {
                            cached_value: s,
                            error: e,
                        }
                    })?;
                    Ok(Some(v.value))
                }
            }
        }

        /// Set a cached value
        async fn cache_set(&self, key: K, val: V) -> Result<Option<V>, Self::Error> {
            let mut conn = self.connection.clone();
            let mut pipe = redis::pipe();
            let key = self.generate_key(&key);

            let val = CachedRedisValue::new(val);
            pipe.get(&key);
            pipe.set_ex::<String, String>(
                key,
                serde_json::to_string(&val)
                    .map_err(|e| RedisCacheError::CacheSerializationError { error: e })?,
                self.ttl.as_secs(),
            )
            .ignore();

            let res: (Option<String>,) = pipe.query_async(&mut conn).await?;
            match res.0 {
                None => Ok(None),
                Some(s) => {
                    let v: CachedRedisValue<V> = serde_json::from_str(&s).map_err(|e| {
                        RedisCacheError::CacheDeserializationError {
                            cached_value: s,
                            error: e,
                        }
                    })?;
                    Ok(Some(v.value))
                }
            }
        }

        /// Remove a cached value
        async fn cache_remove(&self, key: &K) -> Result<Option<V>, Self::Error> {
            let mut conn = self.connection.clone();
            let mut pipe = redis::pipe();
            let key = self.generate_key(key);

            pipe.get(&key);
            pipe.del::<String>(key).ignore();
            let res: (Option<String>,) = pipe.query_async(&mut conn).await?;
            match res.0 {
                None => Ok(None),
                Some(s) => {
                    let v: CachedRedisValue<V> = serde_json::from_str(&s).map_err(|e| {
                        RedisCacheError::CacheDeserializationError {
                            cached_value: s,
                            error: e,
                        }
                    })?;
                    Ok(Some(v.value))
                }
            }
        }

        /// Set the flag to control whether cache hits refresh the ttl of cached values, returns the old flag value
        fn cache_set_refresh(&mut self, refresh: bool) -> bool {
            let old = self.refresh;
            self.refresh = refresh;
            old
        }

        /// Return the lifespan of cached values (time to eviction)
        fn cache_lifespan(&self) -> Option<Duration> {
            Some(self.ttl)
        }

        /// Set the lifespan of cached values, returns the old value
        fn cache_set_lifespan(&mut self, ttl: Duration) -> Option<Duration> {
            let old = self.ttl;
            self.ttl = ttl;
            Some(old)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::thread::sleep;
        use std::time::Duration;

        fn now_millis() -> u128 {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        }

        #[tokio::test]
        async fn test_async_redis_cache() {
            let mut c: AsyncRedisCache<u32, u32> = AsyncRedisCache::new(
                format!("{}:async-redis-cache-test", now_millis()),
                Duration::from_secs(2),
            )
            .build()
            .await
            .unwrap();

            assert!(c.cache_get(&1).await.unwrap().is_none());

            assert!(c.cache_set(1, 100).await.unwrap().is_none());
            assert!(c.cache_get(&1).await.unwrap().is_some());

            sleep(Duration::new(2, 500_000));
            assert!(c.cache_get(&1).await.unwrap().is_none());

            let old = c.cache_set_lifespan(Duration::from_secs(1)).unwrap();
            assert_eq!(2, old.as_secs());
            assert!(c.cache_set(1, 100).await.unwrap().is_none());
            assert!(c.cache_get(&1).await.unwrap().is_some());

            sleep(Duration::new(1, 600_000));
            assert!(c.cache_get(&1).await.unwrap().is_none());

            c.cache_set_lifespan(Duration::from_secs(10)).unwrap();
            assert!(c.cache_set(1, 100).await.unwrap().is_none());
            assert!(c.cache_set(2, 100).await.unwrap().is_none());
            assert_eq!(c.cache_get(&1).await.unwrap().unwrap(), 100);
            assert_eq!(c.cache_get(&1).await.unwrap().unwrap(), 100);
        }
    }
}

#[cfg(all(
    feature = "async",
    any(feature = "redis_async_std", feature = "redis_tokio")
))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(
        feature = "async",
        any(feature = "redis_async_std", feature = "redis_tokio")
    )))
)]
pub use async_redis::{AsyncRedisCache, AsyncRedisCacheBuilder};

#[cfg(test)]
/// Cache store tests
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use super::*;

    fn now_millis() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    #[test]
    fn redis_cache() {
        let mut c: RedisCache<u32, u32> = RedisCache::new(
            format!("{}:redis-cache-test", now_millis()),
            Duration::from_secs(2),
        )
        .set_namespace("in-tests:")
        .build()
        .unwrap();

        assert!(c.cache_get(&1).unwrap().is_none());

        assert!(c.cache_set(1, 100).unwrap().is_none());
        assert!(c.cache_get(&1).unwrap().is_some());

        sleep(Duration::new(2, 500_000));
        assert!(c.cache_get(&1).unwrap().is_none());

        let old = c.cache_set_lifespan(Duration::from_secs(1)).unwrap();
        assert_eq!(2, old.as_secs());
        assert!(c.cache_set(1, 100).unwrap().is_none());
        assert!(c.cache_get(&1).unwrap().is_some());

        sleep(Duration::new(1, 600_000));
        assert!(c.cache_get(&1).unwrap().is_none());

        c.cache_set_lifespan(Duration::from_secs(10)).unwrap();
        assert!(c.cache_set(1, 100).unwrap().is_none());
        assert!(c.cache_set(2, 100).unwrap().is_none());
        assert_eq!(c.cache_get(&1).unwrap().unwrap(), 100);
        assert_eq!(c.cache_get(&1).unwrap().unwrap(), 100);
    }

    #[test]
    fn remove() {
        let c: RedisCache<u32, u32> = RedisCache::new(
            format!("{}:redis-cache-test-remove", now_millis()),
            Duration::from_secs(3600),
        )
        .build()
        .unwrap();

        assert!(c.cache_set(1, 100).unwrap().is_none());
        assert!(c.cache_set(2, 200).unwrap().is_none());
        assert!(c.cache_set(3, 300).unwrap().is_none());

        assert_eq!(100, c.cache_remove(&1).unwrap().unwrap());
    }
}
