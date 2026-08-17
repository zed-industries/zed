use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::{CacheKey, CacheStorage, CacheWriter, CachedResponse};
use http::{HeaderMap, Uri};

pub struct InMemoryCache {
    inner: Arc<Mutex<CacheData>>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CacheData {
                keys: HashMap::new(),
                responses: HashMap::new(),
            })),
        }
    }
}

impl Default for InMemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

struct CacheData {
    keys: HashMap<Uri, CacheKey>,
    responses: HashMap<Uri, CachedResponse>,
}

struct InMemoryWriter {
    cache: Arc<Mutex<CacheData>>,
    uri: Uri,
    key: CacheKey,
    response: CachedResponse,
}

impl CacheStorage for InMemoryCache {
    fn try_hit(&self, uri: &Uri) -> Option<CacheKey> {
        self.inner.lock().unwrap().keys.get(uri).cloned()
    }

    fn load(&self, uri: &Uri) -> Option<CachedResponse> {
        self.inner.lock().unwrap().responses.get(uri).cloned()
    }

    fn writer(&self, uri: &Uri, key: CacheKey, headers: HeaderMap) -> Box<dyn CacheWriter> {
        Box::new(InMemoryWriter {
            cache: self.inner.clone(),
            uri: uri.clone(),
            key,
            response: CachedResponse {
                body: Vec::new(),
                headers,
            },
        })
    }
}

impl CacheWriter for InMemoryWriter {
    fn write_body(&mut self, data: &[u8]) {
        self.response.body.extend_from_slice(data);
    }
}

impl Drop for InMemoryWriter {
    fn drop(&mut self) {
        // The whole response was received, hence the writer is dropped. We need
        // to add the response body to the cache.
        let uri = self.uri.clone();
        let key = self.key.clone();
        let response = std::mem::take(&mut self.response);

        let mut cache = self.cache.lock().unwrap();
        cache.keys.insert(uri.clone(), key);
        cache.responses.insert(uri, response);
    }
}
