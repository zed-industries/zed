use {Future, Poll, Async};
use future::Either;

/// Future for the `select2` combinator, waiting for one of two differently-typed
/// futures to complete.
///
/// This is created by the [`Future::select2`] method.
///
/// [`Future::select2`]: trait.Future.html#method.select2
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct Select2<A, B> {
    inner: Option<(A, B)>,
}

pub fn new<A, B>(a: A, b: B) -> Select2<A, B> {
    Select2 { inner: Some((a, b)) }
}

impl<A, B> Future for Select2<A, B> where A: Future, B: Future {
    type Item = Either<(A::Item, B), (B::Item, A)>;
    type Error = Either<(A::Error, B), (B::Error, A)>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let (mut a, mut b) = self.inner.take().expect("cannot poll Select2 twice");
        match a.poll() {
            Err(e) => Err(Either::A((e, b))),
            Ok(Async::Ready(x)) => Ok(Async::Ready(Either::A((x, b)))),
            Ok(Async::NotReady) => match b.poll() {
                Err(e) => Err(Either::B((e, a))),
                Ok(Async::Ready(x)) => Ok(Async::Ready(Either::B((x, a)))),
                Ok(Async::NotReady) => {
                    self.inner = Some((a, b));
                    Ok(Async::NotReady)
                }
            }
        }
    }
}
