use crate::IOCached;
use directories::BaseDirs;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sled::Db;
use std::marker::PhantomData;
use std::path::Path;
use std::{path::PathBuf, time::SystemTime};
use web_time::Duration;

pub struct DiskCacheBuilder<K, V> {
    ttl: Option<Duration>,
    refresh: bool,
    sync_to_disk_on_cache_change: bool,
    disk_dir: Option<PathBuf>,
    cache_name: String,
    connection_config: Option<sled::Config>,
    _phantom: PhantomData<(K, V)>,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiskCacheBuildError {
    #[error("Storage connection error")]
    ConnectionError(#[from] sled::Error),
    #[error("Connection string not specified or invalid in env var {env_key:?}: {error:?}")]
    MissingDiskPath {
        env_key: String,
        error: std::env::VarError,
    },
}

static DISK_FILE_PREFIX: &str = "cached_disk_cache";
const DISK_FILE_VERSION: u64 = 1;

impl<K, V> DiskCacheBuilder<K, V>
where
    K: ToString,
    V: Serialize + DeserializeOwned,
{
    /// Initialize a `DiskCacheBuilder`
    pub fn new<S: AsRef<str>>(cache_name: S) -> DiskCacheBuilder<K, V> {
        Self {
            ttl: None,
            refresh: false,
            sync_to_disk_on_cache_change: false,
            disk_dir: None,
            cache_name: cache_name.as_ref().to_string(),
            connection_config: None,
            _phantom: Default::default(),
        }
    }

    /// Specify the cache TTL/lifespan in seconds
    pub fn set_lifespan(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// Specify whether cache hits refresh the TTL
    pub fn set_refresh(mut self, refresh: bool) -> Self {
        self.refresh = refresh;
        self
    }

    /// Set the disk path for where the data will be stored
    pub fn set_disk_directory<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.disk_dir = Some(dir.as_ref().into());
        self
    }

    /// Specify whether the cache should sync to disk on each cache change.
    /// [sled] flushes every [sled::Config::flush_every_ms] which has a default value.
    /// In some use cases, the default value may not be quick enough,
    /// or a user may want to reduce the flush rate / turn off auto-flushing to reduce IO (and only flush on cache changes).
    /// (see [DiskCacheBuilder::set_connection_config] for more control over the sled connection)
    pub fn set_sync_to_disk_on_cache_change(mut self, sync_to_disk_on_cache_change: bool) -> Self {
        self.sync_to_disk_on_cache_change = sync_to_disk_on_cache_change;
        self
    }

    /// Specify the [sled::Config] to use for the connection to the disk cache.
    ///
    /// ### Note
    /// Don't use [sled::Config::path] as any value set here will be overwritten by either
    /// the path specified in [DiskCacheBuilder::set_disk_directory], or the default value calculated by [DiskCacheBuilder].
    ///
    /// ### Example Use Case
    /// By default [sled] automatically syncs to disk at a frequency specified in [sled::Config::flush_every_ms].
    /// A user may want to reduce IO by setting a lower flush frequency, or by setting [sled::Config::flush_every_ms] to [None].
    /// Also see [DiskCacheBuilder::set_sync_to_disk_on_cache_change] which allows for syncing to disk on each cache change.
    /// ```rust
    /// use cached::stores::{DiskCacheBuilder, DiskCache};
    ///
    /// let config = sled::Config::new().flush_every_ms(None);
    /// let cache: DiskCache<String, String> = DiskCacheBuilder::new("my-cache")
    ///     .set_connection_config(config)
    ///     .set_sync_to_disk_on_cache_change(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn set_connection_config(mut self, config: sled::Config) -> Self {
        self.connection_config = Some(config);
        self
    }

    fn default_disk_dir() -> PathBuf {
        BaseDirs::new()
            .map(|base_dirs| {
                let exe_name = std::env::current_exe()
                    .ok()
                    .and_then(|path| {
                        path.file_name()
                            .and_then(|os_str| os_str.to_str().map(|s| format!("{}_", s)))
                    })
                    .unwrap_or_default();
                let dir_prefix = format!("{}{}", exe_name, DISK_FILE_PREFIX);
                base_dirs.cache_dir().join(dir_prefix)
            })
            .unwrap_or_else(|| {
                std::env::current_dir().expect("disk cache unable to determine current directory")
            })
    }

    pub fn build(self) -> Result<DiskCache<K, V>, DiskCacheBuildError> {
        let disk_dir = self.disk_dir.unwrap_or_else(|| Self::default_disk_dir());
        let disk_path = disk_dir.join(format!("{}_v{}", self.cache_name, DISK_FILE_VERSION));
        let connection = match self.connection_config {
            Some(config) => config.path(disk_path.clone()).open()?,
            None => sled::open(disk_path.clone())?,
        };

        Ok(DiskCache {
            ttl: self.ttl,
            refresh: self.refresh,
            sync_to_disk_on_cache_change: self.sync_to_disk_on_cache_change,
            version: DISK_FILE_VERSION,
            disk_path,
            connection,
            _phantom: self._phantom,
        })
    }
}

/// Cache store backed by disk
pub struct DiskCache<K, V> {
    pub(super) ttl: Option<Duration>,
    pub(super) refresh: bool,
    sync_to_disk_on_cache_change: bool,
    #[allow(unused)]
    version: u64,
    #[allow(unused)]
    disk_path: PathBuf,
    connection: Db,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> DiskCache<K, V>
where
    K: ToString,
    V: Serialize + DeserializeOwned,
{
    #[allow(clippy::new_ret_no_self)]
    /// Initialize a `DiskCacheBuilder`
    pub fn new(cache_name: &str) -> DiskCacheBuilder<K, V> {
        DiskCacheBuilder::new(cache_name)
    }

    pub fn remove_expired_entries(&self) -> Result<(), DiskCacheError> {
        let now = SystemTime::now();

        for (key, value) in self.connection.iter().flatten() {
            if let Ok(cached) = rmp_serde::from_slice::<CachedDiskValue<V>>(&value) {
                if let Some(ttl) = self.ttl {
                    if now
                        .duration_since(cached.created_at)
                        .unwrap_or(Duration::from_secs(0))
                        >= ttl
                    {
                        self.connection.remove(key)?;
                    }
                }
            }
        }

        if self.sync_to_disk_on_cache_change {
            self.connection.flush()?;
        }
        Ok(())
    }

    /// Provide access to the underlying [sled::Db] connection
    /// This is useful for i.e. manually flushing the cache to disk.
    pub fn connection(&self) -> &Db {
        &self.connection
    }

    /// Provide mutable access to the underlying [sled::Db] connection
    pub fn connection_mut(&mut self) -> &mut Db {
        &mut self.connection
    }
}

#[derive(Error, Debug)]
pub enum DiskCacheError {
    #[error("Storage error")]
    StorageError(#[from] sled::Error),
    #[error("Error deserializing cached value")]
    CacheDeserializationError(#[from] rmp_serde::decode::Error),
    #[error("Error serializing cached value")]
    CacheSerializationError(#[from] rmp_serde::encode::Error),
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CachedDiskValue<V> {
    pub(crate) value: V,
    pub(crate) created_at: SystemTime,
    pub(crate) version: u64,
}

impl<V> CachedDiskValue<V> {
    fn new(value: V) -> Self {
        Self {
            value,
            created_at: SystemTime::now(),
            version: 1,
        }
    }

    fn refresh_created_at(&mut self) {
        self.created_at = SystemTime::now();
    }
}

impl<K, V> IOCached<K, V> for DiskCache<K, V>
where
    K: ToString,
    V: Serialize + DeserializeOwned,
{
    type Error = DiskCacheError;

    fn cache_get(&self, key: &K) -> Result<Option<V>, DiskCacheError> {
        let key = key.to_string();
        let ttl = self.ttl;
        let refresh = self.refresh;
        let mut cache_updated = false;
        let update = |old: Option<&[u8]>| -> Option<Vec<u8>> {
            let old = old?;
            if ttl.is_none() {
                return Some(old.to_vec());
            }
            let ttl = ttl.unwrap();
            let mut cached = match rmp_serde::from_slice::<CachedDiskValue<V>>(old) {
                Ok(cached) => cached,
                Err(_) => {
                    // unable to deserialize, treat it as not existing
                    return None;
                }
            };
            if SystemTime::now()
                .duration_since(cached.created_at)
                .unwrap_or(Duration::from_secs(0))
                < ttl
            {
                if refresh {
                    cached.refresh_created_at();
                    cache_updated = true;
                }
                let cache_val =
                    rmp_serde::to_vec(&cached).expect("error serializing cached disk value");
                Some(cache_val)
            } else {
                None
            }
        };

        let result = if let Some(data) = self.connection.update_and_fetch(key, update)? {
            let cached = rmp_serde::from_slice::<CachedDiskValue<V>>(&data)?;
            Ok(Some(cached.value))
        } else {
            Ok(None)
        };

        if cache_updated && self.sync_to_disk_on_cache_change {
            self.connection.flush()?;
        }

        result
    }

    fn cache_set(&self, key: K, value: V) -> Result<Option<V>, DiskCacheError> {
        let key = key.to_string();
        let value = rmp_serde::to_vec(&CachedDiskValue::new(value))?;

        let result = if let Some(data) = self.connection.insert(key, value)? {
            let cached = rmp_serde::from_slice::<CachedDiskValue<V>>(&data)?;

            if let Some(ttl) = self.ttl {
                if SystemTime::now()
                    .duration_since(cached.created_at)
                    .unwrap_or(Duration::from_secs(0))
                    < ttl
                {
                    Ok(Some(cached.value))
                } else {
                    Ok(None)
                }
            } else {
                Ok(Some(cached.value))
            }
        } else {
            Ok(None)
        };

        if self.sync_to_disk_on_cache_change {
            self.connection.flush()?;
        }

        result
    }

    fn cache_remove(&self, key: &K) -> Result<Option<V>, DiskCacheError> {
        let key = key.to_string();
        let result = if let Some(data) = self.connection.remove(key)? {
            let cached = rmp_serde::from_slice::<CachedDiskValue<V>>(&data)?;

            if let Some(ttl) = self.ttl {
                if SystemTime::now()
                    .duration_since(cached.created_at)
                    .unwrap_or(Duration::from_secs(0))
                    < ttl
                {
                    Ok(Some(cached.value))
                } else {
                    Ok(None)
                }
            } else {
                Ok(Some(cached.value))
            }
        } else {
            Ok(None)
        };

        if self.sync_to_disk_on_cache_change {
            self.connection.flush()?;
        }

        result
    }

    fn cache_lifespan(&self) -> Option<Duration> {
        self.ttl
    }

    fn cache_set_lifespan(&mut self, ttl: Duration) -> Option<Duration> {
        let old = self.ttl;
        self.ttl = Some(ttl);
        old
    }

    fn cache_set_refresh(&mut self, refresh: bool) -> bool {
        let old = self.refresh;
        self.refresh = refresh;
        old
    }

    fn cache_unset_lifespan(&mut self) -> Option<Duration> {
        self.ttl.take()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test_DiskCache {
    use googletest::{
        assert_that,
        matchers::{anything, eq, none, ok, some},
        GoogleTestSupport as _,
    };
    use std::thread::sleep;
    use std::time::Duration;
    use tempfile::TempDir;

    use super::*;

    /// If passing `no_exist` to the macro:
    /// This gives you a TempDir where the directory does not exist
    /// so you can copy / move things to the returned TmpDir.path()
    /// and those files will be removed when the TempDir is dropped
    macro_rules! temp_dir {
        () => {
            TempDir::new().expect("Error creating temp dir")
        };
        (no_exist) => {{
            let tmp_dir = TempDir::new().expect("Error creating temp dir");
            std::fs::remove_dir_all(tmp_dir.path()).expect("error emptying the tmp dir");
            tmp_dir
        }};
    }

    fn now_millis() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    const TEST_KEY: u32 = 1;
    const TEST_VAL: u32 = 100;
    const TEST_KEY_1: u32 = 2;
    const TEST_VAL_1: u32 = 200;
    const LIFE_SPAN_2_SECS: Duration = Duration::from_secs(2);
    const LIFE_SPAN_1_SEC: Duration = Duration::from_secs(1);
    #[googletest::test]
    fn cache_get_after_cache_remove_returns_none() {
        let tmp_dir = temp_dir!();
        let cache: DiskCache<u32, u32> = DiskCache::new("test-cache")
            .set_disk_directory(tmp_dir.path())
            .build()
            .unwrap();

        let cached = cache.cache_get(&TEST_KEY).unwrap();
        assert_that!(
            cached,
            none(),
            "Getting a non-existent key-value should return None"
        );

        let cached = cache.cache_set(TEST_KEY, TEST_VAL).unwrap();
        assert_that!(cached, none(), "Setting a new key-value should return None");

        let cached = cache.cache_set(TEST_KEY, TEST_VAL_1).unwrap();
        assert_that!(
            cached,
            some(eq(TEST_VAL)),
            "Setting an existing key-value should return the old value"
        );

        let cached = cache.cache_get(&TEST_KEY).unwrap();
        assert_that!(
            cached,
            some(eq(TEST_VAL_1)),
            "Getting an existing key-value should return the value"
        );

        let cached = cache.cache_remove(&TEST_KEY).unwrap();
        assert_that!(
            cached,
            some(eq(TEST_VAL_1)),
            "Removing an existing key-value should return the value"
        );

        let cached = cache.cache_get(&TEST_KEY).unwrap();
        assert_that!(cached, none(), "Getting a removed key should return None");

        drop(cache);
    }

    #[googletest::test]
    fn values_expire_when_lifespan_elapses_returning_none() {
        let tmp_dir = temp_dir!();
        let cache: DiskCache<u32, u32> = DiskCache::new("test-cache")
            .set_disk_directory(tmp_dir.path())
            .set_lifespan(LIFE_SPAN_2_SECS)
            .build()
            .unwrap();

        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(none()),
            "Getting a non-existent key-value should return None"
        );

        assert_that!(
            cache.cache_set(TEST_KEY, 100),
            ok(none()),
            "Setting a new key-value should return None"
        );
        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(some(anything())),
            "Getting an existing key-value before it expires should return the value"
        );

        // Let the lifespan expire
        sleep(LIFE_SPAN_2_SECS);
        sleep(Duration::from_micros(500)); // a bit extra for good measure
        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(none()),
            "Getting an expired key-value should return None"
        );
    }

    #[googletest::test]
    fn set_lifespan_to_a_different_lifespan_is_respected() {
        // COPY PASTE of [values_expire_when_lifespan_elapses_returning_none]
        let tmp_dir = temp_dir!();
        let mut cache: DiskCache<u32, u32> = DiskCache::new("test-cache")
            .set_disk_directory(tmp_dir.path())
            .set_lifespan(LIFE_SPAN_2_SECS)
            .build()
            .unwrap();

        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(none()),
            "Getting a non-existent key-value should return None"
        );

        assert_that!(
            cache.cache_set(TEST_KEY, TEST_VAL),
            ok(none()),
            "Setting a new key-value should return None"
        );

        // Let the lifespan expire
        sleep(LIFE_SPAN_2_SECS);
        sleep(Duration::from_micros(500)); // a bit extra for good measure
        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(none()),
            "Getting an expired key-value should return None"
        );

        let old_from_setting_lifespan = cache
            .cache_set_lifespan(LIFE_SPAN_1_SEC)
            .expect("error setting new lifespan");
        assert_that!(
            old_from_setting_lifespan,
            eq(LIFE_SPAN_2_SECS),
            "Setting lifespan should return the old lifespan"
        );
        assert_that!(
            cache.cache_set(TEST_KEY, TEST_VAL),
            ok(none()),
            "Setting a previously expired key-value should return None"
        );
        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(some(eq(TEST_VAL))),
            "Getting a newly set (previously expired) key-value should return the value"
        );

        // Let the new lifespan expire
        sleep(LIFE_SPAN_1_SEC);
        sleep(Duration::from_micros(500)); // a bit extra for good measure
        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(none()),
            "Getting an expired key-value should return None"
        );

        cache
            .cache_set_lifespan(Duration::from_secs(10))
            .expect("error setting lifespan");
        assert_that!(
            cache.cache_set(TEST_KEY, TEST_VAL),
            ok(none()),
            "Setting a previously expired key-value should return None"
        );

        // TODO: Why are we now setting an irrelevant key?
        assert_that!(
            cache.cache_set(TEST_KEY_1, TEST_VAL),
            ok(none()),
            "Setting a new, separate, key-value should return None"
        );

        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(some(eq(TEST_VAL))),
            "Getting a newly set (previously expired) key-value should return the value"
        );
        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(some(eq(TEST_VAL))),
            "Getting the same value again should return the value"
        );
    }

    #[googletest::test]
    fn refreshing_on_cache_get_delays_cache_expiry() {
        // NOTE: Here we're relying on the fact that setting then sleeping for 2 secs and getting takes longer than 2 secs.
        const LIFE_SPAN: Duration = LIFE_SPAN_2_SECS;
        const HALF_LIFE_SPAN: Duration = LIFE_SPAN_1_SEC;
        let tmp_dir = temp_dir!();
        let cache: DiskCache<u32, u32> = DiskCache::new("test-cache")
            .set_disk_directory(tmp_dir.path())
            .set_lifespan(LIFE_SPAN)
            .set_refresh(true) // ENABLE REFRESH - this is what we're testing
            .build()
            .unwrap();

        assert_that!(cache.cache_set(TEST_KEY, TEST_VAL), ok(none()));

        // retrieve before expiry, this should refresh the created_at so we don't expire just yet
        sleep(HALF_LIFE_SPAN);
        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(some(eq(TEST_VAL))),
            "Getting a value before expiry should return the value"
        );

        // This is after the initial expiry, but since we refreshed the created_at, we should still get the value
        sleep(HALF_LIFE_SPAN);
        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(some(eq(TEST_VAL))),
            "Getting a value after the initial expiry should return the value as we have refreshed"
        );

        // This is after the new refresh expiry, we should get None
        sleep(LIFE_SPAN);
        assert_that!(
            cache.cache_get(&TEST_KEY),
            ok(none()),
            "Getting a value after the refreshed expiry should return None"
        );

        drop(cache);
    }

    #[googletest::test]
    // TODO: Consider removing this test, as it's not really testing anything.
    // If we want to check that setting a different disk directory to the default doesn't change anything,
    // we should design the tests to run all the same tests but paramaterized with different conditions.
    fn does_not_break_when_constructed_using_default_disk_directory() {
        let cache: DiskCache<u32, u32> =
            DiskCache::new(&format!("{}:disk-cache-test-default-dir", now_millis()))
                // use the default disk directory
                .build()
                .unwrap();

        let cached = cache.cache_get(&TEST_KEY).unwrap();
        assert_that!(
            cached,
            none(),
            "Getting a non-existent key-value should return None"
        );

        let cached = cache.cache_set(TEST_KEY, TEST_VAL).unwrap();
        assert_that!(cached, none(), "Setting a new key-value should return None");

        let cached = cache.cache_set(TEST_KEY, TEST_VAL_1).unwrap();
        assert_that!(
            cached,
            some(eq(TEST_VAL)),
            "Setting an existing key-value should return the old value"
        );

        // remove the cache dir to clean up the test as we're not using a temp dir
        std::fs::remove_dir_all(cache.disk_path).expect("error in clean up removeing the cache dir")
    }

    mod set_sync_to_disk_on_cache_change {

        mod when_no_auto_flushing {
            use super::super::*;

            fn check_on_recovered_cache(
                set_sync_to_disk_on_cache_change: bool,
                run_on_original_cache: fn(&DiskCache<u32, u32>) -> (),
                run_on_recovered_cache: fn(&DiskCache<u32, u32>) -> (),
            ) {
                let original_cache_tmp_dir = temp_dir!();
                let copied_cache_tmp_dir = temp_dir!(no_exist);
                const CACHE_NAME: &str = "test-cache";

                let cache: DiskCache<u32, u32> = DiskCache::new(CACHE_NAME)
                    .set_disk_directory(original_cache_tmp_dir.path())
                    .set_sync_to_disk_on_cache_change(set_sync_to_disk_on_cache_change) // WHAT'S BEING TESTED
                    // NOTE: disabling automatic flushing, so that we only test the flushing of cache_set
                    .set_connection_config(sled::Config::new().flush_every_ms(None))
                    .build()
                    .unwrap();

                // flush the cache to disk before any cache setting, so that when we create the recovered cache
                // it has something to recover from, even if set_cache doesn't write to disk as we'd like.
                cache
                    .connection
                    .flush()
                    .expect("error flushing cache before any cache setting");

                run_on_original_cache(&cache);

                // freeze the current state of the cache files by copying them to a new location
                // we do this before dropping the cache, as dropping the cache seems to flush to the disk
                let recovered_cache = clone_cache_to_new_location_no_flushing(
                    CACHE_NAME,
                    &cache,
                    copied_cache_tmp_dir.path(),
                );

                assert_that!(recovered_cache.connection.was_recovered(), eq(true));

                run_on_recovered_cache(&recovered_cache);
            }

            mod changes_persist_after_recovery_when_set_to_true {
                use super::*;

                #[googletest::test]
                fn for_cache_set() {
                    check_on_recovered_cache(
                        false,
                        |cache| {
                            // write to the cache, we expect this to persist if the connection is flushed on cache_set
                            cache
                                .cache_set(TEST_KEY, TEST_VAL)
                                .expect("error setting cache in assemble stage");
                        },
                        |recovered_cache| {
                            assert_that!(
                                    recovered_cache.cache_get(&TEST_KEY),
                                    ok(none()),
                                    "set_sync_to_disk_on_cache_change is false, and there is no auto-flushing, so the cache should not have persisted"
                                );
                        },
                    )
                }

                #[googletest::test]
                fn for_cache_remove() {
                    check_on_recovered_cache(
                        false,
                        |cache| {
                            // write to the cache, we expect this to persist if the connection is flushed on cache_set
                            cache
                                .cache_set(TEST_KEY, TEST_VAL)
                                .expect("error setting cache in assemble stage");

                            // manually flush the cache so that we only test cache_remove
                            cache.connection.flush().expect("error flushing cache");

                            cache
                                .cache_remove(&TEST_KEY)
                                .expect("error removing cache in assemble stage");
                        },
                        |recovered_cache| {
                            assert_that!(
                                    recovered_cache.cache_get(&TEST_KEY),
                                    ok(some(eq(TEST_VAL))),
                                    "set_sync_to_disk_on_cache_change is false, and there is no auto-flushing, so the cache_remove should not have persisted"
                                );
                        },
                    )
                }

                #[ignore = "Not implemented"]
                #[googletest::test]
                fn for_cache_get_when_refreshing() {
                    todo!("Test not implemented.")
                }
            }

            /// This is the anti-test
            mod changes_do_not_persist_after_recovery_when_set_to_false {
                use super::*;

                #[googletest::test]
                fn for_cache_set() {
                    check_on_recovered_cache(
                        true,
                        |cache| {
                            // write to the cache, we expect this to persist if the connection is flushed on cache_set
                            cache
                                .cache_set(TEST_KEY, TEST_VAL)
                                .expect("error setting cache in assemble stage");
                        },
                        |recovered_cache| {
                            assert_that!(
                                recovered_cache.cache_get(&TEST_KEY),
                                ok(some(eq(TEST_VAL))),
                                "Getting a set key should return the value"
                            );
                        },
                    )
                }

                #[googletest::test]
                fn for_cache_remove() {
                    check_on_recovered_cache(
                        true,
                        |cache| {
                            // write to the cache, we expect this to persist if the connection is flushed on cache_set
                            cache
                                .cache_set(TEST_KEY, TEST_VAL)
                                .expect("error setting cache in assemble stage");

                            cache
                                .cache_remove(&TEST_KEY)
                                .expect("error removing cache in assemble stage");
                        },
                        |recovered_cache| {
                            assert_that!(
                                recovered_cache.cache_get(&TEST_KEY),
                                ok(none()),
                                "Getting a removed key should return None"
                            );
                        },
                    )
                }

                #[ignore = "Not implemented"]
                #[googletest::test]
                fn for_cache_get_when_refreshing() {
                    todo!("Test not implemented.")
                }
            }

            fn clone_cache_to_new_location_no_flushing(
                cache_name: &str,
                cache: &DiskCache<u32, u32>,
                new_location: &Path,
            ) -> DiskCache<u32, u32> {
                copy_dir::copy_dir(cache.disk_path.parent().unwrap(), new_location)
                    .expect("error copying cache files to new location");

                DiskCache::new(cache_name)
                    .set_disk_directory(new_location)
                    .build()
                    .expect("error building cache from copied files")
            }
        }
    }
}
