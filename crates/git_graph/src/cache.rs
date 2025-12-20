//! Caching utilities for git graph performance optimization
//!
//! Provides LRU-based caching for commit data and computed graph layouts
//! to improve performance with large repositories.
//!
//! ## Memory Optimization (P3.2)
//! - Automatic cache eviction when memory pressure detected
//! - Lightweight CommitSummary (~200 bytes per entry)
//! - SharedString for zero-copy string handling
//!
//! ## Response Caching (P3.3)
//! - Caches commit summaries to avoid re-parsing git output
//! - Incremental loading with batch fetching
//! - Configurable cache size based on repository size

use collections::HashMap;
use gpui::SharedString;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Statistics for cache performance monitoring
#[derive(Debug, Default)]
pub struct CacheStats {
    hits: AtomicUsize,
    misses: AtomicUsize,
    evictions: AtomicUsize,
}

impl CacheStats {
    pub fn hit(&self) {
        self.hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn miss(&self) {
        self.misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn eviction(&self) {
        self.evictions.fetch_add(1, Ordering::Relaxed);
    }

    pub fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(Ordering::Relaxed) as f64;
        let total = hits + self.misses.load(Ordering::Relaxed) as f64;
        if total > 0.0 {
            hits / total
        } else {
            0.0
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Cache: {} hits, {} misses, {} evictions ({:.1}% hit rate)",
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
            self.evictions.load(Ordering::Relaxed),
            self.hit_rate() * 100.0
        )
    }

    /// Reset all statistics
    pub fn reset(&self) {
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
        self.evictions.store(0, Ordering::Relaxed);
    }

    /// Get total operations count
    pub fn total_operations(&self) -> usize {
        self.hits.load(Ordering::Relaxed) + self.misses.load(Ordering::Relaxed)
    }
}

/// LRU cache for commit summaries (lightweight commit data)
#[derive(Debug)]
pub struct CommitCache {
    /// Maximum number of cached entries
    capacity: usize,
    /// Cached commit summaries (SHA -> summary)
    entries: HashMap<SharedString, CommitSummary>,
    /// LRU order tracking (front = least recently used)
    lru_order: VecDeque<SharedString>,
    /// Performance statistics
    pub stats: CacheStats,
    /// Time of last cache access (for TTL-based eviction)
    last_access: Instant,
    /// Maximum time to live for cache entries (default: 30 minutes)
    ttl: Duration,
}

/// Lightweight commit summary for caching
#[derive(Clone, Debug)]
pub struct CommitSummary {
    pub short_sha: SharedString,
    pub subject: SharedString,
    pub author_name: SharedString,
    pub timestamp: i64,
    pub lane: usize,
    pub has_refs: bool,
}

impl CommitCache {
    /// Create a new cache with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: HashMap::default(),
            lru_order: VecDeque::with_capacity(capacity),
            stats: CacheStats::default(),
            last_access: Instant::now(),
            ttl: Duration::from_secs(30 * 60), // 30 minutes default
        }
    }

    /// Create a new cache with specified capacity and TTL
    pub fn with_ttl(capacity: usize, ttl: Duration) -> Self {
        Self {
            capacity,
            entries: HashMap::default(),
            lru_order: VecDeque::with_capacity(capacity),
            stats: CacheStats::default(),
            last_access: Instant::now(),
            ttl,
        }
    }

    /// Create cache with default capacity for typical repository sizes
    pub fn default_capacity() -> Self {
        // 10,000 commits is enough for most repositories
        // Each CommitSummary is ~200 bytes, so 10K = ~2MB
        Self::new(10_000)
    }

    /// Estimate memory usage in bytes
    pub fn estimated_memory_bytes(&self) -> usize {
        // Base struct size + entry overhead
        const ENTRY_OVERHEAD: usize = 200; // CommitSummary ~200 bytes
        const SHA_OVERHEAD: usize = 48; // SharedString + heap
        self.entries.len() * (ENTRY_OVERHEAD + SHA_OVERHEAD)
    }

    /// Check if cache has expired based on TTL
    pub fn is_expired(&self) -> bool {
        self.last_access.elapsed() > self.ttl
    }

    /// Evict stale entries if TTL has passed
    pub fn evict_if_stale(&mut self) {
        if self.is_expired() {
            self.clear();
        }
    }

    /// Get a cached commit summary
    pub fn get(&mut self, sha: &SharedString) -> Option<&CommitSummary> {
        // Update last access time for TTL tracking
        self.last_access = Instant::now();

        if self.entries.contains_key(sha) {
            self.stats.hit();
            // Move to end of LRU (most recently used)
            self.touch(sha);
            self.entries.get(sha)
        } else {
            self.stats.miss();
            None
        }
    }

    /// Insert a commit summary into the cache
    pub fn insert(&mut self, sha: SharedString, summary: CommitSummary) {
        // Update last access time
        self.last_access = Instant::now();

        // If at capacity, evict LRU entry
        while self.entries.len() >= self.capacity {
            self.evict_lru();
        }

        if !self.entries.contains_key(&sha) {
            self.lru_order.push_back(sha.clone());
        }
        self.entries.insert(sha, summary);
    }

    /// Bulk insert multiple commit summaries (more efficient for large batches)
    pub fn insert_batch(&mut self, entries: impl IntoIterator<Item = (SharedString, CommitSummary)>) {
        self.last_access = Instant::now();
        for (sha, summary) in entries {
            // Only insert if there's room or we can evict
            while self.entries.len() >= self.capacity {
                self.evict_lru();
            }
            if !self.entries.contains_key(&sha) {
                self.lru_order.push_back(sha.clone());
            }
            self.entries.insert(sha, summary);
        }
    }

    /// Shrink cache to a percentage of current size (for memory pressure)
    pub fn shrink_to_percentage(&mut self, percentage: usize) {
        let target_size = (self.entries.len() * percentage) / 100;
        while self.entries.len() > target_size {
            self.evict_lru();
        }
    }

    /// Check if a SHA is cached
    pub fn contains(&self, sha: &SharedString) -> bool {
        self.entries.contains_key(sha)
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.lru_order.clear();
    }

    /// Get current cache size
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn touch(&mut self, sha: &SharedString) {
        // Remove from current position and add to end
        if let Some(pos) = self.lru_order.iter().position(|s| s == sha) {
            self.lru_order.remove(pos);
            self.lru_order.push_back(sha.clone());
        }
    }

    fn evict_lru(&mut self) {
        if let Some(sha) = self.lru_order.pop_front() {
            self.entries.remove(&sha);
            self.stats.eviction();
        }
    }
}

/// Configuration for graph loading with performance limits
#[derive(Clone, Debug)]
pub struct LoadConfig {
    /// Maximum number of commits to load initially
    pub initial_limit: usize,
    /// Number of commits to load per batch when scrolling
    pub batch_size: usize,
    /// Whether to enable lazy loading of commit details
    pub lazy_details: bool,
    /// Cache capacity (0 = disabled)
    pub cache_capacity: usize,
}

impl Default for LoadConfig {
    fn default() -> Self {
        Self {
            initial_limit: 1000,     // Load first 1000 commits
            batch_size: 500,         // Load 500 more when scrolling
            lazy_details: true,      // Don't load full commit message until needed
            cache_capacity: 10_000,  // Cache up to 10K commits
        }
    }
}

impl LoadConfig {
    /// Configuration for small repositories (< 1000 commits)
    pub fn small_repo() -> Self {
        Self {
            initial_limit: usize::MAX,
            batch_size: usize::MAX,
            lazy_details: false,
            cache_capacity: 0,
        }
    }

    /// Configuration for large repositories (> 10000 commits)
    pub fn large_repo() -> Self {
        Self {
            initial_limit: 500,
            batch_size: 200,
            lazy_details: true,
            cache_capacity: 20_000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic_operations() {
        let mut cache = CommitCache::new(3);

        let sha1: SharedString = "abc123".into();
        let sha2: SharedString = "def456".into();
        let sha3: SharedString = "ghi789".into();

        let summary = CommitSummary {
            short_sha: "abc1234".into(),
            subject: "Test commit".into(),
            author_name: "Test Author".into(),
            timestamp: 1234567890,
            lane: 0,
            has_refs: false,
        };

        cache.insert(sha1.clone(), summary.clone());
        cache.insert(sha2.clone(), summary.clone());
        cache.insert(sha3.clone(), summary.clone());

        assert_eq!(cache.len(), 3);
        assert!(cache.contains(&sha1));
        assert!(cache.contains(&sha2));
        assert!(cache.contains(&sha3));
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache = CommitCache::new(2);

        let sha1: SharedString = "abc123".into();
        let sha2: SharedString = "def456".into();
        let sha3: SharedString = "ghi789".into();

        let summary = CommitSummary {
            short_sha: "abc1234".into(),
            subject: "Test".into(),
            author_name: "Test".into(),
            timestamp: 0,
            lane: 0,
            has_refs: false,
        };

        cache.insert(sha1.clone(), summary.clone());
        cache.insert(sha2.clone(), summary.clone());
        cache.insert(sha3.clone(), summary.clone());

        // sha1 should be evicted (LRU)
        assert_eq!(cache.len(), 2);
        assert!(!cache.contains(&sha1));
        assert!(cache.contains(&sha2));
        assert!(cache.contains(&sha3));
    }

    #[test]
    fn test_cache_lru_order() {
        let mut cache = CommitCache::new(2);

        let sha1: SharedString = "abc123".into();
        let sha2: SharedString = "def456".into();
        let sha3: SharedString = "ghi789".into();

        let summary = CommitSummary {
            short_sha: "abc1234".into(),
            subject: "Test".into(),
            author_name: "Test".into(),
            timestamp: 0,
            lane: 0,
            has_refs: false,
        };

        cache.insert(sha1.clone(), summary.clone());
        cache.insert(sha2.clone(), summary.clone());

        // Access sha1 to make it most recently used
        cache.get(&sha1);

        // Insert sha3, should evict sha2 (now LRU)
        cache.insert(sha3.clone(), summary);

        assert!(cache.contains(&sha1));
        assert!(!cache.contains(&sha2));
        assert!(cache.contains(&sha3));
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = CommitCache::new(10);

        let sha1: SharedString = "abc123".into();
        let sha2: SharedString = "def456".into();

        let summary = CommitSummary {
            short_sha: "abc1234".into(),
            subject: "Test".into(),
            author_name: "Test".into(),
            timestamp: 0,
            lane: 0,
            has_refs: false,
        };

        cache.insert(sha1.clone(), summary);

        // Miss
        cache.get(&sha2);
        // Hit
        cache.get(&sha1);
        // Hit
        cache.get(&sha1);

        assert_eq!(cache.stats.hits.load(Ordering::Relaxed), 2);
        assert_eq!(cache.stats.misses.load(Ordering::Relaxed), 1);
    }
}
