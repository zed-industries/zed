//! Writer-based compression/decompression streams

use std::io;
use std::io::prelude::*;

#[cfg(feature = "tokio")]
use futures::Poll;
#[cfg(feature = "tokio")]
use tokio_io::{AsyncRead, AsyncWrite};

use {Action, Compress, Compression, Decompress, Status};

/// A compression stream which will have uncompressed data written to it and
/// will write compressed data to an output stream.
pub struct BzEncoder<W: Write> {
    data: Compress,
    obj: Option<W>,
    buf: Vec<u8>,
    done: bool,
}

/// A compression stream which will have compressed data written to it and
/// will write uncompressed data to an output stream.
pub struct BzDecoder<W: Write> {
    data: Decompress,
    obj: Option<W>,
    buf: Vec<u8>,
    done: bool,
}

impl<W: Write> BzEncoder<W> {
    /// Create a new compression stream which will compress at the given level
    /// to write compress output to the give output stream.
    pub fn new(obj: W, level: Compression) -> BzEncoder<W> {
        BzEncoder {
            data: Compress::new(level, 30),
            obj: Some(obj),
            buf: Vec::with_capacity(32 * 1024),
            done: false,
        }
    }

    fn dump(&mut self) -> io::Result<()> {
        while self.buf.len() > 0 {
            let n = match self.obj.as_mut().unwrap().write(&self.buf) {
                Ok(n) => n,
                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            };
            self.buf.drain(..n);
        }
        Ok(())
    }

    /// Acquires a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        self.obj.as_ref().unwrap()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that mutating the output/input state of the stream may corrupt this
    /// object, so care must be taken when using this method.
    pub fn get_mut(&mut self) -> &mut W {
        self.obj.as_mut().unwrap()
    }

    /// Attempt to finish this output stream, writing out final chunks of data.
    ///
    /// Note that this function can only be used once data has finished being
    /// written to the output stream. After this function is called then further
    /// calls to `write` may result in a panic.
    ///
    /// # Panics
    ///
    /// Attempts to write data to this stream may result in a panic after this
    /// function is called.
    pub fn try_finish(&mut self) -> io::Result<()> {
        while !self.done {
            self.dump()?;
            let res = self.data.compress_vec(&[], &mut self.buf, Action::Finish);
            if res == Ok(Status::StreamEnd) {
                self.done = true;
                break;
            }
        }
        self.dump()
    }

    /// Consumes this encoder, flushing the output stream.
    ///
    /// This will flush the underlying data stream and then return the contained
    /// writer if the flush succeeded.
    ///
    /// Note that this function may not be suitable to call in a situation where
    /// the underlying stream is an asynchronous I/O stream. To finish a stream
    /// the `try_finish` (or `shutdown`) method should be used instead. To
    /// re-acquire ownership of a stream it is safe to call this method after
    /// `try_finish` or `shutdown` has returned `Ok`.
    pub fn finish(mut self) -> io::Result<W> {
        self.try_finish()?;
        Ok(self.obj.take().unwrap())
    }

    /// Returns the number of bytes produced by the compressor
    ///
    /// Note that, due to buffering, this only bears any relation to
    /// `total_in()` after a call to `flush()`.  At that point,
    /// `total_out() / total_in()` is the compression ratio.
    pub fn total_out(&self) -> u64 {
        self.data.total_out()
    }

    /// Returns the number of bytes consumed by the compressor
    /// (e.g. the number of bytes written to this stream.)
    pub fn total_in(&self) -> u64 {
        self.data.total_in()
    }
}

impl<W: Write> Write for BzEncoder<W> {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        loop {
            self.dump()?;

            let total_in = self.total_in();
            self.data
                .compress_vec(data, &mut self.buf, Action::Run)
                .unwrap();
            let written = (self.total_in() - total_in) as usize;

            if written > 0 || data.len() == 0 {
                return Ok(written);
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        loop {
            self.dump()?;
            let before = self.total_out();
            self.data
                .compress_vec(&[], &mut self.buf, Action::Flush)
                .unwrap();

            if before == self.total_out() {
                break;
            }
        }
        self.obj.as_mut().unwrap().flush()
    }
}

#[cfg(feature = "tokio")]
impl<W: AsyncWrite> AsyncWrite for BzEncoder<W> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        try_nb!(self.try_finish());
        self.get_mut().shutdown()
    }
}

impl<W: Read + Write> Read for BzEncoder<W> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.get_mut().read(buf)
    }
}

#[cfg(feature = "tokio")]
impl<W: AsyncRead + AsyncWrite> AsyncRead for BzEncoder<W> {}

impl<W: Write> Drop for BzEncoder<W> {
    fn drop(&mut self) {
        if self.obj.is_some() {
            let _ = self.try_finish();
        }
    }
}

impl<W: Write> BzDecoder<W> {
    /// Create a new decoding stream which will decompress all data written
    /// to it into `obj`.
    pub fn new(obj: W) -> BzDecoder<W> {
        BzDecoder {
            data: Decompress::new(false),
            obj: Some(obj),
            buf: Vec::with_capacity(32 * 1024),
            done: false,
        }
    }

    /// Acquires a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        self.obj.as_ref().unwrap()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that mutating the output/input state of the stream may corrupt this
    /// object, so care must be taken when using this method.
    pub fn get_mut(&mut self) -> &mut W {
        self.obj.as_mut().unwrap()
    }

    fn dump(&mut self) -> io::Result<()> {
        while self.buf.len() > 0 {
            let n = match self.obj.as_mut().unwrap().write(&self.buf) {
                Ok(n) => n,
                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            };
            self.buf.drain(..n);
        }
        Ok(())
    }

    /// Attempt to finish this output stream, writing out final chunks of data.
    ///
    /// Note that this function can only be used once data has finished being
    /// written to the output stream. After this function is called then further
    /// calls to `write` may result in a panic.
    ///
    /// # Panics
    ///
    /// Attempts to write data to this stream may result in a panic after this
    /// function is called.
    pub fn try_finish(&mut self) -> io::Result<()> {
        while !self.done {
            self.write(&[])?;
        }
        self.dump()
    }

    /// Unwrap the underlying writer, finishing the compression stream.
    ///
    /// Note that this function may not be suitable to call in a situation where
    /// the underlying stream is an asynchronous I/O stream. To finish a stream
    /// the `try_finish` (or `shutdown`) method should be used instead. To
    /// re-acquire ownership of a stream it is safe to call this method after
    /// `try_finish` or `shutdown` has returned `Ok`.
    pub fn finish(&mut self) -> io::Result<W> {
        self.try_finish()?;
        Ok(self.obj.take().unwrap())
    }

    /// Returns the number of bytes produced by the decompressor
    ///
    /// Note that, due to buffering, this only bears any relation to
    /// `total_in()` after a call to `flush()`.  At that point,
    /// `total_in() / total_out()` is the compression ratio.
    pub fn total_out(&self) -> u64 {
        self.data.total_out()
    }

    /// Returns the number of bytes consumed by the decompressor
    /// (e.g. the number of bytes written to this stream.)
    pub fn total_in(&self) -> u64 {
        self.data.total_in()
    }
}

impl<W: Write> Write for BzDecoder<W> {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if self.done {
            return Ok(0);
        }
        loop {
            self.dump()?;

            let before = self.total_in();
            let res = self.data.decompress_vec(data, &mut self.buf);
            let written = (self.total_in() - before) as usize;

            let res = res.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

            if res == Status::StreamEnd {
                self.done = true;
            }
            if written > 0 || data.len() == 0 || self.done {
                return Ok(written);
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.dump()?;
        self.obj.as_mut().unwrap().flush()
    }
}

#[cfg(feature = "tokio")]
impl<W: AsyncWrite> AsyncWrite for BzDecoder<W> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        try_nb!(self.try_finish());
        self.get_mut().shutdown()
    }
}

impl<W: Read + Write> Read for BzDecoder<W> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.get_mut().read(buf)
    }
}

#[cfg(feature = "tokio")]
impl<W: AsyncRead + AsyncWrite> AsyncRead for BzDecoder<W> {}

impl<W: Write> Drop for BzDecoder<W> {
    fn drop(&mut self) {
        if self.obj.is_some() {
            let _ = self.try_finish();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BzDecoder, BzEncoder};
    use partial_io::{GenInterrupted, PartialWithErrors, PartialWrite};
    use std::io::prelude::*;
    use std::iter::repeat;

    #[test]
    fn smoke() {
        let d = BzDecoder::new(Vec::new());
        let mut c = BzEncoder::new(d, ::Compression::default());
        c.write_all(b"12834").unwrap();
        let s = repeat("12345").take(100000).collect::<String>();
        c.write_all(s.as_bytes()).unwrap();
        let data = c.finish().unwrap().finish().unwrap();
        assert_eq!(&data[0..5], b"12834");
        assert_eq!(data.len(), 500005);
        assert!(format!("12834{}", s).as_bytes() == &*data);
    }

    #[test]
    fn write_empty() {
        let d = BzDecoder::new(Vec::new());
        let mut c = BzEncoder::new(d, ::Compression::default());
        c.write(b"").unwrap();
        let data = c.finish().unwrap().finish().unwrap();
        assert_eq!(&data[..], b"");
    }

    #[test]
    fn qc() {
        ::quickcheck::quickcheck(test as fn(_) -> _);

        fn test(v: Vec<u8>) -> bool {
            let w = BzDecoder::new(Vec::new());
            let mut w = BzEncoder::new(w, ::Compression::default());
            w.write_all(&v).unwrap();
            v == w.finish().unwrap().finish().unwrap()
        }
    }

    #[test]
    fn qc_partial() {
        quickcheck6::quickcheck(test as fn(_, _, _) -> _);

        fn test(
            v: Vec<u8>,
            encode_ops: PartialWithErrors<GenInterrupted>,
            decode_ops: PartialWithErrors<GenInterrupted>,
        ) -> bool {
            let w = BzDecoder::new(PartialWrite::new(Vec::new(), decode_ops));
            let mut w = BzEncoder::new(PartialWrite::new(w, encode_ops), ::Compression::default());
            w.write_all(&v).unwrap();
            v == w
                .finish()
                .unwrap()
                .into_inner()
                .finish()
                .unwrap()
                .into_inner()
        }
    }
}
