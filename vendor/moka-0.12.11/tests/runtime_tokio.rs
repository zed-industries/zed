#![cfg(all(test, feature = "future"))]

use std::sync::Arc;

use moka::future::Cache;
use tokio::sync::Barrier;

#[tokio::test]
async fn main() {
    const NUM_TASKS: usize = 12;
    const NUM_THREADS: usize = 4;
    const NUM_KEYS_PER_TASK: usize = 64;

    fn value(n: usize) -> String {
        format!("value {n}")
    }

    // Create a cache that can store up to 10,000 entries.
    let cache = Cache::new(10_000);

    let barrier = Arc::new(Barrier::new(NUM_THREADS + NUM_TASKS));

    // Spawn async tasks and write to and read from the cache.
    let tasks: Vec<_> = (0..NUM_TASKS)
        .map(|i| {
            // To share the same cache across the async tasks and OS threads, clone
            // it. This is a cheap operation.
            let my_cache = cache.clone();
            let my_barrier = Arc::clone(&barrier);
            let start = i * NUM_KEYS_PER_TASK;
            let end = (i + 1) * NUM_KEYS_PER_TASK;

            tokio::spawn(async move {
                // Wait for the all async tasks and threads to be spawned.
                my_barrier.wait().await;

                // Insert 64 entries. (NUM_KEYS_PER_TASK = 64)
                for key in start..end {
                    my_cache.insert(key, value(key)).await;
                    assert_eq!(my_cache.get(&key).await, Some(value(key)));
                }

                // Invalidate every 4 element of the inserted entries.
                for key in (start..end).step_by(4) {
                    my_cache.invalidate(&key).await;
                }
            })
        })
        .collect();

    // Spawn OS threads and write to and read from the cache.
    let threads: Vec<_> = (0..NUM_THREADS)
        .map(|i| i + NUM_TASKS)
        .map(|i| {
            let my_cache = cache.clone();
            let my_barrier = Arc::clone(&barrier);
            let start = i * NUM_KEYS_PER_TASK;
            let end = (i + 1) * NUM_KEYS_PER_TASK;
            let rt = tokio::runtime::Handle::current();

            std::thread::spawn(move || {
                // Wait for the all async tasks and threads to be spawned.
                rt.block_on(my_barrier.wait());

                // Insert 64 entries. (NUM_KEYS_PER_TASK = 64)
                for key in start..end {
                    rt.block_on(my_cache.insert(key, value(key)));
                    assert_eq!(rt.block_on(my_cache.get(&key)), Some(value(key)));
                }

                // Invalidate every 4 element of the inserted entries.
                for key in (start..end).step_by(4) {
                    rt.block_on(my_cache.invalidate(&key));
                }
            })
        })
        .collect();

    // Wait for all tasks and threads to complete.
    futures_util::future::join_all(tasks).await;
    for t in threads {
        t.join().unwrap();
    }

    // Verify the result.
    for key in 0..(NUM_TASKS * NUM_KEYS_PER_TASK) {
        if key % 4 == 0 {
            assert_eq!(cache.get(&key).await, None);
        } else {
            assert_eq!(cache.get(&key).await, Some(value(key)));
        }
    }
}
