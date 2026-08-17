macro_rules! create_two_threads_benchmark {
    ($($id:literal, $create:expr, $push:expr, $pop:expr);+) => {

use std::convert::TryInto as _;
use std::sync::{Arc, Barrier};

use criterion::{black_box, criterion_group, criterion_main};

fn help_with_type_inference<P, C, Create, Push, Pop>(create: Create, push: Push, pop: Pop) -> (Create, Push, Pop)
where
    Create: Fn(usize) -> (P, C),
    Push: Fn(&mut P, u8) -> bool,
    Pop: Fn(&mut C) -> Option<u8>,
{
    (create, push, pop)
}

#[allow(unused)]
fn criterion_benchmark(criterion: &mut criterion::Criterion) {
$(
    let (create, push, pop) = help_with_type_inference($create, $push, $pop);
    // Just a quick check if the ring buffer works as expected:
    let (mut p, mut c) = create(2);
    assert!(pop(&mut c).is_none());
    assert!(push(&mut p, 1));
    assert!(push(&mut p, 2));
    assert!(!push(&mut p, 3));
    assert_eq!(pop(&mut c).unwrap(), 1);
    assert_eq!(pop(&mut c).unwrap(), 2);
    assert!(pop(&mut c).is_none());
)+

    let mut group_large = criterion.benchmark_group("two-threads-large");
    group_large.throughput(criterion::Throughput::Bytes(1));
$(
    group_large.bench_function($id, |b| {
        b.iter_custom(|iters| {
            let (create, push, pop) = help_with_type_inference($create, $push, $pop);
            // Queue is so long that there is no contention between threads.
            let (mut p, mut c) = create((2 * iters).try_into().unwrap());
            for i in 0..iters {
                push(&mut p, i as u8);
            }
            let barrier = Arc::new(Barrier::new(3));
            let push_thread = {
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    barrier.wait();
                    let start_pushing = std::time::Instant::now();
                    for i in 0..iters {
                        // NB: This conversion truncates:
                        push(&mut p, i as u8);
                    }
                    let stop_pushing = std::time::Instant::now();
                    (start_pushing, stop_pushing)
                })
            };
            let trigger_thread = {
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    // Try to force other threads to go to sleep on barrier.
                    std::thread::yield_now();
                    std::thread::yield_now();
                    std::thread::yield_now();
                    barrier.wait();
                    // Hopefully, the other two threads now wake up at the same time.
                })
            };
            barrier.wait();
            let start_popping = std::time::Instant::now();
            for _ in 0..iters {
                black_box(pop(&mut c));
            }
            let stop_popping = std::time::Instant::now();
            let (start_pushing, stop_pushing) = push_thread.join().unwrap();
            trigger_thread.join().unwrap();
            let total = stop_pushing
                .max(stop_popping)
                .duration_since(start_pushing.min(start_popping));

            /*
            if start_pushing < start_popping {
                println!(
                    "popping started {:?} after pushing",
                    start_popping.duration_since(start_pushing)
                );
            } else {
                println!(
                    "pushing started {:?} after popping",
                    start_pushing.duration_since(start_popping)
                );
            }
            */

            // The goal is that both threads are finished at around the same time.
            // This can be checked with the following output.
            /*
            if stop_pushing < stop_popping {
                let diff = stop_popping.duration_since(stop_pushing);
                println!(
                    "popping stopped {diff:?} after pushing ({:.1}% of total time)",
                    (diff.as_secs_f64() / total.as_secs_f64()) * 100.0
                );
            } else {
                let diff = stop_pushing.duration_since(stop_popping);
                println!(
                    "pushing stopped {diff:?} after popping ({:.1}% of total time)",
                    (diff.as_secs_f64() / total.as_secs_f64()) * 100.0
                );
            }
            */

            #[allow(clippy::let_and_return)]
            total
        });
    });
)+
    group_large.finish();

    let mut group_small = criterion.benchmark_group("two-threads-small");
    group_small.throughput(criterion::Throughput::Bytes(1));
$(
    group_small.bench_function($id, |b| {
        b.iter_custom(|iters| {
            let (create, push, pop) = help_with_type_inference($create, $push, $pop);
            // Queue is very short in order to force a lot of contention between threads.
            let (mut p, mut c) = create(2);
            let push_thread = {
                std::thread::spawn(move || {
                    // The timing starts once both threads are ready.
                    let start = std::time::Instant::now();
                    for i in 0..iters {
                        while !push(&mut p, i as u8) {
                            std::hint::spin_loop();
                        }
                    }
                    start
                })
            };
            // While the second thread is still starting up, this thread will busy-wait.
            for i in 0..iters {
                loop {
                    if let Some(x) = pop(&mut c) {
                        assert_eq!(x, i as u8);
                        break;
                    }
                    std::hint::spin_loop();
                }
            }
            // The timing stops once all items have been received.
            let stop = std::time::Instant::now();
            let start = push_thread.join().unwrap();
            stop.duration_since(start)
        });
    });
)+
    group_small.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

    };
}

create_two_threads_benchmark!(
    "rtrb",
    rtrb::RingBuffer::new,
    |p, i| p.push(i).is_ok(),
    |c| c.pop().ok()
);
