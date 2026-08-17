#[macro_use]
extern crate criterion;

use std::{
    sync::mpsc,
    thread,
    fmt::Debug,
};
use criterion::{Criterion, Bencher, black_box};
use std::time::Instant;

trait Sender: Clone + Send + Sized + 'static {
    type Item: Debug + Default;
    type BoundedSender: Sender<Item=Self::Item>;
    type Receiver: Receiver<Item=Self::Item>;

    fn unbounded() -> (Self, Self::Receiver);
    fn bounded(n: usize) -> (Self::BoundedSender, Self::Receiver);
    fn send(&self, msg: Self::Item);
}

trait Receiver: Send + Sized + 'static {
    type Item: Default;
    fn recv(&self) -> Self::Item;
    fn iter(&self) -> Box<dyn Iterator<Item=Self::Item> + '_>;
}

impl<T: Send + Debug + Default + 'static> Sender for flume::Sender<T> {
    type Item = T;
    type BoundedSender = Self;
    type Receiver = flume::Receiver<T>;

    fn unbounded() -> (Self, Self::Receiver) {
        flume::unbounded()
    }

    fn bounded(n: usize) -> (Self::BoundedSender, Self::Receiver) {
        flume::bounded(n)
    }

    fn send(&self, msg: T) {
        flume::Sender::send(self, msg).unwrap();
    }
}

impl<T: Send + Default + 'static> Receiver for flume::Receiver<T> {
    type Item = T;

    fn recv(&self) -> Self::Item {
        flume::Receiver::recv(self).unwrap()
    }

    fn iter(&self) -> Box<dyn Iterator<Item=T> + '_> {
        Box::new(std::iter::from_fn(move || flume::Receiver::recv(self).ok()))
    }
}

impl<T: Send + Debug + Default + 'static> Sender for crossbeam_channel::Sender<T> {
    type Item = T;
    type BoundedSender = Self;
    type Receiver = crossbeam_channel::Receiver<T>;

    fn unbounded() -> (Self, Self::Receiver) {
        crossbeam_channel::unbounded()
    }

    fn bounded(n: usize) -> (Self::BoundedSender, Self::Receiver) {
        crossbeam_channel::bounded(n)
    }

    fn send(&self, msg: T) {
        crossbeam_channel::Sender::send(self, msg).unwrap();
    }
}

impl<T: Send + Default + 'static> Receiver for crossbeam_channel::Receiver<T> {
    type Item = T;

    fn recv(&self) -> Self::Item {
        crossbeam_channel::Receiver::recv(self).unwrap()
    }

    fn iter(&self) -> Box<dyn Iterator<Item=T> + '_> {
        Box::new(crossbeam_channel::Receiver::iter(self))
    }
}

impl<T: Send + Debug + Default + 'static> Sender for mpsc::Sender<T> {
    type Item = T;
    type BoundedSender = mpsc::SyncSender<T>;
    type Receiver = mpsc::Receiver<T>;

    fn unbounded() -> (Self, Self::Receiver) {
        mpsc::channel()
    }

    fn bounded(n: usize) -> (Self::BoundedSender, Self::Receiver) {
        mpsc::sync_channel(n)
    }

    fn send(&self, msg: T) {
        mpsc::Sender::send(self, msg).unwrap();
    }
}

impl<T: Send + Debug + Default + 'static> Sender for mpsc::SyncSender<T> {
    type Item = T;
    type BoundedSender = Self;
    type Receiver = mpsc::Receiver<T>;

    fn unbounded() -> (Self, Self::Receiver) { unimplemented!() }
    fn bounded(_: usize) -> (Self::BoundedSender, Self::Receiver) { unimplemented!() }

    fn send(&self, msg: T) {
        mpsc::SyncSender::send(self, msg).unwrap();
    }
}

impl<T: Send + Default + 'static> Receiver for mpsc::Receiver<T> {
    type Item = T;

    fn recv(&self) -> Self::Item {
        mpsc::Receiver::recv(self).unwrap()
    }

    fn iter(&self) -> Box<dyn Iterator<Item=T> + '_> {
        Box::new(mpsc::Receiver::iter(self))
    }
}

fn test_create<S: Sender>(b: &mut Bencher) {
    b.iter(|| S::unbounded());
}

fn test_oneshot<S: Sender>(b: &mut Bencher) {
    b.iter(|| {
        let (tx, rx) = S::unbounded();
        tx.send(Default::default());
        black_box(rx.recv());
    });
}

fn test_inout<S: Sender>(b: &mut Bencher) {
    let (tx, rx) = S::unbounded();
    b.iter(|| {
        tx.send(Default::default());
        black_box(rx.recv());
    });
}

fn test_hydra<S: Sender>(b: &mut Bencher, thread_num: usize, msg_num: usize) {
    let (main_tx, main_rx) = S::unbounded();

    let mut txs = Vec::new();
    for _ in 0..thread_num {
        let main_tx = main_tx.clone();
        let (tx, rx) = S::unbounded();
        txs.push(tx);

        thread::spawn(move || {
            for msg in rx.iter() {
                main_tx.send(msg);
            }
        });
    }

    drop(main_tx);

    b.iter(|| {
        for tx in &txs {
            for _ in 0..msg_num {
                tx.send(Default::default());
            }
        }

        for _ in 0..thread_num {
            for _ in 0..msg_num {
                black_box(main_rx.recv());
            }
        }
    });
}

fn test_kitsune<S: Sender>(b: &mut Bencher, thread_num: usize, msg_num: usize)
    where S::Receiver: Clone
{
    let (out_tx, out_rx) = S::unbounded();
    let (in_tx, in_rx) = S::unbounded();

    for _ in 0..thread_num {
        let in_tx = in_tx.clone();
        let out_rx = out_rx.clone();

        thread::spawn(move || {
            for msg in out_rx.iter() {
                in_tx.send(msg);
            }
        });
    }

    b.iter(|| {
        for _ in 0..thread_num {
            for _ in 0..msg_num {
                out_tx.send(Default::default());
            }
        }

        for _ in 0..thread_num {
            for _ in 0..msg_num {
                black_box(in_rx.recv());
            }
        }
    });
}

fn test_robin_u<S: Sender>(b: &mut Bencher, thread_num: usize, msg_num: usize) {
    let (mut main_tx, main_rx) = S::unbounded();

    for _ in 0..thread_num {
        let (mut tx, rx) = S::unbounded();
        std::mem::swap(&mut tx, &mut main_tx);

        thread::spawn(move || {
            for msg in rx.iter() {
                tx.send(msg);
            }
        });
    }

    b.iter(|| {
        for _ in 0..msg_num {
            main_tx.send(Default::default());
        }

        for _ in 0..msg_num {
            black_box(main_rx.recv());
        }
    });
}

fn test_robin_b<S: Sender>(b: &mut Bencher, thread_num: usize, msg_num: usize) {
    let (mut main_tx, main_rx) = S::bounded(1);

    for _ in 0..thread_num {
        let (mut tx, rx) = S::bounded(1);
        std::mem::swap(&mut tx, &mut main_tx);

        thread::spawn(move || {
            for msg in rx.iter() {
                tx.send(msg);
            }
        });
    }

    b.iter(|| {
        let main_tx = main_tx.clone();
        thread::spawn(move || {
            for _ in 0..msg_num {
                main_tx.send(Default::default());
            }
        });

        for _ in 0..msg_num {
            black_box(main_rx.recv());
        }
    });
}

fn test_mpsc_bounded_no_wait<S: Sender>(b: &mut Bencher, thread_num: u64) {
    b.iter_custom(|iters| {
        let iters = iters * 1000;
        let (tx, rx) = S::bounded(iters as usize);
        let start = Instant::now();

        crossbeam_utils::thread::scope(|scope| {
            for _ in 0..thread_num {
                let tx = tx.clone();
                scope.spawn(move |_| {
                    for _ in 0..iters / thread_num {
                        tx.send(Default::default());
                    }
                });
            }

            for _ in 0..iters - ((iters / thread_num) * thread_num) {
                tx.send(Default::default());
            }

            for _ in 0..iters {
                black_box(rx.recv());
            }
        })
            .unwrap();

        start.elapsed()
    })
}

fn test_mpsc_bounded<S: Sender>(b: &mut Bencher, bound: usize, thread_num: usize) {
    b.iter_custom(|iters| {
        let (tx, rx) = S::bounded(bound);
        let start = Instant::now();

        crossbeam_utils::thread::scope(|scope| {
            let msgs = iters as usize * bound.max(1);

            for _ in 0..thread_num {
                let tx = tx.clone();
                scope.spawn(move |_| {
                    for _ in 0..msgs / thread_num as usize {
                        tx.send(Default::default());
                    }
                });
            }

            scope.spawn(move |_| {
                // Remainder
                for _ in 0..msgs - (msgs / thread_num as usize * thread_num)  {
                    tx.send(Default::default());
                }
            });

            for _ in 0..msgs {
                black_box(rx.recv());
            }
        })
            .unwrap();

        start.elapsed()
    })
}

fn create(b: &mut Criterion) {
    b.bench_function("create-flume", |b| test_create::<flume::Sender<u32>>(b));
    b.bench_function("create-crossbeam", |b| test_create::<crossbeam_channel::Sender<u32>>(b));
    b.bench_function("create-std", |b| test_create::<mpsc::Sender<u32>>(b));
}

fn oneshot(b: &mut Criterion) {
    b.bench_function("oneshot-flume", |b| test_oneshot::<flume::Sender<u32>>(b));
    b.bench_function("oneshot-crossbeam", |b| test_oneshot::<crossbeam_channel::Sender<u32>>(b));
    b.bench_function("oneshot-std", |b| test_oneshot::<mpsc::Sender<u32>>(b));
}

fn inout(b: &mut Criterion) {
    b.bench_function("inout-flume", |b| test_inout::<flume::Sender<u32>>(b));
    b.bench_function("inout-crossbeam", |b| test_inout::<crossbeam_channel::Sender<u32>>(b));
    b.bench_function("inout-std", |b| test_inout::<mpsc::Sender<u32>>(b));
}

fn hydra_32t_1m(b: &mut Criterion) {
    b.bench_function("hydra-32t-1m-flume", |b| test_hydra::<flume::Sender<u32>>(b, 32, 1));
    b.bench_function("hydra-32t-1m-crossbeam", |b| test_hydra::<crossbeam_channel::Sender<u32>>(b, 32, 1));
    b.bench_function("hydra-32t-1m-std", |b| test_hydra::<mpsc::Sender<u32>>(b, 32, 1));
}

fn hydra_32t_1000m(b: &mut Criterion) {
    b.bench_function("hydra-32t-1000m-flume", |b| test_hydra::<flume::Sender<u32>>(b, 32, 1000));
    b.bench_function("hydra-32t-1000m-crossbeam", |b| test_hydra::<crossbeam_channel::Sender<u32>>(b, 32, 1000));
    b.bench_function("hydra-32t-1000m-std", |b| test_hydra::<mpsc::Sender<u32>>(b, 32, 1000));
}

fn hydra_256t_1m(b: &mut Criterion) {
    b.bench_function("hydra-256t-1m-flume", |b| test_hydra::<flume::Sender<u32>>(b, 256, 1));
    b.bench_function("hydra-256t-1m-crossbeam", |b| test_hydra::<crossbeam_channel::Sender<u32>>(b, 256, 1));
    b.bench_function("hydra-256t-1m-std", |b| test_hydra::<mpsc::Sender<u32>>(b, 256, 1));
}

fn hydra_1t_1000m(b: &mut Criterion) {
    b.bench_function("hydra-1t-1000m-flume", |b| test_hydra::<flume::Sender<u32>>(b, 1, 1000));
    b.bench_function("hydra-1t-1000m-crossbeam", |b| test_hydra::<crossbeam_channel::Sender<u32>>(b, 1, 1000));
    b.bench_function("hydra-1t-1000m-std", |b| test_hydra::<mpsc::Sender<u32>>(b, 1, 1000));
}

fn hydra_4t_10000m(b: &mut Criterion) {
    b.bench_function("hydra-4t-10000m-flume", |b| test_hydra::<flume::Sender<u32>>(b, 4, 10000));
    b.bench_function("hydra-4t-10000m-crossbeam", |b| test_hydra::<crossbeam_channel::Sender<u32>>(b, 4, 10000));
    b.bench_function("hydra-4t-10000m-std", |b| test_hydra::<mpsc::Sender<u32>>(b, 4, 10000));
}

fn kitsune_32t_1m(b: &mut Criterion) {
    b.bench_function("kitsune-32t-1m-flume", |b| test_kitsune::<flume::Sender<u32>>(b, 32, 1));
    b.bench_function("kitsune-32t-1m-crossbeam", |b| test_kitsune::<crossbeam_channel::Sender<u32>>(b, 32, 1));
    //b.bench_function("kitsune-32t-1m-std", |b| test_kitsune::<mpsc::Sender<u32>>(b, 32, 1));
}

fn kitsune_32t_1000m(b: &mut Criterion) {
    b.bench_function("kitsune-32t-1000m-flume", |b| test_kitsune::<flume::Sender<u32>>(b, 32, 1000));
    b.bench_function("kitsune-32t-1000m-crossbeam", |b| test_kitsune::<crossbeam_channel::Sender<u32>>(b, 32, 1000));
    //b.bench_function("kitsune-32t-1000m-std", |b| test_kitsune::<mpsc::Sender<u32>>(b, 32, 1000));
}

fn kitsune_256t_1m(b: &mut Criterion) {
    b.bench_function("kitsune-256t-1m-flume", |b| test_kitsune::<flume::Sender<u32>>(b, 256, 1));
    b.bench_function("kitsune-256t-1m-crossbeam", |b| test_kitsune::<crossbeam_channel::Sender<u32>>(b, 256, 1));
    //b.bench_function("kitsune-256t-1m-std", |b| test_kitsune::<mpsc::Sender<u32>>(b, 256, 1));
}

fn kitsune_1t_1000m(b: &mut Criterion) {
    b.bench_function("kitsune-1t-1000m-flume", |b| test_kitsune::<flume::Sender<u32>>(b, 1, 1000));
    b.bench_function("kitsune-1t-1000m-crossbeam", |b| test_kitsune::<crossbeam_channel::Sender<u32>>(b, 1, 1000));
    //b.bench_function("kitsune-1t-1000m-std", |b| test_kitsune::<mpsc::Sender<u32>>(b, 1, 1000));
}

fn kitsune_4t_10000m(b: &mut Criterion) {
    b.bench_function("kitsune-4t-10000m-flume", |b| test_kitsune::<flume::Sender<u32>>(b, 4, 10000));
    b.bench_function("kitsune-4t-10000m-crossbeam", |b| test_kitsune::<crossbeam_channel::Sender<u32>>(b, 4, 10000));
    //b.bench_function("kitsune-4t-10000m-std", |b| test_kitsune::<mpsc::Sender<u32>>(b, 4, 10000));
}

fn robin_u_32t_1m(b: &mut Criterion) {
    b.bench_function("robin-u-32t-1m-flume", |b| test_robin_u::<flume::Sender<u32>>(b, 32, 1));
    b.bench_function("robin-u-32t-1m-crossbeam", |b| test_robin_u::<crossbeam_channel::Sender<u32>>(b, 32, 1));
    b.bench_function("robin-u-32t-1m-std", |b| test_robin_u::<mpsc::Sender<u32>>(b, 32, 1));
}

fn robin_u_4t_1000m(b: &mut Criterion) {
    b.bench_function("robin-u-4t-1000m-flume", |b| test_robin_u::<flume::Sender<u32>>(b, 4, 1000));
    b.bench_function("robin-u-4t-1000m-crossbeam", |b| test_robin_u::<crossbeam_channel::Sender<u32>>(b, 4, 1000));
    b.bench_function("robin-u-4t-1000m-std", |b| test_robin_u::<mpsc::Sender<u32>>(b, 4, 1000));
}

fn robin_b_32t_16m(b: &mut Criterion) {
    b.bench_function("robin-b-32t-16m-flume", |b| test_robin_b::<flume::Sender<u32>>(b, 32, 16));
    b.bench_function("robin-b-32t-16m-crossbeam", |b| test_robin_b::<crossbeam_channel::Sender<u32>>(b, 32, 16));
    b.bench_function("robin-b-32t-16m-std", |b| test_robin_b::<mpsc::Sender<u32>>(b, 32, 16));
}

fn robin_b_4t_1000m(b: &mut Criterion) {
    b.bench_function("robin-b-4t-1000m-flume", |b| test_robin_b::<flume::Sender<u32>>(b, 4, 1000));
    b.bench_function("robin-b-4t-1000m-crossbeam", |b| test_robin_b::<crossbeam_channel::Sender<u32>>(b, 4, 1000));
    b.bench_function("robin-b-4t-1000m-std", |b| test_robin_b::<mpsc::Sender<u32>>(b, 4, 1000));
}

fn mpsc_bounded_no_wait_4t(b: &mut Criterion) {
    b.bench_function("mpsc-bounded-no-wait-4t-flume", |b| test_mpsc_bounded_no_wait::<flume::Sender<u32>>(b, 4));
    b.bench_function("mpsc-bounded-no-wait-4t-crossbeam", |b| test_mpsc_bounded_no_wait::<crossbeam_channel::Sender<u32>>(b, 4));
    b.bench_function("mpsc-bounded-no-wait-4t-std", |b| test_mpsc_bounded_no_wait::<mpsc::Sender<u32>>(b, 4));
}

fn mpsc_bounded_4t(b: &mut Criterion) {
    for bound in &[0, 1, 10, 50, 10_000] {
        let text = format!("mpsc-bounded-small-4t-{}m-", bound);
        let bound = *bound;

        b.bench_function(&format!("{}{}", text, "flume"), |b| test_mpsc_bounded::<flume::Sender<u32>>(b, bound, 4));
        b.bench_function(&format!("{}{}", text, "crossbeam"), |b| test_mpsc_bounded::<crossbeam_channel::Sender<u32>>(b, bound, 4));
        b.bench_function(&format!("{}{}", text, "std"), |b| test_mpsc_bounded::<mpsc::Sender<u32>>(b, bound, 4));
    }
}

criterion_group!(
    compare,
    create,
    oneshot,
    inout,
    hydra_32t_1m,
    hydra_32t_1000m,
    hydra_256t_1m,
    hydra_1t_1000m,
    hydra_4t_10000m,
    robin_b_32t_16m,
    robin_b_4t_1000m,
    robin_u_32t_1m,
    robin_u_4t_1000m,
    mpsc_bounded_no_wait_4t,
    mpsc_bounded_4t,
    kitsune_32t_1m,
    kitsune_32t_1000m,
    kitsune_256t_1m,
    kitsune_1t_1000m,
    kitsune_4t_10000m,
);
criterion_main!(compare);
