use {Future, Poll, Async};

/// Future for the `select` combinator, waiting for one of two futures to
/// complete.
///
/// This is created by the `Future::select` method.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Select<A, B> where A: Future, B: Future<Item=A::Item, Error=A::Error> {
    inner: Option<(A, B)>,
}

/// Future yielded as the second result in a `Select` future.
///
/// This sentinel future represents the completion of the second future to a
/// `select` which finished second.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct SelectNext<A, B> where A: Future, B: Future<Item=A::Item, Error=A::Error> {
    inner: OneOf<A, B>,
}

#[derive(Debug)]
enum OneOf<A, B> where A: Future, B: Future {
    A(A),
    B(B),
}

pub fn new<A, B>(a: A, b: B) -> Select<A, B>
    where A: Future,
          B: Future<Item=A::Item, Error=A::Error>
{
    Select {
        inner: Some((a, b)),
    }
}

impl<A, B> Future for Select<A, B>
    where A: Future,
          B: Future<Item=A::Item, Error=A::Error>,
{
    type Item = (A::Item, SelectNext<A, B>);
    type Error = (A::Error, SelectNext<A, B>);

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let (ret, is_a) = match self.inner {
            Some((ref mut a, ref mut b)) => {
                match a.poll() {
                    Err(a) => (Err(a), true),
                    Ok(Async::Ready(a)) => (Ok(a), true),
                    Ok(Async::NotReady) => {
                        match b.poll() {
                            Err(a) => (Err(a), false),
                            Ok(Async::Ready(a)) => (Ok(a), false),
                            Ok(Async::NotReady) => return Ok(Async::NotReady),
                        }
                    }
                }
            }
            None => panic!("cannot poll select twice"),
        };

        let (a, b) = self.inner.take().unwrap();
        let next = if is_a {OneOf::B(b)} else {OneOf::A(a)};
        let next = SelectNext { inner: next };
        match ret {
            Ok(a) => Ok(Async::Ready((a, next))),
            Err(e) => Err((e, next)),
        }
    }
}

impl<A, B> Future for SelectNext<A, B>
    where A: Future,
          B: Future<Item=A::Item, Error=A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.inner {
            OneOf::A(ref mut a) => a.poll(),
            OneOf::B(ref mut b) => b.poll(),
        }
    }
}
