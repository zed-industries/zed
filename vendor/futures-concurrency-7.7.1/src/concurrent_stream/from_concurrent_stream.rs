use super::{ConcurrentStream, Consumer, ConsumerState, IntoConcurrentStream};
use crate::future::FutureGroup;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
use core::future::Future;
use core::pin::Pin;
use futures_lite::StreamExt;
use pin_project::pin_project;

/// Conversion from a [`ConcurrentStream`]
#[allow(async_fn_in_trait)]
pub trait FromConcurrentStream<A>: Sized {
    /// Creates a value from a concurrent iterator.
    async fn from_concurrent_stream<T>(iter: T) -> Self
    where
        T: IntoConcurrentStream<Item = A>;
}

impl<T> FromConcurrentStream<T> for Vec<T> {
    async fn from_concurrent_stream<S>(iter: S) -> Self
    where
        S: IntoConcurrentStream<Item = T>,
    {
        let stream = iter.into_co_stream();
        let mut output = Vec::with_capacity(stream.size_hint().1.unwrap_or_default());
        stream.drive(VecConsumer::new(&mut output)).await;
        output
    }
}

impl<T, E> FromConcurrentStream<Result<T, E>> for Result<Vec<T>, E> {
    async fn from_concurrent_stream<S>(iter: S) -> Self
    where
        S: IntoConcurrentStream<Item = Result<T, E>>,
    {
        let stream = iter.into_co_stream();
        let mut output = Ok(Vec::with_capacity(stream.size_hint().1.unwrap_or_default()));
        stream.drive(ResultVecConsumer::new(&mut output)).await;
        output
    }
}

// TODO: replace this with a generalized `fold` operation
#[pin_project]
pub(crate) struct VecConsumer<'a, Fut: Future> {
    #[pin]
    group: FutureGroup<Fut>,
    output: &'a mut Vec<Fut::Output>,
}

impl<'a, Fut: Future> VecConsumer<'a, Fut> {
    pub(crate) fn new(output: &'a mut Vec<Fut::Output>) -> Self {
        Self {
            group: FutureGroup::new(),
            output,
        }
    }
}

impl<Item, Fut> Consumer<Item, Fut> for VecConsumer<'_, Fut>
where
    Fut: Future<Output = Item>,
{
    type Output = ();

    async fn send(self: Pin<&mut Self>, future: Fut) -> super::ConsumerState {
        let mut this = self.project();
        // unbounded concurrency, so we just goooo
        this.group.as_mut().insert_pinned(future);
        ConsumerState::Continue
    }

    async fn progress(self: Pin<&mut Self>) -> super::ConsumerState {
        let mut this = self.project();
        while let Some(item) = this.group.next().await {
            this.output.push(item);
        }
        ConsumerState::Empty
    }
    async fn flush(self: Pin<&mut Self>) -> Self::Output {
        let mut this = self.project();
        while let Some(item) = this.group.next().await {
            this.output.push(item);
        }
    }
}

#[pin_project]
pub(crate) struct ResultVecConsumer<'a, Fut: Future, T, E> {
    #[pin]
    group: FutureGroup<Fut>,
    output: &'a mut Result<Vec<T>, E>,
}

impl<'a, Fut: Future, T, E> ResultVecConsumer<'a, Fut, T, E> {
    pub(crate) fn new(output: &'a mut Result<Vec<T>, E>) -> Self {
        Self {
            group: FutureGroup::new(),
            output,
        }
    }
}

impl<Fut, T, E> Consumer<Result<T, E>, Fut> for ResultVecConsumer<'_, Fut, T, E>
where
    Fut: Future<Output = Result<T, E>>,
{
    type Output = ();

    async fn send(self: Pin<&mut Self>, future: Fut) -> super::ConsumerState {
        let mut this = self.project();
        // unbounded concurrency, so we just goooo
        this.group.as_mut().insert_pinned(future);
        ConsumerState::Continue
    }

    async fn progress(self: Pin<&mut Self>) -> super::ConsumerState {
        let mut this = self.project();
        let Ok(items) = this.output else {
            return ConsumerState::Break;
        };

        while let Some(item) = this.group.next().await {
            match item {
                Ok(item) => {
                    items.push(item);
                }
                Err(e) => {
                    **this.output = Err(e);
                    return ConsumerState::Break;
                }
            }
        }
        ConsumerState::Empty
    }

    async fn flush(self: Pin<&mut Self>) -> Self::Output {
        self.progress().await;
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use futures_lite::stream;

    #[test]
    fn collect() {
        futures_lite::future::block_on(async {
            let v: Vec<_> = stream::repeat(1).co().take(5).collect().await;
            assert_eq!(v, &[1, 1, 1, 1, 1]);
        });
    }

    #[test]
    fn collect_to_result_ok() {
        futures_lite::future::block_on(async {
            let v: Result<Vec<_>, ()> = stream::repeat(Ok(1)).co().take(5).collect().await;
            assert_eq!(v, Ok(vec![1, 1, 1, 1, 1]));
        });
    }

    #[test]
    fn collect_to_result_err() {
        futures_lite::future::block_on(async {
            let v: Result<Vec<_>, _> = stream::repeat(Err::<u8, _>(()))
                .co()
                .take(5)
                .collect()
                .await;
            assert_eq!(v, Err(()));
        });
    }
}
