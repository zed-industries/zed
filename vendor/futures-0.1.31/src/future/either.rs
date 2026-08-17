use {Future, Poll, Stream};

/// Combines two different futures yielding the same item and error
/// types into a single type.
#[derive(Debug)]
pub enum Either<A, B> {
    /// First branch of the type
    A(A),
    /// Second branch of the type
    B(B),
}

impl<T, A, B> Either<(T, A), (T, B)> {
    /// Splits out the homogeneous type from an either of tuples.
    ///
    /// This method is typically useful when combined with the `Future::select2`
    /// combinator.
    pub fn split(self) -> (T, Either<A, B>) {
        match self {
            Either::A((a, b)) => (a, Either::A(b)),
            Either::B((a, b)) => (a, Either::B(b)),
        }
    }
}

impl<A, B> Future for Either<A, B>
    where A: Future,
          B: Future<Item = A::Item, Error = A::Error>
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<A::Item, A::Error> {
        match *self {
            Either::A(ref mut a) => a.poll(),
            Either::B(ref mut b) => b.poll(),
        }
    }
}

impl<A, B> Stream for Either<A, B>
    where A: Stream,
          B: Stream<Item = A::Item, Error = A::Error>
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<A::Item>, A::Error> {
        match *self {
            Either::A(ref mut a) => a.poll(),
            Either::B(ref mut b) => b.poll(),
        }
    }
}
