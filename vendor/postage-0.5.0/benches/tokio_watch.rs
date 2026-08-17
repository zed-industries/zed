use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures_test::task::noop_context;
use std::{future::Future, pin::Pin};
use tokio::sync::watch;

#[derive(Clone, Debug)]
struct Message;

pub fn send_recv(c: &mut Criterion) {
    let (tx, mut rx) = watch::channel::<Message>(Message {});
    c.bench_function("tokio::watch::send_recv", |b| {
        let mut cx = noop_context();
        b.iter(|| {
            tx.send(black_box(Message {})).unwrap();
            let mut rx_changed = rx.changed();
            let pin = unsafe { Pin::new_unchecked(&mut rx_changed) };
            pin.poll(black_box(&mut cx)).is_ready();
        });
    });
}

pub fn recv_empty(c: &mut Criterion) {
    let (_tx, mut rx) = watch::channel::<Message>(Message {});

    c.bench_function("tokio::watch::recv_empty", |b| {
        b.iter(|| {
            let mut cx = noop_context();
            let mut rx_changed = rx.changed();
            let pin = unsafe { Pin::new_unchecked(&mut rx_changed) };
            pin.poll(black_box(&mut cx)).is_ready();
        });
    });
}

criterion_group!(benches, send_recv, recv_empty);
criterion_main!(benches);
