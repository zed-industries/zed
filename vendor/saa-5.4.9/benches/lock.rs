use std::hint::{black_box, spin_loop};
use std::pin::pin;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;
use std::time::{Duration, Instant};

use criterion::async_executor::FuturesExecutor;
use criterion::{Criterion, criterion_group, criterion_main};
use saa::{Barrier, Lock, Pager};

fn exclusive_unlock_async(c: &mut Criterion) {
    async fn test() {
        let lock = Lock::default();
        lock.lock_async().await;
        assert!(lock.release_lock());
    }
    c.bench_function("lock-exclusive-unlock-async", |b| {
        b.to_async(FuturesExecutor).iter(test);
    });
}

fn exclusive_unlock_sync(c: &mut Criterion) {
    c.bench_function("lock-exclusive-unlock-sync", |b| {
        b.iter(|| {
            let lock = Lock::default();
            lock.lock_sync();
            assert!(lock.release_lock());
        });
    });
}

fn exclusive_unlock_async_with(c: &mut Criterion) {
    async fn test() {
        let lock = Lock::default();
        let mut waited = false;
        lock.lock_async_with(|| {
            waited = true;
            std::thread::yield_now();
        })
        .await;
        assert!(!waited);
        assert!(lock.release_lock());
    }
    c.bench_function("lock-exclusive-unlock-async-with", |b| {
        b.to_async(FuturesExecutor).iter(test);
    });
}

fn exclusive_unlock_sync_with(c: &mut Criterion) {
    c.bench_function("lock-exclusive-unlock-sync-with", |b| {
        b.iter(|| {
            let lock = Lock::default();
            let mut waited = false;
            lock.lock_sync_with(|| {
                waited = true;
                std::thread::yield_now();
            });
            assert!(!waited);
            assert!(lock.release_lock());
        });
    });
}

fn shared_shared_unlock_unlock(c: &mut Criterion) {
    c.bench_function("share-shared-unlock-unlock", |b| {
        b.iter(|| {
            let lock = Lock::default();
            lock.share_sync();
            lock.share_sync();
            assert!(lock.release_share());
            assert!(lock.release_share());
        });
    });
}

fn spin_try_lock(c: &mut Criterion) {
    c.bench_function("spin_try_lock", |b| {
        b.iter_custom(|iters| {
            let lock = Lock::default();
            lock.lock_sync();
            let now = Instant::now();
            for _ in 0..iters {
                assert!(!lock.try_lock());
                spin_loop();
            }
            now.elapsed()
        });
    });
}

fn wait_awake(c: &mut Criterion) {
    c.bench_function("wait_awake", |b| {
        b.iter_custom(|iters| {
            let lock = Arc::new(Lock::default());
            let entered = Arc::new(AtomicBool::new(false));
            let mut acc = Duration::from_secs(0);
            for _ in 0..iters {
                assert!(lock.lock_sync());
                let lock_clone = lock.clone();
                let entered_clone = entered.clone();
                let thread = thread::spawn(move || {
                    let mut start = Instant::now();
                    lock_clone.lock_sync_with(|| {
                        entered_clone.swap(true, Release);
                        start = Instant::now();
                    });
                    start.elapsed()
                });
                while !entered.load(Acquire) {}
                assert!(lock.release_lock());
                acc += thread.join().unwrap();
                assert!(lock.release_lock());
                assert!(entered.swap(false, Release));
            }
            acc
        })
    });
}

fn multi_threaded_workload(iters: u64, num_threads: usize, num_cycles: usize) -> Duration {
    let lock = Arc::new(Lock::default());
    let mut threads = Vec::with_capacity(num_threads);
    let barrier = Arc::new(Barrier::with_count(num_threads));
    for _ in 0..num_threads {
        let barrier = barrier.clone();
        let lock = lock.clone();
        let join = thread::spawn(move || {
            barrier.wait_sync();
            let start = Instant::now();
            for _ in 0..iters {
                assert!(lock.lock_sync());
                let acc = black_box({
                    (black_box(0)..black_box(num_cycles)).fold(black_box(0), |acc, v| acc + v)
                });
                assert_eq!(acc, (0..num_cycles).sum::<usize>());
                assert!(lock.release_lock());
            }
            start.elapsed()
        });
        threads.push(join);
    }
    threads
        .into_iter()
        .fold(Duration::from_nanos(0), |acc, t| acc.max(t.join().unwrap()))
}

macro_rules! multi_threaded {
    ($name:ident, $num_threads:expr, $num_cycles:expr) => {
        fn $name(c: &mut Criterion) {
            c.bench_function(stringify!($name), |b| {
                b.iter_custom(|iters| multi_threaded_workload(iters, $num_threads, $num_cycles))
            });
        }
    };
}

multi_threaded!(multi_threaded_4_256, 4, 256);
multi_threaded!(multi_threaded_4_65536, 4, 65536);
multi_threaded!(multi_threaded_16_256, 16, 256);
multi_threaded!(multi_threaded_16_65536, 16, 65536);

fn multi_threaded_pager_workload(iters: u64, num_threads: usize, num_cycles: usize) -> Duration {
    let lock = Arc::new(Lock::default());
    let mut threads = Vec::with_capacity(num_threads);
    let barrier = Arc::new(Barrier::with_count(num_threads));
    for _ in 0..num_threads {
        let barrier = barrier.clone();
        let lock = lock.clone();
        let join = thread::spawn(move || {
            barrier.wait_sync();
            let start = Instant::now();
            for _ in 0..iters {
                let mut pager = pin!(Pager::default());
                lock.register_pager(&mut pager, saa::lock::Mode::WaitExclusive, true);
                assert!(pager.poll_sync().unwrap());
                assert!(lock.lock_sync());
                let acc = black_box({
                    (black_box(0)..black_box(num_cycles)).fold(black_box(0), |acc, v| acc + v)
                });
                assert_eq!(acc, (0..num_cycles).sum::<usize>());
                assert!(lock.release_lock());
            }
            start.elapsed()
        });
        threads.push(join);
    }
    threads
        .into_iter()
        .fold(Duration::from_nanos(0), |acc, t| acc.max(t.join().unwrap()))
}

macro_rules! multi_threaded_pager {
    ($name:ident, $num_threads:expr, $num_cycles:expr) => {
        fn $name(c: &mut Criterion) {
            c.bench_function(stringify!($name), |b| {
                b.iter_custom(|iters| {
                    multi_threaded_pager_workload(iters, $num_threads, $num_cycles)
                })
            });
        }
    };
}

multi_threaded_pager!(multi_threaded_pager_4_256, 4, 256);
multi_threaded_pager!(multi_threaded_pager_4_65536, 4, 65536);
multi_threaded_pager!(multi_threaded_pager_16_256, 16, 256);
multi_threaded_pager!(multi_threaded_pager_16_65536, 16, 65536);

criterion_group!(
    lock,
    exclusive_unlock_async,
    exclusive_unlock_sync,
    exclusive_unlock_async_with,
    exclusive_unlock_sync_with,
    shared_shared_unlock_unlock,
    spin_try_lock,
    wait_awake,
    multi_threaded_4_256,
    multi_threaded_4_65536,
    multi_threaded_16_256,
    multi_threaded_16_65536,
    multi_threaded_pager_4_256,
    multi_threaded_pager_4_65536,
    multi_threaded_pager_16_256,
    multi_threaded_pager_16_65536,
);
criterion_main!(lock);
