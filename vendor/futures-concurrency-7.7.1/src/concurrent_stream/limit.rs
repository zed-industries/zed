use pin_project::pin_project;

use super::{ConcurrentStream, Consumer};
use core::future::Future;
use core::num::NonZeroUsize;
use core::pin::Pin;

/// A concurrent iterator that limits the amount of concurrency applied.
///
/// This `struct` is created by the [`limit`] method on [`ConcurrentStream`]. See its
/// documentation for more.
///
/// [`limit`]: ConcurrentStream::limit
/// [`ConcurrentStream`]: trait.ConcurrentStream.html
#[derive(Debug)]
pub struct Limit<CS: ConcurrentStream> {
    inner: CS,
    limit: Option<NonZeroUsize>,
}

impl<CS: ConcurrentStream> Limit<CS> {
    pub(crate) fn new(inner: CS, limit: Option<NonZeroUsize>) -> Self {
        Self { inner, limit }
    }
}

impl<CS: ConcurrentStream> ConcurrentStream for Limit<CS> {
    type Item = CS::Item;
    type Future = CS::Future;

    async fn drive<C>(self, consumer: C) -> C::Output
    where
        C: Consumer<Self::Item, Self::Future>,
    {
        self.inner.drive(LimitConsumer { inner: consumer }).await
    }

    // NOTE: this is the only interesting bit in this module. When a limit is
    // set, this now starts using it.
    fn concurrency_limit(&self) -> Option<NonZeroUsize> {
        self.limit
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[pin_project]
struct LimitConsumer<C> {
    #[pin]
    inner: C,
}
impl<C, Item, Fut> Consumer<Item, Fut> for LimitConsumer<C>
where
    Fut: Future<Output = Item>,
    C: Consumer<Item, Fut>,
{
    type Output = C::Output;

    async fn send(self: Pin<&mut Self>, future: Fut) -> super::ConsumerState {
        let this = self.project();
        this.inner.send(future).await
    }

    async fn progress(self: Pin<&mut Self>) -> super::ConsumerState {
        let this = self.project();
        this.inner.progress().await
    }

    async fn flush(self: Pin<&mut Self>) -> Self::Output {
        let this = self.project();
        this.inner.flush().await
    }
}
