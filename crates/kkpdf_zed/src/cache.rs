//! Memory-budgeted LRU cache for rasterized PDF page bitmaps.
//!
//! Stores rendered RGBA framebuffers indexed by `(page_index, zoom_bucket, dark_mode)`.
//! Automatically evicts oldest unused pages when memory consumption exceeds the configured budget.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

/// Default maximum memory budget: 256 Megabytes.
pub const DEFAULT_MEMORY_BUDGET_BYTES: usize = 256 * 1024 * 1024;

/// Cache lookup key capturing page index, discrete zoom bucket, and dark-mode setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub page_index: usize,
    pub zoom_bucket: u32,
    pub dark_mode: bool,
}

impl CacheKey {
    /// Creates a new cache key with zoom discretized to 2 decimal places (e.g. 1.25 -> 125).
    pub fn new(page_index: usize, zoom_factor: f32, dark_mode: bool) -> Self {
        let zoom_bucket = (zoom_factor * 100.0).round().max(1.0) as u32;
        Self {
            page_index,
            zoom_bucket,
            dark_mode,
        }
    }
}

/// A fully rasterized PDF page holding raw RGBA pixel data.
#[derive(Debug, Clone)]
pub struct RenderedPage {
    pub page_index: usize,
    pub width: u32,
    pub height: u32,
    pub zoom_factor: f32,
    pub dark_mode: bool,
    pub rgba_buffer: Arc<Vec<u8>>,
}

impl RenderedPage {
    /// Constructs a new RenderedPage.
    pub fn new(
        page_index: usize,
        width: u32,
        height: u32,
        zoom_factor: f32,
        dark_mode: bool,
        rgba_buffer: Vec<u8>,
    ) -> Self {
        Self {
            page_index,
            width,
            height,
            zoom_factor,
            dark_mode,
            rgba_buffer: Arc::new(rgba_buffer),
        }
    }

    /// Returns the exact memory footprint of this bitmap in bytes (width * height * 4).
    #[inline]
    pub fn byte_size(&self) -> usize {
        (self.width as usize) * (self.height as usize) * 4
    }
}

/// A Least-Recently-Used (LRU) cache bounded by byte size and item limits.
#[derive(Debug)]
pub struct PageLruCache {
    max_memory_bytes: usize,
    current_memory_bytes: usize,
    max_pages: Option<usize>,
    entries: HashMap<CacheKey, RenderedPage>,
    lru_order: VecDeque<CacheKey>,
}

impl PageLruCache {
    /// Creates a new cache with the specified memory budget in bytes.
    pub fn new(max_memory_bytes: usize) -> Self {
        Self {
            max_memory_bytes: max_memory_bytes.max(1024 * 1024), // Minimum 1MB
            current_memory_bytes: 0,
            max_pages: None,
            entries: HashMap::new(),
            lru_order: VecDeque::new(),
        }
    }

    /// Configures an optional hard ceiling on the maximum number of cached pages.
    pub fn with_max_pages(mut self, max_pages: usize) -> Self {
        self.max_pages = Some(max_pages);
        self
    }

    /// Retrieves a cached page, refreshing its LRU position.
    pub fn get(&mut self, key: &CacheKey) -> Option<RenderedPage> {
        if self.entries.contains_key(key) {
            // Move key to back (most recently used)
            if let Some(pos) = self.lru_order.iter().position(|k| k == key) {
                self.lru_order.remove(pos);
                self.lru_order.push_back(*key);
            }
            self.entries.get(key).cloned()
        } else {
            None
        }
    }

    /// Inserts a newly rasterized page into the cache, evicting older pages if needed.
    pub fn insert(&mut self, key: CacheKey, page: RenderedPage) {
        let page_size = page.byte_size();

        // If replacing existing entry, remove old size first
        if let Some(old_page) = self.entries.remove(&key) {
            self.current_memory_bytes = self
                .current_memory_bytes
                .saturating_sub(old_page.byte_size());
            if let Some(pos) = self.lru_order.iter().position(|k| k == &key) {
                self.lru_order.remove(pos);
            }
        }

        // Evict until new page fits within budget
        self.evict_for_space(page_size);

        // If page is larger than max budget itself, we do not cache it
        if page_size > self.max_memory_bytes {
            log::warn!(
                "Rendered page size ({page_size} bytes) exceeds total cache budget ({max} bytes); skipping cache insertion",
                max = self.max_memory_bytes
            );
            return;
        }

        self.current_memory_bytes += page_size;
        self.entries.insert(key, page);
        self.lru_order.push_back(key);
    }

    /// Evicts oldest entries until at least `needed_bytes` is available.
    fn evict_for_space(&mut self, needed_bytes: usize) {
        // Enforce page count limit if configured
        if let Some(max_pages) = self.max_pages {
            while self.entries.len() >= max_pages {
                if !self.evict_oldest() {
                    break;
                }
            }
        }

        // Enforce memory budget
        while self.current_memory_bytes + needed_bytes > self.max_memory_bytes {
            if !self.evict_oldest() {
                break;
            }
        }
    }

    /// Evicts the single least recently used item. Returns false if cache was empty.
    fn evict_oldest(&mut self) -> bool {
        if let Some(oldest_key) = self.lru_order.pop_front() {
            if let Some(evicted_page) = self.entries.remove(&oldest_key) {
                self.current_memory_bytes = self
                    .current_memory_bytes
                    .saturating_sub(evicted_page.byte_size());
                log::debug!(
                    "Evicted page {page_idx} (zoom: {zoom}) to free {bytes} bytes (current memory: {cur} / {max})",
                    page_idx = oldest_key.page_index,
                    zoom = oldest_key.zoom_bucket,
                    bytes = evicted_page.byte_size(),
                    cur = self.current_memory_bytes,
                    max = self.max_memory_bytes
                );
                return true;
            }
        }
        false
    }

    /// Clears all entries from the cache and resets memory tracking.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.lru_order.clear();
        self.current_memory_bytes = 0;
    }

    /// Number of pages currently cached.
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the cache contains no pages.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Current memory usage in bytes.
    #[inline]
    pub fn memory_usage(&self) -> usize {
        self.current_memory_bytes
    }

    /// Maximum memory budget in bytes.
    #[inline]
    pub fn max_memory(&self) -> usize {
        self.max_memory_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_discretization() {
        let key1 = CacheKey::new(0, 1.004, false);
        let key2 = CacheKey::new(0, 1.001, false);
        let key3 = CacheKey::new(0, 1.250, false);
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
        assert_eq!(key1.zoom_bucket, 100);
        assert_eq!(key3.zoom_bucket, 125);
    }

    #[test]
    fn test_lru_eviction_on_memory_budget() {
        // 1 MB cache budget
        let mut cache = PageLruCache::new(1024 * 1024);

        // Create 3 mock pages of 400 KB each (100x1000 RGBA = 400,000 bytes)
        let page_size_bytes = 400_000;
        let buf = vec![0u8; page_size_bytes];

        let page0 = RenderedPage::new(0, 100, 1000, 1.0, false, buf.clone());
        let page1 = RenderedPage::new(1, 100, 1000, 1.0, false, buf.clone());
        let page2 = RenderedPage::new(2, 100, 1000, 1.0, false, buf);

        let key0 = CacheKey::new(0, 1.0, false);
        let key1 = CacheKey::new(1, 1.0, false);
        let key2 = CacheKey::new(2, 1.0, false);

        // Insert page 0 (400 KB) -> total: 400 KB
        cache.insert(key0, page0);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.memory_usage(), 400_000);

        // Insert page 1 (400 KB) -> total: 800 KB
        cache.insert(key1, page1);
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.memory_usage(), 800_000);

        // Insert page 2 (400 KB) -> 800 + 400 = 1200 KB > 1024 KB
        // Page 0 should be evicted!
        cache.insert(key2, page2);
        assert_eq!(cache.len(), 2);
        assert!(
            cache.get(&key0).is_none(),
            "Page 0 should have been evicted"
        );
        assert!(cache.get(&key1).is_some(), "Page 1 should still exist");
        assert!(cache.get(&key2).is_some(), "Page 2 should still exist");
    }

    #[test]
    fn test_lru_access_refresh() {
        let mut cache = PageLruCache::new(1024 * 1024);
        let buf = vec![0u8; 400_000];

        let key0 = CacheKey::new(0, 1.0, false);
        let key1 = CacheKey::new(1, 1.0, false);
        let key2 = CacheKey::new(2, 1.0, false);

        cache.insert(
            key0,
            RenderedPage::new(0, 100, 1000, 1.0, false, buf.clone()),
        );
        cache.insert(
            key1,
            RenderedPage::new(1, 100, 1000, 1.0, false, buf.clone()),
        );

        // Access page 0, making page 1 the oldest
        let _ = cache.get(&key0);

        // Insert page 2 -> page 1 should be evicted instead of page 0!
        cache.insert(key2, RenderedPage::new(2, 100, 1000, 1.0, false, buf));

        assert!(
            cache.get(&key0).is_some(),
            "Page 0 was accessed and should survive"
        );
        assert!(
            cache.get(&key1).is_none(),
            "Page 1 was oldest and should be evicted"
        );
        assert!(cache.get(&key2).is_some(), "Page 2 is newly inserted");
    }
}
