use {Future, IntoFuture, Poll};
use super::chain::Chain;

/// Future for the `or_else` combinator, chaining a computation onto the end of
/// a future which fails with an error.
///
/// This is created by the `Future::or_else` method.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct OrElse<A, B, F> where A: Future, B: IntoFuture {
    state: Chain<A, B::Future, F>,
}

pub fn new<A, B, F>(future: A, f: F) -> OrElse<A, B, F>
    where A: Future,
          B: IntoFuture<Item=A::Item>,
{
    OrElse {
        state: Chain::new(future, f),
    }
}

impl<A, B, F> Future for OrElse<A, B, F>
    where A: Future,
          B: IntoFuture<Item=A::Item>,
          F: FnOnce(A::Error) -> B,
{
    type Item = B::Item;
    type Error = B::Error;

    fn poll(&mut self) -> Poll<B::Item, B::Error> {
        self.state.poll(|a, f| {
            match a {
                Ok(item) => Ok(Ok(item)),
                Err(e) => Ok(Err(f(e).into_future()))
            }
        })
    }
}
