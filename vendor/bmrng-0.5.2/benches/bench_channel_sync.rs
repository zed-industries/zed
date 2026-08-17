use bmrng::channel;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tokio::sync::mpsc;

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(6)
        .build()
        .unwrap()
}

fn benchmark_sync(c: &mut Criterion) {
    let rt = rt();

    let mut group = c.benchmark_group("benchmarks_sync");

    group.throughput(Throughput::Elements(1u64));

    group.bench_function("bmrng, bounded, capacity = 1", |b| {
        b.iter(|| {
            rt.block_on(async move {
                let (tx, rx) = channel::<(), ()>(1);
                tokio::spawn(async move {
                    let mut rx = rx;
                    let req = rx.recv().await;
                    if let Ok(req) = req {
                        let _ = req.1.respond(());
                    }
                });
                let _ = tx.send_receive(()).await;
            });
        });
    });

    group.bench_function("mpsc, bounded, capacity = 1", |b| {
        b.iter(|| {
            rt.block_on(async move {
                let (tx, rx) = mpsc::channel::<()>(1);
                tokio::spawn(async move {
                    let mut rx = rx;
                    let _ = rx.recv().await;
                });
                let _ = tx.send(());
            });
        });
    });
}

criterion_group!(benches, benchmark_sync);
criterion_main!(benches);
