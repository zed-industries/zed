use std::time::Instant;

use criterion::async_executor::FuturesExecutor;
use criterion::{Criterion, criterion_group, criterion_main};
use scc::{Guard, TreeIndex};

fn insert_async(c: &mut Criterion) {
    c.bench_function("TreeIndex: insert_async", |b| {
        b.to_async(FuturesExecutor).iter_custom(async |iters| {
            let treeindex: TreeIndex<u64, u64> = TreeIndex::default();
            let start = Instant::now();
            for i in 0..iters {
                assert!(treeindex.insert_async(i, i).await.is_ok());
            }
            start.elapsed()
        })
    });
}

fn insert_sync(c: &mut Criterion) {
    c.bench_function("TreeIndex: insert_sync", |b| {
        b.iter_custom(|iters| {
            let treeindex: TreeIndex<u64, u64> = TreeIndex::default();
            let start = Instant::now();
            for i in 0..iters {
                assert!(treeindex.insert_sync(i, i).is_ok());
            }
            start.elapsed()
        })
    });
}

fn insert_rev(c: &mut Criterion) {
    c.bench_function("TreeIndex: insert, rev", |b| {
        b.iter_custom(|iters| {
            let treeindex: TreeIndex<u64, u64> = TreeIndex::default();
            let start = Instant::now();
            for i in (0..iters).rev() {
                assert!(treeindex.insert_sync(i, i).is_ok());
            }
            start.elapsed()
        })
    });
}

fn iter(c: &mut Criterion) {
    c.bench_function("TreeIndex: iter", |b| {
        b.iter_custom(|iters| {
            let treeindex: TreeIndex<u64, u64> = TreeIndex::default();
            for i in 0..iters {
                assert!(treeindex.insert_sync(i, i).is_ok());
            }
            let start = Instant::now();
            let guard = Guard::new();
            let iter = treeindex.iter(&guard);
            for e in iter {
                assert_eq!(e.0, e.1);
            }
            start.elapsed()
        })
    });
}

fn range(c: &mut Criterion) {
    c.bench_function("TreeIndex: range", |b| {
        b.iter_custom(|iters| {
            let treeindex: TreeIndex<u64, u64> = TreeIndex::default();
            for i in 0..iters {
                assert!(treeindex.insert_sync(i, i).is_ok());
            }
            let start = Instant::now();
            let guard = Guard::new();
            for s in 0..iters {
                let range = s..iters;
                let mut iter = treeindex.range(range.clone(), &guard);
                assert_eq!(range.is_empty(), iter.next().is_none());
            }
            start.elapsed()
        })
    });
}

fn range_rev(c: &mut Criterion) {
    c.bench_function("TreeIndex: range, rev", |b| {
        b.iter_custom(|iters| {
            let treeindex: TreeIndex<u64, u64> = TreeIndex::default();
            for i in 0..iters {
                assert!(treeindex.insert_sync(i, i).is_ok());
            }
            let start = Instant::now();
            let guard = Guard::new();
            for s in 0..iters {
                let range = 0..iters - s;
                let mut iter = treeindex.range(range.clone(), &guard);
                assert_eq!(range.is_empty(), iter.next_back().is_none());
            }
            start.elapsed()
        })
    });
}

fn peek(c: &mut Criterion) {
    c.bench_function("TreeIndex: peek", |b| {
        b.iter_custom(|iters| {
            let treeindex: TreeIndex<u64, u64> = TreeIndex::default();
            for i in 0..iters {
                assert!(treeindex.insert_sync(i, i).is_ok());
            }
            let start = Instant::now();
            let guard = Guard::new();
            for i in 0..iters {
                assert_eq!(treeindex.peek(&i, &guard), Some(&i));
            }
            start.elapsed()
        })
    });
}

criterion_group!(
    tree_index,
    insert_async,
    insert_sync,
    insert_rev,
    iter,
    range,
    range_rev,
    peek
);
criterion_main!(tree_index);
