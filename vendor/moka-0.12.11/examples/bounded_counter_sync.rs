//! This example demonstrates how to increment a cached `u64` counter. It uses the
//! `and_compute_with` method of `Cache`.

use moka::{
    ops::compute::{CompResult, Op},
    sync::Cache,
};

fn main() {
    let cache: Cache<String, u64> = Cache::new(100);
    let key = "key".to_string();

    // This should insert a new counter value 1 to the cache, and return the value
    // with the kind of the operation performed.
    let result = inclement_or_remove_counter(&cache, &key);
    let CompResult::Inserted(entry) = result else {
        panic!("`Inserted` should be returned: {result:?}");
    };
    assert_eq!(entry.into_value(), 1);

    // This should increment the cached counter value by 1.
    let result = inclement_or_remove_counter(&cache, &key);
    let CompResult::ReplacedWith(entry) = result else {
        panic!("`ReplacedWith` should be returned: {result:?}");
    };
    assert_eq!(entry.into_value(), 2);

    // This should remove the cached counter from the cache, and returns the
    // _removed_ value.
    let result = inclement_or_remove_counter(&cache, &key);
    let CompResult::Removed(entry) = result else {
        panic!("`Removed` should be returned: {result:?}");
    };
    assert_eq!(entry.into_value(), 2);

    // The key should no longer exist.
    assert!(!cache.contains_key(&key));

    // This should start over; insert a new counter value 1 to the cache.
    let result = inclement_or_remove_counter(&cache, &key);
    let CompResult::Inserted(entry) = result else {
        panic!("`Inserted` should be returned: {result:?}");
    };
    assert_eq!(entry.into_value(), 1);
}

/// Increment a cached `u64` counter. If the counter is greater than or equal to 2,
/// remove it.
///
/// This method uses cache's `and_compute_with` method.
fn inclement_or_remove_counter(cache: &Cache<String, u64>, key: &str) -> CompResult<String, u64> {
    // - If the counter does not exist, insert a new value of 1.
    // - If the counter is less than 2, increment it by 1.
    // - If the counter is greater than or equal to 2, remove it.
    cache.entry_by_ref(key).and_compute_with(|maybe_entry| {
        if let Some(entry) = maybe_entry {
            // The entry exists.
            let counter = entry.into_value();
            if counter < 2 {
                // Increment the counter by 1.
                Op::Put(counter.saturating_add(1))
            } else {
                // Remove the entry.
                Op::Remove
            }
        } else {
            // The entry does not exist, insert a new value of 1.
            Op::Put(1)
        }
    })
}
