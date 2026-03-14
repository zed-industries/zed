use crate::{BackgroundExecutor, Task};
use std::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
    task,
    time::Duration,
};

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

#[pin_project::pin_project]
pub struct WithTimeout<T> {
    #[pin]
    future: T,
    #[pin]
    timer: Task<()>,
}

#[derive(Debug, thiserror::Error)]
#[error("Timed out before future resolved")]
/// Error returned by with_timeout when the timeout duration elapsed before the future resolved
pub struct Timeout;

impl<T: Future> Future for WithTimeout<T> {
    type Output = Result<T::Output, Timeout>;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> task::Poll<Self::Output> {
        let this = self.project();

        if let task::Poll::Ready(output) = this.future.poll(cx) {
            task::Poll::Ready(Ok(output))
        } else if this.timer.poll(cx).is_ready() {
            task::Poll::Ready(Err(Timeout))
        } else {
            task::Poll::Pending
        }
    }
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

/// Rounds to the nearest integer with ±0.5 ties toward zero.
///
/// This is the single rounding policy for all device-pixel snapping in the
/// rendering pipeline. A consistent midpoint rule prevents 1-device-pixel
/// gaps or overlaps between adjacent elements.
#[inline]
pub(crate) fn round_half_toward_zero(value: f32) -> f32 {
    if value >= 0.0 {
        (value - 0.5).ceil()
    } else {
        (value + 0.5).floor()
    }
}

/// f64 variant of [`round_half_toward_zero`] for scroll-offset arithmetic
/// that must preserve f64 precision.
#[inline]
pub(crate) fn round_half_toward_zero_f64(value: f64) -> f64 {
    if value >= 0.0 {
        (value - 0.5).ceil()
    } else {
        (value + 0.5).floor()
    }
}

#[cfg(test)]
mod tests {
    use crate::TestAppContext;

    use super::*;

    #[test]
    fn test_round_half_toward_zero() {
        // Midpoint ties go toward zero
        assert_eq!(round_half_toward_zero(0.5), 0.0);
        assert_eq!(round_half_toward_zero(1.5), 1.0);
        assert_eq!(round_half_toward_zero(2.5), 2.0);
        assert_eq!(round_half_toward_zero(-0.5), 0.0);
        assert_eq!(round_half_toward_zero(-1.5), -1.0);
        assert_eq!(round_half_toward_zero(-2.5), -2.0);

        // Non-midpoint values round to nearest
        assert_eq!(round_half_toward_zero(1.5001), 2.0);
        assert_eq!(round_half_toward_zero(1.4999), 1.0);
        assert_eq!(round_half_toward_zero(-1.5001), -2.0);
        assert_eq!(round_half_toward_zero(-1.4999), -1.0);

        // Integers are unchanged
        assert_eq!(round_half_toward_zero(0.0), 0.0);
        assert_eq!(round_half_toward_zero(3.0), 3.0);
        assert_eq!(round_half_toward_zero(-3.0), -3.0);
    }

    #[test]
    fn test_round_half_toward_zero_f64() {
        assert_eq!(round_half_toward_zero_f64(0.5), 0.0);
        assert_eq!(round_half_toward_zero_f64(-0.5), 0.0);
        assert_eq!(round_half_toward_zero_f64(1.5), 1.0);
        assert_eq!(round_half_toward_zero_f64(-1.5), -1.0);
        assert_eq!(round_half_toward_zero_f64(2.5001), 3.0);
    }

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
