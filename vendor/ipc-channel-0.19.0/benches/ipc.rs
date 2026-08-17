#![allow(clippy::identity_op)]
use criterion::{criterion_group, criterion_main, Criterion};
use ipc_channel::ipc;

/// Allows doing multiple inner iterations per bench.iter() run.
///
/// This is mostly to amortise the overhead of spawning a thread in the benchmark
/// when sending larger messages (that might be fragmented).
///
/// Note that you need to compensate the displayed results
/// for the proportionally longer runs yourself,
/// as the benchmark framework doesn't know about the inner iterations...
const ITERATIONS: usize = 1;

fn transfer_empty(criterion: &mut Criterion) {
    criterion.bench_function("transfer_empty", |bencher| {
        let (tx, rx) = ipc::channel().unwrap();
        bencher.iter(|| {
            for _ in 0..ITERATIONS {
                tx.send(()).unwrap();
                rx.recv().unwrap()
            }
        });
    });
}

fn transfer_senders<const COUNT: usize>(criterion: &mut Criterion) {
    criterion.bench_function(&format!("transfer_senders_{COUNT:02}"), |bencher| {
        let (main_tx, main_rx) = ipc::channel().unwrap();
        let transfer_txs: Vec<_> = (0..COUNT)
            .map(|_| ipc::channel::<()>().unwrap())
            .map(|(tx, _)| tx)
            .collect();
        let mut transfer_txs = Some(transfer_txs);
        bencher.iter(|| {
            for _ in 0..ITERATIONS {
                main_tx.send(transfer_txs.take().unwrap()).unwrap();
                transfer_txs = Some(main_rx.recv().unwrap());
            }
        });
    });
}

fn transfer_receivers<const COUNT: usize>(criterion: &mut Criterion) {
    criterion.bench_function(&format!("transfer_receivers_{COUNT:02}"), |bencher| {
        let (main_tx, main_rx) = ipc::channel().unwrap();
        let transfer_rxs: Vec<_> = (0..COUNT)
            .map(|_| ipc::channel::<()>().unwrap())
            .map(|(_, rx)| rx)
            .collect();
        let mut transfer_rxs = Some(transfer_rxs);
        bencher.iter(|| {
            for _ in 0..ITERATIONS {
                main_tx.send(transfer_rxs.take().unwrap()).unwrap();
                transfer_rxs = Some(main_rx.recv().unwrap());
            }
        });
    });
}

criterion_group!(
    benches,
    transfer_empty,
    transfer_senders<0>,
    transfer_senders<1>,
    transfer_senders<8>,
    transfer_senders<64>,
    transfer_receivers<0>,
    transfer_receivers<1>,
    transfer_receivers<8>,
    transfer_receivers<64>,
);
criterion_main!(benches);
