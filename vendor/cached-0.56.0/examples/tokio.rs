use cached::proc_macro::cached;
use std::time::Duration;
use tokio::time::sleep;

async fn sleep_secs(secs: u64) {
    sleep(Duration::from_secs(secs)).await;
}

#[cached]
async fn cached_sleep_secs(secs: u64) {
    sleep(Duration::from_secs(secs)).await;
}

#[cached(result = true, with_cached_flag = true)]
async fn cached_was_cached(count: u32) -> Result<cached::Return<String>, ()> {
    Ok(cached::Return::new(
        (0..count).map(|_| "a").collect::<Vec<_>>().join(""),
    ))
}

#[tokio::main]
async fn main() {
    println!("sleeping for 4 seconds");
    sleep_secs(4).await;
    println!("first cached sleeping for 4 seconds");
    cached_sleep_secs(4).await;
    println!("second cached sleeping for 4 seconds");
    cached_sleep_secs(4).await;

    let a = cached_was_cached(4).await.unwrap();
    assert_eq!(a.to_uppercase(), "AAAA");
    assert!(!a.was_cached);
    let a = cached_was_cached(4).await.unwrap();
    assert!(a.was_cached);

    println!("done!");
}
