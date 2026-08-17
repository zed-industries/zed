#![allow(clippy::identity_op)]
use criterion::{criterion_group, criterion_main, Criterion};

/// Allows doing multiple inner iterations per bench.iter() run.
///
/// This is mostly to amortise the overhead of spawning a thread in the benchmark
/// when sending larger messages (that might be fragmented).
///
/// Note that you need to compensate the displayed results
/// for the proportionally longer runs yourself,
/// as the benchmark framework doesn't know about the inner iterations...
const ITERATIONS: usize = 1;

use ipc_channel::ipc::{self, IpcReceiverSet};

/// Benchmark selecting over a set of `n` receivers,
/// with `to_send` of them actually having pending data.
fn bench_send_on_m_of_n<const TO_SEND: usize, const N: usize>(criterion: &mut Criterion) {
    criterion.bench_function(&format!("bench_send_on_{TO_SEND}_of_{N}"), |bencher| {
        let mut senders = Vec::with_capacity(N);
        let mut rx_set = IpcReceiverSet::new().unwrap();
        for _ in 0..N {
            let (tx, rx) = ipc::channel().unwrap();
            rx_set.add(rx).unwrap();
            senders.push(tx);
        }
        bencher.iter(|| {
            for _ in 0..ITERATIONS {
                for tx in senders.iter().take(TO_SEND) {
                    tx.send(()).unwrap();
                }
                let mut received = 0;
                while received < TO_SEND {
                    received += rx_set.select().unwrap().len();
                }
            }
        });
    });
}

fn create_set_of_n<const N: usize>() -> IpcReceiverSet {
    let mut rx_set = IpcReceiverSet::new().unwrap();
    for _ in 0..N {
        let (_, rx) = ipc::channel::<()>().unwrap();
        rx_set.add(rx).unwrap();
    }
    rx_set
}

fn create_and_destroy_set_of_n<const N: usize>(criterion: &mut Criterion) {
    criterion.bench_function(&format!("create_and_destroy_set_of_{N}"), |bencher| {
        bencher.iter(|| {
            for _ in 0..ITERATIONS {
                create_set_of_n::<N>();
            }
        });
    });
}

// Benchmark performance of removing closed receivers from set.
// This also includes the time for adding receivers,
// as there is no way to measure the removing in isolation.
fn add_and_remove_n_closed_receivers<const N: usize>(criterion: &mut Criterion) {
    criterion.bench_function(&format!("add_and_remove_{N}_closed_receivers"), |bencher| {
        bencher.iter(|| {
            for _ in 0..ITERATIONS {
                // We could keep adding/removing senders to the same set,
                // instead of creating a new one in each iteration.
                // However, this would actually make the results harder to compare...
                let mut rx_set = create_set_of_n::<N>();

                let mut dropped_count = 0;
                while dropped_count < N {
                    // On `select()`, receivers with a "ClosedChannel" event will be closed,
                    // and automatically dropped from the set.
                    dropped_count += rx_set.select().unwrap().len();
                }
            }
        });
    });
}

criterion_group!(
    benches,
    bench_send_on_m_of_n<1,1>,
    bench_send_on_m_of_n<1,5>,
    bench_send_on_m_of_n<2,5>,
    bench_send_on_m_of_n<5,5>,
    bench_send_on_m_of_n<1,20>,
    bench_send_on_m_of_n<5,20>,
    bench_send_on_m_of_n<20,20>,
    bench_send_on_m_of_n<1,100>,
    bench_send_on_m_of_n<5,100>,
    bench_send_on_m_of_n<20,100>,
    bench_send_on_m_of_n<100,100>,
    create_and_destroy_set_of_n<0>,
    create_and_destroy_set_of_n<1>,
    create_and_destroy_set_of_n<10>,
    create_and_destroy_set_of_n<100>,
    add_and_remove_n_closed_receivers<1>,
    add_and_remove_n_closed_receivers<10>,
    add_and_remove_n_closed_receivers<100>,
);
criterion_main!(benches);
