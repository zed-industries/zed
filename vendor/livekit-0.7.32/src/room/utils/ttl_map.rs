// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    time::{Duration, SystemTime},
};

/// Time to live (TTL) map
///
/// Elements older than the TTL duration are automatically removed.
///
#[derive(Debug)]
pub struct TtlMap<K, V> {
    inner: HashMap<K, Entry<V>>,
    last_cleanup: SystemTime,
    ttl: Duration,
}

#[derive(Debug)]
struct Entry<V> {
    value: V,
    expires_at: SystemTime,
}

impl<K, V> TtlMap<K, V> {
    /// Creates an empty `TtlMap`.
    pub fn new(ttl: Duration) -> Self {
        Self { inner: HashMap::new(), last_cleanup: SystemTime::now(), ttl }
    }

    /// Returns the number of elements in the map.
    pub fn len(&mut self) -> usize {
        self.cleanup();
        self.inner.len()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    pub fn iter(&mut self) -> impl Iterator<Item = (&K, &V)> {
        self.cleanup();
        self.inner.iter().map(|(key, entry)| (key, &entry.value))
    }

    /// Removes expired elements.
    fn cleanup(&mut self) {
        let now = SystemTime::now();
        self.inner.retain(|_, entry| entry.expires_at >= now);
        self.last_cleanup = now;
    }
}

impl<K, V> TtlMap<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Returns a reference to the value corresponding to the key.
    pub fn get(&mut self, k: &K) -> Option<&V> {
        let expires_at = self.inner.get(k).map(|entry| entry.expires_at)?;
        let now = SystemTime::now();
        if expires_at < now {
            _ = self.inner.remove(k);
            return None;
        }
        Some(&self.inner.get(k).unwrap().value)
    }

    /// Sets the value for the given key.
    pub fn set(&mut self, k: &K, v: Option<V>) {
        let now = SystemTime::now();
        let Ok(elapsed) = now.duration_since(self.last_cleanup) else {
            log::error!("System clock anomaly detected");
            return;
        };
        let half_ttl = self.ttl.div_f64(2.0);
        if elapsed > half_ttl {
            self.cleanup();
        }

        let Some(value) = v else {
            _ = self.inner.remove(&k);
            return;
        };
        let expires_at = now + self.ttl;
        let entry = Entry { value, expires_at };
        self.inner.insert(k.clone(), entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use tokio::time::sleep;

    const SHORT_TTL: Duration = Duration::from_millis(100);

    #[tokio::test]
    async fn test_expiration() {
        let mut map = TtlMap::<char, u8>::new(SHORT_TTL);
        map.set(&'a', Some(1));
        map.set(&'b', Some(2));
        map.set(&'c', Some(3));

        assert_eq!(map.len(), 3);
        assert!(map.get(&'a').is_some());
        assert!(map.get(&'b').is_some());
        assert!(map.get(&'c').is_some());

        sleep(SHORT_TTL).await;

        assert_eq!(map.len(), 0);
        assert!(map.get(&'a').is_none());
        assert!(map.get(&'b').is_none());
        assert!(map.get(&'c').is_none());
    }

    #[test]
    fn test_iter() {
        let mut map = TtlMap::<char, u8>::new(SHORT_TTL);
        map.set(&'a', Some(1));
        map.set(&'b', Some(2));
        map.set(&'c', Some(3));

        let elements: HashSet<_> = map.iter().map(|(k, v)| (*k, *v)).collect();
        assert!(elements.contains(&('a', 1)));
        assert!(elements.contains(&('b', 2)));
        assert!(elements.contains(&('c', 3)));
    }
}
