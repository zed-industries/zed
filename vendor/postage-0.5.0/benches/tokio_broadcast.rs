use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokio::sync::broadcast;
#[derive(Clone, Debug)]
struct Message;

pub fn send_recv(c: &mut Criterion) {
    let (tx, mut rx) = broadcast::channel::<Message>(8);
    c.bench_function("tokio::broadcast::send_recv", |b| {
        b.iter(|| {
            tx.send(black_box(Message {})).unwrap();
            rx.try_recv().unwrap();
        });
    });
}

pub fn send_full(c: &mut Criterion) {
    let (tx, _rx) = broadcast::channel::<Message>(4);
    for _ in 0..4 {
        tx.send(Message {}).unwrap();
    }

    c.bench_function("tokio::broadcast::send_full", |b| {
        b.iter(|| {
            tx.send(black_box(Message {})).ok();
        });
    });
}

pub fn recv_empty(c: &mut Criterion) {
    let (_, mut rx) = broadcast::channel::<Message>(4);

    c.bench_function("tokio::broadcast::recv_empty", |b| {
        b.iter(|| {
            black_box(rx.try_recv().ok());
        });
    });
}

criterion_group!(benches, send_recv, send_full, recv_empty);
criterion_main!(benches);
