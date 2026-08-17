use std::time::{Duration, Instant};

use criterion::async_executor::FuturesExecutor;
use criterion::{Criterion, criterion_group, criterion_main};
use scc::HashMap;

fn insert_remove_single_async(c: &mut Criterion) {
    c.bench_function("HashMap: insert_remove_single_async", |b| {
        let hashmap: HashMap<u64, u64> = HashMap::default();
        assert!(hashmap.insert_sync(0, 0).is_ok());
        async fn test(hashmap: &HashMap<u64, u64>) {
            assert!(hashmap.insert_async(1, 1).await.is_ok());
            assert!(hashmap.remove_async(&1).await.is_some());
        }
        b.to_async(FuturesExecutor).iter(|| test(&hashmap));
    });
}

fn insert_remove_single_sync(c: &mut Criterion) {
    c.bench_function("HashMap: insert_remove_single_sync", |b| {
        let hashmap: HashMap<u64, u64> = HashMap::default();
        assert!(hashmap.insert_sync(0, 0).is_ok());
        fn test(hashmap: &HashMap<u64, u64>) {
            assert!(hashmap.insert_sync(1, 1).is_ok());
            assert!(hashmap.remove_sync(&1).is_some());
        }
        b.iter(|| test(&hashmap));
    });
}

fn insert_cold_async(c: &mut Criterion) {
    c.bench_function("HashMap: insert_cold_async", |b| {
        b.to_async(FuturesExecutor).iter_custom(async |iters| {
            let hashmap: HashMap<u64, u64> = HashMap::default();
            let start = Instant::now();
            for i in 0..iters {
                assert!(hashmap.insert_async(i, i).await.is_ok());
            }
            start.elapsed()
        })
    });
}

fn insert_cold_sync(c: &mut Criterion) {
    c.bench_function("HashMap: insert_cold_sync", |b| {
        b.iter_custom(|iters| {
            let hashmap: HashMap<u64, u64> = HashMap::default();
            let start = Instant::now();
            for i in 0..iters {
                assert!(hashmap.insert_sync(i, i).is_ok());
            }
            start.elapsed()
        })
    });
}

fn insert_warmed_up_async(c: &mut Criterion) {
    c.bench_function("HashMap: insert_warmed_up_async", |b| {
        b.to_async(FuturesExecutor).iter_custom(async |iters| {
            let hashmap: HashMap<u64, u64> = HashMap::with_capacity(iters as usize * 2);
            let start = Instant::now();
            for i in 0..iters {
                assert!(hashmap.insert_async(i, i).await.is_ok());
            }
            start.elapsed()
        })
    });
}

fn insert_warmed_up_sync(c: &mut Criterion) {
    c.bench_function("HashMap: insert_warmed_up_sync", |b| {
        b.iter_custom(|iters| {
            let hashmap: HashMap<u64, u64> = HashMap::with_capacity(iters as usize * 2);
            let start = Instant::now();
            for i in 0..iters {
                assert!(hashmap.insert_sync(i, i).is_ok());
            }
            start.elapsed()
        })
    });
}

fn read_async(c: &mut Criterion) {
    c.bench_function("HashMap: read_async", |b| {
        b.to_async(FuturesExecutor).iter_custom(async |iters| {
            let hashmap: HashMap<u64, u64> = HashMap::with_capacity(iters as usize * 2);
            for i in 0..iters {
                assert!(hashmap.insert_async(i, i).await.is_ok());
            }
            let start = Instant::now();
            for i in 0..iters {
                assert_eq!(hashmap.read_async(&i, |_, v| *v == i).await, Some(true));
            }
            start.elapsed()
        })
    });
}

fn read_sync(c: &mut Criterion) {
    c.bench_function("HashMap: read_sync", |b| {
        b.iter_custom(|iters| {
            let hashmap: HashMap<u64, u64> = HashMap::with_capacity(iters as usize * 2);
            for i in 0..iters {
                assert!(hashmap.insert_sync(i, i).is_ok());
            }
            let start = Instant::now();
            for i in 0..iters {
                assert_eq!(hashmap.read_sync(&i, |_, v| *v == i), Some(true));
            }
            start.elapsed()
        })
    });
}

fn insert_tail_latency(c: &mut Criterion) {
    c.bench_function("HashMap: insert, tail latency", move |b| {
        b.iter_custom(|iters| {
            let mut agg_max_latency = Duration::default();
            for _ in 0..iters {
                let hashmap: HashMap<u64, u64> = HashMap::default();
                let mut key = 0;
                let mut max_latency = Duration::default();
                (0..1048576).for_each(|_| {
                    key += 1;
                    let start = Instant::now();
                    assert!(hashmap.insert_sync(key, key).is_ok());
                    let elapsed = start.elapsed();
                    if elapsed > max_latency {
                        max_latency = elapsed;
                    }
                });
                agg_max_latency += max_latency;
            }
            agg_max_latency
        })
    });
}

criterion_group!(
    hash_map,
    insert_remove_single_async,
    insert_remove_single_sync,
    insert_cold_async,
    insert_cold_sync,
    insert_warmed_up_async,
    insert_warmed_up_sync,
    insert_tail_latency,
    read_async,
    read_sync
);
criterion_main!(hash_map);
