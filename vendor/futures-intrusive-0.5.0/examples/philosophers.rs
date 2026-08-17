//! The example in this file demonstrates a solution for the
//! [Dining Philosophers Problem](https://en.wikipedia.org/wiki/Dining_philosophers_problem),
//! which uses async tasks and futures_intrusive primitives in order to
//! simulate philosophers.

#![recursion_limit = "256"]

use futures::{executor::block_on, join, select};
use futures_intrusive::{
    sync::LocalMutex,
    timer::{StdClock, Timer, TimerService},
};
use lazy_static::lazy_static;
use pin_utils::pin_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{sleep, spawn};
use std::time::Duration;

/// We simulate the ownership of a fork through an asynchronously awaitable mutex.
/// In order to acquire a fork, the philosopher acquires the Mutex.
/// In order to release a fork, the philosopher releases the LockGuard. This
/// happens automatically, when the LockGuard goes out of scope.
/// Since all philosophers are subtasks of the same top-level `async` task,
/// a lightweight non thread-safe `LocalMutex` can be utilized.
type Fork = LocalMutex<()>;

// Some setup for the asynchronously awaitable timer
lazy_static! {
    static ref STD_CLOCK: StdClock = StdClock::new();
    static ref TIMER_SERVICE: TimerService = TimerService::new(&*STD_CLOCK);
    static ref STOP_TIMER: AtomicBool = AtomicBool::new(false);
}

/// Returns a reference to the global timer
fn get_timer() -> &'static dyn Timer {
    &*TIMER_SERVICE
}

/// Returns a random delay duration between `min` and `(min + max_extra)`
fn rand_delay(min: Duration, max_extra: Duration) -> Duration {
    let extra_ms = rand::random::<u64>() % (max_extra.as_millis() as u64);
    min + Duration::from_millis(extra_ms)
}

/// How often a philosopher should eat
const TO_EAT: usize = 5;

/// Simulates a single philosopher
async fn philosopher_task<'a>(
    name: &'a str,
    left_fork: &'a Fork,
    right_fork: &'a Fork,
) {
    println!("{} is ready to go", name);
    let mut eaten: usize = 0;

    while eaten != TO_EAT {
        println!("{} is thinking", name);
        get_timer()
            .delay(rand_delay(
                Duration::from_millis(1000),
                Duration::from_millis(1000),
            ))
            .await;
        {
            println!("{} is starting to pick up forks", name);
            // Create futures for acquiring both forks
            let get_left_fork_future = left_fork.lock();
            pin_mut!(get_left_fork_future);
            let get_right_fork_future = right_fork.lock();
            pin_mut!(get_right_fork_future);

            // This sets up a timer. If the philosopher can't obtain both forks
            // during that, they put back all acquired forks and start thinking
            // again.
            let abort_get_forks_future =
                get_timer().delay(Duration::from_millis(300));
            pin_mut!(abort_get_forks_future);

            select! {
                _ = get_left_fork_future => {
                    println!("{} got the left fork and tries to get the right fork", name);

                    select! {
                        _ = get_right_fork_future => {
                            println!("{} got the right fork and starts eating", name);
                            get_timer().delay(
                                rand_delay(Duration::from_millis(1000),
                                Duration::from_millis(200))).await;
                            eaten += 1;
                            println!("{} has finished eating [ate {} times]", name, eaten);
                        },
                        _ = abort_get_forks_future => {
                            println!("{} could not acquire the right fork", name);
                        },
                    }
                },
                _ = get_right_fork_future => {
                    println!("{} got the right fork and tries to get the left fork", name);

                    select! {
                        _ = get_left_fork_future => {
                            println!("{} got the left fork and starts eating", name);
                            get_timer().delay(
                                rand_delay(Duration::from_millis(1000),
                                Duration::from_millis(200))).await;
                            eaten += 1;
                            println!("{} has finished eating [ate {} times]", name, eaten);
                        },
                        _ = abort_get_forks_future => {
                            println!("{} could not acquire the left fork", name);
                        },
                    }
                },
                _ = abort_get_forks_future => {
                    println!("{} could not acquire any fork", name);
                },
            }
        }
    }

    println!("{} has finished", name);
}

async fn simulate_philosophers() {
    // Create the forks for the philosophers
    let forks: [Fork; 5] = [
        Fork::new((), true),
        Fork::new((), true),
        Fork::new((), true),
        Fork::new((), true),
        Fork::new((), true),
    ];

    // Create a task for each philosopher
    let p1 = philosopher_task("A", &forks[4], &forks[0]);
    let p2 = philosopher_task("B", &forks[0], &forks[1]);
    let p3 = philosopher_task("C", &forks[1], &forks[2]);
    let p4 = philosopher_task("D", &forks[2], &forks[3]);
    let p5 = philosopher_task("E", &forks[3], &forks[4]);

    // Wait until all philosophers have finished eating
    join!(p1, p2, p3, p4, p5);
}

fn main() {
    // Spawn a background thread which advances the timer
    let join_handle = spawn(move || {
        timer_thread();
    });

    // And simulate the philosophers
    block_on(simulate_philosophers());

    // Stop the timer thread
    STOP_TIMER.store(true, Ordering::Relaxed);
    join_handle.join().unwrap();
}

fn timer_thread() {
    while !STOP_TIMER.load(Ordering::Relaxed) {
        sleep(Duration::from_millis(25));
        TIMER_SERVICE.check_expirations();
    }
}
