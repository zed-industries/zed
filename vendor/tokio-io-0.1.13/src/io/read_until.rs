use std::io::{self, BufRead};
use std::mem;

use futures::{Future, Poll};

use AsyncRead;

/// A future which can be used to easily read the contents of a stream into a
/// vector until the delimiter is reached.
///
/// Created by the [`read_until`] function.
///
/// [`read_until`]: fn.read_until.html
#[derive(Debug)]
pub struct ReadUntil<A> {
    state: State<A>,
}

#[derive(Debug)]
enum State<A> {
    Reading { a: A, byte: u8, buf: Vec<u8> },
    Empty,
}

/// Creates a future which will read all the bytes associated with the I/O
/// object `A` into the buffer provided until the delimiter `byte` is reached.
/// This method is the async equivalent to [`BufRead::read_until`].
///
/// In case of an error the buffer and the object will be discarded, with
/// the error yielded. In the case of success the object will be destroyed and
/// the buffer will be returned, with all bytes up to, and including, the delimiter
/// (if found).
///
/// [`BufRead::read_until`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_until
pub fn read_until<A>(a: A, byte: u8, buf: Vec<u8>) -> ReadUntil<A>
where
    A: AsyncRead + BufRead,
{
    ReadUntil {
        state: State::Reading {
            a: a,
            byte: byte,
            buf: buf,
        },
    }
}

impl<A> Future for ReadUntil<A>
where
    A: AsyncRead + BufRead,
{
    type Item = (A, Vec<u8>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(A, Vec<u8>), io::Error> {
        match self.state {
            State::Reading {
                ref mut a,
                byte,
                ref mut buf,
            } => {
                // If we get `Ok(n)`, then we know the stream hit EOF or the delimiter.
                // and just return it, as we are finished.
                // If we hit "would block" then all the read data so far
                // is in our buffer, and otherwise we propagate errors.
                try_nb!(a.read_until(byte, buf));
            }
            State::Empty => panic!("poll ReadUntil after it's done"),
        }

        match mem::replace(&mut self.state, State::Empty) {
            State::Reading { a, byte: _, buf } => Ok((a, buf).into()),
            State::Empty => unreachable!(),
        }
    }
}
