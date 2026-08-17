use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures_test::task::noop_context;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
struct Message;

pub fn send_recv(c: &mut Criterion) {
    let (tx, mut rx) = mpsc::channel::<Message>(8);
    c.bench_function("tokio::mpsc::send_recv", |b| {
        let mut cx = noop_context();
        b.iter(|| {
            tx.try_send(black_box(Message {})).unwrap();
            rx.poll_recv(&mut cx).is_ready();
        });
    });
}

pub fn send_full(c: &mut Criterion) {
    let (tx, _rx) = mpsc::channel::<Message>(4);
    for _ in 0..4 {
        tx.try_send(Message {}).unwrap();
    }

    c.bench_function("tokio::mpsc::send_full", |b| {
        b.iter(|| {
            tx.try_send(black_box(Message {})).ok();
        });
    });
}

pub fn recv_empty(c: &mut Criterion) {
    let (_tx, mut rx) = mpsc::channel::<Message>(4);

    c.bench_function("tokio::mpsc::recv_empty", |b| {
        let mut cx = noop_context();
        b.iter(|| {
            black_box(rx.poll_recv(&mut cx)).is_ready();
        });
    });
}

criterion_group!(benches, send_recv, send_full, recv_empty);
criterion_main!(benches);
