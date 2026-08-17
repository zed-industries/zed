use {Async, Future, IntoFuture, Poll};
use stream::Stream;

/// A stream combinator which executes a unit closure over each item on a
/// stream.
///
/// This structure is returned by the `Stream::for_each` method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct ForEach<S, F, U> where U: IntoFuture {
    stream: S,
    f: F,
    fut: Option<U::Future>,
}

pub fn new<S, F, U>(s: S, f: F) -> ForEach<S, F, U>
    where S: Stream,
          F: FnMut(S::Item) -> U,
          U: IntoFuture<Item = (), Error = S::Error>,
{
    ForEach {
        stream: s,
        f: f,
        fut: None,
    }
}

impl<S, F, U> Future for ForEach<S, F, U>
    where S: Stream,
          F: FnMut(S::Item) -> U,
          U: IntoFuture<Item= (), Error = S::Error>,
{
    type Item = ();
    type Error = S::Error;

    fn poll(&mut self) -> Poll<(), S::Error> {
        loop {
            if let Some(mut fut) = self.fut.take() {
                if fut.poll()?.is_not_ready() {
                    self.fut = Some(fut);
                    return Ok(Async::NotReady);
                }
            }

            match try_ready!(self.stream.poll()) {
                Some(e) => self.fut = Some((self.f)(e).into_future()),
                None => return Ok(Async::Ready(())),
            }
        }
    }
}
