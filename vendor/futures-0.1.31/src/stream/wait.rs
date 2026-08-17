use stream::Stream;
use executor;

/// A stream combinator which converts an asynchronous stream to a **blocking
/// iterator**.
///
/// Created by the `Stream::wait` method, this function transforms any stream
/// into a standard iterator. This is implemented by blocking the current thread
/// while items on the underlying stream aren't ready yet.
#[must_use = "iterators do nothing unless advanced"]
#[derive(Debug)]
pub struct Wait<S> {
    stream: executor::Spawn<S>,
}

impl<S> Wait<S> {
    /// Acquires a reference to the underlying stream that this combinator is
    /// pulling from.
    pub fn get_ref(&self) -> &S {
        self.stream.get_ref()
    }

    /// Acquires a mutable reference to the underlying stream that this
    /// combinator is pulling from.
    ///
    /// Note that care must be taken to avoid tampering with the state of the
    /// stream which may otherwise confuse this combinator.
    pub fn get_mut(&mut self) -> &mut S {
        self.stream.get_mut()
    }

    /// Consumes this combinator, returning the underlying stream.
    ///
    /// Note that this may discard intermediate state of this combinator, so
    /// care should be taken to avoid losing resources when this is called.
    pub fn into_inner(self) -> S {
        self.stream.into_inner()
    }
}

pub fn new<S: Stream>(s: S) -> Wait<S> {
    Wait {
        stream: executor::spawn(s),
    }
}

impl<S: Stream> Iterator for Wait<S> {
    type Item = Result<S::Item, S::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.wait_stream()
    }
}
