use core::mem;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::default::Default;

use {Poll, Async};
use future::Future;
use stream::Stream;

/// A stream combinator to concatenate the results of a stream into the first
/// yielded item.
///
/// This structure is produced by the `Stream::concat2` method.
#[must_use = "streams do nothing unless polled"]
pub struct Concat2<S>
    where S: Stream,
{
    inner: ConcatSafe<S>
}

impl<S: Debug> Debug for Concat2<S> where S: Stream, S::Item: Debug {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Concat2")
            .field("inner", &self.inner)
            .finish()
    }
}

pub fn new2<S>(s: S) -> Concat2<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator + Default,
{
    Concat2 {
        inner: new_safe(s)
    }
}

impl<S> Future for Concat2<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator + Default,

{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll().map(|a| {
            match a {
                Async::NotReady => Async::NotReady,
                Async::Ready(None) => Async::Ready(Default::default()),
                Async::Ready(Some(e)) => Async::Ready(e)
            }
        })
    }
}


/// A stream combinator to concatenate the results of a stream into the first
/// yielded item.
///
/// This structure is produced by the `Stream::concat` method.
#[deprecated(since="0.1.18", note="please use `Stream::Concat2` instead")]
#[must_use = "streams do nothing unless polled"]
pub struct Concat<S>
    where S: Stream,
{
    inner: ConcatSafe<S>
}

#[allow(deprecated)]
impl<S: Debug> Debug for Concat<S> where S: Stream, S::Item: Debug {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Concat")
            .field("inner", &self.inner)
            .finish()
    }
}

#[allow(deprecated)]
pub fn new<S>(s: S) -> Concat<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator,
{
    Concat {
        inner: new_safe(s)
    }
}

#[allow(deprecated)]
impl<S> Future for Concat<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator,

{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll().map(|a| {
            match a {
                Async::NotReady => Async::NotReady,
                Async::Ready(None) => panic!("attempted concatenation of empty stream"),
                Async::Ready(Some(e)) => Async::Ready(e)
            }
        })
    }
}


#[derive(Debug)]
struct ConcatSafe<S>
    where S: Stream,
{
    stream: S,
    extend: Inner<S::Item>,
}

fn new_safe<S>(s: S) -> ConcatSafe<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator,
{
    ConcatSafe {
        stream: s,
        extend: Inner::First,
    }
}

impl<S> Future for ConcatSafe<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator,

{
    type Item = Option<S::Item>;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.stream.poll() {
                Ok(Async::Ready(Some(i))) => {
                    match self.extend {
                        Inner::First => {
                            self.extend = Inner::Extending(i);
                        },
                        Inner::Extending(ref mut e) => {
                            e.extend(i);
                        },
                        Inner::Done => unreachable!(),
                    }
                },
                Ok(Async::Ready(None)) => {
                    match mem::replace(&mut self.extend, Inner::Done) {
                        Inner::First => return Ok(Async::Ready(None)),
                        Inner::Extending(e) => return Ok(Async::Ready(Some(e))),
                        Inner::Done => panic!("cannot poll Concat again")
                    }
                },
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) => {
                    self.extend = Inner::Done;
                    return Err(e)
                }
            }
        }
    }
}


#[derive(Debug)]
enum Inner<E> {
    First,
    Extending(E),
    Done,
}
