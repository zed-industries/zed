#![cfg_attr(docsrs, doc(cfg(feature = "proc_macro")))]

/*!
Procedural macros for defining functions that wrap a static-ref cache object.

```rust,no_run
use std::thread::sleep;
use std::time::Duration;
use cached::proc_macro::cached;

/// Use an lru cache with size 100 and a `(String, String)` cache key
#[cached(size=100)]
fn keyed(a: String, b: String) -> usize {
    let size = a.len() + b.len();
    sleep(Duration::new(size as u64, 0));
    size
}
# pub fn main() { }
```

----

```rust,no_run
use std::thread::sleep;
use std::time::Duration;
use cached::proc_macro::cached;

/// Use a timed-lru cache with size 1, a TTL of 60s,
/// and a `(usize, usize)` cache key
#[cached(size=1, time=60)]
fn keyed(a: usize, b: usize) -> usize {
    let total = a + b;
    sleep(Duration::new(total as u64, 0));
    total
}
pub fn main() {
    keyed(1, 2);  // Not cached, will sleep (1+2)s

    keyed(1, 2);  // Cached, no sleep

    sleep(Duration::new(60, 0));  // Sleep for the TTL

    keyed(1, 2);  // 60s TTL has passed so the cached
                  // value has expired, will sleep (1+2)s

    keyed(1, 2);  // Cached, no sleep

    keyed(2, 1);  // New args, not cached, will sleep (2+1)s

    keyed(1, 2);  // Was evicted because of lru size of 1,
                  // will sleep (1+2)s
}
```

----

```rust,no_run
use std::thread::sleep;
use std::time::Duration;
use cached::proc_macro::cached;

/// Use a timed cache with a TTL of 60s
/// that refreshes the entry TTL on cache hit,
/// and a `(String, String)` cache key
#[cached(time=60, time_refresh=true)]
fn keyed(a: String, b: String) -> usize {
    let size = a.len() + b.len();
    sleep(Duration::new(size as u64, 0));
    size
}
# pub fn main() { }
```

----

```rust,no_run
use cached::proc_macro::cached;

# fn do_something_fallible() -> std::result::Result<(), ()> {
#     Ok(())
# }

/// Cache a fallible function. Only `Ok` results are cached.
#[cached(size=1, result = true)]
fn keyed(a: String) -> Result<usize, ()> {
    do_something_fallible()?;
    Ok(a.len())
}
# pub fn main() { }
```

----

```rust,no_run
use cached::proc_macro::cached;

/// Cache an optional function. Only `Some` results are cached.
#[cached(size=1, option = true)]
fn keyed(a: String) -> Option<usize> {
    if a == "a" {
        Some(a.len())
    } else {
        None
    }
}
# pub fn main() { }
```

----

```rust,no_run
use cached::proc_macro::cached;

/// Cache an optional function. Only `Some` results are cached.
/// When called concurrently, duplicate argument-calls will be
/// synchronized so as to only run once - the remaining concurrent
/// calls return a cached value.
#[cached(size=1, option = true, sync_writes = "default")]
fn keyed(a: String) -> Option<usize> {
    if a == "a" {
        Some(a.len())
    } else {
        None
    }
}
# pub fn main() { }
```

----

```rust,no_run
use cached::proc_macro::cached;
use cached::Return;

/// Get a `cached::Return` value that indicates
/// whether the value returned came from the cache:
/// `cached::Return.was_cached`.
/// Use an LRU cache and a `String` cache key.
#[cached(size=1, with_cached_flag = true)]
fn calculate(a: String) -> Return<String> {
    Return::new(a)
}
pub fn main() {
    let r = calculate("a".to_string());
    assert!(!r.was_cached);
    let r = calculate("a".to_string());
    assert!(r.was_cached);
    // Return<String> derefs to String
    assert_eq!(r.to_uppercase(), "A");
}
```

----

```rust,no_run
use cached::proc_macro::cached;
use cached::Return;

# fn do_something_fallible() -> std::result::Result<(), ()> {
#     Ok(())
# }

/// Same as the previous, but returning a Result
#[cached(size=1, result = true, with_cached_flag = true)]
fn calculate(a: String) -> Result<Return<usize>, ()> {
    do_something_fallible()?;
    Ok(Return::new(a.len()))
}
pub fn main() {
    match calculate("a".to_string()) {
        Err(e) => eprintln!("error: {:?}", e),
        Ok(r) => {
            println!("value: {:?}, was cached: {}", *r, r.was_cached);
            // value: "a", was cached: true
        }
    }
}
```

----

```rust,no_run
use cached::proc_macro::cached;
use cached::Return;

/// Same as the previous, but returning an Option
#[cached(size=1, option = true, with_cached_flag = true)]
fn calculate(a: String) -> Option<Return<usize>> {
    if a == "a" {
        Some(Return::new(a.len()))
    } else {
        None
    }
}
pub fn main() {
    if let Some(a) = calculate("a".to_string()) {
        println!("value: {:?}, was cached: {}", *a, a.was_cached);
        // value: "a", was cached: true
    }
}
```

----

```rust,no_run
use std::thread::sleep;
use std::time::Duration;
use cached::proc_macro::cached;
use cached::SizedCache;

/// Use an explicit cache-type with a custom creation block and custom cache-key generating block
#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}{}", a, b) }"#
)]
fn keyed(a: &str, b: &str) -> usize {
    let size = a.len() + b.len();
    sleep(Duration::new(size as u64, 0));
    size
}
# pub fn main() { }
```

----

```rust,no_run
use cached::proc_macro::once;
use std::time::Duration;

/// Only cache the initial function call.
/// Function will be re-executed after the cache
/// expires (according to `time` seconds).
/// When no (or expired) cache, concurrent calls
/// will synchronize (`sync_writes`) so the function
/// is only executed once.
#[once(time=10, option = true, sync_writes = true)]
fn keyed(a: String) -> Option<usize> {
    if a == "a" {
        Some(a.len())
    } else {
        None
    }
}
# pub fn main() { }
```

----

```rust
use std::thread::sleep;
use std::time::Duration;
use cached::proc_macro::cached;

/// Use a timed cache with a TTL of 60s.
/// Run a background thread to continuously refresh a specific key.
#[cached(time = 60, key = "String", convert = r#"{ String::from(a) }"#)]
fn keyed(a: &str) -> usize {
    a.len()
}
pub fn main() {
    let _handler = std::thread::spawn(|| {
        loop {
            sleep(Duration::from_secs(50));
            // this method is generated by the `cached` macro
            keyed_prime_cache("a");
        }
    });
    // handler.join().unwrap();
}
```

----

```rust
use std::thread::sleep;
use std::time::Duration;
use cached::proc_macro::once;

/// Run a background thread to continuously refresh a singleton.
#[once]
fn keyed() -> String {
    // do some long http request
    "some data".to_string()
}
pub fn main() {
    let _handler = std::thread::spawn(|| {
        loop {
            sleep(Duration::from_secs(60));
            // this method is generated by the `cached` macro
            keyed_prime_cache();
        }
    });
    // handler.join().unwrap();
}
```

----

```rust
use std::thread::sleep;
use std::time::Duration;
use cached::proc_macro::cached;

/// Run a background thread to continuously refresh every key of a cache
#[cached(key = "String", convert = r#"{ String::from(a) }"#)]
fn keyed(a: &str) -> usize {
    a.len()
}
pub fn main() {
    let _handler = std::thread::spawn(|| {
        loop {
            sleep(Duration::from_secs(60));
            let keys: Vec<String> = {
                // note the cache keys are a tuple of all function arguments, unless it's one value
                KEYED.lock().unwrap().get_store().keys().map(|k| k.clone()).collect()
            };
            for k in &keys {
                // this method is generated by the `cached` macro
                keyed_prime_cache(k);
            }
        }
    });
    // handler.join().unwrap();
}
```


*/

#[doc(inline)]
pub use cached_proc_macro::{cached, io_cached, once};
#[doc(inline)]
pub use cached_proc_macro_types::Return;
