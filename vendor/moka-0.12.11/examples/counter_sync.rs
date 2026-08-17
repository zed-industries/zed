//! This example demonstrates how to increment a cached `u64` counter. It uses the
//! `and_upsert_with` method of `Cache`.

use moka::{sync::Cache, Entry};

fn main() {
    let cache: Cache<String, u64> = Cache::new(100);
    let key = "key".to_string();

    let entry = increment_counter(&cache, &key);
    assert!(entry.is_fresh());
    assert!(!entry.is_old_value_replaced());
    assert_eq!(entry.into_value(), 1);

    let entry = increment_counter(&cache, &key);
    assert!(entry.is_fresh());
    assert!(entry.is_old_value_replaced());
    assert_eq!(entry.into_value(), 2);

    let entry = increment_counter(&cache, &key);
    assert!(entry.is_fresh());
    assert!(entry.is_old_value_replaced());
    assert_eq!(entry.into_value(), 3);
}

fn increment_counter(cache: &Cache<String, u64>, key: &str) -> Entry<String, u64> {
    cache.entry_by_ref(key).and_upsert_with(|maybe_entry| {
        if let Some(entry) = maybe_entry {
            // The entry exists, increment the value by 1.
            entry.into_value().saturating_add(1)
        } else {
            // The entry does not exist, insert a new value of 1.
            1
        }
    })
}
