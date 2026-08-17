//! Benchmarks to track basic performance across changes.
//!
//! Slightly based on the <background.rs> benchmarks, but simplified and stripped down to run
//! reasonably fast.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use arc_swap::access::{Access, Map};
use arc_swap::cache::Cache;
use arc_swap::ArcSwap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam_utils::thread;

/// Execute a group of measurements
///
/// It expects any kind of „environment“ is already in place for it.
fn batch(c: &mut Criterion, name: &str, shared_number: &ArcSwap<usize>) {
    let mut g = c.benchmark_group(name);

    g.bench_function("load", |b| {
        b.iter(|| {
            black_box(shared_number.load());
        })
    });
    g.bench_function("load_full", |b| {
        b.iter(|| {
            black_box(shared_number.load_full());
        })
    });
    g.bench_function("load_many", |b| {
        // Here we simulate running out of the debt slots scenario
        const MANY: usize = 32;
        let mut guards = Vec::with_capacity(MANY);
        b.iter(|| {
            guards.push(black_box(shared_number.load()));
            if guards.len() == MANY {
                guards.clear();
            }
        })
    });
    g.bench_function("store", |b| {
        b.iter(|| {
            black_box(shared_number.store(Arc::new(42)));
        })
    });
    g.bench_function("cache", |b| {
        let mut cache = Cache::new(shared_number);
        b.iter(|| {
            black_box(cache.load());
        })
    });

    g.finish();
}

fn with_background<F: Fn(&ArcSwap<usize>) + Sync>(
    c: &mut Criterion,
    name: &str,
    cnt: usize,
    noise: F,
) {
    let stop = AtomicBool::new(false);
    let shared_number = ArcSwap::from_pointee(42);
    thread::scope(|s| {
        // Start some background noise threads, to contend the arc swap.
        for _ in 0..cnt {
            s.spawn(|_| {
                while !stop.load(Ordering::Relaxed) {
                    noise(&shared_number);
                }
            });
        }

        // Perform the benchmarks
        batch(c, name, &shared_number);

        // Ask the threads to terminate, so they don't disturb any other benchmarks
        stop.store(true, Ordering::Relaxed);
    })
    .unwrap();
}

fn utilities(c: &mut Criterion) {
    let mut g = c.benchmark_group("utilities");

    struct Composed {
        val: i32,
    }

    g.bench_function("access-map", |b| {
        let a = Arc::new(ArcSwap::from_pointee(Composed { val: 42 }));
        let m = Map::new(Arc::clone(&a), |c: &Composed| &c.val);
        b.iter(|| {
            let g = black_box(m.load());
            assert_eq!(42, *g);
        });
    });
}

fn benchmark(c: &mut Criterion) {
    batch(c, "uncontended", &ArcSwap::from_pointee(42));
    with_background(c, "concurrent_loads", 2, |s| {
        black_box(s.load());
    });
    with_background(c, "concurrent_store", 1, |s| {
        black_box(s.store(Arc::new(42)));
    });
    utilities(c);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
