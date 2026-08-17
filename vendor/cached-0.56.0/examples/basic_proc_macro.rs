use cached::proc_macro::cached;
use cached::proc_macro::once;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[cached(size = 50)]
fn slow_fn(n: u32) -> String {
    if n == 0 {
        return "done".to_string();
    }
    sleep(Duration::new(1, 0));
    slow_fn(n - 1)
}

#[once(time = 1)]
fn once_slow_fn(n: u32) -> String {
    sleep(Duration::new(1, 0));
    format!("{n}")
}

pub fn main() {
    println!("[cached] Initial run...");
    let now = Instant::now();
    let _ = slow_fn(10);
    println!("[cached] Elapsed: {}\n", now.elapsed().as_secs());

    println!("[cached] Cached run...");
    let now = Instant::now();
    let _ = slow_fn(10);
    println!("[cached] Elapsed: {}\n", now.elapsed().as_secs());

    // Inspect the cache
    {
        use cached::Cached; // must be in scope to access cache

        println!("[cached] ** Cache info **");
        let cache = SLOW_FN.lock().unwrap();
        assert_eq!(cache.cache_hits().unwrap(), 1);
        println!("[cached] hits=1 -> {:?}", cache.cache_hits().unwrap() == 1);
        assert_eq!(cache.cache_misses().unwrap(), 11);
        println!(
            "[cached] misses=11 -> {:?}",
            cache.cache_misses().unwrap() == 11
        );
        // make sure the cache-lock is dropped
    }

    println!("[once] Initial run...");
    let now = Instant::now();
    let _ = once_slow_fn(10);
    println!("[once] Elapsed: {}\n", now.elapsed().as_secs());

    println!("[once] Cached run...");
    let now = Instant::now();
    let _ = once_slow_fn(10);
    println!("[once] Elapsed: {}\n", now.elapsed().as_secs());

    println!("done!");
}
