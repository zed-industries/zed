//! This example demonstrates the application of structured concurrency and
//! gracefully cancellation in an async Rust application.
//! An async [`ManualResetEvent`] as provided by `futures-intrusive` is a used
//! as the main signalization mechanism for cooperative cancellation.
//!
//! Usage: cargo run --example cancellation
//! After some seconds, press Ctrl+C and observe the results
//!
//! Structured concurrency is an application model where the lifetime of any
//! concurrent operation is strictly contained within the lifetime of it's
//! parent operation.
//!
//! The concept is described in further detail within
//! https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/
//! https://trio.discourse.group/t/structured-concurrency-kickoff/55
//!
//! The application of structured concurrency principles simplifies concurrent
//! program. It allows for an easier reasoning about which concurrent tasks run
//! at a given point of time, since no subtask will ever run without it's original
//! parent task already having finished. This makes it impossible for the subtask
//! to wait on a certain condition that will no longer occur - or to modify the
//! state of the program when we no longer expect it.
//!
//! One challenge for structured concurrency is the graceful cancellation of
//! subtasks. Within Rusts `Future`s and `async/await` programming model it is
//! generally easy to stop asynchronous subtasks: We can just `drop` their
//! associated `Future`s, which will cancel those tasks. However this foceful
//! cancellation comes with several downsides:
//! - The subtasks can't perform any cleanup work anymore that might be helpful.
//!   Only code inside their destructors can run if the tasks are cancelled.
//! - The subtasks can't return any value.
//!
//! Therefore a cooperative and graceful cancellation is sometimes preferred. In
//! this example we implement graceful cancellation in order to allow a sub task
//! to return it's calculated values.
//!
//! Graceful cancellation is implemented in 3 steps:
//! 1. Signalling the cancellation: One component signals the sub-tasks that they
//!   should stop their work as soon as it is convenient for them. The
//!   cancellation signal can either originate from a parent task, the sub task
//!   itself, or one of the sibling tasks. In order to distribute cancellation
//!   signals we utilize an async `ManualResetEvent` as a cancellation token.
//!   This datastructure allows to signal an arbitrary amount of tasks.
//!   The signal can be emitted by any component which has access to
//!   `ManualResetEvent`.
//! 2. Detecting the signal inside sub-tasks and shutting down. In order to
//!   support graceful cancellation, subtasks need to detect the condition that
//!   they are supposed to shut down. In order to do this we use the futures-rs
//!   `select!` macro to wait in parallel for either the async calculation on
//!   the "normal path" to complete or for the cancellation to get signalled.
//!   Not all subtasks have to explicitly support this. Some of them just need
//!   to forward the cancellation token to their child tasks. When these finish
//!   early due to cancellation, then the parent will also finish early.
//!   Child tasks can return an error result in order to indicate that they have
//!   returned due the explicit cancellation. E.g. `Err(Cancelled)` could be
//!   returned to the parent.
//! 3. The parent tasks waits for all sub-tasks to shut down, via waiting on
//!   their wait-handles (which in our case are `Future`s that can be awaited
//!   via `await` or various `join` functions).
//!
//! After these steps have completed all sub tasks of a given parent have
//! completed and the parent task can also finish. It can thereby return the
//! results of the child tasks if required.
//!
//! The implementation is similar in spirit to cancellation in the Go programming
//! language trough the Context parameter (https://blog.golang.org/context).
//! The main difference is that a `ManualResetEvent` is used for signalling
//! cancellation instead of a `Channel` - and that we can check for the
//! cancellation signal on every `await` of a `Future`. Checking for cancellation
//! is not constrained to interaction with `Channel` types.
//! E.g. we can easily wait on receiving data on a socket while in parallel
//! waiting for cancellation. This is not directly possible in Go.
//!
//! It also similar to the `CancellationToken` mechanism in .NET. There the
//! `CancellationToken` also needs to get forwarded as a parameter.
//!
//! This example demonstrates the mechanisms via a distributed "FizzBuzz" checker.
//! The "algorithm" uses a parent tasks which uses 2 child tasks for it's work.
//! When the user cancels the program, a graceful shutdown as described should
//! be performed. This allows the user to retrieve the results of the algorithm.

use futures::{executor::block_on, join, select};
use futures_intrusive::{
    channel::LocalUnbufferedChannel,
    sync::{LocalManualResetEvent, ManualResetEvent},
    timer::{StdClock, Timer, TimerService},
};
use lazy_static::lazy_static;
use signal_hook;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{sleep, spawn},
    time::Duration,
};

/// The result of our search for FizzBuzz values
#[derive(Debug, Default)]
struct SearchResult {
    highest_fizz: Option<usize>,
    highest_buzz: Option<usize>,
    highest_fizzbuzz: Option<usize>,
}

/// This is our main async function that supports cooperative cancellation.
/// The purpose of this function is to check values up to `max` for their
/// fizzbuzzness and return the highest values in each category.
///
/// The method can be be cancelled by signalling the cancellation token. In this
/// case the method will return its latest findings.
/// This is in contrast to just cancelling a `Future` - which would not allow us
/// to return any results. Cancellation tokens can be passed as `Arc<ManualResetEvent>`
/// if multiple independent subtasks need to get cancelled, or as a plain reference
/// if only subtasks of a single task need to get signalled. For tasks which run
/// on a singlethreaded executor `LocalManualResetEvent` provides an even higher
/// lower overhead solution which does not require any internal synchronization.
async fn fizzbuzz_search(
    max: usize,
    cancellation_token: Arc<ManualResetEvent>,
) -> SearchResult {
    // We start two child-tasks:
    // - One produces values to check
    // - The other task will check the values and store the results in the
    //   result data structure.
    //
    // Both tasks are connected via a channel. Since the tasks are running as
    // subtasks of the same task in a singlethreaded executor, we can use an
    // extremely efficient LocalChannel for this.
    //
    // In order to make things a bit more interesting we do not utilize the same
    // cancellation signal for both tasks (which would also be a valid solution).
    // Instead we implement a sequential shutdown:
    // - When the main `cancellation_token` is signalled from the outside,
    //   only the producer task will shut down.
    // - Before the producer task exits, it will signal another cancellation
    //   token. That one will lead the checker task to shut down.
    let channel = LocalUnbufferedChannel::<usize>::new();
    let checker_cancellation_token = LocalManualResetEvent::new(false);
    let producer_future = producer_task(
        max,
        &channel,
        &cancellation_token,
        &checker_cancellation_token,
    );
    let checker_future = check_task(&channel, &checker_cancellation_token);

    // Here we wait for both tasks to complete. Waiting for all subtasks to
    // complete is one important part of structured concurrency.
    let results = join!(producer_future, checker_future);
    println!("All subtasks have completed");

    // Since we waited for all subtasks to complete we can return the search
    // result.
    // If the async subtasks had been forcefully instead of cooperatively
    // cancelled the results would not have been available.
    results.1
}

/// The producing task produces all values that need to get checked for
/// fizzbuzzness.
/// The task will run until it either has generated all values to check or
/// until the task gets cancelled.
async fn producer_task(
    max: usize,
    channel: &LocalUnbufferedChannel<usize>,
    main_cancellation_token: &ManualResetEvent,
    consumer_cancellation_token: &LocalManualResetEvent,
) {
    for value in 1..max {
        select! {
            result = channel.send(value) => {
                if !result.is_ok() {
                    unreachable!("This can not happen in this example");
                }
            },
            _ = main_cancellation_token.wait() => {
                // The operation was cancelled
                break;
            }
        };
    }

    // No more values to check or we had been cancelled.
    // In this case we signal the `cancellation_token`, in order to let the
    // consumer shut down.
    // We should here have alternatively `.close()`d the channel to signal the
    // consumer to join. However we want mainly want to demonstrate the
    // cancellation concept here.
    println!("Goodbye from the producer. Now signalling the checker");
    consumer_cancellation_token.set();
}

/// The check task runs until it gets cancelled. That can happen either due
/// to a cancellation being signalled, or due to the input channel getting
/// closed. In a real application one of those strategies would be good sufficient.
/// Since this example focusses on cancellation and structured concurrency, this
/// task will **always** get shut down via the cancellation token.
///
/// It is important that this tasks runs to completion instead of getting
/// forcefully cancelled. Otherwise no results would be available.
async fn check_task(
    channel: &LocalUnbufferedChannel<usize>,
    cancellation_token: &LocalManualResetEvent,
) -> SearchResult {
    // Initialize the result with `None`s
    let mut result: SearchResult = Default::default();

    loop {
        select! {
            value = channel.receive() => {
                if let Some(value) = value {
                    // Received a value that needs to get checked for fizzbuzzness
                    println!("Checking {} of fizzbuzzness", value);
                    match (value % 3 == 0, value % 5 == 0) {
                        (true, true) => result.highest_fizzbuzz = Some(value),
                        (true, false) => result.highest_fizz = Some(value),
                        (false, true) => result.highest_buzz = Some(value),
                        _ => {},
                    }
                } else {
                    unreachable!("this is not allowed in this example");
                    // Otherwise just doing the following here would be ok:
                    // break;
                }
            },
            _ = cancellation_token.wait() => {
                // The operation was cancelled
                break;
            }
        };

        // Waits until the timer elapses or the task gets cancelled - whatever
        // comes first. This slows down our consumer, and introduces another
        // cancellation point. Since we use an unbuffered channel to accept
        // values to check from the producer, the producer is slowed down by
        // the same amount of time.
        select! {
            _ = get_timer().delay(Duration::from_millis(1000)) => {},
            _ = cancellation_token.wait() => {
                // The operation was cancelled
                break;
            },
        }
    }

    println!("Goodbye from the checker");
    result
}

fn main() {
    // Spawn a background thread which advances the timer
    let timer_join_handle = spawn(move || {
        timer_thread();
    });

    // This is the asynchronous ManualResetEvent that will be used as a cancellation
    // token. When the cancellation is requested, the token will be set. Thereby
    // all tasks which are waiting for cancellation will get signalled and awoken.
    let cancellation_token = Arc::new(ManualResetEvent::new(false));

    // This sets up a signal listener. When SIGINT (Ctrl+C) is signalled,
    // the Cancellation Token is set - which will lead the async task to run
    // to completion. Since setting the cancellation token is not signal safe,
    // we apply a workaround and set only an atomic variable in the signal handler.
    // A background thread regularly checks the signal and sets the event once
    // the signal had been observed.
    let cloned_token = cancellation_token.clone(); // Clone for the background thread
    std::thread::spawn(move || {
        let term = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term))
            .unwrap();
        while !term.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(100));
        }
        println!("Starting cancellation");
        cloned_token.set();
    });

    // Start our async task. This gets the cancellation token passed as argument
    let result = block_on(fizzbuzz_search(std::usize::MAX, cancellation_token));
    // At this point in time, the task has finished - either due to running to
    // completion or due to being cancelled. The task can return results in both
    // situations.

    println!("Discovered these awesome results: {:?}", result);

    // Stop the timer thread
    STOP_TIMER.store(true, Ordering::Relaxed);
    timer_join_handle.join().unwrap();
}

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

/// A background thread that drives the async timer service
fn timer_thread() {
    while !STOP_TIMER.load(Ordering::Relaxed) {
        sleep(Duration::from_millis(25));
        TIMER_SERVICE.check_expirations();
    }
}
