use std::time::Instant;

use criterion::{Criterion, criterion_group, criterion_main};
use scc::{Guard, HashIndex};

fn iter_with(c: &mut Criterion) {
    c.bench_function("HashIndex: iter_with", |b| {
        b.iter_custom(|iters| {
            let hashindex: HashIndex<u64, u64> = HashIndex::with_capacity(iters as usize * 2);
            for i in 0..iters {
                assert!(hashindex.insert_sync(i, i).is_ok());
            }
            let start = Instant::now();
            let guard = Guard::new();
            let iter = hashindex.iter(&guard);
            for e in iter {
                assert_eq!(e.0, e.1);
            }
            start.elapsed()
        })
    });
}

fn peek(c: &mut Criterion) {
    c.bench_function("HashIndex: peek", |b| {
        b.iter_custom(|iters| {
            let hashindex: HashIndex<u64, u64> = HashIndex::with_capacity(iters as usize * 2);
            for i in 0..iters {
                assert!(hashindex.insert_sync(i, i).is_ok());
            }
            let start = Instant::now();
            let guard = Guard::new();
            for i in 0..iters {
                assert_eq!(hashindex.peek(&i, &guard), Some(&i));
            }
            start.elapsed()
        })
    });
}

fn peek_with(c: &mut Criterion) {
    c.bench_function("HashIndex: peek_with", |b| {
        b.iter_custom(|iters| {
            let hashindex: HashIndex<u64, u64> = HashIndex::with_capacity(iters as usize * 2);
            for i in 0..iters {
                assert!(hashindex.insert_sync(i, i).is_ok());
            }
            let start = Instant::now();
            for i in 0..iters {
                assert_eq!(hashindex.peek_with(&i, |_, v| *v == i), Some(true));
            }
            start.elapsed()
        })
    });
}

criterion_group!(hash_index, iter_with, peek, peek_with);
criterion_main!(hash_index);
