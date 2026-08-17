use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use futures_concurrency::prelude::*;

mod utils;

fn join_vs_join_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare vec::join vs join_all");
    for i in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("futures-concurrency", i), i, |b, i| {
            b.to_async(FuturesExecutor).iter(|| async {
                let futs = utils::futures_vec(*i as usize);
                let output = futs.join().await;
                assert_eq!(output.len(), *i as usize);
            })
        });
        group.bench_with_input(BenchmarkId::new("futures-rs", i), i, |b, i| {
            b.to_async(FuturesExecutor).iter(|| async {
                let futs = utils::futures_vec(*i as usize);
                let output = futures::future::join_all(futs).await;
                assert_eq!(output.len(), *i as usize);
            })
        });
    }
    group.finish();
}

fn select_vs_merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare array::merge vs select!");
    const I: u64 = 10;
    const N: usize = I as usize;
    group.bench_with_input(BenchmarkId::new("futures-concurrency", I), &I, |b, _| {
        b.to_async(FuturesExecutor).iter(|| async {
            use futures_lite::prelude::*;
            let mut counter = 0;
            let streams = utils::streams_array::<N>();
            let mut s = streams.merge();
            while s.next().await.is_some() {
                counter += 1;
            }
            assert_eq!(counter, N);
        })
    });
    group.bench_with_input(BenchmarkId::new("futures-rs", I), &I, |b, _| {
        b.to_async(FuturesExecutor).iter(|| async {
            use futures::select;
            use futures::stream::StreamExt;

            // Create two streams of numbers. Both streams require being `fuse`d.
            let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j] =
                utils::streams_array::<N>().map(|fut| fut.fuse());

            // Initialize the output counter.
            let mut total = 0usize;

            // Process each item in the stream;
            // break once there are no more items left to sum.
            loop {
                let item = select! {
                    item = a.next() => item,
                    item = b.next() => item,
                    item = c.next() => item,
                    item = d.next() => item,
                    item = e.next() => item,
                    item = f.next() => item,
                    item = g.next() => item,
                    item = h.next() => item,
                    item = i.next() => item,
                    item = j.next() => item,
                    complete => break,
                };
                if item.is_some() {
                    // Increment the counter
                    total += 1;
                }
            }

            assert_eq!(total, N);
        })
    });
    group.finish();
}

criterion_group!(join_bench, join_vs_join_all);
criterion_group!(merge_bench, select_vs_merge);
criterion_main!(join_bench, merge_bench);
