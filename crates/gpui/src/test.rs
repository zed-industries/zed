//! Test support for GPUI.
//!
//! GPUI provides first-class support for testing, which includes a macro to run test that rely on having a context,
//! and a test implementation of the `ForegroundExecutor` and `BackgroundExecutor` which ensure that your tests run
//! deterministically even in the face of arbitrary parallelism.
//!
//! The output of the `gpui::test` macro is understood by other rust test runners, so you can use it with `cargo test`
//! or `cargo-nextest`, or another runner of your choice.
//!
//! To make it possible to test collaborative user interfaces (like Zed) you can ask for as many different contexts
//! as you need.
//!
//! ## Example
//!
//! ```
//! use gpui;
//!
//! #[gpui::test]
//! async fn test_example(cx: &TestAppContext) {
//!   assert!(true)
//! }
//!
//! #[gpui::test]
//! async fn test_collaboration_example(cx_a: &TestAppContext, cx_b: &TestAppContext) {
//!   assert!(true)
//! }
//! ```
use crate::{Entity, Subscription, TestAppContext, TestDispatcher};
use futures::StreamExt as _;
use rand::prelude::*;
use smol::channel;
use std::{
    env,
    panic::{self, RefUnwindSafe},
    pin::Pin,
};

/// Run the given test function with the configured parameters.
/// This is intended for use with the `gpui::test` macro
/// and generally should not be used directly.
pub fn run_test(
    mut num_iterations: u64,
    max_retries: usize,
    test_fn: &mut (dyn RefUnwindSafe + Fn(TestDispatcher, u64)),
    on_fail_fn: Option<fn()>,
) {
    let starting_seed = env::var("SEED")
        .map(|seed| seed.parse().expect("invalid SEED variable"))
        .unwrap_or(0);
    if let Ok(iterations) = env::var("ITERATIONS") {
        num_iterations = iterations.parse().expect("invalid ITERATIONS variable");
    }
    let is_randomized = num_iterations > 1;

    for seed in starting_seed..starting_seed + num_iterations {
        let mut retry = 0;
        loop {
            if is_randomized {
                eprintln!("seed = {seed}");
            }
            let result = panic::catch_unwind(|| {
                let dispatcher = TestDispatcher::new(StdRng::seed_from_u64(seed));
                test_fn(dispatcher, seed);
            });

            match result {
                Ok(_) => break,
                Err(error) => {
                    if retry < max_retries {
                        println!("retrying: attempt {}", retry);
                        retry += 1;
                    } else {
                        if is_randomized {
                            eprintln!("failing seed: {}", seed);
                        }
                        if let Some(f) = on_fail_fn {
                            f()
                        }
                        panic::resume_unwind(error);
                    }
                }
            }
        }
    }
}

/// A test struct for converting an observation callback into a stream.
pub struct Observation<T> {
    rx: Pin<Box<channel::Receiver<T>>>,
    _subscription: Subscription,
}

impl<T: 'static> futures::Stream for Observation<T> {
    type Item = T;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

/// observe returns a stream of the change events from the given `View` or `Model`
pub fn observe<T: 'static>(entity: &impl Entity<T>, cx: &mut TestAppContext) -> Observation<()> {
    let (tx, rx) = smol::channel::unbounded();
    let _subscription = cx.update(|cx| {
        cx.observe(entity, move |_, _| {
            let _ = smol::block_on(tx.send(()));
        })
    });
    let rx = Box::pin(rx);

    Observation { rx, _subscription }
}
