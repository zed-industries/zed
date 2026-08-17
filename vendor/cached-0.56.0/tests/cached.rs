/*!
Full tests of macro-defined functions
*/
#[macro_use]
extern crate cached;

use cached::{
    proc_macro::cached, proc_macro::once, Cached, CanExpire, ExpiringValueCache, SizedCache,
    TimedCache, TimedSizedCache, UnboundCache,
};
use serial_test::serial;
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

cached! {
    UNBOUND_FIB;
    fn fib0(n: u32) -> u32 = {
        if n == 0 || n == 1 { return n }
        fib0(n-1) + fib0(n-2)
    }
}

#[test]
fn test_unbound_cache() {
    fib0(20);
    {
        let cache = UNBOUND_FIB.lock().unwrap();
        assert_eq!(21, cache.cache_size());
    }
}

cached! {
    SIZED_FIB: SizedCache<u32, u32> = SizedCache::with_size(3);
    fn fib1(n: u32) -> u32 = {
        if n == 0 || n == 1 { return n }
        fib1(n-1) + fib1(n-2)
    }
}

#[test]
fn test_sized_cache() {
    let last = fib1(20);
    {
        let cache = SIZED_FIB.lock().unwrap();
        assert_eq!(3, cache.cache_size());
        let items = cache.get_order().iter().collect::<Vec<_>>();
        assert_eq!(3, items.len());
        // (arg, result)
        assert_eq!(&(20, last), items[0]);
    }
}

cached! {
    TIMED: TimedCache<u32, u32> = TimedCache::with_lifespan_and_capacity(Duration::from_secs(2), 5);
    fn timed(n: u32) -> u32 = {
        sleep(Duration::new(3, 0));
        n
    }
}

#[test]
fn test_timed_cache() {
    timed(1);
    timed(1);
    {
        let cache = TIMED.lock().unwrap();
        assert_eq!(1, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
    }
    sleep(Duration::new(3, 0));
    timed(1);
    {
        let cache = TIMED.lock().unwrap();
        assert_eq!(2, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
    }
    {
        let mut cache = TIMED.lock().unwrap();
        assert_eq!(
            2,
            cache
                .cache_set_lifespan(Duration::from_secs(1))
                .unwrap()
                .as_secs()
        );
    }
    timed(1);
    sleep(Duration::new(1, 0));
    timed(1);
    {
        let cache = TIMED.lock().unwrap();
        assert_eq!(3, cache.cache_misses().unwrap());
        assert_eq!(2, cache.cache_hits().unwrap());
    }
}

cached! {
    TIMED_SIZED: TimedSizedCache<u32, u32> = TimedSizedCache::with_size_and_lifespan(3, Duration::from_secs(2));
    fn timefac(n: u32) -> u32 = {
        sleep(Duration::new(1, 0));
        if n > 1 {
            n * timefac(n - 1)
        } else {
            n
        }
    }
}

#[test]
fn test_timed_sized_cache() {
    timefac(1);
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(1, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
    }
    sleep(Duration::new(3, 0));
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(2, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
    }
    {
        let mut cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(
            2,
            cache
                .cache_set_lifespan(Duration::from_secs(1))
                .unwrap()
                .as_secs()
        );
    }
    timefac(1);
    sleep(Duration::new(1, 0));
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(3, cache.cache_misses().unwrap());
        assert_eq!(2, cache.cache_hits().unwrap());
    }
    {
        let mut cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(
            1,
            cache
                .cache_set_lifespan(Duration::from_secs(6))
                .unwrap()
                .as_secs()
        );
    }
    timefac(2);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(4, cache.cache_misses().unwrap());
        assert_eq!(3, cache.cache_hits().unwrap());
    }
    timefac(3);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(5, cache.cache_misses().unwrap());
        assert_eq!(4, cache.cache_hits().unwrap());
    }
    timefac(3);
    timefac(2);
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(5, cache.cache_misses().unwrap());
        assert_eq!(7, cache.cache_hits().unwrap());
    }
    timefac(4);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(6, cache.cache_misses().unwrap());
        assert_eq!(8, cache.cache_hits().unwrap());
    }
    timefac(6);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(8, cache.cache_misses().unwrap());
        assert_eq!(9, cache.cache_hits().unwrap());
    }
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(9, cache.cache_misses().unwrap());
        assert_eq!(9, cache.cache_hits().unwrap());
        assert_eq!(3, cache.cache_size());
    }
}

cached! {
    STRING_CACHE_EXPLICIT: SizedCache<(String, String), String> = SizedCache::with_size(1);
    fn string_1(a: String, b: String) -> String = {
        a + b.as_ref()
    }
}

#[test]
fn test_string_cache() {
    string_1("a".into(), "b".into());
    {
        let cache = STRING_CACHE_EXPLICIT.lock().unwrap();
        assert_eq!(1, cache.cache_size());
    }
}

cached_key! {
    TIMED_CACHE: TimedCache<u32, u32> = TimedCache::with_lifespan_and_capacity(Duration::from_secs(2), 5);
    Key = { n };
    fn timed_2(n: u32) -> u32 = {
        sleep(Duration::new(3, 0));
        n
    }
}

#[test]
fn test_timed_cache_key() {
    timed_2(1);
    timed_2(1);
    {
        let cache = TIMED_CACHE.lock().unwrap();
        assert_eq!(1, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
    }
    sleep(Duration::new(3, 0));
    timed_2(1);
    {
        let cache = TIMED_CACHE.lock().unwrap();
        assert_eq!(2, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
    }
}

cached_key! {
    SIZED_CACHE: SizedCache<String, usize> = SizedCache::with_size(2);
    Key = { format!("{a}{b}") };
    fn sized_key(a: &str, b: &str) -> usize = {
        let size = a.len() + b.len();
        sleep(Duration::new(size as u64, 0));
        size
    }
}

#[test]
fn test_sized_cache_key() {
    sized_key("a", "1");
    sized_key("a", "1");
    {
        let cache = SIZED_CACHE.lock().unwrap();
        assert_eq!(1, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
        assert_eq!(1, cache.cache_size());
    }
    sized_key("a", "1");
    {
        let cache = SIZED_CACHE.lock().unwrap();
        assert_eq!(1, cache.cache_misses().unwrap());
        assert_eq!(2, cache.cache_hits().unwrap());
        assert_eq!(1, cache.cache_size());
    }
    sized_key("a", "2");
    {
        let cache = SIZED_CACHE.lock().unwrap();
        assert_eq!(2, cache.cache_hits().unwrap());
        assert_eq!(2, cache.cache_size());
        assert_eq!(vec!["a2", "a1"], cache.key_order().collect::<Vec<_>>());
        assert_eq!(vec![&2, &2], cache.value_order().collect::<Vec<_>>());
    }
    sized_key("a", "3");
    {
        let cache = SIZED_CACHE.lock().unwrap();
        assert_eq!(2, cache.cache_size());
        assert_eq!(vec!["a3", "a2"], cache.key_order().collect::<Vec<_>>());
        assert_eq!(vec![&2, &2], cache.value_order().collect::<Vec<_>>());
    }
    sized_key("a", "4");
    sized_key("a", "5");
    {
        let cache = SIZED_CACHE.lock().unwrap();
        assert_eq!(2, cache.cache_size());
        assert_eq!(vec!["a5", "a4"], cache.key_order().collect::<Vec<_>>());
        assert_eq!(vec![&2, &2], cache.value_order().collect::<Vec<_>>());
    }
    sized_key("a", "67");
    sized_key("a", "8");
    {
        let cache = SIZED_CACHE.lock().unwrap();
        assert_eq!(2, cache.cache_size());
        assert_eq!(vec!["a8", "a67"], cache.key_order().collect::<Vec<_>>());
        assert_eq!(vec![&2, &3], cache.value_order().collect::<Vec<_>>());
    }
}

cached_key_result! {
    RESULT_CACHE_KEY: UnboundCache<u32, u32> = UnboundCache::new();
    Key = { n };
    fn test_result_key(n: u32) -> Result<u32, ()> = {
        if n < 5 { Ok(n) } else { Err(()) }
    }
}

#[test]
fn cache_result_key() {
    assert!(test_result_key(2).is_ok());
    assert!(test_result_key(4).is_ok());
    assert!(test_result_key(6).is_err());
    assert!(test_result_key(6).is_err());
    assert!(test_result_key(2).is_ok());
    assert!(test_result_key(4).is_ok());
    {
        let cache = RESULT_CACHE_KEY.lock().unwrap();
        assert_eq!(2, cache.cache_size());
        assert_eq!(2, cache.cache_hits().unwrap());
        assert_eq!(4, cache.cache_misses().unwrap());
    }
}

cached_result! {
    RESULT_CACHE: UnboundCache<u32, u32> = UnboundCache::new();
    fn test_result_no_default(n: u32) -> Result<u32, ()> = {
        if n < 5 { Ok(n) } else { Err(()) }
    }
}

#[test]
fn cache_result_no_default() {
    assert!(test_result_no_default(2).is_ok());
    assert!(test_result_no_default(4).is_ok());
    assert!(test_result_no_default(6).is_err());
    assert!(test_result_no_default(6).is_err());
    assert!(test_result_no_default(2).is_ok());
    assert!(test_result_no_default(4).is_ok());
    {
        let cache = RESULT_CACHE.lock().unwrap();
        assert_eq!(2, cache.cache_size());
        assert_eq!(2, cache.cache_hits().unwrap());
        assert_eq!(4, cache.cache_misses().unwrap());
    }
}

cached_control! {
    CONTROL_CACHE: UnboundCache<String, String> = UnboundCache::new();
    Key = { input.to_owned() };
    PostGet(cached_val) = return Ok(cached_val.clone());
    PostExec(body_result) = {
        #[allow(clippy::question_mark)]
        match body_result {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
    Set(set_value) = set_value.clone();
    Return(return_value) = {
        println!("{return_value}");
        Ok(return_value)
    };
    fn can_fail(input: &str) -> Result<String, String> = {
        let len = input.len();
        if len < 3 { Ok(format!("{input}-{len}")) }
        else { Err("too big".to_string()) }
    }
}

#[test]
fn test_can_fail() {
    assert_eq!(can_fail("ab"), Ok("ab-2".to_string()));
    assert_eq!(can_fail("abc"), Err("too big".to_string()));
    {
        let cache = CONTROL_CACHE.lock().unwrap();
        assert_eq!(2, cache.cache_misses().unwrap());
    }
    assert_eq!(can_fail("ab"), Ok("ab-2".to_string()));
    {
        let cache = CONTROL_CACHE.lock().unwrap();
        assert_eq!(1, cache.cache_hits().unwrap());
    }
}

cached_key! {
    SIZED_KEY_RESULT_CACHE: SizedCache<String, String> = SizedCache::with_size(2);
    Key = { format!("{a}/{b}") };
    fn slow_small_cache(a: &str, b: &str) -> String = {
        sleep(Duration::new(1, 0));
        format!("{a}:{b}")
    }
}

#[test]
/// This is a regression test to confirm that racing cache sets on a `SizedCache`
/// do not cause duplicates to exist in the internal `order`. See issue #7
fn test_racing_duplicate_keys_do_not_duplicate_sized_cache_ordering() {
    let a = thread::spawn(|| slow_small_cache("a", "b"));
    sleep(Duration::new(0, 500_000));
    let b = thread::spawn(|| slow_small_cache("a", "b"));
    a.join().unwrap();
    b.join().unwrap();
    // at this point, the cache should have a size of one since the keys are the same
    // and the internal `order` list should also have one item.
    // Since the method's cache has a capacity of 2, caching two more unique keys should
    // force the full eviction of the original values.
    slow_small_cache("c", "d");
    slow_small_cache("e", "f");
    slow_small_cache("g", "h");
}

// NoClone is not cloneable. So this also tests that the Result type
// itself does not have to be cloneable, just the type for the Ok
// value.
// Vec has Clone, but not Copy, to make sure Copy isn't required.
struct NoClone {}

#[cached(result = true)]
fn proc_cached_result(n: u32) -> Result<Vec<u32>, NoClone> {
    if n < 5 {
        Ok(vec![n])
    } else {
        Err(NoClone {})
    }
}

#[test]
fn test_proc_cached_result() {
    assert!(proc_cached_result(2).is_ok());
    assert!(proc_cached_result(4).is_ok());
    assert!(proc_cached_result(6).is_err());
    assert!(proc_cached_result(6).is_err());
    assert!(proc_cached_result(2).is_ok());
    assert!(proc_cached_result(4).is_ok());
    {
        let cache = PROC_CACHED_RESULT.lock().unwrap();
        assert_eq!(2, cache.cache_size());
        assert_eq!(2, cache.cache_hits().unwrap());
        assert_eq!(4, cache.cache_misses().unwrap());
    }
}

#[cached(option = true)]
fn proc_cached_option(n: u32) -> Option<Vec<u32>> {
    if n < 5 {
        Some(vec![n])
    } else {
        None
    }
}

#[test]
fn test_proc_cached_option() {
    assert!(proc_cached_option(2).is_some());
    assert!(proc_cached_option(4).is_some());
    assert!(proc_cached_option(1).is_some());
    assert!(proc_cached_option(6).is_none());
    assert!(proc_cached_option(6).is_none());
    assert!(proc_cached_option(2).is_some());
    assert!(proc_cached_option(1).is_some());
    assert!(proc_cached_option(4).is_some());
    {
        let cache = PROC_CACHED_OPTION.lock().unwrap();
        assert_eq!(3, cache.cache_size());
        assert_eq!(3, cache.cache_hits().unwrap());
        assert_eq!(5, cache.cache_misses().unwrap());
    }
}

cached_result! {
    RESULT_CACHE_RETARM: UnboundCache<u32, u32> = UnboundCache::new();
    fn test_result_missing_result_arm(n: u32) -> Result<u32, ()> = {
        Ok(n)
    }
}

cached_key_result! {
    RESULT_CACHE_KEY_RETARM: UnboundCache<u32, u32> = UnboundCache::new();
    Key = { n };
    fn test_result_key_missing_result_arm(n: u32) -> Result<u32, ()> = {
        Ok(n)
    }
}

#[cached(size = 1, time = 1)]
fn proc_timed_sized_sleeper(n: u64) -> u64 {
    sleep(Duration::new(1, 0));
    n
}

#[test]
fn test_proc_timed_sized_cache() {
    proc_timed_sized_sleeper(1);
    proc_timed_sized_sleeper(1);
    {
        let cache = PROC_TIMED_SIZED_SLEEPER.lock().unwrap();
        assert_eq!(1, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
    }
    // sleep to expire the one entry
    sleep(Duration::new(1, 0));
    proc_timed_sized_sleeper(1);
    {
        let cache = PROC_TIMED_SIZED_SLEEPER.lock().unwrap();
        assert_eq!(2, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
        assert_eq!(cache.key_order().collect::<Vec<_>>(), vec![&1]);
    }
    // sleep to expire the one entry
    sleep(Duration::new(1, 0));
    {
        let cache = PROC_TIMED_SIZED_SLEEPER.lock().unwrap();
        assert!(cache.key_order().next().is_none());
    }
    proc_timed_sized_sleeper(1);
    proc_timed_sized_sleeper(1);
    {
        let cache = PROC_TIMED_SIZED_SLEEPER.lock().unwrap();
        assert_eq!(3, cache.cache_misses().unwrap());
        assert_eq!(2, cache.cache_hits().unwrap());
        assert_eq!(cache.key_order().collect::<Vec<_>>(), vec![&1]);
    }
    // lru size is 1, so this new thing evicts the existing key
    proc_timed_sized_sleeper(2);
    {
        let cache = PROC_TIMED_SIZED_SLEEPER.lock().unwrap();
        assert_eq!(4, cache.cache_misses().unwrap());
        assert_eq!(2, cache.cache_hits().unwrap());
        assert_eq!(cache.key_order().collect::<Vec<_>>(), vec![&2]);
    }
}

#[cached(with_cached_flag = true)]
fn cached_return_flag(n: i32) -> cached::Return<i32> {
    cached::Return::new(n)
}

#[test]
fn test_cached_return_flag() {
    let r = cached_return_flag(1);
    assert!(!r.was_cached);
    assert_eq!(*r, 1);
    let r = cached_return_flag(1);
    assert!(r.was_cached);
    // derefs to inner
    assert_eq!(*r, 1);
    assert!(r.is_positive());
    {
        let cache = CACHED_RETURN_FLAG.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(1));
    }
}

#[cached(result = true, with_cached_flag = true)]
fn cached_return_flag_result(n: i32) -> Result<cached::Return<i32>, ()> {
    if n == 10 {
        return Err(());
    }
    Ok(cached::Return::new(n))
}

#[test]
fn test_cached_return_flag_result() {
    let r = cached_return_flag_result(1).unwrap();
    assert!(!r.was_cached);
    assert_eq!(*r, 1);
    let r = cached_return_flag_result(1).unwrap();
    assert!(r.was_cached);
    // derefs to inner
    assert_eq!(*r, 1);
    assert!(r.is_positive());

    let r = cached_return_flag_result(10);
    assert!(r.is_err());
    {
        let cache = CACHED_RETURN_FLAG_RESULT.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(2));
    }
}

#[cached(option = true, with_cached_flag = true)]
fn cached_return_flag_option(n: i32) -> Option<cached::Return<i32>> {
    if n == 10 {
        return None;
    }
    Some(cached::Return::new(n))
}

#[test]
fn test_cached_return_flag_option() {
    let r = cached_return_flag_option(1).unwrap();
    assert!(!r.was_cached);
    assert_eq!(*r, 1);
    let r = cached_return_flag_option(1).unwrap();
    assert!(r.was_cached);
    // derefs to inner
    assert_eq!(*r, 1);
    assert!(r.is_positive());

    let r = cached_return_flag_option(10);
    assert!(r.is_none());
    {
        let cache = CACHED_RETURN_FLAG_OPTION.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(2));
    }
}

/// should only cache the _first_ value returned for 1 second.
/// all arguments are ignored for subsequent calls until the
/// cache expires after a second.
#[once(time = 1)]
fn only_cached_once_per_second(s: String) -> Vec<String> {
    vec![s]
}

#[test]
fn test_only_cached_once_per_second() {
    let a = only_cached_once_per_second("a".to_string());
    let b = only_cached_once_per_second("b".to_string());
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_once_per_second("b".to_string());
    assert_eq!(vec!["b".to_string()], b);
}

#[cfg(feature = "async")]
#[once(time = 1)]
async fn only_cached_once_per_second_a(s: String) -> Vec<String> {
    vec![s]
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_only_cached_once_per_second_a() {
    let a = only_cached_once_per_second_a("a".to_string()).await;
    let b = only_cached_once_per_second_a("b".to_string()).await;
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_once_per_second_a("b".to_string()).await;
    assert_eq!(vec!["b".to_string()], b);
}

/// should only cache the _first_ `Ok` returned.
/// all arguments are ignored for subsequent calls.
#[once(result = true)]
fn only_cached_result_once(s: String, error: bool) -> std::result::Result<Vec<String>, u32> {
    if error {
        Err(1)
    } else {
        Ok(vec![s])
    }
}

#[test]
fn test_only_cached_result_once() {
    assert!(only_cached_result_once("z".to_string(), true).is_err());
    let a = only_cached_result_once("a".to_string(), false).unwrap();
    let b = only_cached_result_once("b".to_string(), false).unwrap();
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_result_once("b".to_string(), false).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "async")]
#[once(result = true)]
async fn only_cached_result_once_a(
    s: String,
    error: bool,
) -> std::result::Result<Vec<String>, u32> {
    if error {
        Err(1)
    } else {
        Ok(vec![s])
    }
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_only_cached_result_once_a() {
    assert!(only_cached_result_once_a("z".to_string(), true)
        .await
        .is_err());
    let a = only_cached_result_once_a("a".to_string(), false)
        .await
        .unwrap();
    let b = only_cached_result_once_a("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_result_once_a("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
}

/// should only cache the _first_ `Ok` returned for 1 second.
/// all arguments are ignored for subsequent calls until the
/// cache expires after a second.
#[once(result = true, time = 1)]
fn only_cached_result_once_per_second(
    s: String,
    error: bool,
) -> std::result::Result<Vec<String>, u32> {
    if error {
        Err(1)
    } else {
        Ok(vec![s])
    }
}

#[test]
fn test_only_cached_result_once_per_second() {
    assert!(only_cached_result_once_per_second("z".to_string(), true).is_err());
    let a = only_cached_result_once_per_second("a".to_string(), false).unwrap();
    let b = only_cached_result_once_per_second("b".to_string(), false).unwrap();
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_result_once_per_second("b".to_string(), false).unwrap();
    assert_eq!(vec!["b".to_string()], b);
}

#[cfg(feature = "async")]
#[once(result = true, time = 1)]
async fn only_cached_result_once_per_second_a(
    s: String,
    error: bool,
) -> std::result::Result<Vec<String>, u32> {
    if error {
        Err(1)
    } else {
        Ok(vec![s])
    }
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_only_cached_result_once_per_second_a() {
    assert!(only_cached_result_once_per_second_a("z".to_string(), true)
        .await
        .is_err());
    let a = only_cached_result_once_per_second_a("a".to_string(), false)
        .await
        .unwrap();
    let b = only_cached_result_once_per_second_a("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_result_once_per_second_a("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(vec!["b".to_string()], b);
}

/// should only cache the _first_ `Some` returned .
/// all arguments are ignored for subsequent calls
#[once(option = true)]
fn only_cached_option_once(s: String, none: bool) -> Option<Vec<String>> {
    if none {
        None
    } else {
        Some(vec![s])
    }
}

#[test]
fn test_only_cached_option_once() {
    assert!(only_cached_option_once("z".to_string(), true).is_none());
    let a = only_cached_option_once("a".to_string(), false).unwrap();
    let b = only_cached_option_once("b".to_string(), false).unwrap();
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_option_once("b".to_string(), false).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "async")]
#[once(option = true)]
async fn only_cached_option_once_a(s: String, none: bool) -> Option<Vec<String>> {
    if none {
        None
    } else {
        Some(vec![s])
    }
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_only_cached_option_once_a() {
    assert!(only_cached_option_once_a("z".to_string(), true)
        .await
        .is_none());
    let a = only_cached_option_once_a("a".to_string(), false)
        .await
        .unwrap();
    let b = only_cached_option_once_a("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_option_once_a("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
}

/// should only cache the _first_ `Some` returned for 1 second.
/// all arguments are ignored for subsequent calls until the
/// cache expires after a second.
#[once(option = true, time = 1)]
fn only_cached_option_once_per_second(s: String, none: bool) -> Option<Vec<String>> {
    if none {
        None
    } else {
        Some(vec![s])
    }
}

#[test]
fn test_only_cached_option_once_per_second() {
    assert!(only_cached_option_once_per_second("z".to_string(), true).is_none());
    let a = only_cached_option_once_per_second("a".to_string(), false).unwrap();
    let b = only_cached_option_once_per_second("b".to_string(), false).unwrap();
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_option_once_per_second("b".to_string(), false).unwrap();
    assert_eq!(vec!["b".to_string()], b);
}

#[cfg(feature = "async")]
#[once(option = true, time = 1)]
async fn only_cached_option_once_per_second_a(s: String, none: bool) -> Option<Vec<String>> {
    if none {
        None
    } else {
        Some(vec![s])
    }
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_only_cached_option_once_per_second_a() {
    assert!(only_cached_option_once_per_second_a("z".to_string(), true)
        .await
        .is_none());
    let a = only_cached_option_once_per_second_a("a".to_string(), false)
        .await
        .unwrap();
    let b = only_cached_option_once_per_second_a("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
    sleep(Duration::new(1, 0));
    let b = only_cached_option_once_per_second_a("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(vec!["b".to_string()], b);
}

/// should only cache the _first_ value returned for 2 seconds.
/// all arguments are ignored for subsequent calls until the
/// cache expires after a second.
/// when multiple un-cached tasks are running concurrently, only
/// _one_ call will be "executed" and all others will be synchronized
/// to return the cached result of the one call instead of all
/// concurrently un-cached tasks executing and writing concurrently.
#[cfg(feature = "async")]
#[once(time = 2, sync_writes)]
async fn only_cached_once_per_second_sync_writes(s: String) -> Vec<String> {
    vec![s]
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_only_cached_once_per_second_sync_writes() {
    let a = tokio::spawn(only_cached_once_per_second_sync_writes("a".to_string()));
    tokio::time::sleep(Duration::new(1, 0)).await;
    let b = tokio::spawn(only_cached_once_per_second_sync_writes("b".to_string()));
    assert_eq!(a.await.unwrap(), b.await.unwrap());
}

#[cached(time = 2, sync_writes = "default", key = "u32", convert = "{ 1 }")]
fn cached_sync_writes(s: String) -> Vec<String> {
    vec![s]
}

#[test]
fn test_cached_sync_writes() {
    let a = std::thread::spawn(|| cached_sync_writes("a".to_string()));
    sleep(Duration::new(1, 0));
    let b = std::thread::spawn(|| cached_sync_writes("b".to_string()));
    let c = std::thread::spawn(|| cached_sync_writes("c".to_string()));
    let a = a.join().unwrap();
    let b = b.join().unwrap();
    let c = c.join().unwrap();
    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[cfg(feature = "async")]
#[cached(time = 2, sync_writes = "default", key = "u32", convert = "{ 1 }")]
async fn cached_sync_writes_a(s: String) -> Vec<String> {
    vec![s]
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_cached_sync_writes_a() {
    let a = tokio::spawn(cached_sync_writes_a("a".to_string()));
    tokio::time::sleep(Duration::new(1, 0)).await;
    let b = tokio::spawn(cached_sync_writes_a("b".to_string()));
    let c = tokio::spawn(cached_sync_writes_a("c".to_string()));
    let a = a.await.unwrap();
    assert_eq!(a, b.await.unwrap());
    assert_eq!(a, c.await.unwrap());
}

#[cached(time = 2, sync_writes = "by_key", key = "u32", convert = "{ 1 }")]
fn cached_sync_writes_by_key(s: String) -> Vec<String> {
    sleep(Duration::new(1, 0));
    vec![s]
}

#[test]
fn test_cached_sync_writes_by_key() {
    let a = std::thread::spawn(|| cached_sync_writes_by_key("a".to_string()));
    let b = std::thread::spawn(|| cached_sync_writes_by_key("b".to_string()));
    let c = std::thread::spawn(|| cached_sync_writes_by_key("c".to_string()));
    let start = Instant::now();
    let _ = a.join().unwrap();
    let _ = b.join().unwrap();
    let _ = c.join().unwrap();
    assert!(start.elapsed() < Duration::from_secs(2));
}

#[cfg(feature = "async")]
#[cached(
    time = 5,
    sync_writes = "by_key",
    key = "String",
    convert = r#"{ format!("{}", s) }"#
)]
async fn cached_sync_writes_by_key_a(s: String) -> Vec<String> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    vec![s]
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_cached_sync_writes_by_key_a() {
    let a = tokio::spawn(cached_sync_writes_by_key_a("a".to_string()));
    let b = tokio::spawn(cached_sync_writes_by_key_a("b".to_string()));
    let c = tokio::spawn(cached_sync_writes_by_key_a("c".to_string()));
    let start = Instant::now();
    a.await.unwrap();
    b.await.unwrap();
    c.await.unwrap();
    assert!(start.elapsed() < Duration::from_secs(2));
}

#[cfg(feature = "async")]
#[once(sync_writes = true)]
async fn once_sync_writes_a(s: &tokio::sync::Mutex<String>) -> String {
    let mut guard = s.lock().await;
    let results: String = (*guard).clone().to_string();
    *guard = "consumed".to_string();
    results.to_string()
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_once_sync_writes_a() {
    let a_mutex = tokio::sync::Mutex::new("a".to_string());
    let b_mutex = tokio::sync::Mutex::new("b".to_string());
    let fut_a = once_sync_writes_a(&a_mutex);
    let fut_b = once_sync_writes_a(&b_mutex);
    let a = fut_a.await;
    let b = fut_b.await;
    assert_eq!(a, b);
    assert_eq!("a", a);

    //check if cache function is executed for a
    assert_eq!("consumed", a_mutex.lock().await.to_string());
    //check if cache inner is not executed for b (not executed second time)
    assert_eq!("b", b_mutex.lock().await.to_string());
}

#[cached(size = 2)]
fn cached_smartstring(s: smartstring::alias::String) -> smartstring::alias::String {
    if s == "very stringy" {
        smartstring::alias::String::from("equal")
    } else {
        smartstring::alias::String::from("not equal")
    }
}

#[test]
fn test_cached_smartstring() {
    let mut string = smartstring::alias::String::new();
    string.push_str("very stringy");
    assert_eq!("equal", cached_smartstring(string.clone()));
    {
        let cache = CACHED_SMARTSTRING.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    assert_eq!("equal", cached_smartstring(string.clone()));
    {
        let cache = CACHED_SMARTSTRING.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    let string = smartstring::alias::String::from("also stringy");
    assert_eq!("not equal", cached_smartstring(string));
    {
        let cache = CACHED_SMARTSTRING.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(2));
    }
}

#[cached(
    size = 2,
    key = "smartstring::alias::String",
    convert = r#"{ smartstring::alias::String::from(s) }"#
)]
fn cached_smartstring_from_str(s: &str) -> bool {
    s == "true"
}

#[test]
fn test_cached_smartstring_from_str() {
    assert!(cached_smartstring_from_str("true"));
    {
        let cache = CACHED_SMARTSTRING_FROM_STR.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    assert!(cached_smartstring_from_str("true"));
    {
        let cache = CACHED_SMARTSTRING_FROM_STR.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    assert!(!cached_smartstring_from_str("false"));
    {
        let cache = CACHED_SMARTSTRING_FROM_STR.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(2));
    }
}

#[cached(
    time = 1,
    time_refresh = true,
    key = "String",
    convert = r#"{ String::from(s) }"#
)]
fn cached_timed_refresh(s: &str) -> bool {
    s == "true"
}

#[test]
fn test_cached_timed_refresh() {
    assert!(cached_timed_refresh("true"));
    {
        let cache = CACHED_TIMED_REFRESH.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    assert!(cached_timed_refresh("true"));
    {
        let cache = CACHED_TIMED_REFRESH.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_refresh("true"));
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_refresh("true"));
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_refresh("true"));
    {
        let cache = CACHED_TIMED_REFRESH.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(4));
        assert_eq!(cache.cache_misses(), Some(1));
    }
}

#[cached(
    size = 2,
    time = 1,
    time_refresh = true,
    key = "String",
    convert = r#"{ String::from(s) }"#
)]
fn cached_timed_sized_refresh(s: &str) -> bool {
    s == "true"
}

#[test]
fn test_cached_timed_sized_refresh() {
    assert!(cached_timed_sized_refresh("true"));
    {
        let cache = CACHED_TIMED_SIZED_REFRESH.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    assert!(cached_timed_sized_refresh("true"));
    {
        let cache = CACHED_TIMED_SIZED_REFRESH.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_sized_refresh("true"));
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_sized_refresh("true"));
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_sized_refresh("true"));
    {
        let cache = CACHED_TIMED_SIZED_REFRESH.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(4));
        assert_eq!(cache.cache_misses(), Some(1));
    }
}

#[cached(
    size = 2,
    time = 1,
    time_refresh = true,
    key = "String",
    convert = r#"{ String::from(s) }"#
)]
fn cached_timed_sized_refresh_prime(s: &str) -> bool {
    s == "true"
}

#[test]
fn test_cached_timed_sized_refresh_prime() {
    assert!(cached_timed_sized_refresh_prime("true"));
    {
        let cache = CACHED_TIMED_SIZED_REFRESH_PRIME.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(1));
    }
    assert!(cached_timed_sized_refresh_prime("true"));
    {
        let cache = CACHED_TIMED_SIZED_REFRESH_PRIME.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_sized_refresh_prime_prime_cache("true"));
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_sized_refresh_prime_prime_cache("true"));
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_sized_refresh_prime_prime_cache("true"));

    // stats unchanged (other than this new hit) since we kept priming
    assert!(cached_timed_sized_refresh_prime("true"));
    {
        let cache = CACHED_TIMED_SIZED_REFRESH_PRIME.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(2));
        assert_eq!(cache.cache_misses(), Some(1));
    }
}

#[cached(size = 2, time = 1, key = "String", convert = r#"{ String::from(s) }"#)]
fn cached_timed_sized_prime(s: &str) -> bool {
    s == "true"
}

#[test]
fn test_cached_timed_sized_prime() {
    assert!(cached_timed_sized_prime("true"));
    {
        let cache = CACHED_TIMED_SIZED_PRIME.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(1));
    }
    assert!(cached_timed_sized_prime("true"));
    {
        let cache = CACHED_TIMED_SIZED_PRIME.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_sized_prime_prime_cache("true"));
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_sized_prime_prime_cache("true"));
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(cached_timed_sized_prime_prime_cache("true"));

    // stats unchanged (other than this new hit) since we kept priming
    assert!(cached_timed_sized_prime("true"));
    {
        let mut cache = CACHED_TIMED_SIZED_PRIME.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(2));
        assert_eq!(cache.cache_misses(), Some(1));
        assert!(cache.cache_size() > 0);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        cache.flush();
        assert_eq!(cache.cache_size(), 0);
    }
}

#[once]
fn once_for_priming() -> bool {
    true
}

#[test]
fn test_once_for_priming() {
    assert!(once_for_priming_prime_cache());
    {
        let cache = ONCE_FOR_PRIMING.read().unwrap();
        assert!(cache.is_some());
    }
}

#[cached]
fn mutable_args(mut a: i32, mut b: i32) -> (i32, i32) {
    a += 1;
    b += 1;
    (a, b)
}

#[test]
fn test_mutable_args() {
    assert_eq!((2, 2), mutable_args(1, 1));
    assert_eq!((2, 2), mutable_args(1, 1));
}

#[cached]
fn mutable_args_str(mut a: String) -> String {
    a.push_str("-ok");
    a
}

#[test]
fn test_mutable_args_str() {
    assert_eq!("a-ok", mutable_args_str(String::from("a")));
    assert_eq!("a-ok", mutable_args_str(String::from("a")));
}

#[once]
fn mutable_args_once(mut a: i32, mut b: i32) -> (i32, i32) {
    a += 1;
    b += 1;
    (a, b)
}

#[test]
fn test_mutable_args_once() {
    assert_eq!((2, 2), mutable_args_once(1, 1));
    assert_eq!((2, 2), mutable_args_once(1, 1));
    assert_eq!((2, 2), mutable_args_once(5, 6));
}

#[cfg(feature = "disk_store")]
mod disk_tests {
    use super::*;
    use cached::proc_macro::io_cached;
    use cached::DiskCache;
    use thiserror::Error;

    #[derive(Error, Debug, PartialEq, Clone)]
    enum TestError {
        #[error("error with disk cache `{0}`")]
        DiskError(String),
        #[error("count `{0}`")]
        Count(u32),
    }

    #[io_cached(
        disk = true,
        time = 1,
        map_error = r##"|e| TestError::DiskError(format!("{:?}", e))"##
    )]
    fn cached_disk(n: u32) -> Result<u32, TestError> {
        if n < 5 {
            Ok(n)
        } else {
            Err(TestError::Count(n))
        }
    }

    #[test]
    fn test_cached_disk() {
        assert_eq!(cached_disk(1), Ok(1));
        assert_eq!(cached_disk(1), Ok(1));
        assert_eq!(cached_disk(5), Err(TestError::Count(5)));
        assert_eq!(cached_disk(6), Err(TestError::Count(6)));
    }

    #[io_cached(
        disk = true,
        time = 1,
        with_cached_flag = true,
        map_error = r##"|e| TestError::DiskError(format!("{:?}", e))"##
    )]
    fn cached_disk_cached_flag(n: u32) -> Result<cached::Return<u32>, TestError> {
        if n < 5 {
            Ok(cached::Return::new(n))
        } else {
            Err(TestError::Count(n))
        }
    }

    #[test]
    fn test_cached_disk_cached_flag() {
        assert!(!cached_disk_cached_flag(1).unwrap().was_cached);
        assert!(cached_disk_cached_flag(1).unwrap().was_cached);
        assert!(cached_disk_cached_flag(5).is_err());
        assert!(cached_disk_cached_flag(6).is_err());
    }

    #[io_cached(
        map_error = r##"|e| TestError::DiskError(format!("{:?}", e))"##,
        ty = "cached::DiskCache<u32, u32>",
        create = r##" { DiskCache::new("cached_disk_cache_create").set_lifespan(Duration::from_secs(1)).set_refresh(true).build().expect("error building disk cache") } "##
    )]
    fn cached_disk_cache_create(n: u32) -> Result<u32, TestError> {
        if n < 5 {
            Ok(n)
        } else {
            Err(TestError::Count(n))
        }
    }

    #[test]
    fn test_cached_disk_cache_create() {
        assert_eq!(cached_disk_cache_create(1), Ok(1));
        assert_eq!(cached_disk_cache_create(1), Ok(1));
        assert_eq!(cached_disk_cache_create(5), Err(TestError::Count(5)));
        assert_eq!(cached_disk_cache_create(6), Err(TestError::Count(6)));
    }

    /// Just calling the macro with connection_config to test it doesn't break with an expected string
    /// for connection_config.
    /// There are no simple tests to test this here
    #[io_cached(
        disk = true,
        map_error = r##"|e| TestError::DiskError(format!("{:?}", e))"##,
        connection_config = r##"sled::Config::new().flush_every_ms(None)"##
    )]
    fn cached_disk_connection_config(n: u32) -> Result<u32, TestError> {
        if n < 5 {
            Ok(n)
        } else {
            Err(TestError::Count(n))
        }
    }

    /// Just calling the macro with sync_to_disk_on_cache_change to test it doesn't break with an expected value
    /// There are no simple tests to test this here
    #[io_cached(
        disk = true,
        map_error = r##"|e| TestError::DiskError(format!("{:?}", e))"##,
        sync_to_disk_on_cache_change = true
    )]
    fn cached_disk_sync_to_disk_on_cache_change(n: u32) -> Result<u32, TestError> {
        if n < 5 {
            Ok(n)
        } else {
            Err(TestError::Count(n))
        }
    }

    #[cfg(feature = "async")]
    mod async_test {
        use super::*;

        #[io_cached(
            disk = true,
            map_error = r##"|e| TestError::DiskError(format!("{:?}", e))"##
        )]
        async fn async_cached_disk(n: u32) -> Result<u32, TestError> {
            if n < 5 {
                Ok(n)
            } else {
                Err(TestError::Count(n))
            }
        }

        #[tokio::test]
        async fn test_async_cached_disk() {
            assert_eq!(async_cached_disk(1).await, Ok(1));
            assert_eq!(async_cached_disk(1).await, Ok(1));
            assert_eq!(async_cached_disk(5).await, Err(TestError::Count(5)));
            assert_eq!(async_cached_disk(6).await, Err(TestError::Count(6)));
        }
    }
}

#[cfg(feature = "redis_store")]
mod redis_tests {
    use super::*;
    use cached::proc_macro::io_cached;
    use cached::RedisCache;
    use thiserror::Error;

    #[derive(Error, Debug, PartialEq, Clone)]
    enum TestError {
        #[error("error with redis cache `{0}`")]
        RedisError(String),
        #[error("count `{0}`")]
        Count(u32),
    }

    #[io_cached(
        redis = true,
        time = 1,
        cache_prefix_block = "{ \"__cached_redis_proc_macro_test_fn_cached_redis\" }",
        map_error = r##"|e| TestError::RedisError(format!("{:?}", e))"##
    )]
    fn cached_redis(n: u32) -> Result<u32, TestError> {
        if n < 5 {
            Ok(n)
        } else {
            Err(TestError::Count(n))
        }
    }

    #[test]
    fn test_cached_redis() {
        assert_eq!(cached_redis(1), Ok(1));
        assert_eq!(cached_redis(1), Ok(1));
        assert_eq!(cached_redis(5), Err(TestError::Count(5)));
        assert_eq!(cached_redis(6), Err(TestError::Count(6)));
    }

    #[io_cached(
        redis = true,
        time = 1,
        with_cached_flag = true,
        map_error = r##"|e| TestError::RedisError(format!("{:?}", e))"##
    )]
    fn cached_redis_cached_flag(n: u32) -> Result<cached::Return<u32>, TestError> {
        if n < 5 {
            Ok(cached::Return::new(n))
        } else {
            Err(TestError::Count(n))
        }
    }

    #[test]
    fn test_cached_redis_cached_flag() {
        assert!(!cached_redis_cached_flag(1).unwrap().was_cached);
        assert!(cached_redis_cached_flag(1).unwrap().was_cached);
        assert!(cached_redis_cached_flag(5).is_err());
        assert!(cached_redis_cached_flag(6).is_err());
    }

    #[io_cached(
        map_error = r##"|e| TestError::RedisError(format!("{:?}", e))"##,
        ty = "cached::RedisCache<u32, u32>",
        create = r##" { RedisCache::new("cache_redis_test_cache_create", Duration::from_secs(1)).set_refresh(true).build().expect("error building redis cache") } "##
    )]
    fn cached_redis_cache_create(n: u32) -> Result<u32, TestError> {
        if n < 5 {
            Ok(n)
        } else {
            Err(TestError::Count(n))
        }
    }

    #[test]
    fn test_cached_redis_cache_create() {
        assert_eq!(cached_redis_cache_create(1), Ok(1));
        assert_eq!(cached_redis_cache_create(1), Ok(1));
        assert_eq!(cached_redis_cache_create(5), Err(TestError::Count(5)));
        assert_eq!(cached_redis_cache_create(6), Err(TestError::Count(6)));
    }

    #[cfg(any(feature = "redis_async_std", feature = "redis_tokio"))]
    mod async_redis_tests {
        use super::*;

        #[io_cached(
            redis = true,
            time = 1,
            cache_prefix_block = "{ \"__cached_redis_proc_macro_test_fn_async_cached_redis\" }",
            map_error = r##"|e| TestError::RedisError(format!("{:?}", e))"##
        )]
        async fn async_cached_redis(n: u32) -> Result<u32, TestError> {
            if n < 5 {
                Ok(n)
            } else {
                Err(TestError::Count(n))
            }
        }

        #[tokio::test]
        async fn test_async_cached_redis() {
            assert_eq!(async_cached_redis(1).await, Ok(1));
            assert_eq!(async_cached_redis(1).await, Ok(1));
            assert_eq!(async_cached_redis(5).await, Err(TestError::Count(5)));
            assert_eq!(async_cached_redis(6).await, Err(TestError::Count(6)));
        }

        #[io_cached(
            redis = true,
            time = 1,
            with_cached_flag = true,
            map_error = r##"|e| TestError::RedisError(format!("{:?}", e))"##
        )]
        async fn async_cached_redis_cached_flag(n: u32) -> Result<cached::Return<u32>, TestError> {
            if n < 5 {
                Ok(cached::Return::new(n))
            } else {
                Err(TestError::Count(n))
            }
        }

        #[tokio::test]
        async fn test_async_cached_redis_cached_flag() {
            assert!(!async_cached_redis_cached_flag(1).await.unwrap().was_cached);
            assert!(async_cached_redis_cached_flag(1).await.unwrap().was_cached,);
            assert!(async_cached_redis_cached_flag(5).await.is_err());
            assert!(async_cached_redis_cached_flag(6).await.is_err());
        }

        use cached::AsyncRedisCache;
        #[io_cached(
            map_error = r##"|e| TestError::RedisError(format!("{:?}", e))"##,
            ty = "cached::AsyncRedisCache<u32, u32>",
            create = r##" { AsyncRedisCache::new("async_cached_redis_test_cache_create", Duration::from_secs(1)).set_refresh(true).build().await.expect("error building async redis cache") } "##
        )]
        async fn async_cached_redis_cache_create(n: u32) -> Result<u32, TestError> {
            if n < 5 {
                Ok(n)
            } else {
                Err(TestError::Count(n))
            }
        }

        #[tokio::test]
        async fn test_async_cached_redis_cache_create() {
            assert_eq!(async_cached_redis_cache_create(1).await, Ok(1));
            assert_eq!(async_cached_redis_cache_create(1).await, Ok(1));
            assert_eq!(
                async_cached_redis_cache_create(5).await,
                Err(TestError::Count(5))
            );
            assert_eq!(
                async_cached_redis_cache_create(6).await,
                Err(TestError::Count(6))
            );
        }
    }
}

#[derive(Clone)]
pub struct NewsArticle {
    slug: String,
    is_expired: bool,
}

impl CanExpire for NewsArticle {
    fn is_expired(&self) -> bool {
        self.is_expired
    }
}

const EXPIRED_SLUG: &str = "expired_slug";
const UNEXPIRED_SLUG: &str = "unexpired_slug";

#[cached(
    ty = "ExpiringValueCache<String, NewsArticle>",
    create = "{ ExpiringValueCache::with_size(3) }",
    result = true
)]
fn fetch_article(slug: String) -> Result<NewsArticle, ()> {
    match slug.as_str() {
        EXPIRED_SLUG => Ok(NewsArticle {
            slug: String::from(EXPIRED_SLUG),
            is_expired: true,
        }),
        UNEXPIRED_SLUG => Ok(NewsArticle {
            slug: String::from(UNEXPIRED_SLUG),
            is_expired: false,
        }),
        _ => Err(()),
    }
}

#[test]
#[serial(ExpiringCacheTest)]
fn test_expiring_value_expired_article_returned_with_miss() {
    {
        let mut cache = FETCH_ARTICLE.lock().unwrap();
        cache.cache_reset();
        cache.cache_reset_metrics();
    }
    let expired_article = fetch_article(EXPIRED_SLUG.to_string());

    assert!(expired_article.is_ok());
    assert_eq!(EXPIRED_SLUG, expired_article.unwrap().slug.as_str());

    // The article was fetched due to a cache miss and the result cached.
    {
        let cache = FETCH_ARTICLE.lock().unwrap();
        assert_eq!(1, cache.cache_size());
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    let _ = fetch_article(EXPIRED_SLUG.to_string());

    // The article was fetched again as it had expired.
    {
        let cache = FETCH_ARTICLE.lock().unwrap();
        assert_eq!(1, cache.cache_size());
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(2));
    }
}

#[test]
#[serial(ExpiringCacheTest)]
fn test_expiring_value_unexpired_article_returned_with_hit() {
    {
        let mut cache = FETCH_ARTICLE.lock().unwrap();
        cache.cache_reset();
        cache.cache_reset_metrics();
    }
    let unexpired_article = fetch_article(UNEXPIRED_SLUG.to_string());

    assert!(unexpired_article.is_ok());
    assert_eq!(UNEXPIRED_SLUG, unexpired_article.unwrap().slug.as_str());

    // The article was fetched due to a cache miss and the result cached.
    {
        let cache = FETCH_ARTICLE.lock().unwrap();
        assert_eq!(1, cache.cache_size());
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    let cached_article = fetch_article(UNEXPIRED_SLUG.to_string());
    assert!(cached_article.is_ok());
    assert_eq!(UNEXPIRED_SLUG, cached_article.unwrap().slug.as_str());

    // The article was not fetched but returned as a hit from the cache.
    {
        let cache = FETCH_ARTICLE.lock().unwrap();
        assert_eq!(1, cache.cache_size());
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(1));
    }
}

#[cached::proc_macro::cached(result = true, time = 1, result_fallback = true)]
fn always_failing() -> Result<String, ()> {
    Err(())
}

#[test]
fn test_result_fallback() {
    assert!(always_failing().is_err());
    {
        let cache = ALWAYS_FAILING.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(0));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    // Pretend it succeeded once
    ALWAYS_FAILING
        .lock()
        .unwrap()
        .cache_set((), "abc".to_string());
    assert_eq!(always_failing(), Ok("abc".to_string()));
    {
        let cache = ALWAYS_FAILING.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(1));
    }

    std::thread::sleep(std::time::Duration::from_millis(2000));

    // Even though the cache should've expired, the `result_fallback` flag means it refreshes the cache with the last valid result
    assert_eq!(always_failing(), Ok("abc".to_string()));
    {
        let cache = ALWAYS_FAILING.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(1));
        assert_eq!(cache.cache_misses(), Some(2));
    }

    assert_eq!(always_failing(), Ok("abc".to_string()));
    {
        let cache = ALWAYS_FAILING.lock().unwrap();
        assert_eq!(cache.cache_hits(), Some(2));
        assert_eq!(cache.cache_misses(), Some(2));
    }
}
