/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Provides types to support stream-like operations for paginators.

use crate::future::pagination_stream::collect::sealed::Collectable;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub mod collect;
pub mod fn_stream;
use fn_stream::FnStream;

/// Stream specifically made to support paginators.
///
/// `PaginationStream` provides two primary mechanisms for accessing stream of data.
/// 1. With [`.next()`](PaginationStream::next) (or [`try_next()`](PaginationStream::try_next)):
///
/// ```no_run
/// # async fn docs() {
/// # use aws_smithy_async::future::pagination_stream::PaginationStream;
/// # fn operation_to_yield_paginator<T>() -> PaginationStream<T> {
/// #     todo!()
/// # }
/// # struct Page;
/// let mut stream: PaginationStream<Page> = operation_to_yield_paginator();
/// while let Some(page) = stream.next().await {
///     // process `page`
/// }
/// # }
/// ```
/// 2. With [`.collect()`](PaginationStream::collect) (or [`try_collect()`](PaginationStream::try_collect)):
///
/// ```no_run
/// # async fn docs() {
/// # use aws_smithy_async::future::pagination_stream::PaginationStream;
/// # fn operation_to_yield_paginator<T>() -> PaginationStream<T> {
/// #     todo!()
/// # }
/// # struct Page;
/// let mut stream: PaginationStream<Page> = operation_to_yield_paginator();
/// let result = stream.collect::<Vec<Page>>().await;
/// # }
/// ```
///
/// [`PaginationStream`] is implemented in terms of [`FnStream`], but the latter is meant to be
/// used internally and not by external users.
#[derive(Debug)]
pub struct PaginationStream<Item>(FnStream<Item>);

impl<Item> PaginationStream<Item> {
    /// Creates a `PaginationStream` from the given [`FnStream`].
    pub fn new(stream: FnStream<Item>) -> Self {
        Self(stream)
    }

    /// Consumes and returns the next `Item` from this stream.
    pub async fn next(&mut self) -> Option<Item> {
        self.0.next().await
    }

    /// Poll an item from the stream
    pub fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }

    /// Consumes this stream and gathers elements into a collection.
    pub async fn collect<T: Collectable<Item>>(self) -> T {
        self.0.collect().await
    }
}

impl<T, E> PaginationStream<Result<T, E>> {
    /// Yields the next item in the stream or returns an error if an error is encountered.
    pub async fn try_next(&mut self) -> Result<Option<T>, E> {
        self.next().await.transpose()
    }

    /// Convenience method for `.collect::<Result<Vec<_>, _>()`.
    pub async fn try_collect(self) -> Result<Vec<T>, E> {
        self.collect::<Result<Vec<T>, E>>().await
    }
}

/// Utility wrapper to flatten paginated results
///
/// When flattening paginated results, it's most convenient to produce an iterator where the `Result`
/// is present in each item. This provides `items()` which can wrap an stream of `Result<Page, Err>`
/// and produce a stream of `Result<Item, Err>`.
#[derive(Debug)]
pub struct TryFlatMap<Page, Err>(PaginationStream<Result<Page, Err>>);

impl<Page, Err> TryFlatMap<Page, Err> {
    /// Creates a `TryFlatMap` that wraps the input.
    pub fn new(stream: PaginationStream<Result<Page, Err>>) -> Self {
        Self(stream)
    }

    /// Produces a new [`PaginationStream`] by mapping this stream with `map` then flattening the result.
    pub fn flat_map<M, Item, Iter>(mut self, map: M) -> PaginationStream<Result<Item, Err>>
    where
        Page: Send + 'static,
        Err: Send + 'static,
        M: Fn(Page) -> Iter + Send + 'static,
        Item: Send + 'static,
        Iter: IntoIterator<Item = Item> + Send,
        <Iter as IntoIterator>::IntoIter: Send,
    {
        PaginationStream::new(FnStream::new(|tx| {
            Box::pin(async move {
                while let Some(page) = self.0.next().await {
                    match page {
                        Ok(page) => {
                            let mapped = map(page);
                            for item in mapped.into_iter() {
                                let _ = tx.send(Ok(item)).await;
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            break;
                        }
                    }
                }
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::future::pagination_stream::{FnStream, PaginationStream, TryFlatMap};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    /// basic test of FnStream functionality
    #[tokio::test]
    async fn fn_stream_returns_results() {
        tokio::time::pause();
        let mut stream = FnStream::new(|tx| {
            Box::pin(async move {
                tx.send("1").await.expect("failed to send");
                tokio::time::sleep(Duration::from_secs(1)).await;
                tokio::time::sleep(Duration::from_secs(1)).await;
                tx.send("2").await.expect("failed to send");
                tokio::time::sleep(Duration::from_secs(1)).await;
                tx.send("3").await.expect("failed to send");
            })
        });
        let mut out = vec![];
        while let Some(value) = stream.next().await {
            out.push(value);
        }
        assert_eq!(vec!["1", "2", "3"], out);
    }

    #[tokio::test]
    async fn fn_stream_try_next() {
        tokio::time::pause();
        let mut stream = FnStream::new(|tx| {
            Box::pin(async move {
                tx.send(Ok(1)).await.unwrap();
                tx.send(Ok(2)).await.unwrap();
                tx.send(Err("err")).await.unwrap();
            })
        });
        let mut out = vec![];
        while let Ok(value) = stream.try_next().await {
            out.push(value);
        }
        assert_eq!(vec![Some(1), Some(2)], out);
    }

    // smithy-rs#1902: there was a bug where we could continue to poll the generator after it
    // had returned Poll::Ready. This test case leaks the tx half so that the channel stays open
    // but the send side generator completes. By calling `poll` multiple times on the resulting future,
    // we can trigger the bug and validate the fix.
    #[tokio::test]
    async fn fn_stream_doesnt_poll_after_done() {
        let mut stream = FnStream::new(|tx| {
            Box::pin(async move {
                assert!(tx.send("blah").await.is_ok());
                Box::leak(Box::new(tx));
            })
        });
        assert_eq!(Some("blah"), stream.next().await);
        let mut test_stream = tokio_test::task::spawn(stream);
        // `tokio_test::task::Spawn::poll_next` can only be invoked when the wrapped
        // type implements the `Stream` trait. Here, `FnStream` does not implement it,
        // so we work around it by using the `enter` method.
        test_stream.enter(|ctx, pin| {
            let polled = pin.poll_next(ctx);
            assert!(polled.is_pending());
        });
        test_stream.enter(|ctx, pin| {
            let polled = pin.poll_next(ctx);
            assert!(polled.is_pending());
        });
    }

    /// Tests that the generator will not advance until demand exists
    #[tokio::test]
    async fn waits_for_reader() {
        let progress = Arc::new(Mutex::new(0));
        let mut stream = FnStream::new(|tx| {
            let progress = progress.clone();
            Box::pin(async move {
                *progress.lock().unwrap() = 1;
                tx.send("1").await.expect("failed to send");
                *progress.lock().unwrap() = 2;
                tx.send("2").await.expect("failed to send");
                *progress.lock().unwrap() = 3;
                tx.send("3").await.expect("failed to send");
                *progress.lock().unwrap() = 4;
            })
        });
        assert_eq!(*progress.lock().unwrap(), 0);
        stream.next().await.expect("ready");
        assert_eq!(*progress.lock().unwrap(), 1);

        assert_eq!("2", stream.next().await.expect("ready"));
        assert_eq!(2, *progress.lock().unwrap());

        let _ = stream.next().await.expect("ready");
        assert_eq!(3, *progress.lock().unwrap());
        assert_eq!(None, stream.next().await);
        assert_eq!(4, *progress.lock().unwrap());
    }

    #[tokio::test]
    async fn generator_with_errors() {
        let mut stream = FnStream::new(|tx| {
            Box::pin(async move {
                for i in 0..5 {
                    if i != 2 {
                        if tx.send(Ok(i)).await.is_err() {
                            return;
                        }
                    } else {
                        tx.send(Err(i)).await.unwrap();
                        return;
                    }
                }
            })
        });
        let mut out = vec![];
        while let Some(Ok(value)) = stream.next().await {
            out.push(value);
        }
        assert_eq!(vec![0, 1], out);
    }

    #[tokio::test]
    async fn flatten_items_ok() {
        #[derive(Debug)]
        struct Output {
            items: Vec<u8>,
        }
        let stream: FnStream<Result<_, &str>> = FnStream::new(|tx| {
            Box::pin(async move {
                tx.send(Ok(Output {
                    items: vec![1, 2, 3],
                }))
                .await
                .unwrap();
                tx.send(Ok(Output {
                    items: vec![4, 5, 6],
                }))
                .await
                .unwrap();
            })
        });
        assert_eq!(
            Ok(vec![1, 2, 3, 4, 5, 6]),
            TryFlatMap::new(PaginationStream::new(stream))
                .flat_map(|output| output.items.into_iter())
                .try_collect()
                .await,
        );
    }

    #[tokio::test]
    async fn flatten_items_error() {
        #[derive(Debug)]
        struct Output {
            items: Vec<u8>,
        }
        let stream = FnStream::new(|tx| {
            Box::pin(async move {
                tx.send(Ok(Output {
                    items: vec![1, 2, 3],
                }))
                .await
                .unwrap();
                tx.send(Err("bummer")).await.unwrap();
            })
        });
        assert_eq!(
            Err("bummer"),
            TryFlatMap::new(PaginationStream::new(stream))
                .flat_map(|output| output.items.into_iter())
                .try_collect()
                .await
        )
    }
}
