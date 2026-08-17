#![allow(clippy::identity_op)]
use criterion::{criterion_group, criterion_main, Criterion};
use ipc_channel::platform;
use std::sync::{mpsc, Mutex};

/// Allows doing multiple inner iterations per bench.iter() run.
///
/// This is mostly to amortise the overhead of spawning a thread in the benchmark
/// when sending larger messages (that might be fragmented).
///
/// Note that you need to compensate the displayed results
/// for the proportionally longer runs yourself,
/// as the benchmark framework doesn't know about the inner iterations...
const ITERATIONS: usize = 1;

fn create_channel(criterion: &mut Criterion) {
    criterion.bench_function("create_channel", |bencher| {
        bencher.iter(|| {
            for _ in 0..ITERATIONS {
                platform::channel().unwrap();
            }
        });
    });
}

fn transfer_data<const SIZE: usize>(criterion: &mut Criterion) {
    criterion.bench_function(&format!("transfer_data_{SIZE}"), |bencher| {
        let data: Vec<u8> = (0..SIZE).map(|i| (i % 251) as u8).collect();
        let (tx, rx) = platform::channel().unwrap();

        let (wait_tx, wait_rx) = mpsc::channel();
        let wait_rx = Mutex::new(wait_rx);

        if SIZE > platform::OsIpcSender::get_max_fragment_size() {
            bencher.iter(|| {
                crossbeam_utils::thread::scope(|scope| {
                    let tx = tx.clone();
                    scope.spawn(|_| {
                        let wait_rx = wait_rx.lock().unwrap();
                        let tx = tx;
                        for _ in 0..ITERATIONS {
                            tx.send(&data, vec![], vec![]).unwrap();
                            if ITERATIONS > 1 {
                                // Prevent beginning of the next send
                                // from overlapping with receive of last fragment,
                                // as otherwise results of runs with a large tail fragment
                                // are significantly skewed.
                                wait_rx.recv().unwrap();
                            }
                        }
                    });
                    for _ in 0..ITERATIONS {
                        rx.recv().unwrap();
                        if ITERATIONS > 1 {
                            wait_tx.send(()).unwrap();
                        }
                    }
                    // For reasons mysterious to me,
                    // not returning a value *from every branch*
                    // adds some 100 ns or so of overhead to all results --
                    // which is quite significant for very short tests...
                    0
                })
            });
        } else {
            bencher.iter(|| {
                for _ in 0..ITERATIONS {
                    tx.send(&data, vec![], vec![]).unwrap();
                    rx.recv().unwrap();
                }
                0
            });
        }
    });
}

criterion_group!(
    benches,
    create_channel,
    transfer_data<1>,
    transfer_data<2>,
    transfer_data<4>,
    transfer_data<8>,
    transfer_data<16>,
    transfer_data<32>,
    transfer_data<64>,
    transfer_data<128>,
    transfer_data<256>,
    transfer_data<512>,
    transfer_data<{ 1 * 1024 }>,
    transfer_data<{ 2 * 1024 }>,
    transfer_data<{ 4 * 1024 }>,
    transfer_data<{ 8 * 1024 }>,
    transfer_data<{ 16 * 1024 }>,
    transfer_data<{ 32 * 1024 }>,
    transfer_data<{ 64 * 1024 }>,
    transfer_data<{ 128 * 1024 }>,
    transfer_data<{ 256 * 1024 }>,
    transfer_data<{ 512 * 1024 }>,
    transfer_data<{ 1 * 1024 * 1024 }>,
    transfer_data<{ 2 * 1024 * 1024 }>,
    transfer_data<{ 4 * 1024 * 1024 }>,
    transfer_data<{ 8 * 1024 * 1024 }>,
);
criterion_main!(benches);
