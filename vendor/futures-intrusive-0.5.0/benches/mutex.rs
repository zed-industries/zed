//! Benchmarks for asynchronous Mutex implementations

use async_std::{sync::Mutex as AsyncStdMutex, task};
use criterion::{criterion_group, criterion_main, Benchmark, Criterion};
use futures_intrusive::sync::{Mutex as IntrusiveMutex, Semaphore};
use tokio::sync::Mutex as TokioMutex;

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

mod utils;
use utils::Yield;

const ITERATIONS: usize = 300;
const CONTENTION_THREADS: usize = 10;
/// With a chance of 25% chance the operation inside the async Mutex blocks,
/// which is emulated by yielding `NR_YIELD` times back to the executor.
const YIELD_CHANCE: usize = 25;
const NR_YIELDS: usize = 10;

/// Extension trait to add support for `block_on` for runtimes which not
/// natively support it as member function
trait Block {
    fn block_on<F: Future<Output = ()>>(&self, f: F);
}

struct FakeAsyncStdRuntime;

impl Block for FakeAsyncStdRuntime {
    fn block_on<F: Future<Output = ()>>(&self, f: F) {
        task::block_on(f);
    }
}

macro_rules! run_with_mutex {
    (
        $mutex_constructor: expr,
        $nr_tasks: expr,
        $nr_iterations: expr,
        $spawn_fn: expr
    ) => {
        let m = Arc::new($mutex_constructor);
        let mut tasks = Vec::new();
        let sem = Arc::new(Semaphore::new(false, 0));

        for _ in 0..$nr_tasks {
            let m = m.clone();
            let s = sem.clone();
            tasks.push($spawn_fn(async move {
                for count in 0..$nr_iterations {
                    let _ = m.lock().await;
                    // Asynchronous mutexes are intended to guard over
                    // operations which are potentially task-blocking and take
                    // a certain amount of time to complete. In order to simulate
                    // the behavior we yield a certain amount of times to back
                    // to the executor. This is more consistent than e.g. using
                    // a timer, and the overhead of yielding is the same for the
                    // various Mutex implementations.

                    if YIELD_CHANCE != 0 && (count % (100 / YIELD_CHANCE) == 0) {
                        Yield::new(NR_YIELDS).await;
                    }
                }
                s.release(1);
            }));
        }

        sem.acquire($nr_tasks).await;
    };
}

macro_rules! contention {
    (
        $b: ident,
        $rt_setup: expr, $spawn_fn: expr,
        $mutex_constructor: expr, $nr_iterations: expr
    ) => {
        #[allow(unused_mut)] // mut is only required for some runtimes
        let mut rt = $rt_setup;
        $b.iter(|| {
            rt.block_on(async {
                run_with_mutex!(
                    $mutex_constructor,
                    CONTENTION_THREADS,
                    $nr_iterations,
                    $spawn_fn
                );
            })
        });
    };
}

macro_rules! no_contention {
    (
        $b: ident,
        $rt_setup: expr, $spawn_fn: expr,
        $mutex_constructor: expr, $nr_iterations: expr
    ) => {
        #[allow(unused_mut)] // mut is only required for some runtimes
        let mut rt = $rt_setup;
        $b.iter(|| {
            rt.block_on(async {
                run_with_mutex!(
                    $mutex_constructor,
                    1,
                    $nr_iterations,
                    $spawn_fn
                );
            })
        });
    };
}

macro_rules! benchmarks {
    (
        $c: ident,
        $rt_name: literal, $rt_setup: expr, $spawn_fn: expr,
        $mutex_name: literal, $mutex_constructor: expr
    ) => {
        $c.bench(
            concat!($rt_name, "/", $mutex_name),
            Benchmark::new("contention", |b| {
                contention!(
                    b,
                    $rt_setup,
                    $spawn_fn,
                    $mutex_constructor,
                    ITERATIONS
                );
            })
            .with_function("no_contention", |b| {
                no_contention!(
                    b,
                    $rt_setup,
                    $spawn_fn,
                    $mutex_constructor,
                    ITERATIONS
                );
            }),
        );
    };
}

fn tokio_rt_intrusive_fair_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "tokio_rt",
        tokio::runtime::Runtime::new().unwrap(),
        tokio::spawn,
        "futures_intrusive(fair=true)",
        IntrusiveMutex::new((), true)
    );
}

fn tokio_rt_intrusive_unfair_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "tokio_rt",
        tokio::runtime::Runtime::new().unwrap(),
        tokio::spawn,
        "futures_intrusive(fair=false)",
        IntrusiveMutex::new((), false)
    );
}

fn tokio_rt_async_std_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "tokio_rt",
        tokio::runtime::Runtime::new().unwrap(),
        tokio::spawn,
        "async_std",
        AsyncStdMutex::new(())
    );
}

fn tokio_rt_tokio_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "tokio_rt",
        tokio::runtime::Runtime::new().unwrap(),
        tokio::spawn,
        "tokio",
        TokioMutex::new(())
    );
}

fn async_std_intrusive_fair_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "async_std_rt",
        FakeAsyncStdRuntime {},
        task::spawn,
        "futures_intrusive(fair=true)",
        IntrusiveMutex::new((), true)
    );
}

fn async_std_intrusive_unfair_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "async_std_rt",
        FakeAsyncStdRuntime {},
        task::spawn,
        "futures_intrusive(fair=false)",
        IntrusiveMutex::new((), false)
    );
}

fn async_std_async_std_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "async_std_rt",
        FakeAsyncStdRuntime {},
        task::spawn,
        "async_std",
        AsyncStdMutex::new(())
    );
}

fn async_std_tokio_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "async_std_rt",
        FakeAsyncStdRuntime {},
        task::spawn,
        "tokio",
        TokioMutex::new(())
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets =
        // tokio
        tokio_rt_intrusive_fair_benchmarks,
        tokio_rt_intrusive_unfair_benchmarks,
        tokio_rt_async_std_benchmarks,
        tokio_rt_tokio_benchmarks,
        // async-std
        async_std_intrusive_fair_benchmarks,
        async_std_intrusive_unfair_benchmarks,
        async_std_async_std_benchmarks,
        async_std_tokio_benchmarks
}
criterion_main!(benches);
