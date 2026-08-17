use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

#[cfg(feature = "_rt-async-std")]
pub mod rt_async_std;

#[cfg(feature = "_rt-tokio")]
pub mod rt_tokio;

#[derive(Debug, thiserror::Error)]
#[error("operation timed out")]
pub struct TimeoutError(());

pub enum JoinHandle<T> {
    #[cfg(feature = "_rt-async-std")]
    AsyncStd(async_std::task::JoinHandle<T>),
    #[cfg(feature = "_rt-tokio")]
    Tokio(tokio::task::JoinHandle<T>),
    // `PhantomData<T>` requires `T: Unpin`
    _Phantom(PhantomData<fn() -> T>),
}

pub async fn timeout<F: Future>(duration: Duration, f: F) -> Result<F::Output, TimeoutError> {
    #[cfg(feature = "_rt-tokio")]
    if rt_tokio::available() {
        return tokio::time::timeout(duration, f)
            .await
            .map_err(|_| TimeoutError(()));
    }

    #[cfg(feature = "_rt-async-std")]
    {
        async_std::future::timeout(duration, f)
            .await
            .map_err(|_| TimeoutError(()))
    }

    #[cfg(not(feature = "_rt-async-std"))]
    missing_rt((duration, f))
}

pub async fn sleep(duration: Duration) {
    #[cfg(feature = "_rt-tokio")]
    if rt_tokio::available() {
        return tokio::time::sleep(duration).await;
    }

    #[cfg(feature = "_rt-async-std")]
    {
        async_std::task::sleep(duration).await
    }

    #[cfg(not(feature = "_rt-async-std"))]
    missing_rt(duration)
}

#[track_caller]
pub fn spawn<F>(fut: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    #[cfg(feature = "_rt-tokio")]
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        return JoinHandle::Tokio(handle.spawn(fut));
    }

    #[cfg(feature = "_rt-async-std")]
    {
        JoinHandle::AsyncStd(async_std::task::spawn(fut))
    }

    #[cfg(not(feature = "_rt-async-std"))]
    missing_rt(fut)
}

#[track_caller]
pub fn spawn_blocking<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    #[cfg(feature = "_rt-tokio")]
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        return JoinHandle::Tokio(handle.spawn_blocking(f));
    }

    #[cfg(feature = "_rt-async-std")]
    {
        JoinHandle::AsyncStd(async_std::task::spawn_blocking(f))
    }

    #[cfg(not(feature = "_rt-async-std"))]
    missing_rt(f)
}

pub async fn yield_now() {
    #[cfg(feature = "_rt-tokio")]
    if rt_tokio::available() {
        return tokio::task::yield_now().await;
    }

    #[cfg(feature = "_rt-async-std")]
    {
        async_std::task::yield_now().await;
    }

    #[cfg(not(feature = "_rt-async-std"))]
    missing_rt(())
}

#[track_caller]
pub fn test_block_on<F: Future>(f: F) -> F::Output {
    #[cfg(feature = "_rt-tokio")]
    {
        return tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to start Tokio runtime")
            .block_on(f);
    }

    #[cfg(all(feature = "_rt-async-std", not(feature = "_rt-tokio")))]
    {
        async_std::task::block_on(f)
    }

    #[cfg(not(any(feature = "_rt-async-std", feature = "_rt-tokio")))]
    {
        missing_rt(f)
    }
}

#[track_caller]
pub fn missing_rt<T>(_unused: T) -> ! {
    if cfg!(feature = "_rt-tokio") {
        panic!("this functionality requires a Tokio context")
    }

    panic!("either the `runtime-async-std` or `runtime-tokio` feature must be enabled")
}

impl<T: Send + 'static> Future for JoinHandle<T> {
    type Output = T;

    #[track_caller]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &mut *self {
            #[cfg(feature = "_rt-async-std")]
            Self::AsyncStd(handle) => Pin::new(handle).poll(cx),
            #[cfg(feature = "_rt-tokio")]
            Self::Tokio(handle) => Pin::new(handle)
                .poll(cx)
                .map(|res| res.expect("spawned task panicked")),
            Self::_Phantom(_) => {
                let _ = cx;
                unreachable!("runtime should have been checked on spawn")
            }
        }
    }
}
