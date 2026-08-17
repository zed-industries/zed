use std::io::{self, BufRead};
use std::mem;

use futures::{Poll, Stream};

use AsyncRead;

/// Combinator created by the top-level `lines` method which is a stream over
/// the lines of text on an I/O object.
#[derive(Debug)]
pub struct Lines<A> {
    io: A,
    line: String,
}

/// Creates a new stream from the I/O object given representing the lines of
/// input that are found on `A`.
///
/// This method takes an asynchronous I/O object, `a`, and returns a `Stream` of
/// lines that the object contains. The returned stream will reach its end once
/// `a` reaches EOF.
pub fn lines<A>(a: A) -> Lines<A>
where
    A: AsyncRead + BufRead,
{
    Lines {
        io: a,
        line: String::new(),
    }
}

impl<A> Lines<A> {
    /// Returns the underlying I/O object.
    ///
    /// Note that this may lose data already read into internal buffers. It's
    /// recommended to only call this once the stream has reached its end.
    pub fn into_inner(self) -> A {
        self.io
    }
}

impl<A> Stream for Lines<A>
where
    A: AsyncRead + BufRead,
{
    type Item = String;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<String>, io::Error> {
        let n = try_nb!(self.io.read_line(&mut self.line));
        if n == 0 && self.line.len() == 0 {
            return Ok(None.into());
        }
        if self.line.ends_with("\n") {
            self.line.pop();
            if self.line.ends_with("\r") {
                self.line.pop();
            }
        }
        Ok(Some(mem::replace(&mut self.line, String::new())).into())
    }
}
