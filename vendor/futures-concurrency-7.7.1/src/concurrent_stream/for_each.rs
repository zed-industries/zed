use super::{Consumer, ConsumerState};
use crate::future::FutureGroup;
use futures_lite::StreamExt;
use pin_project::pin_project;

use alloc::sync::Arc;
use core::future::Future;
use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::task::{ready, Context, Poll};

// OK: validated! - all bounds should check out
#[pin_project]
pub(crate) struct ForEachConsumer<FutT, T, F, FutB>
where
    FutT: Future<Output = T>,
    F: Fn(T) -> FutB,
    FutB: Future<Output = ()>,
{
    // NOTE: we can remove the `Arc` here if we're willing to make this struct self-referential
    count: Arc<AtomicUsize>,
    #[pin]
    group: FutureGroup<ForEachFut<F, FutT, T, FutB>>,
    limit: usize,
    f: F,
    _phantom: PhantomData<(T, FutB)>,
}

impl<A, T, F, B> ForEachConsumer<A, T, F, B>
where
    A: Future<Output = T>,
    F: Fn(T) -> B,
    B: Future<Output = ()>,
{
    pub(crate) fn new(limit: Option<NonZeroUsize>, f: F) -> Self {
        let limit = match limit {
            Some(n) => n.get(),
            None => usize::MAX,
        };
        Self {
            limit,
            f,
            _phantom: PhantomData,
            count: Arc::new(AtomicUsize::new(0)),
            group: FutureGroup::new(),
        }
    }
}

// OK: validated! - we push types `B` into the next consumer
impl<FutT, T, F, B> Consumer<T, FutT> for ForEachConsumer<FutT, T, F, B>
where
    FutT: Future<Output = T>,
    F: Fn(T) -> B,
    F: Clone,
    B: Future<Output = ()>,
{
    type Output = ();

    async fn send(self: Pin<&mut Self>, future: FutT) -> super::ConsumerState {
        let mut this = self.project();
        // If we have no space, we're going to provide backpressure until we have space
        while this.count.load(Ordering::Relaxed) >= *this.limit {
            this.group.next().await;
        }

        // Space was available! - insert the item for posterity
        this.count.fetch_add(1, Ordering::Relaxed);
        let fut = ForEachFut::new(this.f.clone(), future, this.count.clone());
        this.group.as_mut().insert_pinned(fut);

        ConsumerState::Continue
    }

    async fn progress(self: Pin<&mut Self>) -> super::ConsumerState {
        let mut this = self.project();
        while (this.group.next().await).is_some() {}
        ConsumerState::Empty
    }

    async fn flush(self: Pin<&mut Self>) -> Self::Output {
        let mut this = self.project();
        // 4. We will no longer receive any additional futures from the
        // underlying stream; wait until all the futures in the group have
        // resolved.
        while (this.group.next().await).is_some() {}
    }
}

/// Takes a future and maps it to another future via a closure
#[derive(Debug)]
pub struct ForEachFut<F, FutT, T, FutB>
where
    FutT: Future<Output = T>,
    F: Fn(T) -> FutB,
    FutB: Future<Output = ()>,
{
    done: bool,
    count: Arc<AtomicUsize>,
    f: F,
    fut_t: Option<FutT>,
    fut_b: Option<FutB>,
}

impl<F, FutT, T, FutB> ForEachFut<F, FutT, T, FutB>
where
    FutT: Future<Output = T>,
    F: Fn(T) -> FutB,
    FutB: Future<Output = ()>,
{
    fn new(f: F, fut_t: FutT, count: Arc<AtomicUsize>) -> Self {
        Self {
            done: false,
            count,
            f,
            fut_t: Some(fut_t),
            fut_b: None,
        }
    }
}

impl<F, FutT, T, FutB> Future for ForEachFut<F, FutT, T, FutB>
where
    FutT: Future<Output = T>,
    F: Fn(T) -> FutB,
    FutB: Future<Output = ()>,
{
    type Output = ();

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
            ready!(unsafe { Pin::new_unchecked(fut) }.poll(cx));
            this.count.fetch_sub(1, Ordering::Relaxed);
            this.done = true;
            return Poll::Ready(());
        }

        unreachable!("neither future `a` nor future `b` were ready");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    use futures_lite::stream;

    #[test]
    fn concurrency_one() {
        futures_lite::future::block_on(async {
            let count = Arc::new(AtomicUsize::new(0));
            stream::repeat(1)
                .take(2)
                .co()
                .limit(NonZeroUsize::new(1))
                .for_each(|n| {
                    let count = count.clone();
                    async move {
                        count.fetch_add(n, Ordering::Relaxed);
                    }
                })
                .await;

            assert_eq!(count.load(Ordering::Relaxed), 2);
        });
    }

    #[test]
    fn concurrency_three() {
        futures_lite::future::block_on(async {
            let count = Arc::new(AtomicUsize::new(0));
            stream::repeat(1)
                .take(10)
                .co()
                .limit(NonZeroUsize::new(3))
                .for_each(|n| {
                    let count = count.clone();
                    async move {
                        count.fetch_add(n, Ordering::Relaxed);
                    }
                })
                .await;

            assert_eq!(count.load(Ordering::Relaxed), 10);
        });
    }
}
