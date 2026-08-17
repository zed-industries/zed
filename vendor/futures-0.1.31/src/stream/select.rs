use {Poll, Async};
use stream::{Stream, Fuse};

/// An adapter for merging the output of two streams.
///
/// The merged stream produces items from either of the underlying streams as
/// they become available, and the streams are polled in a round-robin fashion.
/// Errors, however, are not merged: you get at most one error at a time.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Select<S1, S2> {
    stream1: Fuse<S1>,
    stream2: Fuse<S2>,
    flag: bool,
}

pub fn new<S1, S2>(stream1: S1, stream2: S2) -> Select<S1, S2>
    where S1: Stream,
          S2: Stream<Item = S1::Item, Error = S1::Error>
{
    Select {
        stream1: stream1.fuse(),
        stream2: stream2.fuse(),
        flag: false,
    }
}

impl<S1, S2> Stream for Select<S1, S2>
    where S1: Stream,
          S2: Stream<Item = S1::Item, Error = S1::Error>
{
    type Item = S1::Item;
    type Error = S1::Error;

    fn poll(&mut self) -> Poll<Option<S1::Item>, S1::Error> {
        let (a, b) = if self.flag {
            (&mut self.stream2 as &mut Stream<Item=_, Error=_>,
             &mut self.stream1 as &mut Stream<Item=_, Error=_>)
        } else {
            (&mut self.stream1 as &mut Stream<Item=_, Error=_>,
             &mut self.stream2 as &mut Stream<Item=_, Error=_>)
        };
        self.flag = !self.flag;

        let a_done = match a.poll()? {
            Async::Ready(Some(item)) => return Ok(Some(item).into()),
            Async::Ready(None) => true,
            Async::NotReady => false,
        };

        match b.poll()? {
            Async::Ready(Some(item)) => {
                // If the other stream isn't finished yet, give them a chance to
                // go first next time as we pulled something off `b`.
                if !a_done {
                    self.flag = !self.flag;
                }
                Ok(Some(item).into())
            }
            Async::Ready(None) if a_done => Ok(None.into()),
            Async::Ready(None) | Async::NotReady => Ok(Async::NotReady),
        }
    }
}
