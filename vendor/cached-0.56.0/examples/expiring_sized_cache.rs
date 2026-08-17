use cached::stores::ExpiringSizedCache;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use web_time::Instant;

#[tokio::main]
async fn main() {
    let mut cache = ExpiringSizedCache::new(Duration::from_millis(20_000));
    cache.size_limit(100);

    let cache = Arc::new(RwLock::new(cache));

    let write_cache = cache.clone();
    let write_handle = tokio::spawn(async move {
        for _ in 0..10 {
            {
                let mut cache = write_cache.write().await;
                cache
                    .insert("A".to_string(), "A".to_string())
                    .expect("write failure");
                println!("[expiring_sized] wrote to cache");
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    let mut read_handles = vec![];
    for i in 0..5 {
        let reader = i + 1;
        let read_cache = cache.clone();
        let read_handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let start = Instant::now();
            let mut count = 0;
            while Instant::now().duration_since(start) < Duration::from_millis(5_000) {
                let cache = read_cache.read().await;
                assert_eq!(cache.get_borrowed("A"), Some(&"A".to_string()));
                count += 1;
                if count % 1_000_000 == 0 {
                    println!("[expiring_sized] read 1M times in reader {}", reader);
                }
            }
        });
        read_handles.push(read_handle);
    }

    write_handle.await.expect("error in write loop");
    for (i, h) in read_handles.into_iter().enumerate() {
        h.await
            .map_err(|e| format!("error in read handle {}: {:?}", i + 1, e))
            .unwrap();
    }
}
