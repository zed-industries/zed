/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Module to define utility to drive a stream with an async function and a channel.

use crate::future::pagination_stream::collect::sealed::Collectable;
use crate::future::rendezvous;
use pin_project_lite::pin_project;
use std::fmt;
use std::future::poll_fn;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// The closure is passed a reference to a `Sender` which acts as a rendezvous channel. Messages
    /// sent to the sender will be emitted to the stream. Because the stream is 1-bounded, the function
    /// will not proceed until the stream is read.
    ///
    /// This utility is used by generated paginators to generate a stream of paginated results.
    ///
    /// If `tx.send` returns an error, the function MUST return immediately.
    ///
    /// Note `FnStream` is only `Send` but not `Sync` because `generator` is a boxed future that
    /// is `Send` and returns `()` as output when it is done.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn docs() {
    /// use aws_smithy_async::future::pagination_stream::fn_stream::FnStream;
    /// let mut stream = FnStream::new(|tx| Box::pin(async move {
    ///     if let Err(_) = tx.send("Hello!").await {
    ///         return;
    ///     }
    ///     if let Err(_) = tx.send("Goodbye!").await {
    ///         return;
    ///     }
    /// }));
    /// assert_eq!(stream.collect::<Vec<_>>().await, vec!["Hello!", "Goodbye!"]);
    /// # }
    pub struct FnStream<Item> {
        #[pin]
        rx: rendezvous::Receiver<Item>,
        generator: Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    }
}

impl<Item> fmt::Debug for FnStream<Item> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let item_typename = std::any::type_name::<Item>();
        write!(f, "FnStream<{item_typename}>")
    }
}

impl<Item> FnStream<Item> {
    /// Creates a new function based stream driven by `generator`.
    ///
    /// For examples, see the documentation for [`FnStream`]
    pub fn new<T>(generator: T) -> Self
    where
        T: FnOnce(rendezvous::Sender<Item>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    {
        let (tx, rx) = rendezvous::channel::<Item>();
        Self {
            rx,
            generator: Some(Box::pin(generator(tx))),
        }
    }

    /// Consumes and returns the next `Item` from this stream.
    pub async fn next(&mut self) -> Option<Item>
    where
        Self: Unpin,
    {
        let mut me = Pin::new(self);
        poll_fn(|cx| me.as_mut().poll_next(cx)).await
    }

    /// Attempts to pull out the next value of this stream, returning `None` if the stream is
    /// exhausted.
    pub(crate) fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Item>> {
        let mut me = self.project();
        match me.rx.poll_recv(cx) {
            Poll::Ready(item) => Poll::Ready(item),
            Poll::Pending => {
                if let Some(generator) = me.generator {
                    if generator.as_mut().poll(cx).is_ready() {
                        // `generator` keeps writing items to `tx` and will not be `Poll::Ready`
                        // until it is done writing to `tx`. Once it is done, it returns `()`
                        // as output and is `Poll::Ready`, at which point we MUST NOT poll it again
                        // since doing so will cause a panic.
                        *me.generator = None;
                    }
                }
                Poll::Pending
            }
        }
    }

    /// Consumes this stream and gathers elements into a collection.
    pub async fn collect<T: Collectable<Item>>(mut self) -> T {
        let mut collection = T::initialize();
        while let Some(item) = self.next().await {
            if !T::extend(&mut collection, item) {
                break;
            }
        }
        T::finalize(collection)
    }
}

impl<T, E> FnStream<Result<T, E>> {
    /// Yields the next item in the stream or returns an error if an error is encountered.
    pub async fn try_next(&mut self) -> Result<Option<T>, E> {
        self.next().await.transpose()
    }

    /// Convenience method for `.collect::<Result<Vec<_>, _>()`.
    pub async fn try_collect(self) -> Result<Vec<T>, E> {
        self.collect::<Result<Vec<T>, E>>().await
    }
}
