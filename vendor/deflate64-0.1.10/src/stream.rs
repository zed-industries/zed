// TODO: move this module to deflate64 crate

use crate::InflaterManaged;
use std::io::{self, BufRead, BufReader, Read};

/// The reader the decompresses deflate64 from another BufRead.
pub struct Deflate64Decoder<R> {
    inner: R,
    inflater: Box<InflaterManaged>,
}

impl<R: Read> Deflate64Decoder<BufReader<R>> {
    /// Creates Deflate64Decoder with Read
    pub fn new(inner: R) -> Self {
        Self::with_buffer(BufReader::new(inner))
    }
}

impl<R: BufRead> Deflate64Decoder<R> {
    /// Creates Deflate64Decoder with BufRead
    pub fn with_buffer(inner: R) -> Self {
        Self {
            inner,
            inflater: Box::new(InflaterManaged::new()),
        }
    }
}

impl<R> Deflate64Decoder<R> {
    /// Returns inner BufRead instance
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Returns reference to innner BufRead instance
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Returns mutable reference to innner BufRead instance
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }
}

impl<R: BufRead> Read for Deflate64Decoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            // we received empty buffer, so it won't be possible to write anything
            return Ok(0);
        }

        loop {
            let input = self.inner.fill_buf()?;
            let eof = input.is_empty();

            let result = self.inflater.inflate(input, buf);

            self.inner.consume(result.bytes_consumed);

            if result.data_error {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid deflate64",
                ));
            }

            if result.bytes_written == 0 && !eof && !self.inflater.finished() {
                // if we haven't ready any data and we haven't hit EOF yet,
                // ask again. We must not return 0 in such case
                continue;
            }

            return Ok(result.bytes_written);
        }
    }
}
