#[macro_use]
extern crate cached;

use std::thread::sleep;
use std::time::{Duration, Instant};

use cached::SizedCache;

cached! {
    SLOW_FN: SizedCache<u32, String> = SizedCache::with_size(50);
    fn slow_fn(n: u32) -> String = {
        if n == 0 { return "done".to_string(); }
        sleep(Duration::new(1, 0));
        slow_fn(n-1)
    }
}

pub fn main() {
    println!("Initial run...");
    let now = Instant::now();
    let _ = slow_fn(10);
    println!("Elapsed: {}\n", now.elapsed().as_secs());

    println!("Cached run...");
    let now = Instant::now();
    let _ = slow_fn(10);
    println!("Elapsed: {}\n", now.elapsed().as_secs());

    // Inspect the cache
    {
        use cached::Cached; // must be in scope to access cache

        println!(" ** Cache info **");
        let cache = SLOW_FN.lock().unwrap();
        assert_eq!(cache.cache_hits().unwrap(), 1);
        println!("hits=1 -> {:?}", cache.cache_hits().unwrap() == 1);
        assert_eq!(cache.cache_misses().unwrap(), 11);
        println!("misses=11 -> {:?}", cache.cache_misses().unwrap() == 11);
        // make sure the cache-lock is dropped
    }

    println!("done!");
}
