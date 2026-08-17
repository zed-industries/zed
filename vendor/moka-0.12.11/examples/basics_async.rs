// Use the asynchronous cache.
use moka::future::Cache;

#[tokio::main]
async fn main() {
    const NUM_TASKS: usize = 16;
    const NUM_KEYS_PER_TASK: usize = 64;

    fn value(n: usize) -> String {
        format!("value {n}")
    }

    // Create a cache that can store up to 10,000 entries.
    let cache = Cache::new(10_000);

    // Spawn async tasks and write to and read from the cache.
    let tasks: Vec<_> = (0..NUM_TASKS)
        .map(|i| {
            // To share the same cache across the async tasks, clone it.
            // This is a cheap operation.
            let my_cache = cache.clone();
            let start = i * NUM_KEYS_PER_TASK;
            let end = (i + 1) * NUM_KEYS_PER_TASK;

            tokio::spawn(async move {
                // Insert 64 entries. (NUM_KEYS_PER_TASK = 64)
                for key in start..end {
                    // insert() is an async method, so await it.
                    my_cache.insert(key, value(key)).await;
                    // get() returns Option<String>, a clone of the stored value.
                    assert_eq!(my_cache.get(&key).await, Some(value(key)));
                }

                // Invalidate every 4 element of the inserted entries.
                for key in (start..end).step_by(4) {
                    // invalidate() is an async method, so await it.
                    my_cache.invalidate(&key).await;
                }
            })
        })
        .collect();

    // Wait for all tasks to complete.
    futures_util::future::join_all(tasks).await;

    // Verify the result.
    for key in 0..(NUM_TASKS * NUM_KEYS_PER_TASK) {
        if key % 4 == 0 {
            assert_eq!(cache.get(&key).await, None);
        } else {
            assert_eq!(cache.get(&key).await, Some(value(key)));
        }
    }
}
