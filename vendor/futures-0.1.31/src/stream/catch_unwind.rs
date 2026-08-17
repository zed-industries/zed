use std::prelude::v1::*;
use std::any::Any;
use std::panic::{catch_unwind, UnwindSafe, AssertUnwindSafe};
use std::mem;

use super::super::{Poll, Async};
use super::Stream;

/// Stream for the `catch_unwind` combinator.
///
/// This is created by the `Stream::catch_unwind` method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct CatchUnwind<S> where S: Stream {
    state: CatchUnwindState<S>,
}

pub fn new<S>(stream: S) -> CatchUnwind<S>
    where S: Stream + UnwindSafe,
{
    CatchUnwind {
        state: CatchUnwindState::Stream(stream),
    }
}

#[derive(Debug)]
enum CatchUnwindState<S> {
    Stream(S),
    Eof,
    Done,
}

impl<S> Stream for CatchUnwind<S>
    where S: Stream + UnwindSafe,
{
    type Item = Result<S::Item, S::Error>;
    type Error = Box<Any + Send>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut stream = match mem::replace(&mut self.state, CatchUnwindState::Eof) {
            CatchUnwindState::Done => panic!("cannot poll after eof"),
            CatchUnwindState::Eof => {
                self.state = CatchUnwindState::Done;
                return Ok(Async::Ready(None));
            }
            CatchUnwindState::Stream(stream) => stream,
        };
        let res = catch_unwind(|| (stream.poll(), stream));
        match res {
            Err(e) => Err(e), // and state is already Eof
            Ok((poll, stream)) => {
                self.state = CatchUnwindState::Stream(stream);
                match poll {
                    Err(e) => Ok(Async::Ready(Some(Err(e)))),
                    Ok(Async::NotReady) => Ok(Async::NotReady),
                    Ok(Async::Ready(Some(r))) => Ok(Async::Ready(Some(Ok(r)))),
                    Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
                }
            }
        }
    }
}

impl<S: Stream> Stream for AssertUnwindSafe<S> {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        self.0.poll()
    }
}
