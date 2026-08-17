use std::cmp::{Eq, PartialEq, PartialOrd, Ord, Ordering};
use std::collections::BinaryHeap;
use std::fmt::{self, Debug};
use std::iter::FromIterator;

use {Async, Future, IntoFuture, Poll, Stream};
use stream::FuturesUnordered;

#[derive(Debug)]
struct OrderWrapper<T> {
    item: T,
    index: usize,
}

impl<T> PartialEq for OrderWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> Eq for OrderWrapper<T> {}

impl<T> PartialOrd for OrderWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for OrderWrapper<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max heap, so compare backwards here.
        other.index.cmp(&self.index)
    }
}

impl<T> Future for OrderWrapper<T>
    where T: Future
{
    type Item = OrderWrapper<T::Item>;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let result = try_ready!(self.item.poll());
        Ok(Async::Ready(OrderWrapper {
            item: result,
            index: self.index
        }))
    }
}

/// An unbounded queue of futures.
///
/// This "combinator" is similar to `FuturesUnordered`, but it imposes an order
/// on top of the set of futures. While futures in the set will race to
/// completion in parallel, results will only be returned in the order their
/// originating futures were added to the queue.
///
/// Futures are pushed into this queue and their realized values are yielded in
/// order. This structure is optimized to manage a large number of futures.
/// Futures managed by `FuturesOrdered` will only be polled when they generate
/// notifications. This reduces the required amount of work needed to coordinate
/// large numbers of futures.
///
/// When a `FuturesOrdered` is first created, it does not contain any futures.
/// Calling `poll` in this state will result in `Ok(Async::Ready(None))` to be
/// returned. Futures are submitted to the queue using `push`; however, the
/// future will **not** be polled at this point. `FuturesOrdered` will only
/// poll managed futures when `FuturesOrdered::poll` is called. As such, it
/// is important to call `poll` after pushing new futures.
///
/// If `FuturesOrdered::poll` returns `Ok(Async::Ready(None))` this means that
/// the queue is currently not managing any futures. A future may be submitted
/// to the queue at a later time. At that point, a call to
/// `FuturesOrdered::poll` will either return the future's resolved value
/// **or** `Ok(Async::NotReady)` if the future has not yet completed. When
/// multiple futures are submitted to the queue, `FuturesOrdered::poll` will
/// return `Ok(Async::NotReady)` until the first future completes, even if
/// some of the later futures have already completed.
///
/// Note that you can create a ready-made `FuturesOrdered` via the
/// `futures_ordered` function in the `stream` module, or you can start with an
/// empty queue with the `FuturesOrdered::new` constructor.
#[must_use = "streams do nothing unless polled"]
pub struct FuturesOrdered<T>
    where T: Future
{
    in_progress: FuturesUnordered<OrderWrapper<T>>,
    queued_results: BinaryHeap<OrderWrapper<T::Item>>,
    next_incoming_index: usize,
    next_outgoing_index: usize,
}

/// Converts a list of futures into a `Stream` of results from the futures.
///
/// This function will take an list of futures (e.g. a vector, an iterator,
/// etc), and return a stream. The stream will yield items as they become
/// available on the futures internally, in the order that their originating
/// futures were submitted to the queue. If the futures complete out of order,
/// items will be stored internally within `FuturesOrdered` until all preceding
/// items have been yielded.
///
/// Note that the returned queue can also be used to dynamically push more
/// futures into the queue as they become available.
pub fn futures_ordered<I>(futures: I) -> FuturesOrdered<<I::Item as IntoFuture>::Future>
    where I: IntoIterator,
          I::Item: IntoFuture
{
    let mut queue = FuturesOrdered::new();

    for future in futures {
        queue.push(future.into_future());
    }

    return queue
}

impl<T> Default for FuturesOrdered<T> where T: Future {
    fn default() -> Self {
        FuturesOrdered::new()
    }
}

impl<T> FuturesOrdered<T>
    where T: Future
{
    /// Constructs a new, empty `FuturesOrdered`
    ///
    /// The returned `FuturesOrdered` does not contain any futures and, in this
    /// state, `FuturesOrdered::poll` will return `Ok(Async::Ready(None))`.
    pub fn new() -> FuturesOrdered<T> {
        FuturesOrdered {
            in_progress: FuturesUnordered::new(),
            queued_results: BinaryHeap::new(),
            next_incoming_index: 0,
            next_outgoing_index: 0,
        }
    }

    /// Returns the number of futures contained in the queue.
    ///
    /// This represents the total number of in-flight futures, both
    /// those currently processing and those that have completed but
    /// which are waiting for earlier futures to complete.
    pub fn len(&self) -> usize {
        self.in_progress.len() + self.queued_results.len()
    }

    /// Returns `true` if the queue contains no futures
    pub fn is_empty(&self) -> bool {
        self.in_progress.is_empty() && self.queued_results.is_empty()
    }

    /// Push a future into the queue.
    ///
    /// This function submits the given future to the internal set for managing.
    /// This function will not call `poll` on the submitted future. The caller
    /// must ensure that `FuturesOrdered::poll` is called in order to receive
    /// task notifications.
    pub fn push(&mut self, future: T) {
        let wrapped = OrderWrapper {
            item: future,
            index: self.next_incoming_index,
        };
        self.next_incoming_index += 1;
        self.in_progress.push(wrapped);
    }
}

impl<T> Stream for FuturesOrdered<T>
    where T: Future
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // Get any completed futures from the unordered set.
        loop {
            match self.in_progress.poll()? {
                Async::Ready(Some(result)) => self.queued_results.push(result),
                Async::Ready(None) | Async::NotReady => break,
            }
        }

        if let Some(next_result) = self.queued_results.peek() {
            // PeekMut::pop is not stable yet QQ
            if next_result.index != self.next_outgoing_index {
                return Ok(Async::NotReady);
            }
        } else if !self.in_progress.is_empty() {
            return Ok(Async::NotReady);
        } else {
            return Ok(Async::Ready(None));
        }

        let next_result = self.queued_results.pop().unwrap();
        self.next_outgoing_index += 1;
        Ok(Async::Ready(Some(next_result.item)))
    }
}

impl<T: Debug> Debug for FuturesOrdered<T>
    where T: Future
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "FuturesOrdered {{ ... }}")
    }
}

impl<F: Future> FromIterator<F> for FuturesOrdered<F> {
    fn from_iter<T>(iter: T) -> Self 
        where T: IntoIterator<Item = F>
    {
        let mut new = FuturesOrdered::new();
        for future in iter.into_iter() {
            new.push(future);
        }
        new
    }
}
