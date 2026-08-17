use std::sync::Arc;

use fluent_uri::Uri;
use hashbrown::hash_map::{EntryRef, HashMap};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};

use crate::{uri, Error};

type CacheBucket = HashMap<String, Arc<Uri<String>>>;
type CacheMap = HashMap<String, CacheBucket>;

#[derive(Debug, Clone)]
pub(crate) struct UriCache {
    cache: CacheMap,
}

impl UriCache {
    pub(crate) fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(capacity),
        }
    }

    pub(crate) fn resolve_against(
        &mut self,
        base: &Uri<&str>,
        uri: impl AsRef<str>,
    ) -> Result<Arc<Uri<String>>, Error> {
        let base_str = base.as_str();
        let reference = uri.as_ref();

        let resolved = match self.cache.entry_ref(base_str) {
            EntryRef::Occupied(mut entry) => {
                if let Some(cached) = entry.get().get(reference) {
                    return Ok(Arc::clone(cached));
                }

                let resolved = Arc::new(uri::resolve_against(base, reference)?);
                entry
                    .get_mut()
                    .insert(reference.to_owned(), Arc::clone(&resolved));
                resolved
            }
            EntryRef::Vacant(entry) => {
                let resolved = Arc::new(uri::resolve_against(base, reference)?);
                let mut inner = HashMap::with_capacity(1);
                inner.insert(reference.to_owned(), Arc::clone(&resolved));
                entry.insert(inner);
                resolved
            }
        };

        Ok(resolved)
    }

    pub(crate) fn into_shared(self) -> SharedUriCache {
        SharedUriCache {
            cache: RwLock::new(self.cache),
        }
    }
}

/// A dedicated type for URI resolution caching.
#[derive(Debug)]
pub(crate) struct SharedUriCache {
    cache: RwLock<CacheMap>,
}

impl Clone for SharedUriCache {
    fn clone(&self) -> Self {
        Self {
            cache: RwLock::new(
                self.cache
                    .read()
                    .iter()
                    .map(|(base, entries)| {
                        (
                            base.clone(),
                            entries
                                .iter()
                                .map(|(reference, value)| (reference.clone(), Arc::clone(value)))
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        }
    }
}

impl SharedUriCache {
    pub(crate) fn resolve_against(
        &self,
        base: &Uri<&str>,
        uri: impl AsRef<str>,
    ) -> Result<Arc<Uri<String>>, Error> {
        let base_str = base.as_str();
        let reference = uri.as_ref();

        if let Some(cached) = self
            .cache
            .read()
            .get(base_str)
            .and_then(|inner| inner.get(reference))
        {
            return Ok(Arc::clone(cached));
        }

        let cache = self.cache.upgradable_read();
        if let Some(inner) = cache.get(base_str).and_then(|inner| inner.get(reference)) {
            return Ok(Arc::clone(inner));
        }

        let resolved = Arc::new(uri::resolve_against(base, reference)?);

        let mut cache = RwLockUpgradableReadGuard::upgrade(cache);
        let inserted = match cache.entry_ref(base_str) {
            EntryRef::Occupied(mut entry) => {
                if let Some(existing) = entry.get().get(reference) {
                    return Ok(Arc::clone(existing));
                }
                entry
                    .get_mut()
                    .insert(reference.to_owned(), Arc::clone(&resolved));
                resolved
            }
            EntryRef::Vacant(entry) => {
                let mut inner = HashMap::with_capacity(1);
                inner.insert(reference.to_owned(), Arc::clone(&resolved));
                entry.insert(inner);
                resolved
            }
        };

        Ok(inserted)
    }

    pub(crate) fn into_local(self) -> UriCache {
        UriCache {
            cache: self.cache.into_inner(),
        }
    }
}
