use crate::highlight_map::{CaptureId, CapturedRange};
use collections::FxHasher;
use lru::LruCache;
use parking_lot::Mutex;
use smallvec::{Array, SmallVec};
use std::{hash::Hash, hash::Hasher as _, sync::Arc};

pub const MAX_TEXT_CAPTURES_CACHE_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_TEXT_CAPTURES_ENTRY_BYTES: usize = MAX_TEXT_CAPTURES_CACHE_BYTES / 8;
const APPROXIMATE_LRU_NODE_BYTES: usize = 4 * size_of::<usize>();

struct CostBudgetedLru<K: Hash + Eq, V> {
    entries: LruCache<K, (V, usize)>,
    total_cost: usize,
    max_total_cost: usize,
    max_entry_cost: usize,
}

impl<K: Hash + Eq, V> CostBudgetedLru<K, V> {
    const ENTRY_OVERHEAD_BYTES: usize =
        size_of::<K>() + size_of::<(V, usize)>() + APPROXIMATE_LRU_NODE_BYTES;

    fn new(max_total_cost: usize, max_entry_cost: usize) -> Self {
        Self {
            entries: LruCache::unbounded(),
            total_cost: 0,
            max_total_cost,
            max_entry_cost,
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        self.entries.get(key).map(|(value, _)| value)
    }

    fn insert(&mut self, key: K, value: V, cost: usize) {
        if cost > self.max_entry_cost {
            return;
        }
        let cost = cost + Self::ENTRY_OVERHEAD_BYTES;
        if let Some((_, old_cost)) = self.entries.put(key, (value, cost)) {
            self.total_cost -= old_cost;
        }
        self.total_cost += cost;
        while self.total_cost > self.max_total_cost {
            let Some((_, (_, evicted_cost))) = self.entries.pop_lru() else {
                break;
            };
            self.total_cost -= evicted_cost;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextCapturesKey {
    text_hash: u64,
    text_len: usize,
}

impl TextCapturesKey {
    pub fn new<'a>(text_chunks: impl Iterator<Item = &'a str>, text_len: usize) -> Self {
        let mut hasher = FxHasher::default();
        for chunk in text_chunks {
            hasher.write(chunk.as_bytes());
        }
        Self {
            text_hash: hasher.finish(),
            text_len,
        }
    }
}

struct TextCapturesEntry {
    text: Arc<str>,
    captures: Arc<[CapturedRange]>,
}

pub struct TextHighlightCache(Mutex<CostBudgetedLru<TextCapturesKey, TextCapturesEntry>>);

impl Default for TextHighlightCache {
    fn default() -> Self {
        Self(Mutex::new(CostBudgetedLru::new(
            MAX_TEXT_CAPTURES_CACHE_BYTES,
            MAX_TEXT_CAPTURES_ENTRY_BYTES,
        )))
    }
}

impl TextHighlightCache {
    pub fn get<'a>(
        &self,
        key: &TextCapturesKey,
        text_chunks: impl Iterator<Item = &'a str>,
    ) -> Option<Arc<[CapturedRange]>> {
        let mut cache = self.0.lock();
        let entry = cache.get(key)?;
        if !chunks_match_text(&entry.text, text_chunks) {
            return None;
        }
        Some(entry.captures.clone())
    }

    pub fn insert(
        &self,
        key: TextCapturesKey,
        text: Arc<str>,
        captures: Arc<[CapturedRange]>,
    ) -> Arc<[CapturedRange]> {
        let captures_cost = captures
            .iter()
            .map(|captured| {
                size_of::<CapturedRange>() + small_vec_heap_bytes(&captured.capture_ids)
            })
            .sum::<usize>();
        let cost = text.len() + captures_cost;
        self.0.lock().insert(
            key,
            TextCapturesEntry {
                text,
                captures: Arc::clone(&captures),
            },
            cost,
        );
        captures
    }
}

fn small_vec_heap_bytes<A: Array>(vec: &SmallVec<A>) -> usize {
    if vec.spilled() {
        vec.capacity() * size_of::<A::Item>()
    } else {
        0
    }
}

fn chunks_match_text<'a>(text: &str, text_chunks: impl Iterator<Item = &'a str>) -> bool {
    let mut remaining = text;
    for chunk in text_chunks {
        let Some(rest) = remaining.strip_prefix(chunk) else {
            return false;
        };
        remaining = rest;
    }
    remaining.is_empty()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HighlightCaptureRef {
    pub grammar_index: usize,
    pub capture_id: CaptureId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;

    #[test]
    fn test_budget_evicts_least_recently_used() {
        let overhead = CostBudgetedLru::<&str, u32>::ENTRY_OVERHEAD_BYTES;
        let mut cache = CostBudgetedLru::<&str, u32>::new(2 * (40 + overhead), 100);
        cache.insert("a", 1, 40);
        cache.insert("b", 2, 40);
        cache.get(&"a");
        cache.insert("c", 3, 40);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"c"), Some(&3));
        assert_eq!(cache.total_cost, 2 * (40 + overhead));
    }

    #[test]
    fn test_replacing_an_entry_updates_the_budget() {
        let overhead = CostBudgetedLru::<&str, u32>::ENTRY_OVERHEAD_BYTES;
        let mut cache = CostBudgetedLru::<&str, u32>::new(100 + 2 * overhead, 100);
        cache.insert("a", 1, 60);
        cache.insert("a", 2, 30);
        assert_eq!(cache.total_cost, 30 + overhead);
        assert_eq!(cache.get(&"a"), Some(&2));
        cache.insert("b", 3, 70);
        assert_eq!(cache.get(&"a"), Some(&2));
        assert_eq!(cache.get(&"b"), Some(&3));
    }

    #[test]
    fn test_oversized_entries_are_rejected_without_flushing() {
        let overhead = CostBudgetedLru::<&str, u32>::ENTRY_OVERHEAD_BYTES;
        let mut cache = CostBudgetedLru::<&str, u32>::new(100 + overhead, 50);
        cache.insert("a", 1, 40);
        cache.insert("b", 2, 60);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.total_cost, 40 + overhead);
    }

    #[test]
    fn test_zero_cost_entries_still_consume_budget() {
        let overhead = CostBudgetedLru::<usize, u32>::ENTRY_OVERHEAD_BYTES;
        let mut cache = CostBudgetedLru::<usize, u32>::new(3 * overhead, 100);
        for key in 0..5_usize {
            cache.insert(key, key as u32, 0);
        }
        assert_eq!(cache.get(&0), None);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&2));
        assert_eq!(cache.get(&3), Some(&3));
        assert_eq!(cache.get(&4), Some(&4));
        assert_eq!(
            cache.total_cost,
            3 * overhead,
            "Entries with zero caller-supplied cost must still be bounded by the budget"
        );
    }

    #[test]
    fn test_oversized_captures_are_returned_but_not_cached() {
        let text = Arc::<str>::from("fn main() {}");
        let max_entry_cost = text.len() + 2 * size_of::<CapturedRange>();
        let cache = TextHighlightCache(Mutex::new(CostBudgetedLru::new(
            max_entry_cost * 10,
            max_entry_cost,
        )));
        let captured_range = |range: Range<usize>| CapturedRange {
            range,
            capture_ids: [CaptureId(0)].into_iter().collect(),
        };

        let small_text = Arc::<str>::from("fn f() {}");
        let small_key = TextCapturesKey::new([small_text.as_ref()].into_iter(), small_text.len());
        let small_captures = cache.insert(
            small_key.clone(),
            Arc::clone(&small_text),
            [captured_range(0..2)].into_iter().collect(),
        );
        assert_eq!(small_captures.as_ref(), &[captured_range(0..2)]);
        assert_eq!(
            cache
                .get(&small_key, [small_text.as_ref()].into_iter())
                .as_deref(),
            Some(small_captures.as_ref()),
            "captures within the entry budget must be cached as-is"
        );

        let big_key = TextCapturesKey::new([text.as_ref()].into_iter(), text.len());
        let big_captures_source = [
            captured_range(0..2),
            captured_range(3..7),
            captured_range(8..9),
        ]
        .into_iter()
        .collect::<Arc<[_]>>();
        let big_captures = cache.insert(
            big_key.clone(),
            Arc::clone(&text),
            Arc::clone(&big_captures_source),
        );
        assert!(
            Arc::ptr_eq(&big_captures, &big_captures_source),
            "captures over the entry budget must be returned unchanged"
        );
        assert_eq!(
            cache.get(&big_key, [text.as_ref()].into_iter()),
            None,
            "captures over the entry budget must not be cached"
        );
    }
}
