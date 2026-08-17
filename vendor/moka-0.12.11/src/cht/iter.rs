use std::hash::Hash;

pub(crate) trait ScanningGet<K, V>
where
    K: Clone,
    V: Clone,
{
    /// Returns a _clone_ of the value corresponding to the key.
    fn scanning_get(&self, key: &K) -> Option<V>;

    /// Returns a vec of keys in a specified segment of the concurrent hash table.
    fn keys(&self, cht_segment: usize) -> Option<Vec<K>>;
}

pub(crate) struct Iter<'i, K, V> {
    keys: Option<Vec<K>>,
    map: &'i dyn ScanningGet<K, V>,
    num_segments: usize,
    seg_index: usize,
    is_done: bool,
}

impl<'i, K, V> Iter<'i, K, V> {
    pub(crate) fn with_single_cache_segment(
        map: &'i dyn ScanningGet<K, V>,
        num_segments: usize,
    ) -> Self {
        Self {
            keys: None,
            map,
            num_segments,
            seg_index: 0,
            is_done: false,
        }
    }
}

impl<K, V> Iterator for Iter<'_, K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        while let Some(key) = self.next_key() {
            if let Some(v) = self.map.scanning_get(&key) {
                return Some((key, v));
            }
        }

        self.is_done = true;
        None
    }
}

impl<K, V> Iter<'_, K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn next_key(&mut self) -> Option<K> {
        while let Some(keys) = self.current_keys() {
            if let key @ Some(_) = keys.pop() {
                return key;
            }
        }
        None
    }

    fn current_keys(&mut self) -> Option<&mut Vec<K>> {
        // If keys is none or some but empty, try to get next keys.
        while self.keys.as_ref().map_or(true, Vec::is_empty) {
            // Adjust indices.
            if self.seg_index >= self.num_segments {
                return None;
            }

            self.keys = self.map.keys(self.seg_index);
            self.seg_index += 1;
        }

        self.keys.as_mut()
    }
}
