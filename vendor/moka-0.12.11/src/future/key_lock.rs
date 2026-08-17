use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

use crate::{cht::SegmentedHashMap, common::concurrent::arc::MiniArc};

use async_lock::{Mutex, MutexGuard};

const LOCK_MAP_NUM_SEGMENTS: usize = 64;

type LockMap<K, S> = SegmentedHashMap<Arc<K>, MiniArc<Mutex<()>>, S>;

// We need the `where` clause here because of the Drop impl.
pub(crate) struct KeyLock<'a, K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    map: &'a LockMap<K, S>,
    key: Arc<K>,
    hash: u64,
    lock: MiniArc<Mutex<()>>,
}

impl<K, S> Drop for KeyLock<'_, K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn drop(&mut self) {
        if MiniArc::count(&self.lock) <= 2 {
            self.map.remove_if(
                self.hash,
                |k| k == &self.key,
                |_k, v| MiniArc::count(v) <= 2,
            );
        }
    }
}

impl<'a, K, S> KeyLock<'a, K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn new(map: &'a LockMap<K, S>, key: &Arc<K>, hash: u64, lock: MiniArc<Mutex<()>>) -> Self {
        Self {
            map,
            key: Arc::clone(key),
            hash,
            lock,
        }
    }

    pub(crate) async fn lock(&self) -> MutexGuard<'_, ()> {
        self.lock.lock().await
    }
}

pub(crate) struct KeyLockMap<K, S> {
    locks: LockMap<K, S>,
}

impl<K, S> KeyLockMap<K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub(crate) fn with_hasher(hasher: S) -> Self {
        Self {
            locks: SegmentedHashMap::with_num_segments_and_hasher(LOCK_MAP_NUM_SEGMENTS, hasher),
        }
    }

    pub(crate) fn key_lock(&self, key: &Arc<K>) -> KeyLock<'_, K, S> {
        let hash = self.locks.hash(key);
        let kl = MiniArc::new(Mutex::new(()));
        match self
            .locks
            .insert_if_not_present(Arc::clone(key), hash, kl.clone())
        {
            None => KeyLock::new(&self.locks, key, hash, kl),
            Some(existing_kl) => KeyLock::new(&self.locks, key, hash, existing_kl),
        }
    }
}

#[cfg(test)]
impl<K, S> KeyLockMap<K, S> {
    pub(crate) fn is_empty(&self) -> bool {
        self.locks.len() == 0
    }
}
