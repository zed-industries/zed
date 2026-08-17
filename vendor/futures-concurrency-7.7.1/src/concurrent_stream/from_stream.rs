use super::Consumer;
use crate::concurrent_stream::ConsumerState;
use crate::prelude::*;

use core::future::{ready, Ready};
use core::num::NonZeroUsize;
use core::pin::pin;
use futures_lite::{Stream, StreamExt};

/// A concurrent for each implementation from a `Stream`
#[pin_project::pin_project]
#[derive(Debug)]
pub struct FromStream<S: Stream> {
    #[pin]
    stream: S,
}

impl<S: Stream> FromStream<S> {
    pub(crate) fn new(stream: S) -> Self {
        Self { stream }
    }
}

impl<S> ConcurrentStream for FromStream<S>
where
    S: Stream,
{
    type Item = S::Item;
    type Future = Ready<Self::Item>;

    async fn drive<C>(self, consumer: C) -> C::Output
    where
        C: Consumer<Self::Item, Self::Future>,
    {
        let mut iter = pin!(self.stream);
        let mut consumer = pin!(consumer);

        // Concurrently progress the consumer as well as the stream. Whenever
        // there is an item from the stream available, we submit it to the
        // consumer and we wait.
        //
        // NOTE(yosh): we're relying on the fact that `Stream::next` can be
        // dropped and recreated freely. That's also true for
        // `Consumer::progress`; though that is intentional. It should be
        // possible to write a combinator which does not drop the `Stream::next`
        // future repeatedly. However for now we're happy to rely on this
        // property here.
        loop {
            // Drive the stream forward
            let a = async {
                let item = iter.next().await;
                State::Item(item)
            };

            // Drive the consumer forward
            let b = async {
                let control_flow = consumer.as_mut().progress().await;
                State::Progress(control_flow)
            };

            // If an item is available, submit it to the consumer and wait for
            // it to be ready.
            match (b, a).race().await {
                State::Progress(control_flow) => match control_flow {
                    ConsumerState::Break => break,
                    ConsumerState::Continue => continue,
                    ConsumerState::Empty => match iter.next().await {
                        Some(item) => match consumer.as_mut().send(ready(item)).await {
                            ConsumerState::Break => break,
                            ConsumerState::Empty | ConsumerState::Continue => continue,
                        },
                        None => break,
                    },
                },
                State::Item(Some(item)) => match consumer.as_mut().send(ready(item)).await {
                    ConsumerState::Break => break,
                    ConsumerState::Empty | ConsumerState::Continue => continue,
                },
                State::Item(None) => break,
            }
        }

        // We will no longer receive items from the underlying stream, which
        // means we're ready to wait for the consumer to finish up.
        consumer.as_mut().flush().await
    }

    fn concurrency_limit(&self) -> Option<NonZeroUsize> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

enum State<T> {
    Progress(super::ConsumerState),
    Item(T),
}
