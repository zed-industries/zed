//! Benchmarks for asynchronous Semaphore implementations

use criterion::{criterion_group, criterion_main, Benchmark, Criterion};
use futures_intrusive::sync::{
    Semaphore as IntrusiveSemaphore,
    SemaphoreReleaser as IntrusiveSemaphoreReleaser,
};
use tokio::sync::{
    Semaphore as TokioSemaphore, SemaphorePermit as TokioSemaphorePermit,
};

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

mod utils;
use utils::Yield;

/// How often each task should acquire the semaphore
const NR_ACQUIRES: usize = 50;
/// How many tasks are used
const TASKS: usize = 200;

/// The amount of available permits when we are testing strong contention
const CONTENTION_PERMITS: usize = 100;
/// The amount of available permits when testing light contention
const NORMAL_PERMITS: usize = 180;
/// The amount of available permits when testing no contention
const UNCONTENDED_PERMITS: usize = TASKS;

/// The number of yields we perform after the Semaphore was acquired
const NR_YIELDS: usize = 4;

/// Extension trait to add support for `block_on` for runtimes which not
/// natively support it as member function
trait Block {
    fn block_on<F: Future<Output = ()>>(&self, f: F);
}

fn create_intrusive_fair_semaphore(permits: usize) -> IntrusiveSemaphore {
    IntrusiveSemaphore::new(true, permits)
}

fn create_intrusive_unfair_semaphore(permits: usize) -> IntrusiveSemaphore {
    IntrusiveSemaphore::new(false, permits)
}

fn create_tokio_semaphore(permits: usize) -> TokioSemaphore {
    TokioSemaphore::new(permits)
}

async fn acquire_intrusive_semaphore(
    sem: &IntrusiveSemaphore,
) -> IntrusiveSemaphoreReleaser<'_> {
    sem.acquire(1).await
}

async fn acquire_tokio_semaphore(
    sem: &TokioSemaphore,
) -> TokioSemaphorePermit<'_> {
    sem.acquire().await.unwrap()
}

macro_rules! run_with_semaphore {
    (
        $nr_tasks: expr,
        $nr_iterations: expr,
        $nr_permits: expr,
        $spawn_fn: expr,
        $create_semaphore_fn: ident,
        $acquire_fn: ident,
    ) => {
        let semaphore = Arc::new($create_semaphore_fn($nr_permits));
        let mut tasks = Vec::new();
        let sem = Arc::new(IntrusiveSemaphore::new(false, 0));

        for _ in 0..$nr_tasks {
            let semaphore = semaphore.clone();
            let s = sem.clone();
            tasks.push($spawn_fn(async move {
                for _count in 0..$nr_iterations {
                    let _releaser = $acquire_fn(&*semaphore).await;
                    Yield::new(NR_YIELDS).await;
                }
                s.release(1);
            }));
        }

        sem.acquire($nr_tasks).await;
    };
}

macro_rules! bench {
    (
        $b: ident,
        $rt_setup: expr,
        $spawn_fn: expr,
        $nr_iterations: expr,
        $nr_permits: expr,
        $create_semaphore_fn: ident,
        $acquire_fn: ident,
    ) => {
        #[allow(unused_mut)] // mut is only required for some runtimes
        let mut rt = $rt_setup;
        $b.iter(|| {
            rt.block_on(async {
                run_with_semaphore!(
                    TASKS,
                    $nr_iterations,
                    $nr_permits,
                    $spawn_fn,
                    $create_semaphore_fn,
                    $acquire_fn,
                );
            })
        });
    };
}

macro_rules! benchmarks {
    (
        $c: ident,
        $rt_name: literal,
        $rt_setup: expr,
        $spawn_fn: expr,
        $semaphore_name: literal,
        $create_semaphore_fn: ident,
        $acquire_fn: ident,
    ) => {
        $c.bench(
            concat!($rt_name, "/", $semaphore_name),
            Benchmark::new("heavy contention", |b| {
                bench!(
                    b,
                    $rt_setup,
                    $spawn_fn,
                    NR_ACQUIRES,
                    CONTENTION_PERMITS,
                    $create_semaphore_fn,
                    $acquire_fn,
                );
            })
            .with_function("normal contention", |b| {
                bench!(
                    b,
                    $rt_setup,
                    $spawn_fn,
                    NR_ACQUIRES,
                    NORMAL_PERMITS,
                    $create_semaphore_fn,
                    $acquire_fn,
                );
            })
            .with_function("no contention", |b| {
                bench!(
                    b,
                    $rt_setup,
                    $spawn_fn,
                    NR_ACQUIRES,
                    UNCONTENDED_PERMITS,
                    $create_semaphore_fn,
                    $acquire_fn,
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
        create_intrusive_fair_semaphore,
        acquire_intrusive_semaphore,
    );
}

fn tokio_rt_intrusive_unfair_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "tokio_rt",
        tokio::runtime::Runtime::new().unwrap(),
        tokio::spawn,
        "futures_intrusive(fair=false)",
        create_intrusive_unfair_semaphore,
        acquire_intrusive_semaphore,
    );
}

fn tokio_rt_tokio_benchmarks(c: &mut Criterion) {
    benchmarks!(
        c,
        "tokio_rt",
        tokio::runtime::Runtime::new().unwrap(),
        tokio::spawn,
        "tokio",
        create_tokio_semaphore,
        acquire_tokio_semaphore,
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets =
        tokio_rt_intrusive_fair_benchmarks,
        tokio_rt_intrusive_unfair_benchmarks,
        tokio_rt_tokio_benchmarks,
}
criterion_main!(benches);
