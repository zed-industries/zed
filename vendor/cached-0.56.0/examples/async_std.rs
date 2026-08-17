use async_std::task::sleep;
use cached::proc_macro::cached;
use cached::proc_macro::once;
use std::time::Duration;

async fn sleep_secs(secs: u64) {
    sleep(Duration::from_secs(secs)).await;
}

/// should only sleep the first time it's called
#[cached]
async fn cached_sleep_secs(secs: u64) {
    sleep(Duration::from_secs(secs)).await;
}

/// should only cache the result for a second, and only when
/// the result is `Ok`
#[cached(time = 1, key = "bool", convert = r#"{ true }"#, result = true)]
async fn only_cached_a_second(
    s: String,
) -> std::result::Result<Vec<String>, &'static dyn std::error::Error> {
    Ok(vec![s])
}

/// should only cache the _first_ `Ok` returned.
/// all arguments are ignored for subsequent calls.
#[once(result = true)]
async fn only_cached_result_once(s: String, error: bool) -> std::result::Result<Vec<String>, u32> {
    if error {
        Err(1)
    } else {
        Ok(vec![s])
    }
}

/// should only cache the _first_ `Ok` returned for 1 second.
/// all arguments are ignored for subsequent calls until the
/// cache expires after a second.
#[once(result = true, time = 1)]
async fn only_cached_result_once_per_second(
    s: String,
    error: bool,
) -> std::result::Result<Vec<String>, u32> {
    if error {
        Err(1)
    } else {
        Ok(vec![s])
    }
}

/// should only cache the _first_ `Some` returned .
/// all arguments are ignored for subsequent calls
#[once(option = true)]
async fn only_cached_option_once(s: String, none: bool) -> Option<Vec<String>> {
    if none {
        None
    } else {
        Some(vec![s])
    }
}

/// should only cache the _first_ `Some` returned for 1 second.
/// all arguments are ignored for subsequent calls until the
/// cache expires after a second.
#[once(option = true, time = 1)]
async fn only_cached_option_once_per_second(s: String, none: bool) -> Option<Vec<String>> {
    if none {
        None
    } else {
        Some(vec![s])
    }
}

/// should only cache the _first_ value returned for 1 second.
/// all arguments are ignored for subsequent calls until the
/// cache expires after a second.
#[once(time = 1)]
async fn only_cached_once_per_second(s: String) -> Vec<String> {
    vec![s]
}

/// should only cache the _first_ value returned for 2 seconds.
/// all arguments are ignored for subsequent calls until the
/// cache expires after a second.
/// when multiple un-cached tasks are running concurrently, only
/// _one_ call will be "executed" and all others will be synchronized
/// to return the cached result of the one call instead of all
/// concurrently un-cached tasks executing and writing concurrently.
#[once(time = 2, sync_writes = true)]
async fn only_cached_once_per_second_sync_writes(s: String) -> Vec<String> {
    vec![s]
}

#[async_std::main]
async fn main() {
    let a = only_cached_a_second("a".to_string()).await.unwrap();
    let b = only_cached_a_second("b".to_string()).await.unwrap();
    assert_eq!(a, b);
    sleep_secs(1).await;
    let b = only_cached_a_second("b".to_string()).await.unwrap();
    assert_ne!(a, b);

    println!("cached sleeping for 1 seconds");
    cached_sleep_secs(1).await;
    println!("cached sleeping for 1 seconds");
    cached_sleep_secs(1).await;

    println!("cached result once");
    assert!(only_cached_result_once("z".to_string(), true)
        .await
        .is_err());
    let a = only_cached_result_once("a".to_string(), false)
        .await
        .unwrap();
    let b = only_cached_result_once("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
    sleep_secs(1).await;
    let b = only_cached_result_once("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);

    println!("cached result once per second");
    assert!(only_cached_result_once_per_second("z".to_string(), true)
        .await
        .is_err());
    let a = only_cached_result_once_per_second("a".to_string(), false)
        .await
        .unwrap();
    let b = only_cached_result_once_per_second("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
    sleep_secs(1).await;
    let b = only_cached_result_once_per_second("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(vec!["b".to_string()], b);

    println!("cached option once");
    assert!(only_cached_option_once("z".to_string(), true)
        .await
        .is_none());
    let a = only_cached_option_once("a".to_string(), false)
        .await
        .unwrap();
    let b = only_cached_option_once("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
    sleep_secs(1).await;
    let b = only_cached_option_once("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);

    println!("cached option once per second");
    assert!(only_cached_option_once_per_second("z".to_string(), true)
        .await
        .is_none());
    let a = only_cached_option_once_per_second("a".to_string(), false)
        .await
        .unwrap();
    let b = only_cached_option_once_per_second("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(a, b);
    sleep_secs(1).await;
    let b = only_cached_option_once_per_second("b".to_string(), false)
        .await
        .unwrap();
    assert_eq!(vec!["b".to_string()], b);

    println!("cached once per second");
    let a = only_cached_once_per_second("a".to_string()).await;
    let b = only_cached_once_per_second("b".to_string()).await;
    assert_eq!(a, b);
    sleep_secs(1).await;
    let b = only_cached_once_per_second("b".to_string()).await;
    assert_eq!(vec!["b".to_string()], b);

    println!("cached once per second synchronized writes");
    let a = async_std::task::spawn(only_cached_once_per_second_sync_writes("a".to_string()));
    sleep_secs(1).await;
    let b = async_std::task::spawn(only_cached_once_per_second_sync_writes("b".to_string()));
    assert_eq!(a.await, b.await);

    println!("done!");
}
