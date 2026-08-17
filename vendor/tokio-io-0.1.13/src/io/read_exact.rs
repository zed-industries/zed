use std::io;
use std::mem;

use futures::{Future, Poll};

use AsyncRead;

/// A future which can be used to easily read exactly enough bytes to fill
/// a buffer.
///
/// Created by the [`read_exact`] function.
///
/// [`read_exact`]: fn.read_exact.html
#[derive(Debug)]
pub struct ReadExact<A, T> {
    state: State<A, T>,
}

#[derive(Debug)]
enum State<A, T> {
    Reading { a: A, buf: T, pos: usize },
    Empty,
}

/// Creates a future which will read exactly enough bytes to fill `buf`,
/// returning an error if EOF is hit sooner.
///
/// The returned future will resolve to both the I/O stream as well as the
/// buffer once the read operation is completed.
///
/// In the case of an error the buffer and the object will be discarded, with
/// the error yielded. In the case of success the object will be destroyed and
/// the buffer will be returned, with all data read from the stream appended to
/// the buffer.
pub fn read_exact<A, T>(a: A, buf: T) -> ReadExact<A, T>
where
    A: AsyncRead,
    T: AsMut<[u8]>,
{
    ReadExact {
        state: State::Reading {
            a: a,
            buf: buf,
            pos: 0,
        },
    }
}

fn eof() -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, "early eof")
}

impl<A, T> Future for ReadExact<A, T>
where
    A: AsyncRead,
    T: AsMut<[u8]>,
{
    type Item = (A, T);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(A, T), io::Error> {
        match self.state {
            State::Reading {
                ref mut a,
                ref mut buf,
                ref mut pos,
            } => {
                let buf = buf.as_mut();
                while *pos < buf.len() {
                    let n = try_ready!(a.poll_read(&mut buf[*pos..]));
                    *pos += n;
                    if n == 0 {
                        return Err(eof());
                    }
                }
            }
            State::Empty => panic!("poll a ReadExact after it's done"),
        }

        match mem::replace(&mut self.state, State::Empty) {
            State::Reading { a, buf, .. } => Ok((a, buf).into()),
            State::Empty => panic!(),
        }
    }
}
