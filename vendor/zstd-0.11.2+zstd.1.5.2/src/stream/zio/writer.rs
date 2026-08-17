use std::io::{self, Write};

use crate::stream::raw::{InBuffer, Operation, OutBuffer};

// input -> [ zstd -> buffer -> writer ]

/// Implements the [`Write`] API around an [`Operation`].
///
/// This can be used to wrap a raw in-memory operation in a write-focused API.
///
/// It can be used with either compression or decompression, and forwards the
/// output to a wrapped `Write`.
pub struct Writer<W, D> {
    writer: W,
    operation: D,

    offset: usize,
    buffer: Vec<u8>,

    // When `true`, indicates that nothing should be added to the buffer.
    // All that's left if to empty the buffer.
    finished: bool,

    finished_frame: bool,
}

impl<W, D> Writer<W, D>
where
    W: Write,
    D: Operation,
{
    /// Creates a new `Writer`.
    ///
    /// All output from the given operation will be forwarded to `writer`.
    pub fn new(writer: W, operation: D) -> Self {
        Writer {
            writer,
            operation,

            offset: 0,
            // 32KB buffer? That's what flate2 uses
            buffer: Vec::with_capacity(32 * 1024),

            finished: false,
            finished_frame: false,
        }
    }

    /// Ends the stream.
    ///
    /// This *must* be called after all data has been written to finish the
    /// stream.
    ///
    /// If you forget to call this and just drop the `Writer`, you *will* have
    /// an incomplete output.
    ///
    /// Keep calling it until it returns `Ok(())`, then don't call it again.
    pub fn finish(&mut self) -> io::Result<()> {
        loop {
            // Keep trying until we're really done.
            self.write_from_offset()?;

            // At this point the buffer has been fully written out.

            if self.finished {
                return Ok(());
            }

            // Let's fill this buffer again!

            let finished_frame = self.finished_frame;
            let hint =
                self.with_buffer(|dst, op| op.finish(dst, finished_frame));
            self.offset = 0;
            // println!("Hint: {:?}\nOut:{:?}", hint, &self.buffer);

            // We return here if zstd had a problem.
            // Could happen with invalid data, ...
            let hint = hint?;

            if hint != 0 && self.buffer.is_empty() {
                // This happens if we are decoding an incomplete frame.
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "incomplete frame",
                ));
            }

            // println!("Finishing {}, {}", bytes_written, hint);

            self.finished = hint == 0;
        }
    }

    /// Run the given closure on `self.buffer`.
    ///
    /// The buffer will be cleared, and made available wrapped in an `OutBuffer`.
    fn with_buffer<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut OutBuffer<'_, Vec<u8>>, &mut D) -> T,
    {
        self.buffer.clear();
        let mut output = OutBuffer::around(&mut self.buffer);
        // eprintln!("Output: {:?}", output);
        f(&mut output, &mut self.operation)
    }

    /// Attempt to write `self.buffer` to the wrapped writer.
    ///
    /// Returns `Ok(())` once all the buffer has been written.
    fn write_from_offset(&mut self) -> io::Result<()> {
        // The code looks a lot like `write_all`, but keeps track of what has
        // been written in case we're interrupted.
        while self.offset < self.buffer.len() {
            match self.writer.write(&self.buffer[self.offset..]) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "writer will not accept any more data",
                    ))
                }
                Ok(n) => self.offset += n,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Return the wrapped `Writer` and `Operation`.
    ///
    /// Careful: if you call this before calling [`Writer::finish()`], the
    /// output may be incomplete.
    pub fn into_inner(self) -> (W, D) {
        (self.writer, self.operation)
    }

    /// Gives a reference to the inner writer.
    pub fn writer(&self) -> &W {
        &self.writer
    }

    /// Gives a mutable reference to the inner writer.
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Gives a reference to the inner operation.
    pub fn operation(&self) -> &D {
        &self.operation
    }

    /// Gives a mutable reference to the inner operation.
    pub fn operation_mut(&mut self) -> &mut D {
        &mut self.operation
    }

    /// Returns the offset in the current buffer. Only useful for debugging.
    #[cfg(test)]
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the current buffer. Only useful for debugging.
    #[cfg(test)]
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl<W, D> Write for Writer<W, D>
where
    W: Write,
    D: Operation,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Keep trying until _something_ has been consumed.
        // As soon as some input has been taken, we cannot afford
        // to take any chance: if an error occurs, the user couldn't know
        // that some data _was_ successfully written.
        loop {
            // First, write any pending data from `self.buffer`.
            self.write_from_offset()?;
            // At this point `self.buffer` can safely be discarded.

            // Support writing concatenated frames by re-initializing the
            // context.
            if self.finished_frame {
                self.operation.reinit()?;
                self.finished_frame = false;
            }

            let mut src = InBuffer::around(buf);
            let hint = self.with_buffer(|dst, op| op.run(&mut src, dst));
            let bytes_read = src.pos;

            // eprintln!(
            //     "Write Hint: {:?}\n src: {:?}\n dst: {:?}",
            //     hint, src, self.buffer
            // );

            self.offset = 0;
            let hint = hint?;

            if hint == 0 {
                self.finished_frame = true;
            }

            // As we said, as soon as we've consumed something, return.
            if bytes_read > 0 || buf.is_empty() {
                // println!("Returning {}", bytes_read);
                return Ok(bytes_read);
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut finished = self.finished;
        loop {
            // If the output is blocked or has an error, return now.
            self.write_from_offset()?;

            if finished {
                return Ok(());
            }

            let hint = self.with_buffer(|dst, op| op.flush(dst));

            self.offset = 0;
            let hint = hint?;

            finished = hint == 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Writer;
    use std::io::Write;

    #[test]
    fn test_noop() {
        use crate::stream::raw::NoOp;

        let input = b"AbcdefghAbcdefgh.";

        // Test writer
        let mut output = Vec::new();
        {
            let mut writer = Writer::new(&mut output, NoOp);
            writer.write_all(input).unwrap();
            writer.finish().unwrap();
        }
        assert_eq!(&output, input);
    }

    #[test]
    fn test_compress() {
        use crate::stream::raw::Encoder;

        let input = b"AbcdefghAbcdefgh.";

        // Test writer
        let mut output = Vec::new();
        {
            let mut writer =
                Writer::new(&mut output, Encoder::new(1).unwrap());
            writer.write_all(input).unwrap();
            writer.finish().unwrap();
        }
        // println!("Output: {:?}", output);
        let decoded = crate::decode_all(&output[..]).unwrap();
        assert_eq!(&decoded, input);
    }

    #[test]
    fn test_decompress() {
        use crate::stream::raw::Decoder;

        let input = b"AbcdefghAbcdefgh.";
        let compressed = crate::encode_all(&input[..], 1).unwrap();

        // Test writer
        let mut output = Vec::new();
        {
            let mut writer = Writer::new(&mut output, Decoder::new().unwrap());
            writer.write_all(&compressed).unwrap();
            writer.finish().unwrap();
        }
        // println!("Output: {:?}", output);
        assert_eq!(&output, input);
    }
}
