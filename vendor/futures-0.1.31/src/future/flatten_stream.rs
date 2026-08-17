use {Async, Future, Poll};
use core::fmt;
use stream::Stream;

/// Future for the `flatten_stream` combinator, flattening a
/// future-of-a-stream to get just the result of the final stream as a stream.
///
/// This is created by the `Future::flatten_stream` method.
#[must_use = "streams do nothing unless polled"]
pub struct FlattenStream<F>
    where F: Future,
          <F as Future>::Item: Stream<Error=F::Error>,
{
    state: State<F>
}

impl<F> fmt::Debug for FlattenStream<F>
    where F: Future + fmt::Debug,
          <F as Future>::Item: Stream<Error=F::Error> + fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("FlattenStream")
            .field("state", &self.state)
            .finish()
    }
}

pub fn new<F>(f: F) -> FlattenStream<F>
    where F: Future,
          <F as Future>::Item: Stream<Error=F::Error>,
{
    FlattenStream {
        state: State::Future(f)
    }
}

#[derive(Debug)]
enum State<F>
    where F: Future,
          <F as Future>::Item: Stream<Error=F::Error>,
{
    // future is not yet called or called and not ready
    Future(F),
    // future resolved to Stream
    Stream(F::Item),
    // EOF after future resolved to error
    Eof,
    // after EOF after future resolved to error
    Done,
}

impl<F> Stream for FlattenStream<F>
    where F: Future,
          <F as Future>::Item: Stream<Error=F::Error>,
{
    type Item = <F::Item as Stream>::Item;
    type Error = <F::Item as Stream>::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            let (next_state, ret_opt) = match self.state {
                State::Future(ref mut f) => {
                    match f.poll() {
                        Ok(Async::NotReady) => {
                            // State is not changed, early return.
                            return Ok(Async::NotReady)
                        },
                        Ok(Async::Ready(stream)) => {
                            // Future resolved to stream.
                            // We do not return, but poll that
                            // stream in the next loop iteration.
                            (State::Stream(stream), None)
                        }
                        Err(e) => {
                            (State::Eof, Some(Err(e)))
                        }
                    }
                }
                State::Stream(ref mut s) => {
                    // Just forward call to the stream,
                    // do not track its state.
                    return s.poll();
                }
                State::Eof => {
                    (State::Done, Some(Ok(Async::Ready(None))))
                }
                State::Done => {
                    panic!("poll called after eof");
                }
            };

            self.state = next_state;
            if let Some(ret) = ret_opt {
                return ret;
            }
        }
    }
}

