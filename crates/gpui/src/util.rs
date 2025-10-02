use crate::{BackgroundExecutor, Task};
use std::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
    task,
    time::Duration,
};

pub use util::*;

/// A helper trait for building complex objects with imperative conditionals in a fluent style.
pub trait FluentBuilder {
    /// Imperatively modify self with the given closure.
    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U
    where
        Self: Sized,
    {
        f(self)
    }

    /// Conditionally modify self with the given closure.
    fn when(self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if condition { then(this) } else { this })
    }

    /// Conditionally modify self with the given closure.
    fn when_else(
        self,
        condition: bool,
        then: impl FnOnce(Self) -> Self,
        else_fn: impl FnOnce(Self) -> Self,
    ) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if condition { then(this) } else { else_fn(this) })
    }

    /// Conditionally unwrap and modify self with the given closure, if the given option is Some.
    fn when_some<T>(self, option: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| {
            if let Some(value) = option {
                then(this, value)
            } else {
                this
            }
        })
    }
    /// Conditionally unwrap and modify self with the given closure, if the given option is None.
    fn when_none<T>(self, option: &Option<T>, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if option.is_some() { this } else { then(this) })
    }
}

/// Extensions for Future types that provide additional combinators and utilities.
pub trait FutureExt {
    /// Requires a Future to complete before the specified duration has elapsed.
    /// Similar to tokio::timeout.
    fn with_timeout(self, timeout: Duration, executor: &BackgroundExecutor) -> WithTimeout<Self>
    where
        Self: Sized;
}

impl<T: Future> FutureExt for T {
    fn with_timeout(self, timeout: Duration, executor: &BackgroundExecutor) -> WithTimeout<Self>
    where
        Self: Sized,
    {
        WithTimeout {
            future: self,
            timer: executor.timer(timeout),
        }
    }
}

pub struct WithTimeout<T> {
    future: T,
    timer: Task<()>,
}

#[derive(Debug, thiserror::Error)]
#[error("Timed out before future resolved")]
/// Error returned by with_timeout when the timeout duration elapsed before the future resolved
pub struct Timeout;

impl<T: Future> Future for WithTimeout<T> {
    type Output = Result<T::Output, Timeout>;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> task::Poll<Self::Output> {
        // SAFETY: the fields of Timeout are private and we never move the future ourselves
        // And its already pinned since we are being polled (all futures need to be pinned to be polled)
        let this = unsafe { &raw mut *self.get_unchecked_mut() };
        let future = unsafe { Pin::new_unchecked(&mut (*this).future) };
        let timer = unsafe { Pin::new_unchecked(&mut (*this).timer) };

        if let task::Poll::Ready(output) = future.poll(cx) {
            task::Poll::Ready(Ok(output))
        } else if timer.poll(cx).is_ready() {
            task::Poll::Ready(Err(Timeout))
        } else {
            task::Poll::Pending
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
/// Uses smol executor to run a given future no longer than the timeout specified.
/// Note that this won't "rewind" on `cx.executor().advance_clock` call, truly waiting for the timeout to elapse.
pub async fn smol_timeout<F, T>(timeout: Duration, f: F) -> Result<T, ()>
where
    F: Future<Output = T>,
{
    let timer = async {
        smol::Timer::after(timeout).await;
        Err(())
    };
    let future = async move { Ok(f.await) };
    smol::future::FutureExt::race(timer, future).await
}

/// Increment the given atomic counter if it is not zero.
/// Return the new value of the counter.
pub(crate) fn atomic_incr_if_not_zero(counter: &AtomicUsize) -> usize {
    let mut loaded = counter.load(SeqCst);
    loop {
        if loaded == 0 {
            return 0;
        }
        match counter.compare_exchange_weak(loaded, loaded + 1, SeqCst, SeqCst) {
            Ok(x) => return x + 1,
            Err(actual) => loaded = actual,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TestAppContext;

    use super::*;

    #[gpui::test]
    async fn test_with_timeout(cx: &mut TestAppContext) {
        Task::ready(())
            .with_timeout(Duration::from_secs(1), &cx.executor())
            .await
            .expect("Timeout should be noop");

        let long_duration = Duration::from_secs(6000);
        let short_duration = Duration::from_secs(1);
        cx.executor()
            .timer(long_duration)
            .with_timeout(short_duration, &cx.executor())
            .await
            .expect_err("timeout should have triggered");

        let fut = cx
            .executor()
            .timer(long_duration)
            .with_timeout(short_duration, &cx.executor());
        cx.executor().advance_clock(short_duration * 2);
        futures::FutureExt::now_or_never(fut)
            .unwrap_or_else(|| panic!("timeout should have triggered"))
            .expect_err("timeout");
    }
}
