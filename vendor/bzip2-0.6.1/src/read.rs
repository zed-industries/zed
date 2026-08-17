//! Reader-based compression/decompression streams

use std::io::prelude::*;
use std::io::{self, BufReader};

use crate::bufread;
use crate::Compression;

/// A compression stream which wraps an uncompressed stream of data. Compressed
/// data will be read from the stream.
pub struct BzEncoder<R> {
    inner: bufread::BzEncoder<BufReader<R>>,
}

/// A decompression stream which wraps a compressed stream of data. Decompressed
/// data will be read from the stream.
pub struct BzDecoder<R> {
    inner: bufread::BzDecoder<BufReader<R>>,
}

impl<R: Read> BzEncoder<R> {
    /// Create a new compression stream which will compress at the given level
    /// to read compress output to the give output stream.
    pub fn new(r: R, level: Compression) -> Self {
        Self {
            inner: bufread::BzEncoder::new(BufReader::new(r), level),
        }
    }

    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        self.inner.get_ref().get_ref()
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        self.inner.get_mut().get_mut()
    }

    /// Unwrap the underlying writer, finishing the compression stream.
    pub fn into_inner(self) -> R {
        self.inner.into_inner().into_inner()
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
        self.inner.total_out()
    }

    /// Returns the number of bytes consumed by the compressor
    /// (e.g. the number of bytes read from the underlying stream)
    pub fn total_in(&self) -> u64 {
        self.inner.total_in()
    }
}

impl<R: Read> Read for BzEncoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<W: Write + Read> Write for BzEncoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.get_mut().flush()
    }
}

impl<R: Read> BzDecoder<R> {
    /// Create a new decompression stream, which will read compressed
    /// data from the given input stream and decompress it.
    pub fn new(r: R) -> Self {
        Self {
            inner: bufread::BzDecoder::new(BufReader::new(r)),
        }
    }

    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        self.inner.get_ref().get_ref()
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        self.inner.get_mut().get_mut()
    }

    /// Unwrap the underlying writer, finishing the compression stream.
    pub fn into_inner(self) -> R {
        self.inner.into_inner().into_inner()
    }

    /// Returns the number of bytes produced by the decompressor
    /// (e.g. the number of bytes read from this stream)
    ///
    /// Note that, due to buffering, this only bears any relation to
    /// total_in() when the decompressor reaches a sync point
    /// (e.g. where the original compressed stream was flushed).
    /// At that point, `total_in() / total_out()` is the compression ratio.
    pub fn total_out(&self) -> u64 {
        self.inner.total_out()
    }

    /// Returns the number of bytes consumed by the decompressor
    /// (e.g. the number of bytes read from the underlying stream)
    pub fn total_in(&self) -> u64 {
        self.inner.total_in()
    }
}

impl<R: Read> Read for BzDecoder<R> {
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        self.inner.read(into)
    }
}

impl<W: Write + Read> Write for BzDecoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.get_mut().flush()
    }
}

/// A bzip2 streaming decoder that decodes all members of a multistream
///
/// Wikipedia, particularly, uses bzip2 multistream for their dumps.
pub struct MultiBzDecoder<R> {
    inner: bufread::MultiBzDecoder<BufReader<R>>,
}

impl<R: Read> MultiBzDecoder<R> {
    /// Creates a new decoder from the given reader, immediately parsing the
    /// (first) gzip header. If the gzip stream contains multiple members all will
    /// be decoded.
    pub fn new(r: R) -> Self {
        Self {
            inner: bufread::MultiBzDecoder::new(BufReader::new(r)),
        }
    }
}

impl<R> MultiBzDecoder<R> {
    /// Acquires a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        self.inner.get_ref().get_ref()
    }

    /// Acquires a mutable reference to the underlying stream.
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        self.inner.get_mut().get_mut()
    }

    /// Consumes this decoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner.into_inner().into_inner()
    }
}

impl<R: Read> Read for MultiBzDecoder<R> {
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        self.inner.read(into)
    }
}

#[cfg(test)]
mod tests {
    use crate::read::{BzDecoder, BzEncoder, MultiBzDecoder};
    use crate::Compression;
    use partial_io::quickcheck_types::{GenInterrupted, PartialWithErrors};
    use partial_io::PartialRead;
    use rand::distr::StandardUniform;
    use rand::{rng, Rng};
    use std::io::Read;

    #[test]
    fn smoke() {
        let m: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
        let mut c = BzEncoder::new(m, Compression::default());
        let mut data = vec![];
        c.read_to_end(&mut data).unwrap();
        let mut d = BzDecoder::new(&data[..]);
        let mut data2 = Vec::new();
        d.read_to_end(&mut data2).unwrap();
        assert_eq!(data2, m);
    }

    #[test]
    fn smoke2() {
        let m: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
        let c = BzEncoder::new(m, Compression::default());
        let mut d = BzDecoder::new(c);
        let mut data = vec![];
        d.read_to_end(&mut data).unwrap();
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn smoke3() {
        let m = vec![3u8; 128 * 1024 + 1];
        let c = BzEncoder::new(&m[..], Compression::default());
        let mut d = BzDecoder::new(c);
        let mut data = vec![];
        d.read_to_end(&mut data).unwrap();
        assert!(data == m[..]);
    }

    #[test]
    fn self_terminating() {
        let m = vec![3u8; 128 * 1024 + 1];
        let mut c = BzEncoder::new(&m[..], Compression::default());

        let mut result = Vec::new();
        c.read_to_end(&mut result).unwrap();

        let v = rng()
            .sample_iter(&StandardUniform)
            .take(1024)
            .collect::<Vec<u8>>();
        for _ in 0..200 {
            result.extend(v.iter().copied());
        }

        let mut d = BzDecoder::new(&result[..]);
        let mut data = vec![0; m.len()];
        assert!(d.read(&mut data).unwrap() == m.len());
        assert!(data == m[..]);
    }

    #[test]
    fn zero_length_read_at_eof() {
        let m = Vec::new();
        let mut c = BzEncoder::new(&m[..], Compression::default());

        let mut result = Vec::new();
        c.read_to_end(&mut result).unwrap();

        let mut d = BzDecoder::new(&result[..]);
        let mut data = Vec::new();
        assert!(d.read(&mut data).unwrap() == 0);
    }

    #[test]
    fn zero_length_read_with_data() {
        let m = vec![3u8; 128 * 1024 + 1];
        let mut c = BzEncoder::new(&m[..], Compression::default());

        let mut result = Vec::new();
        c.read_to_end(&mut result).unwrap();

        let mut d = BzDecoder::new(&result[..]);
        let mut data = Vec::new();
        assert!(d.read(&mut data).unwrap() == 0);
    }

    #[test]
    fn multistream_read_till_eof() {
        let m = vec![3u8; 128 * 1024 + 1];
        let repeat = 3;
        let mut result = Vec::new();

        for _i in 0..repeat {
            let mut c = BzEncoder::new(&m[..], Compression::default());
            c.read_to_end(&mut result).unwrap();
        }

        let mut d = MultiBzDecoder::new(&result[..]);
        let mut data = Vec::new();

        let a = d.read_to_end(&mut data).unwrap();
        let b = m.len() * repeat;
        assert!(a == b, "{} {}", a, b);
    }

    #[test]
    fn empty() {
        let r = BzEncoder::new(&[][..], Compression::default());
        let mut r = BzDecoder::new(r);
        let mut v2 = Vec::new();
        r.read_to_end(&mut v2).unwrap();
        assert!(v2.is_empty());
    }

    #[test]
    fn qc() {
        ::quickcheck::quickcheck(test as fn(_) -> _);

        fn test(v: Vec<u8>) -> bool {
            let r = BzEncoder::new(&v[..], Compression::default());
            let mut r = BzDecoder::new(r);
            let mut v2 = Vec::new();
            r.read_to_end(&mut v2).unwrap();
            v == v2
        }
    }

    #[test]
    fn qc_partial() {
        ::quickcheck::quickcheck(test as fn(_, _, _) -> _);

        fn test(
            v: Vec<u8>,
            encode_ops: PartialWithErrors<GenInterrupted>,
            decode_ops: PartialWithErrors<GenInterrupted>,
        ) -> bool {
            let r = BzEncoder::new(PartialRead::new(&v[..], encode_ops), Compression::default());
            let mut r = BzDecoder::new(PartialRead::new(r, decode_ops));
            let mut v2 = Vec::new();
            r.read_to_end(&mut v2).unwrap();
            v == v2
        }
    }
}
