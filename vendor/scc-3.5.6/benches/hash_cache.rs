use std::time::Instant;

use criterion::{Criterion, criterion_group, criterion_main};
use scc::HashCache;

fn get_saturated(c: &mut Criterion) {
    let hashcache: HashCache<u64, u64> = HashCache::with_capacity(64, 64);
    for k in 0..256 {
        assert!(hashcache.put_sync(k, k).is_ok());
    }
    let mut max_key = 256;
    c.bench_function("HashCache: get, saturated", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for i in max_key..(max_key + iters) {
                drop(hashcache.get_sync(&i));
            }
            max_key += iters;
            start.elapsed()
        })
    });
}

fn put_saturated(c: &mut Criterion) {
    let hashcache: HashCache<u64, u64> = HashCache::with_capacity(64, 64);
    for k in 0..256 {
        assert!(hashcache.put_sync(k, k).is_ok());
    }
    let mut max_key = 256;
    c.bench_function("HashCache: put, saturated", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for i in max_key..(max_key + iters) {
                assert!(hashcache.put_sync(i, i).is_ok());
            }
            max_key += iters;
            start.elapsed()
        })
    });
}

criterion_group!(hash_cache, get_saturated, put_saturated);
criterion_main!(hash_cache);
