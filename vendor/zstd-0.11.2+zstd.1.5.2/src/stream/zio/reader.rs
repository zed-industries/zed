use std::io::{self, BufRead, Read};

use crate::stream::raw::{InBuffer, Operation, OutBuffer};

// [ reader -> zstd ] -> output
/// Implements the [`Read`] API around an [`Operation`].
///
/// This can be used to wrap a raw in-memory operation in a read-focused API.
///
/// It can wrap either a compression or decompression operation, and pulls
/// input data from a wrapped `Read`.
pub struct Reader<R, D> {
    reader: R,
    operation: D,

    state: State,

    single_frame: bool,
    finished_frame: bool,
}

enum State {
    // Still actively reading from the inner `Read`
    Reading,
    // We reached EOF from the inner `Read`, now flushing.
    PastEof,
    // We are fully done, nothing can be read.
    Finished,
}

impl<R, D> Reader<R, D> {
    /// Creates a new `Reader`.
    ///
    /// `reader` will be used to pull input data for the given operation.
    pub fn new(reader: R, operation: D) -> Self {
        Reader {
            reader,
            operation,
            state: State::Reading,
            single_frame: false,
            finished_frame: false,
        }
    }

    /// Sets `self` to stop after the first decoded frame.
    pub fn set_single_frame(&mut self) {
        self.single_frame = true;
    }

    /// Returns a mutable reference to the underlying operation.
    pub fn operation_mut(&mut self) -> &mut D {
        &mut self.operation
    }

    /// Returns a mutable reference to the underlying reader.
    pub fn reader_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Returns a reference to the underlying reader.
    pub fn reader(&self) -> &R {
        &self.reader
    }

    /// Returns the inner reader.
    pub fn into_inner(self) -> R {
        self.reader
    }
}
// Read and retry on Interrupted errors.
fn fill_buf<R>(reader: &mut R) -> io::Result<&[u8]>
where
    R: BufRead,
{
    // This doesn't work right now because of the borrow-checker.
    // When it can be made to compile, it would allow Reader to automatically
    // retry on `Interrupted` error.
    /*
    loop {
        match reader.fill_buf() {
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            otherwise => return otherwise,
        }
    }
    */

    // Workaround for now
    let res = reader.fill_buf()?;

    // eprintln!("Filled buffer: {:?}", res);

    Ok(res)
}

impl<R, D> Read for Reader<R, D>
where
    R: BufRead,
    D: Operation,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Keep trying until _something_ has been written.
        let mut first = true;
        loop {
            match self.state {
                State::Reading => {
                    let (bytes_read, bytes_written) = {
                        // Start with a fresh pool of un-processed data.
                        // This is the only line that can return an interruption error.
                        let input = if first {
                            // eprintln!("First run, no input coming.");
                            b""
                        } else {
                            fill_buf(&mut self.reader)?
                        };

                        // eprintln!("Input = {:?}", input);

                        // It's possible we don't have any new data to read.
                        // (In this case we may still have zstd's own buffer to clear.)
                        if !first && input.is_empty() {
                            self.state = State::PastEof;
                            continue;
                        }
                        first = false;

                        let mut src = InBuffer::around(input);
                        let mut dst = OutBuffer::around(buf);

                        // We don't want empty input (from first=true) to cause a frame
                        // re-initialization.
                        if self.finished_frame && !input.is_empty() {
                            // eprintln!("!! Reigniting !!");
                            self.operation.reinit()?;
                            self.finished_frame = false;
                        }

                        // Phase 1: feed input to the operation
                        let hint = self.operation.run(&mut src, &mut dst)?;
                        // eprintln!(
                        //     "Hint={} Just run our operation:\n In={:?}\n Out={:?}",
                        //     hint, src, dst
                        // );

                        if hint == 0 {
                            // In practice this only happens when decoding, when we just finished
                            // reading a frame.
                            self.finished_frame = true;
                            if self.single_frame {
                                self.state = State::Finished;
                            }
                        }

                        // eprintln!("Output: {:?}", dst);

                        (src.pos(), dst.pos())
                    };

                    self.reader.consume(bytes_read);

                    if bytes_written > 0 {
                        return Ok(bytes_written);
                    }

                    // We need more data! Try again!
                }
                State::PastEof => {
                    let mut dst = OutBuffer::around(buf);

                    // We already sent all the input we could get to zstd. Time to flush out the
                    // buffer and be done with it.

                    // Phase 2: flush out the operation's buffer
                    // Keep calling `finish()` until the buffer is empty.
                    let hint = self
                        .operation
                        .finish(&mut dst, self.finished_frame)?;
                    // eprintln!("Hint: {} ; Output: {:?}", hint, dst);
                    if hint == 0 {
                        // This indicates that the footer is complete.
                        // This is the only way to terminate the stream cleanly.
                        self.state = State::Finished;
                    }

                    return Ok(dst.pos());
                }
                State::Finished => {
                    return Ok(0);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;
    use std::io::{Cursor, Read};

    #[test]
    fn test_noop() {
        use crate::stream::raw::NoOp;

        let input = b"AbcdefghAbcdefgh.";

        // Test reader
        let mut output = Vec::new();
        {
            let mut reader = Reader::new(Cursor::new(input), NoOp);
            reader.read_to_end(&mut output).unwrap();
        }
        assert_eq!(&output, input);
    }

    #[test]
    fn test_compress() {
        use crate::stream::raw::Encoder;

        let input = b"AbcdefghAbcdefgh.";

        // Test reader
        let mut output = Vec::new();
        {
            let mut reader =
                Reader::new(Cursor::new(input), Encoder::new(1).unwrap());
            reader.read_to_end(&mut output).unwrap();
        }
        // eprintln!("{:?}", output);
        let decoded = crate::decode_all(&output[..]).unwrap();
        assert_eq!(&decoded, input);
    }
}
