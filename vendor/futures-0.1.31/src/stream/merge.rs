#![deprecated(note = "functionality provided by `select` now")]
#![allow(deprecated)]

use {Poll, Async};
use stream::{Stream, Fuse};

/// An adapter for merging the output of two streams.
///
/// The merged stream produces items from one or both of the underlying
/// streams as they become available. Errors, however, are not merged: you
/// get at most one error at a time.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Merge<S1, S2: Stream> {
    stream1: Fuse<S1>,
    stream2: Fuse<S2>,
    queued_error: Option<S2::Error>,
}

pub fn new<S1, S2>(stream1: S1, stream2: S2) -> Merge<S1, S2>
    where S1: Stream, S2: Stream<Error = S1::Error>
{
    Merge {
        stream1: stream1.fuse(),
        stream2: stream2.fuse(),
        queued_error: None,
    }
}

/// An item returned from a merge stream, which represents an item from one or
/// both of the underlying streams.
#[derive(Debug)]
pub enum MergedItem<I1, I2> {
    /// An item from the first stream
    First(I1),
    /// An item from the second stream
    Second(I2),
    /// Items from both streams
    Both(I1, I2),
}

impl<S1, S2> Stream for Merge<S1, S2>
    where S1: Stream, S2: Stream<Error = S1::Error>
{
    type Item = MergedItem<S1::Item, S2::Item>;
    type Error = S1::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(e) = self.queued_error.take() {
            return Err(e)
        }

        match self.stream1.poll()? {
            Async::NotReady => {
                match try_ready!(self.stream2.poll()) {
                    Some(item2) => Ok(Async::Ready(Some(MergedItem::Second(item2)))),
                    None => Ok(Async::NotReady),
                }
            }
            Async::Ready(None) => {
                match try_ready!(self.stream2.poll()) {
                    Some(item2) => Ok(Async::Ready(Some(MergedItem::Second(item2)))),
                    None => Ok(Async::Ready(None)),
                }
            }
            Async::Ready(Some(item1)) => {
                match self.stream2.poll() {
                    Err(e) => {
                        self.queued_error = Some(e);
                        Ok(Async::Ready(Some(MergedItem::First(item1))))
                    }
                    Ok(Async::NotReady) | Ok(Async::Ready(None)) => {
                        Ok(Async::Ready(Some(MergedItem::First(item1))))
                    }
                    Ok(Async::Ready(Some(item2))) => {
                        Ok(Async::Ready(Some(MergedItem::Both(item1, item2))))
                    }
                }
            }
        }
    }
}
