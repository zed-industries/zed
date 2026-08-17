use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use pin_project_lite::pin_project;

use crate::io;
use crate::utils::{timer_after, Timer};

/// Awaits an I/O future or times out after a duration of time.
///
/// If you want to await a non I/O future consider using
/// [`future::timeout`](../future/fn.timeout.html) instead.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use std::time::Duration;
///
/// use async_std::io;
///
/// io::timeout(Duration::from_secs(5), async {
///     let stdin = io::stdin();
///     let mut line = String::new();
///     let n = stdin.read_line(&mut line).await?;
///     Ok(())
/// })
/// .await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn timeout<F, T>(dur: Duration, f: F) -> io::Result<T>
where
    F: Future<Output = io::Result<T>>,
{
    Timeout {
        timeout: timer_after(dur),
        future: f,
    }
    .await
}

pin_project! {
    /// Future returned by the `FutureExt::timeout` method.
    #[derive(Debug)]
    pub struct Timeout<F, T>
    where
        F: Future<Output = io::Result<T>>,
    {
        #[pin]
        future: F,
        #[pin]
        timeout: Timer,
    }
}

impl<F, T> Future for Timeout<F, T>
where
    F: Future<Output = io::Result<T>>,
{
    type Output = io::Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Pending => {}
            other => return other,
        }

        if this.timeout.poll(cx).is_ready() {
            let err = Err(io::Error::new(io::ErrorKind::TimedOut, "future timed out"));
            Poll::Ready(err)
        } else {
            Poll::Pending
        }
    }
}
