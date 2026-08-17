use criterion::{black_box, criterion_group, criterion_main, Criterion};
use postage::dispatch;
use postage::{sink::Sink, stream::Stream};
#[derive(Clone, Debug)]
struct Message;

pub fn send_recv(c: &mut Criterion) {
    let (mut tx, mut rx) = dispatch::channel::<Message>(8);
    c.bench_function("dispatch::send_recv", |b| {
        b.iter(|| {
            tx.try_send(black_box(Message {})).unwrap();
            rx.try_recv().unwrap();
        });
    });
}

pub fn send_full(c: &mut Criterion) {
    let (mut tx, _rx) = dispatch::channel::<Message>(4);
    for _ in 0..4 {
        tx.try_send(Message {}).unwrap();
    }

    c.bench_function("dispatch::send_full", |b| {
        b.iter(|| {
            tx.try_send(black_box(Message {})).ok();
        });
    });
}

pub fn recv_empty(c: &mut Criterion) {
    let (_, mut rx) = dispatch::channel::<Message>(4);

    c.bench_function("dispatch::recv_empty", |b| {
        b.iter(|| {
            black_box(rx.try_recv().ok());
        });
    });
}

criterion_group!(benches, send_recv, send_full, recv_empty);
criterion_main!(benches);
