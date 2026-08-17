/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Test time/sleep implementations that work by manually advancing time with a `tick()`
//!
//! # Examples
//!
//! Spawning a task that creates new sleep tasks and waits for them sequentially,
//! and advancing passed all of them with a single call to `tick()`.
//!
//! ```rust,no_run
//! use std::time::{Duration, SystemTime};
//! use aws_smithy_async::test_util::tick_advance_sleep::tick_advance_time_and_sleep;
//! use aws_smithy_async::time::TimeSource;
//! use aws_smithy_async::rt::sleep::AsyncSleep;
//!
//! # async fn example() {
//! // Create the test time/sleep implementations.
//! // They will start at SystemTime::UNIX_EPOCH.
//! let (time, sleep) = tick_advance_time_and_sleep();
//!
//! // Spawn the task that sequentially sleeps
//! let task = tokio::spawn(async move {
//!     sleep.sleep(Duration::from_secs(1)).await;
//!     sleep.sleep(Duration::from_secs(2)).await;
//!     sleep.sleep(Duration::from_secs(3)).await;
//! });
//! // Verify that task hasn't done anything yet since we haven't called `tick`
//! tokio::task::yield_now().await;
//! assert!(!task.is_finished());
//! assert_eq!(SystemTime::UNIX_EPOCH, time.now());
//!
//! // Tick 6 seconds, which is long enough to go passed all the sequential sleeps
//! time.tick(Duration::from_secs(6)).await;
//! assert_eq!(SystemTime::UNIX_EPOCH + Duration::from_secs(6), time.now());
//!
//! // Verify the task joins, indicating all the sleeps are done
//! task.await.unwrap();
//! # }
//! ```

use crate::{
    rt::sleep::{AsyncSleep, Sleep},
    time::TimeSource,
};
use std::{
    future::IntoFuture,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
struct QueuedSleep {
    /// Duration since `UNIX_EPOCH` at which point the sleep is finished.
    presents_at: Duration,
    notify: Option<Sender<()>>,
}

#[derive(Default, Debug)]
struct Inner {
    // Need to use a Vec since VecDeque doesn't have sort functions,
    // and BTreeSet doesn't fit since we could have more than one sleep presenting
    // at the same time (and there's no way to compare the notify channels).
    sleeps: Vec<QueuedSleep>,
    /// Duration since `UNIX_EPOCH` that represents "now".
    now: Duration,
}

impl Inner {
    fn push(&mut self, sleep: QueuedSleep) {
        self.sleeps.push(sleep);
        self.sleeps.sort_by_key(|s| s.presents_at);
    }

    fn next_presenting(&mut self, time: Duration) -> Option<QueuedSleep> {
        if self
            .sleeps
            .first()
            .map(|f| f.presents_at <= time)
            .unwrap_or(false)
        {
            Some(self.sleeps.remove(0))
        } else {
            None
        }
    }
}

#[derive(Clone, Default, Debug)]
struct SharedInner {
    inner: Arc<Mutex<Inner>>,
}
impl SharedInner {
    fn get(&self) -> impl Deref<Target = Inner> + '_ {
        self.inner.lock().unwrap()
    }
    fn get_mut(&self) -> impl DerefMut<Target = Inner> + '_ {
        self.inner.lock().unwrap()
    }
}

/// Tick-advancing test sleep implementation.
///
/// See [module docs](crate::test_util::tick_advance_sleep) for more information.
#[derive(Clone, Debug)]
pub struct TickAdvanceSleep {
    inner: SharedInner,
}

impl AsyncSleep for TickAdvanceSleep {
    fn sleep(&self, duration: Duration) -> Sleep {
        // Use a one-shot channel to block the sleep future until `TickAdvanceTime::tick`
        // chooses to complete it by sending with the receiver.
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();

        let mut inner = self.inner.get_mut();
        let now = inner.now;

        // Add the sleep to the queue, which `TickAdvanceTime` will examine when ticking.
        inner.push(QueuedSleep {
            presents_at: now + duration,
            notify: Some(tx),
        });

        Sleep::new(async move {
            let _ = rx.into_future().await;
        })
    }
}

/// Tick-advancing test time source implementation.
///
/// See [module docs](crate::test_util::tick_advance_sleep) for more information.
#[derive(Clone, Debug)]
pub struct TickAdvanceTime {
    inner: SharedInner,
}

impl TickAdvanceTime {
    /// Advance time by `duration`.
    ///
    /// This will yield the async runtime after each sleep that presents between
    /// the previous current time and the time after the given duration. This allows
    /// for async tasks pending one of those sleeps to do some work and also create
    /// additional sleep tasks. Created sleep tasks may also complete during this
    /// call to `tick()` if they present before the given time duration.
    pub async fn tick(&self, duration: Duration) {
        let time = self.inner.get().now + duration;

        // Tick to each individual sleep time and yield the runtime so that any
        // futures waiting on a sleep run before futures waiting on a later sleep.
        //
        // We also need to recheck the list of queued sleeps every iteration since
        // unblocked tasks could have queued up more sleeps, and these sleeps may also
        // need to present before ones that were previously queued.
        loop {
            // Can't do `while let` since that holds the lock open
            let Some(mut presenting) = self.inner.get_mut().next_presenting(time) else {
                break;
            };

            // Make sure the time is always accurate for async code that runs
            // after completing the sleep.
            self.inner.get_mut().now = presenting.presents_at;

            // Notify the sleep, and then yield to let work blocked on that sleep to proceed
            let _ = presenting.notify.take().unwrap().send(());
            tokio::task::yield_now().await;
        }

        // Set the final time.
        self.inner.get_mut().now = time;
    }
}

impl TimeSource for TickAdvanceTime {
    fn now(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + self.inner.get().now
    }
}

/// Creates tick-advancing test time/sleep implementations.
///
/// See [module docs](crate::test_util::tick_advance_sleep) for more information.
pub fn tick_advance_time_and_sleep() -> (TickAdvanceTime, TickAdvanceSleep) {
    let inner = SharedInner::default();
    (
        TickAdvanceTime {
            inner: inner.clone(),
        },
        TickAdvanceSleep {
            inner: inner.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::FutureExt;

    #[tokio::test]
    async fn tick_advances() {
        let (time, sleep) = tick_advance_time_and_sleep();

        assert_eq!(SystemTime::UNIX_EPOCH, time.now());
        time.tick(Duration::from_secs(1)).await;
        assert_eq!(SystemTime::UNIX_EPOCH + Duration::from_secs(1), time.now());

        let sleeps = vec![
            tokio::spawn(sleep.sleep(Duration::from_millis(500))),
            tokio::spawn(sleep.sleep(Duration::from_secs(1))),
            tokio::spawn(sleep.sleep(Duration::from_secs(2))),
            tokio::spawn(sleep.sleep(Duration::from_secs(3))),
            tokio::spawn(sleep.sleep(Duration::from_secs(4))),
        ];

        tokio::task::yield_now().await;
        for sleep in &sleeps {
            assert!(!sleep.is_finished());
        }

        time.tick(Duration::from_secs(1)).await;
        assert_eq!(SystemTime::UNIX_EPOCH + Duration::from_secs(2), time.now());
        assert!(sleeps[0].is_finished());
        assert!(sleeps[1].is_finished());
        assert!(!sleeps[2].is_finished());
        assert!(!sleeps[3].is_finished());
        assert!(!sleeps[4].is_finished());

        time.tick(Duration::from_secs(2)).await;
        assert_eq!(SystemTime::UNIX_EPOCH + Duration::from_secs(4), time.now());
        assert!(sleeps[2].is_finished());
        assert!(sleeps[3].is_finished());
        assert!(!sleeps[4].is_finished());

        time.tick(Duration::from_secs(1)).await;
        assert_eq!(SystemTime::UNIX_EPOCH + Duration::from_secs(5), time.now());
        assert!(sleeps[4].is_finished());
    }

    #[tokio::test]
    async fn sleep_leading_to_sleep() {
        let (time, sleep) = tick_advance_time_and_sleep();

        let task = tokio::spawn(async move {
            sleep.sleep(Duration::from_secs(1)).await;
            sleep.sleep(Duration::from_secs(2)).await;
            sleep.sleep(Duration::from_secs(3)).await;
        });
        tokio::task::yield_now().await;
        assert!(!task.is_finished());
        assert_eq!(SystemTime::UNIX_EPOCH, time.now());

        time.tick(Duration::from_secs(6)).await;
        assert_eq!(SystemTime::UNIX_EPOCH + Duration::from_secs(6), time.now());
        task.await.unwrap();
    }

    #[tokio::test]
    async fn racing_sleeps() {
        let (time, sleep) = tick_advance_time_and_sleep();

        let task = tokio::spawn(async move {
            let sleep1 = sleep.sleep(Duration::from_secs(1)).then({
                let sleep = sleep.clone();
                move |_| async move {
                    sleep.sleep(Duration::from_secs(1)).await;
                }
            });
            let sleep2 = sleep.sleep(Duration::from_secs(3));
            tokio::select! {
                _ = sleep1 => { /* good */}
                _ = sleep2 => { panic!("sleep2 should not complete before sleep1") }
            }
        });
        tokio::task::yield_now().await;
        assert!(!task.is_finished());
        assert_eq!(SystemTime::UNIX_EPOCH, time.now());

        time.tick(Duration::from_secs(6)).await;
        assert_eq!(SystemTime::UNIX_EPOCH + Duration::from_secs(6), time.now());
        task.await.unwrap();
    }
}
