use moka::sync::Cache;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Make an artificially small cache and 1-second ttl to observe eviction listener.
    let ttl = 1;
    {
        let cache = Cache::builder()
            .max_capacity(2)
            .time_to_live(Duration::from_secs(ttl))
            .eviction_listener(|key, value, cause| {
                println!("Evicted ({key:?},{value:?}) because {cause:?}")
            })
            .build();
        // Overload capacity of the cache.
        cache.insert(&0, "zero".to_string());
        cache.insert(&1, "one".to_string());
        cache.insert(&2, "twice".to_string());
        // This causes "twice" to be evicted by cause Replaced.
        cache.insert(&2, "two".to_string());
        // With 1-second ttl, keys 0 and 1 will be evicted if we wait long enough.
        sleep(Duration::from_secs(ttl + 1));
        println!("Wake up!");
        cache.insert(&3, "three".to_string());
        cache.insert(&4, "four".to_string());

        // Remove from cache and return value:
        if let Some(v) = cache.remove(&3) {
            println!("Removed: {v}")
        };
        // Or remove from cache without returning the value.
        cache.invalidate(&4);

        cache.insert(&5, "five".to_string());

        // invalidate_all() removes entries using a background thread, so there will
        // be some delay before entries are removed and the eviction listener is
        // called. If you want to remove all entries immediately, call
        // run_pending_tasks() method repeatedly like the loop below.
        cache.invalidate_all();
        loop {
            // Synchronization is limited to at most 500 entries for each call.
            cache.run_pending_tasks();
            // Check if all is done. Calling entry_count() requires calling
            // run_pending_tasks() first!
            if cache.entry_count() == 0 {
                break;
            }
        }

        cache.insert(&6, "six".to_string());
        // When cache is dropped eviction listener is not called. Either call
        // invalidate_all() or wait longer than ttl.
        sleep(Duration::from_secs(ttl + 1));
    } // cache is dropped here.

    println!("Cache structure removed.");
    sleep(Duration::from_secs(1));
    println!("Exit program.");
}
