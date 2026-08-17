//! This example demonstrates how to append an `i32` value to a cached `Vec<i32>`
//! value. It uses the `and_upsert_with` method of `Cache`.

use std::sync::Arc;

use moka::{future::Cache, Entry};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // We want to store a raw value `Vec<i32>` for each `String` key. We are going to
    // append `i32` values to the `Vec` in the cache.
    //
    // Note that we have to wrap the `Vec` in an `Arc<RwLock<_>>`. We need the `Arc`,
    // an atomic reference counted shared pointer, because `and_upsert_with` method
    // of `Cache` passes a _clone_ of the value to our closure, instead of passing a
    // `&mut` reference. We do not want to clone the `Vec` every time we append a
    // value to it, so we wrap it in an `Arc`. Then we need the `RwLock` because we
    // mutate the `Vec` when we append a value to it.
    //
    // The reason that `and_upsert_with` cannot pass a `&mut Vec<_>` to the closure
    // is because the internal concurrent hash table of `Cache` is a lock free data
    // structure and does not use any mutexes. So it cannot guarantee: (1) the `&mut
    // Vec<_>` is unique, and (2) it is not accessed concurrently by other threads.
    let cache: Cache<String, Arc<RwLock<Vec<i32>>>> = Cache::new(100);

    let key = "key".to_string();

    let entry = append_to_cached_vec(&cache, &key, 1).await;
    // It was not an update.
    assert!(!entry.is_old_value_replaced());
    assert!(entry.is_fresh());
    assert_eq!(*entry.into_value().read().await, &[1]);

    let entry = append_to_cached_vec(&cache, &key, 2).await;
    assert!(entry.is_fresh());
    // It was an update.
    assert!(entry.is_old_value_replaced());
    assert_eq!(*entry.into_value().read().await, &[1, 2]);

    let entry = append_to_cached_vec(&cache, &key, 3).await;
    assert!(entry.is_fresh());
    assert!(entry.is_old_value_replaced());
    assert_eq!(*entry.into_value().read().await, &[1, 2, 3]);
}

async fn append_to_cached_vec(
    cache: &Cache<String, Arc<RwLock<Vec<i32>>>>,
    key: &str,
    value: i32,
) -> Entry<String, Arc<RwLock<Vec<i32>>>> {
    cache
        .entry_by_ref(key)
        .and_upsert_with(|maybe_entry| async {
            if let Some(entry) = maybe_entry {
                // The entry exists, append the value to the Vec.
                let v = entry.into_value();
                v.write().await.push(value);
                v
            } else {
                // The entry does not exist, insert a new Vec containing
                // the value.
                Arc::new(RwLock::new(vec![value]))
            }
        })
        .await
}
