#![cfg(all(test, feature = "future"))]

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use actix_rt::Runtime;
use async_lock::Barrier;
use moka::future::Cache;

const NUM_THREADS: u8 = 16;

#[test]
fn test_get_with() -> Result<(), Box<dyn std::error::Error>> {
    const TEN_MIB: usize = 10 * 1024 * 1024; // 10MiB
    let cache = Cache::new(100);
    let call_counter = Arc::new(AtomicUsize::default());
    let barrier = Arc::new(Barrier::new(NUM_THREADS as usize));

    let rt = Runtime::new()?;

    let tasks: Vec<_> = (0..NUM_THREADS)
        .map(|task_id| {
            let my_cache = cache.clone();
            let my_call_counter = Arc::clone(&call_counter);
            let my_barrier = Arc::clone(&barrier);

            rt.spawn(async move {
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

    rt.block_on(futures_util::future::join_all(tasks));
    assert_eq!(call_counter.load(Ordering::Acquire), 1);

    Ok(())
}
