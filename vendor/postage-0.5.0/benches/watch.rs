use criterion::{black_box, criterion_group, criterion_main, Criterion};
use postage::watch;
use postage::{sink::Sink, stream::Stream};

#[derive(Clone, Debug, Default)]
struct Message;

pub fn send_recv(c: &mut Criterion) {
    let (mut tx, mut rx) = watch::channel::<Message>();
    c.bench_function("watch::send_recv", |b| {
        b.iter(|| {
            tx.try_send(black_box(Message {})).unwrap();
            rx.try_recv().unwrap();
        });
    });
}

pub fn recv_empty(c: &mut Criterion) {
    let (_tx, mut rx) = watch::channel::<Message>();
    rx.try_recv().ok();

    c.bench_function("watch::recv_empty", |b| {
        b.iter(|| {
            black_box(rx.try_recv().ok());
        });
    });
}
criterion_group!(benches, send_recv, recv_empty);
criterion_main!(benches);
