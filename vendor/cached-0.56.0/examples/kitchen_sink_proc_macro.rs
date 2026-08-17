use cached::proc_macro::cached;
use cached::Return;
use cached::{Cached, SizedCache, UnboundCache};
use std::cmp::Eq;
use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;
use std::thread::{sleep, spawn};
use std::time::Duration;

// cached shorthand, uses the default unbounded cache.
// Equivalent to specifying `ty = "UnboundCache<(u32), u32>", create= "{ UnboundCache::new() }"`
#[cached]
fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

#[cached(name = "FLIB")]
fn fib_2(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

// Same as above, but preallocates some space.
#[cached(
    ty = "UnboundCache<u32, u32>",
    create = "{ UnboundCache::with_capacity(50) }"
)]
fn fib_specific(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    fib_specific(n - 1) + fib_specific(n - 2)
}

// Specify a specific cache type
// Note that the cache key type is a tuple of function argument types.
#[cached(
    ty = "SizedCache<(u32, u32), u32>",
    create = "{ SizedCache::with_size(100) }"
)]
fn slow(a: u32, b: u32) -> u32 {
    sleep(Duration::new(2, 0));
    a * b
}

// Specify a specific cache type and an explicit key expression
// Note that the cache key type is a `String` created from the borrow arguments
// Note that key is not used, convert requires either key or type to be set.
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

#[cached(key = "String", convert = r#"{ format!("{}{}", a, b) }"#)]
fn keyed_key(a: &str, b: &str) -> usize {
    let size = a.len() + b.len();
    sleep(Duration::new(size as u64, 0));
    size
}

// Implement our own cache type
struct MyCache<K: Hash + Eq, V> {
    store: HashMap<K, V>,
    capacity: usize,
}
impl<K: Hash + Eq, V> MyCache<K, V> {
    pub fn with_capacity(size: usize) -> MyCache<K, V> {
        MyCache {
            store: HashMap::with_capacity(size),
            capacity: size,
        }
    }
}
impl<K: Hash + Eq, V> Cached<K, V> for MyCache<K, V> {
    fn cache_get<Q>(&mut self, k: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.store.get(k)
    }
    fn cache_get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.store.get_mut(k)
    }
    fn cache_get_or_set_with<F: FnOnce() -> V>(&mut self, k: K, f: F) -> &mut V {
        self.store.entry(k).or_insert_with(f)
    }
    fn cache_try_get_or_set_with<F: FnOnce() -> Result<V, E>, E>(
        &mut self,
        k: K,
        f: F,
    ) -> Result<&mut V, E> {
        let v = match self.store.entry(k) {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => vacant.insert(f()?),
        };

        Ok(v)
    }
    fn cache_set(&mut self, k: K, v: V) -> Option<V> {
        self.store.insert(k, v)
    }
    fn cache_remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.store.remove(k)
    }
    fn cache_clear(&mut self) {
        self.store.clear();
    }
    fn cache_reset(&mut self) {
        self.store = HashMap::with_capacity(self.capacity);
    }
    fn cache_size(&self) -> usize {
        self.store.len()
    }
}

// Specify our custom cache and supply an instance to use
#[cached(ty = "MyCache<u32, ()>", create = "{ MyCache::with_capacity(50) }")]
fn custom(n: u32) {
    if n == 0 {
        return;
    }
    custom(n - 1);
}

// handle results, don't cache errors
#[cached(result = true)]
fn slow_result(a: u32, b: u32) -> Result<u32, ()> {
    sleep(Duration::new(2, 0));
    Ok(a * b)
}

// return a flag indicated whether the result was cached
#[cached(with_cached_flag = true)]
fn with_cached_flag(a: String) -> Return<String> {
    sleep(Duration::new(1, 0));
    Return::new(a)
}

// return a flag indicated whether the result was cached, with a result type
#[cached(result = true, with_cached_flag = true)]
fn with_cached_flag_result(a: String) -> Result<cached::Return<String>, ()> {
    sleep(Duration::new(1, 0));
    Ok(Return::new(a))
}

// return a flag indicated whether the result was cached, with an option type
#[cached(option = true, with_cached_flag = true)]
fn with_cached_flag_option(a: String) -> Option<Return<String>> {
    sleep(Duration::new(1, 0));
    Some(Return::new(a))
}

// A simple cache that expires after a second. We'll keep the
// value fresh by priming it in a separate thread.
#[cached(time = 1)]
fn expires_for_priming(a: i32) -> i32 {
    a
}

// NOTE:
// The following fails with compilation error
// ```
//   error:
//   When specifying `with_cached_flag = true`, the return type must be wrapped in `cached::Return<T>`.
//   The following return types are supported:
//   |    `cached::Return<T>`
//   |    `std::result::Result<cachedReturn<T>, E>`
//   |    `std::option::Option<cachedReturn<T>>`
//   Found type: std::result::Result<u32,()>.
// ```
//
// #[cached(with_cached_flag = true)]
// fn with_cached_flag_requires_return_type(a: u32) -> std::result::Result<u32, ()> {
//     Ok(1)
// }

pub fn main() {
    println!("\n ** default cache with default name **");
    fib(3);
    fib(3);
    {
        let cache = FIB.lock().unwrap();
        println!("hits: {:?}", cache.cache_hits());
        assert_eq!(cache.cache_hits().unwrap(), 2);
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses(), Some(4));
        // make sure lock is dropped
    }
    fib(10);
    fib(10);

    println!("\n ** default cache with explicit name **");
    fib_2(3);
    fib_2(3);
    {
        let cache = FLIB.lock().unwrap();
        println!("hits: {:?}", cache.cache_hits());
        assert_eq!(cache.cache_hits().unwrap(), 1);
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses(), Some(1));
        // make sure lock is dropped
    }

    println!("\n ** specific cache **");
    fib_specific(20);
    fib_specific(20);
    {
        let cache = FIB_SPECIFIC.lock().unwrap();
        println!("hits: {:?}", cache.cache_hits());
        assert_eq!(cache.cache_hits().unwrap(), 19);
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses(), Some(21));
        // make sure lock is dropped
    }
    fib_specific(20);
    fib_specific(20);

    println!("\n ** custom cache **");
    custom(25);
    {
        let cache = CUSTOM.lock().unwrap();
        println!("hits: {:?}", cache.cache_hits());
        assert_eq!(cache.cache_hits(), None);
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses(), None);
        //custom cache doesn't implement these so they're None
        // make sure lock is dropped
    }

    println!("\n ** slow func **");
    println!(" - first run `slow(10)`");
    slow(10, 10);
    println!(" - second run `slow(10)`");
    slow(10, 10);
    {
        let cache = SLOW.lock().unwrap();
        println!("hits: {:?}", cache.cache_hits());
        assert_eq!(cache.cache_hits().unwrap(), 1);
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses().unwrap(), 1);
        // make sure the cache-lock is dropped
    }

    println!("\n ** slow result func **");
    println!(" - first run `slow_result(10)`");
    let _ = slow_result(10, 10);
    println!(" - second run `slow_result(10)`");
    let _ = slow_result(10, 10);
    {
        let cache = SLOW_RESULT.lock().unwrap();
        println!("hits: {:?}", cache.cache_hits());
        assert_eq!(cache.cache_hits().unwrap(), 1);
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses(), Some(1));
        // make sure the cache-lock is dropped
    }

    println!("\n ** with cached flag func **");
    println!(" - first run `with_cached_flag(\"a\")`");
    let r = with_cached_flag("a".to_string());
    println!("was cached: {}", r.was_cached);
    println!(" - second run `with_cached_flag(\"a\")`");
    let r = with_cached_flag("a".to_string());
    println!("was cached: {}", r.was_cached);
    println!("derefs to inner, *r == \"a\" : {}", *r == "a");
    println!(
        "derefs to inner, r.as_str() == \"a\" : {}",
        r.as_str() == "a"
    );
    {
        let cache = WITH_CACHED_FLAG.lock().unwrap();
        println!("hits: {:?}", cache.cache_hits());
        assert_eq!(cache.cache_hits().unwrap(), 1);
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses(), Some(1));
        // make sure the cache-lock is dropped
    }

    println!("\n ** with cached flag result func **");
    println!(" - first run `with_cached_flag_result(\"a\")`");
    let r = with_cached_flag_result("a".to_string()).expect("with_cached_flag_result failed");
    println!("was cached: {}", r.was_cached);
    println!(" - second run `with_cached_flag_result(\"a\")`");
    let r = with_cached_flag_result("a".to_string()).expect("with_cached_flag_result failed");
    println!("was cached: {}", r.was_cached);
    println!("derefs to inner, *r : {:?}", *r);
    println!("derefs to inner, *r == \"a\" : {}", *r == "a");
    println!(
        "derefs to inner, r.as_str() == \"a\" : {}",
        r.as_str() == "a"
    );
    {
        let cache = WITH_CACHED_FLAG_RESULT.lock().unwrap();
        println!("hits: {:?}", cache.cache_hits());
        assert_eq!(cache.cache_hits().unwrap(), 1);
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses(), Some(1));
        // make sure the cache-lock is dropped
    }

    println!("\n ** with cached flag option func **");
    println!(" - first run `with_cached_flag_option(\"a\")`");
    let r = with_cached_flag_option("a".to_string()).expect("with_cached_flag_result failed");
    println!("was cached: {}", r.was_cached);
    println!(" - second run `with_cached_flag_option(\"a\")`");
    let r = with_cached_flag_option("a".to_string()).expect("with_cached_flag_result failed");
    println!("was cached: {}", r.was_cached);
    println!("derefs to inner, *r : {:?}", *r);
    println!("derefs to inner, *r == \"a\" : {}", *r == "a");
    println!(
        "derefs to inner, r.as_str() == \"a\" : {}",
        r.as_str() == "a"
    );
    {
        let cache = WITH_CACHED_FLAG_OPTION.lock().unwrap();
        println!("hits: {:?}", cache.cache_hits());
        assert_eq!(cache.cache_hits().unwrap(), 1);
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses(), Some(1));
        // make sure the cache-lock is dropped
    }

    println!("\n ** refresh by priming **");
    let h = spawn(|| {
        for _ in 1..6 {
            expires_for_priming_prime_cache(1);
            sleep(Duration::from_millis(500));
        }
    });
    sleep(Duration::from_millis(200));
    for n in 1..6 {
        assert_eq!(1, expires_for_priming(1));
        {
            let c = EXPIRES_FOR_PRIMING.lock().unwrap();
            assert_eq!(n, c.cache_hits().unwrap());
            assert_eq!(0, c.cache_misses().unwrap());
            println!(
                "primed cache hits: {}, misses: {}",
                c.cache_hits().unwrap(),
                c.cache_misses().unwrap()
            );
        }
        sleep(Duration::from_millis(500));
    }
    h.join().unwrap();
    println!("now wait for expiration");
    sleep(Duration::from_millis(1000));
    assert_eq!(1, expires_for_priming(1));
    {
        let c = EXPIRES_FOR_PRIMING.lock().unwrap();
        assert_eq!(5, c.cache_hits().unwrap());
        assert_eq!(1, c.cache_misses().unwrap());
        println!(
            "primed cache hits: {}, misses: {}",
            c.cache_hits().unwrap(),
            c.cache_misses().unwrap()
        );
    }

    println!("\ndone!");
}
