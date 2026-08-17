// Take a look at the license at the top of the repository in the LICENSE file.

use std::{error, fmt};

use futures_util::{
    future::{self, Either, Future},
    pin_mut,
};

// rustdoc-stripper-ignore-next
/// The error returned when a future times out.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FutureWithTimeoutError;

impl fmt::Display for FutureWithTimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("The future timed out")
    }
}

impl error::Error for FutureWithTimeoutError {}

// rustdoc-stripper-ignore-next
/// Add a timeout to a `Future`.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub async fn future_with_timeout_with_priority<T>(
    priority: crate::Priority,
    timeout: std::time::Duration,
    fut: impl Future<Output = T>,
) -> Result<T, FutureWithTimeoutError> {
    let timeout = crate::timeout_future_with_priority(priority, timeout);
    pin_mut!(fut);

    match future::select(fut, timeout).await {
        Either::Left((x, _)) => Ok(x),
        _ => Err(FutureWithTimeoutError),
    }
}

// rustdoc-stripper-ignore-next
/// Add a timeout to a `Future`.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub async fn future_with_timeout<T>(
    timeout: std::time::Duration,
    fut: impl Future<Output = T>,
) -> Result<T, FutureWithTimeoutError> {
    future_with_timeout_with_priority(crate::Priority::default(), timeout, fut).await
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures_util::FutureExt;

    use super::*;
    use crate::{MainContext, MainLoop};

    #[test]
    fn test_future_with_timeout() {
        let c = MainContext::new();

        let fut = future::pending::<()>();
        let result = c.block_on(future_with_timeout(Duration::from_millis(20), fut));
        assert_eq!(result, Err(FutureWithTimeoutError));

        let fut = future::ready(());
        let result = c.block_on(future_with_timeout(Duration::from_millis(20), fut));
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_future_with_timeout_send() {
        let c = MainContext::new();
        let l = MainLoop::new(Some(&c), false);

        let l_clone = l.clone();
        let fut = future::pending::<()>();
        c.spawn(
            future_with_timeout(Duration::from_millis(20), fut).then(move |result| {
                l_clone.quit();
                assert_eq!(result, Err(FutureWithTimeoutError));
                futures_util::future::ready(())
            }),
        );

        l.run();

        let l_clone = l.clone();
        let fut = future::ready(());
        c.spawn(
            future_with_timeout(Duration::from_millis(20), fut).then(move |result| {
                l_clone.quit();
                assert_eq!(result, Ok(()));
                futures_util::future::ready(())
            }),
        );

        l.run();
    }
}
