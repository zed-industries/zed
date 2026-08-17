use crate::{Error, Result};
use std::{future::Future, io::ErrorKind, time::Duration};

/// Awaits a future with a provided timeout.
#[cfg(feature = "tokio")]
pub(crate) async fn timeout<F, T>(fut: F, timeout: Duration) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    tokio::time::timeout(timeout, fut).await.map_err(|_| {
        Error::from(std::io::Error::new(
            ErrorKind::TimedOut,
            "timed out".to_string(),
        ))
    })?
}

/// Awaits a future with a provided timeout.
#[cfg(not(feature = "tokio"))]
pub(crate) async fn timeout<F, T>(fut: F, timeout: Duration) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    use futures_lite::FutureExt;

    fut.or(async {
        async_io::Timer::after(timeout).await;

        Err(Error::from(std::io::Error::new(
            ErrorKind::TimedOut,
            "timed out",
        )))
    })
    .await
}
