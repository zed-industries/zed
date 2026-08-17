//! Definition of the `SelectOk` combinator, finding the first successful future
//! in a list.

use std::mem;
use std::prelude::v1::*;

use {Future, IntoFuture, Poll, Async};

/// Future for the `select_ok` combinator, waiting for one of any of a list of
/// futures to successfully complete. Unlike `select_all`, this future ignores all
/// but the last error, if there are any.
///
/// This is created by the `select_ok` function.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct SelectOk<A> where A: Future {
    inner: Vec<A>,
}

/// Creates a new future which will select the first successful future over a list of futures.
///
/// The returned future will wait for any future within `iter` to be ready and Ok. Unlike
/// `select_all`, this will only return the first successful completion, or the last
/// failure. This is useful in contexts where any success is desired and failures
/// are ignored, unless all the futures fail.
///
/// # Panics
///
/// This function will panic if the iterator specified contains no items.
pub fn select_ok<I>(iter: I) -> SelectOk<<I::Item as IntoFuture>::Future>
    where I: IntoIterator,
          I::Item: IntoFuture,
{
    let ret = SelectOk {
        inner: iter.into_iter()
                   .map(|a| a.into_future())
                   .collect(),
    };
    assert!(ret.inner.len() > 0);
    ret
}

impl<A> Future for SelectOk<A> where A: Future {
    type Item = (A::Item, Vec<A>);
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // loop until we've either exhausted all errors, a success was hit, or nothing is ready
        loop {
            let item = self.inner.iter_mut().enumerate().filter_map(|(i, f)| {
                match f.poll() {
                    Ok(Async::NotReady) => None,
                    Ok(Async::Ready(e)) => Some((i, Ok(e))),
                    Err(e) => Some((i, Err(e))),
                }
            }).next();

            match item {
                Some((idx, res)) => {
                    // always remove Ok or Err, if it's not the last Err continue looping
                    drop(self.inner.remove(idx));
                    match res {
                        Ok(e) => {
                            let rest = mem::replace(&mut self.inner, Vec::new());
                            return Ok(Async::Ready((e, rest)))
                        },
                        Err(e) => {
                            if self.inner.is_empty() {
                                return Err(e)
                            }
                        },
                    }
                }
                None => {
                    // based on the filter above, nothing is ready, return
                    return Ok(Async::NotReady)
                },
            }
        }
    }
}
