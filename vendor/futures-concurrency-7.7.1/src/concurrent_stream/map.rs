use pin_project::pin_project;

use super::{ConcurrentStream, Consumer};
use core::num::NonZeroUsize;
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{ready, Context, Poll},
};

/// Convert items from one type into another
#[derive(Debug)]
pub struct Map<CS, F, FutT, T, FutB, B>
where
    CS: ConcurrentStream<Item = T, Future = FutT>,
    F: Fn(T) -> FutB,
    F: Clone,
    FutT: Future<Output = T>,
    FutB: Future<Output = B>,
{
    inner: CS,
    f: F,
    _phantom: PhantomData<(FutT, T, FutB, B)>,
}

impl<CS, F, FutT, T, FutB, B> Map<CS, F, FutT, T, FutB, B>
where
    CS: ConcurrentStream<Item = T, Future = FutT>,
    F: Fn(T) -> FutB,
    F: Clone,
    FutT: Future<Output = T>,
    FutB: Future<Output = B>,
{
    pub(crate) fn new(inner: CS, f: F) -> Self {
        Self {
            inner,
            f,
            _phantom: PhantomData,
        }
    }
}

impl<CS, F, FutT, T, FutB, B> ConcurrentStream for Map<CS, F, FutT, T, FutB, B>
where
    CS: ConcurrentStream<Item = T, Future = FutT>,
    F: Fn(T) -> FutB,
    F: Clone,
    FutT: Future<Output = T>,
    FutB: Future<Output = B>,
{
    type Future = MapFuture<F, FutT, T, FutB, B>;
    type Item = B;

    async fn drive<C>(self, consumer: C) -> C::Output
    where
        C: Consumer<Self::Item, Self::Future>,
    {
        let consumer = MapConsumer {
            inner: consumer,
            f: self.f,
            _phantom: PhantomData,
        };
        self.inner.drive(consumer).await
    }

    fn concurrency_limit(&self) -> Option<NonZeroUsize> {
        self.inner.concurrency_limit()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[pin_project]
pub struct MapConsumer<C, F, FutT, T, FutB, B>
where
    FutT: Future<Output = T>,
    C: Consumer<B, MapFuture<F, FutT, T, FutB, B>>,
    F: Fn(T) -> FutB,
    F: Clone,
    FutB: Future<Output = B>,
{
    #[pin]
    inner: C,
    f: F,
    _phantom: PhantomData<(FutT, T, FutB, B)>,
}

impl<C, F, FutT, T, FutB, B> Consumer<T, FutT> for MapConsumer<C, F, FutT, T, FutB, B>
where
    FutT: Future<Output = T>,
    C: Consumer<B, MapFuture<F, FutT, T, FutB, B>>,
    F: Fn(T) -> FutB,
    F: Clone,
    FutB: Future<Output = B>,
{
    type Output = C::Output;

    async fn progress(self: Pin<&mut Self>) -> super::ConsumerState {
        let this = self.project();
        this.inner.progress().await
    }

    async fn send(self: Pin<&mut Self>, future: FutT) -> super::ConsumerState {
        let this = self.project();
        let fut = MapFuture::new(this.f.clone(), future);
        this.inner.send(fut).await
    }

    async fn flush(self: Pin<&mut Self>) -> Self::Output {
        let this = self.project();
        this.inner.flush().await
    }
}

/// Takes a future and maps it to another future via a closure
#[derive(Debug)]
pub struct MapFuture<F, FutT, T, FutB, B>
where
    FutT: Future<Output = T>,
    F: Fn(T) -> FutB,
    FutB: Future<Output = B>,
{
    done: bool,
    f: F,
    fut_t: Option<FutT>,
    fut_b: Option<FutB>,
}

impl<F, FutT, T, FutB, B> MapFuture<F, FutT, T, FutB, B>
where
    FutT: Future<Output = T>,
    F: Fn(T) -> FutB,
    FutB: Future<Output = B>,
{
    fn new(f: F, fut_t: FutT) -> Self {
        Self {
            done: false,
            f,
            fut_t: Some(fut_t),
            fut_b: None,
        }
    }
}

impl<F, FutT, T, FutB, B> Future for MapFuture<F, FutT, T, FutB, B>
where
    FutT: Future<Output = T>,
    F: Fn(T) -> FutB,
    FutB: Future<Output = B>,
{
    type Output = B;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: we need to access the inner future's fields to project them
        let this = unsafe { self.get_unchecked_mut() };
        if this.done {
            panic!("future has already been polled to completion once");
        }

        // Poll forward the future containing the value of `T`
        if let Some(fut) = this.fut_t.as_mut() {
            // SAFETY: we're pin projecting here
            let t = ready!(unsafe { Pin::new_unchecked(fut) }.poll(cx));
            let fut_b = (this.f)(t);
            this.fut_t = None;
            this.fut_b = Some(fut_b);
        }

        // Poll forward the future returned by the closure
        if let Some(fut) = this.fut_b.as_mut() {
            // SAFETY: we're pin projecting here
            let t = ready!(unsafe { Pin::new_unchecked(fut) }.poll(cx));
            this.done = true;
            return Poll::Ready(t);
        }

        unreachable!("neither future `a` nor future `b` were ready");
    }
}
