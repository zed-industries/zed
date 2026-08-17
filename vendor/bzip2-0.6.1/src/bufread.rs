//! I/O streams for wrapping `BufRead` types as encoders/decoders

use std::io;
use std::io::prelude::*;

use crate::{Action, Compress, Compression, Decompress, Status};

/// A bz2 encoder, or compressor.
///
/// This structure implements a [`BufRead`] interface and will read uncompressed
/// data from an underlying stream and emit a stream of compressed data.
pub struct BzEncoder<R> {
    obj: R,
    data: Compress,
    done: bool,
}

/// A bz2 decoder, or decompressor.
///
/// This structure implements a [`BufRead`] interface and takes a stream of
/// compressed data as input, providing the decompressed data when read from.
pub struct BzDecoder<R> {
    obj: R,
    data: Decompress,
    done: bool,
    multi: bool,
}

impl<R: BufRead> BzEncoder<R> {
    /// Creates a new encoder which will read uncompressed data from the given
    /// stream and emit the compressed stream.
    pub fn new(r: R, level: Compression) -> Self {
        Self {
            obj: r,
            data: Compress::new(level, 30),
            done: false,
        }
    }
}

impl<R> BzEncoder<R> {
    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        &self.obj
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.obj
    }

    /// Consumes this encoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.obj
    }

    /// Returns the number of bytes produced by the compressor
    /// (e.g. the number of bytes read from this stream)
    ///
    /// Note that, due to buffering, this only bears any relation to
    /// total_in() when the compressor chooses to flush its data
    /// (unfortunately, this won't happen in general
    /// at the end of the stream, because the compressor doesn't know
    /// if there's more data to come).  At that point,
    /// `total_out() / total_in()` would be the compression ratio.
    pub fn total_out(&self) -> u64 {
        self.data.total_out()
    }

    /// Returns the number of bytes consumed by the compressor
    /// (e.g. the number of bytes read from the underlying stream)
    pub fn total_in(&self) -> u64 {
        self.data.total_in()
    }
}

impl<R: BufRead> Read for BzEncoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.done {
            return Ok(0);
        }
        loop {
            let (read, consumed, eof, ret);
            {
                let input = self.obj.fill_buf()?;
                eof = input.is_empty();
                let before_out = self.data.total_out();
                let before_in = self.data.total_in();
                let action = if eof { Action::Finish } else { Action::Run };
                ret = self.data.compress(input, buf, action);
                read = (self.data.total_out() - before_out) as usize;
                consumed = (self.data.total_in() - before_in) as usize;
            }
            self.obj.consume(consumed);

            // we should never get the sequence error that's possible to be
            // returned from compression
            let ret = ret.unwrap();

            // If we haven't ready any data and we haven't hit EOF yet, then we
            // need to keep asking for more data because if we return that 0
            // bytes of data have been read then it will be interpreted as EOF.
            if read == 0 && !eof && !buf.is_empty() {
                continue;
            }
            if ret == Status::StreamEnd {
                self.done = true;
            }
            return Ok(read);
        }
    }
}

impl<W: Write> Write for BzEncoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.get_mut().flush()
    }
}

impl<R: BufRead> BzDecoder<R> {
    /// Creates a new decoder which will decompress data read from the given
    /// stream.
    pub fn new(r: R) -> Self {
        Self {
            obj: r,
            data: Decompress::new(false),
            done: false,
            multi: false,
        }
    }

    fn multi(mut self, flag: bool) -> Self {
        self.multi = flag;
        self
    }
}

impl<R> BzDecoder<R> {
    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        &self.obj
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.obj
    }

    /// Consumes this decoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.obj
    }

    /// Returns the number of bytes that the decompressor has consumed.
    ///
    /// Note that this will likely be smaller than what the decompressor
    /// actually read from the underlying stream due to buffering.
    pub fn total_in(&self) -> u64 {
        self.data.total_in()
    }

    /// Returns the number of bytes that the decompressor has produced.
    pub fn total_out(&self) -> u64 {
        self.data.total_out()
    }
}

impl<R: BufRead> Read for BzDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            if self.done && !self.multi {
                return Ok(0);
            }
            let (read, consumed, remaining, ret);
            {
                let input = self.obj.fill_buf()?;
                if self.done {
                    assert!(self.multi);
                    if input.is_empty() {
                        // beyond last stream in multi-stream case
                        return Ok(0);
                    } else {
                        // previous stream ended, more data follows => create new decompressor
                        self.data = Decompress::new(false);
                        self.done = false;
                    }
                }
                let before_out = self.data.total_out();
                let before_in = self.data.total_in();
                ret = self.data.decompress(input, buf);
                read = (self.data.total_out() - before_out) as usize;
                consumed = (self.data.total_in() - before_in) as usize;
                remaining = input.len() - consumed;
            }
            self.obj.consume(consumed);

            let ret = ret.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            if ret == Status::StreamEnd {
                self.done = true;
            } else if consumed == 0 && remaining == 0 && read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "decompression not finished but EOF reached",
                ));
            }

            if read > 0 || buf.is_empty() {
                return Ok(read);
            }
        }
    }
}

impl<W: Write> Write for BzDecoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.get_mut().flush()
    }
}

/// A bzip2 streaming decoder that decodes all members of a multistream.
///
/// Wikipedia, particularly, uses bzip2 multistream for their dumps, and the
/// `pbzip2` tool creates such data as well.
pub struct MultiBzDecoder<R>(BzDecoder<R>);

impl<R: BufRead> MultiBzDecoder<R> {
    /// Creates a new decoder from the given reader. If the bzip2 stream contains multiple members
    /// all will be decoded.
    pub fn new(r: R) -> Self {
        Self(BzDecoder::new(r).multi(true))
    }
}

impl<R> MultiBzDecoder<R> {
    /// Acquires a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        self.0.get_ref()
    }

    /// Acquires a mutable reference to the underlying stream.
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        self.0.get_mut()
    }

    /// Consumes this decoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.0.into_inner()
    }
}

impl<R: BufRead> Read for MultiBzDecoder<R> {
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        self.0.read(into)
    }
}

impl<R: BufRead + Write> Write for MultiBzDecoder<R> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.get_mut().flush()
    }
}

#[cfg(test)]
mod tests {
    use super::MultiBzDecoder;
    use std::io::{BufReader, Read};

    #[test]
    fn bug_61() {
        let compressed_bytes = include_bytes!("../tests/bug_61.bz2");
        let uncompressed_bytes = include_bytes!("../tests/bug_61.raw");
        let reader = BufReader::with_capacity(8192, compressed_bytes.as_ref());

        let mut d = MultiBzDecoder::new(reader);
        let mut data = Vec::new();

        assert_eq!(d.read_to_end(&mut data).unwrap(), uncompressed_bytes.len());
        assert_eq!(data, uncompressed_bytes);
    }
}
