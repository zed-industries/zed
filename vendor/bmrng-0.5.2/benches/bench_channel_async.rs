use bmrng::channel;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tokio::sync::mpsc;

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(6)
        .build()
        .unwrap()
}

fn benchmark_async(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmarks");

    group.throughput(Throughput::Elements(1u64));

    group.bench_function("bmrng async, bounded, capacity = 1", move |b| {
        b.to_async(rt()).iter(|| async {
            let (tx, rx) = channel::<(), ()>(1);
            tokio::spawn(async move {
                let mut rx = rx;
                let req = rx.recv().await;
                if let Ok(req) = req {
                    let _ = req.1.respond(());
                }
            });
            let _ = tx.send_receive(()).await;
        })
    });

    group.bench_function("mpsc async, bounded, capacity = 1", move |b| {
        b.to_async(rt()).iter(|| async {
            let (tx, rx) = mpsc::channel::<()>(1);
            tokio::spawn(async move {
                let mut rx = rx;
                let _ = rx.recv().await;
            });
            let _ = tx.send(());
        })
    });
}

criterion_group!(benches, benchmark_async);
criterion_main!(benches);
