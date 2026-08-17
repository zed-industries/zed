use criterion::{criterion_group, criterion_main, Criterion};
use tracy_client::Client;

fn client_start(c: &mut Criterion) {
    let mut clients = Vec::<Client>::with_capacity(1_000_000_000);
    c.bench_function("start", |b| b.iter(|| clients.push(Client::start())));
}

fn client_clone(c: &mut Criterion) {
    let mut clients = Vec::<Client>::with_capacity(1_000_000_000);
    let client = Client::start();
    c.bench_function("clone", |b| b.iter(|| clients.push(client.clone())));
}

fn client_running(c: &mut Criterion) {
    let _client = Client::start();
    c.bench_function("running", |b| b.iter(Client::running));
}

fn ops_alloc(c: &mut Criterion) {
    let client = Client::start();
    c.bench_function("span_alloc_callstack/0", |bencher| {
        bencher.iter(|| {
            let _ = client
                .clone()
                .span_alloc(Some("hello"), "function", "file", 42, 0);
        });
    });
    c.bench_function("span_alloc_callstack/100", |bencher| {
        bencher.iter(|| {
            let _ = client
                .clone()
                .span_alloc(Some("hello"), "function", "file", 42, 100);
        });
    });
}

fn ops_static(c: &mut Criterion) {
    let _client = tracy_client::Client::start();
    c.bench_function("span_callstack/0", |bencher| {
        bencher.iter(|| {
            let _ = tracy_client::span!("some_name", 0);
        });
    });
    c.bench_function("span_callstack/100", |bencher| {
        bencher.iter(|| {
            let _ = tracy_client::span!("some_name", 100);
        });
    });
}

criterion_group!(
    benches,
    client_start,
    client_clone,
    client_running,
    ops_alloc,
    ops_static
);
criterion_main!(benches);
