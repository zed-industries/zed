#![cfg(all(test, feature = "future"))]

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use async_lock::Barrier;
use moka::{future::Cache, Entry};

const NUM_THREADS: u8 = 16;
const SITE: &str = "https://www.rust-lang.org/";

#[tokio::test]
async fn test_get_with() {
    const TEN_MIB: usize = 10 * 1024 * 1024; // 10MiB
    let cache = Cache::new(100);
    let call_counter = Arc::new(AtomicUsize::default());
    let barrier = Arc::new(Barrier::new(NUM_THREADS as usize));

    let tasks: Vec<_> = (0..NUM_THREADS)
        .map(|task_id| {
            let my_cache = cache.clone();
            let my_call_counter = Arc::clone(&call_counter);
            let my_barrier = Arc::clone(&barrier);

            tokio::spawn(async move {
                my_barrier.wait().await;

                println!("Task {task_id} started.");

                let key = "key1".to_string();
                let value = match task_id % 4 {
                    0 => {
                        my_cache
                            .get_with(key.clone(), async move {
                                println!("Task {task_id} inserting a value.");
                                my_call_counter.fetch_add(1, Ordering::AcqRel);
                                Arc::new(vec![0u8; TEN_MIB])
                            })
                            .await
                    }
                    1 => {
                        my_cache
                            .get_with_by_ref(key.as_str(), async move {
                                println!("Task {task_id} inserting a value.");
                                my_call_counter.fetch_add(1, Ordering::AcqRel);
                                Arc::new(vec![0u8; TEN_MIB])
                            })
                            .await
                    }
                    2 => my_cache
                        .entry(key.clone())
                        .or_insert_with(async move {
                            println!("Task {task_id} inserting a value.");
                            my_call_counter.fetch_add(1, Ordering::AcqRel);
                            Arc::new(vec![0u8; TEN_MIB])
                        })
                        .await
                        .into_value(),
                    3 => my_cache
                        .entry_by_ref(key.as_str())
                        .or_insert_with(async move {
                            println!("Task {task_id} inserting a value.");
                            my_call_counter.fetch_add(1, Ordering::AcqRel);
                            Arc::new(vec![0u8; TEN_MIB])
                        })
                        .await
                        .into_value(),
                    _ => unreachable!(),
                };

                assert_eq!(value.len(), TEN_MIB);
                assert!(my_cache.get(key.as_str()).await.is_some());

                println!("Task {task_id} got the value. (len: {})", value.len());
            })
        })
        .collect();

    futures_util::future::join_all(tasks).await;
    assert_eq!(call_counter.load(Ordering::Acquire), 1);
}

#[tokio::test]
async fn test_optionally_get_with() {
    let cache = Cache::new(100);
    let call_counter = Arc::new(AtomicUsize::default());
    let barrier = Arc::new(Barrier::new(NUM_THREADS as usize));

    async fn get_html(task_id: u8, uri: &str, call_counter: &AtomicUsize) -> Option<String> {
        println!("get_html() called by task {task_id}.");
        call_counter.fetch_add(1, Ordering::AcqRel);
        reqwest::get(uri).await.ok()?.text().await.ok()
    }

    let tasks: Vec<_> = (0..NUM_THREADS)
        .map(|task_id| {
            let my_cache = cache.clone();
            let my_call_counter = Arc::clone(&call_counter);
            let my_barrier = Arc::clone(&barrier);

            tokio::spawn(async move {
                my_barrier.wait().await;

                println!("Task {task_id} started.");

                let key = "key1".to_string();
                let value = match task_id % 4 {
                    0 => {
                        my_cache
                            .optionally_get_with(
                                key.clone(),
                                get_html(task_id, SITE, &my_call_counter),
                            )
                            .await
                    }
                    1 => {
                        my_cache
                            .optionally_get_with_by_ref(
                                key.as_str(),
                                get_html(task_id, SITE, &my_call_counter),
                            )
                            .await
                    }
                    2 => my_cache
                        .entry(key.clone())
                        .or_optionally_insert_with(get_html(task_id, SITE, &my_call_counter))
                        .await
                        .map(Entry::into_value),
                    3 => my_cache
                        .entry_by_ref(key.as_str())
                        .or_optionally_insert_with(get_html(task_id, SITE, &my_call_counter))
                        .await
                        .map(Entry::into_value),
                    _ => unreachable!(),
                };

                assert!(value.is_some());
                assert!(my_cache.get(key.as_str()).await.is_some());

                println!(
                    "Task {task_id} got the value. (len: {})",
                    value.unwrap().len()
                );
            })
        })
        .collect();

    futures_util::future::join_all(tasks).await;
    assert_eq!(call_counter.load(Ordering::Acquire), 1);
}

#[tokio::test]
async fn test_try_get_with() {
    let cache = Cache::new(100);
    let call_counter = Arc::new(AtomicUsize::default());
    let barrier = Arc::new(Barrier::new(NUM_THREADS as usize));

    async fn get_html(
        task_id: u8,
        uri: &str,
        call_counter: &AtomicUsize,
    ) -> Result<String, reqwest::Error> {
        println!("get_html() called by task {task_id}.");
        call_counter.fetch_add(1, Ordering::AcqRel);
        reqwest::get(uri).await?.text().await
    }

    let tasks: Vec<_> = (0..NUM_THREADS)
        .map(|task_id| {
            let my_cache = cache.clone();
            let my_call_counter = Arc::clone(&call_counter);
            let my_barrier = Arc::clone(&barrier);

            tokio::spawn(async move {
                my_barrier.wait().await;

                println!("Task {task_id} started.");

                let key = "key1".to_string();
                let value = match task_id % 4 {
                    0 => {
                        my_cache
                            .try_get_with(key.clone(), get_html(task_id, SITE, &my_call_counter))
                            .await
                    }
                    1 => {
                        my_cache
                            .try_get_with_by_ref(
                                key.as_str(),
                                get_html(task_id, SITE, &my_call_counter),
                            )
                            .await
                    }
                    2 => my_cache
                        .entry(key.clone())
                        .or_try_insert_with(get_html(task_id, SITE, &my_call_counter))
                        .await
                        .map(Entry::into_value),
                    3 => my_cache
                        .entry_by_ref(key.as_str())
                        .or_try_insert_with(get_html(task_id, SITE, &my_call_counter))
                        .await
                        .map(Entry::into_value),
                    _ => unreachable!(),
                };

                assert!(value.is_ok());
                assert!(my_cache.get(key.as_str()).await.is_some());

                println!(
                    "Task {task_id} got the value. (len: {})",
                    value.unwrap().len()
                );
            })
        })
        .collect();

    futures_util::future::join_all(tasks).await;
    assert_eq!(call_counter.load(Ordering::Acquire), 1);
}
