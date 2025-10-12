use collections::HashMap;
use gpui::HighlightStyle;
use std::cell::RefCell;

use crate::rainbow_highlighter::fnv1a_hash;

thread_local! {
    static RAINBOW_CACHE: RefCell<RainbowCache> = RefCell::new(RainbowCache::new());
}

pub fn with_rainbow_cache<F, R>(f: F) -> R where F: FnOnce(&mut RainbowCache) -> R {
    RAINBOW_CACHE.with(|cache| f(&mut cache.borrow_mut()))
}

pub fn clear_rainbow_cache() {
    RAINBOW_CACHE.with(|cache| cache.borrow_mut().clear())
}

#[derive(Debug)]
pub struct RainbowCache {
    cache: HashMap<u64, HighlightStyle>,
    max_entries: usize,
}

impl RainbowCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::default(),
            max_entries: 1000,
        }
    }

    #[inline]
    pub fn get(&self, identifier: &str) -> Option<HighlightStyle> {
        let hash = fnv1a_hash(identifier);
        self.cache.get(&hash).copied()
    }

    pub fn insert(&mut self, identifier: &str, style: HighlightStyle) {
        if self.cache.len() >= self.max_entries {
            self.cache.retain(|hash, _| hash % 2 == 0);
        }

        let hash = fnv1a_hash(identifier);
        self.cache.insert(hash, style);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for RainbowCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_hit_miss() {
        let mut cache = RainbowCache::new();

        assert!(cache.get("test_var").is_none());

        let style = HighlightStyle::default();
        cache.insert("test_var", style);
        assert!(cache.get("test_var").is_some());

        assert!(cache.get("other_var").is_none());
    }

    #[test]
    fn test_eviction() {
        let mut cache = RainbowCache {
            cache: HashMap::default(),
            max_entries: 10,
        };

        let style = HighlightStyle::default();

        for i in 0..15 {
            cache.insert(&format!("var_{}", i), style);
        }

        assert!(cache.cache.len() < 15);
    }

    #[test]
    fn test_deterministic_hashing() {
        let hash1 = fnv1a_hash("my_variable");
        let hash2 = fnv1a_hash("my_variable");
        let hash3 = fnv1a_hash("other_variable");

        assert_eq!(hash1, hash2, "Same identifier should produce same hash");
        assert_ne!(hash1, hash3, "Different identifiers should produce different hashes");
    }
}
