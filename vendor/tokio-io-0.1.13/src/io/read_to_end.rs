use std::io;
use std::mem;

use futures::{Future, Poll};

use AsyncRead;

/// A future which can be used to easily read the entire contents of a stream
/// into a vector.
///
/// Created by the [`read_to_end`] function.
///
/// [`read_to_end`]: fn.read_to_end.html
#[derive(Debug)]
pub struct ReadToEnd<A> {
    state: State<A>,
}

#[derive(Debug)]
enum State<A> {
    Reading { a: A, buf: Vec<u8> },
    Empty,
}

/// Creates a future which will read all the bytes associated with the I/O
/// object `A` into the buffer provided.
///
/// In the case of an error the buffer and the object will be discarded, with
/// the error yielded. In the case of success both the object and the buffer
/// will be returned, with all data read from the stream appended to the buffer.
pub fn read_to_end<A>(a: A, buf: Vec<u8>) -> ReadToEnd<A>
where
    A: AsyncRead,
{
    ReadToEnd {
        state: State::Reading { a: a, buf: buf },
    }
}

impl<A> Future for ReadToEnd<A>
where
    A: AsyncRead,
{
    type Item = (A, Vec<u8>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(A, Vec<u8>), io::Error> {
        match self.state {
            State::Reading {
                ref mut a,
                ref mut buf,
            } => {
                // If we get `Ok`, then we know the stream hit EOF and we're done. If we
                // hit "would block" then all the read data so far is in our buffer, and
                // otherwise we propagate errors
                try_nb!(a.read_to_end(buf));
            }
            State::Empty => panic!("poll ReadToEnd after it's done"),
        }

        match mem::replace(&mut self.state, State::Empty) {
            State::Reading { a, buf } => Ok((a, buf).into()),
            State::Empty => unreachable!(),
        }
    }
}
